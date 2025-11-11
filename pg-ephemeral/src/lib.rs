use rand::Rng;

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

struct SchemaDump<'a> {
    container: &'a Container<'a>,
}

impl mmigration::SchemaDump for SchemaDump<'_> {
    async fn schema_dump(&self) -> mmigration::Schema {
        self.container.exec_schema_dump().into()
    }
}

#[derive(Debug)]
pub struct Container<'a> {
    client_config: pg_client::Config,
    container: cbt::Container,
    definition: &'a Definition,
}

impl<'a> Container<'a> {
    fn run(definition: &'a Definition) -> Self {
        let password = generate_password();

        let container = definition
            .to_cbt_definition()
            .remove()
            .env("POSTGRES_PASSWORD", password.as_ref())
            .env("POSTGRES_USER", definition.superuser.as_ref())
            .publish(cbt::Publish::from("127.0.0.1::5432/tcp"))
            .run_detached();

        let port = pg_client::Port(
            container
                .inspect_format(
                    "{{(index (index .NetworkSettings.Ports \"5432/tcp\") 0).HostPort}}",
                )
                .parse()
                .expect("invalid port"),
        );

        let host = pg_client::host!("localhost");

        let client_config = pg_client::Config {
            application_name: definition.application_name.clone(),
            database: definition.database.clone(),
            endpoint: pg_client::Endpoint::Network {
                host,
                host_addr: None,
                port: Some(port),
            },
            password: Some(password),
            ssl_mode: pg_client::SslMode::Disable,
            ssl_root_cert: None,
            username: definition.superuser.clone(),
        };

        Container {
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

        for _ in 0..100 {
            match sqlx::ConnectOptions::connect(&config).await {
                Ok(connection) => {
                    sqlx::Connection::close(connection)
                        .await
                        .expect("connection close failed");

                    return;
                }
                Err(error) => {
                    log::trace!("{error:#?}, retry in 100ms");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        panic!("container did not become avaialble within ~10 seconds!");
    }

    fn exec_schema_dump(&self) -> String {
        convert_schema(&self.container.exec_capture_only_stdout(
            self.container_client_config().to_pg_env(),
            "pg_dump",
            ["--schema-only"],
        ))
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
        git_revision: &'static str,
    ) {
        let sql = cbt::Command::new("git")
            .argument("show")
            .argument(format!("{git_revision}:{}", path.to_str().unwrap()))
            .capture_only_stdout_string();

        self.apply_sql(&sql).await
    }

    pub async fn apply_sql(&self, sql: &str) {
        self.with_connection(async |connection| {
            log::debug!("Executing: {sql}");
            sqlx::raw_sql(sql).execute(connection).await.unwrap();
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

fn convert_schema(value: &[u8]) -> String {
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
