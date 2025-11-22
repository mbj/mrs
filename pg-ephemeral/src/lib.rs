use rand::Rng;

pub mod certificate;
pub mod cli;
pub mod config;
pub mod definition;
pub mod image;
pub mod seed;

pub use config::Config;
pub use definition::{BackendSelection, Definition};
pub use image::Image;
pub use seed::Command;
pub use seed::DuplicateSeedName;
pub use seed::Seed;
pub use seed::SeedName;
pub use seed::SeedNameError;

/// The version of pg-ephemeral
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

struct SchemaDump<'a> {
    container: &'a Container<'a>,
}

impl mmigration::SchemaDump for SchemaDump<'_> {
    async fn schema_dump(&self) -> mmigration::Schema {
        self.container.exec_schema_dump().into()
    }
}

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

#[derive(Debug)]
pub struct Container<'a> {
    host_port: pg_client::Port,
    client_config: pg_client::Config,
    container: cbt::Container,
    definition: &'a Definition,
}

impl<'a> Container<'a> {
    fn run(definition: &'a Definition) -> Self {
        let password = generate_password();

        let host_ip: std::net::IpAddr = if definition.cross_container_access {
            std::net::Ipv4Addr::UNSPECIFIED.into()
        } else {
            std::net::Ipv4Addr::LOCALHOST.into()
        };

        let mut cbt_definition = definition
            .to_cbt_definition()
            .remove()
            .environment_variable("POSTGRES_PASSWORD", password.as_ref())
            .environment_variable("POSTGRES_USER", definition.superuser.as_ref())
            .publish(cbt::Publish::tcp(5432).host_ip(host_ip));

        let ssl_bundle = if let Some(ssl_config) = &definition.ssl_config {
            let hostname = match ssl_config {
                definition::SslConfig::Generated { hostname } => hostname.as_str(),
            };

            let bundle = certificate::Bundle::generate(hostname)
                .expect("Failed to generate SSL certificate bundle");

            let ssl_dir = "/var/lib/postgresql";

            cbt_definition = cbt_definition
                .entrypoint("sh")
                .argument("-e")
                .argument("-c")
                .argument(SSL_SETUP_SCRIPT)
                .argument("--")
                .argument("postgres")
                .argument("--ssl=on")
                .argument(format!("--ssl_cert_file={}/server.crt", ssl_dir))
                .argument(format!("--ssl_key_file={}/server.key", ssl_dir))
                .argument(format!("--ssl_ca_file={}/root.crt", ssl_dir))
                .environment_variable("PG_EPHEMERAL_SSL_DIR", ssl_dir)
                .environment_variable("PG_EPHEMERAL_CA_CERT_PEM", &bundle.ca_cert_pem)
                .environment_variable("PG_EPHEMERAL_SERVER_CERT_PEM", &bundle.server_cert_pem)
                .environment_variable("PG_EPHEMERAL_SERVER_KEY_PEM", &bundle.server_key_pem);

            Some(bundle)
        } else {
            None
        };

        let container = cbt_definition.run_detached();

        let port = pg_client::Port(
            container
                .read_host_tcp_port(5432)
                .expect("port 5432 not published"),
        );

        let (host, host_addr, ssl_mode, ssl_root_cert) =
            if let Some(ssl_config) = &definition.ssl_config {
                let hostname = match ssl_config {
                    definition::SslConfig::Generated { hostname } => hostname.clone(),
                };

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let ca_cert_path =
                    std::env::temp_dir().join(format!("pg_ephemeral_ca_{}.crt", timestamp));
                std::fs::write(&ca_cert_path, &ssl_bundle.as_ref().unwrap().ca_cert_pem)
                    .expect("Failed to write CA certificate to temp file");

                (
                    pg_client::Host::HostName(hostname),
                    Some("127.0.0.1".parse().unwrap()),
                    pg_client::SslMode::VerifyFull,
                    Some(pg_client::SslRootCert::File(ca_cert_path)),
                )
            } else {
                (
                    pg_client::host!("localhost"),
                    None,
                    pg_client::SslMode::Disable,
                    None,
                )
            };

        let client_config = pg_client::Config {
            application_name: definition.application_name.clone(),
            database: definition.database.clone(),
            endpoint: pg_client::Endpoint::Network {
                host,
                host_addr,
                port: Some(port),
            },
            password: Some(password),
            ssl_mode,
            ssl_root_cert,
            username: definition.superuser.clone(),
        };

        Container {
            host_port: port,
            container,
            definition,
            client_config,
        }
    }

