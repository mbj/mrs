use crate::config::Config;
use crate::seed::{CacheStatus, SeedName};
use crate::{InstanceMap, InstanceName};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Command(#[from] cmd_proc::CommandError),
    #[error(transparent)]
    Config(#[from] crate::config::Error),
    #[error(transparent)]
    Container(#[from] crate::container::Error),
    #[error("Unknown instance: {0}")]
    UnknownInstance(InstanceName),
    #[error("Instance {instance} has no seeds; cache credentials requires a cacheable seed")]
    NoSeedsDefined { instance: InstanceName },
    #[error("Instance {instance} has no seed named {seed}")]
    UnknownSeed {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error(
        "Seed {seed} on instance {instance} is uncacheable; cache credentials requires a cacheable seed"
    )]
    SeedUncacheable {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error(
        "Seed {seed} on instance {instance} is not yet cached; run `pg-ephemeral cache populate` first"
    )]
    SeedNotCached {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error("Failed to resolve container backend for instance {instance}")]
    BackendResolve {
        instance: InstanceName,
        #[source]
        source: ociman::backend::resolve::Error,
    },
}

#[derive(Clone, Debug, Default)]
pub enum ConfigFileSource {
    #[default]
    Implicit,
    Explicit(std::path::PathBuf),
    None,
}

impl ConfigFileSource {
    fn from_arguments(config_file: Option<std::path::PathBuf>, no_config_file: bool) -> Self {
        match (config_file, no_config_file) {
            (Some(path), false) => Self::Explicit(path),
            (None, true) => Self::None,
            (None, false) => Self::Implicit,
            (Some(_), true) => unreachable!("clap conflicts_with prevents this"),
        }
    }
}

#[derive(Clone, Debug, clap::Parser)]
#[command(after_help = "INSTANCE SELECTION:
    All commands target the \"main\" instance by default.
    Use --instance <NAME> to target a different instance.")]
#[command(version = crate::VERSION_STR)]
pub struct App {
    /// Config file to use, defaults to attempt to load database.toml
    ///
    /// If absent on default location a single "main" database is assumed on
    /// autodetected backend with latest postgres and no other configuration.
    #[arg(long, conflicts_with = "no_config_file")]
    config_file: Option<std::path::PathBuf>,
    /// Do not load any config file, use default instance map
    #[arg(long, conflicts_with = "config_file")]
    no_config_file: bool,
    /// Overwrite backend
    ///
    /// If not specified on the CLI and not in the config file will be autodetected:
    /// first based on env variable OCIMAN_BACKEND, then on installed tools.
    /// If the autodetection fails exits with an error.
    #[arg(long)]
    backend: Option<ociman::backend::Selection>,
    /// Overwrite image
    #[arg(long)]
    image: Option<crate::image::Image>,
    /// Enable SSL with the specified hostname
    #[arg(long)]
    ssl_hostname: Option<pg_client::config::HostName>,
    #[clap(subcommand)]
    command: Option<Command>,
}

impl App {
    pub async fn run(&self) -> Result<(), Error> {
        let overwrites = crate::config::InstanceDefinition {
            backend: self.backend,
            image: self.image.clone(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: self
                .ssl_hostname
                .clone()
                .map(|hostname| crate::config::SslConfigDefinition { hostname }),
            wait_available_timeout: None,
        };

        let config_file_source =
            ConfigFileSource::from_arguments(self.config_file.clone(), self.no_config_file);

        let instance_map = match config_file_source {
            ConfigFileSource::Explicit(config_file) => {
                Config::load_toml_file(&config_file, &overwrites)?
            }
            ConfigFileSource::None => {
                log::debug!("--no-config-file specified, using default instance map");
                crate::Config::default().instance_map(&overwrites)?
            }
            ConfigFileSource::Implicit => {
                log::debug!("No config file specified, trying to load from default location");

                match Config::load_toml_file("database.toml", &overwrites) {
                    Ok(value) => value,
                    Err(crate::config::Error::IO(crate::config::IoError(
                        std::io::ErrorKind::NotFound,
                    ))) => {
                        log::debug!(
                            "Config file does not exist in default location, using default instance map"
                        );
                        crate::Config::default().instance_map(&overwrites)?
                    }
                    Err(error) => return Err(error.into()),
                }
            }
        };

        self.command
            .clone()
            .unwrap_or_default()
            .run(&instance_map)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum CacheCommand {
    /// Print cache status for seeds
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove cached images for the instance
    Reset {
        /// Force removal even if images are in use by stopped containers
        #[arg(long)]
        force: bool,
    },
    /// Populate cache by running seeds and committing at each cacheable point
    Populate,
    /// Print full pg-ephemeral metadata for a cached image as JSON
    Inspect {
        /// Image reference, e.g. pg-ephemeral/main:abc123...
        reference: ociman::Reference,
    },
    /// Print connection credentials baked into a cached seed image as JSON.
    ///
    /// Reads the user, database, password, and (when configured) SSL CA cert
    /// from the cached image's labels — no container is booted. Emits no
    /// host/port, since those are runtime artifacts of an actual container,
    /// not properties of the cache image.
    ///
    /// Fails when the targeted seed is uncacheable, when it is cacheable
    /// but not yet built, or when the instance has no seeds at all.
    Credentials {
        /// Seed to read credentials from. Defaults to the last declared seed
        /// in the chain.
        #[arg(long = "seed-name")]
        seed_name: Option<SeedName>,
    },
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Cache related commands
    Cache {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        #[clap(subcommand)]
        command: CacheCommand,
    },
    /// Run interactive psql session on the container
    #[command(name = "container-psql")]
    ContainerPsql {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// List defined instances
    List,
    /// Run schema dump from the container
    #[command(name = "container-schema-dump")]
    ContainerSchemaDump {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// Run interactive shell on the container
    #[command(name = "container-shell")]
    ContainerShell {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// Run integration server
    ///
    /// Intent to be used for automation with other languages wrapping pg-ephemeral.
    ///
    /// After successful boot connects to the inherited pipe file descriptors,
    /// writes a single JSON line with connection details to --result-fd,
    /// then waits for EOF on --control-fd before shutting down.
    #[command(name = "integration-server")]
    IntegrationServer {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// File descriptor for writing the result JSON
        #[arg(long)]
        result_fd: std::os::fd::RawFd,
        /// File descriptor for reading the control signal (EOF = shutdown)
        #[arg(long)]
        control_fd: std::os::fd::RawFd,
    },
    /// Run interactive psql on the host
    Psql {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// Run shell command with environment variables for PostgreSQL connection
    ///
    /// Sets all PostgreSQL-related environment variables:
    /// - libpq-style PG* environment variables (PGHOST, PGPORT, PGUSER, PGDATABASE, PGPASSWORD, PGSSLMODE, etc.)
    /// - DATABASE_URL in PostgreSQL URL format
    RunEnv {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// The command to run
        command: String,
        /// Arguments to pass to the command
        arguments: Vec<String>,
    },
    /// Platform related commands
    #[command(name = "platform")]
    Platform {
        #[clap(subcommand)]
        command: PlatformCommand,
    },
}

#[derive(Clone, Debug, clap::Parser)]
pub enum PlatformCommand {
    /// Check if the current platform is supported
    ///
    /// Exits with status 0 if platform is supported.
    /// Exits with status 1 if platform is not supported.
    Support,
    /// Trigger a panic to test backtrace quality
    ///
    /// Used by integration tests to verify that backtraces
    /// contain file paths and line numbers in release builds.
    TestBacktrace,
}

impl PlatformCommand {
    fn run(&self) {
        match self {
            Self::Support => match ociman::platform::support() {
                Ok(()) => {
                    std::process::exit(0);
                }
                Err(error) => {
                    log::info!("pg-ephemeral is not supported on this platform: {error}");
                    std::process::exit(1);
                }
            },
            Self::TestBacktrace => {
                trigger_test_panic();
            }
        }
    }
}

#[inline(never)]
fn trigger_test_panic() {
    inner_function_for_backtrace_test();
}

#[inline(never)]
fn inner_function_for_backtrace_test() {
    panic!("intentional panic for backtrace testing");
}

impl Default for Command {
    fn default() -> Self {
        Self::Psql {
            instance_name: InstanceName::default(),
        }
    }
}

impl Command {
    pub async fn run(&self, instance_map: &InstanceMap) -> Result<(), Error> {
        match self {
            Self::Cache {
                instance_name,
                command,
            } => match command {
                CacheCommand::Status { json } => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .await
                        .unwrap();
                    definition.print_cache_status(instance_name, *json).await?
                }
                CacheCommand::Reset { force } => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .await
                        .unwrap();
                    let name: ociman::reference::Name =
                        format!("pg-ephemeral/{instance_name}").parse().unwrap();
                    let references = definition.backend.image_references_by_name(&name).await;
                    for reference in &references {
                        if *force {
                            definition.backend.remove_image_force(reference).await;
                        } else {
                            definition.backend.remove_image(reference).await;
                        }
                        println!("Removed: {reference}");
                    }
                }
                CacheCommand::Populate => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .await
                        .unwrap();
                    let loaded_seeds = definition
                        .load_seeds(instance_name)
                        .await
                        .map_err(crate::container::Error::from)?;
                    definition.populate_cache(&loaded_seeds).await?;
                    definition.print_cache_status(instance_name, false).await?;
                }
                CacheCommand::Inspect { reference } => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .await
                        .unwrap();
                    let labels =
                        definition
                            .backend
                            .image_labels(reference)
                            .await
                            .map_err(|source| crate::container::Error::InspectImage {
                                reference: reference.clone(),
                                source,
                            })?;
                    let metadata =
                        crate::label::read_image(&labels).map_err(crate::container::Error::from)?;
                    let json = serde_json::to_string_pretty(&inspect_output(&metadata))
                        .map_err(crate::container::Error::SerializeImageMetadata)?;
                    println!("{json}");
                }
                CacheCommand::Credentials { seed_name } => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .await
                        .map_err(|source| Error::BackendResolve {
                            instance: instance_name.clone(),
                            source,
                        })?;
                    let loaded_seeds = definition
                        .load_seeds(instance_name)
                        .await
                        .map_err(crate::container::Error::from)?;

                    let target = match seed_name {
                        Some(name) => loaded_seeds
                            .iter_seeds()
                            .find(|seed| seed.name() == name)
                            .ok_or_else(|| Error::UnknownSeed {
                                instance: instance_name.clone(),
                                seed: name.clone(),
                            })?,
                        None => loaded_seeds.iter_seeds().last().ok_or_else(|| {
                            Error::NoSeedsDefined {
                                instance: instance_name.clone(),
                            }
                        })?,
                    };

                    let (reference, labels) = match target.cache_status() {
                        CacheStatus::Hit {
                            reference, labels, ..
                        } => (reference, labels),
                        CacheStatus::Miss { .. } => {
                            return Err(Error::SeedNotCached {
                                instance: instance_name.clone(),
                                seed: target.name().clone(),
                            });
                        }
                        CacheStatus::Uncacheable => {
                            return Err(Error::SeedUncacheable {
                                instance: instance_name.clone(),
                                seed: target.name().clone(),
                            });
                        }
                    };

                    let metadata =
                        crate::label::read_image(labels).map_err(crate::container::Error::from)?;
                    let json =
                        serde_json::to_string_pretty(&credentials_output(reference, &metadata))
                            .map_err(crate::container::Error::SerializeImageMetadata)?;
                    println!("{json}");
                }
            },
            Self::ContainerPsql { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_psql).await?
            }
            Self::ContainerSchemaDump { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_schema_dump).await?
            }
            Self::ContainerShell { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_shell).await?
            }
            Self::IntegrationServer {
                instance_name,
                result_fd,
                control_fd,
            } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition
                    .run_integration_server(*result_fd, *control_fd)
                    .await?
            }
            Self::List => {
                for instance_name in instance_map.keys() {
                    println!("{instance_name}")
                }
            }
            Self::Psql { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(host_psql).await??
            }
            Self::RunEnv {
                instance_name,
                command,
                arguments,
            } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition
                    .with_container(async |container| {
                        host_command(container, command, arguments).await
                    })
                    .await??
            }
            Self::Platform { command } => command.run(),
        }

        Ok(())
    }

    #[allow(
        clippy::result_large_err,
        reason = "cli::Error aggregates container/seed errors that intentionally carry diagnostic context; the 128-byte threshold targets async-server hot paths that don't apply here"
    )]
    fn get_instance<'a>(
        instance_map: &'a InstanceMap,
        instance_name: &InstanceName,
    ) -> Result<&'a crate::config::Instance, Error> {
        instance_map
            .get(instance_name)
            .ok_or_else(|| Error::UnknownInstance(instance_name.clone()))
    }
}

