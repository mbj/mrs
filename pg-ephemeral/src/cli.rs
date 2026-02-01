use crate::config::Config;
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
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Protocol {
    V0,
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
    ssl_hostname: Option<pg_client::HostName>,
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
                log::info!("--no-config-file specified, using default instance map");
                crate::Config::default().instance_map(&overwrites)?
            }
            ConfigFileSource::Implicit => {
                log::info!("No config file specified, trying to load from default location");

                match Config::load_toml_file("database.toml", &overwrites) {
                    Ok(value) => value,
                    Err(crate::config::Error::IO(crate::config::IoError(
                        std::io::ErrorKind::NotFound,
                    ))) => {
                        log::info!(
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
        /// Show full cache key output instead of truncated
        #[arg(long)]
        verbose: bool,
    },
    /// Remove cached images for the instance
    Reset,
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
    /// After successful boot this command will print a single line to stdout containing a JSON
    /// representation of the root connection details.
    ///
    /// The server will stop once stdin returns EOF, aka the parent process closed it.
    #[command(name = "integration-server")]
    IntegrationServer {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// Protocol version to use
        #[arg(long, value_enum)]
        protocol: Protocol,
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
                CacheCommand::Status { verbose } => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .unwrap();
                    definition.print_cache_status(instance_name, *verbose)?
                }
                CacheCommand::Reset => {
                    let definition = Self::get_instance(instance_map, instance_name)?
                        .definition(instance_name)
                        .unwrap();
                    let name: ociman::reference::Name =
                        format!("pg-ephemeral/{instance_name}").parse().unwrap();
                    let references = definition.backend.image_references_by_name(&name);
                    for reference in &references {
                        definition.backend.remove_image(reference);
                        println!("Removed: {reference}");
                    }
                }
            },
            Self::ContainerPsql { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .unwrap();
                definition.with_container(container_psql).await?
            }
            Self::ContainerSchemaDump { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .unwrap();
                definition.with_container(container_schema_dump).await?
            }
            Self::ContainerShell { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .unwrap();
                definition.with_container(container_shell).await?
            }
            Self::IntegrationServer {
                instance_name,
                protocol: _,
            } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .unwrap();
                definition.run_integration_server().await?
            }
            Self::List => {
                for instance_name in instance_map.keys() {
                    println!("{instance_name}")
                }
            }
            Self::Psql { instance_name } => {
                let definition = Self::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
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

    fn get_instance<'a>(
        instance_map: &'a InstanceMap,
        instance_name: &InstanceName,
    ) -> Result<&'a crate::config::Instance, Error> {
        instance_map
            .get(instance_name)
            .ok_or_else(|| Error::UnknownInstance(instance_name.clone()))
    }
}

async fn host_psql(container: &crate::container::Container) -> Result<(), cmd_proc::CommandError> {
    cmd_proc::Command::new("psql")
        .envs(container.pg_env())
        .status()
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
}

async fn container_psql(container: &crate::container::Container) {
    container.exec_psql()
}

async fn container_schema_dump(container: &crate::container::Container) {
    println!("{}", container.exec_schema_dump());
}

async fn container_shell(container: &crate::container::Container) {
    container.exec_container_shell()
}
