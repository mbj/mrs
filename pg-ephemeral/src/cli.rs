use crate::config::Config;
use crate::{InstanceMap, InstanceName};

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    /// Config file to use, defaults to attempt to load database.toml
    ///
    /// If absent on default location a single "main" database is assumed on
    /// autodetected backend with 17.5 postgres and no other configuration.
    #[arg(long)]
    config_file: Option<std::path::PathBuf>,
    /// Overwrite backend
    ///
    /// If not specified on the CLI and not in the config file will be autotected:
    /// first based on env variable CBT_BACKEND, than on installed tools.
    /// If the autodetection fails exits with an error.
    #[arg(long)]
    backend: Option<crate::cbt::Backend>,
    /// Overwrite image
    #[arg(long)]
    image: Option<crate::image::Image>,
    #[clap(subcommand)]
    command: Option<Command>,
}

impl App {
    pub async fn run(&self) {
        let overwrites = crate::config::InstanceDefinition {
            backend: self.backend,
            image: self.image.clone(),
        };

        let result = match &self.config_file {
            Some(config_file) => {
                Config::load_toml_file(config_file, &overwrites).map_err(|error| {
                    format!("Could not load config file specified on the CLI: {error}")
                })
            }
            None => {
                log::info!("No config file specified, trying to load from default location");

                match Config::load_toml_file("database.toml", &overwrites) {
                    Ok(value) => Ok(value),
                    Err(error) => {
                        match error {
                            crate::config::Error::IO(error)
                                if error.kind() == std::io::ErrorKind::NotFound =>
                            {
                                log::info!(
                                    "Config file does not exist in default location, using default instance map"
                                );
                                crate::Config::default().instance_map(&overwrites).map_err(
                                    |error| format!("Could not load default config file: {error}"),
                                )
                            }
                            error => Err(format!(
                                "Could not load config file specified on the CLI: {error}"
                            )),
                        }
                    }
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

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Interact with a specific instance, also the default subcommand with chosing "main"
    /// instance.
    Instance {
        /// specify instane name, defaults to "main"
        #[arg(long = "name")]
        instance_name: Option<InstanceName>,
        #[clap(subcommand)]
        command: crate::definition::cli::Command,
    },
    /// List defined instances
    List,
}

impl std::default::Default for Command {
    fn default() -> Self {
        Command::Instance {
            instance_name: None,
            command: crate::definition::cli::Command::Psql,
        }
    }
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
            Self::List => {
                for instance_name in instance_map.keys() {
                    println!("{}", instance_name)
                }
            }
        }
    }
}
