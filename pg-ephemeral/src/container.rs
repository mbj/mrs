use rand::RngExt;

use crate::LOCALHOST_HOST_ADDR;
use crate::LOCALHOST_IP;
use crate::UNSPECIFIED_IP;
use crate::certificate;
use crate::definition;

pub const PGDATA: &str = "/var/lib/pg-ephemeral";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PostgreSQL did not become available within {timeout:?}")]
    ConnectionTimeout {
        timeout: std::time::Duration,
        #[source]
        source: Option<sqlx::Error>,
    },
    #[error("Failed to execute command in container")]
    ContainerExec(#[from] cmd_proc::CommandError),
    #[error(transparent)]
    SeedLoad(#[from] crate::seed::LoadError),
}
const ENV_POSTGRES_PASSWORD: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_PASSWORD");
const ENV_POSTGRES_USER: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_USER");
const ENV_PGDATA: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGDATA");
const ENV_PG_EPHEMERAL_SSL_DIR: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SSL_DIR");
const ENV_PG_EPHEMERAL_CA_CERT_PEM: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_CA_CERT_PEM");
const ENV_PG_EPHEMERAL_SERVER_CERT_PEM: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SERVER_CERT_PEM");
const ENV_PG_EPHEMERAL_SERVER_KEY_PEM: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SERVER_KEY_PEM");

const SSL_SETUP_SCRIPT: &str = r#"
printf '%s' "$PG_EPHEMERAL_CA_CERT_PEM" > ${PG_EPHEMERAL_SSL_DIR}/root.crt
printf '%s' "$PG_EPHEMERAL_SERVER_CERT_PEM" > ${PG_EPHEMERAL_SSL_DIR}/server.crt
printf '%s' "$PG_EPHEMERAL_SERVER_KEY_PEM" > ${PG_EPHEMERAL_SSL_DIR}/server.key
chown postgres ${PG_EPHEMERAL_SSL_DIR}/root.crt
chown postgres ${PG_EPHEMERAL_SSL_DIR}/server.crt
chown postgres ${PG_EPHEMERAL_SSL_DIR}/server.key
chmod 600 ${PG_EPHEMERAL_SSL_DIR}/root.crt
chmod 600 ${PG_EPHEMERAL_SSL_DIR}/server.crt
chmod 600 ${PG_EPHEMERAL_SSL_DIR}/server.key
exec docker-entrypoint.sh "$@"
"#;

/// Low-level container definition for running a pre-initialized PostgreSQL image.
///
/// All fields are assumed to represent values already stored in the referenced image.
/// No validation is performed - the caller is responsible for ensuring the credentials
/// and configuration match what exists in the image.
#[derive(Debug)]
pub struct Definition {
    pub image: ociman::image::Reference,
    pub password: pg_client::Password,
    pub user: pg_client::User,
    pub database: pg_client::Database,
    pub backend: ociman::Backend,
    pub cross_container_access: bool,
    pub application_name: Option<pg_client::ApplicationName>,
    pub ssl_config: Option<definition::SslConfig>,
    pub wait_available_timeout: std::time::Duration,
}

#[derive(Debug)]
pub struct Container {
    host_port: pg_client::Port,
    pub(crate) client_config: pg_client::Config,
    container: ociman::Container,
    backend: ociman::Backend,
    wait_available_timeout: std::time::Duration,
}

impl Container {
    pub(crate) async fn run_definition(definition: &crate::definition::Definition) -> Self {
        let password = generate_password();

        let ociman_definition = definition
            .to_ociman_definition()
            .environment_variable(ENV_POSTGRES_PASSWORD, password.as_ref())
            .environment_variable(ENV_POSTGRES_USER, definition.superuser.as_ref());

        run_container(
            ociman_definition,
            definition.cross_container_access,
            &definition.ssl_config,
            &definition.backend,
            &definition.application_name,
            &definition.database,
            &password,
            &definition.superuser,
            definition.wait_available_timeout,
            definition.remove,
        )
        .await
    }

    pub async fn run_container_definition(definition: &Definition) -> Self {
        let ociman_definition =
            ociman::Definition::new(definition.backend.clone(), definition.image.clone());

        run_container(
            ociman_definition,
            definition.cross_container_access,
            &definition.ssl_config,
            &definition.backend,
            &definition.application_name,
            &definition.database,
            &definition.password,
            &definition.user,
            definition.wait_available_timeout,
            true, // Always remove containers when using low-level API
        )
        .await
    }

    pub async fn wait_available(&self) -> Result<(), Error> {
        let config = self.client_config.to_sqlx_connect_options().unwrap();

        let start = std::time::Instant::now();
        let max_duration = self.wait_available_timeout;
        let sleep_duration = std::time::Duration::from_millis(100);

        let mut last_error: Option<sqlx::Error> = None;

        while start.elapsed() <= max_duration {
            log::trace!("connection attempt");
            match sqlx::ConnectOptions::connect(&config).await {
                Ok(connection) => {
                    sqlx::Connection::close(connection)
                        .await
                        .expect("connection close failed");

                    log::debug!(
                        "pg is available on endpoint: {:#?}",
                        self.client_config.endpoint
                    );

                    return Ok(());
                }
                Err(error) => {
                    log::trace!("{error:#?}, retry in 100ms");
                    last_error = Some(error);
                }
            }
            tokio::time::sleep(sleep_duration).await;
        }

        Err(Error::ConnectionTimeout {
            timeout: max_duration,
            source: last_error,
        })
    }

    pub(crate) async fn exec_schema_dump(&self) -> String {
        let output = self
            .container
            .exec("pg_dump")
            .argument("--schema-only")
            .environment_variables(self.container_client_config().to_pg_env())
            .build()
            .stdout_capture()
            .bytes()
            .await
            .unwrap();
        crate::convert_schema(&output)
    }

    #[must_use]
    pub fn client_config(&self) -> &pg_client::Config {
        &self.client_config
    }

    pub async fn with_connection<T, F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> T>(
        &self,
        mut action: F,
    ) -> T {
        self.client_config
            .with_sqlx_connection(async |connection| action(connection).await)
            .await
            .unwrap()
    }

    pub async fn apply_sql(&self, sql: &str) {
        self.with_connection(async |connection| {
            log::debug!("Executing: {sql}");
            sqlx::raw_sql(sqlx::AssertSqlSafe(sql))
                .execute(connection)
                .await
                .unwrap();
        })
        .await
    }

    pub(crate) async fn exec_container_shell(&self) {
        self.container
            .exec("sh")
            .environment_variables(self.container_client_config().to_pg_env())
            .interactive()
            .status()
            .await
            .unwrap();
    }

    pub(crate) async fn exec_psql(&self) {
        self.container
            .exec("psql")
            .environment_variables(self.container_client_config().to_pg_env())
            .interactive()
            .status()
            .await
            .unwrap();
    }

    fn container_client_config(&self) -> pg_client::Config {
        let mut config = self.client_config.clone();
        if let pg_client::Endpoint::Network {
            ref host,
            ref channel_binding,
            ref host_addr,
            ..
        } = config.endpoint
        {
            config.endpoint = pg_client::Endpoint::Network {
                host: host.clone(),
                channel_binding: *channel_binding,
                host_addr: host_addr.clone(),
                port: Some(pg_client::Port::new(5432)),
            };
        }
        config
    }

    pub async fn cross_container_client_config(&self) -> pg_client::Config {
        // Resolve the container host from inside a container
        // This DNS name only works from inside containers, not from the host
        let ip_address = self
            .backend
            .resolve_container_host()
            .await
            .expect("Failed to resolve container host from container");

        let channel_binding = match &self.client_config.endpoint {
            pg_client::Endpoint::Network {
                channel_binding, ..
            } => *channel_binding,
            pg_client::Endpoint::SocketPath(_) => None,
        };

        let endpoint = pg_client::Endpoint::Network {
            host: pg_client::Host::IpAddr(ip_address),
            channel_binding,
            host_addr: None,
            port: Some(self.host_port),
        };

        self.client_config.clone().endpoint(endpoint)
    }

    #[must_use]
    pub fn pg_env(&self) -> std::collections::BTreeMap<cmd_proc::EnvVariableName<'static>, String> {
        self.client_config.to_pg_env()
    }

    #[must_use]
    pub fn database_url(&self) -> String {
        self.client_config.to_url_string()
    }

    pub async fn stop(&mut self) {
        self.container.stop().await
    }

    /// Stop the container (clean PostgreSQL shutdown), commit it to an image,
    /// and remove the stopped container.
    pub(crate) async fn stop_commit_remove(&mut self, reference: &ociman::Reference) {
        self.container.stop().await;
        self.container.commit(reference, false).await.unwrap();
        self.container.remove().await;
    }

    async fn wait_for_unix_socket(&self) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let max_duration = self.wait_available_timeout;
        let sleep_duration = std::time::Duration::from_millis(100);

        while start.elapsed() <= max_duration {
            if self
                .container
                .exec("pg_isready")
                .argument("--host")
                .argument("/var/run/postgresql")
                .build()
                .stdout_capture()
                .bytes()
                .await
                .is_ok()
            {
                return Ok(());
            }
            tokio::time::sleep(sleep_duration).await;
        }

        Err(Error::ConnectionTimeout {
            timeout: max_duration,
            source: None,
        })
    }

    /// Set the superuser password using peer authentication via Unix domain socket.
    ///
    /// This is useful when resuming from a cached image where the password
    /// doesn't match the newly generated one.
    pub async fn set_superuser_password(
        &self,
        password: &pg_client::Password,
    ) -> Result<(), Error> {
        self.wait_for_unix_socket().await?;

        self.container
            .exec("psql")
            .argument("--host")
            .argument("/var/run/postgresql")
            .argument("--username")
            .argument(self.client_config.user.as_ref())
            .argument("--dbname")
            .argument("postgres")
            .argument("--variable")
            .argument(format!("target_user={}", self.client_config.user.as_ref()))
            .argument("--variable")
            .argument(format!("new_password={}", password.as_ref()))
            .stdin("ALTER USER :target_user WITH PASSWORD :'new_password'")
            .build()
            .stdout_capture()
            .bytes()
            .await?;

        Ok(())
    }
}

