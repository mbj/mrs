use super::InstanceName;
use crate::definition::{Definition, SslConfig};
use crate::image::Image;
use crate::seed::{Command, Seed, SeedCacheConfig, SeedName};

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    pub application_name: Option<pg_client::config::ApplicationName>,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SeedConfig {
    SqlFile {
        path: std::path::PathBuf,
        git_revision: Option<String>,
    },
    SqlStatement {
        statement: String,
    },
    Command {
        command: String,
        #[serde(default)]
        arguments: Vec<String>,
        cache: SeedCacheConfig,
    },
    Script {
        script: String,
        #[serde(default)]
        cache: Option<SeedCacheConfig>,
    },
    ContainerScript {
        script: String,
    },
    CsvFile {
        path: std::path::PathBuf,
        table: pg_client::QualifiedTable,
        delimiter: Option<char>,
    },
}

impl From<SeedConfig> for Seed {
    fn from(value: SeedConfig) -> Self {
        match value {
            SeedConfig::SqlFile { path, git_revision } => match git_revision {
                Some(git_revision) => Seed::SqlFileGitRevision { git_revision, path },
                None => Seed::SqlFile { path },
            },
            SeedConfig::SqlStatement { statement } => Seed::SqlStatement { statement },
            SeedConfig::Command {
                command,
                arguments,
                cache,
            } => Seed::Command {
                command: Command::new(command, arguments),
                cache,
            },
            SeedConfig::Script { script, cache } => Seed::Script {
                script,
                cache: cache.unwrap_or(SeedCacheConfig::CommandHash),
            },
            SeedConfig::ContainerScript { script } => Seed::ContainerScript { script },
            SeedConfig::CsvFile {
                path,
                table,
                delimiter,
            } => Seed::CsvFile {
                path,
                table,
                delimiter: delimiter.unwrap_or(','),
            },
        }
    }
}

impl From<&Seed> for SeedConfig {
    fn from(value: &Seed) -> Self {
        match value {
            Seed::SqlFile { path } => SeedConfig::SqlFile {
                path: path.clone(),
                git_revision: None,
            },
            Seed::SqlFileGitRevision { git_revision, path } => SeedConfig::SqlFile {
                path: path.clone(),
                git_revision: Some(git_revision.clone()),
            },
            Seed::SqlStatement { statement } => SeedConfig::SqlStatement {
                statement: statement.clone(),
            },
            Seed::Command { command, cache } => SeedConfig::Command {
                command: command.command.clone(),
                arguments: command.arguments.clone(),
                cache: cache.clone(),
            },
            Seed::Script { script, cache } => SeedConfig::Script {
                script: script.clone(),
                cache: Some(cache.clone()),
            },
            Seed::ContainerScript { script } => SeedConfig::ContainerScript {
                script: script.clone(),
            },
            Seed::CsvFile {
                path,
                table,
                delimiter,
            } => SeedConfig::CsvFile {
                path: path.clone(),
                table: table.clone(),
                delimiter: Some(*delimiter),
            },
        }
    }
}

#[cfg(test)]
mod from_seed_tests {
    use super::*;

    fn round_trip(config: SeedConfig) {
        let seed: Seed = config.clone().into();
        let restored: SeedConfig = (&seed).into();
        assert_eq!(restored, config);
    }

    #[test]
    fn round_trip_sql_file_no_git() {
        round_trip(SeedConfig::SqlFile {
            path: "schema.sql".into(),
            git_revision: None,
        });
    }

    #[test]
    fn round_trip_sql_file_with_git() {
        round_trip(SeedConfig::SqlFile {
            path: "schema.sql".into(),
            git_revision: Some("abc1234".to_string()),
        });
    }

    #[test]
    fn round_trip_sql_statement() {
        round_trip(SeedConfig::SqlStatement {
            statement: "CREATE TABLE t (id INT)".to_string(),
        });
    }

    #[test]
    fn round_trip_command() {
        round_trip(SeedConfig::Command {
            command: "psql".to_string(),
            arguments: vec!["-c".to_string(), "SELECT 1".to_string()],
            cache: SeedCacheConfig::CommandHash,
        });
    }