fn credentials_output(
    reference: &ociman::Reference,
    metadata: &crate::label::ImageMetadata,
) -> serde_json::Value {
    let mut superuser = serde_json::json!({
        "user": metadata.superuser.user.as_ref(),
        "database": metadata.superuser.database.as_ref(),
        "password": metadata.superuser.password.as_ref(),
    });
    if let Some(application) = metadata.superuser.application.as_ref() {
        superuser["application"] = serde_json::Value::String(application.as_ref().to_string());
    }

    let mut output = serde_json::json!({
        "cache_image": reference.to_string(),
        "superuser": superuser,
    });
    if let Some(ssl) = metadata.ssl.as_ref() {
        output["ssl"] = serde_json::json!({
            "hostname": ssl.hostname.as_str(),
            "ca_cert_pem": ssl.ca_cert_pem,
        });
    }
    output
}

fn inspect_output(metadata: &crate::label::ImageMetadata) -> serde_json::Value {
    let mut superuser = serde_json::Map::new();
    superuser.insert(
        "user".to_string(),
        serde_json::Value::String(metadata.superuser.user.as_ref().to_string()),
    );
    superuser.insert(
        "database".to_string(),
        serde_json::Value::String(metadata.superuser.database.as_ref().to_string()),
    );
    superuser.insert(
        "password".to_string(),
        serde_json::Value::String(metadata.superuser.password.as_ref().to_string()),
    );
    if let Some(application) = metadata.superuser.application.as_ref() {
        superuser.insert(
            "application".to_string(),
            serde_json::Value::String(application.as_ref().to_string()),
        );
    }

    let mut output = serde_json::Map::new();
    output.insert(
        "version".to_string(),
        serde_json::Value::String(metadata.version.to_string()),
    );
    output.insert(
        "instance".to_string(),
        serde_json::Value::String(metadata.instance.as_ref().to_string()),
    );
    output.insert(
        "image".to_string(),
        serde_json::Value::String(metadata.image.to_string()),
    );
    output.insert(
        "superuser".to_string(),
        serde_json::Value::Object(superuser),
    );
    output.insert(
        "seeds".to_string(),
        serde_json::to_value(&metadata.seeds).unwrap(),
    );
    if let Some(ssl) = metadata.ssl.as_ref() {
        let mut ssl_map = serde_json::Map::new();
        ssl_map.insert(
            "hostname".to_string(),
            serde_json::Value::String(ssl.hostname.as_str().to_string()),
        );
        ssl_map.insert(
            "ca_cert_pem".to_string(),
            serde_json::Value::String(ssl.ca_cert_pem.clone()),
        );
        output.insert("ssl".to_string(), serde_json::Value::Object(ssl_map));
    }

    serde_json::Value::Object(output)
}

async fn host_psql(container: &crate::container::Container) -> Result<(), cmd_proc::CommandError> {
    cmd_proc::Command::new("psql")
        .envs(container.pg_env())
        .status()
        .await
}

async fn host_command(
    container: &crate::container::Container,
    command: &str,
    arguments: &Vec<String>,
) -> Result<(), cmd_proc::CommandError> {
    cmd_proc::Command::new(command)
        .arguments(arguments)
        .envs(container.pg_env())
        .env(&crate::ENV_DATABASE_URL, container.database_url())
        .status()
        .await
}

async fn container_psql(container: &crate::container::Container) {
    container.exec_psql().await
}

async fn container_schema_dump(container: &crate::container::Container) {
    let pg_schema_dump = pg_client::PgSchemaDump::new();
    println!("{}", container.exec_schema_dump(&pg_schema_dump).await);
}

async fn container_shell(container: &crate::container::Container) {
    container.exec_container_shell().await
}
