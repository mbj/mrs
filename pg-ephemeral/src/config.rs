use super::InstanceName;
use crate::cbt;
use crate::definition::Definition;
use crate::definition::version::{Major, Minor, Version};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Backend autodetection failed: {0}")]
    BackendAutodetect(crate::cbt::backend::autodetect::Error),
    #[error("Cloud not load config file: {0}")]
    IO(std::io::Error),
    #[error("Decoding as toml failed: {0}")]
    TomlDecode(toml::de::Error),
    #[error("Instance {instance_name} does not specify {field} and no default applies")]
    MissingInstanceField {
        instance_name: InstanceName,
        field: &'static str,
    },
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceDefinition {
    pub backend: Option<cbt::Backend>,
    pub version: Option<Version>,
}

impl InstanceDefinition {
    pub fn empty() -> Self {
        Self {
            backend: None,
            version: None,
        }
    }

    fn definition(
        &self,
        autodetect: &cbt::backend::autodetect::Lazy,
        instance_name: &InstanceName,
        defaults: &InstanceDefinition,
        overwrites: &InstanceDefinition,
    ) -> Result<Definition, Error> {
        let version = match overwrites
            .version
            .as_ref()
            .or(self.version.as_ref())
            .or(defaults.version.as_ref())
        {
            Some(version) => version.clone(),
            None => {
                return Err(Error::MissingInstanceField {
                    instance_name: instance_name.clone(),
                    field: "version",
                });
            }
        };

        let backend: cbt::Backend = match overwrites.backend.or(self.backend).or(defaults.backend) {
            Some(version) => version,
            None => match autodetect.result() {
                Ok(value) => *value,
                Err(error) => return Err(Error::BackendAutodetect(error.clone())),
            },
        };

        Ok(Definition {
            application_name: None,
            backend,
            database: pg_client::database!("postgres"),
            migration_config: None,
            steps: vec![],
            superuser: pg_client::username!("postgres"),
            version,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    version: Option<Version>,
    backend: Option<cbt::Backend>,
    instances: Option<std::collections::BTreeMap<InstanceName, InstanceDefinition>>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: Some(Version::new(Major::new(17), Minor::new(6))),
            backend: None,
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
            .map_err(Error::IO)
            .and_then(Self::load_toml)
            .and_then(|config| config.instance_map(overwrites))
    }

    pub fn load_toml(contents: impl AsRef<str>) -> Result<Config, Error> {
        toml::from_str(contents.as_ref()).map_err(Error::TomlDecode)
    }

    pub fn instance_map(
        &self,
        overwrites: &InstanceDefinition,
    ) -> Result<super::InstanceMap, Error> {
        let autodetect = crate::cbt::backend::autodetect::Lazy::new();

        let defaults = InstanceDefinition {
            version: self.version.clone(),
            backend: self.backend,
        };

        match &self.instances {
            None => {
                let instance_name = InstanceName::default();

                defaults
                    .definition(&autodetect, &instance_name, &defaults, overwrites)
                    .map(|definition| [(instance_name, definition)].into())
            }
            Some(map) => {
                let mut instance_map = std::collections::BTreeMap::new();

                for (instance_name, instance_definition) in map {
                    let definition = instance_definition.definition(
                        &autodetect,
                        instance_name,
                        &defaults,
                        overwrites,
                    )?;

                    instance_map.insert(instance_name.clone(), definition);
                }

                Ok(instance_map)
            }
        }
    }
}
