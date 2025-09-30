/// Macro to generate `std::str::FromStr` for string wrapped newtypes
macro_rules! from_str_impl {
    ($struct: ident) => {
        impl std::str::FromStr for $struct {
            type Err = &'static str;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let max_length = Self::MAX_LENGTH;
                let actual = value.len();

                if actual > max_length {
                    Err(concat!(
                        stringify!($struct),
                        " byte max length: {max_length} violated, got: {actual}"
                    ))
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
    };
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct HostName(String);

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Host {
    HostName(HostName),
    SocketAddr(std::net::SocketAddr),
    SocketPath(std::path::PathBuf),
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
            Self::SocketAddr(value) => value.to_string(),
            Self::SocketPath(value) => value
                .to_str()
                .expect("socket path contains invalid utf8")
                .to_string(),
        }
    }
}

impl std::str::FromStr for Host {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match <std::net::SocketAddr as std::str::FromStr>::from_str(value) {
            Ok(addr) => Ok(Self::SocketAddr(addr)),
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Port(pub u16);

impl std::str::FromStr for Port {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match <u16 as std::str::FromStr>::from_str(value) {
            Ok(port) => Ok(Port(port)),
            Err(_) => Err("invalid postgresql port string"),
        }
    }
}

impl Port {
    fn to_pg_env_value(&self) -> String {
        self.0.to_string()
    }

    fn to_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ApplicationName(String);
from_str_impl!(ApplicationName);

#[macro_export]
macro_rules! application_name {
    ($string: literal) => {
        <pg_client::ApplicationName as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl ApplicationName {
    const MAX_LENGTH: usize = 63;

    fn as_str(&self) -> &str {
        &self.0
    }

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Database(String);

from_str_impl!(Database);

#[macro_export]
macro_rules! database {
    ($string: literal) => {
        <pg_client::Database as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl Database {
    const MAX_LENGTH: usize = 63;

    fn as_str(&self) -> &str {
        &self.0
    }

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Username(String);

from_str_impl!(Username);

#[macro_export]
macro_rules! username {
    ($string: literal) => {
        <pg_client::Username as std::str::FromStr>::from_str($string).unwrap()
    };
}

impl Username {
    const MAX_LENGTH: usize = 63;

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Password(String);

from_str_impl!(Password);

impl Password {
    const MAX_LENGTH: usize = 4096;

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SslMode {
    Allow,
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Disable => "disable",
            Self::Allow => "allow",
            Self::Prefer => "prefer",
            Self::Require => "require",
            Self::VerifyCa => "verify-ca",
            Self::VerifyFull => "verify-full",
        }
    }

    fn to_sqlx_ssl_mode(&self) -> sqlx::postgres::PgSslMode {
        use sqlx::postgres::PgSslMode;

        match self {
            Self::Disable => PgSslMode::Disable,
            Self::Allow => PgSslMode::Allow,
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

#[macro_export]
macro_rules! ssl_root_cert_file {
    ($string: literal) => {
        <pg_client::FileBuf as std::str::FromStr>::from_str($string)
            .map(pg_client::SslRootCert::File)
            .unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Config {
    pub application_name: Option<ApplicationName>,
    pub database: Database,
    pub host: Host,
    pub password: Option<Password>,
    pub port: Port,
    pub ssl_mode: SslMode,
    pub ssl_root_cert: Option<SslRootCert>,
    pub username: Username,
}

impl Config {
    /// Convert to PG environment variable names
    pub fn to_pg_env(&self) -> std::collections::BTreeMap<&'static str, String> {
        let mut map = std::collections::BTreeMap::new();

        map.insert("PGHOST", self.host.to_pg_env_value());
        map.insert("PGPORT", self.port.to_pg_env_value());
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

    /// Convert to an SQL pg connection config
    ///
    /// ```
    /// # use pg_client::*;
    /// # use std::str::FromStr;
    ///
    /// let config = Config {
    ///     application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///     database: Database::from_str("some-database").unwrap(),
    ///     host: Host::from_str("some-host").unwrap(),
    ///     password: Some(Password::from_str("some-password").unwrap()),
    ///     port: Port(5432),
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     username: Username::from_str("some-username").unwrap(),
    /// };
    ///
    /// let options = config.to_sqlx_connect_options();
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
    /// # Panics
    ///
    /// Function may panic if fields inferred  from the process environment variables
    /// infered by `PgConnectOptions::new` contradict the settings in `PgConfig`, and
    /// there is no public API in `PgConnectOptions` to reset these values.
    pub fn to_sqlx_connect_options(&self) -> sqlx::postgres::PgConnectOptions {
        fn reject_env(env_key: &str, field_name: &str) {
            if std::env::var(env_key).is_ok() {
                panic!(
                    "`PgConnectOptions::new` has inferred a `{field_name}` from `{env_key}` environment variable, but `pg_client::Config` does not specify a `{field_name}` value. `PgConnectOptions` does not provide an API to construct an instance without inferring from the environment, does not provide an API to unset the field, we have to bail out at this point. Please remove the environment variable!"
                )
            }
        }

        fn unsupported_env(env_key: &str, field_name: &str) {
            if std::env::var(env_key).is_ok() {
                panic!(
                    "`PgConnnectOptions::new` has inferred `{field_name}` from the `{env_key}` environment variable, but `pg_client::Config` does not support that feature at this point. As `PgConnectOptions` has no option to unset that field, or a constructor that allows us to bypass the inference: we have to bail out, please remove the environment variable!"
                )
            }
        }

        // This is the "least powerful" API available to create a `PgConnectOptions`
        // instance. Still it does ENV variable snooping and we below try hard to
        // reset all of that snooped variables.
        let mut options = sqlx::postgres::PgConnectOptions::new_without_pgpass();

        unsupported_env("PGSSLKEY", "ssl_client_key");
        unsupported_env("PGSSLCERT", "ssl_client_cert");
        unsupported_env("PGOPTIONS", "options");

        options = options.database(self.database.as_str());
        options = options.host(&self.host.to_pg_env_value());
        options = options.port(self.port.to_u16());
        options = options.ssl_mode(self.ssl_mode.to_sqlx_ssl_mode());
        options = options.username(self.username.as_str());

        if let Some(application_name) = &self.application_name {
            options = options.application_name(application_name.as_str());
        } else {
            reject_env("PGAPPNAME", "application_name");
        }

        if let Some(password) = &self.password {
            options = options.password(password.as_str());
        } else {
            reject_env("PGPASSWORD", "password");
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            options = options.ssl_root_cert(ssl_root_cert.to_pg_env_value());
        } else {
            reject_env("PGSSLROOTCERT", "ssl_root_cert")
        }

        options
    }

    pub async fn with_sqlx_connection<T, F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> T>(
        &self,
        mut action: F,
    ) -> T {
        let config = self.to_sqlx_connect_options();

        let mut connection = sqlx::ConnectOptions::connect(&config).await.unwrap();

        let result = action(&mut connection).await;

        sqlx::Connection::close(connection).await.unwrap();

        result
    }
}
