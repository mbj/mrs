use crate::identifier::{Database, Role, User};

/// Macro to generate `std::str::FromStr` plus helpers for string wrapped newtypes
macro_rules! from_str_impl {
    ($struct: ident, $min: expr, $max: expr) => {
        impl std::str::FromStr for $struct {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let min_length = Self::MIN_LENGTH;
                let max_length = Self::MAX_LENGTH;
                let actual = value.len();

                if actual < min_length {
                    Err(format!(
                        "{} byte min length: {min_length} violated, got: {actual}",
                        stringify!($struct)
                    ))
                } else if actual > max_length {
                    Err(format!(
                        "{} byte max length: {max_length} violated, got: {actual}",
                        stringify!($struct)
                    ))
                } else if value.as_bytes().contains(&0) {
                    Err(format!("{} contains NUL byte", stringify!($struct)))
                } else {
                    Ok(Self(value.to_string()))
                }
            }
        }

        impl AsRef<str> for $struct {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl $struct {
            pub const MIN_LENGTH: usize = $min;
            pub const MAX_LENGTH: usize = $max;

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct HostName(String);

impl HostName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for HostName {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if hostname_validator::is_valid(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("invalid host name")
        }
    }
}

impl<'de> serde::Deserialize<'de> for HostName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Host {
    HostName(HostName),
    IpAddr(std::net::IpAddr),
}

impl serde::Serialize for Host {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.pg_env_value())
    }
}

impl Host {
    pub(crate) fn pg_env_value(&self) -> String {
        match self {
            Self::HostName(value) => value.0.clone(),
            Self::IpAddr(value) => value.to_string(),
        }
    }
}

impl std::str::FromStr for Host {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match std::net::IpAddr::from_str(value) {
            Ok(addr) => Ok(Self::IpAddr(addr)),
            Err(_) => match HostName::from_str(value) {
                Ok(host_name) => Ok(Self::HostName(host_name)),
                Err(_) => Err("Not a socket address or FQDN"),
            },
        }
    }
}

impl From<HostName> for Host {
    fn from(value: HostName) -> Self {
        Self::HostName(value)
    }
}

