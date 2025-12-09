use crate::Container;
use crate::seed::{Command, DuplicateSeedName, LoadError, LoadedSeed, Seed, SeedName};

#[derive(Clone, Debug, PartialEq)]
pub enum SslConfig {
    Generated { hostname: pg_client::HostName },
    // UserProvided { ca_cert: PathBuf, server_cert: PathBuf, server_key: PathBuf },
}

#[derive(Debug, PartialEq)]
pub struct Definition {
    pub application_name: Option<pg_client::ApplicationName>,
    pub backend: ociman::Backend,
    pub database: pg_client::Database,
    pub seeds: indexmap::IndexMap<SeedName, Seed>,
    pub ssl_config: Option<SslConfig>,
    pub superuser: pg_client::Username,
    pub image: crate::image::Image,
    pub cross_container_access: bool,
}

impl Definition {
    pub fn new(backend: ociman::backend::Backend, image: crate::image::Image) -> Self {
        Self {
            backend,
            application_name: None,
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
            superuser: pg_client::username!("postgres"),
            database: pg_client::database!("postgres"),
            image,
            cross_container_access: false,
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
        self.add_seed(name, Seed::SqlFile { path })
    }

    fn load_seeds(&self) -> impl Iterator<Item = Result<LoadedSeed, LoadError>> + '_ {
        self.seeds
            .iter()
            .map(|(name, seed)| seed.load(name.clone()))
    }

    pub fn superuser(self, username: pg_client::Username) -> Self {
        Self {
            superuser: username,
            ..self
        }
    }

    pub fn apply_file_from_git_revision(
        self,
        name: SeedName,
        path: std::path::PathBuf,
        git_revision: impl Into<String>,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::SqlFileGitRevision {
                git_revision: git_revision.into(),
                path,
            },
        )
    }

    pub fn apply_command(
        self,
        name: SeedName,
        command: Command,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::Command { command })
    }

    pub fn apply_script(
        self,
        name: SeedName,
        script: impl Into<String>,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::Script {
                script: script.into(),
            },
        )
    }

    pub fn ssl_config(self, ssl_config: SslConfig) -> Self {
        Self {
            ssl_config: Some(ssl_config),
            ..self
        }
    }

    pub fn cross_container_access(self, enabled: bool) -> Self {
        Self {
            cross_container_access: enabled,
            ..self
        }
    }

    pub fn to_ociman_definition(&self) -> ociman::Definition {
        ociman::Definition::new(self.backend.clone(), (&self.image).into())
    }

    pub async fn with_container<T>(&self, mut action: impl AsyncFnMut(&Container) -> T) -> T {
        let loaded_seeds: Vec<LoadedSeed> = self
            .load_seeds()
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|error| panic!("{error}"));

        let mut db_container = Container::run(self);

        db_container.wait_available().await;

        for loaded_seed in &loaded_seeds {
            self.apply_loaded_seed(&db_container, loaded_seed).await
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

    async fn apply_loaded_seed(&self, db_container: &Container<'_>, loaded_seed: &LoadedSeed) {
        match loaded_seed {
            LoadedSeed::SqlFile { content, .. } => db_container.apply_sql(content).await,
            LoadedSeed::SqlFileGitRevision { content, .. } => db_container.apply_sql(content).await,
            LoadedSeed::Command { command, .. } => self.execute_command(db_container, command),
            LoadedSeed::Script { script, .. } => self.execute_script(db_container, script),
        }
    }

    pub fn apply_pg_env(cmd: &mut std::process::Command, db_container: &Container<'_>) {
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
        cmd.arg("-e");
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
        let (effective_config, mounts) = apply_ociman_mounts(client_config);

        let mut effective_arguments = vec!["--schema-only".to_string()];

        effective_arguments.extend_from_slice(extra_arguments);

        let bytes = self
            .to_ociman_definition()
            .entrypoint("pg_dump")
            .arguments(effective_arguments)
            .environment_variables(effective_config.to_pg_env())
            .mounts(mounts)
            .run_capture_only_stdout();

        crate::convert_schema(&bytes)
    }
}

pub fn apply_ociman_mounts(
    client_config: &pg_client::Config,
) -> (pg_client::Config, Vec<ociman::Mount>) {
    let owned_client_config = client_config.clone();

    match client_config.ssl_root_cert {
        Some(ref ssl_root_cert) => match ssl_root_cert {
            pg_client::SslRootCert::File(file) => {
                let host =
                    std::fs::canonicalize(file).expect("could not canonicalize ssl root path");

                let mut container_path = std::path::PathBuf::new();

                container_path.push("/pg_ephemeral");
                container_path.push(file.file_name().unwrap());

                let mounts = vec![ociman::Mount::from(format!(
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

#[cfg(test)]
mod test {
    use super::*;

    fn dummy_backend() -> ociman::backend::Backend {
        ociman::backend::Backend::Podman {
            version: semver::Version::new(0, 0, 0),
        }
    }

    #[test]
    fn test_add_seed_rejects_duplicate() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());
        let seed_name: SeedName = "test-seed".parse().unwrap();

        let definition = definition
            .add_seed(
                seed_name.clone(),
                Seed::SqlFile {
                    path: "file1.sql".into(),
                },
            )
            .unwrap();

        let result = definition.add_seed(
            seed_name.clone(),
            Seed::SqlFile {
                path: "file2.sql".into(),
            },
        );

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_add_seed_allows_different_names() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());

        let definition = definition
            .add_seed(
                "seed1".parse().unwrap(),
                Seed::SqlFile {
                    path: "file1.sql".into(),
                },
            )
            .unwrap();

        let result = definition.add_seed(
            "seed2".parse().unwrap(),
            Seed::SqlFile {
                path: "file2.sql".into(),
            },
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_file_rejects_duplicate() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());
        let seed_name: SeedName = "test-seed".parse().unwrap();

        let definition = definition
            .apply_file(seed_name.clone(), "file1.sql".into())
            .unwrap();

        let result = definition.apply_file(seed_name.clone(), "file2.sql".into());

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_command_adds_seed() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());

        let result = definition.apply_command(
            "test-command".parse().unwrap(),
            Command::new("echo", vec!["test"]),
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_command_rejects_duplicate() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());
        let seed_name: SeedName = "test-command".parse().unwrap();

        let definition = definition
            .apply_command(seed_name.clone(), Command::new("echo", vec!["test1"]))
            .unwrap();

        let result =
            definition.apply_command(seed_name.clone(), Command::new("echo", vec!["test2"]));

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_script_adds_seed() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());

        let result = definition.apply_script("test-script".parse().unwrap(), "echo test");

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_script_rejects_duplicate() {
        let definition = Definition::new(dummy_backend(), crate::Image::default());
        let seed_name: SeedName = "test-script".parse().unwrap();

        let definition = definition
            .apply_script(seed_name.clone(), "echo test1")
            .unwrap();

        let result = definition.apply_script(seed_name.clone(), "echo test2");

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }
}
