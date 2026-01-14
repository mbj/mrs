use super::InstanceName;
use crate::definition::{Definition, SslConfig};
use crate::image::Image;
use crate::seed::{Command, CommandCacheConfig, Seed, SeedName};

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    pub application_name: Option<pg_client::ApplicationName>,
    pub backend: ociman::backend::Selection,
    pub database: pg_client::Database,
    pub seeds: indexmap::IndexMap<SeedName, Seed>,
    pub ssl_config: Option<SslConfig>,
    pub superuser: pg_client::Username,
    pub image: Image,
    pub cross_container_access: bool,
    pub wait_available_timeout: std::time::Duration,
}

impl Instance {
    #[must_use]
    pub fn new(backend: ociman::backend::Selection, image: Image) -> Self {
        Self {
            backend,
            application_name: None,
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
            superuser: pg_client::username!("postgres"),
            database: pg_client::database!("postgres"),
            image,
            cross_container_access: false,
            wait_available_timeout: std::time::Duration::from_secs(10),
        }
    }

    pub fn definition(&self) -> Result<Definition, ociman::backend::resolve::Error> {
        Ok(Definition {
            application_name: self.application_name.clone(),
            backend: self.backend.resolve()?,
            database: self.database.clone(),
            seeds: self.seeds.clone(),
            ssl_config: self.ssl_config.clone(),
            superuser: self.superuser.clone(),
            image: self.image.clone(),
            cross_container_access: self.cross_container_access,
            wait_available_timeout: self.wait_available_timeout,
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Could not load config file: {0}")]
    IO(IoError),
    #[error("Decoding as toml failed: {0}")]
    TomlDecode(#[from] toml::de::Error),
    #[error("Instance {instance_name} does not specify {field} and no default applies")]
    MissingInstanceField {
        instance_name: InstanceName,
        field: &'static str,
    },
}

#[derive(Debug, PartialEq)]
pub struct IoError(pub std::io::ErrorKind);

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::io::Error::from(self.0))
    }
}

impl std::error::Error for IoError {}

impl From<std::io::Error> for IoError {
    fn from(error: std::io::Error) -> Self {
        Self(error.kind())
    }
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SeedConfig {
    SqlFile {
        path: std::path::PathBuf,
        git_revision: Option<String>,
    },
    Command {
        command: String,
        #[serde(default)]
        arguments: Vec<String>,
        cache: CommandCacheConfig,
    },
    Script {
        script: String,
    },
}

impl From<SeedConfig> for Seed {
    fn from(value: SeedConfig) -> Self {
        match value {
            SeedConfig::SqlFile { path, git_revision } => match git_revision {
                Some(git_revision) => Seed::SqlFileGitRevision { git_revision, path },
                None => Seed::SqlFile { path },
            },
            SeedConfig::Command {
                command,
                arguments,
                cache,
            } => Seed::Command {
                command: Command::new(command, arguments),
                cache,
            },
            SeedConfig::Script { script } => Seed::Script { script },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SslConfigDefinition {
    pub hostname: pg_client::HostName,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InstanceDefinition {
    pub backend: Option<ociman::backend::Selection>,
    pub image: Option<Image>,
    #[serde(default)]
    pub seeds: indexmap::IndexMap<SeedName, SeedConfig>,
    pub ssl_config: Option<SslConfigDefinition>,
    #[serde(default, with = "humantime_serde")]
    pub wait_available_timeout: Option<std::time::Duration>,
}

impl InstanceDefinition {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            backend: None,
            image: None,
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
            wait_available_timeout: None,
        }
    }

    fn into_instance(
        self,
        instance_name: &InstanceName,
        defaults: &InstanceDefinition,
        overwrites: &InstanceDefinition,
    ) -> Result<Instance, Error> {
        let image = match overwrites
            .image
            .as_ref()
            .or(self.image.as_ref())
            .or(defaults.image.as_ref())
        {
            Some(image) => image.clone(),
            None => {
                return Err(Error::MissingInstanceField {
                    instance_name: instance_name.clone(),
                    field: "image",
                });
            }
        };

        let backend = overwrites
            .backend
            .or(self.backend)
            .or(defaults.backend)
            .unwrap_or(ociman::backend::Selection::Auto);

        let seeds = self
            .seeds
            .into_iter()
            .map(|(name, seed_config)| (name, seed_config.into()))
            .collect();

        let ssl_config = overwrites
            .ssl_config
            .as_ref()
            .or(self.ssl_config.as_ref())
            .or(defaults.ssl_config.as_ref())
            .map(|ssl_config_def| SslConfig::Generated {
                hostname: ssl_config_def.hostname.clone(),
            });

        let wait_available_timeout = overwrites
            .wait_available_timeout
            .or(self.wait_available_timeout)
            .or(defaults.wait_available_timeout)
            .unwrap_or(std::time::Duration::from_secs(10));

        Ok(Instance {
            application_name: None,
            backend,
            database: pg_client::database!("postgres"),
            seeds,
            ssl_config,
            superuser: pg_client::username!("postgres"),
            image,
            cross_container_access: false,
            wait_available_timeout,
        })
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    image: Option<Image>,
    backend: Option<ociman::backend::Selection>,
    ssl_config: Option<SslConfigDefinition>,
    #[serde(default, with = "humantime_serde")]
    wait_available_timeout: Option<std::time::Duration>,
    instances: Option<std::collections::BTreeMap<InstanceName, InstanceDefinition>>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            image: Some(Image::default()),
            backend: None,
            ssl_config: None,
            wait_available_timeout: None,
            instances: None,
        }
    }
}

impl Config {
    pub fn load_toml_file(
        file: impl AsRef<std::path::Path>,
        overwrites: &InstanceDefinition,
    ) -> Result<super::InstanceMap, Error> {
        std::fs::read_to_string(file)
            .map_err(|error| Error::IO(error.into()))
            .and_then(Self::load_toml)
            .and_then(|config| config.instance_map(overwrites))
    }

    pub fn load_toml(contents: impl AsRef<str>) -> Result<Config, Error> {
        toml::from_str(contents.as_ref()).map_err(Error::TomlDecode)
    }

    pub fn instance_map(
        self,
        overwrites: &InstanceDefinition,
    ) -> Result<super::InstanceMap, Error> {
        let defaults = InstanceDefinition {
            backend: self.backend,
            image: self.image.clone(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: self.ssl_config.clone(),
            wait_available_timeout: self.wait_available_timeout,
        };

        match self.instances {
            None => {
                let instance_name = InstanceName::default();

                InstanceDefinition::empty()
                    .into_instance(&instance_name, &defaults, overwrites)
                    .map(|instance| [(instance_name, instance)].into())
            }
            Some(map) => {
                let mut instance_map = std::collections::BTreeMap::new();

                for (instance_name, instance_definition) in map {
                    let instance =
                        instance_definition.into_instance(&instance_name, &defaults, overwrites)?;

                    instance_map.insert(instance_name, instance);
                }

                Ok(instance_map)
            }
        }
    }
}
