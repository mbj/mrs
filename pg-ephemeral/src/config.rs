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
    pub superuser: pg_client::User,
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
            superuser: pg_client::User::POSTGRES,
            database: pg_client::Database::POSTGRES,
            image,
            cross_container_access: false,
            wait_available_timeout: std::time::Duration::from_secs(10),
        }
    }

    pub async fn definition(
        &self,
        instance_name: &crate::InstanceName,
    ) -> Result<Definition, ociman::backend::resolve::Error> {
        Ok(Definition {
            instance_name: instance_name.clone(),
            application_name: self.application_name.clone(),
            backend: self.backend.resolve().await?,
            database: self.database.clone(),
            seeds: self.seeds.clone(),
            ssl_config: self.ssl_config.clone(),
            superuser: self.superuser.clone(),
            image: self.image.clone(),
            cross_container_access: self.cross_container_access,
            wait_available_timeout: self.wait_available_timeout,
            remove: true,
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
    ContainerScript {
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
            SeedConfig::ContainerScript { script } => Seed::ContainerScript { script },
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
            database: pg_client::Database::POSTGRES,
            seeds,
            ssl_config,
            superuser: pg_client::User::POSTGRES,
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
        let file = file.as_ref();
        let base_dir = file
            .parent()
            .map(std::path::Path::to_path_buf)
            .unwrap_or_default();

        std::fs::read_to_string(file)
            .map_err(|error| Error::IO(error.into()))
            .and_then(Self::load_toml)
            .map(|config| config.resolve_paths(&base_dir))
            .and_then(|config| config.instance_map(overwrites))
    }

    fn resolve_paths(mut self, base_dir: &std::path::Path) -> Self {
        let resolve_path = |path: std::path::PathBuf| -> std::path::PathBuf {
            if path.is_relative() {
                base_dir.join(path)
            } else {
                path
            }
        };

        // Resolve a command string if it looks like a relative file path (contains a
        // path separator). Plain command names such as "sh" or "psql" are left alone
        // so they continue to be resolved via PATH.
        let resolve_command = |command: &mut String| {
            let path = std::path::Path::new(command.as_str());
            if path.is_relative() && path.components().count() > 1 {
                // Strip leading CurDir (`.`) components so `./bin/foo` and `bin/foo`
                // both produce the same absolute result after joining.
                let stripped: std::path::PathBuf = path
                    .components()
                    .filter(|c| !matches!(c, std::path::Component::CurDir))
                    .collect();
                *command = base_dir.join(stripped).to_string_lossy().into_owned();
            }
        };

        if let Some(instances) = self.instances.as_mut() {
            for instance in instances.values_mut() {
                for seed in instance.seeds.values_mut() {
                    match seed {
                        SeedConfig::SqlFile { path, .. } => *path = resolve_path(path.clone()),
                        SeedConfig::Command { command, cache, .. } => {
                            resolve_command(command);
                            if let CommandCacheConfig::KeyCommand {
                                command: key_command,
                                ..
                            } = cache
                            {
                                resolve_command(key_command);
                            }
                        }
                        SeedConfig::Script { .. } | SeedConfig::ContainerScript { .. } => {}
                    }
                }
            }
        }

        self
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sql_file_path_resolved_relative_to_config() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-sql-file");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.schema]
                type = "sql-file"
                path = "db/structure.sql"
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "schema".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::SqlFile {
                path: dir.join("db/structure.sql"),
            }
        );
    }

    #[test]
    fn command_path_resolved_relative_to_config() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-command");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.migrate]
                type = "command"
                command = "./bin/migrate"
                arguments = ["up"]
                cache = { type = "none" }
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "migrate".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::Command {
                command: crate::seed::Command::new(
                    dir.join("bin/migrate").to_string_lossy(),
                    ["up"],
                ),
                cache: crate::seed::CommandCacheConfig::None,
            }
        );
    }

    #[test]
    fn bare_command_name_not_resolved() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-bare-command");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.schema]
                type = "command"
                command = "psql"
                arguments = ["-f", "schema.sql"]
                cache = { type = "command-hash" }
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "schema".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::Command {
                command: crate::seed::Command::new("psql", ["-f", "schema.sql"]),
                cache: crate::seed::CommandCacheConfig::CommandHash,
            }
        );
    }

    #[test]
    fn container_script_parsed() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-container-script");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.install-ext]
                type = "container-script"
                script = "apt-get update && apt-get install -y postgresql-15-cron"
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "install-ext".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::ContainerScript {
                script: "apt-get update && apt-get install -y postgresql-15-cron".to_string(),
            }
        );
    }
}
