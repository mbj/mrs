pub mod analyze;

use crate::{
    Config, Endpoint, PGAPPNAME, PGCHANNELBINDING, PGHOSTADDR, PGPASSWORD, PGPORT, PGSSLROOTCERT,
    SslMode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionsError {
    EnvConflict { env_key: String, field_name: String },
    UnsupportedFeature { env_key: String, field_name: String },
    SslRootCertSystemNotSupported,
}

impl std::fmt::Display for OptionsError {
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

fn reject_env(
    env_key: &cmd_proc::EnvVariableName<'static>,
    field_name: &str,
) -> Result<(), OptionsError> {
    if std::env::var(env_key.as_str()).is_ok() {
        Err(OptionsError::EnvConflict {
            env_key: env_key.as_str().to_string(),
            field_name: field_name.to_string(),
        })
    } else {
        Ok(())
    }
}

fn unsupported_env(env_key: &str, field_name: &str) -> Result<(), OptionsError> {
    if std::env::var(env_key).is_ok() {
        Err(OptionsError::UnsupportedFeature {
            env_key: env_key.to_string(),
            field_name: field_name.to_string(),
        })
    } else {
        Ok(())
    }
}

impl Config {
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
    ///         channel_binding: None,
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
    /// Returns an error if fields inferred from the process environment variables
    /// by `PgConnectOptions::new` contradict the settings in `Config`, and
    /// there is no public API in `PgConnectOptions` to reset these values.
    pub fn to_sqlx_connect_options(
        &self,
    ) -> Result<sqlx::postgres::PgConnectOptions, OptionsError> {
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
                channel_binding,
                host_addr,
                port,
            } => {
                options = options.host(&host.pg_env_value());
                if let Some(port) = port {
                    options = options.port(port.into());
                } else {
                    reject_env(&PGPORT, "port")?;
                }
                if channel_binding.is_some() {
                    return Err(OptionsError::UnsupportedFeature {
                        env_key: PGCHANNELBINDING.as_str().to_string(),
                        field_name: "channel_binding".to_string(),
                    });
                } else {
                    reject_env(&PGCHANNELBINDING, "channel_binding")?;
                }
                if let Some(host_addr) = host_addr {
                    options = options.host_addr(&host_addr.to_string())
                } else {
                    reject_env(&PGHOSTADDR, "hostaddr")?;
                }
            }
            Endpoint::SocketPath(path) => {
                options = options.host(path.to_str().expect("socket path contains invalid utf8"));
                reject_env(&PGPORT, "port")?;
                reject_env(&PGCHANNELBINDING, "channel_binding")?;
                reject_env(&PGHOSTADDR, "hostaddr")?;
            }
        }

        options = options.ssl_mode((&self.ssl_mode).into());
        options = options.username(self.username.as_str());

        if let Some(application_name) = &self.application_name {
            options = options.application_name(application_name.as_str());
        } else {
            reject_env(&PGAPPNAME, "application_name")?;
        }

        if let Some(password) = &self.password {
            options = options.password(password.as_str());
        } else {
            reject_env(&PGPASSWORD, "password")?;
        }

        if let Some(ssl_root_cert) = &self.ssl_root_cert {
            match ssl_root_cert {
                crate::SslRootCert::File(path) => {
                    options = options.ssl_root_cert(path.to_str().unwrap());
                }
                crate::SslRootCert::System => {
                    return Err(OptionsError::SslRootCertSystemNotSupported);
                }
            }
        } else {
            reject_env(&PGSSLROOTCERT, "ssl_root_cert")?;
        }

        Ok(options)
    }

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
    use crate::{Database, Endpoint, Host, Port, SslMode, SslRootCert, Username};
    use std::str::FromStr;

    #[test]
    fn test_ssl_root_cert_system_not_supported() {
        let config = Config {
            application_name: None,
            database: Database::from_str("test").unwrap(),
            endpoint: Endpoint::Network {
                host: Host::from_str("localhost").unwrap(),
                channel_binding: None,
                host_addr: None,
                port: Some(Port::new(5432)),
            },
            password: None,
            ssl_mode: SslMode::VerifyFull,
            ssl_root_cert: Some(SslRootCert::System),
            username: Username::from_str("test").unwrap(),
        };

        let result = config.to_sqlx_connect_options();

        assert!(matches!(
            result,
            Err(OptionsError::SslRootCertSystemNotSupported)
        ));
    }
}
