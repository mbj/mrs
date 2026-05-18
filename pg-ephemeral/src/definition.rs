use std::os::fd::FromRawFd;

use crate::Container;
use crate::seed::{
    Command, DuplicateSeedName, LoadError, LoadedSeed, LoadedSeeds, Seed, SeedCacheConfig, SeedName,
};

#[derive(Debug, thiserror::Error)]
pub enum SeedApplyError {
    #[error("Failed to apply command seed")]
    Command(#[from] cmd_proc::CommandError),
    #[error("Failed to apply SQL seed")]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    EnvVariableValue(#[from] cmd_proc::EnvVariableValueError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SslConfig {
    Generated {
        hostname: pg_client::config::HostName,
    },
    // UserProvided { ca_cert: PathBuf, server_cert: PathBuf, server_key: PathBuf },
}

/// Absolute, UTF-8 host path mirrored into a container as a bind mount.
///
/// Constructed via [`TryFrom<PathBuf>`]. Validates at construction so the
/// invariants (absolute + UTF-8) are proven by the type — downstream code
/// (mount string formatting, `--workdir` flag emission) can rely on
/// `as_str()` without re-checking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransparentWorkdir(String);

impl TransparentWorkdir {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn as_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransparentWorkdirError {
    #[error("transparent workdir path is not absolute: {0:?}")]
    NotAbsolute(std::path::PathBuf),
    #[error("transparent workdir path is not valid UTF-8: {0:?}")]
    NotUtf8(std::path::PathBuf),
}

impl TryFrom<std::path::PathBuf> for TransparentWorkdir {
    type Error = TransparentWorkdirError;

    fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
        if !path.is_absolute() {
            return Err(TransparentWorkdirError::NotAbsolute(path));
        }
        match path.into_os_string().into_string() {
            Ok(string) => Ok(Self(string)),
            Err(os_string) => Err(TransparentWorkdirError::NotUtf8(os_string.into())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub instance_name: crate::InstanceName,
    pub application_name: Option<pg_client::config::ApplicationName>,
    pub backend: ociman::Backend,
    pub database: pg_client::Database,
    pub parameters: pg_client::parameter::Map,
    pub seeds: indexmap::IndexMap<SeedName, Seed>,
    pub ssl_config: Option<SslConfig>,
    pub superuser: pg_client::User,
    pub image: crate::image::Image,
    pub cross_container_access: bool,
    pub wait_available_timeout: std::time::Duration,
    pub remove: bool,
    /// Optional user-facing identifier for the resulting container.
    ///
    /// When set, [`crate::label::apply`] emits a `pg-ephemeral.session.name`
    /// label carrying this value, and the launched container gets a
    /// deterministic OCI name (`pg-ephemeral-session-<name>`) so the runtime
    /// atomically rejects collisions with another container of the same name.
    ///
    /// Naming is independent of lifecycle. The same Definition can be named
    /// and ephemeral (driven through `with_container` with `--rm`), or named
    /// and persistent (driven through `run_detached` with no auto-remove).
    /// The label distinguishes named runs from anonymous ones; the OCI name
    /// enforces uniqueness.
    pub session_name: Option<crate::session::Name>,
    /// Host path to bind-mount into the container at the same path on launch.
    ///
    /// When set, the launched container gets a `--mount type=bind,source=<path>,target=<path>`,
    /// making the host directory accessible inside the container at the
    /// mirrored path. Powers the "transparent" CLI mode where bare
    /// commands operate on the user's cwd as if they were running locally.
    pub transparent_workdir: Option<TransparentWorkdir>,
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
            parameters: pg_client::parameter::Map::new(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
            superuser: pg_client::User::POSTGRES,
            database: pg_client::Database::POSTGRES,
            image,
            cross_container_access: false,
            wait_available_timeout: std::time::Duration::from_secs(10),
            remove: true,
            session_name: None,
            transparent_workdir: None,
        }
    }

    /// Attach a user-facing session name to this Definition.
    ///
    /// See [`Definition::session_name`] for semantics — in particular that
    /// naming is independent of lifecycle.
    #[must_use]
    pub fn session_name(self, name: crate::session::Name) -> Self {
        Self {
            session_name: Some(name),
            ..self
        }
    }

    #[must_use]
    pub fn remove(self, remove: bool) -> Self {
        Self { remove, ..self }
    }

    /// Bind-mount the given host path into the container at the same path.
    ///
    /// Used by the transparent CLI mode so the user's cwd is visible to
    /// container-side tooling at the same absolute path it has on host.
    /// The caller constructs [`TransparentWorkdir`] via `TryFrom<PathBuf>`,
    /// which validates the path is absolute and UTF-8 — invariants Definition
    /// relies on at launch / exec time.
    #[must_use]
    pub fn transparent_workdir(self, workdir: TransparentWorkdir) -> Self {
        Self {
            transparent_workdir: Some(workdir),
            ..self
        }
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
            &self.parameters,
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

    pub fn apply_sql_statement(
        self,
        name: SeedName,
        statement: impl Into<String>,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::SqlStatement {
                statement: statement.into(),
            },
        )
    }

    pub fn apply_command(
        self,
        name: SeedName,
        command: Command,
        cache: SeedCacheConfig,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(name, Seed::Command { command, cache })
    }

    pub fn apply_script(
        self,
        name: SeedName,
        script: impl Into<String>,
        cache: SeedCacheConfig,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::Script {
                script: script.into(),
                cache,
            },
        )
    }

