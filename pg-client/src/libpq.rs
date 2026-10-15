pub mod env;
pub mod url;

use crate::config::{ChannelBinding, Host, HostAddr, Password, Port};
use crate::name::{Database, User};

#[derive(Clone, Debug, PartialEq, Eq)]
/// PG connection config with various presentation modes.
///
/// Supported:
///
/// 1. Env variables via `pg_env()`
/// 2. JSON document via `serde`
/// 3. Individual field access
pub struct Config {
    pub endpoint: Endpoint,
    pub session: Session,
    pub ssl_mode: SslMode,
    pub ssl_root_cert: Option<SslRootCert>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Endpoint {
    Network {
        host: Host,
        channel_binding: Option<ChannelBinding>,
        host_addr: Option<HostAddr>,
        port: Option<Port>,
    },
    SocketPath {
        path: std::path::PathBuf,
        port: Option<Port>,
    },
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
            Self::SocketPath { path, port } => {
                let mut state = serializer.serialize_struct("Endpoint", 2)?;
                state.serialize_field(
                    "socket_path",
                    &path.to_str().expect("socket path contains invalid utf8"),
                )?;
                if let Some(port) = port {
                    state.serialize_field("port", port)?;
                }
                state.end()
            }
        }
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SslRootCert {
    File(std::path::PathBuf),
    System,
}

impl SslRootCert {
    pub(crate) fn pg_env_value(&self) -> String {
        match self {
            Self::File(path) => path
                .to_str()
                .expect("root certificate path contains invalid utf8")
                .to_string(),
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
    pub application_name: Option<crate::parameter::Value>,
    pub database: Database<'static>,
    pub password: Option<Password>,
    pub user: User<'static>,
}

impl serde::Serialize for Config {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Config", 8)?;

        if let Some(application_name) = &self.session.application_name {
            state.serialize_field("application_name", application_name)?;
        }

        state.serialize_field("database", &self.session.database)?;
        state.serialize_field("endpoint", &self.endpoint)?;

        if let Some(password) = &self.session.password {
            state.serialize_field("password", password)?;
        }

        state.serialize_field("ssl_mode", &self.ssl_mode)?;

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            state.serialize_field("ssl_root_cert", ssl_root_cert)?;
        }

        state.serialize_field("user", &self.session.user)?;
        state.serialize_field("url", &self.to_url())?;

        state.end()
    }
}

impl Config {
    /// Convert to PG connection URL
    ///
    /// ```
    /// # use pg_client::config::*;
    /// # use pg_client::libpq::*;
    /// # use pg_client::*;
    /// # use std::str::FromStr;
    ///
    /// let config = Config {
    ///     endpoint: Endpoint::Network {
    ///         host: Host::from_str("some-host").unwrap(),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("some-database"),
    ///         password: None,
    ///         user: User::from_static_or_panic("some-user"),
    ///     },
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    /// };
    ///
    /// assert_eq!(
    ///     config.to_url(),
    ///     "postgres://some-user@some-host:5432/some-database?sslmode=verify-full"
    /// );
    ///
    /// assert_eq!(
    ///     Config {
    ///         session: Session {
    ///             application_name: Some(pg_client::parameter::Value::from_static_or_panic("some-app")),
    ///             password: Some(Password::from_str("some-password").unwrap()),
    ///             ..config.session.clone()
    ///         },
    ///         ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///         ..config.clone()
    ///     }.to_url(),
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
    ///     }.to_url(),
    ///     "postgres://some-user@some-host:5432/some-database?hostaddr=127.0.0.1&sslmode=verify-full"
    /// );
    ///
    /// // IPv4 example
    /// let ipv4_config = Config {
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("mydb"),
    ///         password: None,
    ///         user: User::from_static_or_panic("user"),
    ///     },
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    /// };
    /// assert_eq!(
    ///     ipv4_config.to_url(),
    ///     "postgres://user@127.0.0.1:5432/mydb?sslmode=disable"
    /// );
    ///
    /// // IPv6 example (automatically bracketed)
    /// let ipv6_config = Config {
    ///     endpoint: Endpoint::Network {
    ///         host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("mydb"),
    ///         password: None,
    ///         user: User::from_static_or_panic("user"),
    ///     },
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    /// };
    /// assert_eq!(
    ///     ipv6_config.to_url(),
    ///     "postgres://user@[::1]:5432/mydb?sslmode=disable"
    /// );
    /// ```
    #[must_use]
    pub fn to_url(&self) -> String {
        use crate::config::Host;
        use url::{append_query_pair, encode_into};

        let mut value = String::from("postgres://");
        let mut query = String::new();

        match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                encode_into(&mut value, self.session.user.pg_env_value().as_str());
                if let Some(password) = &self.session.password {
                    value.push(':');
                    encode_into(&mut value, password.as_str());
                }
                value.push('@');

                match host {
                    Host::IpAddr(std::net::IpAddr::V4(addr)) => {
                        value.push_str(&addr.to_string());
                    }
                    Host::IpAddr(std::net::IpAddr::V6(addr)) => {
                        value.push_str(&format!("[{addr}]"));
                    }
                    Host::HostName(name) => encode_into(&mut value, name.as_str()),
                }

                if let Some(port) = port {
                    value.push_str(&format!(":{}", u16::from(port)));
                }

                value.push('/');
                encode_into(&mut value, self.session.database.as_str());

                if let Some(addr) = host_addr {
                    append_query_pair(&mut query, "hostaddr", &addr.to_string());
                }
                if let Some(channel_binding) = channel_binding {
                    append_query_pair(&mut query, "channel_binding", channel_binding.as_str());
                }
            }
            Endpoint::SocketPath { path, port } => {
                append_query_pair(
                    &mut query,
                    "host",
                    path.to_str().expect("socket path contains invalid utf8"),
                );
                if let Some(port) = port {
                    append_query_pair(&mut query, "port", &port.pg_env_value());
                }
                append_query_pair(&mut query, "dbname", self.session.database.as_str());
                append_query_pair(
                    &mut query,
                    "user",
                    self.session.user.pg_env_value().as_str(),
                );
                if let Some(password) = &self.session.password {
                    append_query_pair(&mut query, "password", password.as_str());
                }
            }
        }

