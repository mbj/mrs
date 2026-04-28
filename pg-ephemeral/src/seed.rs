use git_proc::Build;

/// Content-addressed cache hash for a seed.
///
/// Wraps the SHA-256 digest produced by [`HashChain`] so the type expresses
/// "seed hash", not "32 raw bytes". Round-trips through hex strings via
/// [`std::fmt::Display`] / [`std::str::FromStr`] / serde.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeedHash(sha2::digest::Output<sha2::Sha256>);

impl SeedHash {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<sha2::digest::Output<sha2::Sha256>> for SeedHash {
    fn from(digest: sha2::digest::Output<sha2::Sha256>) -> Self {
        Self(digest)
    }
}

impl std::fmt::Display for SeedHash {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&hex::encode(self.0))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SeedHashError {
    #[error("expected 64 hex characters, got {0}")]
    InvalidLength(usize),
    #[error("invalid hex character")]
    InvalidHex,
}

impl std::str::FromStr for SeedHash {
    type Err = SeedHashError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_len = input.len();
        let decoded = hex::decode(input).map_err(|_| SeedHashError::InvalidHex)?;
        sha2::digest::Output::<sha2::Sha256>::try_from(decoded.as_slice())
            .map(Self)
            .map_err(|_| SeedHashError::InvalidLength(input_len))
    }
}

impl serde::Serialize for SeedHash {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> serde::Deserialize<'de> for SeedHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        raw.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod seed_hash_tests {
    use super::*;

    /// SHA-256 of the empty input (well-known constant).
    const EMPTY_DIGEST_HEX: &str =
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    fn empty_seed_hash() -> SeedHash {
        use sha2::Digest;
        sha2::Sha256::new().finalize().into()
    }

    #[test]
    fn from_digest_via_into() {
        // Confirms the From<Output<Sha256>> impl works through .into().
        let hash = empty_seed_hash();
        assert_eq!(hash.to_string(), EMPTY_DIGEST_HEX);
    }

    #[test]
    fn display_is_lowercase_hex() {
        let hash = empty_seed_hash();
        let rendered = hash.to_string();
        assert_eq!(rendered.len(), 64);
        assert!(
            rendered
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
        assert_eq!(rendered, EMPTY_DIGEST_HEX);
    }

    #[test]
    fn from_str_round_trip() {
        let parsed: SeedHash = EMPTY_DIGEST_HEX.parse().unwrap();
        assert_eq!(parsed.to_string(), EMPTY_DIGEST_HEX);
        assert_eq!(parsed, empty_seed_hash());
    }

    #[test]
    fn json_round_trip() {
        let hash = empty_seed_hash();
        let json = serde_json::to_string(&hash).unwrap();
        assert_eq!(json, format!("\"{EMPTY_DIGEST_HEX}\""));
        let parsed: SeedHash = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, hash);
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(
            "".parse::<SeedHash>(),
            Err(SeedHashError::InvalidLength(0))
        ));
    }

    #[test]
    fn rejects_short_even_length() {
        assert!(matches!(
            "abcd".parse::<SeedHash>(),
            Err(SeedHashError::InvalidLength(4))
        ));
    }

    #[test]
    fn rejects_long_even_length() {
        let oversized = "a".repeat(66);
        assert!(matches!(
            oversized.parse::<SeedHash>(),
            Err(SeedHashError::InvalidLength(66))
        ));
    }

    #[test]
    fn rejects_odd_length() {
        // Odd length fails the hex decode before the length check applies.
        assert!(matches!(
            "abc".parse::<SeedHash>(),
            Err(SeedHashError::InvalidHex)
        ));
    }

    #[test]
    fn rejects_non_hex_character() {
        let mut bad = EMPTY_DIGEST_HEX.to_string();
        bad.replace_range(0..1, "z");
        assert!(matches!(
            bad.parse::<SeedHash>(),
            Err(SeedHashError::InvalidHex)
        ));
    }

    #[test]
    fn rejects_uppercase_hex_via_serde_round_trip() {
        // FromStr allows mixed case (hex::decode accepts both); Display always
        // emits lowercase. Round-trip through Display normalises.
        let upper = EMPTY_DIGEST_HEX.to_uppercase();
        let parsed: SeedHash = upper.parse().unwrap();
        assert_eq!(parsed.to_string(), EMPTY_DIGEST_HEX);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CacheStatus {
    Hit {
        hash: SeedHash,
        reference: ociman::Reference,
    },
    Miss {
        hash: SeedHash,
        reference: ociman::Reference,
    },
    Uncacheable,
}

impl CacheStatus {
    async fn from_cache_key(
        cache_key: Option<SeedHash>,
        backend: &ociman::Backend,
        instance_name: &crate::InstanceName,
    ) -> Result<Self, LoadError> {
        let Some(hash) = cache_key else {
            return Ok(Self::Uncacheable);
        };
        let reference: ociman::Reference = format!("pg-ephemeral/{instance_name}:{hash}")
            .parse()
            .unwrap();
        match backend.is_image_present(&reference).await {
            Ok(true) => Ok(Self::Hit { hash, reference }),
            Ok(false) => Ok(Self::Miss { hash, reference }),
            Err(source) => Err(LoadError::CacheImagePresent { reference, source }),
        }
    }

    #[must_use]
    pub fn reference(&self) -> Option<&ociman::Reference> {
        match self {
            Self::Hit { reference, .. } | Self::Miss { reference, .. } => Some(reference),
            Self::Uncacheable => None,
        }
    }

    #[must_use]
    pub fn hash(&self) -> Option<&SeedHash> {
        match self {
            Self::Hit { hash, .. } | Self::Miss { hash, .. } => Some(hash),
            Self::Uncacheable => None,
        }
    }

    #[must_use]
    pub fn is_hit(&self) -> bool {
        matches!(self, Self::Hit { .. })
    }

    #[must_use]
    pub fn status_str(&self) -> &'static str {
        match self {
            Self::Hit { .. } => "hit",
            Self::Miss { .. } => "miss",
            Self::Uncacheable => "uncacheable",
        }
    }
}

