use crate::Container;
use crate::seed::{
    Command, CommandCacheConfig, DuplicateSeedName, LoadError, LoadedSeed, LoadedSeeds, Seed,
    SeedName,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SslConfig {
    Generated { hostname: pg_client::HostName },
    // UserProvided { ca_cert: PathBuf, server_cert: PathBuf, server_key: PathBuf },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub instance_name: crate::InstanceName,
    pub application_name: Option<pg_client::ApplicationName>,
    pub backend: ociman::Backend,
    pub database: pg_client::Database,
    pub seeds: indexmap::IndexMap<SeedName, Seed>,
    pub ssl_config: Option<SslConfig>,
    pub superuser: pg_client::User,
    pub image: crate::image::Image,
    pub cross_container_access: bool,
    pub wait_available_timeout: std::time::Duration,
    pub remove: bool,
}

impl Definition {
    #[must_use]
    pub fn new(
        backend: ociman::backend::Backend,
        image: crate::image::Image,
        instance_name: crate::InstanceName,
    ) -> Self {
        Self {
            instance_name,
            backend,
            application_name: None,
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
            superuser: pg_client::User::POSTGRES,
            database: pg_client::Database::POSTGRES,
            image,
            cross_container_access: false,
            wait_available_timeout: std::time::Duration::from_secs(10),
            remove: true,
        }
    }

    #[must_use]
    pub fn remove(self, remove: bool) -> Self {
        Self { remove, ..self }
    }

    #[must_use]
    pub fn image(self, image: crate::image::Image) -> Self {
        Self { image, ..self }
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

    pub async fn load_seeds(
        &self,
        instance_name: &crate::InstanceName,
    ) -> Result<LoadedSeeds<'_>, LoadError> {
        LoadedSeeds::load(
            &self.image,
            self.ssl_config.as_ref(),
            &self.seeds,
            &self.backend,
            instance_name,
        )
        .await
    }

    pub async fn print_cache_status(
        &self,
        instance_name: &crate::InstanceName,
        json: bool,
    ) -> Result<(), crate::container::Error> {
        let loaded_seeds = self.load_seeds(instance_name).await?;
        if json {
            loaded_seeds.print_json(instance_name);
        } else {
            loaded_seeds.print(instance_name);
        }
        Ok(())
    }

    #[must_use]
    pub fn superuser(self, user: pg_client::User) -> Self {
        Self {
            superuser: user,
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
        cache: CommandCacheConfig,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::Command { command, cache })
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

    #[must_use]
    pub fn ssl_config(self, ssl_config: SslConfig) -> Self {
        Self {
            ssl_config: Some(ssl_config),
            ..self
        }
    }

    #[must_use]
    pub fn cross_container_access(self, enabled: bool) -> Self {
        Self {
            cross_container_access: enabled,
            ..self
        }
    }

    #[must_use]
    pub fn wait_available_timeout(self, timeout: std::time::Duration) -> Self {
        Self {
            wait_available_timeout: timeout,
            ..self
        }
    }

    #[must_use]
    pub fn to_ociman_definition(&self) -> ociman::Definition {
        ociman::Definition::new(self.backend.clone(), (&self.image).into())
    }

    pub async fn with_container<T>(
        &self,
        mut action: impl AsyncFnMut(&Container) -> T,
    ) -> Result<T, crate::container::Error> {
        let (last_cache_hit, uncached_seeds) = self.populate_cache(&self.instance_name).await?;

        let boot_definition = match &last_cache_hit {
            Some(reference) => self
                .clone()
                .image(crate::image::Image::Explicit(reference.clone())),
            None => self.clone(),
        };

        let mut db_container = Container::run_definition(&boot_definition).await;

        if last_cache_hit.is_some() {
            db_container
                .set_superuser_password(db_container.client_config.password.as_ref().unwrap())
                .await?;
        }

        db_container.wait_available().await?;

        for seed in &uncached_seeds {
            self.apply_loaded_seed(&db_container, seed).await;
        }

        let result = action(&db_container).await;

        db_container.stop().await;

        Ok(result)
    }

    /// Populate cache images for seeds.
    ///
    /// Returns a tuple of:
    /// - The last cache hit reference (if any), which can be used to boot from
    /// - The loaded seeds that could not be cached because the cache chain was broken
    pub async fn populate_cache(
        &self,
        instance_name: &crate::InstanceName,
    ) -> Result<(Option<ociman::Reference>, Vec<LoadedSeed>), crate::container::Error> {
        let loaded_seeds = self.load_seeds(instance_name).await?;

        let mut previous_cache_reference: Option<&ociman::Reference> = None;
        let mut seeds_iter = loaded_seeds.iter_seeds().peekable();

        while let Some(seed) = seeds_iter.next() {
            let Some(cache_reference) = seed.cache_status().reference() else {
                // Uncacheable seed - cache chain is broken, return remaining seeds
                let mut remaining = vec![seed.clone()];
                remaining.extend(seeds_iter.cloned());
                return Ok((previous_cache_reference.cloned(), remaining));
            };

            if seed.cache_status().is_hit() {
                previous_cache_reference = Some(cache_reference);
                continue;
            }

            let caching_definition = self.clone().remove(false).image(
                previous_cache_reference
                    .map(|reference| crate::image::Image::Explicit(reference.clone()))
                    .unwrap_or_else(|| self.image.clone()),
            );

            let mut container = Container::run_definition(&caching_definition).await;

            if previous_cache_reference.is_some() {
                container
                    .set_superuser_password(container.client_config.password.as_ref().unwrap())
                    .await?;
            }

            container.wait_available().await?;

            self.apply_loaded_seed(&container, seed).await;
            container.stop_commit_remove(cache_reference).await;
            log::info!("Committed cache image: {cache_reference}");

            previous_cache_reference = Some(cache_reference);
        }

        Ok((previous_cache_reference.cloned(), Vec::new()))
    }

    pub async fn run_integration_server(&self) -> Result<(), crate::container::Error> {
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

    async fn apply_loaded_seed(&self, db_container: &Container, loaded_seed: &LoadedSeed) {
        log::info!("Applying seed: {}", loaded_seed.name());
        match loaded_seed {
            LoadedSeed::SqlFile { content, .. } => db_container.apply_sql(content).await,
            LoadedSeed::SqlFileGitRevision { content, .. } => db_container.apply_sql(content).await,
            LoadedSeed::Command { command, .. } => {
                self.execute_command(db_container, command).await
            }
            LoadedSeed::Script { script, .. } => self.execute_script(db_container, script).await,
        }
    }

    async fn execute_command(&self, db_container: &Container, command: &Command) {
        cmd_proc::Command::new(&command.command)
            .arguments(&command.arguments)
            .envs(db_container.pg_env())
            .env(&crate::ENV_DATABASE_URL, db_container.database_url())
            .status()
            .await
            .expect("Failed to execute command");
    }

    async fn execute_script(&self, db_container: &Container, script: &str) {
        cmd_proc::Command::new("sh")
            .arguments(["-e", "-c"])
            .argument(script)
            .envs(db_container.pg_env())
            .env(&crate::ENV_DATABASE_URL, db_container.database_url())
            .status()
            .await
            .expect("Failed to execute script");
    }

    pub async fn schema_dump(
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
            .run_capture_only_stdout()
            .await;

        crate::convert_schema(&bytes)
    }
}

#[must_use]
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

    fn test_backend() -> ociman::Backend {
        ociman::Backend::Podman {
            version: semver::Version::new(4, 0, 0),
        }
    }

    fn test_instance_name() -> crate::InstanceName {
        "test".parse().unwrap()
    }

    #[test]
    fn test_add_seed_rejects_duplicate() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
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
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

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
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
        let seed_name: SeedName = "test-seed".parse().unwrap();

        let definition = definition
            .apply_file(seed_name.clone(), "file1.sql".into())
            .unwrap();

        let result = definition.apply_file(seed_name.clone(), "file2.sql".into());

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_command_adds_seed() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

        let result = definition.apply_command(
            "test-command".parse().unwrap(),
            Command::new("echo", vec!["test"]),
            CommandCacheConfig::CommandHash,
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_command_rejects_duplicate() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
        let seed_name: SeedName = "test-command".parse().unwrap();

        let definition = definition
            .apply_command(
                seed_name.clone(),
                Command::new("echo", vec!["test1"]),
                CommandCacheConfig::CommandHash,
            )
            .unwrap();

        let result = definition.apply_command(
            seed_name.clone(),
            Command::new("echo", vec!["test2"]),
            CommandCacheConfig::CommandHash,
        );

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_script_adds_seed() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

        let result = definition.apply_script("test-script".parse().unwrap(), "echo test");

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_script_rejects_duplicate() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
        let seed_name: SeedName = "test-script".parse().unwrap();

        let definition = definition
            .apply_script(seed_name.clone(), "echo test1")
            .unwrap();

        let result = definition.apply_script(seed_name.clone(), "echo test2");

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }
}
