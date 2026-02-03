use git_proc::Build;

type CacheKey = [u8; 32];

#[derive(Clone, Debug, PartialEq)]
pub enum CacheStatus {
    Hit { reference: ociman::Reference },
    Miss { reference: ociman::Reference },
    Uncacheable,
}

impl CacheStatus {
    fn from_cache_key(
        cache_key: Option<CacheKey>,
        backend: &ociman::Backend,
        instance_name: &crate::InstanceName,
    ) -> Self {
        match cache_key {
            Some(key) => {
                let reference = format!("pg-ephemeral/{}:{}", instance_name, hex::encode(key))
                    .parse()
                    .unwrap();
                if backend.is_image_present(&reference) {
                    Self::Hit { reference }
                } else {
                    Self::Miss { reference }
                }
            }
            None => Self::Uncacheable,
        }
    }

    #[must_use]
    pub fn reference(&self) -> Option<&ociman::Reference> {
        match self {
            Self::Hit { reference } | Self::Miss { reference } => Some(reference),
            Self::Uncacheable => None,
        }
    }

    #[must_use]
    pub fn is_hit(&self) -> bool {
        matches!(self, Self::Hit { .. })
    }

    fn status_str(&self) -> &'static str {
        match self {
            Self::Hit { .. } => "hit",
            Self::Miss { .. } => "miss",
            Self::Uncacheable => "uncacheable",
        }
    }
}

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

#[derive(Clone, Debug, serde::Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CommandCacheConfig {
    /// Disable caching, breaks the cache chain
    None,
    /// Hash the command and arguments
    CommandHash,
    /// Run a command to get cache key input
    KeyCommand {
        command: String,
        #[serde(default)]
        arguments: Vec<String>,
    },
    /// Run a script to get cache key input
    KeyScript { script: String },
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
        cache: CommandCacheConfig,
    },
    Script {
        script: String,
    },
}

