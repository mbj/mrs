pub mod cache;
pub mod platform;

use crate::config::Config;
use crate::seed::SeedName;
use crate::{InstanceMap, InstanceName};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Command(#[from] cmd_proc::CommandError),
    #[error(transparent)]
    Config(#[from] crate::config::Error),
    #[error(transparent)]
    Container(#[from] crate::container::Error),
    #[error(transparent)]
    EnvVariableValue(#[from] cmd_proc::EnvVariableValueError),
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
            parameters: pg_client::parameter::Map::new(),
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
pub enum Command {
    /// Cache related commands
    Cache {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        #[clap(subcommand)]
        command: cache::Command,
    },
    /// Run pgbench inside the container, connected via the unix socket
    #[command(name = "container-pgbench")]
    ContainerPgbench {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// Arguments forwarded to pgbench
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        arguments: Vec<String>,
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
        command: platform::Command,
    },
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
            } => command.run(instance_map, instance_name).await?,
            Self::ContainerPgbench {
                instance_name,
                arguments,
            } => {
                let definition = get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition
                    .with_container(async |container| container_pgbench(container, arguments).await)
                    .await??
            }
            Self::ContainerPsql { instance_name } => {
                let definition = get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_psql).await??
            }
            Self::ContainerSchemaDump { instance_name } => {
                let definition = get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_schema_dump).await??
            }
            Self::ContainerShell { instance_name } => {
                let definition = get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .unwrap();
                definition.with_container(container_shell).await??
            }
            Self::IntegrationServer {
                instance_name,
                result_fd,
                control_fd,
            } => {
                let definition = get_instance(instance_map, instance_name)?
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
                let definition = get_instance(instance_map, instance_name)?
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
                let definition = get_instance(instance_map, instance_name)?
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
}

#[allow(
    clippy::result_large_err,
    reason = "cli::Error aggregates container/seed errors that intentionally carry diagnostic context; the 128-byte threshold targets async-server hot paths that don't apply here"
)]
pub(super) fn get_instance<'a>(
    instance_map: &'a InstanceMap,
    instance_name: &InstanceName,
) -> Result<&'a crate::config::Instance, Error> {
    instance_map
        .get(instance_name)
        .ok_or_else(|| Error::UnknownInstance(instance_name.clone()))
}

async fn host_psql(container: &crate::container::Container) -> Result<(), Error> {
    cmd_proc::Command::new("psql")
        .envs(container.pg_env()?)
        .status()
        .await?;
    Ok(())
}

async fn host_command(
    container: &crate::container::Container,
    command: &str,
    arguments: &Vec<String>,
) -> Result<(), Error> {
    cmd_proc::Command::new(command)
        .arguments(arguments)
        .envs(container.pg_env()?)
        .env(
            &crate::ENV_DATABASE_URL,
            container
                .database_url()
                .parse::<cmd_proc::EnvVariableValue>()?,
        )
        .status()
        .await?;
    Ok(())
}

async fn container_pgbench(
    container: &crate::container::Container,
    arguments: &[String],
) -> Result<(), Error> {
    container.exec_pgbench(arguments).await?;
    Ok(())
}

async fn container_psql(container: &crate::container::Container) -> Result<(), Error> {
    container.exec_psql().await?;
    Ok(())
}

async fn container_schema_dump(container: &crate::container::Container) -> Result<(), Error> {
    let pg_schema_dump = pg_client::PgSchemaDump::new();
    println!("{}", container.exec_schema_dump(&pg_schema_dump).await?);
    Ok(())
}

async fn container_shell(container: &crate::container::Container) -> Result<(), Error> {
    container.exec_container_shell().await?;
    Ok(())
}
