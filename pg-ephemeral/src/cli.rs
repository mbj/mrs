use crate::config::Config;
use crate::{InstanceMap, InstanceName};

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
#[command(after_help = "COMMAND TYPES:
  Instance Management:
    instance, instance-list - Work with specific instances

  Main Instance Commands:
    All other commands (psql, run-env, container-*, integration-server, migration)
    target the \"main\" instance by default. Use 'instance --name <NAME>' to target
    other instances.")]
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
    /// first based on env variable CBT_BACKEND, then on installed tools.
    /// If the autodetection fails exits with an error.
    #[arg(long)]
    backend: Option<cbt::Backend>,
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
    pub async fn run(&self) {
        let overwrites = crate::config::InstanceDefinition {
            backend: self.backend,
            image: self.image.clone(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: self
                .ssl_hostname
                .clone()
                .map(|hostname| crate::config::SslConfigDefinition { hostname }),
        };

        let config_file_source =
            ConfigFileSource::from_arguments(self.config_file.clone(), self.no_config_file);

        let result = match config_file_source {
            ConfigFileSource::Explicit(config_file) => {
                Config::load_toml_file(&config_file, &overwrites).map_err(|error| {
                    format!("Could not load config file specified on the CLI: {error}")
                })
            }
            ConfigFileSource::None => {
                log::info!("--no-config-file specified, using default instance map");
                crate::Config::default()
                    .instance_map(&overwrites)
                    .map_err(|error| format!("Could not load default config: {error}"))
            }
            ConfigFileSource::Implicit => {
                log::info!("No config file specified, trying to load from default location");

                match Config::load_toml_file("database.toml", &overwrites) {
                    Ok(value) => Ok(value),
                    Err(crate::config::Error::IO(crate::config::IoError(
                        std::io::ErrorKind::NotFound,
                    ))) => {
                        log::info!(
                            "Config file does not exist in default location, using default instance map"
                        );
                        crate::Config::default()
                            .instance_map(&overwrites)
                            .map_err(|error| format!("Could not load default config: {error}"))
                    }
                    Err(error) => Err(format!(
                        "Could not load config file from default location: {error}"
                    )),
                }
            }
        };

        match result {
            Ok(instance_map) => {
                self.command
                    .clone()
                    .unwrap_or_default()
                    .run(&instance_map)
                    .await
            }
            Err(error) => panic!("{error}"),
        }
    }
}

#[derive(Clone, Debug, clap::Parser, Default)]
pub enum Command {
    /// Interact with a specific instance.
    Instance {
        /// specify instance name
        #[arg(long = "name")]
        instance_name: Option<InstanceName>,
        #[clap(subcommand)]
        command: crate::definition::cli::Command,
    },
    /// List defined instances
    InstanceList,
    /// Run interactive psql session on the container
    #[command(name = "container-psql")]
    ContainerPsql,
    /// Run schema dump from the container
    #[command(name = "container-schema-dump")]
    ContainerSchemaDump,
    /// Run interactive shell on the container
    #[command(name = "container-shell")]
    ContainerShell,
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
        /// Protocol version to use
        #[arg(long, value_enum)]
        protocol: Protocol,
    },
    /// Migration subcommands
    Migration(mmigration::cli::App),
    /// Run interactive psql on the host
    #[default]
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
    /// Check if the current platform is supported
    ///
    /// Exits with status 0 if platform is supported.
    /// Exits with status 1 if platform is not supported.
    /// This command does not require a database instance.
    #[command(name = "platform")]
    Platform,
}

impl Command {
    pub async fn run(&self, instance_map: &InstanceMap) {
        match self {
            Self::Instance {
                instance_name,
                command,
            } => {
                let instance_name = instance_name.clone().unwrap_or_default();

                match instance_map.get(&instance_name) {
                    None => panic!("Unknown instance: {instance_name}"),
                    Some(definition) => command.run(definition).await,
                }
            }
            Self::InstanceList => {
                for instance_name in instance_map.keys() {
                    println!("{}", instance_name)
                }
            }
            // Top-level commands that implicitly target the "main" instance
            Self::ContainerPsql => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::ContainerPsql,
                )
                .await
            }
            Self::ContainerSchemaDump => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::ContainerSchemaDump,
                )
                .await
            }
            Self::ContainerShell => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::ContainerShell,
                )
                .await
            }
            Self::IntegrationServer { protocol } => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::IntegrationServer {
                        protocol: protocol.clone(),
                    },
                )
                .await
            }
            Self::Migration(app) => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::Migration(app.clone()),
                )
                .await
            }
            Self::Psql => {
                self.run_on_main_instance(instance_map, crate::definition::cli::Command::Psql)
                    .await
            }
            Self::RunEnv { command, arguments } => {
                self.run_on_main_instance(
                    instance_map,
                    crate::definition::cli::Command::RunEnv {
                        command: command.clone(),
                        arguments: arguments.clone(),
                    },
                )
                .await
            }
            Self::Platform => match cbt::platform::support() {
                Ok(()) => {
                    std::process::exit(0);
                }
                Err(error) => {
                    log::info!("pg-ephemeral is not supported on this platform: {}", error);
                    std::process::exit(1);
                }
            },
        }
    }

    async fn run_on_main_instance(
        &self,
        instance_map: &InstanceMap,
        command: crate::definition::cli::Command,
    ) {
        let instance_name = InstanceName::default();

        match instance_map.get(&instance_name) {
            None => panic!("Unknown instance: {instance_name}"),
            Some(definition) => command.run(definition).await,
        }
    }
}
