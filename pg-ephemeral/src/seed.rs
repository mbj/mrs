#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String")]
pub struct SeedName(String);

impl SeedName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SeedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Seed name cannot be empty")]
pub struct SeedNameError;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Duplicate seed name: {0}")]
pub struct DuplicateSeedName(pub SeedName);

impl std::str::FromStr for SeedName {
    type Err = SeedNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(SeedNameError)
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

impl TryFrom<String> for SeedName {
    type Error = SeedNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(SeedNameError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&str> for SeedName {
    type Error = SeedNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Command {
    pub fn new(
        command: impl Into<String>,
        arguments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            command: command.into(),
            arguments: arguments.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Seed {
    SqlFile {
        path: std::path::PathBuf,
    },
    SqlFileGitRevision {
        git_revision: String,
        path: std::path::PathBuf,
    },
    Command {
        command: Command,
    },
    Script {
        script: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Failed to load seed {name}: could not read file {path}: {source}")]
    FileRead {
        name: SeedName,
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[error(
        "Failed to load seed {name}: could not read {path} at git revision {git_revision}: {message}"
    )]
    GitRevision {
        name: SeedName,
        path: std::path::PathBuf,
        git_revision: String,
        message: String,
    },
    #[error("Failed to load seed {name}: path {path} is not valid UTF-8")]
    InvalidUtf8Path {
        name: SeedName,
        path: std::path::PathBuf,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoadedSeed {
    SqlFile {
        name: SeedName,
        path: std::path::PathBuf,
        content: String,
    },
    SqlFileGitRevision {
        name: SeedName,
        path: std::path::PathBuf,
        git_revision: String,
        content: String,
    },
    Command {
        name: SeedName,
        command: Command,
    },
    Script {
        name: SeedName,
        script: String,
    },
}

impl Seed {
    pub fn load(&self, name: SeedName) -> Result<LoadedSeed, LoadError> {
        match self {
            Seed::SqlFile { path } => {
                let content =
                    std::fs::read_to_string(path).map_err(|source| LoadError::FileRead {
                        name: name.clone(),
                        path: path.clone(),
                        source,
                    })?;
                Ok(LoadedSeed::SqlFile {
                    name,
                    path: path.clone(),
                    content,
                })
            }
            Seed::SqlFileGitRevision { path, git_revision } => {
                let path_str = path.to_str().ok_or_else(|| LoadError::InvalidUtf8Path {
                    name: name.clone(),
                    path: path.clone(),
                })?;

                let result = std::process::Command::new("git")
                    .arg("show")
                    .arg(format!("{git_revision}:{path_str}"))
                    .output();

                match result {
                    Ok(output) if output.status.success() => Ok(LoadedSeed::SqlFileGitRevision {
                        name,
                        path: path.clone(),
                        git_revision: git_revision.clone(),
                        content: String::from_utf8_lossy(&output.stdout).into_owned(),
                    }),
                    Ok(output) => Err(LoadError::GitRevision {
                        name,
                        path: path.clone(),
                        git_revision: git_revision.clone(),
                        message: String::from_utf8_lossy(&output.stderr).into_owned(),
                    }),
                    Err(error) => Err(LoadError::GitRevision {
                        name,
                        path: path.clone(),
                        git_revision: git_revision.clone(),
                        message: error.to_string(),
                    }),
                }
            }
            Seed::Command { command } => Ok(LoadedSeed::Command {
                name,
                command: command.clone(),
            }),
            Seed::Script { script } => Ok(LoadedSeed::Script {
                name,
                script: script.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seed_name_rejects_empty_string() {
        assert_eq!("".parse::<SeedName>(), Err(SeedNameError));
        assert_eq!(SeedName::try_from(""), Err(SeedNameError));
        assert_eq!(SeedName::try_from(String::new()), Err(SeedNameError));
    }

    #[test]
    fn test_seed_name_accepts_non_empty_string() {
        assert_eq!(
            "valid-name".parse::<SeedName>(),
            Ok(SeedName("valid-name".to_string()))
        );
        assert_eq!(
            SeedName::try_from("valid-name"),
            Ok(SeedName("valid-name".to_string()))
        );
        assert_eq!(
            SeedName::try_from("valid-name".to_string()),
            Ok(SeedName("valid-name".to_string()))
        );
    }

    #[test]
    fn test_seed_name_display() {
        let name: SeedName = "test-seed".parse().unwrap();
        assert_eq!(name.to_string(), "test-seed");
        assert_eq!(name.as_str(), "test-seed");
    }

    #[test]
    fn test_load_sql_file_git_revision_invalid_utf8_path() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let invalid_utf8_bytes = b"invalid-\xff-path.sql";
        let invalid_path = std::path::PathBuf::from(OsStr::from_bytes(invalid_utf8_bytes));

        let seed = Seed::SqlFileGitRevision {
            path: invalid_path.clone(),
            git_revision: "HEAD".to_string(),
        };

        let result = seed.load("test-seed".parse().unwrap());

        assert!(matches!(
            result,
            Err(LoadError::InvalidUtf8Path { name, path })
                if name.as_str() == "test-seed" && path == invalid_path
        ));
    }
}
