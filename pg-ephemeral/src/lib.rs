pub mod certificate;
pub mod cli;
pub mod config;
pub mod container;
pub mod definition;
pub mod image;
pub mod seed;

pub use config::{Config, Instance};
pub use container::Container;
pub use definition::Definition;
pub use image::Image;
pub use seed::Command;
pub use seed::CommandCacheConfig;
pub use seed::DuplicateSeedName;
pub use seed::LoadError;
pub use seed::Seed;
pub use seed::SeedName;
pub use seed::SeedNameError;

pub(crate) const VERSION_STR: &str = env!("CARGO_PKG_VERSION");
pub(crate) const LOCALHOST_IP: std::net::IpAddr =
    std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
pub(crate) const UNSPECIFIED_IP: std::net::IpAddr =
    std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);
pub(crate) const LOCALHOST_HOST_ADDR: pg_client::HostAddr = pg_client::HostAddr::new(LOCALHOST_IP);
pub(crate) const ENV_DATABASE_URL: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("DATABASE_URL");

#[must_use]
pub fn version() -> &'static semver::Version {
    static VERSION: std::sync::LazyLock<semver::Version> =
        std::sync::LazyLock::new(|| semver::Version::parse(VERSION_STR).unwrap());
    &VERSION
}

pub(crate) fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}

/// Error parsing an instance name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceNameError {
    /// Instance name cannot be empty.
    Empty,
    /// Instance name contains an invalid character.
    InvalidCharacter,
    /// Instance name starts with a dash.
    StartsWithDash,
    /// Instance name ends with a dash.
    EndsWithDash,
}

impl InstanceNameError {
    #[must_use]
    const fn message(&self) -> &'static str {
        match self {
            Self::Empty => "instance name cannot be empty",
            Self::InvalidCharacter => {
                "instance name must contain only lowercase ASCII alphanumeric characters or dashes"
            }
            Self::StartsWithDash => "instance name cannot start with a dash",
            Self::EndsWithDash => "instance name cannot end with a dash",
        }
    }
}

impl std::fmt::Display for InstanceNameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message())
    }
}

impl std::error::Error for InstanceNameError {}

const fn validate_instance_name(input: &str) -> Option<InstanceNameError> {
    let bytes = input.as_bytes();

    if bytes.is_empty() {
        return Some(InstanceNameError::Empty);
    }

    if bytes[0] == b'-' {
        return Some(InstanceNameError::StartsWithDash);
    }

    if bytes[bytes.len() - 1] == b'-' {
        return Some(InstanceNameError::EndsWithDash);
    }

    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
            return Some(InstanceNameError::InvalidCharacter);
        }
        index += 1;
    }

    None
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct InstanceName(std::borrow::Cow<'static, str>);

impl TryFrom<String> for InstanceName {
    type Error = InstanceNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match validate_instance_name(&value) {
            Some(error) => Err(error),
            None => Ok(Self(std::borrow::Cow::Owned(value))),
        }
    }
}

impl InstanceName {
    pub const MAIN: Self = Self::from_static_or_panic("main");

    /// Creates a new instance name from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty, contains non-alphanumeric/dash characters,
    /// or starts/ends with a dash.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_instance_name(input) {
            Some(error) => panic!("{}", error.message()),
            None => Self(std::borrow::Cow::Borrowed(input)),
        }
    }

    /// Returns the instance name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::default::Default for InstanceName {
    fn default() -> Self {
        Self::MAIN
    }
}

impl std::str::FromStr for InstanceName {
    type Err = InstanceNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match validate_instance_name(value) {
            Some(error) => Err(error),
            None => Ok(Self(std::borrow::Cow::Owned(value.to_owned()))),
        }
    }
}

impl std::fmt::Display for InstanceName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<str> for InstanceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub type InstanceMap = std::collections::BTreeMap<InstanceName, config::Instance>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_simple() {
        let name: InstanceName = "main".parse().unwrap();
        assert_eq!(name.to_string(), "main");
    }

    #[test]
    fn parse_valid_with_dash() {
        let name: InstanceName = "my-instance".parse().unwrap();
        assert_eq!(name.to_string(), "my-instance");
    }

    #[test]
    fn parse_valid_single_char() {
        let name: InstanceName = "a".parse().unwrap();
        assert_eq!(name.to_string(), "a");
    }

    #[test]
    fn parse_valid_numeric() {
        let name: InstanceName = "123".parse().unwrap();
        assert_eq!(name.to_string(), "123");
    }

    #[test]
    fn parse_empty_fails() {
        assert_eq!("".parse::<InstanceName>(), Err(InstanceNameError::Empty));
    }

    #[test]
    fn parse_starts_with_dash_fails() {
        assert_eq!(
            "-foo".parse::<InstanceName>(),
            Err(InstanceNameError::StartsWithDash)
        );
    }

    #[test]
    fn parse_ends_with_dash_fails() {
        assert_eq!(
            "foo-".parse::<InstanceName>(),
            Err(InstanceNameError::EndsWithDash)
        );
    }

    #[test]
    fn parse_invalid_character_fails() {
        assert_eq!(
            "foo bar".parse::<InstanceName>(),
            Err(InstanceNameError::InvalidCharacter)
        );
    }

    #[test]
    fn parse_underscore_fails() {
        assert_eq!(
            "foo_bar".parse::<InstanceName>(),
            Err(InstanceNameError::InvalidCharacter)
        );
    }

    #[test]
    fn parse_uppercase_fails() {
        assert_eq!(
            "Main".parse::<InstanceName>(),
            Err(InstanceNameError::InvalidCharacter)
        );
    }

    #[test]
    fn default_is_main() {
        assert_eq!(InstanceName::default(), InstanceName::MAIN);
        assert_eq!(InstanceName::default().to_string(), "main");
    }
}