impl Seed {
    fn load(
        &self,
        name: SeedName,
        hash_chain: &mut HashChain,
        backend: &ociman::Backend,
        instance_name: &crate::InstanceName,
    ) -> Result<LoadedSeed, LoadError> {
        match self {
            Seed::SqlFile { path } => {
                let content =
                    std::fs::read_to_string(path).map_err(|source| LoadError::FileRead {
                        name: name.clone(),
                        path: path.clone(),
                        source,
                    })?;

                hash_chain.update(&content);

                Ok(LoadedSeed::SqlFile {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    ),
                    name,
                    path: path.clone(),
                    content,
                })
            }
            Seed::SqlFileGitRevision { path, git_revision } => {
                let output =
                    git_proc::show::new(&format!("{git_revision}:{}", path.to_str().unwrap()))
                        .build()
                        .output()
                        .map_err(|error| LoadError::GitRevision {
                            name: name.clone(),
                            path: path.clone(),
                            git_revision: git_revision.clone(),
                            message: error.to_string(),
                        })?;

                if output.success() {
                    let content = String::from_utf8(output.stdout).map_err(|error| {
                        LoadError::GitRevision {
                            name: name.clone(),
                            path: path.clone(),
                            git_revision: git_revision.clone(),
                            message: error.to_string(),
                        }
                    })?;

                    hash_chain.update(&content);

                    Ok(LoadedSeed::SqlFileGitRevision {
                        cache_status: CacheStatus::from_cache_key(
                            hash_chain.cache_key(),
                            backend,
                            instance_name,
                        ),
                        name,
                        path: path.clone(),
                        git_revision: git_revision.clone(),
                        content,
                    })
                } else {
                    let message = String::from_utf8(output.stderr).map_err(|error| {
                        LoadError::GitRevision {
                            name: name.clone(),
                            path: path.clone(),
                            git_revision: git_revision.clone(),
                            message: error.to_string(),
                        }
                    })?;
                    Err(LoadError::GitRevision {
                        name,
                        path: path.clone(),
                        git_revision: git_revision.clone(),
                        message,
                    })
                }
            }
            Seed::Command { command, cache } => {
                let cache_key_output = match cache {
                    CommandCacheConfig::None => {
                        hash_chain.stop();
                        None
                    }
                    CommandCacheConfig::CommandHash => {
                        hash_chain.update(&command.command);
                        for argument in &command.arguments {
                            hash_chain.update(argument);
                        }
                        None
                    }
                    CommandCacheConfig::KeyCommand {
                        command: key_command,
                        arguments: key_arguments,
                    } => {
                        let output = cmd_proc::Command::new(key_command)
                            .arguments(key_arguments)
                            .output()
                            .map_err(|error| LoadError::KeyCommand {
                                name: name.clone(),
                                command: key_command.clone(),
                                message: error.to_string(),
                            })?;

                        if output.success() {
                            hash_chain.update(&output.stdout);
                            Some(output.stdout)
                        } else {
                            let message = String::from_utf8(output.stderr).map_err(|error| {
                                LoadError::KeyCommand {
                                    name: name.clone(),
                                    command: key_command.clone(),
                                    message: error.to_string(),
                                }
                            })?;
                            return Err(LoadError::KeyCommand {
                                name,
                                command: key_command.clone(),
                                message,
                            });
                        }
                    }
                    CommandCacheConfig::KeyScript { script: key_script } => {
                        let output = cmd_proc::Command::new("sh")
                            .arguments(["-e", "-c"])
                            .argument(key_script)
                            .output()
                            .map_err(|error| LoadError::KeyScript {
                                name: name.clone(),
                                message: error.to_string(),
                            })?;

                        if output.success() {
                            hash_chain.update(&output.stdout);
                            Some(output.stdout)
                        } else {
                            let message = String::from_utf8(output.stderr).map_err(|error| {
                                LoadError::KeyScript {
                                    name: name.clone(),
                                    message: error.to_string(),
                                }
                            })?;
                            return Err(LoadError::KeyScript { name, message });
                        }
                    }
                };

                Ok(LoadedSeed::Command {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    ),
                    cache_key_output,
                    name,
                    command: command.clone(),
                })
            }
            Seed::Script { script } => {
                hash_chain.update(script);

                Ok(LoadedSeed::Script {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    ),
                    name,
                    script: script.clone(),
                })
            }
        }
    }
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
    #[error("Failed to load seed {name}: cache key command {command} failed: {message}")]
    KeyCommand {
        name: SeedName,
        command: String,
        message: String,
    },
    #[error("Failed to load seed {name}: cache key script failed: {message}")]
    KeyScript { name: SeedName, message: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoadedSeed {
    SqlFile {
        cache_status: CacheStatus,
        name: SeedName,
        path: std::path::PathBuf,
        content: String,
    },
    SqlFileGitRevision {
        cache_status: CacheStatus,
        name: SeedName,
        path: std::path::PathBuf,
        git_revision: String,
        content: String,
    },
    Command {
        cache_status: CacheStatus,
        cache_key_output: Option<Vec<u8>>,
        name: SeedName,
        command: Command,
    },
    Script {
        cache_status: CacheStatus,
        name: SeedName,
        script: String,
    },
}

impl LoadedSeed {
    #[must_use]
    pub fn cache_status(&self) -> &CacheStatus {
        match self {
            Self::SqlFile { cache_status, .. } => cache_status,
            Self::SqlFileGitRevision { cache_status, .. } => cache_status,
            Self::Command { cache_status, .. } => cache_status,
            Self::Script { cache_status, .. } => cache_status,
        }
    }

    #[must_use]
    pub fn name(&self) -> &SeedName {
        match self {
            Self::SqlFile { name, .. } => name,
            Self::SqlFileGitRevision { name, .. } => name,
            Self::Command { name, .. } => name,
            Self::Script { name, .. } => name,
        }
    }

    fn variant_name(&self) -> &'static str {
        match self {
            Self::SqlFile { .. } => "sql-file",
            Self::SqlFileGitRevision { .. } => "sql-file-git-revision",
            Self::Command { .. } => "command",
            Self::Script { .. } => "script",
        }
    }

    fn cache_key_output(&self) -> Option<&[u8]> {
        match self {
            Self::Command {
                cache_key_output, ..
            } => cache_key_output.as_deref(),
            _ => None,
        }
    }
}

fn format_cache_key_output(output: &[u8], verbose: bool) -> String {
    match std::str::from_utf8(output) {
        Ok(text) if verbose => text.to_string(),
        Ok(text) => {
            let first_line = text.lines().next().unwrap_or("");
            let line_count = text.lines().count();
            if line_count > 1 {
                format!("{first_line} [...{} more lines]", line_count - 1)
            } else {
                first_line.to_string()
            }
        }
        Err(_) if verbose => hex::encode(output),
        Err(_) => {
            let hex_string = hex::encode(output);
            if hex_string.len() > 256 {
                format!(
                    "{}... [{} more bytes]",
                    &hex_string[..256],
                    output.len() - 128
                )
            } else {
                hex_string
            }
        }
    }
}

struct HashChain {
    hasher: Option<sha2::Sha256>,
}

