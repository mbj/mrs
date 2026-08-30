//! Connection configuration in libpq's terms, mostly meant for integrating
//! local tools: `psql`, `pg_dump` and anything else that reads `PG*`
//! variables or a libpq connection URL. pg-client's own connections are
//! configured by [`crate::session::Config`].

pub mod env;
pub mod require_auth;
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
    pub require_auth: Option<require_auth::Requirement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Endpoint {
    Network {
        host: Host,
        channel_binding: Option<ChannelBinding>,
        host_addr: Option<HostAddr>,
        port: Port,
    },
    SocketPath {
        path: std::path::PathBuf,
        port: Port,
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
                state.serialize_field("port", port)?;
                state.end()
            }
            Self::SocketPath { path, port } => {
                let mut state = serializer.serialize_struct("Endpoint", 2)?;
                state.serialize_field(
                    "socket_path",
                    &path.to_str().expect("socket path contains invalid utf8"),
                )?;
                state.serialize_field("port", port)?;
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
        let mut state = serializer.serialize_struct("Config", 9)?;

        if let Some(application_name) = &self.session.application_name {
            state.serialize_field("application_name", application_name)?;
        }

        state.serialize_field("database", &self.session.database)?;
        state.serialize_field("endpoint", &self.endpoint)?;

        if let Some(password) = &self.session.password {
            state.serialize_field("password", password.expose())?;
        }

        state.serialize_field("ssl_mode", &self.ssl_mode)?;

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            state.serialize_field("ssl_root_cert", ssl_root_cert)?;
        }

        if let Some(require_auth) = &self.require_auth {
            state.serialize_field("require_auth", &require_auth.pg_env_value())?;
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
    ///         port: Port::new(5432),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("some-database"),
    ///         password: None,
    ///         user: User::from_static_or_panic("some-user"),
    ///     },
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     require_auth: None,
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
    ///             port: Port::new(5432),
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
    ///         port: Port::new(5432),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("mydb"),
    ///         password: None,
    ///         user: User::from_static_or_panic("user"),
    ///     },
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     require_auth: None,
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
    ///         port: Port::new(5432),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: Database::from_static_or_panic("mydb"),
    ///         password: None,
    ///         user: User::from_static_or_panic("user"),
    ///     },
    ///     ssl_mode: SslMode::Disable,
    ///     ssl_root_cert: None,
    ///     require_auth: None,
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
                    encode_into(&mut value, password.expose());
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

                value.push_str(&format!(":{}", u16::from(port)));

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
                append_query_pair(&mut query, "port", &port.pg_env_value());
                append_query_pair(&mut query, "dbname", self.session.database.as_str());
                append_query_pair(
                    &mut query,
                    "user",
                    self.session.user.pg_env_value().as_str(),
                );
                if let Some(password) = &self.session.password {
                    append_query_pair(&mut query, "password", password.expose());
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
        if let Some(require_auth) = &self.require_auth {
            append_query_pair(query, "require_auth", &require_auth.pg_env_value());
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
    ///         port: Port::new(5432),
    ///     },
    ///     session: Session {
    ///         application_name: None,
    ///         database: "some-database".parse().unwrap(),
    ///         password: None,
    ///         user: "some-user".parse().unwrap(),
    ///     },
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: None,
    ///     require_auth: None,
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
    ///         port: Port::new(5432),
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
                map.insert(
                    env::PGPORT.clone(),
                    cmd_proc::EnvVariableValue::try_from(port.pg_env_value())?,
                );
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
                map.insert(
                    env::PGPORT.clone(),
                    cmd_proc::EnvVariableValue::try_from(port.pg_env_value())?,
                );
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

        if let Some(require_auth) = &self.require_auth {
            map.insert(
                env::PGREQUIREAUTH.clone(),
                cmd_proc::EnvVariableValue::try_from(require_auth.pg_env_value())?,
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

    pub fn from_session_config(config: &crate::session::Config) -> Result<Self, FromSessionError> {
        let methods = &config.authentication.methods;
        let channel_binding = methods.scram.as_ref().map(|scram| scram.channel_binding);

        let endpoint = match &config.connection.endpoint {
            crate::connection::Endpoint::Tcp { tls: Some(_), .. } => {
                return Err(FromSessionError::Tls);
            }
            crate::connection::Endpoint::Tcp {
                host,
                address,
                port,
                tls: None,
            } => Endpoint::Network {
                host: host.clone(),
                channel_binding,
                host_addr: address.clone(),
                port: *port,
            },
            crate::connection::Endpoint::Unix { path } => {
                Endpoint::from_socket(path).map_err(FromSessionError::Socket)?
            }
        };

        if channel_binding == Some(ChannelBinding::Require) {
            return Err(FromSessionError::ChannelBindingWithoutTls);
        }

        let mut application_name = None;

        for (name, value) in config.parameters.iter() {
            if name.as_str() == crate::parameter::APPLICATION_NAME {
                application_name = Some(value.clone());
            } else {
                return Err(FromSessionError::Parameter { name: name.clone() });
            }
        }

        Ok(Self {
            endpoint,
            session: Session {
                application_name,
                database: config.database.clone(),
                password: methods
                    .shared_static_password()
                    .map_err(FromSessionError::Password)?
                    .cloned(),
                user: config.authentication.user.clone(),
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: Some(require_auth(methods)?),
        })
    }

    /// Refuses what a session cannot express: a missing password, an
    /// `sslmode` other than `disable` or `verify-full`, `sslrootcert=system`,
    /// `channel_binding=require` without TLS, and a `require_auth` naming
    /// none of `none`, `password` or `scram-sha-256`, or negating methods.
    /// Without `require_auth` the session answers SCRAM only.
    pub fn to_session_config(&self) -> Result<crate::session::Config, SessionConfigError> {
        let (endpoint, channel_binding) = match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                let channel_binding = channel_binding.unwrap_or(ChannelBinding::Prefer);

                (
                    crate::connection::Endpoint::Tcp {
                        host: host.clone(),
                        address: host_addr.clone(),
                        port: *port,
                        tls: self.tls(channel_binding)?,
                    },
                    channel_binding,
                )
            }
            Endpoint::SocketPath { path, port } => (
                crate::connection::Endpoint::Unix {
                    path: socket_file(path, *port),
                },
                ChannelBinding::Prefer,
            ),
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

        Ok(crate::session::Config::new(
            crate::connection::Config::new(endpoint),
            crate::authentication::Config {
                user: self.session.user.clone(),
                methods: self.session_methods(channel_binding)?,
            },
            self.session.database.clone(),
        )
        .with_parameters(parameters))
    }

    // Without require_auth libpq answers any method the server asks for; the
    // session answers SCRAM. An allow list keeps the methods a session speaks
    // and drops the rest, which only narrows what is accepted. A deny list
    // would have to be turned into the complement and is refused for now.
    fn session_methods(
        &self,
        channel_binding: ChannelBinding,
    ) -> Result<crate::authentication::Methods, SessionConfigError> {
        use crate::authentication::{Methods, Password, method};

        let password = || {
            self.session
                .password
                .clone()
                .map(Password::Static)
                .ok_or(SessionConfigError::PasswordMissing)
        };

        let accepted = match &self.require_auth {
            None => {
                return Ok(
                    Methods::default().with_scram(method::Scram::new(password()?, channel_binding))
                );
            }
            Some(require_auth::Requirement::Deny(_)) => {
                return Err(SessionConfigError::DenyListUnsupported);
            }
            Some(require_auth::Requirement::Allow(accepted)) => accepted,
        };

        let mut methods = Methods::default();
        let mut supported = false;

        if accepted.contains(require_auth::Method::None) {
            methods = methods.with_unchallenged();
            supported = true;
        }

        if accepted.contains(require_auth::Method::Password) {
            methods = methods.with_cleartext_password(method::CleartextPassword {
                password: password()?,
            });
            supported = true;
        }

        if accepted.contains(require_auth::Method::ScramSha256) {
            methods = methods.with_scram(method::Scram::new(password()?, channel_binding));
            supported = true;
        }

        if supported {
            Ok(methods)
        } else {
            Err(SessionConfigError::NoSupportedMethod)
        }
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
                Some(SslRootCert::File(path)) => tls_from_file(path).map(Some),
            },
            ssl_mode
            @ (SslMode::Allow | SslMode::Prefer | SslMode::Require | SslMode::VerifyCa) => {
                Err(SessionConfigError::UnsupportedSslMode { ssl_mode })
            }
        }
    }
}

pub(crate) const DEFAULT_PORT: Port = Port::new(5432);

// https://github.com/postgres/postgres/blob/master/src/include/libpq/pqcomm.h (UNIXSOCK_PATH)
const SOCKET_FILE_PREFIX: &str = ".s.PGSQL.";

#[must_use]
pub fn socket_file(directory: &std::path::Path, port: Port) -> std::path::PathBuf {
    directory.join(format!("{SOCKET_FILE_PREFIX}{}", u16::from(port)))
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SocketPathError {
    #[error("socket file {name:?} is not named .s.PGSQL.<port>, which libpq cannot address")]
    FileName { name: std::ffi::OsString },
    #[error("socket directory {directory:?} is not absolute, which libpq reads as a host name")]
    RelativeDirectory { directory: std::path::PathBuf },
}

impl Endpoint {
    pub fn from_socket(path: &std::path::Path) -> Result<Self, SocketPathError> {
        let name = path.file_name().unwrap_or_default();

        let port = name
            .to_str()
            .and_then(|name| name.strip_prefix(SOCKET_FILE_PREFIX))
            .and_then(|digits| {
                digits
                    .parse::<Port>()
                    .ok()
                    .filter(|port| u16::from(*port) != 0 && port.pg_env_value() == digits)
            })
            .ok_or_else(|| SocketPathError::FileName {
                name: name.to_owned(),
            })?;

        let directory = path.parent().unwrap_or_else(|| std::path::Path::new(""));

        if !directory.is_absolute() {
            return Err(SocketPathError::RelativeDirectory {
                directory: directory.to_owned(),
            });
        }

        Ok(Self::SocketPath {
            path: directory.to_owned(),
            port,
        })
    }
}

fn tls_from_file(path: &std::path::Path) -> Result<crate::connection::Tls, SessionConfigError> {
    use crate::connection::rustls::pki_types::{CertificateDer, pem::PemObject as _};

    CertificateDer::pem_file_iter(path)
        .map_err(crate::connection::RootCertificateError::Read)
        .and_then(crate::connection::Tls::from_certificates)
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
    #[error("require_auth names no method a session speaks (none, password, scram-sha-256)")]
    NoSupportedMethod,
    #[error("a negated require_auth cannot be expressed as a session yet")]
    DenyListUnsupported,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum FromSessionError {
    #[error("TLS cannot be expressed in a libpq config yet")]
    Tls,
    #[error("the socket cannot be addressed by libpq")]
    Socket(#[source] SocketPathError),
    #[error("channel binding is required and the connection has no TLS")]
    ChannelBindingWithoutTls,
    #[error("the session's passwords cannot be expressed as one libpq password")]
    Password(#[source] crate::authentication::SharedPasswordError),
    #[error("parameter {name} cannot be expressed in a libpq config yet")]
    Parameter { name: crate::parameter::Name },
    // libpq reads an empty require_auth as no restriction at all, the opposite
    // of a session that accepts nothing.
    #[error("the session accepts no authentication method, which libpq cannot express")]
    NoMethodAccepted,
}

fn require_auth(
    methods: &crate::authentication::Methods,
) -> Result<require_auth::Requirement, FromSessionError> {
    let accepted = [
        methods
            .accept_unchallenged
            .then_some(require_auth::Method::None),
        methods
            .cleartext_password
            .is_some()
            .then_some(require_auth::Method::Password),
        methods
            .scram
            .is_some()
            .then_some(require_auth::Method::ScramSha256),
    ];

    require_auth::Methods::new(accepted.into_iter().flatten())
        .map(require_auth::Requirement::Allow)
        .ok_or(FromSessionError::NoMethodAccepted)
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: TEST_USER,
            },
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: None,
            require_auth: None,
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
                    port: Port::new(5432),
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
                    port: Port::new(5432),
                },
                ..config.clone()
            },
        );

        assert_config(
            serde_json::json!({
                "database": "some-database",
                "endpoint": {
                    "host": "some-host",
                    "host_addr": "10.0.0.1",
                    "port": 5432,
                },
                "ssl_mode": "verify-full",
                "url": "postgres://some-user@some-host:5432/some-database?hostaddr=10.0.0.1&sslmode=verify-full",
                "user": "some-user"
            }),
            &Config {
                endpoint: Endpoint::Network {
                    host: Host::from_str("some-host").unwrap(),
                    channel_binding: None,
                    host_addr: Some("10.0.0.1".parse().unwrap()),
                    port: Port::new(5432),
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: None,
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: None,
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: None,
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: None,
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
                port: Port::new(5432),
            },
            session: Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: User::POSTGRES,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            require_auth: None,
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
                require_auth: None,
            }
        }

        fn network(channel_binding: Option<ChannelBinding>) -> Endpoint {
            Endpoint::Network {
                host: Host::from_str("some-host").unwrap(),
                channel_binding,
                host_addr: Some("10.0.0.1".parse().unwrap()),
                port: Port::new(5432),
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
                    ..config(network(None), SslMode::Disable).session
                },
                ..config(network(None), SslMode::Disable)
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
            assert!(!session.authentication.methods.accept_unchallenged);
            let Some(crate::authentication::method::Scram {
                password: crate::authentication::Password::Static(password),
                channel_binding,
                iteration_bounds,
            }) = session.authentication.methods.scram
            else {
                panic!("expected SCRAM with a static password");
            };

            assert_eq!(
                iteration_bounds,
                crate::authentication::scram::IterationBounds::DEFAULT
            );
            assert_eq!(password, "some-password".parse().unwrap());
            assert_eq!(channel_binding, ChannelBinding::Prefer);

            assert_eq!(session.database, TEST_DATABASE);
            let mut parameters = crate::session::Parameters::new();
            parameters.set_application_name("some-app".parse().unwrap());
            assert_eq!(session.parameters, parameters);
        }

        #[test]
        fn converts_a_session_back_into_the_libpq_config_it_came_from() {
            let scram: Option<require_auth::Requirement> = Some("scram-sha-256".parse().unwrap());

            let network = Config {
                session: Session {
                    application_name: Some("some-app".parse().unwrap()),
                    ..config(network(Some(ChannelBinding::Prefer)), SslMode::Disable).session
                },
                require_auth: scram.clone(),
                ..config(network(Some(ChannelBinding::Prefer)), SslMode::Disable)
            };
            let socket = Config {
                require_auth: Some("none,password,scram-sha-256".parse().unwrap()),
                ..config(
                    Endpoint::SocketPath {
                        path: "/var/run/postgresql".into(),
                        port: Port::new(5433),
                    },
                    SslMode::Disable,
                )
            };

            for original in [network, socket] {
                assert_eq!(
                    Config::from_session_config(&original.to_session_config().unwrap()),
                    Ok(original)
                );
            }
        }

        #[test]
        fn narrows_an_unrestricted_config_to_the_scram_a_session_answers() {
            let original = config(network(None), SslMode::Disable);

            assert_eq!(original.require_auth, None);
            assert_eq!(
                Config::from_session_config(&original.to_session_config().unwrap())
                    .unwrap()
                    .require_auth,
                Some("scram-sha-256".parse().unwrap())
            );
        }

        #[test]
        fn keeps_the_methods_an_allow_list_shares_with_a_session() {
            let methods = |require_auth: &str| {
                Config {
                    require_auth: Some(require_auth.parse().unwrap()),
                    ..config(network(None), SslMode::Disable)
                }
                .to_session_config()
                .map(|session| {
                    let methods = session.authentication.methods;

                    (
                        methods.accept_unchallenged,
                        methods.cleartext_password.is_some(),
                        methods.scram.is_some(),
                    )
                })
            };

            assert_eq!(methods("none").unwrap(), (true, false, false));
            assert_eq!(methods("password,md5").unwrap(), (false, true, false));
            assert_eq!(methods("gss,scram-sha-256").unwrap(), (false, false, true));
            assert!(matches!(
                methods("md5,gss"),
                Err(SessionConfigError::NoSupportedMethod)
            ));
            assert!(matches!(
                methods("!md5"),
                Err(SessionConfigError::DenyListUnsupported)
            ));
        }

        #[test]
        fn needs_no_password_for_an_unchallenged_session() {
            let original = Config {
                session: Session {
                    password: None,
                    ..config(network(None), SslMode::Disable).session
                },
                require_auth: Some("none".parse().unwrap()),
                ..config(network(None), SslMode::Disable)
            };

            assert!(original.to_session_config().is_ok());
            assert!(matches!(
                Config {
                    require_auth: Some("password".parse().unwrap()),
                    ..original
                }
                .to_session_config(),
                Err(SessionConfigError::PasswordMissing)
            ));
        }

        fn session(endpoint: Endpoint) -> crate::session::Config {
            config(endpoint, SslMode::Disable)
                .to_session_config()
                .unwrap()
        }

        #[test]
        fn refuses_a_session_with_tls() {
            let mut session = session(network(None));

            let crate::connection::Endpoint::Tcp { tls, .. } = &mut session.connection.endpoint
            else {
                panic!("expected a TCP endpoint");
            };
            *tls = Some(crate::connection::Tls::new(
                crate::connection::rustls::RootCertStore::empty(),
            ));

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::Tls)
            );
        }

        #[test]
        fn refuses_a_session_that_requires_channel_binding() {
            let mut session = session(network(None));
            session
                .authentication
                .methods
                .scram
                .as_mut()
                .unwrap()
                .channel_binding = ChannelBinding::Require;

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::ChannelBindingWithoutTls)
            );
        }

        #[test]
        fn refuses_a_session_parameter_libpq_cannot_carry() {
            let mut session = session(network(None));
            session
                .parameters
                .insert(
                    crate::parameter::Name::from_static_or_panic("search_path"),
                    crate::parameter::Value::from_static_or_panic("app"),
                )
                .unwrap();

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::Parameter {
                    name: crate::parameter::Name::from_static_or_panic("search_path")
                })
            );
        }

        #[test]
        fn refuses_a_session_that_accepts_no_method() {
            let mut session = session(network(None));
            session.authentication.methods = crate::authentication::Methods::default();

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::NoMethodAccepted)
            );
        }

        #[test]
        fn writes_require_auth_to_every_output() {
            let config = Config {
                require_auth: Some("none,scram-sha-256".parse().unwrap()),
                ..config(network(None), SslMode::Disable)
            };

            assert_eq!(
                config
                    .pg_env()
                    .unwrap()
                    .get(&env::PGREQUIREAUTH)
                    .map(|value| value.as_str().to_owned()),
                Some("scram-sha-256,none".to_owned())
            );
            assert!(
                config
                    .to_url()
                    .contains("require_auth=scram-sha-256%2Cnone"),
                "{}",
                config.to_url()
            );
            assert_eq!(
                serde_json::to_value(&config).unwrap()["require_auth"],
                "scram-sha-256,none"
            );
        }

        #[test]
        fn refuses_a_session_with_conflicting_passwords() {
            let mut session = session(network(None));
            session.authentication.methods.cleartext_password =
                Some(crate::authentication::method::CleartextPassword {
                    password: crate::authentication::Password::Static(
                        "another-password".parse().unwrap(),
                    ),
                });

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::Password(
                    crate::authentication::SharedPasswordError::Conflicting
                ))
            );
        }

        #[test]
        fn refuses_a_session_socket_libpq_cannot_address() {
            let mut session = session(network(None));
            session.connection.endpoint = crate::connection::Endpoint::Unix {
                path: "/tmp/db.sock".into(),
            };

            assert_eq!(
                Config::from_session_config(&session),
                Err(FromSessionError::Socket(SocketPathError::FileName {
                    name: "db.sock".into()
                }))
            );
        }

        #[test]
        fn recovers_the_socket_endpoint_from_its_path() {
            let endpoint = Endpoint::SocketPath {
                path: "/var/run/postgresql".into(),
                port: Port::new(5433),
            };

            assert_eq!(
                Endpoint::from_socket(&unix_path(&config(endpoint.clone(), SslMode::Disable))),
                Ok(endpoint)
            );
        }

        #[test]
        fn refuses_a_socket_file_libpq_would_not_name() {
            for name in [
                "db.sock",
                ".s.PGSQL.05432",
                ".s.PGSQL.+5432",
                ".s.PGSQL.0",
                ".s.PGSQL.70000",
                ".s.PGSQL.",
            ] {
                assert_eq!(
                    Endpoint::from_socket(&std::path::Path::new("/run").join(name)),
                    Err(SocketPathError::FileName { name: name.into() }),
                    "{name}"
                );
            }
        }

        #[test]
        fn refuses_a_socket_in_a_relative_directory() {
            for (path, directory) in [("run/.s.PGSQL.5432", "run"), (".s.PGSQL.5432", "")] {
                assert_eq!(
                    Endpoint::from_socket(std::path::Path::new(path)),
                    Err(SocketPathError::RelativeDirectory {
                        directory: directory.into()
                    }),
                    "{path}"
                );
            }
        }

        #[test]
        fn connects_to_the_socket_of_the_port() {
            let endpoint = Endpoint::SocketPath {
                path: "/var/run/postgresql".into(),
                port: Port::new(5433),
            };

            assert_eq!(
                unix_path(&config(endpoint, SslMode::Disable)),
                std::path::PathBuf::from("/var/run/postgresql/.s.PGSQL.5433")
            );
        }

        #[test]
        fn refuses_an_absent_password() {
            let config = Config {
                session: Session {
                    password: None,
                    ..config(network(None), SslMode::Disable).session
                },
                ..config(network(None), SslMode::Disable)
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
                let result = config(network(None), ssl_mode).to_session_config();

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
                    ..config(network(None), SslMode::Disable).session
                },
                ..config(network(None), SslMode::Disable)
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
                config(network(None), SslMode::VerifyFull).to_session_config(),
                Err(SessionConfigError::RootCertificateMissing)
            ));
        }

        #[test]
        fn refuses_system_root_certificates() {
            let config = Config {
                ssl_root_cert: Some(SslRootCert::System),
                ..config(network(None), SslMode::VerifyFull)
            };

            assert!(matches!(
                config.to_session_config(),
                Err(SessionConfigError::SystemRootCertificates)
            ));
        }

        #[test]
        fn refuses_channel_binding_require_without_tls() {
            assert!(matches!(
                config(network(Some(ChannelBinding::Require)), SslMode::Disable)
                    .to_session_config(),
                Err(SessionConfigError::ChannelBindingWithoutTls)
            ));
        }

        #[test]
        fn reports_an_unreadable_root_certificate() {
            let path = std::path::PathBuf::from("/nonexistent/root.crt");
            let config = Config {
                ssl_root_cert: Some(SslRootCert::File(path.clone())),
                ..config(network(None), SslMode::VerifyFull)
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