/// Maximum length of a seed name in bytes.
pub const SEED_NAME_MAX_LENGTH: usize = 63;

/// Error parsing a seed name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedNameError {
    /// Seed name cannot be empty.
    Empty,
    /// Seed name exceeds maximum length.
    TooLong,
    /// Seed name contains an invalid character.
    InvalidCharacter,
    /// Seed name starts with a dash.
    StartsWithDash,
    /// Seed name ends with a dash.
    EndsWithDash,
}

impl SeedNameError {
    #[must_use]
    const fn message(&self) -> &'static str {
        match self {
            Self::Empty => "seed name cannot be empty",
            Self::TooLong => "seed name exceeds maximum length of 63 bytes",
            Self::InvalidCharacter => {
                "seed name must contain only lowercase ASCII alphanumeric characters or dashes"
            }
            Self::StartsWithDash => "seed name cannot start with a dash",
            Self::EndsWithDash => "seed name cannot end with a dash",
        }
    }
}

impl std::fmt::Display for SeedNameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message())
    }
}

impl std::error::Error for SeedNameError {}

const fn validate_seed_name(input: &str) -> Option<SeedNameError> {
    let bytes = input.as_bytes();

    if bytes.is_empty() {
        return Some(SeedNameError::Empty);
    }

    if bytes.len() > SEED_NAME_MAX_LENGTH {
        return Some(SeedNameError::TooLong);
    }

    if bytes[0] == b'-' {
        return Some(SeedNameError::StartsWithDash);
    }

    if bytes[bytes.len() - 1] == b'-' {
        return Some(SeedNameError::EndsWithDash);
    }

    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
            return Some(SeedNameError::InvalidCharacter);
        }
        index += 1;
    }

    None
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct SeedName(std::borrow::Cow<'static, str>);

impl SeedName {
    /// Creates a new seed name from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty, exceeds [`SEED_NAME_MAX_LENGTH`],
    /// contains non-lowercase-alphanumeric/dash characters,
    /// or starts/ends with a dash.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_seed_name(input) {
            Some(error) => panic!("{}", error.message()),
            None => Self(std::borrow::Cow::Borrowed(input)),
        }
    }

    /// Returns the seed name as a string slice.
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

impl AsRef<str> for SeedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Duplicate seed name: {0}")]
pub struct DuplicateSeedName(pub SeedName);

impl std::str::FromStr for SeedName {
    type Err = SeedNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match validate_seed_name(value) {
            Some(error) => Err(error),
            None => Ok(Self(std::borrow::Cow::Owned(value.to_owned()))),
        }
    }
}