    pub fn apply_container_script(
        self,
        name: SeedName,
        script: impl Into<String>,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::ContainerScript {
                script: script.into(),
            },
        )
    }

    pub fn apply_csv_file(
        self,
        name: SeedName,
        path: std::path::PathBuf,
        table: pg_client::QualifiedTable,
    ) -> Result<Self, DuplicateSeedName> {
        self.add_seed(
            name,
            Seed::CsvFile {
                path,
                table,
                delimiter: ',',
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
        let mut ociman_definition =
            ociman::Definition::new(self.backend.clone(), (&self.image).into());
        if let Some(session_name) = &self.session_name {
            ociman_definition = ociman_definition.container_name(session_name.container_name());
        }
        ociman_definition
    }

    pub async fn with_container<T>(
        &self,
        mut action: impl AsyncFnMut(&Container) -> T,
    ) -> Result<T, crate::container::Error> {
        let mut db_container = self.boot_and_seed().await?;
        let result = action(&db_container).await;
        db_container.stop().await?;
        Ok(result)
    }

    /// Boot a container from this definition, applying cache hits and seeds,
    /// and return the running handle without stopping it.
    ///
    /// Used by [`Self::with_container`] (which then runs an action and stops)
    /// and [`Self::start`] (which returns control to the CLI with the
    /// container left running as a detached named session).
    async fn boot_and_seed(&self) -> Result<Container, crate::container::Error> {
        let loaded_seeds = self.load_seeds(&self.instance_name).await?;
        let (last_cache_hit, uncached_seeds) = self.populate_cache(&loaded_seeds).await?;

        let boot_definition = match &last_cache_hit {
            Some(reference) => self
                .clone()
                .image(crate::image::Image::Explicit(reference.clone())),
            None => self.clone(),
        };

        let seed_entries = crate::label::build_seed_entries(self, &loaded_seeds);
        let db_container = Container::run_definition(&boot_definition, &seed_entries).await?;

        if last_cache_hit.is_some() {
            db_container
                .set_superuser_password(
                    db_container
                        .client_config
                        .session
                        .password
                        .as_ref()
                        .unwrap(),
                    self.wait_available_timeout,
                )
                .await?;
        }

        db_container
            .wait_available(self.wait_available_timeout)
            .await?;

        for seed in &uncached_seeds {
            self.apply_loaded_seed(&db_container, seed).await?;
        }

        Ok(db_container)
    }

    /// Boot a container, apply cache + seeds, and leave it running.
    ///
    /// Intended for named sessions: the caller is expected to have set
    /// [`Self::session_name`] and `.remove(false)`, so the resulting
    /// container is discoverable via [`crate::session::Session::find`] /
    /// `list` and survives until explicitly stopped.
    pub async fn start(&self) -> Result<(), crate::container::Error> {
        let _container = self.boot_and_seed().await?;
        Ok(())
    }

    /// Populate cache images for seeds.
    ///
    /// Returns a tuple of:
    /// - The last cache hit reference (if any), which can be used to boot from
    /// - The loaded seeds that could not be cached because the cache chain was broken
    pub async fn populate_cache(
        &self,
        loaded_seeds: &LoadedSeeds<'_>,
    ) -> Result<(Option<ociman::Reference>, Vec<LoadedSeed>), crate::container::Error> {
        let all_seed_entries = crate::label::build_seed_entries(self, loaded_seeds);
        let mut previous_cache_reference: Option<&ociman::Reference> = None;
        let mut seeds_iter = loaded_seeds.iter_seeds().enumerate().peekable();

        while let Some((index, seed)) = seeds_iter.next() {
            let Some(cache_reference) = seed.cache_status().reference() else {
                // Uncacheable seed - cache chain is broken, return remaining seeds
                let mut remaining = vec![seed.clone()];
                remaining.extend(seeds_iter.map(|(_, seed)| seed.clone()));
                return Ok((previous_cache_reference.cloned(), remaining));
            };

            if seed.cache_status().is_hit() {
                previous_cache_reference = Some(cache_reference);
                continue;
            }

            let caching_image = previous_cache_reference
                .map(|reference| crate::image::Image::Explicit(reference.clone()))
                .unwrap_or_else(|| self.image.clone());

            // Seeds applied at this cache image's commit point: every seed up
            // through and including the current one.
            let current = &all_seed_entries[..=index];

            if let LoadedSeed::ContainerScript { script, .. } = seed {
                log::info!("Applying container-script seed: {}", seed.name());

                let base_image: ociman::image::Reference = (&caching_image).into();
                let build_dir = create_container_script_build_dir(&base_image, script);

                ociman::image::BuildDefinition::from_directory(
                    &self.backend,
                    cache_reference.clone(),
                    &build_dir,
                )
                .build()
                .await;

                std::fs::remove_dir_all(&build_dir)
                    .expect("failed to clean up container-script build directory");
            } else {
                let caching_definition = self.clone().remove(false).image(caching_image);

                let mut container = Container::run_definition(&caching_definition, current).await?;

                if previous_cache_reference.is_some() {
                    container
                        .set_superuser_password(
                            container.client_config.session.password.as_ref().unwrap(),
                            self.wait_available_timeout,
                        )
                        .await?;
                }

                container
                    .wait_available(self.wait_available_timeout)
                    .await?;

                self.apply_loaded_seed(&container, seed).await?;
                container.stop_commit_remove(cache_reference).await?;
            }

            log::info!("Committed cache image: {cache_reference}");

            previous_cache_reference = Some(cache_reference);
        }

        Ok((previous_cache_reference.cloned(), Vec::new()))
    }

    pub async fn run_integration_server(
        &self,
        result_fd: std::os::fd::RawFd,
        control_fd: std::os::fd::RawFd,
    ) -> Result<(), crate::container::Error> {
        self.with_container(async |container| {
            // SAFETY: The parent process guarantees these are valid, exclusively-owned FDs
            // inherited via the process spawn protocol.
            let result_owned = unsafe { std::os::fd::OwnedFd::from_raw_fd(result_fd) };
            let control_owned = unsafe { std::os::fd::OwnedFd::from_raw_fd(control_fd) };

            let mut result_file = std::fs::File::from(result_owned);
            let json = serde_json::to_string(&container.client_config).unwrap();

            use std::io::Write;
            writeln!(result_file, "{json}").expect("Failed to write config to result pipe");
            drop(result_file);

            log::info!("Integration server is running, waiting for EOF on control pipe");

            let control_fd = tokio::io::unix::AsyncFd::new(control_owned)
                .expect("Failed to register control pipe with tokio");

            let _ = control_fd.readable().await.unwrap();

            log::info!("Integration server received EOF on control pipe, exiting");
        })
        .await
    }

    async fn apply_loaded_seed(
        &self,
        db_container: &Container,
        loaded_seed: &LoadedSeed,
    ) -> Result<(), SeedApplyError> {
        log::info!("Applying seed: {}", loaded_seed.name());
        match loaded_seed {
            LoadedSeed::SqlFile { content, .. } => db_container.apply_sql(content).await?,
            LoadedSeed::SqlFileGitRevision { content, .. } => {
                db_container.apply_sql(content).await?
            }
            LoadedSeed::SqlStatement { statement, .. } => db_container.apply_sql(statement).await?,
            LoadedSeed::Command { command, .. } => {
                self.execute_command(db_container, command).await?
            }
            LoadedSeed::Script { script, .. } => self.execute_script(db_container, script).await?,
            LoadedSeed::ContainerScript { script, .. } => {
                db_container.exec_container_script(script).await?
            }
            LoadedSeed::CsvFile {
                table,
                delimiter,
                content,
                ..
            } => db_container.apply_csv(table, *delimiter, content).await?,
        }

        Ok(())
    }

    async fn execute_command(
        &self,
        db_container: &Container,
        command: &Command,
    ) -> Result<(), SeedApplyError> {
        cmd_proc::Command::new(&command.command)
            .arguments(&command.arguments)
            .envs(db_container.pg_env()?)
            .env(
                &crate::ENV_DATABASE_URL,
                db_container
                    .database_url()
                    .parse::<cmd_proc::EnvVariableValue>()?,
            )
            .status()
            .await?;
        Ok(())
    }

    async fn execute_script(
        &self,
        db_container: &Container,
        script: &str,
    ) -> Result<(), SeedApplyError> {
        cmd_proc::Command::new("sh")
            .arguments(["-e", "-c"])
            .argument(script)
            .envs(db_container.pg_env()?)
            .env(
                &crate::ENV_DATABASE_URL,
                db_container
                    .database_url()
                    .parse::<cmd_proc::EnvVariableValue>()?,
            )
            .status()
            .await?;
        Ok(())
    }
}

fn create_container_script_build_dir(
    base_image: &ociman::image::Reference,
    script: &str,
) -> std::path::PathBuf {
    use rand::RngExt;

    let suffix: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let dir = std::env::temp_dir().join(format!("pg-ephemeral-build-{suffix}"));
    std::fs::create_dir(&dir).expect("failed to create container-script build directory");

    std::fs::write(dir.join("script.sh"), script).expect("failed to write container-script");

    std::fs::write(
        dir.join("Dockerfile"),
        format!("FROM {base_image}\nCOPY script.sh /tmp/pg-ephemeral-script.sh\nRUN sh -e /tmp/pg-ephemeral-script.sh && rm /tmp/pg-ephemeral-script.sh\n"),
    )
    .expect("failed to write Dockerfile");

    dir
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_backend() -> ociman::Backend {
        ociman::Backend::Podman {
            version: semver::Version::new(4, 0, 0),
            rootless: true,
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
    fn test_apply_sql_statement_adds_seed() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

        let result = definition.apply_sql_statement(
            "create-users".parse().unwrap(),
            "CREATE TABLE users (id INT)",
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
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
            SeedCacheConfig::CommandHash,
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
                SeedCacheConfig::CommandHash,
            )
            .unwrap();

        let result = definition.apply_command(
            seed_name.clone(),
            Command::new("echo", vec!["test2"]),
            SeedCacheConfig::CommandHash,
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

        let result = definition.apply_script(
            "test-script".parse().unwrap(),
            "echo test",
            SeedCacheConfig::CommandHash,
        );

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
            .apply_script(
                seed_name.clone(),
                "echo test1",
                SeedCacheConfig::CommandHash,
            )
            .unwrap();

        let result = definition.apply_script(
            seed_name.clone(),
            "echo test2",
            SeedCacheConfig::CommandHash,
        );

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    #[test]
    fn test_apply_container_script_adds_seed() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

        let result = definition.apply_container_script(
            "install-ext".parse().unwrap(),
            "apt-get update && apt-get install -y postgresql-17-cron",
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_container_script_rejects_duplicate() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
        let seed_name: SeedName = "install-ext".parse().unwrap();

        let definition = definition
            .apply_container_script(seed_name.clone(), "apt-get update")
            .unwrap();

        let result = definition.apply_container_script(seed_name.clone(), "apt-get update");

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }

    fn test_qualified_table() -> pg_client::QualifiedTable {
        pg_client::QualifiedTable {
            schema: pg_client::identifier::Schema::PUBLIC,
            table: "users".parse().unwrap(),
        }
    }

    #[test]
    fn test_apply_csv_file_adds_seed() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );

        let result = definition.apply_csv_file(
            "import-users".parse().unwrap(),
            "fixtures/users.csv".into(),
            test_qualified_table(),
        );

        assert!(result.is_ok());
        let definition = result.unwrap();
        assert_eq!(definition.seeds.len(), 1);
    }

    #[test]
    fn test_apply_csv_file_rejects_duplicate() {
        let definition = Definition::new(
            test_backend(),
            crate::Image::default(),
            test_instance_name(),
        );
        let seed_name: SeedName = "import-users".parse().unwrap();

        let definition = definition
            .apply_csv_file(
                seed_name.clone(),
                "fixtures/users.csv".into(),
                test_qualified_table(),
            )
            .unwrap();

        let result = definition.apply_csv_file(
            seed_name.clone(),
            "fixtures/other.csv".into(),
            test_qualified_table(),
        );

        assert_eq!(result, Err(DuplicateSeedName(seed_name)));
    }
}
