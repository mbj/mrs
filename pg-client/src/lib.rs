#![doc = include_str!("../README.md")]

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
        serializer.serialize_str(&self.to_pg_env_value())
    }
}

impl Host {
    fn to_pg_env_value(&self) -> String {
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

#[macro_export]
macro_rules! host {
    ($string: literal) => {
        <pg_client::Host as std::str::FromStr>::from_str($string).unwrap()
    };
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
                host_addr,
                port,
            } => {
                let mut state = serializer.serialize_struct("Endpoint", 3)?;
                state.serialize_field("host", host)?;
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

    fn to_pg_env_value(self) -> String {
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

#[macro_export]
macro_rules! application_name {
    ($string: literal) => {
        <pg_client::ApplicationName as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl ApplicationName {
    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Database(String);

from_str_impl!(Database, 1, 63);

#[macro_export]
macro_rules! database {
    ($string: literal) => {
        <pg_client::Database as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl Database {
    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Username(String);

from_str_impl!(Username, 1, 63);

#[macro_export]
macro_rules! username {
    ($string: literal) => {
        <pg_client::Username as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl Username {
    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Password(String);

from_str_impl!(Password, 0, 4096);

impl Password {
    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, strum::IntoStaticStr, strum::EnumString,
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

    fn to_sqlx_ssl_mode(&self) -> sqlx::postgres::PgSslMode {
        use sqlx::postgres::PgSslMode;

        match self {
            Self::Allow => PgSslMode::Allow,
            Self::Disable => PgSslMode::Disable,
            Self::Prefer => PgSslMode::Prefer,
            Self::Require => PgSslMode::Require,
            Self::VerifyCa => PgSslMode::VerifyCa,
            Self::VerifyFull => PgSslMode::VerifyFull,
        }
    }

    fn to_pg_env_value(&self) -> String {
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
    fn to_pg_env_value(&self) -> String {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlxOptionsError {
    EnvConflict { env_key: String, field_name: String },
    UnsupportedFeature { env_key: String, field_name: String },
}

impl std::fmt::Display for SqlxOptionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvConflict {
                env_key,
                field_name,
            } => write!(
                f,
                "`PgConnectOptions::new` has inferred a `{field_name}` from `{env_key}` environment variable, but `pg_client::Config` does not specify a `{field_name}` value. `PgConnectOptions` does not provide an API to construct an instance without inferring from the environment, does not provide an API to unset the field, we have to bail out at this point. Please remove the environment variable!"
            ),
            Self::UnsupportedFeature {
                env_key,
                field_name,
            } => write!(
                f,
                "`PgConnectOptions::new` has inferred `{field_name}` from the `{env_key}` environment variable, but `pg_client::Config` does not support that feature at this point. As `PgConnectOptions` has no option to unset that field, or a constructor that allows us to bypass the inference: we have to bail out, please remove the environment variable!"
            ),
        }
    }
}

impl std::error::Error for SqlxOptionsError {}

#[derive(Debug, thiserror::Error)]
pub enum SqlxConnectionError {
    #[error("Failed to create SQLx connect options")]
    Options(#[from] SqlxOptionsError),

    #[error("Failed to connect to database")]
    Connect(#[source] sqlx::Error),

    #[error("Failed to close database connection")]
    Close(#[source] sqlx::Error),
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
    pub username: Username,
}

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

        state.serialize_field("username", &self.username)?;
        state.serialize_field("url", &self.to_url())?;

        state.end()
    }
}

impl Config {
    /// Convert to PG connection URL
    ///
    /// ```
    /// # use pg_client::*;
    /// # use std::str::FromStr;
    /// # use url::Url;
    ///
    /// let config = Config {
    ///     application_name: None,
    ///     database: Database::from_str("some-database").unwrap(),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::from_str("some-host").unwrap(),
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     username: Username::from_str("some-username").unwrap(),
    /// };
    ///
    /// let options = config.to_sqlx_connect_options();
    ///
    /// assert_eq!(
    ///     Url::parse(
    ///         "postgres://some-username@some-host:5432/some-database?sslmode=verify-full"
    ///     ).unwrap(),
    ///     config.to_url()
    /// );
    ///
    /// assert_eq!(
    ///     Url::parse(
    ///         "postgres://some-username:some-password@some-host:5432/some-database?application_name=some-app&sslmode=verify-full&sslrootcert=%2Fsome.pem"
    ///     ).unwrap(),
    ///     Config {
    ///         application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///         password: Some(Password::from_str("some-password").unwrap()),
    ///         ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///         ..config.clone()
    ///     }.to_url()
    /// );
    ///
    /// assert_eq!(
    ///     Url::parse(
    ///         "postgres://some-username@some-host:5432/some-database?hostaddr=127.0.0.1&sslmode=verify-full"
    ///     ).unwrap(),
    ///     Config {
    ///         endpoint: Endpoint::Network {
    ///             host: Host::from_str("some-host").unwrap(),
    ///             host_addr: Some("127.0.0.1".parse().unwrap()),
    ///             port: Some(Port::new(5432)),
    ///         },
    ///         ..config.clone()
    ///     }.to_url()
    /// );
    ///
    /// // IPv4 example
    /// let ipv4_config = Config {
    ///     application_name: None,
    ///     database: Database::from_str("mydb").unwrap(),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     username: Username::from_str("user").unwrap(),
    /// };
    /// assert_eq!(
    ///     ipv4_config.to_url().to_string(),
    ///     "postgres://user@127.0.0.1:5432/mydb?sslmode=disable"
    /// );
    ///
    /// // IPv6 example (automatically bracketed)
    /// let ipv6_config = Config {
    ///     application_name: None,
    ///     database: Database::from_str("mydb").unwrap(),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     username: Username::from_str("user").unwrap(),
    /// };
    /// assert_eq!(
    ///     ipv6_config.to_url().to_string(),
    ///     "postgres://user@[::1]:5432/mydb?sslmode=disable"
    /// );
    /// ```
    #[must_use]
    pub fn to_url(&self) -> url::Url {
        let mut url = url::Url::parse("postgres://").unwrap();

        match &self.endpoint {
            Endpoint::Network {
                host,
                host_addr,
                port,
            } => {
                // Use set_ip_host for IP addresses to handle IPv6 bracketing automatically
                match host {
                    Host::IpAddr(ip_addr) => {
                        url.set_ip_host(*ip_addr).unwrap();
                    }
                    Host::HostName(hostname) => {
                        url.set_host(Some(hostname.as_str())).unwrap();
                    }
                }
                url.set_username(self.username.to_pg_env_value().as_str())
                    .unwrap();

                if let Some(password) = &self.password {
                    url.set_password(Some(password.as_str())).unwrap();
                }

                if let Some(port) = port {
                    url.set_port(Some(port.0)).unwrap();
                }

                url.set_path(self.database.as_str());

                // host_addr has no dedicated URL component
                if let Some(addr) = host_addr {
                    url.query_pairs_mut()
                        .append_pair("hostaddr", &addr.to_string());
                }
            }
            Endpoint::SocketPath(path) => {
                // Socket paths require query parameters (no dedicated URL components without a network host)
                url.query_pairs_mut()
                    .append_pair(
                        "host",
                        path.to_str().expect("socket path contains invalid utf8"),
                    )
                    .append_pair("dbname", self.database.as_str())
                    .append_pair("user", self.username.to_pg_env_value().as_str());

                if let Some(password) = &self.password {
                    url.query_pairs_mut()
                        .append_pair("password", password.as_str());
                }
            }
        }

        {
            let mut pairs = url.query_pairs_mut();

            if let Some(application_name) = &self.application_name {
                pairs.append_pair("application_name", application_name.as_str());
            }

            pairs.append_pair("sslmode", &self.ssl_mode.to_pg_env_value());

            if let Some(ssl_root_cert) = &self.ssl_root_cert {
                pairs.append_pair("sslrootcert", &ssl_root_cert.to_pg_env_value());
            }
        }

        url
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
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: None,
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     username: "some-username".parse().unwrap(),
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     ("PGDATABASE", "some-database".to_string()),
    ///     ("PGHOST", "some-host".to_string()),
    ///     ("PGPORT", "5432".to_string()),
    ///     ("PGSSLMODE", "verify-full".to_string()),
    ///     ("PGUSER", "some-username".to_string()),
    /// ]);
    ///
    /// assert_eq!(expected, config.to_pg_env());
    ///
    /// let config_with_optionals = Config {
    ///     application_name: Some("some-app".parse().unwrap()),
    ///     endpoint: Endpoint::Network {
    ///         host: "some-host".parse().unwrap(),
    ///         host_addr: Some("127.0.0.1".parse().unwrap()),
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: Some("some-password".parse().unwrap()),
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     ..config
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     ("PGAPPNAME", "some-app".to_string()),
    ///     ("PGDATABASE", "some-database".to_string()),
    ///     ("PGHOST", "some-host".to_string()),
    ///     ("PGHOSTADDR", "127.0.0.1".to_string()),
    ///     ("PGPASSWORD", "some-password".to_string()),
    ///     ("PGPORT", "5432".to_string()),
    ///     ("PGSSLMODE", "verify-full".to_string()),
    ///     ("PGSSLROOTCERT", "/some.pem".to_string()),
    ///     ("PGUSER", "some-username".to_string()),
    /// ]);
    ///
    /// assert_eq!(expected, config_with_optionals.to_pg_env());
    /// ```
    #[must_use]
    pub fn to_pg_env(&self) -> std::collections::BTreeMap<&'static str, String> {
        let mut map = std::collections::BTreeMap::new();

        match &self.endpoint {
            Endpoint::Network {
                host,
                host_addr,
                port,
            } => {
                map.insert("PGHOST", host.to_pg_env_value());
                if let Some(port) = port {
                    map.insert("PGPORT", port.to_pg_env_value());
                }
                if let Some(addr) = host_addr {
                    map.insert("PGHOSTADDR", addr.to_string());
                }
            }
            Endpoint::SocketPath(path) => {
                map.insert(
                    "PGHOST",
                    path.to_str()
                        .expect("socket path contains invalid utf8")
                        .to_string(),
                );
            }
        }

        map.insert("PGSSLMODE", self.ssl_mode.to_pg_env_value());
        map.insert("PGUSER", self.username.to_pg_env_value());
        map.insert("PGDATABASE", self.database.to_pg_env_value());

        if let Some(application_name) = &self.application_name {
            map.insert("PGAPPNAME", application_name.to_pg_env_value());
        }

        if let Some(password) = &self.password {
            map.insert("PGPASSWORD", password.to_pg_env_value());
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            map.insert("PGSSLROOTCERT", ssl_root_cert.to_pg_env_value());
        }

        map
    }

    /// Convert to an sqlx pg connection config
    ///
    /// ```
    /// # use pg_client::*;
    /// # use std::str::FromStr;
    ///
    /// let config = Config {
    ///     application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///     database: Database::from_str("some-database").unwrap(),
    ///     endpoint: Endpoint::Network {
    ///         host: Host::from_str("some-host").unwrap(),
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     password: Some(Password::from_str("some-password").unwrap()),
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     username: Username::from_str("some-username").unwrap(),
    /// };
    ///
    /// let options = config.to_sqlx_connect_options().unwrap();
    ///
    /// // `PgConnectOptions` does not have `PartialEq` and only partial getters
    /// // so we can only assert a few fields.
    /// assert_eq!(Some("some-app"), options.get_application_name());
    /// assert_eq!("some-host", options.get_host());
    /// assert_eq!(5432, options.get_port());
    /// assert_eq!("some-username", options.get_username());
    /// // No PartialEQ instance
    /// assert_eq!(format!("{:#?}", sqlx::postgres::PgSslMode::VerifyFull), format!("{:#?}", options.get_ssl_mode()));
    /// assert_eq!(Some("some-database"), options.get_database());
    /// // Unsupported.
    /// // assert_eq!("some-password", options.get_password());
    /// // assert_eq!("/some.pem", options.get_ssl_root_cert());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if fields inferred from the process environment variables
    /// by `PgConnectOptions::new` contradict the settings in `Config`, and
    /// there is no public API in `PgConnectOptions` to reset these values.
    pub fn to_sqlx_connect_options(
        &self,
    ) -> Result<sqlx::postgres::PgConnectOptions, SqlxOptionsError> {
        fn reject_env(env_key: &str, field_name: &str) -> Result<(), SqlxOptionsError> {
            if std::env::var(env_key).is_ok() {
                Err(SqlxOptionsError::EnvConflict {
                    env_key: env_key.to_string(),
                    field_name: field_name.to_string(),
                })
            } else {
                Ok(())
            }
        }

        fn unsupported_env(env_key: &str, field_name: &str) -> Result<(), SqlxOptionsError> {
            if std::env::var(env_key).is_ok() {
                Err(SqlxOptionsError::UnsupportedFeature {
                    env_key: env_key.to_string(),
                    field_name: field_name.to_string(),
                })
            } else {
                Ok(())
            }
        }

        // This is the "least powerful" API available to create a `PgConnectOptions`
        // instance. Still it does ENV variable snooping and we below try hard to
        // reset all of that snooped variables.
        let mut options = sqlx::postgres::PgConnectOptions::new_without_pgpass();

        unsupported_env("PGSSLKEY", "ssl_client_key")?;
        unsupported_env("PGSSLCERT", "ssl_client_cert")?;
        unsupported_env("PGOPTIONS", "options")?;

        options = options.database(self.database.as_str());

        match &self.endpoint {
            Endpoint::Network {
                host,
                host_addr,
                port,
            } => {
                options = options.host(&host.to_pg_env_value());
                if let Some(port) = port {
                    options = options.port(port.into());
                } else {
                    reject_env("PGPORT", "port")?;
                }
                if let Some(host_addr) = host_addr {
                    options = options.host_addr(&host_addr.to_string())
                } else {
                    reject_env("PGHOSTADDR", "hostaddr")?;
                }
            }
            Endpoint::SocketPath(path) => {
                options = options.host(path.to_str().expect("socket path contains invalid utf8"));
                reject_env("PGPORT", "port")?;
                reject_env("PGHOSTADDR", "hostaddr")?;
            }
        }

        options = options.ssl_mode(self.ssl_mode.to_sqlx_ssl_mode());
        options = options.username(self.username.as_str());

        if let Some(application_name) = &self.application_name {
            options = options.application_name(application_name.as_str());
        } else {
            reject_env("PGAPPNAME", "application_name")?;
        }

        if let Some(password) = &self.password {
            options = options.password(password.as_str());
        } else {
            reject_env("PGPASSWORD", "password")?;
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            options = options.ssl_root_cert(ssl_root_cert.to_pg_env_value());
        } else {
            reject_env("PGSSLROOTCERT", "ssl_root_cert")?;
        }

        Ok(options)
    }

    pub async fn with_sqlx_connection<T, F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> T>(
        &self,
        mut action: F,
    ) -> Result<T, SqlxConnectionError> {
        let config = self.to_sqlx_connect_options()?;

        let mut connection = sqlx::ConnectOptions::connect(&config)
            .await
            .map_err(SqlxConnectionError::Connect)?;

        let result = action(&mut connection).await;

        sqlx::Connection::close(connection)
            .await
            .map_err(SqlxConnectionError::Close)?;

        Ok(result)
    }

    #[must_use]
    pub fn endpoint(self, endpoint: Endpoint) -> Self {
        Self { endpoint, ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

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
    fn database_lt_min_length() {
        let value = String::new();

        let err = Database::from_str(&value).expect_err("expected min length failure");

        assert_eq!(err, "Database byte min length: 1 violated, got: 0");
    }

    #[test]
    fn database_eq_min_length() {
        let value = repeat('d', 1);

        let database = Database::from_str(&value).expect("expected valid min length value");

        assert_eq!(database, Database(value));
    }

    #[test]
    fn database_gt_min_length() {
        let value = repeat('d', 2);

        let database = Database::from_str(&value).expect("expected valid value greater than min");

        assert_eq!(database, Database(value));
    }

    #[test]
    fn database_lt_max_length() {
        let value = repeat('d', 62);

        let database = Database::from_str(&value).expect("expected valid value less than max");

        assert_eq!(database, Database(value));
    }

    #[test]
    fn database_eq_max_length() {
        let value = repeat('d', 63);

        let database = Database::from_str(&value).expect("expected valid value equal to max");

        assert_eq!(database, Database(value));
    }

    #[test]
    fn database_gt_max_length() {
        let value = repeat('d', 64);

        let err = Database::from_str(&value).expect_err("expected max length failure");

        assert_eq!(err, "Database byte max length: 63 violated, got: 64");
    }

    #[test]
    fn database_contains_nul() {
        let value = String::from('\0');

        let err = Database::from_str(&value).expect_err("expected NUL failure");

        assert_eq!(err, "Database contains NUL byte");
    }

    #[test]
    fn username_lt_min_length() {
        let value = String::new();

        let err = Username::from_str(&value).expect_err("expected min length failure");

        assert_eq!(err, "Username byte min length: 1 violated, got: 0");
    }

    #[test]
    fn username_eq_min_length() {
        let value = repeat('u', 1);

        let username = Username::from_str(&value).expect("expected valid min length value");

        assert_eq!(username, Username(value));
    }

    #[test]
    fn username_gt_min_length() {
        let value = repeat('u', 2);

        let username = Username::from_str(&value).expect("expected valid value greater than min");

        assert_eq!(username, Username(value));
    }

    #[test]
    fn username_lt_max_length() {
        let value = repeat('u', 62);

        let username = Username::from_str(&value).expect("expected valid value less than max");

        assert_eq!(username, Username(value));
    }

    #[test]
    fn username_eq_max_length() {
        let value = repeat('u', 63);

        let username = Username::from_str(&value).expect("expected valid value equal to max");

        assert_eq!(username, Username(value));
    }

    #[test]
    fn username_gt_max_length() {
        let value = repeat('u', 64);

        let err = Username::from_str(&value).expect_err("expected max length failure");

        assert_eq!(err, "Username byte max length: 63 violated, got: 64");
    }

    #[test]
    fn username_contains_nul() {
        let value = String::from('\0');

        let err = Username::from_str(&value).expect_err("expected NUL failure");

        assert_eq!(err, "Username contains NUL byte");
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
            database: Database::from_str("some-database").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::from_str("some-host").unwrap(),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: None,
            username: Username::from_str("some-username").unwrap(),
        };

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-username@some-host:5432/some-database?sslmode=verify-full",
                "username": "some-username",
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
                "url": "postgres://some-username:some-password@some-host:5432/some-database?application_name=some-app&sslmode=verify-full&sslrootcert=%2Fsome.pem",
                "username": "some-username"
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
                "url": "postgres://some-username@127.0.0.1:5432/some-database?sslmode=verify-full",
                "username": "some-username"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("127.0.0.1").unwrap(),
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
                "url": "postgres://?host=%2Fsome%2Fsocket&dbname=some-database&user=some-username&sslmode=verify-full",
                "username": "some-username"
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
                "url": "postgres://some-username@some-host:5432/some-database?sslmode=verify-full&sslrootcert=system",
                "username": "some-username"
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
                "url": "postgres://some-username@some-host:5432/some-database?hostaddr=192.168.1.100&sslmode=verify-full",
                "username": "some-username"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
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
                "url": "postgres://some-username@some-host/some-database?sslmode=verify-full",
                "username": "some-username"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
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
                "url": "postgres://some-username@some-host/some-database?hostaddr=10.0.0.1&sslmode=verify-full",
                "username": "some-username"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
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
            database: Database::from_str("testdb").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            username: Username::from_str("postgres").unwrap(),
        };

        let url = config_ipv6_loopback.to_url();
        assert_eq!(
            url.to_string(),
            "postgres://postgres@[::1]:5432/testdb?sslmode=disable",
            "IPv6 loopback address should be bracketed in URL"
        );

        // Test fe80 link-local IPv6 address
        let config_ipv6_fe80 = Config {
            application_name: None,
            database: Database::from_str("testdb").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0xfe80, 0, 0, 0, 0, 0, 0, 1,
                ))),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            username: Username::from_str("postgres").unwrap(),
        };

        let url = config_ipv6_fe80.to_url();
        assert_eq!(
            url.to_string(),
            "postgres://postgres@[fe80::1]:5432/testdb?sslmode=disable",
            "IPv6 link-local address should be bracketed in URL"
        );

        // Test full IPv6 address
        let config_ipv6_full = Config {
            application_name: None,
            database: Database::from_str("testdb").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x2001, 0x0db8, 0, 0, 0, 0, 0, 1,
                ))),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            username: Username::from_str("postgres").unwrap(),
        };

        let url = config_ipv6_full.to_url();
        assert_eq!(
            url.to_string(),
            "postgres://postgres@[2001:db8::1]:5432/testdb?sslmode=disable",
            "Full IPv6 address should be bracketed in URL"
        );

        // Test IPv4 address (should NOT be bracketed)
        let config_ipv4 = Config {
            application_name: None,
            database: Database::from_str("testdb").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            username: Username::from_str("postgres").unwrap(),
        };

        let url = config_ipv4.to_url();
        assert_eq!(
            url.to_string(),
            "postgres://postgres@127.0.0.1:5432/testdb?sslmode=disable",
            "IPv4 address should NOT be bracketed in URL"
        );

        // Test hostname (should NOT be bracketed)
        let config_hostname = Config {
            application_name: None,
            database: Database::from_str("testdb").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::from_str("localhost").unwrap(),
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            username: Username::from_str("postgres").unwrap(),
        };

        let url = config_hostname.to_url();
        assert_eq!(
            url.to_string(),
            "postgres://postgres@localhost:5432/testdb?sslmode=disable",
            "Hostname should NOT be bracketed in URL"
        );
    }
}