impl HashChain {
    fn new() -> Self {
        use sha2::Digest;

        Self {
            hasher: Some(sha2::Sha256::new()),
        }
    }

    fn update(&mut self, bytes: impl AsRef<[u8]>) {
        use sha2::Digest;

        if let Some(ref mut hasher) = self.hasher {
            hasher.update(bytes)
        }
    }

    fn cache_key(&self) -> Option<CacheKey> {
        use sha2::Digest;

        self.hasher
            .as_ref()
            .map(|hasher| hasher.clone().finalize().into())
    }

    fn stop(&mut self) {
        self.hasher = None
    }
}

#[derive(Debug, PartialEq)]
pub struct LoadedSeeds<'a> {
    image: &'a crate::image::Image,
    seeds: Vec<LoadedSeed>,
}

impl<'a> LoadedSeeds<'a> {
    pub fn load(
        image: &'a crate::image::Image,
        ssl_config: Option<&crate::definition::SslConfig>,
        seeds: &indexmap::IndexMap<SeedName, Seed>,
        backend: &ociman::Backend,
        instance_name: &crate::InstanceName,
    ) -> Result<Self, LoadError> {
        let mut hash_chain = HashChain::new();
        let mut loaded_seeds = Vec::new();

        hash_chain.update(crate::VERSION_STR);
        hash_chain.update(image.to_string());

        match ssl_config {
            Some(crate::definition::SslConfig::Generated { hostname }) => {
                hash_chain.update("ssl:generated:");
                hash_chain.update(hostname.as_str());
            }
            None => {
                hash_chain.update("ssl:none");
            }
        }

        for (name, seed) in seeds {
            let loaded_seed = seed.load(name.clone(), &mut hash_chain, backend, instance_name)?;
            loaded_seeds.push(loaded_seed);
        }

        Ok(Self {
            image,
            seeds: loaded_seeds,
        })
    }

    pub fn iter_seeds(&self) -> impl Iterator<Item = &LoadedSeed> {
        self.seeds.iter()
    }

    pub fn print(&self, verbose: bool) {
        #[derive(serde::Serialize)]
        struct Output {
            version: String,
            image: String,
            seeds: Vec<SeedOutput>,
        }

        #[derive(serde::Serialize)]
        struct SeedOutput {
            name: String,
            #[serde(rename = "type")]
            variant: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            cache_image: Option<String>,
            status: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            cache_key_output: Option<String>,
        }

        let output = Output {
            version: crate::VERSION_STR.to_string(),
            image: self.image.to_string(),
            seeds: self
                .seeds
                .iter()
                .map(|seed| SeedOutput {
                    cache_image: seed
                        .cache_status()
                        .reference()
                        .map(|reference| reference.to_string()),
                    status: seed.cache_status().status_str().to_string(),
                    cache_key_output: seed
                        .cache_key_output()
                        .map(|output| format_cache_key_output(output, verbose)),
                    name: seed.name().to_string(),
                    variant: seed.variant_name().to_string(),
                })
                .collect(),
        };

        print!("{}", toml::to_string_pretty(&output).unwrap());
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
    fn test_cache_status_uncacheable() {
        let loaded_seed = LoadedSeed::Command {
            cache_status: CacheStatus::Uncacheable,
            cache_key_output: None,
            name: "run-migrations".parse().unwrap(),
            command: Command::new("migrate", ["up"]),
        };

        assert!(loaded_seed.cache_status().reference().is_none());
        assert!(!loaded_seed.cache_status().is_hit());
    }

    #[test]
    fn test_cache_status_miss() {
        let reference: ociman::Reference =
            "pg-ephemeral/main:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        let loaded_seed = LoadedSeed::SqlFile {
            cache_status: CacheStatus::Miss {
                reference: reference.clone(),
            },
            name: "schema".parse().unwrap(),
            path: "schema.sql".into(),
            content: "CREATE TABLE test();".to_string(),
        };

        assert_eq!(loaded_seed.cache_status().reference(), Some(&reference));
        assert!(!loaded_seed.cache_status().is_hit());
    }

    #[test]
    fn test_cache_status_hit() {
        let reference: ociman::Reference =
            "pg-ephemeral/main:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        let loaded_seed = LoadedSeed::SqlFile {
            cache_status: CacheStatus::Hit {
                reference: reference.clone(),
            },
            name: "schema".parse().unwrap(),
            path: "schema.sql".into(),
            content: "CREATE TABLE test();".to_string(),
        };

        assert_eq!(loaded_seed.cache_status().reference(), Some(&reference));
        assert!(loaded_seed.cache_status().is_hit());
    }
}