fn generate_password() -> pg_client::Password {
    let value: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    <pg_client::Password as std::str::FromStr>::from_str(&value).unwrap()
}

#[allow(clippy::too_many_arguments)]
async fn run_container(
    ociman_definition: ociman::Definition,
    cross_container_access: bool,
    ssl_config: &Option<definition::SslConfig>,
    backend: &ociman::Backend,
    application_name: &Option<pg_client::ApplicationName>,
    database: &pg_client::Database,
    password: &pg_client::Password,
    user: &pg_client::User,
    wait_available_timeout: std::time::Duration,
    remove: bool,
) -> Container {
    let backend = backend.clone();
    let host_ip = if cross_container_access {
        UNSPECIFIED_IP
    } else {
        LOCALHOST_IP
    };

    let mut ociman_definition = ociman_definition
        .environment_variable(ENV_PGDATA, "/var/lib/pg-ephemeral")
        .publish(ociman::Publish::tcp(5432).host_ip(host_ip));

    if remove {
        ociman_definition = ociman_definition.remove();
    }

    let ssl_bundle = if let Some(ssl_config) = ssl_config {
        let hostname = match ssl_config {
            definition::SslConfig::Generated { hostname } => hostname.as_str(),
        };

        let bundle = certificate::Bundle::generate(hostname)
            .expect("Failed to generate SSL certificate bundle");

        let ssl_dir = "/var/lib/postgresql";

        ociman_definition = ociman_definition
            .entrypoint("sh")
            .argument("-e")
            .argument("-c")
            .argument(SSL_SETUP_SCRIPT)
            .argument("--")
            .argument("postgres")
            .argument("--ssl=on")
            .argument(format!("--ssl_cert_file={ssl_dir}/server.crt"))
            .argument(format!("--ssl_key_file={ssl_dir}/server.key"))
            .argument(format!("--ssl_ca_file={ssl_dir}/root.crt"))
            .environment_variable(ENV_PG_EPHEMERAL_SSL_DIR, ssl_dir)
            .environment_variable(ENV_PG_EPHEMERAL_CA_CERT_PEM, &bundle.ca_cert_pem)
            .environment_variable(ENV_PG_EPHEMERAL_SERVER_CERT_PEM, &bundle.server_cert_pem)
            .environment_variable(ENV_PG_EPHEMERAL_SERVER_KEY_PEM, &bundle.server_key_pem);

        Some(bundle)
    } else {
        None
    };

    let container = ociman_definition.run_detached().await;

    let port: pg_client::Port = container
        .read_host_tcp_port(5432)
        .await
        .expect("port 5432 not published")
        .into();

    let (host, host_addr, ssl_mode, ssl_root_cert) = if let Some(ssl_config) = ssl_config {
        let hostname = match ssl_config {
            definition::SslConfig::Generated { hostname } => hostname.clone(),
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let ca_cert_path = std::env::temp_dir().join(format!("pg_ephemeral_ca_{timestamp}.crt"));
        std::fs::write(&ca_cert_path, &ssl_bundle.as_ref().unwrap().ca_cert_pem)
            .expect("Failed to write CA certificate to temp file");

        (
            pg_client::Host::HostName(hostname),
            Some(LOCALHOST_HOST_ADDR),
            pg_client::SslMode::VerifyFull,
            Some(pg_client::SslRootCert::File(ca_cert_path)),
        )
    } else {
        (
            pg_client::Host::IpAddr(LOCALHOST_IP),
            None,
            pg_client::SslMode::Disable,
            None,
        )
    };

    let client_config = pg_client::Config {
        application_name: application_name.clone(),
        database: database.clone(),
        endpoint: pg_client::Endpoint::Network {
            host,
            channel_binding: None,
            host_addr,
            port: Some(port),
        },
        password: Some(password.clone()),
        ssl_mode,
        ssl_root_cert,
        user: user.clone(),
    };

    Container {
        host_port: port,
        container,
        backend,
        client_config,
        wait_available_timeout,
    }
}