    #[test]
    fn round_trip_script_with_explicit_cache() {
        round_trip(SeedConfig::Script {
            script: "psql -c 'SELECT 1'".to_string(),
            cache: Some(SeedCacheConfig::CommandHash),
        });
    }

    #[test]
    fn script_default_cache_is_recovered_explicitly() {
        let starting = SeedConfig::Script {
            script: "x".to_string(),
            cache: None,
        };
        let seed: Seed = starting.into();
        let restored: SeedConfig = (&seed).into();
        assert_eq!(
            restored,
            SeedConfig::Script {
                script: "x".to_string(),
                cache: Some(SeedCacheConfig::CommandHash),
            }
        );
    }

    #[test]
    fn round_trip_container_script() {
        round_trip(SeedConfig::ContainerScript {
            script: "apt-get install -y foo".to_string(),
        });
    }

    #[test]
    fn round_trip_csv_file_with_delimiter() {
        round_trip(SeedConfig::CsvFile {
            path: "data.csv".into(),
            table: pg_client::QualifiedTable {
                schema: pg_client::identifier::Schema::from_static_or_panic("public"),
                table: pg_client::identifier::Table::from_static_or_panic("t"),
            },
            delimiter: Some(';'),
        });
    }

    #[test]
    fn csv_file_default_delimiter_is_recovered_explicitly() {
        let starting = SeedConfig::CsvFile {
            path: "data.csv".into(),
            table: pg_client::QualifiedTable {
                schema: pg_client::identifier::Schema::from_static_or_panic("public"),
                table: pg_client::identifier::Table::from_static_or_panic("t"),
            },
            delimiter: None,
        };
        let seed: Seed = starting.into();
        let restored: SeedConfig = (&seed).into();
        assert_eq!(
            restored,
            SeedConfig::CsvFile {
                path: "data.csv".into(),
                table: pg_client::QualifiedTable {
                schema: pg_client::identifier::Schema::from_static_or_panic("public"),
                table: pg_client::identifier::Table::from_static_or_panic("t"),
            },
                delimiter: Some(','),
            }
        );
    }
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SslConfigDefinition {
    pub hostname: pg_client::config::HostName,
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
                            if let SeedCacheConfig::KeyCommand {
                                command: key_command,
                                ..
                            } = cache
                            {
                                resolve_command(key_command);
                            }
                        }
                        SeedConfig::Script { cache, .. } => {
                            if let Some(SeedCacheConfig::KeyCommand {
                                command: key_command,
                                ..
                            }) = cache
                            {
                                resolve_command(key_command);
                            }
                        }
                        SeedConfig::CsvFile { path, .. } => *path = resolve_path(path.clone()),
                        SeedConfig::ContainerScript { .. } | SeedConfig::SqlStatement { .. } => {}
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
                cache: crate::seed::SeedCacheConfig::None,
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
                cache: crate::seed::SeedCacheConfig::CommandHash,
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

    #[test]
    fn sql_statement_parsed() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-sql-statement");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.create-users]
                type = "sql-statement"
                statement = "CREATE TABLE users (id INT)"
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "create-users".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::SqlStatement {
                statement: "CREATE TABLE users (id INT)".to_string(),
            }
        );
    }

    #[test]
    fn csv_file_parsed() {
        let dir = std::env::temp_dir().join("pg-ephemeral-config-test-csv-file");
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("database.toml");
        std::fs::write(
            &config_path,
            indoc::indoc! {r#"
                image = "15.6"

                [instances.main.seeds.users]
                type = "csv-file"
                path = "fixtures/users.csv"
                table = { schema = "public", table = "users" }
            "#},
        )
        .unwrap();

        let instance_map =
            Config::load_toml_file(&config_path, &InstanceDefinition::empty()).unwrap();

        let instance_name: crate::InstanceName = "main".parse().unwrap();
        let instance = instance_map.get(&instance_name).unwrap();
        let seed_name: crate::seed::SeedName = "users".parse().unwrap();

        assert_eq!(
            instance.seeds[&seed_name],
            crate::seed::Seed::CsvFile {
                path: dir.join("fixtures/users.csv"),
                table: pg_client::QualifiedTable {
                    schema: pg_client::identifier::Schema::PUBLIC,
                    table: "users".parse().unwrap(),
                },
                delimiter: ',',
            }
        );
    }
}
