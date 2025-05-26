use rand::Rng;

pub mod cbt;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

pub fn apply_cbt_mounts(
    client_config: &pg_client::Config,
) -> (pg_client::Config, Vec<crate::cbt::Mount>) {
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

#[derive(Clone, Debug)]
enum Step {
    SqlFile(file_buf::FileBuf),
    SqlFileGitRevision {
        file: file_buf::FileBuf,
        git_revision: &'static str,
    },
    ApplyPendingMigrations,
    ApplyPendingMigrationsNoSchemaDump,
}

struct SchemaDump<'a> {
    container: &'a Container<'a>,
}

impl mmigration::SchemaDump for SchemaDump<'_> {
    async fn schema_dump(&self) -> mmigration::Schema {
        self.container.exec_schema_dump().into()
    }
}

#[derive(Debug)]
pub struct Definition {
    migration_config: Option<mmigration::Config>,
    steps: Vec<Step>,
    superuser: pg_client::Username,
    version: Version,
}

impl Definition {
    pub fn new(version: Version) -> Self {
        Self {
            migration_config: None,
            steps: vec![],
            superuser: pg_client::username!("postgres"),
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

    fn to_cbt_definition(&self) -> cbt::Definition {
        let image = cbt::Image::from(format!(
            "registry.hub.docker.com/library/postgres:{}",
            self.version
        ));

        cbt::Definition::new(image)
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

#[derive(Debug)]
pub struct Container<'a> {
    client_config: pg_client::Config,
    container: cbt::Container,
    definition: &'a Definition,
}

impl<'a> Container<'a> {
    fn run(definition: &'a Definition) -> Self {
        let password = generate_password();

        let container = definition
            .to_cbt_definition()
            .remove()
            .env("POSTGRES_PASSWORD", password.as_ref())
            .env("POSTGRES_USER", definition.superuser.as_ref())
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
            username: definition.superuser.clone(),
        };

        Container {
            container,
            definition,
            client_config,
        }
    }

    fn migration_context(&'a self) -> mmigration::Context<'a, SchemaDump<'a>> {
        let migration_config = self
            .definition
            .migration_config
            .as_ref()
            .expect("migration not configured");

        mmigration::Context::load(
            migration_config,
            &self.client_config,
            SchemaDump { container: self },
        )
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
                    log::trace!("{error:#?}, retry in 100ms");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        panic!("container did not become avaialble within ~10 seconds!");
    }

    fn exec_schema_dump(&self) -> String {
        convert_schema(&self.container.exec_capture_only_stdout(
            self.container_client_config().to_pg_env(),
            "pg_dump",
            ["--schema-only"],
        ))
    }

    pub async fn with_connection<T, F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> T>(
        &self,
        mut action: F,
    ) -> T {
        self.client_config
            .with_sqlx_connection(async |connection| action(connection).await)
            .await
    }

    pub async fn apply_pending_migrations(&self) {
        self.migration_context().apply_pending().await
    }

    pub async fn apply_pending_migrations_no_schema_dump(&self) {
        self.migration_context()
            .apply_pending_no_schema_dump()
            .await
    }

    pub async fn apply_sql_file(&self, file: &file_buf::FileBuf) {
        self.apply_sql(&file.read_to_string()).await
    }

    pub async fn apply_sql_file_git_revision(
        &self,
        file: &file_buf::FileBuf,
        git_revision: &'static str,
    ) {
        let sql = cbt::Command::new("git")
            .argument("show")
            .argument(format!("{git_revision}:{}", file.to_str()))
            .capture_only_stdout_string();

        self.apply_sql(&sql).await
    }

    pub async fn apply_sql(&self, sql: &str) {
        self.with_connection(async |connection| {
            log::debug!("Executing: {}", sql);
            sqlx::raw_sql(sql).execute(connection).await.unwrap();
        })
        .await
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
        Migration(mmigration::cli::App),
        Psql,
    }

    impl Command {
        pub async fn run(&self, definition: &Definition) {
            match self {
                Self::ContainerPsql => definition.with_container(container_psql).await,
                Self::ContainerSchemaDump => definition.with_container(container_schema_dump).await,
                Self::ContainerShell => definition.with_container(container_shell).await,
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

fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}