impl From<std::net::IpAddr> for Host {
    fn from(value: std::net::IpAddr) -> Self {
        Self::IpAddr(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostAddr(std::net::IpAddr);

impl HostAddr {
    #[must_use]
    pub const fn new(ip: std::net::IpAddr) -> Self {
        Self(ip)
    }
}

impl From<std::net::IpAddr> for HostAddr {
    /// # Example
    /// ```
    /// use pg_client::config::HostAddr;
    /// use std::net::IpAddr;
    ///
    /// let ip: IpAddr = "192.168.1.1".parse().unwrap();
    /// let host_addr = HostAddr::from(ip);
    /// assert_eq!(IpAddr::from(host_addr).to_string(), "192.168.1.1");
    /// ```
    fn from(value: std::net::IpAddr) -> Self {
        Self(value)
    }
}

impl From<HostAddr> for std::net::IpAddr {
    fn from(value: HostAddr) -> Self {
        value.0
    }
}

impl From<&HostAddr> for std::net::IpAddr {
    fn from(value: &HostAddr) -> Self {
        value.0
    }
}

impl std::fmt::Display for HostAddr {
    /// # Example
    /// ```
    /// use pg_client::config::HostAddr;
    ///
    /// let host_addr: HostAddr = "10.0.0.1".parse().unwrap();
    /// assert_eq!(host_addr.to_string(), "10.0.0.1");
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for HostAddr {
    type Err = &'static str;

    /// # Example
    /// ```
    /// use pg_client::config::HostAddr;
    /// use std::str::FromStr;
    ///
    /// let host_addr = HostAddr::from_str("127.0.0.1").unwrap();
    /// assert_eq!(host_addr.to_string(), "127.0.0.1");
    ///
    /// // Also works with the parse method
    /// let host_addr: HostAddr = "::1".parse().unwrap();
    /// assert_eq!(host_addr.to_string(), "::1");
    ///
    /// // Invalid IP addresses return an error
    /// assert!(HostAddr::from_str("not-an-ip").is_err());
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match std::net::IpAddr::from_str(value) {
            Ok(addr) => Ok(Self(addr)),
            Err(_) => Err("invalid IP address"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Endpoint {
    Network {
        host: Host,
        channel_binding: Option<ChannelBinding>,
        host_addr: Option<HostAddr>,
        port: Option<Port>,
    },
    SocketPath(std::path::PathBuf),
}

impl serde::Serialize for Endpoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        match self {
            Self::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                let mut state = serializer.serialize_struct("Endpoint", 4)?;
                state.serialize_field("host", host)?;
                if let Some(channel_binding) = channel_binding {
                    state.serialize_field("channel_binding", channel_binding)?;
                }
                if let Some(addr) = host_addr {
                    state.serialize_field("host_addr", &addr.to_string())?;
                }
                if let Some(port) = port {
                    state.serialize_field("port", port)?;
                }
                state.end()
            }
            Self::SocketPath(path) => {
                let mut state = serializer.serialize_struct("Endpoint", 1)?;
                state.serialize_field(
                    "socket_path",
                    &path.to_str().expect("socket path contains invalid utf8"),
                )?;
                state.end()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Port(u16);

impl Port {
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    pub(crate) fn pg_env_value(self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for Port {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match <u16 as std::str::FromStr>::from_str(value) {
            Ok(port) => Ok(Port(port)),
            Err(_) => Err("invalid postgresql port string"),
        }
    }
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Self(port)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}

impl From<&Port> for u16 {
    fn from(port: &Port) -> Self {
        port.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ApplicationName(String);

from_str_impl!(ApplicationName, 1, 63);

impl ApplicationName {
    pub(crate) fn pg_env_value(&self) -> String {
        self.0.clone()
    }
}

impl Database {
    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

impl Role {
    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Password(String);

from_str_impl!(Password, 0, 4096);

impl Password {
    pub(crate) fn pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, strum::IntoStaticStr, strum::EnumString,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum SslMode {
    Allow,
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, strum::IntoStaticStr, strum::EnumString,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ChannelBinding {
    Disable,
    Prefer,
    Require,
}

impl ChannelBinding {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SslRootCert {
    File(std::path::PathBuf),
    System,
}

impl SslRootCert {
    pub(crate) fn pg_env_value(&self) -> String {
        match self {
            Self::File(path) => path.to_str().unwrap().to_string(),
            Self::System => "system".to_string(),
        }
    }
}

impl From<std::path::PathBuf> for SslRootCert {
    fn from(value: std::path::PathBuf) -> Self {
        Self::File(value)
    }
}

/// Session parameters sent during PostgreSQL connection setup.
///
/// These are independent of how the connection is established (TCP, Unix socket, etc.)
/// and represent what the client identifies as during the startup message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub application_name: Option<ApplicationName>,
    pub database: Database,
    pub password: Option<Password>,
    pub user: User,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    fn repeat(char: char, len: usize) -> String {
        std::iter::repeat_n(char, len).collect()
    }

    #[test]
    fn application_name_lt_min_length() {
        let value = String::new();

        let err = ApplicationName::from_str(&value).expect_err("expected min length failure");

        assert_eq!(err, "ApplicationName byte min length: 1 violated, got: 0");
    }

    #[test]
    fn application_name_eq_min_length() {
        let value = repeat('a', 1);

        let application_name =
            ApplicationName::from_str(&value).expect("expected valid min length value");

        assert_eq!(application_name, ApplicationName(value));
    }

    #[test]
    fn application_name_gt_min_length() {
        let value = repeat('a', 2);

        let application_name =
            ApplicationName::from_str(&value).expect("expected valid value greater than min");

        assert_eq!(application_name, ApplicationName(value));
    }

    #[test]
    fn application_name_lt_max_length() {
        let value = repeat('a', 62);

        let application_name =
            ApplicationName::from_str(&value).expect("expected valid value less than max");

        assert_eq!(application_name, ApplicationName(value));
    }

    #[test]
    fn application_name_eq_max_length() {
        let value = repeat('a', 63);

        let application_name =
            ApplicationName::from_str(&value).expect("expected valid value equal to max");

        assert_eq!(application_name, ApplicationName(value));
    }

    #[test]
    fn application_name_gt_max_length() {
        let value = repeat('a', 64);

        let err = ApplicationName::from_str(&value).expect_err("expected max length failure");

        assert_eq!(err, "ApplicationName byte max length: 63 violated, got: 64");
    }

    #[test]
    fn application_name_contains_nul() {
        let value = String::from('\0');

        let err = ApplicationName::from_str(&value).expect_err("expected NUL failure");

        assert_eq!(err, "ApplicationName contains NUL byte");
    }

    #[test]
    fn password_eq_min_length() {
        let value = String::new();

        let password = Password::from_str(&value).expect("expected valid min length value");

        assert_eq!(password, Password(value));
    }

    #[test]
    fn password_gt_min_length() {
        let value = repeat('p', 1);

        let password = Password::from_str(&value).expect("expected valid value greater than min");

        assert_eq!(password, Password(value));
    }

    #[test]
    fn password_lt_max_length() {
        let value = repeat('p', 4095);

        let password = Password::from_str(&value).expect("expected valid value less than max");

        assert_eq!(password, Password(value));
    }

    #[test]
    fn password_eq_max_length() {
        let value = repeat('p', 4096);

        let password = Password::from_str(&value).expect("expected valid value equal to max");

        assert_eq!(password, Password(value));
    }

    #[test]
    fn password_gt_max_length() {
        let value = repeat('p', 4097);

        let err = Password::from_str(&value).expect_err("expected max length failure");

        assert_eq!(err, "Password byte max length: 4096 violated, got: 4097");
    }

    #[test]
    fn password_contains_nul() {
        let value = String::from('\0');

        let err = Password::from_str(&value).expect_err("expected NUL failure");

        assert_eq!(err, "Password contains NUL byte");
    }
}