        self.append_common_query_params(&mut query);

        value.push('?');
        value.push_str(&query);

        value
    }

    fn append_common_query_params(&self, query: &mut String) {
        use url::append_query_pair;

        if let Some(application_name) = &self.session.application_name {
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
    /// # use pg_client::config::*;
    /// # use pg_client::libpq::env::*;
    /// # use pg_client::libpq::*;
    /// # use pg_client::*;
    /// # use std::collections::BTreeMap;
    ///
    /// let config = Config {
    ///     endpoint: Endpoint::Network {
    ///         host: "some-host".parse().unwrap(),
    ///         channel_binding: None,
    ///         host_addr: None,
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: "some-database".parse().unwrap(),
    ///         password: None,
    ///         user: "some-user".parse().unwrap(),
    ///     },
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     (PGDATABASE, "some-database".parse().unwrap()),
    ///     (PGHOST, "some-host".parse().unwrap()),
    ///     (PGPORT, "5432".parse().unwrap()),
    ///     (PGSSLMODE, "verify-full".parse().unwrap()),
    ///     (PGUSER, "some-user".parse().unwrap()),
    /// ]);
    ///
    /// assert_eq!(expected, config.pg_env().unwrap());
    ///
    /// let config_with_optionals = Config {
    ///     endpoint: Endpoint::Network {
    ///         host: "some-host".parse().unwrap(),
    ///         channel_binding: None,
    ///         host_addr: Some("127.0.0.1".parse().unwrap()),
    ///         port: Some(Port::new(5432)),
    ///     },
    ///     session: Session {
    ///         application_name: Some("some-app".parse().unwrap()),
    ///         password: Some("some-password".parse().unwrap()),
    ///         ..config.session.clone()
    ///     },
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     ..config
    /// };
    ///
    /// let expected = BTreeMap::from([
    ///     (PGAPPNAME, "some-app".parse().unwrap()),
    ///     (PGDATABASE, "some-database".parse().unwrap()),
    ///     (PGHOST, "some-host".parse().unwrap()),
    ///     (PGHOSTADDR, "127.0.0.1".parse().unwrap()),
    ///     (PGPASSWORD, "some-password".parse().unwrap()),
    ///     (PGPORT, "5432".parse().unwrap()),
    ///     (PGSSLMODE, "verify-full".parse().unwrap()),
    ///     (PGSSLROOTCERT, "/some.pem".parse().unwrap()),
    ///     (PGUSER, "some-user".parse().unwrap()),
    /// ]);
    ///
    /// assert_eq!(expected, config_with_optionals.pg_env().unwrap());
    /// ```
    pub fn pg_env(
        &self,
    ) -> Result<
        std::collections::BTreeMap<cmd_proc::EnvVariableName, cmd_proc::EnvVariableValue>,
        cmd_proc::EnvVariableValueError,
    > {
        let mut map = std::collections::BTreeMap::new();

        match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                map.insert(
                    env::PGHOST.clone(),
                    cmd_proc::EnvVariableValue::try_from(host.pg_env_value())?,
                );
                if let Some(port) = port {
                    map.insert(
                        env::PGPORT.clone(),
                        cmd_proc::EnvVariableValue::try_from(port.pg_env_value())?,
                    );
                }
                if let Some(channel_binding) = channel_binding {
                    map.insert(
                        env::PGCHANNELBINDING.clone(),
                        cmd_proc::EnvVariableValue::try_from(channel_binding.pg_env_value())?,
                    );
                }
                if let Some(addr) = host_addr {
                    map.insert(
                        env::PGHOSTADDR.clone(),
                        cmd_proc::EnvVariableValue::try_from(addr.to_string())?,
                    );
                }
            }
            Endpoint::SocketPath { path, port } => {
                map.insert(
                    env::PGHOST.clone(),
                    cmd_proc::EnvVariableValue::try_from(
                        path.to_str()
                            .expect("socket path contains invalid utf8")
                            .to_string(),
                    )?,
                );
                if let Some(port) = port {
                    map.insert(
                        env::PGPORT.clone(),
                        cmd_proc::EnvVariableValue::try_from(port.pg_env_value())?,
                    );
                }
            }
        }

        map.insert(
            env::PGSSLMODE.clone(),
            cmd_proc::EnvVariableValue::try_from(self.ssl_mode.pg_env_value())?,
        );
        map.insert(
            env::PGUSER.clone(),
            cmd_proc::EnvVariableValue::try_from(self.session.user.pg_env_value())?,
        );
        map.insert(
            env::PGDATABASE.clone(),
            cmd_proc::EnvVariableValue::try_from(self.session.database.pg_env_value())?,
        );

        if let Some(application_name) = &self.session.application_name {
            map.insert(
                env::PGAPPNAME.clone(),
                cmd_proc::EnvVariableValue::try_from(application_name.as_str().to_owned())?,
            );
        }

        if let Some(password) = &self.session.password {
            map.insert(
                env::PGPASSWORD.clone(),
                cmd_proc::EnvVariableValue::try_from(password.pg_env_value())?,
            );
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            map.insert(
                env::PGSSLROOTCERT.clone(),
                cmd_proc::EnvVariableValue::try_from(ssl_root_cert.pg_env_value())?,
            );
        }

        Ok(map)
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
    pub fn from_url(url: &str) -> Result<Self, url::ParseError> {
        url::parse(url)
    }

    pub fn to_session_config(&self) -> Result<crate::session::Config, SessionConfigError> {
        let password = self
            .session
            .password
            .clone()
            .ok_or(SessionConfigError::PasswordMissing)?;

        let endpoint = match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => crate::connection::Endpoint::Tcp {
                host: host.clone(),
                address: host_addr.clone(),
                port: port.unwrap_or(DEFAULT_PORT),
                tls: self.tls(channel_binding.unwrap_or(ChannelBinding::Prefer))?,
            },
            Endpoint::SocketPath { path, port } => crate::connection::Endpoint::Unix {
                path: path.join(format!(
                    ".s.PGSQL.{}",
                    u16::from(port.unwrap_or(DEFAULT_PORT))
                )),
            },
        };

        let mut parameters = crate::session::Parameters::new();

        if let Some(application_name) = &self.session.application_name {
            parameters.set_application_name(
                application_name
                    .as_str()
                    .parse()
                    .map_err(SessionConfigError::ApplicationName)?,
            );
        }

        Ok(crate::session::Config {
            connection: crate::connection::Config {
                endpoint,
                receive_buffer: crate::protocol::ReceiveBufferSize::DEFAULT,
            },
            authentication: crate::authentication::Config {
                user: self.session.user.clone(),
                password: crate::authentication::Password::Static(password),
            },
            database: self.session.database.clone(),
            parameters,
        })
    }

    fn tls(
        &self,
        channel_binding: ChannelBinding,
    ) -> Result<Option<crate::connection::Tls>, SessionConfigError> {
        match self.ssl_mode {
            SslMode::Disable => {
                if channel_binding == ChannelBinding::Require {
                    Err(SessionConfigError::ChannelBindingWithoutTls)
                } else {
                    Ok(None)
                }
            }
            SslMode::VerifyFull => match &self.ssl_root_cert {
                None => Err(SessionConfigError::RootCertificateMissing),
                Some(SslRootCert::System) => Err(SessionConfigError::SystemRootCertificates),
                Some(SslRootCert::File(path)) => tls_from_file(path, channel_binding).map(Some),
            },
            ssl_mode
            @ (SslMode::Allow | SslMode::Prefer | SslMode::Require | SslMode::VerifyCa) => {
                Err(SessionConfigError::UnsupportedSslMode { ssl_mode })
            }
        }
    }
}

