use crate::Container;
use crate::seed::{Command, DuplicateSeedName, Seed, SeedName};

#[derive(Clone, Debug, PartialEq)]
pub enum BackendSelection {
    Auto,
    Docker,
    Podman,
}

impl BackendSelection {
    pub fn resolve(&self) -> cbt::Backend {
        match self {
            Self::Auto => cbt::backend::autodetect::run().unwrap(),
            Self::Docker => cbt::Backend::Docker,
            Self::Podman => cbt::Backend::Podman,
        }
    }
}

impl From<cbt::Backend> for BackendSelection {
    fn from(backend: cbt::Backend) -> Self {
        match backend {
            cbt::Backend::Docker => Self::Docker,
            cbt::Backend::Podman => Self::Podman,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Definition {
    pub application_name: Option<pg_client::ApplicationName>,
    pub backend: cbt::Backend,
    pub database: pg_client::Database,
    pub migration_config: Option<mmigration::Config>,
    pub seeds: indexmap::IndexMap<SeedName, Seed>,
    pub superuser: pg_client::Username,
    pub image: crate::image::Image,
}

impl Definition {
    pub fn new(backend_selection: BackendSelection, image: crate::image::Image) -> Self {
        Self {
            backend: backend_selection.resolve(),
            application_name: None,
            migration_config: None,
            seeds: indexmap::IndexMap::new(),
            superuser: pg_client::username!("postgres"),
            database: pg_client::database!("postgres"),
            image,
        }
    }

    pub fn add_seed(self, name: SeedName, seed: Seed) -> Result<Self, DuplicateSeedName> {
        let mut seeds = self.seeds.clone();

        if seeds.contains_key(&name) {
            return Err(DuplicateSeedName(name));
        }

        seeds.insert(name, seed);
        Ok(Self { seeds, ..self })
    }

    pub fn apply_file(
        self,
        name: SeedName,
        path: std::path::PathBuf,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::SqlFile(path))
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

    pub fn apply_pending_migrations(self, name: SeedName) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::ApplyPendingMigrations)
    }

    pub fn apply_pending_migrations_no_schema_dump(
        self,
        name: SeedName,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::ApplyPendingMigrationsNoSchemaDump)
    }

    pub fn apply_file_from_git_revision(
        self,
        name: SeedName,
        path: std::path::PathBuf,
        git_revision: &'static str,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::SqlFileGitRevision { git_revision, path })
    }

    pub fn apply_command(
        self,
        name: SeedName,
        command: Command,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::Command(command))
    }

    pub fn apply_script(self, name: SeedName, script: String) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::Script(script))
    }

    pub fn to_cbt_definition(&self) -> cbt::Definition {
        cbt::Definition::new(self.backend, (&self.image).into())
    }

    pub async fn with_container<T>(&self, mut action: impl AsyncFnMut(&Container) -> T) -> T {
        let mut db_container = Container::run(self);

        db_container.wait_available().await;

        for seed in self.seeds.values() {
            self.apply_seed(&db_container, seed).await
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

    async fn apply_seed(&self, db_container: &Container<'_>, seed: &Seed) {
        match seed {
            Seed::SqlFile(path) => db_container.apply_sql_file(path).await,
            Seed::SqlFileGitRevision { path, git_revision } => {
                db_container
                    .apply_sql_file_git_revision(path, git_revision)
                    .await
            }
            Seed::ApplyPendingMigrations => db_container.apply_pending_migrations().await,
            Seed::ApplyPendingMigrationsNoSchemaDump => {
                db_container.apply_pending_migrations_no_schema_dump().await
            }
            Seed::Command(command) => self.execute_command(db_container, command),
            Seed::Script(script) => self.execute_script(db_container, script),
        }
    }

    fn apply_pg_env(cmd: &mut std::process::Command, db_container: &Container<'_>) {
        cmd.envs(db_container.pg_env());
        cmd.env("DATABASE_URL", db_container.database_url());
    }

    fn execute_command(&self, db_container: &Container<'_>, command: &Command) {
        let mut cmd = std::process::Command::new(&command.command);
        cmd.args(&command.arguments);

        Self::apply_pg_env(&mut cmd, db_container);

        let status = cmd.status().expect("Failed to execute command");

        if !status.success() {
            panic!("Command failed with status: {}", status);
        }
    }

    fn execute_script(&self, db_container: &Container<'_>, script: &str) {
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c");
        cmd.arg(script);

        Self::apply_pg_env(&mut cmd, db_container);

        let status = cmd.status().expect("Failed to execute script");

        if !status.success() {
            panic!("Script failed with status: {}", status);
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
            .entrypoint("pg_dump".to_string())
            .arguments(effective_arguments)
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

        Definition::apply_pg_env(&mut cmd, db_container);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_seed_rejects_duplicate() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());
        let seed_name: SeedName = "test-seed".parse().unwrap();

        let definition = definition
            .add_seed(seed_name.clone(), Seed::ApplyPendingMigrations)
            .unwrap();

        let result =
            definition.add_seed(seed_name.clone(), Seed::ApplyPendingMigrationsNoSchemaDump);

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_add_seed_allows_different_names() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());

        let definition = definition
            .add_seed("seed1".parse().unwrap(), Seed::ApplyPendingMigrations)
            .unwrap();

        let result = definition.add_seed(
            "seed2".parse().unwrap(),
            Seed::ApplyPendingMigrationsNoSchemaDump,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_file_rejects_duplicate() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());
        let seed_name: SeedName = "test-seed".parse().unwrap();

        let definition = definition
            .apply_file(seed_name.clone(), "file1.sql".into())
            .unwrap();

        let result = definition.apply_file(seed_name.clone(), "file2.sql".into());

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_command_adds_seed() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());

        let result = definition.apply_command(
            "test-command".parse().unwrap(),
            Command::new("echo".to_string(), vec!["test".to_string()]),
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_command_rejects_duplicate() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());
        let seed_name: SeedName = "test-command".parse().unwrap();

        let definition = definition
            .apply_command(
                seed_name.clone(),
                Command::new("echo".to_string(), vec!["test1".to_string()]),
            )
            .unwrap();

        let result = definition.apply_command(
            seed_name.clone(),
            Command::new("echo".to_string(), vec!["test2".to_string()]),
        );

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_script_adds_seed() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());

        let result =
            definition.apply_script("test-script".parse().unwrap(), "echo test".to_string());

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_script_rejects_duplicate() {
        let definition = Definition::new(BackendSelection::Podman, crate::Image::default());
        let seed_name: SeedName = "test-script".parse().unwrap();

        let definition = definition
            .apply_script(seed_name.clone(), "echo test1".to_string())
            .unwrap();

        let result = definition.apply_script(seed_name.clone(), "echo test2".to_string());

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }
}
