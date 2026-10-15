use crate::name::{Database, Role};

mod host_name;

/// Macro to generate `std::str::FromStr` plus helpers for string wrapped newtypes,
/// along with a typed parse-error enum.
macro_rules! from_str_impl {
    ($struct: ident, $err: ident, $min: expr, $max: expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
        pub enum $err {
            #[error("{} byte min length: {min} violated, got: {actual}", stringify!($struct))]
            TooShort { min: usize, actual: usize },
            #[error("{} byte max length: {max} violated, got: {actual}", stringify!($struct))]
            TooLong { max: usize, actual: usize },
            #[error("{} contains NUL byte", stringify!($struct))]
            ContainsNul,
        }

        impl std::str::FromStr for $struct {
            type Err = $err;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let actual = value.len();

                if actual < Self::MIN_LENGTH {
                    Err($err::TooShort {
                        min: Self::MIN_LENGTH,
                        actual,
                    })
                } else if actual > Self::MAX_LENGTH {
                    Err($err::TooLong {
                        max: Self::MAX_LENGTH,
                        actual,
                    })
                } else if value.as_bytes().contains(&0) {
                    Err($err::ContainsNul)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid host name")]
pub struct HostNameParseError;

impl std::str::FromStr for HostName {
    type Err = HostNameParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if host_name::is_valid(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(HostNameParseError)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Not a socket address or FQDN")]
pub struct HostParseError;

impl std::str::FromStr for Host {
    type Err = HostParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match std::net::IpAddr::from_str(value) {
            Ok(addr) => Ok(Self::IpAddr(addr)),
            Err(_) => match HostName::from_str(value) {
                Ok(host_name) => Ok(Self::HostName(host_name)),
                Err(_) => Err(HostParseError),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid IP address")]
pub struct HostAddrParseError;

impl std::str::FromStr for HostAddr {
    type Err = HostAddrParseError;

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
            Err(_) => Err(HostAddrParseError),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid postgresql port string")]
pub struct PortParseError;

impl std::str::FromStr for Port {
    type Err = PortParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match <u16 as std::str::FromStr>::from_str(value) {
            Ok(port) => Ok(Port(port)),
            Err(_) => Err(PortParseError),
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

impl Database<'_> {
    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

impl Role<'_> {
    pub(crate) fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Password(String);

from_str_impl!(Password, PasswordParseError, 0, 4096);

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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    fn repeat(char: char, len: usize) -> String {
        std::iter::repeat_n(char, len).collect()
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

        assert_eq!(
            err,
            PasswordParseError::TooLong {
                max: 4096,
                actual: 4097,
            },
        );
        assert_eq!(
            err.to_string(),
            "Password byte max length: 4096 violated, got: 4097",
        );
    }

    #[test]
    fn password_contains_nul() {
        let value = String::from('\0');

        let err = Password::from_str(&value).expect_err("expected NUL failure");

        assert_eq!(err, PasswordParseError::ContainsNul);
        assert_eq!(err.to_string(), "Password contains NUL byte");
    }
}