impl TryFrom<String> for SeedName {
    type Error = SeedNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match validate_seed_name(&value) {
            Some(error) => Err(error),
            None => Ok(Self(std::borrow::Cow::Owned(value))),
        }
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SeedCacheConfig {
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
    SqlStatement {
        statement: String,
    },
    Command {
        command: Command,
        cache: SeedCacheConfig,
    },
    Script {
        script: String,
        cache: SeedCacheConfig,
    },
    ContainerScript {
        script: String,
    },
    CsvFile {
        path: std::path::PathBuf,
        table: pg_client::QualifiedTable,
        delimiter: char,
    },
}

/// Apply a cache strategy to the hash chain for a cacheable seed.
///
/// Always folds the seed's intrinsic `base` chunks (the command + arguments, or the
/// script body) into the chain first, then layers the cache strategy on top:
/// - `None`: stops the chain; nothing further cacheable.
/// - `CommandHash`: `base` alone drives the cache key.
/// - `KeyCommand` / `KeyScript`: runs the configured command/script and folds its
///   stdout into the chain, adding an external cache key input alongside `base`.
///
/// stderr from the key command/script is inherited so users see it live.
async fn apply_cache_config(
    cache: &SeedCacheConfig,
    hash_chain: &mut HashChain,
    name: &SeedName,
    base: &[&[u8]],
) -> Result<(), LoadError> {
    match cache {
        SeedCacheConfig::None => {
            hash_chain.stop();
            Ok(())
        }
        SeedCacheConfig::CommandHash => {
            for chunk in base {
                hash_chain.update(chunk);
            }
            Ok(())
        }
        SeedCacheConfig::KeyCommand {
            command: key_command,
            arguments: key_arguments,
        } => {
            for chunk in base {
                hash_chain.update(chunk);
            }

            let output = cmd_proc::Command::new(key_command)
                .arguments(key_arguments)
                .stdout_capture()
                .run()
                .await
                .map_err(|source| LoadError::KeyCommand {
                    name: name.clone(),
                    command: key_command.clone(),
                    source,
                })?;

            hash_chain.update(&output.bytes);
            Ok(())
        }
        SeedCacheConfig::KeyScript { script: key_script } => {
            for chunk in base {
                hash_chain.update(chunk);
            }

            let output = cmd_proc::Command::new("sh")
                .arguments(["-e", "-c"])
                .argument(key_script)
                .stdout_capture()
                .run()
                .await
                .map_err(|source| LoadError::KeyScript {
                    name: name.clone(),
                    source,
                })?;

            hash_chain.update(&output.bytes);
            Ok(())
        }
    }
}

impl Seed {
    async fn load(
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
                    )
                    .await?,
                    name,
                    path: path.clone(),
                    content,
                })
            }
            Seed::SqlFileGitRevision { path, git_revision } => {
                let output =
                    git_proc::show::new(&format!("{git_revision}:{}", path.to_str().unwrap()))
                        .build()
                        .stdout_capture()
                        .stderr_capture()
                        .accept_nonzero_exit()
                        .run()
                        .await
                        .map_err(|error| LoadError::GitRevision {
                            name: name.clone(),
                            path: path.clone(),
                            git_revision: git_revision.clone(),
                            message: error.to_string(),
                        })?;

                if output.status.success() {
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
                        )
                        .await?,
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
            Seed::SqlStatement { statement } => {
                hash_chain.update(statement);

                Ok(LoadedSeed::SqlStatement {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    )
                    .await?,
                    name,
                    statement: statement.clone(),
                })
            }
            Seed::Command { command, cache } => {
                let mut base: Vec<&[u8]> = Vec::with_capacity(1 + command.arguments.len());
                base.push(command.command.as_bytes());
                for argument in &command.arguments {
                    base.push(argument.as_bytes());
                }
                apply_cache_config(cache, hash_chain, &name, &base).await?;

                Ok(LoadedSeed::Command {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    )
                    .await?,
                    name,
                    command: command.clone(),
                })
            }
            Seed::Script { script, cache } => {
                apply_cache_config(cache, hash_chain, &name, &[script.as_bytes()]).await?;

                Ok(LoadedSeed::Script {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    )
                    .await?,
                    name,
                    script: script.clone(),
                })
            }
            Seed::ContainerScript { script } => {
                hash_chain.update(script);

                Ok(LoadedSeed::ContainerScript {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    )
                    .await?,
                    name,
                    script: script.clone(),
                })
            }
            Seed::CsvFile {
                path,
                table,
                delimiter,
            } => {
                let content =
                    std::fs::read_to_string(path).map_err(|source| LoadError::FileRead {
                        name: name.clone(),
                        path: path.clone(),
                        source,
                    })?;

                hash_chain.update(table.schema.as_ref());
                hash_chain.update(table.table.as_ref());
                hash_chain.update(&content);

                Ok(LoadedSeed::CsvFile {
                    cache_status: CacheStatus::from_cache_key(
                        hash_chain.cache_key(),
                        backend,
                        instance_name,
                    )
                    .await?,
                    name,
                    path: path.clone(),
                    table: table.clone(),
                    delimiter: *delimiter,
                    content,
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
    #[error("Failed to load seed {name}: cache key command {command} failed")]
    KeyCommand {
        name: SeedName,
        command: String,
        #[source]
        source: cmd_proc::CommandError,
    },
    #[error("Failed to load seed {name}: cache key script failed")]
    KeyScript {
        name: SeedName,
        #[source]
        source: cmd_proc::CommandError,
    },
    #[error("Failed to probe cache image {reference} presence")]
    CacheImagePresent {
        reference: ociman::Reference,
        #[source]
        source: ociman::backend::ImagePresentError,
    },
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
    SqlStatement {
        cache_status: CacheStatus,
        name: SeedName,
        statement: String,
    },
    Command {
        cache_status: CacheStatus,
        name: SeedName,
        command: Command,
    },
    Script {
        cache_status: CacheStatus,
        name: SeedName,
        script: String,
    },
    ContainerScript {
        cache_status: CacheStatus,
        name: SeedName,
        script: String,
    },
    CsvFile {
        cache_status: CacheStatus,
        name: SeedName,
        path: std::path::PathBuf,
        table: pg_client::QualifiedTable,
        delimiter: char,
        content: String,
    },
}

impl LoadedSeed {
    #[must_use]
    pub fn cache_status(&self) -> &CacheStatus {
        match self {
            Self::SqlFile { cache_status, .. }
            | Self::SqlFileGitRevision { cache_status, .. }
            | Self::SqlStatement { cache_status, .. }
            | Self::Command { cache_status, .. }
            | Self::Script { cache_status, .. }
            | Self::ContainerScript { cache_status, .. }
            | Self::CsvFile { cache_status, .. } => cache_status,
        }
    }

    #[must_use]
    pub fn name(&self) -> &SeedName {
        match self {
            Self::SqlFile { name, .. }
            | Self::SqlFileGitRevision { name, .. }
            | Self::SqlStatement { name, .. }
            | Self::Command { name, .. }
            | Self::Script { name, .. }
            | Self::ContainerScript { name, .. }
            | Self::CsvFile { name, .. } => name,
        }
    }

    fn variant_name(&self) -> &'static str {
        match self {
            Self::SqlFile { .. } => "sql-file",
            Self::SqlFileGitRevision { .. } => "sql-file-git-revision",
            Self::SqlStatement { .. } => "sql-statement",
            Self::Command { .. } => "command",
            Self::Script { .. } => "script",
            Self::ContainerScript { .. } => "container-script",
            Self::CsvFile { .. } => "csv-file",
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

    fn cache_key(&self) -> Option<SeedHash> {
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
    pub async fn load(
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
            let loaded_seed = seed
                .load(name.clone(), &mut hash_chain, backend, instance_name)
                .await?;
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

    pub fn print(&self, instance_name: &crate::InstanceName) {
        println!("Instance: {instance_name}");
        println!("Image:    {}", self.image);
        println!("Version:  {}", crate::VERSION_STR);
        println!();

        let mut table = comfy_table::Table::new();

        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_header(["Seed", "Type", "Status"]);

        for seed in &self.seeds {
            table.add_row([
                seed.name().as_str(),
                seed.variant_name(),
                seed.cache_status().status_str(),
            ]);
        }

        println!("{table}");
    }

    pub fn print_json(&self, instance_name: &crate::InstanceName) {
        #[derive(serde::Serialize)]
        struct Output<'a> {
            instance: &'a str,
            image: String,
            version: &'a str,
            seeds: Vec<SeedOutput<'a>>,
        }

        #[derive(serde::Serialize)]
        struct SeedOutput<'a> {
            name: &'a str,
            r#type: &'a str,
            status: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            reference: Option<String>,
        }

        let output = Output {
            instance: instance_name.as_ref(),
            image: self.image.to_string(),
            version: crate::VERSION_STR,
            seeds: self
                .seeds
                .iter()
                .map(|seed| SeedOutput {
                    name: seed.name().as_str(),
                    r#type: seed.variant_name(),
                    status: seed.cache_status().status_str(),
                    reference: seed.cache_status().reference().map(|r| r.to_string()),
                })
                .collect(),
        };

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_simple() {
        let name: SeedName = "schema".parse().unwrap();
        assert_eq!(name.to_string(), "schema");
        assert_eq!(name.as_str(), "schema");
    }

    #[test]
    fn parse_valid_with_dash() {
        let name: SeedName = "create-users-table".parse().unwrap();
        assert_eq!(name.to_string(), "create-users-table");
    }

    #[test]
    fn parse_valid_single_char() {
        let name: SeedName = "a".parse().unwrap();
        assert_eq!(name.to_string(), "a");
    }

    #[test]
    fn parse_valid_numeric() {
        let name: SeedName = "123".parse().unwrap();
        assert_eq!(name.to_string(), "123");
    }

    #[test]
    fn parse_valid_max_length() {
        let input = "a".repeat(SEED_NAME_MAX_LENGTH);
        let name: SeedName = input.parse().unwrap();
        assert_eq!(name.to_string(), input);
    }

    #[test]
    fn parse_empty_fails() {
        assert_eq!("".parse::<SeedName>(), Err(SeedNameError::Empty));
        assert_eq!(SeedName::try_from(String::new()), Err(SeedNameError::Empty));
    }

    #[test]
    fn parse_too_long_fails() {
        let input = "a".repeat(SEED_NAME_MAX_LENGTH + 1);
        assert_eq!(input.parse::<SeedName>(), Err(SeedNameError::TooLong));
    }

    #[test]
    fn parse_starts_with_dash_fails() {
        assert_eq!(
            "-schema".parse::<SeedName>(),
            Err(SeedNameError::StartsWithDash)
        );
    }

    #[test]
    fn parse_ends_with_dash_fails() {
        assert_eq!(
            "schema-".parse::<SeedName>(),
            Err(SeedNameError::EndsWithDash)
        );
    }

    #[test]
    fn parse_uppercase_fails() {
        assert_eq!(
            "Schema".parse::<SeedName>(),
            Err(SeedNameError::InvalidCharacter)
        );
    }

    #[test]
    fn parse_underscore_fails() {
        assert_eq!(
            "create_table".parse::<SeedName>(),
            Err(SeedNameError::InvalidCharacter)
        );
    }

    #[test]
    fn parse_space_fails() {
        assert_eq!(
            "my seed".parse::<SeedName>(),
            Err(SeedNameError::InvalidCharacter)
        );
    }

    #[test]
    fn try_from_string_valid() {
        assert_eq!(
            SeedName::try_from("valid-name".to_string()),
            Ok(SeedName::from_static_or_panic("valid-name"))
        );
    }

    #[test]
    fn from_static_or_panic_works() {
        const NAME: SeedName = SeedName::from_static_or_panic("my-seed");
        assert_eq!(NAME.as_str(), "my-seed");
    }

    #[test]
    fn test_cache_status_uncacheable() {
        let loaded_seed = LoadedSeed::Command {
            cache_status: CacheStatus::Uncacheable,
            name: "run-migrations".parse().unwrap(),
            command: Command::new("migrate", ["up"]),
        };

        assert!(loaded_seed.cache_status().reference().is_none());
        assert!(!loaded_seed.cache_status().is_hit());
    }

    #[test]
    fn test_cache_status_miss() {
        let hash: SeedHash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .parse()
            .unwrap();
        let reference: ociman::Reference =
            "pg-ephemeral/main:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        let loaded_seed = LoadedSeed::SqlFile {
            cache_status: CacheStatus::Miss {
                hash: hash.clone(),
                reference: reference.clone(),
            },
            name: "schema".parse().unwrap(),
            path: "schema.sql".into(),
            content: "CREATE TABLE test();".to_string(),
        };

        assert_eq!(loaded_seed.cache_status().reference(), Some(&reference));
        assert_eq!(loaded_seed.cache_status().hash(), Some(&hash));
        assert!(!loaded_seed.cache_status().is_hit());
    }

    #[test]
    fn test_cache_status_hit() {
        let hash: SeedHash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .parse()
            .unwrap();
        let reference: ociman::Reference =
            "pg-ephemeral/main:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        let loaded_seed = LoadedSeed::SqlFile {
            cache_status: CacheStatus::Hit {
                hash: hash.clone(),
                reference: reference.clone(),
            },
            name: "schema".parse().unwrap(),
            path: "schema.sql".into(),
            content: "CREATE TABLE test();".to_string(),
        };

        assert_eq!(loaded_seed.cache_status().reference(), Some(&reference));
        assert_eq!(loaded_seed.cache_status().hash(), Some(&hash));
        assert!(loaded_seed.cache_status().is_hit());
    }
}
