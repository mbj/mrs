#![doc = include_str!("../README.md")]

pub mod identifier;
pub mod pg_dump;

pub use identifier::{Database, QualifiedTable, Role, User};
pub use pg_dump::{PgSchemaDump, RestrictKey};

#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod url;

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
    /// use pg_client::HostAddr;
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
    /// use pg_client::HostAddr;
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
    /// use pg_client::HostAddr;
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

    fn pg_env_value(self) -> String {
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
    fn pg_env_value(&self) -> String {
        self.0.clone()
    }
}

impl Database {
    fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

impl Role {
    fn pg_env_value(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Password(String);

from_str_impl!(Password, 0, 4096);

impl Password {
    fn pg_env_value(&self) -> String {
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

    fn pg_env_value(&self) -> String {
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

    fn pg_env_value(&self) -> String {
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

#[derive(Clone, Debug, PartialEq, Eq)]
/// PG connection config with various presentation modes.
///
/// Supported:
///
/// 1. Env variables via `to_pg_env()`
/// 2. JSON document via `serde`
/// 3. sqlx connect options via `to_sqlx_connect_options()`
/// 4. Individual field access
pub struct Config {
    pub application_name: Option<ApplicationName>,
    pub database: Database,
    pub endpoint: Endpoint,
    pub password: Option<Password>,
    pub ssl_mode: SslMode,
    pub ssl_root_cert: Option<SslRootCert>,
    pub user: User,
}

pub const PGAPPNAME: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGAPPNAME");
pub const PGCHANNELBINDING: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGCHANNELBINDING");
pub const PGDATABASE: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGDATABASE");
pub const PGHOST: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGHOST");
pub const PGHOSTADDR: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGHOSTADDR");
pub const PGPASSWORD: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGPASSWORD");
pub const PGPORT: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGPORT");
pub const PGSSLMODE: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGSSLMODE");
pub const PGSSLROOTCERT: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGSSLROOTCERT");
pub const PGUSER: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGUSER");

impl serde::Serialize for Config {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Config", 8)?;

        if let Some(application_name) = &self.application_name {
            state.serialize_field("application_name", application_name)?;
        }

        state.serialize_field("database", &self.database)?;
        state.serialize_field("endpoint", &self.endpoint)?;

        if let Some(password) = &self.password {
            state.serialize_field("password", password)?;
        }

        state.serialize_field("ssl_mode", &self.ssl_mode)?;

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            state.serialize_field("ssl_root_cert", ssl_root_cert)?;
        }

        state.serialize_field("user", &self.user)?;
        state.serialize_field("url", &self.to_url_string())?;

        state.end()
    }
}

impl Config {
    /// Convert to PG connection URL
    ///
    /// ```
    /// # use pg_client::*;
    /// # use std::str::FromStr;
    ///
    /// let config = Config {
    ///     application_name: None,
    ///     database: Database::from_static_or_panic("some-database"),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::from_str("some-host").unwrap(),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     user: User::from_static_or_panic("some-user"),
    /// };
    ///
    /// assert_eq!(
    ///     config.to_url_string(),
    ///     "postgres://some-user@some-host:5432/some-database?sslmode=verify-full"
    /// );
    ///
    /// assert_eq!(
    ///     Config {
    ///         application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///         password: Some(Password::from_str("some-password").unwrap()),
    ///         ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///         ..config.clone()
    ///     }.to_url_string(),
    ///     "postgres://some-user:some-password@some-host:5432/some-database?application_name=some-app&sslmode=verify-full&sslrootcert=%2Fsome.pem"
    /// );
    ///
    /// assert_eq!(
    ///     Config {
    ///         endpoint: Endpoint::Network {
    ///             host: Host::from_str("some-host").unwrap(),
    ///             channel_binding: None,
    ///             host_addr: Some("127.0.0.1".parse().unwrap()),
    ///             port: Some(Port::new(5432)),
    ///         },
    ///         ..config.clone()
    ///     }.to_url_string(),
    ///     "postgres://some-user@some-host:5432/some-database?hostaddr=127.0.0.1&sslmode=verify-full"
    /// );
    ///
    /// // IPv4 example
    /// let ipv4_config = Config {
    ///     application_name: None,
    ///     database: Database::from_static_or_panic("mydb"),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     user: User::from_static_or_panic("user"),
    /// };
    /// assert_eq!(
    ///     ipv4_config.to_url_string(),
    ///     "postgres://user@127.0.0.1:5432/mydb?sslmode=disable"
    /// );
    ///
    /// // IPv6 example (automatically bracketed)
    /// let ipv6_config = Config {
    ///     application_name: None,
    ///     database: Database::from_static_or_panic("mydb"),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     user: User::from_static_or_panic("user"),
    /// };
    /// assert_eq!(
    ///     ipv6_config.to_url_string(),
    ///     "postgres://user@[::1]:5432/mydb?sslmode=disable"
    /// );
    /// ```
    #[must_use]
    pub fn to_url(&self) -> ::fluent_uri::Uri<String> {
        use ::fluent_uri::{
            Uri,
            build::Builder,
            component::{Authority, Scheme},
            pct_enc::{EStr, EString, encoder},
        };

        const POSTGRES: &Scheme = Scheme::new_or_panic("postgres");

        fn append_query_pair(query: &mut EString<encoder::Query>, key: &str, value: &str) {
            if !query.is_empty() {
                query.push('&');
            }
            query.encode_str::<encoder::Data>(key);
            query.push('=');
            query.encode_str::<encoder::Data>(value);
        }

        let mut query = EString::<encoder::Query>::new();

        match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                let mut userinfo = EString::<encoder::Userinfo>::new();
                userinfo.encode_str::<encoder::Data>(self.user.pg_env_value().as_str());
                if let Some(password) = &self.password {
                    userinfo.push(':');
                    userinfo.encode_str::<encoder::Data>(password.as_str());
                }

                let mut path = EString::<encoder::Path>::new();
                path.push('/');
                path.encode_str::<encoder::Data>(self.database.as_str());

                if let Some(addr) = host_addr {
                    append_query_pair(&mut query, "hostaddr", &addr.to_string());
                }
                if let Some(channel_binding) = channel_binding {
                    append_query_pair(&mut query, "channel_binding", channel_binding.as_str());
                }
                self.append_common_query_params(&mut query, append_query_pair);

                let non_empty_query = if query.is_empty() {
                    None
                } else {
                    Some(query.as_estr())
                };

                // build() only fails on RFC 3986 structural violations:
                // scheme and authority are always present, path starts with '/'.
                Uri::builder()
                    .scheme(POSTGRES)
                    .authority_with(|builder| {
                        let builder = builder.userinfo(&userinfo);
                        let builder = match host {
                            Host::IpAddr(addr) => builder.host(*addr),
                            Host::HostName(name) => {
                                let mut encoded = EString::<encoder::RegName>::new();
                                encoded.encode_str::<encoder::Data>(name.as_str());
                                builder.host(encoded.as_estr())
                            }
                        };
                        match port {
                            Some(port) => builder.port(port.0),
                            None => builder.advance(),
                        }
                    })
                    .path(&path)
                    .optional(Builder::query, non_empty_query)
                    .build()
                    .unwrap()
            }
            Endpoint::SocketPath(path) => {
                append_query_pair(
                    &mut query,
                    "host",
                    path.to_str().expect("socket path contains invalid utf8"),
                );
                append_query_pair(&mut query, "dbname", self.database.as_str());
                append_query_pair(&mut query, "user", self.user.pg_env_value().as_str());
                if let Some(password) = &self.password {
                    append_query_pair(&mut query, "password", password.as_str());
                }
                self.append_common_query_params(&mut query, append_query_pair);

                // build() only fails on RFC 3986 structural violations:
                // scheme and authority are always present, path is empty.
                Uri::builder()
                    .scheme(POSTGRES)
                    .authority(Authority::EMPTY)
                    .path(EStr::EMPTY)
                    .query(&query)
                    .build()
                    .unwrap()
            }
        }
    }

    /// Convert to PG connection URL string
    #[must_use]
    pub fn to_url_string(&self) -> String {
        self.to_url().into_string()
    }

    fn append_common_query_params(
        &self,
        query: &mut ::fluent_uri::pct_enc::EString<::fluent_uri::pct_enc::encoder::Query>,
        append_query_pair: fn(
            &mut ::fluent_uri::pct_enc::EString<::fluent_uri::pct_enc::encoder::Query>,
            &str,
            &str,
        ),
    ) {
        if let Some(application_name) = &self.application_name {
            append_query_pair(query, "application_name", application_name.as_str());
        }
        append_query_pair(query, "sslmode", &self.ssl_mode.pg_env_value());
        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            append_query_pair(query, "sslrootcert", &ssl_root_cert.pg_env_value());
        }
    }

    /// Convert to PG environment variable names
    ///
    /// ```
    /// # use pg_client::*;
    /// # use std::collections::BTreeMap;
    ///
    /// let config = Config {
    ///     application_name: None,
    ///     database: "some-database".parse().unwrap(),
    ///     endpoint: Endpoint::Network {
    ///         host: "some-host".parse().unwrap(),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     user: "some-user".parse().unwrap(),
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     (PGDATABASE, "some-database".to_string()),
    ///     (PGHOST, "some-host".to_string()),
    ///     (PGPORT, "5432".to_string()),
    ///     (PGSSLMODE, "verify-full".to_string()),
    ///     (PGUSER, "some-user".to_string()),
    /// ]);
    ///
    /// assert_eq!(expected, config.to_pg_env());
    ///
    /// let config_with_optionals = Config {
    ///     application_name: Some("some-app".parse().unwrap()),
    ///     endpoint: Endpoint::Network {
    ///         host: "some-host".parse().unwrap(),
    ///         channel_binding: None,
    ///         host_addr: Some("127.0.0.1".parse().unwrap()),
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: Some("some-password".parse().unwrap()),
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     ..config
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     (PGAPPNAME, "some-app".to_string()),
    ///     (PGDATABASE, "some-database".to_string()),
    ///     (PGHOST, "some-host".to_string()),
    ///     (PGHOSTADDR, "127.0.0.1".to_string()),
    ///     (PGPASSWORD, "some-password".to_string()),
    ///     (PGPORT, "5432".to_string()),
    ///     (PGSSLMODE, "verify-full".to_string()),
    ///     (PGSSLROOTCERT, "/some.pem".to_string()),
    ///     (PGUSER, "some-user".to_string()),
    /// ]);
    ///
    /// assert_eq!(expected, config_with_optionals.to_pg_env());
    /// ```
    #[must_use]
    pub fn to_pg_env(
        &self,
    ) -> std::collections::BTreeMap<cmd_proc::EnvVariableName<'static>, String> {
        let mut map = std::collections::BTreeMap::new();

        match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                map.insert(PGHOST.clone(), host.pg_env_value());
                if let Some(port) = port {
                    map.insert(PGPORT.clone(), port.pg_env_value());
                }
                if let Some(channel_binding) = channel_binding {
                    map.insert(PGCHANNELBINDING.clone(), channel_binding.pg_env_value());
                }
                if let Some(addr) = host_addr {
                    map.insert(PGHOSTADDR.clone(), addr.to_string());
                }
            }
            Endpoint::SocketPath(path) => {
                map.insert(
                    PGHOST.clone(),
                    path.to_str()
                        .expect("socket path contains invalid utf8")
                        .to_string(),
                );
            }
        }

