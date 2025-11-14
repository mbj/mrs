use super::InstanceName;
use crate::definition::Definition;
use crate::image::Image;
use crate::seed::{Command, Seed, SeedName};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Backend autodetection failed: {0}")]
    BackendAutodetect(#[from] cbt::backend::autodetect::Error),
    #[error("Cloud not load config file: {0}")]
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
    },
    Command {
        command: String,
        arguments: Vec<String>,
    },
    Script {
        script: String,
    },
}

impl From<SeedConfig> for Seed {
    fn from(value: SeedConfig) -> Self {
        match value {
            SeedConfig::SqlFile { path } => Seed::SqlFile(path),
            SeedConfig::Command { command, arguments } => {
                Seed::Command(Command::new(command, arguments))
            }
            SeedConfig::Script { script } => Seed::Script(script),
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
    pub backend: Option<cbt::Backend>,
    pub image: Option<Image>,
    #[serde(default)]
    pub seeds: indexmap::IndexMap<SeedName, SeedConfig>,
    pub ssl_config: Option<SslConfigDefinition>,
}

impl InstanceDefinition {
    pub fn empty() -> Self {
        Self {
            backend: None,
            image: None,
            seeds: indexmap::IndexMap::new(),
            ssl_config: None,
        }
    }

    fn definition(
        self,
        autodetect: &cbt::backend::autodetect::Lazy,
        instance_name: &InstanceName,
        defaults: &InstanceDefinition,
        overwrites: &InstanceDefinition,
    ) -> Result<Definition, Error> {
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

        let backend: cbt::Backend = match overwrites.backend.or(self.backend).or(defaults.backend) {
            Some(image) => image,
            None => match autodetect.result() {
                Ok(value) => *value,
                Err(error) => return Err(Error::BackendAutodetect(error.clone())),
            },
        };

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
            .map(|ssl_config_def| crate::definition::SslConfig::Generated {
                hostname: ssl_config_def.hostname.clone(),
            });

        Ok(Definition {
            application_name: None,
            backend,
            database: pg_client::database!("postgres"),
            migration_config: None,
            seeds,
            ssl_config,
            superuser: pg_client::username!("postgres"),
            image,
            cross_container_access: false,
        })
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    image: Option<Image>,
    backend: Option<cbt::Backend>,
    ssl_config: Option<SslConfigDefinition>,
    instances: Option<std::collections::BTreeMap<InstanceName, InstanceDefinition>>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            image: Some(Image::default()),
            backend: None,
            ssl_config: None,
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
        let autodetect = cbt::backend::autodetect::Lazy::new();

        let defaults = InstanceDefinition {
            backend: self.backend,
            image: self.image.clone(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: self.ssl_config.clone(),
        };

        match self.instances {
            None => {
                let instance_name = InstanceName::default();

                InstanceDefinition::empty()
                    .definition(&autodetect, &instance_name, &defaults, overwrites)
                    .map(|definition| [(instance_name, definition)].into())
            }
            Some(map) => {
                let mut instance_map = std::collections::BTreeMap::new();

                for (instance_name, instance_definition) in map {
                    let definition = instance_definition.definition(
                        &autodetect,
                        &instance_name,
                        &defaults,
                        overwrites,
                    )?;

                    instance_map.insert(instance_name, definition);
                }

                Ok(instance_map)
            }
        }
    }
}
