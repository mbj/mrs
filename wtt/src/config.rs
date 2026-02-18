use std::path::PathBuf;

use crate::Branch;

#[derive(Clone, Debug)]
pub struct Config {
    pub base_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().expect("HOME directory not found");
        Self {
            base_dir: home.join("devel"),
        }
    }
}

impl Config {
    #[must_use]
    pub fn repo_path(&self, repo: &crate::RepoName) -> PathBuf {
        self.base_dir.join(repo.as_str())
    }

    #[must_use]
    pub fn worktree_path(&self, repo: &crate::RepoName, branch: &Branch) -> PathBuf {
        self.repo_path(repo).join(branch.as_str())
    }

    pub fn load(source: &Source) -> Result<Self, Error> {
        match source {
            Source::None => Ok(Self::default()),
            Source::File(path) => Self::load_file(path),
            Source::Implicit => Self::load_implicit(),
        }
    }

    fn load_file(path: &PathBuf) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path).map_err(|error| Error::IO {
            path: path.clone(),
            kind: error.kind(),
        })?;
        Self::load_toml(&contents)
    }

    fn load_implicit() -> Result<Self, Error> {
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
            .join(".config/wtt.toml")
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    base_dir: Option<PathBuf>,
}

impl From<FileConfig> for Config {
    fn from(file_config: FileConfig) -> Self {
        let default = Self::default();
        Self {
            base_dir: file_config.base_dir.unwrap_or(default.base_dir),
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

pub enum Source {
    None,
    File(PathBuf),
    Implicit,
}
