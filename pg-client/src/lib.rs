#![doc = include_str!("../README.md")]

pub mod config;
pub mod identifier;
pub mod pg_dump;

pub use identifier::{Database, QualifiedTable, Role, User};
pub use pg_dump::{PgSchemaDump, RestrictKey};

#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod url;

use config::{Endpoint, SslRootCert};

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
    pub endpoint: Endpoint,
    pub session: config::Session,
    pub ssl_mode: config::SslMode,
    pub ssl_root_cert: Option<SslRootCert>,
    #[cfg(feature = "sqlx")]
    pub sqlx: crate::sqlx::Settings,
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
        state.serialize_field("url", &self.to_url_string())?;

        state.end()
    }
}

impl Config {
    /// Convert to PG connection URL
    ///
    /// ```
    /// # use pg_client::config::*;
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
    ///     sqlx: Default::default(), // requires "sqlx" feature
    /// };
    ///
    /// assert_eq!(
    ///     config.to_url_string(),
    ///     "postgres://some-user@some-host:5432/some-database?sslmode=verify-full"
    /// );
    ///
    /// assert_eq!(
    ///     Config {
    ///         session: Session {
    ///             application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///             password: Some(Password::from_str("some-password").unwrap()),
    ///             ..config.session.clone()
    ///         },
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
    ///     sqlx: Default::default(), // requires "sqlx" feature
    /// };
    /// assert_eq!(
    ///     ipv4_config.to_url_string(),
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
    ///     sqlx: Default::default(), // requires "sqlx" feature
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

        use config::Host;

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
                userinfo.encode_str::<encoder::Data>(self.session.user.pg_env_value().as_str());
                if let Some(password) = &self.session.password {
                    userinfo.push(':');
                    userinfo.encode_str::<encoder::Data>(password.as_str());
                }

                let mut path = EString::<encoder::Path>::new();
                path.push('/');
                path.encode_str::<encoder::Data>(self.session.database.as_str());

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
                            Some(port) => builder.port(u16::from(port)),
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
                append_query_pair(&mut query, "dbname", self.session.database.as_str());
                append_query_pair(
                    &mut query,
                    "user",
                    self.session.user.pg_env_value().as_str(),
                );
                if let Some(password) = &self.session.password {
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
    ///     sqlx: Default::default(), // requires "sqlx" feature
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
        map.insert(PGUSER.clone(), self.session.user.pg_env_value());
        map.insert(PGDATABASE.clone(), self.session.database.pg_env_value());

        if let Some(application_name) = &self.session.application_name {
            map.insert(PGAPPNAME.clone(), application_name.pg_env_value());
        }

        if let Some(password) = &self.session.password {
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
    use config::*;
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
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
                    application_name: Some(ApplicationName::from_str("some-app").unwrap()),
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
        };

        assert_eq!(
            config_ipv6_loopback.to_url_string(),
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
        };

        assert_eq!(
            config_ipv6_fe80.to_url_string(),
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
        };

        assert_eq!(
            config_ipv6_full.to_url_string(),
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
        };

        assert_eq!(
            config_ipv4.to_url_string(),
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
            #[cfg(feature = "sqlx")]
            sqlx: Default::default(),
        };

        assert_eq!(
            config_hostname.to_url_string(),
            "postgres://postgres@localhost:5432/some-database?sslmode=disable",
            "Hostname should NOT be bracketed in URL"
        );
    }
}
