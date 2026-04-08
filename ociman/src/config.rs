use std::path::PathBuf;

use crate::backend::Selection;

#[derive(Clone, Debug)]
pub struct Config {
    pub default_backend: Selection,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_backend: Selection::Auto,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let path = Self::default_config_path();

        match std::fs::read_to_string(&path) {
            Ok(contents) => Self::load_toml(&contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(Error::IO {
                path,
                kind: error.kind(),
            }),
        }
    }

    fn load_toml(contents: &str) -> Result<Self, Error> {
        let file_config: FileConfig = toml::from_str(contents)?;
        Ok(file_config.into())
    }

    #[must_use]
    pub fn default_config_path() -> PathBuf {
        dirs::home_dir()
            .expect("HOME directory not found")
            .join(".config/ociman.toml")
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    default_backend: Option<Selection>,
}

impl From<FileConfig> for Config {
    fn from(file_config: FileConfig) -> Self {
        let default = Self::default();
        Self {
            default_backend: file_config
                .default_backend
                .unwrap_or(default.default_backend),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not load config file {path}: {kind}")]
    IO {
        path: PathBuf,
        kind: std::io::ErrorKind,
    },
    #[error("Decoding as toml failed: {0}")]
    TomlDecode(#[from] toml::de::Error),
}
