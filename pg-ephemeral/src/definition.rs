use crate::Container;
use crate::cbt;

#[derive(Clone, Debug, PartialEq)]
pub enum BackendSelection {
    Auto,
    Docker,
    Podman,
}

impl BackendSelection {
    pub fn resolve(&self) -> cbt::Backend {
        match self {
            Self::Auto => crate::cbt::backend::autodetect::run().unwrap(),
            Self::Docker => cbt::Backend::Docker,
            Self::Podman => cbt::Backend::Podman,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    ApplyPendingMigrations,
    ApplyPendingMigrationsNoSchemaDump,
    SqlFile(std::path::PathBuf),
    SqlFileGitRevision {
        git_revision: &'static str,
        path: std::path::PathBuf,
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
    pub image: crate::image::Image,
}

impl Definition {
    pub fn new(backend_selection: BackendSelection, image: crate::image::Image) -> Self {
        Self {
            backend: backend_selection.resolve(),
            application_name: None,
            migration_config: None,
            steps: vec![],
            superuser: pg_client::username!("postgres"),
            database: pg_client::database!("postgres"),
            image,
        }
    }

    pub fn apply_file(self, path: std::path::PathBuf) -> Self {
        self.push_step(Step::SqlFile(path))
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
        path: std::path::PathBuf,
        git_revision: &'static str,
    ) -> Self {
        self.push_step(Step::SqlFileGitRevision { git_revision, path })
    }

    fn push_step(self, step: Step) -> Self {
        let mut steps = self.steps.clone();

        steps.push(step);

        Self { steps, ..self }
    }

    pub fn to_cbt_definition(&self) -> cbt::Definition {
        cbt::Definition::new(self.backend, (&self.image).into())
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
            println!(
                "{}",
                serde_json::to_string(&container.client_config).unwrap()
            );
            log::info!("Integration server is running waiting for EOF on stdin");
            let mut stdin = tokio::io::stdin();
            let mut buf = [0u8; 128];

            loop {
                match stdin.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(length) => {
                        log::warn!(
                            "Integration server received unexpected data on stdin! bytes: {length}"
                        )
                    }
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
            Step::SqlFile(path) => db_container.apply_sql_file(path).await,
            Step::SqlFileGitRevision { path, git_revision } => {
                db_container
                    .apply_sql_file_git_revision(path, git_revision)
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

                let mut container_path = std::path::PathBuf::new();

                container_path.push("/pg_ephemeral");
                container_path.push(file.file_name().unwrap());

                let mounts = vec![cbt::Mount::from(format!(
                    "type=bind,ro,source={},target={}",
                    host.to_str().unwrap(),
                    container_path.to_str().unwrap()
                ))];

                (
                    pg_client::Config {
                        ssl_root_cert: Some(container_path.into()),
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
        /// Run shell command with environment variables for PostgreSQL connection
        ///
        /// Sets all PostgreSQL-related environment variables:
        /// - libpq-style PG* environment variables (PGHOST, PGPORT, PGUSER, PGDATABASE, PGPASSWORD, PGSSLMODE, etc.)
        /// - DATABASE_URL in PostgreSQL URL format
        RunEnv {
            /// The command to run
            command: String,
            /// Arguments to pass to the command
            arguments: Vec<String>,
        },
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
                Self::RunEnv { command, arguments } => {
                    definition
                        .with_container(async |container| {
                            host_command(container, command, arguments).await
                        })
                        .await
                }
            }
        }
    }

    async fn host_psql(db_container: &Container<'_>) {
        let _ = std::process::Command::new("psql")
            .envs(db_container.client_config.to_pg_env())
            .status();
    }

    async fn host_command<'a>(
        db_container: &'a Container<'a>,
        command: &str,
        arguments: &'a Vec<String>,
    ) {
        let mut cmd = std::process::Command::new(command);
        cmd.args(arguments);

        // Set all PG* environment variables
        cmd.envs(db_container.pg_env());

        // Set DATABASE_URL
        cmd.env("DATABASE_URL", db_container.database_url());

        let _ = cmd.status();
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