const DEFAULT_PORT: Port = Port::new(5432);

fn tls_from_file(
    path: &std::path::Path,
    channel_binding: ChannelBinding,
) -> Result<crate::connection::Tls, SessionConfigError> {
    use crate::connection::rustls::pki_types::{CertificateDer, pem::PemObject as _};

    CertificateDer::pem_file_iter(path)
        .map_err(crate::connection::RootCertificateError::Read)
        .and_then(|certificates| {
            crate::connection::Tls::from_certificates(certificates, channel_binding)
        })
        .map_err(|source| SessionConfigError::RootCertificate {
            path: path.to_owned(),
            source,
        })
}

#[derive(Debug, thiserror::Error)]
pub enum SessionConfigError {
    #[error("a session needs a password")]
    PasswordMissing,
    #[error("sslmode={} cannot be expressed as a session", ssl_mode.as_str())]
    UnsupportedSslMode { ssl_mode: SslMode },
    #[error("sslmode=verify-full needs sslrootcert")]
    RootCertificateMissing,
    #[error("sslrootcert=system cannot be expressed as a session")]
    SystemRootCertificates,
    #[error("root certificate {path:?} cannot be used")]
    RootCertificate {
        path: std::path::PathBuf,
        #[source]
        source: crate::connection::RootCertificateError,
    },
    #[error("channel_binding=require needs TLS")]
    ChannelBindingWithoutTls,
    #[error("application_name would be rewritten by the server")]
    ApplicationName(#[source] crate::name::CleanAsciiError),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    const TEST_DATABASE: Database = Database::from_static_or_panic("some-database");
    const TEST_USER: User = User::from_static_or_panic("some-user");

    fn assert_config(expected: serde_json::Value, config: &Config) {
        assert_eq!(expected, serde_json::to_value(config).unwrap());
    }

    #[test]
    fn test_json() {
        let config = Config {
            endpoint: Endpoint::Network {
                host: Host::from_str("some-host").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: TEST_USER,
            },
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: None,
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
                session: Session {
                    application_name: Some(crate::parameter::Value::from_static_or_panic(
                        "some-app",
                    )),
                    password: Some(Password::from_str("some-password").unwrap()),
                    ..config.session.clone()
                },
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
                endpoint: Endpoint::SocketPath {
                    path: "/some/socket".into(),
                    port: None,
                },
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
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
        };

        assert_eq!(
            config_ipv6_loopback.to_url(),
            "postgres://postgres@[::1]:5432/some-database?sslmode=disable",
            "IPv6 loopback address should be bracketed in URL"
        );

        // Test fe80 link-local IPv6 address
        let config_ipv6_fe80 = Config {
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0xfe80, 0, 0, 0, 0, 0, 0, 1,
                ))),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
        };

        assert_eq!(
            config_ipv6_fe80.to_url(),
            "postgres://postgres@[fe80::1]:5432/some-database?sslmode=disable",
            "IPv6 link-local address should be bracketed in URL"
        );

        // Test full IPv6 address
        let config_ipv6_full = Config {
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x2001, 0x0db8, 0, 0, 0, 0, 0, 1,
                ))),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
        };

        assert_eq!(
            config_ipv6_full.to_url(),
            "postgres://postgres@[2001:db8::1]:5432/some-database?sslmode=disable",
            "Full IPv6 address should be bracketed in URL"
        );

        // Test IPv4 address (should NOT be bracketed)
        let config_ipv4 = Config {
            endpoint: Endpoint::Network {
                host: Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
        };

        assert_eq!(
            config_ipv4.to_url(),
            "postgres://postgres@127.0.0.1:5432/some-database?sslmode=disable",
            "IPv4 address should NOT be bracketed in URL"
        );

        // Test hostname (should NOT be bracketed)
        let config_hostname = Config {
            endpoint: Endpoint::Network {
                host: Host::from_str("localhost").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
        };

        assert_eq!(
            config_hostname.to_url(),
            "postgres://postgres@localhost:5432/some-database?sslmode=disable",
            "Hostname should NOT be bracketed in URL"
        );
    }

    mod to_session_config {
        use super::*;
        use pretty_assertions::assert_eq;

        fn config(endpoint: Endpoint, ssl_mode: SslMode) -> Config {
            Config {
                endpoint,
                session: Session {
                    application_name: None,
                    database: TEST_DATABASE,
                    password: Some("some-password".parse().unwrap()),
                    user: TEST_USER,
                },
                ssl_mode,
                ssl_root_cert: None,
            }
        }

        fn network(channel_binding: Option<ChannelBinding>, port: Option<Port>) -> Endpoint {
            Endpoint::Network {
                host: Host::from_str("some-host").unwrap(),
                channel_binding,
                host_addr: Some("10.0.0.1".parse().unwrap()),
                port,
            }
        }

        fn unix_path(config: &Config) -> std::path::PathBuf {
            match config.to_session_config().unwrap().connection.endpoint {
                crate::connection::Endpoint::Unix { path } => path,
                crate::connection::Endpoint::Tcp { .. } => panic!("expected a Unix endpoint"),
            }
        }

        #[test]
        fn converts_a_network_config_without_tls() {
            let config = Config {
                session: Session {
                    application_name: Some("some-app".parse().unwrap()),
                    ..config(network(None, None), SslMode::Disable).session
                },
                ..config(network(None, None), SslMode::Disable)
            };

            let session = config.to_session_config().unwrap();

            let crate::connection::Endpoint::Tcp {
                host,
                address,
                port,
                tls,
            } = session.connection.endpoint
            else {
                panic!("expected a TCP endpoint");
            };
            assert_eq!(host, Host::from_str("some-host").unwrap());
            assert_eq!(address, Some("10.0.0.1".parse().unwrap()));
            assert_eq!(port, Port::new(5432));
            assert!(tls.is_none());

            assert_eq!(session.authentication.user, TEST_USER);
            let crate::authentication::Password::Static(password) = session.authentication.password
            else {
                panic!("expected a static password");
            };
            assert_eq!(password, "some-password".parse().unwrap());

            assert_eq!(session.database, TEST_DATABASE);
            let mut parameters = crate::session::Parameters::new();
            parameters.set_application_name("some-app".parse().unwrap());
            assert_eq!(session.parameters, parameters);
        }

        #[test]
        fn connects_to_the_socket_of_the_port() {
            let endpoint = |port| Endpoint::SocketPath {
                path: "/var/run/postgresql".into(),
                port,
            };

            assert_eq!(
                unix_path(&config(endpoint(Some(Port::new(5433))), SslMode::Disable)),
                std::path::PathBuf::from("/var/run/postgresql/.s.PGSQL.5433")
            );
            assert_eq!(
                unix_path(&config(endpoint(None), SslMode::Disable)),
                std::path::PathBuf::from("/var/run/postgresql/.s.PGSQL.5432")
            );
        }

        #[test]
        fn refuses_an_absent_password() {
            let config = Config {
                session: Session {
                    password: None,
                    ..config(network(None, None), SslMode::Disable).session
                },
                ..config(network(None, None), SslMode::Disable)
            };

            assert!(matches!(
                config.to_session_config(),
                Err(SessionConfigError::PasswordMissing)
            ));
        }

        #[test]
        fn refuses_ssl_modes_a_session_cannot_express() {
            for ssl_mode in [
                SslMode::Allow,
                SslMode::Prefer,
                SslMode::Require,
                SslMode::VerifyCa,
            ] {
                let result = config(network(None, None), ssl_mode).to_session_config();

                assert!(
                    matches!(
                        result,
                        Err(SessionConfigError::UnsupportedSslMode { ssl_mode: refused })
                            if refused == ssl_mode
                    ),
                    "{ssl_mode:?}"
                );
            }
        }

        #[test]
        fn refuses_an_application_name_the_server_would_rewrite() {
            let config = Config {
                session: Session {
                    application_name: Some(crate::parameter::Value::from_static_or_panic("café")),
                    ..config(network(None, None), SslMode::Disable).session
                },
                ..config(network(None, None), SslMode::Disable)
            };

            assert!(matches!(
                config.to_session_config(),
                Err(SessionConfigError::ApplicationName(
                    crate::name::CleanAsciiError::NotPrintable {
                        index: 3,
                        byte: 0xc3
                    }
                ))
            ));
        }

        #[test]
        fn refuses_verify_full_without_root_certificate() {
            assert!(matches!(
                config(network(None, None), SslMode::VerifyFull).to_session_config(),
                Err(SessionConfigError::RootCertificateMissing)
            ));
        }

        #[test]
        fn refuses_system_root_certificates() {
            let config = Config {
                ssl_root_cert: Some(SslRootCert::System),
                ..config(network(None, None), SslMode::VerifyFull)
            };

            assert!(matches!(
                config.to_session_config(),
                Err(SessionConfigError::SystemRootCertificates)
            ));
        }

        #[test]
        fn refuses_channel_binding_require_without_tls() {
            assert!(matches!(
                config(
                    network(Some(ChannelBinding::Require), None),
                    SslMode::Disable
                )
                .to_session_config(),
                Err(SessionConfigError::ChannelBindingWithoutTls)
            ));
        }

        #[test]
        fn reports_an_unreadable_root_certificate() {
            let path = std::path::PathBuf::from("/nonexistent/root.crt");
            let config = Config {
                ssl_root_cert: Some(SslRootCert::File(path.clone())),
                ..config(network(None, None), SslMode::VerifyFull)
            };

            assert!(matches!(
                config.to_session_config(),
                Err(SessionConfigError::RootCertificate {
                    path: reported,
                    source: crate::connection::RootCertificateError::Read(_),
                }) if reported == path
            ));
        }
    }
}
