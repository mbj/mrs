use crate::Container;
use crate::cbt;

pub mod version {
    #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
    pub struct Major(u8);

    impl std::fmt::Display for Major {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "{}", self.0)
        }
    }

    impl Major {
        pub const fn new(value: u8) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
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

    #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
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

    impl std::str::FromStr for Version {
        type Err = String;

        fn from_str(value: &str) -> Result<Self, Self::Err> {
            fn parse<T>(
                constructor: fn(u8) -> T,
                field: &str,
                captures: &regex_lite::Captures,
            ) -> Result<T, String> {
                match captures.name(field).unwrap().as_str().parse() {
                    Ok(value) => Ok(constructor(value)),
                    Err(error) => Err(format!(
                        "Cannot parse version component {field}, error: {error}"
                    )),
                }
            }

            let regex =
                regex_lite::Regex::new(r#"\A(?<major>[0-9]+)\.(?<minor>[0-9]+)\z"#).unwrap();

            match regex.captures(value) {
                None => Err("invalid version format".to_string()),
                Some(captures) => {
                    let minor = parse(Minor, "minor", &captures)?;
                    let major = parse(Major, "major", &captures)?;

                    Ok(Self { major, minor })
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    ApplyPendingMigrations,
    ApplyPendingMigrationsNoSchemaDump,
    SqlFile(file_buf::FileBuf),
    SqlFileGitRevision {
        file: file_buf::FileBuf,
        git_revision: &'static str,
    },
}

#[derive(Debug, PartialEq)]
pub struct Definition {
    pub application_name: Option<pg_client::ApplicationName>,
    pub backend: crate::cbt::Backend,
    pub database: pg_client::Database,
    pub migration_config: Option<mmigration::Config>,
    pub steps: Vec<Step>,
    pub superuser: pg_client::Username,
    pub version: version::Version,
}

impl Definition {
    pub fn new(version: version::Version) -> Self {
        Self {
            backend: crate::cbt::backend::autodetect::run().unwrap(),
            application_name: None,
            migration_config: None,
            steps: vec![],
            superuser: pg_client::username!("postgres"),
            database: pg_client::database!("postgres"),
            version,
        }
    }

    pub fn apply_file(self, file: file_buf::FileBuf) -> Self {
        self.push_step(Step::SqlFile(file))
    }

    pub fn migration_config(self, migration_config: mmigration::Config) -> Self {
        Self {
            migration_config: Some(migration_config),
            ..self
        }
    }

    pub fn superuser(self, username: pg_client::Username) -> Self {
        Self {
            superuser: username,
            ..self
        }
    }

    pub fn apply_pending_migrations(self) -> Self {
        self.push_step(Step::ApplyPendingMigrations)
    }

    pub fn apply_pending_migrations_no_schema_dump(self) -> Self {
        self.push_step(Step::ApplyPendingMigrationsNoSchemaDump)
    }

    pub fn apply_file_from_git_revision(
        self,
        file: file_buf::FileBuf,
        git_revision: &'static str,
    ) -> Self {
        self.push_step(Step::SqlFileGitRevision { file, git_revision })
    }

    fn push_step(self, step: Step) -> Self {
        let mut steps = self.steps.clone();

        steps.push(step);

        Self { steps, ..self }
    }

    pub fn to_cbt_definition(&self) -> cbt::Definition {
        let image = cbt::Image::from(format!(
            "registry.hub.docker.com/library/postgres:{}",
            self.version
        ));

        cbt::Definition::new(self.backend, image)
    }

    pub async fn with_container<T>(&self, mut action: impl AsyncFnMut(&Container) -> T) -> T {
        let mut db_container = Container::run(self);

        db_container.wait_available().await;

        for step in &self.steps {
            self.apply_step(&db_container, step).await
        }

        let result = action(&db_container).await;

        db_container.stop();

        result
    }

    pub async fn run_integration_server(&self) {
        use tokio::io::AsyncReadExt;

        self.with_container(async |container| {
            println!("{}", serde_json::to_string(&container.client_config).unwrap());
            log::info!("Integration server is running waiting for EOF on stdin");
            let mut stdin = tokio::io::stdin();
            let mut buf = [0u8; 128];

            loop {
                match stdin.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(length) => { log::warn!("Integration server received unexpected data on stdin! bytes: {length}") }
                    Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(error) => panic!("{error}"),
                }
            }

            log::info!("Integration server received EOF on stdin, exiting");
        })
        .await
    }

    async fn apply_step(&self, db_container: &Container<'_>, step: &Step) {
        match step {
            Step::SqlFile(file) => db_container.apply_sql_file(file).await,
            Step::SqlFileGitRevision { file, git_revision } => {
                db_container
                    .apply_sql_file_git_revision(file, git_revision)
                    .await
            }
            Step::ApplyPendingMigrations => db_container.apply_pending_migrations().await,
            Step::ApplyPendingMigrationsNoSchemaDump => {
                db_container.apply_pending_migrations_no_schema_dump().await
            }
        }
    }

    pub fn schema_dump(
        &self,
        client_config: &pg_client::Config,
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

fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}

pub fn apply_cbt_mounts(client_config: &pg_client::Config) -> (pg_client::Config, Vec<cbt::Mount>) {
    let owned_client_config = client_config.clone();

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
                        ..owned_client_config
                    },
                    mounts,
                )
            }
            pg_client::SslRootCert::System => (owned_client_config, vec![]),
        },
        None => (owned_client_config, vec![]),
    }
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
        /// Run interactive psql session on the container
        ContainerPsql,
        /// Run schema dump form the container
        ContainerSchemaDump,
        /// Run interactive shell on the container
        ContainerShell,
        /// Run integration server
        ///
        /// Intent to be used for automation with other languages wrapping pg-ephemeral.
        ///
        /// After sucessful boot this command will print a single line to stdout containing a JSON
        /// represnetation of the root connection details.
        ///
        /// The server will stop once stdin returns EOF, aka the parent process closed it.
        IntegrationServer,
        /// Migration subcommands
        Migration(mmigration::cli::App),
        /// Run interactive psql on the host
        Psql,
    }

    impl Command {
        pub async fn run(&self, definition: &Definition) {
            match self {
                Self::ContainerPsql => definition.with_container(container_psql).await,
                Self::ContainerSchemaDump => definition.with_container(container_schema_dump).await,
                Self::ContainerShell => definition.with_container(container_shell).await,
                Self::IntegrationServer => definition.run_integration_server().await,
                Self::Migration(app) => run_migration(definition, app).await,
                Self::Psql => definition.with_container(host_psql).await,
            }
        }
    }

    async fn host_psql(db_container: &Container<'_>) {
        let _ = std::process::Command::new("psql")
            .envs(db_container.client_config.to_pg_env())
            .status();
    }

    async fn run_migration(definition: &Definition, app: &mmigration::cli::App) {
        definition
            .with_container(async |container| app.run(container.migration_context()).await)
            .await
    }

    async fn container_schema_dump(db_container: &Container<'_>) {
        println!("{}", db_container.exec_schema_dump());
    }

    async fn container_psql(db_container: &Container<'_>) {
        db_container.exec_psql()
    }

    async fn container_shell(db_container: &Container<'_>) {
        db_container.exec_container_shell()
    }
}