        map.insert(PGSSLMODE.clone(), self.ssl_mode.pg_env_value());
        map.insert(PGUSER.clone(), self.user.pg_env_value());
        map.insert(PGDATABASE.clone(), self.database.pg_env_value());

        if let Some(application_name) = &self.application_name {
            map.insert(PGAPPNAME.clone(), application_name.pg_env_value());
        }

        if let Some(password) = &self.password {
            map.insert(PGPASSWORD.clone(), password.pg_env_value());
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            map.insert(PGSSLROOTCERT.clone(), ssl_root_cert.pg_env_value());
        }

        map
    }

    #[must_use]
    pub fn endpoint(self, endpoint: Endpoint) -> Self {
        Self { endpoint, ..self }
    }

    /// Parse a PostgreSQL connection URL string into a Config.
    ///
    /// When the URL does not specify `sslmode`, it defaults to `verify-full`
    /// to ensure secure connections by default.
    ///
    /// See [`url::parse`] for full documentation.
    pub fn from_str_url(url: &str) -> Result<Self, crate::url::ParseError> {
        crate::url::parse(url)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    const TEST_DATABASE: Database = Database::from_static_or_panic("some-database");
    const TEST_USER: User = User::from_static_or_panic("some-user");

    fn assert_config(expected: serde_json::Value, config: &Config) {
        assert_eq!(expected, serde_json::to_value(config).unwrap());
    }

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

    #[test]
    fn test_json() {
        let config = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::from_str("some-host").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: None,
            user: TEST_USER,
        };

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@some-host:5432/some-database?sslmode=verify-full",
                "user": "some-user",
            }),
            &config,
        );

        assert_config(
            serde_json::json!({
                "application_name": "some-app",
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "port": 5432,
                },
                "password": "some-password",
                "ssl_mode": "verify-full",
                "ssl_root_cert": {
                    "file": "/some.pem"
                },
                "url": "postgres://some-user:some-password@some-host:5432/some-database?application_name=some-app&sslmode=verify-full&sslrootcert=%2Fsome.pem",
                "user": "some-user"
            }),
            &Config {
                application_name: Some(ApplicationName::from_str("some-app").unwrap()),
                password: Some(Password::from_str("some-password").unwrap()),
                ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
                ..config.clone()
            },
        );

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "127.0.0.1",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@127.0.0.1:5432/some-database?sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("127.0.0.1").unwrap(),
                    channel_binding: None,
                    host_addr: None,
                    port: Some(Port::new(5432)),
                },
                ..config.clone()
            },
        );

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "socket_path": "/some/socket",
                },
                "ssl_mode": "verify-full",
                "url": "postgres://?host=%2Fsome%2Fsocket&dbname=some-database&user=some-user&sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::SocketPath("/some/socket".into()),
                ..config.clone()
            },
        );

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "ssl_root_cert": "system",
                "url": "postgres://some-user@some-host:5432/some-database?sslmode=verify-full&sslrootcert=system",
                "user": "some-user"
            }),
            &Config {
                ssl_root_cert: Some(SslRootCert::System),
                ..config.clone()
            },
        );

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "host_addr": "192.168.1.100",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@some-host:5432/some-database?hostaddr=192.168.1.100&sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
                    channel_binding: None,
                    host_addr: Some("192.168.1.100".parse().unwrap()),
                    port: Some(Port::new(5432)),
                },
                ..config.clone()
            },
        );

        // Test Network endpoint without port (should use default)
        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@some-host/some-database?sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
                    channel_binding: None,
                    host_addr: None,
                    port: None,
                },
                ..config.clone()
            },
        );

        // Test Network endpoint with host_addr but without port
        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "host_addr": "10.0.0.1",
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@some-host/some-database?hostaddr=10.0.0.1&sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
                    channel_binding: None,
                    host_addr: Some("10.0.0.1".parse().unwrap()),
                    port: None,
                },
                ..config.clone()
            },
        );
    }

    #[test]
    fn test_ipv6_url_formation() {
        // Test IPv6 loopback address
        let config_ipv6_loopback = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            user: User::POSTGRES,
        };

        assert_eq!(
            config_ipv6_loopback.to_url_string(),
            "postgres://postgres@[::1]:5432/some-database?sslmode=disable",
            "IPv6 loopback address should be bracketed in URL"
        );

        // Test fe80 link-local IPv6 address
        let config_ipv6_fe80 = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0xfe80, 0, 0, 0, 0, 0, 0, 1,
                ))),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            user: User::POSTGRES,
        };

        assert_eq!(
            config_ipv6_fe80.to_url_string(),
            "postgres://postgres@[fe80::1]:5432/some-database?sslmode=disable",
            "IPv6 link-local address should be bracketed in URL"
        );

        // Test full IPv6 address
        let config_ipv6_full = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x2001, 0x0db8, 0, 0, 0, 0, 0, 1,
                ))),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            user: User::POSTGRES,
        };

        assert_eq!(
            config_ipv6_full.to_url_string(),
            "postgres://postgres@[2001:db8::1]:5432/some-database?sslmode=disable",
            "Full IPv6 address should be bracketed in URL"
        );

        // Test IPv4 address (should NOT be bracketed)
        let config_ipv4 = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            user: User::POSTGRES,
        };

        assert_eq!(
            config_ipv4.to_url_string(),
            "postgres://postgres@127.0.0.1:5432/some-database?sslmode=disable",
            "IPv4 address should NOT be bracketed in URL"
        );

        // Test hostname (should NOT be bracketed)
        let config_hostname = Config {
            application_name: None,
            database: TEST_DATABASE,
            endpoint: Endpoint::Network {
                host: Host::from_str("localhost").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            user: User::POSTGRES,
        };

        assert_eq!(
            config_hostname.to_url_string(),
            "postgres://postgres@localhost:5432/some-database?sslmode=disable",
            "Hostname should NOT be bracketed in URL"
        );
    }
}
