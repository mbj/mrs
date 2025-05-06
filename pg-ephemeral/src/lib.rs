use rand::Rng;

pub mod cbt;
pub mod pg_client;

pub enum Major {
    R15,
    R16,
    R17,
    R18,
}

impl std::fmt::Display for Major {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = match self {
            Self::R15 => "15",
            Self::R16 => "16",
            Self::R17 => "17",
            Self::R18 => "18",
        };

        write!(formatter, "{}", value)
    }
}

pub struct Minor(u8);

impl Minor {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Minor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

pub struct Version {
    major: Major,
    minor: Minor,
}

impl Version {
    pub const fn new(major: Major, minor: Minor) -> Self {
        Self { major, minor }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

pub struct Definition {
    version: Version,
}

pub fn apply_cbt_mounts(
    client_config: pg_client::Config,
) -> (pg_client::Config, Vec<crate::cbt::Mount>) {
    match client_config.ssl_root_cert {
        Some(ref ssl_root_cert) => match ssl_root_cert {
            pg_client::SslRootCert::File(file) => {
                let host =
                    std::fs::canonicalize(file).expect("could not canonicalize ssl root path");

                let mut container = std::path::PathBuf::new();

                container.push("/pg_ephemeral");
                container.push(file.file_name());

                let mounts = vec![cbt::Mount::from(format!(
                    "type=bind,ro,source={},target={}",
                    host.to_str().unwrap(),
                    container.to_str().unwrap()
                ))];

                (
                    pg_client::Config {
                        ssl_root_cert: Some(
                            pg_client::SslRootCert::from_path_unchecked_existance(container)
                                .unwrap(),
                        ),
                        ..client_config
                    },
                    mounts,
                )
            }
            pg_client::SslRootCert::System => (client_config, vec![]),
        },
        None => (client_config, vec![]),
    }
}

impl Definition {
    pub const fn new(version: Version) -> Self {
        Self { version }
    }

    fn to_cbt_definition(&self) -> cbt::Definition {
        let image = cbt::Image::from(format!(
            "registry.hub.docker.com/library/postgres:{}",
            self.version
        ));

        cbt::Definition::new(image)
    }

    async fn with_container<T>(&self, action: impl Fn(&Container) -> T) -> T {
        let mut db_container = Container::run(self);

        db_container.wait_available().await;

        let result = action(&db_container);

        db_container.stop();

        result
    }

    pub fn schema_dump(
        &self,
        client_config: pg_client::Config,
        extra_arguments: &[String],
    ) -> String {
        let (effective_config, mounts) = apply_cbt_mounts(client_config);

        let mut effective_arguments = vec!["--schema-only".to_string()];

        effective_arguments.extend_from_slice(extra_arguments);

        let bytes = self
            .to_cbt_definition()
            .entrypoint("pg_dump".to_string(), effective_arguments)
            .envs(effective_config.to_pg_env())
            .mounts(mounts)
            .run_capture_only_stdout();

        convert_schema(&bytes)
    }
}

#[derive(Debug)]
pub struct Container {
    client_config: pg_client::Config,
    container: cbt::Container,
}

impl Container {
    fn run(definition: &Definition) -> Self {
        let password = generate_password();

        let container = definition
            .to_cbt_definition()
            .remove()
            .env("POSTGRES_PASSWORD", password.as_ref())
            .publish(cbt::Publish::from("127.0.0.1::5432/tcp"))
            .run_detached();

        let port = pg_client::Port(
            std::str::FromStr::from_str(&container.inspect_format(
                "{{(index (index .NetworkSettings.Ports \"5432/tcp\") 0).HostPort}}",
            ))
            .expect("invalid port"),
        );

        let host = pg_client::host!("localhost");

        let client_config = pg_client::Config {
            database: None,
            host,
            password: Some(password),
            port,
            ssl_mode: pg_client::SslMode::Disable,
            ssl_root_cert: None,
            username: pg_client::username!("postgres"),
        };

        Container {
            container,
            client_config,
        }
    }

    pub async fn wait_available(&self) {
        let config = self.client_config.to_sqlx_connect_options();

        for _ in 0..100 {
            match sqlx::ConnectOptions::connect(&config).await {
                Ok(connection) => {
                    sqlx::Connection::close(connection)
                        .await
                        .expect("connection close failed");

                    return;
                }
                Err(error) => {
                    eprintln!("{error:#?}, retry in 100ms");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        panic!("container did not become avaialble within ~10 seconds!");
    }

    fn exec_container_schema_dump(&self) -> String {
        convert_schema(&self.container.exec_capture_only_stdout(
            self.container_client_config().to_pg_env(),
            "pg_dump",
            ["--schema-only"],
        ))
    }

    fn exec_container_shell(&self) {
        self.container
            .exec_interactive(self.container_client_config().to_pg_env(), "sh", [])
    }

    fn exec_psql(&self) {
        self.container
            .exec_interactive(self.container_client_config().to_pg_env(), "psql", [])
    }

    fn container_client_config(&self) -> pg_client::Config {
        pg_client::Config {
            port: pg_client::Port(5432),
            ..self.client_config.clone()
        }
    }

    fn stop(&mut self) {
        self.container.stop()
    }
}

fn generate_password() -> pg_client::Password {
    let rng = rand::rng();

    let value: String = rng
        .sample_iter(rand::distr::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    <pg_client::Password as std::str::FromStr>::from_str(&value).unwrap()
}

pub mod cli {
    use super::*;

    #[derive(Clone, Debug, clap::Parser)]
    pub struct App {
        #[clap(subcommand)]
        command: Command,
    }

    impl App {
        pub async fn run(&self, definition: &Definition) {
            self.command.run(definition).await
        }
    }

    #[derive(Clone, Debug, clap::Parser)]
    pub enum Command {
        ContainerPsql,
        ContainerSchemaDump,
        ContainerShell,
        Psql,
    }

    impl Command {
        pub async fn run(&self, definition: &Definition) {
            match self {
                Self::ContainerPsql => definition.with_container(container_psql).await,
                Self::ContainerSchemaDump => definition.with_container(container_schema_dump).await,
                Self::ContainerShell => definition.with_container(container_shell).await,
                Self::Psql => definition.with_container(host_psql).await,
            }
        }
    }

    fn host_psql(db_container: &Container) {
        let _ = std::process::Command::new("psql")
            .envs(db_container.client_config.to_pg_env())
            .status();
    }

    fn container_schema_dump(db_container: &Container) {
        eprintln!("{}", db_container.exec_container_schema_dump());
    }

    fn container_psql(db_container: &Container) {
        db_container.exec_psql()
    }

    fn container_shell(db_container: &Container) {
        db_container.exec_container_shell()
    }
}

fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}