    fn migration_context(&'a self) -> mmigration::Context<'a, SchemaDump<'a>> {
        let migration_config = self
            .definition
            .migration_config
            .as_ref()
            .expect("migration not configured");

        mmigration::Context::load(
            migration_config,
            &self.client_config,
            SchemaDump { container: self },
        )
    }

    pub async fn wait_available(&self) {
        let config = self.client_config.to_sqlx_connect_options().unwrap();

        let start = std::time::Instant::now();
        let max_duration = std::time::Duration::from_secs(10);
        let sleep_duration = std::time::Duration::from_millis(100);

        let mut last_error: Option<_> = None;

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

                    return;
                }
                Err(error) => {
                    log::trace!("{error:#?}, retry in 100ms");
                    last_error = Some(error);
                }
            }
            tokio::time::sleep(sleep_duration).await;
        }

        panic!(
            "Container did not become available within ~10 seconds! Last connection error: {last_error:#?}"
        );
    }

    fn exec_schema_dump(&self) -> String {
        convert_schema(&self.container.exec_capture_only_stdout(
            self.container_client_config().to_pg_env(),
            "pg_dump",
            ["--schema-only"],
        ))
    }

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

    pub async fn apply_pending_migrations(&self) {
        self.migration_context().apply_pending().await
    }

    pub async fn apply_pending_migrations_no_schema_dump(&self) {
        self.migration_context()
            .apply_pending_no_schema_dump()
            .await
    }

    pub async fn apply_sql_file(&self, path: &std::path::Path) {
        self.apply_sql(&std::fs::read_to_string(path).unwrap())
            .await
    }

    pub async fn apply_sql_file_git_revision(
        &self,
        path: &std::path::Path,
        git_revision: impl Into<String>,
    ) {
        let git_revision = git_revision.into();
        let sql = cbt::Command::new("git")
            .argument("show")
            .argument(format!("{git_revision}:{}", path.to_str().unwrap()))
            .capture_only_stdout_string();

        self.apply_sql(&sql).await
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

    fn exec_container_shell(&self) {
        self.container
            .exec_interactive(self.container_client_config().to_pg_env(), "sh", [])
    }

    fn exec_psql(&self) {
        self.container
            .exec_interactive(self.container_client_config().to_pg_env(), "psql", [])
    }

    fn container_client_config(&self) -> pg_client::Config {
        let mut config = self.client_config.clone();
        if let pg_client::Endpoint::Network {
            ref host,
            ref host_addr,
            ..
        } = config.endpoint
        {
            config.endpoint = pg_client::Endpoint::Network {
                host: host.clone(),
                host_addr: host_addr.clone(),
                port: Some(pg_client::Port(5432)),
            };
        }
        config
    }

    pub fn cross_container_client_config(&self) -> pg_client::Config {
        // Resolve the container host from inside a container
        // This DNS name only works from inside containers, not from the host
        let ip_address = self
            .definition
            .backend
            .resolve_container_host()
            .expect("Failed to resolve container host from container");

        let endpoint = pg_client::Endpoint::Network {
            host: pg_client::Host::IpAddr(ip_address),
            host_addr: None,
            port: Some(self.host_port),
        };

        self.client_config.clone().endpoint(endpoint)
    }

    pub fn pg_env(&self) -> std::collections::BTreeMap<&'static str, String> {
        self.client_config.to_pg_env()
    }

    pub fn database_url(&self) -> String {
        self.client_config.to_url().to_string()
    }

    fn stop(&mut self) {
        self.container.stop()
    }
}

fn generate_password() -> pg_client::Password {
    let rng = rand::rng();

    let value: String = rng
        .sample_iter(rand::distr::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    <pg_client::Password as std::str::FromStr>::from_str(&value).unwrap()
}

pub(crate) fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
pub struct InstanceName(pub String);

impl std::default::Default for InstanceName {
    fn default() -> Self {
        Self("main".to_string())
    }
}

impl std::str::FromStr for InstanceName {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.into()))
    }
}

impl std::fmt::Display for InstanceName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

pub type InstanceMap = std::collections::BTreeMap<InstanceName, Definition>;
