pub mod analyze;
pub mod partitioned_index;
pub mod sqlstate;
pub mod transaction;

use crate::Config;
use crate::config::{Endpoint, SslMode};

/// Sqlx-specific connection settings that don't map to standard PostgreSQL
/// environment variables or connection URL parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    /// Override the prepared statement cache capacity (sqlx default: 100).
    pub statement_cache_capacity: Option<usize>,
    /// Log level for executed statements.
    pub log_statements: Option<log::LevelFilter>,
    /// Log level and duration threshold for slow statements.
    pub log_slow_statements: Option<(log::LevelFilter, std::time::Duration)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionsError {
    UnsupportedFeature { field_name: String },
    SslRootCertSystemNotSupported,
}

impl std::fmt::Display for OptionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedFeature { field_name } => write!(
                f,
                "`pg_client::Config` specifies `{field_name}`, but sqlx's `PgConnectOptions` does not support that feature"
            ),
            Self::SslRootCertSystemNotSupported => write!(
                f,
                "`SslRootCert::System` is not supported by sqlx, which expects a file path for `ssl_root_cert`"
            ),
        }
    }
}

impl std::error::Error for OptionsError {}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to create SQLx connect options")]
    Options(#[from] OptionsError),

    #[error("Failed to connect to database")]
    Connect(#[source] sqlx::Error),

    /// Closing the connection failed after the action ran.
    ///
    /// The action's result is intentionally dropped: a close failure means the
    /// connection is broken, which fails the operation. A committed transaction's
    /// result is not guaranteed to reach the caller over the network in any case,
    /// so callers must already be able to reconcile in-doubt outcomes.
    #[error("Failed to close database connection")]
    Close(#[source] sqlx::Error),
}

impl From<&SslMode> for sqlx::postgres::PgSslMode {
    fn from(value: &SslMode) -> Self {
        match value {
            SslMode::Allow => Self::Allow,
            SslMode::Disable => Self::Disable,
            SslMode::Prefer => Self::Prefer,
            SslMode::Require => Self::Require,
            SslMode::VerifyCa => Self::VerifyCa,
            SslMode::VerifyFull => Self::VerifyFull,
        }
    }
}

impl Config {
    /// Convert to an sqlx pg connection config
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
    ///         application_name: Some(ApplicationName::from_str("some-app").unwrap()),
    ///         database: Database::from_static_or_panic("some-database"),
    ///         password: Some(Password::from_str("some-password").unwrap()),
    ///         user: User::from_static_or_panic("some-user"),
    ///     },
    ///     ssl_mode: SslMode::VerifyFull,
    ///     ssl_root_cert: Some(SslRootCert::File("/some.pem".into())),
    ///     sqlx: Default::default(), // requires "sqlx" feature
    /// };
    ///
    /// let options = config.to_sqlx_connect_options().unwrap();
    ///
    /// // `PgConnectOptions` does not have `PartialEq` and only partial getters
    /// // so we can only assert a few fields.
    /// assert_eq!(Some("some-app"), options.get_application_name());
    /// assert_eq!("some-host", options.get_host());
    /// assert_eq!(5432, options.get_port());
    /// assert_eq!("some-user", options.get_username());
    /// // No PartialEQ instance, compare debug output
    /// assert_eq!("VerifyFull", format!("{:?}", options.get_ssl_mode()));
    /// assert_eq!(Some("some-database"), options.get_database());
    /// // Unsupported.
    /// // assert_eq!("some-password", options.get_password());
    /// // assert_eq!("/some.pem", options.get_ssl_root_cert());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if `Config` specifies a feature that sqlx's
    /// `PgConnectOptions` does not support (e.g. channel binding, or a system
    /// SSL root certificate store).
    pub fn to_sqlx_connect_options(
        &self,
    ) -> Result<sqlx::postgres::PgConnectOptions, OptionsError> {
        // `default_without_env` produces explicit defaults without any process
        // environment variable snooping, so every field is set solely from `Config`.
        let mut options = sqlx::postgres::PgConnectOptions::default_without_env();

        options = options.database(self.session.database.as_str());

        match &self.endpoint {
            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            } => {
                options = options.host(&host.pg_env_value());
                if let Some(port) = port {
                    options = options.port(port.into());
                }
                if channel_binding.is_some() {
                    return Err(OptionsError::UnsupportedFeature {
                        field_name: "channel_binding".to_string(),
                    });
                }
                if let Some(host_addr) = host_addr {
                    options = options.host_addr(&host_addr.to_string())
                }
            }
            Endpoint::SocketPath(path) => {
                options = options.host(path.to_str().expect("socket path contains invalid utf8"));
            }
        }

        options = options.ssl_mode((&self.ssl_mode).into());
        options = options.username(self.session.user.as_str());

        if let Some(application_name) = &self.session.application_name {
            options = options.application_name(application_name.as_str());
        }

        if let Some(password) = &self.session.password {
            options = options.password(password.as_str());
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            match ssl_root_cert {
                crate::config::SslRootCert::File(path) => {
                    options = options.ssl_root_cert(path.to_str().unwrap());
                }
                crate::config::SslRootCert::System => {
                    return Err(OptionsError::SslRootCertSystemNotSupported);
                }
            }
        }

        if let Some(capacity) = self.sqlx.statement_cache_capacity {
            options = options.statement_cache_capacity(capacity);
        }

        if let Some(level) = self.sqlx.log_statements {
            options = sqlx::ConnectOptions::log_statements(options, level);
        }

        if let Some((level, duration)) = self.sqlx.log_slow_statements {
            options = sqlx::ConnectOptions::log_slow_statements(options, level, duration);
        }

        Ok(options)
    }

    /// Open a connection, run `action` against it, then close it.
    ///
    /// Returns the action's value on success.
    ///
    /// # Errors
    ///
    /// - [`ConnectionError::Options`] / [`ConnectionError::Connect`] if the
    ///   connection cannot be established; `action` is not run.
    /// - [`ConnectionError::Close`] if closing the connection afterwards fails.
    ///   **In this case the action's value is dropped** — a close failure means the
    ///   connection is broken, which fails the operation. Note this is unavoidable in
    ///   general: a result (e.g. a committed transaction's outcome) is never
    ///   guaranteed to reach the caller over the network, so callers must be able to
    ///   reconcile in-doubt outcomes regardless of this method.
    ///
    /// This is a convenience wrapper with one opinionated close policy. Callers that
    /// need different behaviour (e.g. preserving the action's value across a close
    /// failure, retries, or pooling) are free to open and manage a connection
    /// themselves via [`Config::to_sqlx_connect_options`].
    pub async fn with_sqlx_connection<T, F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> T>(
        &self,
        mut action: F,
    ) -> Result<T, ConnectionError> {
        let config = self.to_sqlx_connect_options()?;

        let mut connection = sqlx::ConnectOptions::connect(&config)
            .await
            .map_err(ConnectionError::Connect)?;

        let result = action(&mut connection).await;

        sqlx::Connection::close(connection)
            .await
            .map_err(ConnectionError::Close)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Endpoint, Host, Port, SslMode, SslRootCert};
    use crate::{Database, User};
    use std::str::FromStr;

    const TEST_DATABASE: Database = Database::from_static_or_panic("some-database");
    const TEST_USER: User = User::from_static_or_panic("some-user");

    fn test_config(sqlx: Settings) -> Config {
        Config {
            endpoint: Endpoint::Network {
                host: Host::from_str("localhost").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            session: crate::config::Session {
                application_name: None,
                database: TEST_DATABASE,
                password: None,
                user: TEST_USER,
            },
            ssl_mode: SslMode::Disable,
            ssl_root_cert: None,
            sqlx,
        }
    }

    #[test]
    fn test_statement_cache_capacity_default() {
        let options = test_config(Default::default())
            .to_sqlx_connect_options()
            .unwrap();

        let debug = format!("{options:?}");

        assert!(
            debug.contains("statement_cache_capacity: 100"),
            "Expected default statement_cache_capacity of 100, got: {debug}"
        );
    }

    #[test]
    fn test_statement_cache_capacity_override() {
        let options = test_config(Settings {
            statement_cache_capacity: Some(42),
            ..Default::default()
        })
        .to_sqlx_connect_options()
        .unwrap();

        let debug = format!("{options:?}");

        assert!(
            debug.contains("statement_cache_capacity: 42"),
            "Expected statement_cache_capacity of 42, got: {debug}"
        );
    }

    #[test]
    fn test_log_statements_override() {
        let options = test_config(Settings {
            log_statements: Some(log::LevelFilter::Off),
            ..Default::default()
        })
        .to_sqlx_connect_options()
        .unwrap();

        let debug = format!("{options:?}");

        assert!(
            debug.contains("statements_level: Off"),
            "Expected statements_level: Off, got: {debug}"
        );
    }

    #[test]
    fn test_log_slow_statements_override() {
        let options = test_config(Settings {
            log_slow_statements: Some((log::LevelFilter::Warn, std::time::Duration::from_secs(5))),
            ..Default::default()
        })
        .to_sqlx_connect_options()
        .unwrap();

        let debug = format!("{options:?}");

        assert!(
            debug.contains("slow_statements_level: Warn"),
            "Expected slow_statements_level: Warn, got: {debug}"
        );
        assert!(
            debug.contains("slow_statements_duration: 5s"),
            "Expected slow_statements_duration: 5s, got: {debug}"
        );
    }

    #[test]
    fn test_ssl_root_cert_system_not_supported() {
        let config = Config {
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: Some(SslRootCert::System),
            ..test_config(Default::default())
        };

        let result = config.to_sqlx_connect_options();

        assert!(matches!(
            result,
            Err(OptionsError::SslRootCertSystemNotSupported)
        ));
    }
}
