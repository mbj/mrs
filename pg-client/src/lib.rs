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
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Database(String);

from_str_impl!(Database);

impl Database {
    const MAX_LENGTH: usize = 63;

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(String);

from_str_impl!(Password);

impl Password {
    const MAX_LENGTH: usize = 4096;

    fn to_pg_env_value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub type SslMode = sqlx::postgres::PgSslMode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SslRootCert {
    File(file_buf::FileBuf),
    System,
}

impl SslRootCert {
    pub fn from_path_unchecked_existance(path: std::path::PathBuf) -> Option<SslRootCert> {
        file_buf::FileBuf::from_path_unchecked_existance(path).map(Self::File)
    }

    fn to_pg_env_value(&self) -> String {
        match self {
            Self::File(file) => file.as_ref().to_str().unwrap().to_string(),
            Self::System => "system".to_string(),
        }
    }
}

impl From<file_buf::FileBuf> for SslRootCert {
    fn from(value: file_buf::FileBuf) -> Self {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub database: Option<Database>,
    pub host: Host,
    pub password: Option<Password>,
    pub port: Port,
    pub ssl_mode: SslMode,
    pub ssl_root_cert: Option<SslRootCert>,
    pub username: Username,
}

impl Config {
    pub fn to_pg_env(&self) -> std::collections::BTreeMap<&'static str, String> {
        let mut map = std::collections::BTreeMap::new();

        map.insert("PGHOST", self.host.to_pg_env_value());
        map.insert("PGPORT", self.port.to_pg_env_value());
        map.insert("PGSSLMODE", self.ssl_mode.to_static_str().to_string());
        map.insert("PGUSER", self.username.to_pg_env_value());

        if let Some(database) = &self.database {
            map.insert("PGDATABASE", database.to_pg_env_value());
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
    ///     database: Some(Database::from_str("some_db").unwrap()),
    ///     host: Host::from_str("some-host").unwrap(),
    ///     password: Some(Password::from_str("some_password").unwrap()),
    ///     port: Port(5432),
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: Some(SslRootCert::File(
    ///         file_buf::FileBuf::from_path_unchecked_existance("/some.pem".into()).unwrap(),
    ///     )),
    ///     username: Username::from_str("some_username").unwrap(),
    /// };
    ///
    /// assert_eq!(
    ///     sqlx::postgres::PgConnectOptions::new_without_environment(
    ///         "some-host".to_string(),
    ///         5432,
    ///         "some_username".to_string(),
    ///         SslMode::VerifyFull
    ///     )
    ///     .database("some_db")
    ///     .password("some_password")
    ///     .ssl_root_cert("/some.pem"),
    ///     config.to_sqlx_connect_options()
    /// )
    /// ```
    pub fn to_sqlx_connect_options(&self) -> sqlx::postgres::PgConnectOptions {
        let mut options = sqlx::postgres::PgConnectOptions::new_without_environment(
            self.host.to_pg_env_value(),
            self.port.to_u16(),
            self.username.to_pg_env_value(),
            self.ssl_mode,
        );

        if let Some(database) = &self.database {
            options = options.database(&database.to_pg_env_value());
        }

        if let Some(password) = &self.password {
            options = options.password(&password.to_pg_env_value());
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            options = options.ssl_root_cert(ssl_root_cert.to_pg_env_value());
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
