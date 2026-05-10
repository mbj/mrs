use rand::RngExt;

use crate::LOCALHOST_HOST_ADDR;
use crate::LOCALHOST_IP;
use crate::UNSPECIFIED_IP;
use crate::certificate;
use crate::definition;

pub const PGDATA: &str = "/var/lib/pg-ephemeral";

/// Cached host TTY status: true iff *both* stdin and stdout are terminals.
///
/// Requiring both avoids two failure modes:
/// - cargo-test-style invocations forward stdin TTY but capture stdout —
///   forcing `--tty` on these mangles output with PTY CRLF translation.
/// - `pg-ephemeral psql > file.sql` captures stdout — PTY would corrupt
///   the redirected stream.
///
/// pg-ephemeral's CLI doesn't manipulate fd 0/1 mid-process, so the value
/// is stable for the process lifetime.
///
/// TODO: this lives here for now alongside the only consumers; should move
/// to a CLI-scoped module so the `Container` library API doesn't bake in
/// the TTY-autodetect policy.
static HOST_HAS_TTY: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
});

/// Allocate a TTY on the in-container exec iff host stdin is a terminal.
///
/// Forcing `--tty` unconditionally breaks captured-stdio callers (test
/// harnesses, scripts capturing output): podman exec needs a real TTY and
/// the call hangs without one. Forcing no-TTY breaks interactive callers
/// (`pg-ephemeral psql` in a terminal expects a REPL). Mirroring host
/// stdin's terminal-ness matches what a local tool would do — TTY when
/// called from a terminal, pipe when called from a script.
///
/// ociman's [`ociman::ExecCommand::tty`] stays 1:1 with the runtime `--tty`
/// flag; this is pg-ephemeral's CLI-side policy.
trait TtyIfTerminal: Sized {
    fn tty_if_terminal(self) -> Self;
}

impl TtyIfTerminal for ociman::ExecCommand<'_> {
    fn tty_if_terminal(self) -> Self {
        if *HOST_HAS_TTY { self.tty() } else { self }
    }
}

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
    #[error("Failed to apply pg-ephemeral metadata labels")]
    ApplyLabels(#[from] crate::label::ApplyError),
    #[error("Failed to decode pg-ephemeral metadata labels")]
    DecodeImageLabels(#[from] crate::label::ReadImageError),
    #[error("Failed to inspect cache image {reference}")]
    InspectImage {
        reference: ociman::Reference,
        #[source]
        source: ociman::label::ImageError,
    },
    #[error("Failed to serialize image metadata as JSON")]
    SerializeImageMetadata(#[source] serde_json::Error),
    #[error("Failed to read host TCP port from container")]
    ReadHostTcpPort(#[from] ociman::ReadHostTcpPortError),
    #[error(transparent)]
    SeedApply(#[from] crate::definition::SeedApplyError),
    #[error(transparent)]
    SeedLoad(#[from] crate::seed::LoadError),
    #[error("Failed to terminate backend connections")]
    TerminateConnections(#[source] sqlx::Error),
    #[error("Failed to checkpoint")]
    Checkpoint(#[source] sqlx::Error),
    #[error("Failed to stop container")]
    ContainerStop(#[source] cmd_proc::CommandError),
    #[error("Failed to remove container")]
    ContainerRemove(#[source] cmd_proc::CommandError),
    #[error(transparent)]
    EnvVariableValue(#[from] cmd_proc::EnvVariableValueError),
    #[error(
        "Parameter `{name}` conflicts with ssl_config; pg-ephemeral controls this parameter when ssl_config is set"
    )]
    ParameterConflictsWithSslConfig { name: pg_client::parameter::Name },
}
const ENV_POSTGRES_PASSWORD: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_PASSWORD");
const ENV_POSTGRES_USER: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_USER");
const ENV_PGDATA: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("PGDATA");
const ENV_PG_EPHEMERAL_SSL_DIR: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SSL_DIR");
const ENV_PG_EPHEMERAL_CA_CERT_PEM: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_CA_CERT_PEM");
const ENV_PG_EPHEMERAL_SERVER_CERT_PEM: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SERVER_CERT_PEM");
const ENV_PG_EPHEMERAL_SERVER_KEY_PEM: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("PG_EPHEMERAL_SERVER_KEY_PEM");

const SSL_DIR: &str = "/var/lib/postgresql";

// PG parameter names pg-ephemeral controls when ssl_config is set. Declared
// as `static` (not `const`) so references can outlive the use site and feed
// into the effective-parameters map without cloning.
static PARAM_SSL: pg_client::parameter::Name =
    pg_client::parameter::Name::from_static_or_panic("ssl");
static PARAM_SSL_CERT_FILE: pg_client::parameter::Name =
    pg_client::parameter::Name::from_static_or_panic("ssl_cert_file");
static PARAM_SSL_KEY_FILE: pg_client::parameter::Name =
    pg_client::parameter::Name::from_static_or_panic("ssl_key_file");
static PARAM_SSL_CA_FILE: pg_client::parameter::Name =
    pg_client::parameter::Name::from_static_or_panic("ssl_ca_file");

// Parameter values for SSL. Paths are kept as full literals (rather than
// composed from SSL_DIR at runtime) so the construction is const and
// infallible. Keep in sync with SSL_DIR.
static VALUE_ON: pg_client::parameter::Value =
    pg_client::parameter::Value::from_static_or_panic("on");
static VALUE_SSL_CERT_PATH: pg_client::parameter::Value =
    pg_client::parameter::Value::from_static_or_panic("/var/lib/postgresql/server.crt");
static VALUE_SSL_KEY_PATH: pg_client::parameter::Value =
    pg_client::parameter::Value::from_static_or_panic("/var/lib/postgresql/server.key");
static VALUE_SSL_CA_PATH: pg_client::parameter::Value =
    pg_client::parameter::Value::from_static_or_panic("/var/lib/postgresql/root.crt");

// Custom entrypoint script for SSL boots. Writes the generated CA/server
// cert/key PEMs from env variables to disk with the right ownership and
// permissions, then execs the normal postgres entrypoint. PG-side SSL
// configuration (ssl=on, ssl_*_file=...) is injected separately via the
// `parameters` `-c` flag mechanism — this script handles only the
// filesystem staging that has to happen before postgres starts.
const SSL_FILE_STAGING_SCRIPT: &str = r#"
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
pub struct Container {
    host_port: pg_client::config::Port,
    pub(crate) client_config: pg_client::Config,
    container: ociman::Container,
    backend: ociman::Backend,
    wait_available_timeout: std::time::Duration,
}

impl Container {
    pub(crate) async fn run_definition(
        definition: &crate::definition::Definition,
        seeds: &[crate::label::SeedEntry],
    ) -> Result<Self, Error> {
        if definition.ssl_config.is_some() {
            for reserved in [
                &PARAM_SSL,
                &PARAM_SSL_CERT_FILE,
                &PARAM_SSL_KEY_FILE,
                &PARAM_SSL_CA_FILE,
            ] {
                if definition.parameters.contains_key(reserved) {
                    return Err(Error::ParameterConflictsWithSslConfig {
                        name: reserved.clone(),
                    });
                }
            }
        }

        let password = generate_password();

        let host_ip = if definition.cross_container_access {
            UNSPECIFIED_IP
        } else {
            LOCALHOST_IP
        };

        let mut ociman_definition = definition
            .to_ociman_definition()
            .environment_variable(
                ENV_POSTGRES_PASSWORD,
                password.as_ref().parse::<cmd_proc::EnvVariableValue>()?,
            )
            .environment_variable(
                ENV_POSTGRES_USER,
                definition
                    .superuser
                    .as_ref()
                    .parse::<cmd_proc::EnvVariableValue>()?,
            )
            .environment_variable(ENV_PGDATA, "/var/lib/pg-ephemeral")
            .publish(ociman::Publish::tcp(5432).host_ip(host_ip));

        if definition.remove {
            ociman_definition = ociman_definition.remove();
        }

        if let Some(workdir) = &definition.transparent_workdir {
            let workdir_str = workdir.as_str();
            ociman_definition = ociman_definition.mount(format!(
                "type=bind,source={workdir_str},target={workdir_str}"
            ));
        }

        let mut effective_parameters: std::collections::BTreeMap<
            &pg_client::parameter::Name,
            &pg_client::parameter::Value,
        > = definition.parameters.iter().collect();

        let ssl_bundle = if let Some(ssl_config) = &definition.ssl_config {
            let definition::SslConfig::Generated { hostname } = ssl_config;
            let bundle = certificate::Bundle::generate(hostname.as_str())
                .expect("Failed to generate SSL certificate bundle");

            ociman_definition = ociman_definition
                .entrypoint("sh")
                .argument("-e")
                .argument("-c")
                .argument(SSL_FILE_STAGING_SCRIPT)
                .argument("--")
                .argument("postgres")
                .environment_variable(ENV_PG_EPHEMERAL_SSL_DIR, SSL_DIR)
                .environment_variable(
                    ENV_PG_EPHEMERAL_CA_CERT_PEM,
                    bundle.ca_cert_pem.parse::<cmd_proc::EnvVariableValue>()?,
                )
                .environment_variable(
                    ENV_PG_EPHEMERAL_SERVER_CERT_PEM,
                    bundle
                        .server_cert_pem
                        .parse::<cmd_proc::EnvVariableValue>()?,
                )
                .environment_variable(
                    ENV_PG_EPHEMERAL_SERVER_KEY_PEM,
                    bundle
                        .server_key_pem
                        .parse::<cmd_proc::EnvVariableValue>()?,
                );

            effective_parameters.insert(&PARAM_SSL, &VALUE_ON);
            effective_parameters.insert(&PARAM_SSL_CERT_FILE, &VALUE_SSL_CERT_PATH);
            effective_parameters.insert(&PARAM_SSL_KEY_FILE, &VALUE_SSL_KEY_PATH);
            effective_parameters.insert(&PARAM_SSL_CA_FILE, &VALUE_SSL_CA_PATH);

            Some(bundle)
        } else {
            None
        };

        // PG parameters (user-supplied via `[instances.X.parameters]` plus any
        // pg-ephemeral-controlled additions like the SSL block above) become
        // `-c name=value` flags appended to the container command. BTreeMap
        // iteration is sorted, so the resulting flag order is deterministic.
        // Name validation rejects `=`, so the first `=` in each formatted
        // flag is unambiguously the name/value separator. Without SSL, these
        // flags become the container CMD which docker-entrypoint.sh prepends
        // `postgres` to; with SSL, they extend the explicit `postgres`
        // invocation in the SSL file-staging script.
        for (name, value) in &effective_parameters {
            ociman_definition = ociman_definition
                .argument("-c")
                .argument(format!("{name}={value}"));
        }

        ociman_definition = crate::label::apply(
            ociman_definition,
            definition,
            &password,
            ssl_bundle.as_ref(),
            seeds,
        )?;

        let container = ociman_definition.run_detached().await;
        let port: pg_client::config::Port = container.read_host_tcp_port(5432).await?.into();

        let (host, host_addr, ssl_mode, ssl_root_cert) =
            if let Some(ssl_config) = &definition.ssl_config {
                let definition::SslConfig::Generated { hostname } = ssl_config;

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let ca_cert_path =
                    std::env::temp_dir().join(format!("pg_ephemeral_ca_{timestamp}.crt"));
                std::fs::write(&ca_cert_path, &ssl_bundle.as_ref().unwrap().ca_cert_pem)
                    .expect("Failed to write CA certificate to temp file");

                (
                    pg_client::config::Host::HostName(hostname.clone()),
                    Some(LOCALHOST_HOST_ADDR),
                    pg_client::config::SslMode::VerifyFull,
                    Some(pg_client::config::SslRootCert::File(ca_cert_path)),
                )
            } else {
                (
                    pg_client::config::Host::IpAddr(LOCALHOST_IP),
                    None,
                    pg_client::config::SslMode::Disable,
                    None,
                )
            };

        let client_config = pg_client::Config {
            endpoint: pg_client::config::Endpoint::Network {
                host,
                channel_binding: None,
                host_addr,
                port: Some(port),
            },
            session: pg_client::config::Session {
                application_name: definition.application_name.clone(),
                database: definition.database.clone(),
                password: Some(password.clone()),
                user: definition.superuser.clone(),
            },
            ssl_mode,
            ssl_root_cert,
            sqlx: Default::default(),
        };

        Ok(Container {
            host_port: port,
            container,
            backend: definition.backend.clone(),
            client_config,
            wait_available_timeout: definition.wait_available_timeout,
        })
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

    pub async fn exec_schema_dump(
        &self,
        pg_schema_dump: &pg_client::PgSchemaDump,
    ) -> Result<String, Error> {
        let output = self
            .container
            .exec("pg_dump")
            .arguments(pg_schema_dump.arguments())
            .environment_variables(self.container_client_config().pg_env()?)
            .build()
            .stdout_capture()
            .bytes()
            .await
            .unwrap();
        Ok(crate::convert_schema(&output))
    }

    #[must_use]
    pub fn client_config(&self) -> &pg_client::Config {
        &self.client_config
    }

    pub async fn labels(
        &self,
    ) -> Result<ociman::label::ContainerLabels, ociman::label::ContainerError> {
        self.container.labels().await
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

    pub async fn apply_sql(&self, sql: &str) -> Result<(), sqlx::Error> {
        self.with_connection(async |connection| {
            log::debug!("Executing: {sql}");
            sqlx::raw_sql(sqlx::AssertSqlSafe(sql))
                .execute(connection)
                .await
                .map(|_| ())
        })
        .await
    }

    /// Apply CSV content to a table using PostgreSQL's COPY protocol.
    ///
    /// The line delimiter is hardcoded to `\n`.
    pub async fn apply_csv(
        &self,
        table: &pg_client::QualifiedTable,
        delimiter: char,
        content: &str,
    ) -> Result<(), sqlx::Error> {
        self.with_connection(async |connection| {
            let header_line = content.lines().next().unwrap_or_default();

            let columns: Vec<&str> = header_line.split(delimiter).map(str::trim).collect();

            let row = sqlx::query(
                r#"SELECT 'COPY ' || format('%I.%I', $1, $2)
                    || '(' || (SELECT string_agg(format('%I', "column"), ', ') FROM unnest($3::text[]) AS "column") || ')'
                    || ' FROM STDIN WITH (FORMAT csv, HEADER MATCH, DELIMITER ' || quote_literal($4) || ')'
                    AS statement"#,
            )
            .bind(table.schema.as_ref())
            .bind(table.table.as_ref())
            .bind(&columns)
            .bind(delimiter.to_string())
            .fetch_one(&mut *connection)
            .await?;
            let statement: String = sqlx::Row::get(&row, "statement");

            log::debug!("Executing: {statement}");
            let mut copy = connection.copy_in_raw(&statement).await?;
            copy.send(content.as_bytes()).await?;
            copy.finish().await?;
            Ok(())
        })
        .await
    }

    pub(crate) async fn exec_container_script(
        &self,
        script: &str,
    ) -> Result<(), cmd_proc::CommandError> {
        self.container
            .exec("sh")
            .arguments(["-e", "-c", script])
            .build()
            .status()
            .await
    }

    pub(crate) async fn exec_container_shell(&self) -> Result<(), Error> {
        self.container
            .exec("sh")
            .environment_variables(self.container_client_config().pg_env()?)
            .tty_if_terminal()
            .interactive()
            .status()
            .await
            .unwrap();
        Ok(())
    }

    pub(crate) async fn exec_pgbench(&self, arguments: &[String]) -> Result<(), Error> {
        let mut env = self.container_client_config().pg_env()?;
        env.insert(
            cmd_proc::EnvVariableName::from_static_or_panic("PGHOST"),
            cmd_proc::EnvVariableValue::from_static_or_panic("/var/run/postgresql"),
        );
        self.container
            .exec("pgbench")
            .environment_variables(env)
            .arguments(arguments.iter().cloned())
            .status()
            .await
            .unwrap();
        Ok(())
    }

    pub(crate) async fn exec_psql(&self) -> Result<(), Error> {
        self.container
            .exec("psql")
            .environment_variables(self.container_client_config().pg_env()?)
            .tty_if_terminal()
            .interactive()
            .status()
            .await
            .unwrap();
        Ok(())
    }

    pub async fn exec_run_env(&self, command: &str, arguments: &[String]) -> Result<(), Error> {
        let config = self.container_unix_socket_config();
        self.container
            .exec(command)
            .arguments(arguments.iter().cloned())
            .environment_variables(config.pg_env()?)
            .environment_variable(
                crate::ENV_DATABASE_URL,
                config
                    .to_url_string()
                    .parse::<cmd_proc::EnvVariableValue>()?,
            )
            .tty_if_terminal()
            .interactive()
            .status()
            .await?;
        Ok(())
    }

    /// Pick the `--user UID:GID` value to pass to transparent-mode execs so
    /// that bind-mount writes come back owned by the host user.
    ///
    /// - **Rootless** (podman rootless, rootless docker): the default
    ///   user-namespace maps container uid 0 to the running host user, so
    ///   exec'ing as `0:0` makes writes land as the host user.
    /// - **Rootful** (rootful docker, rootful podman): no user namespace —
    ///   container uid == host uid directly, so we pass the host
    ///   `(getuid, getgid)` straight through.
    ///
    /// macOS Docker Desktop / Podman Machine run a Linux VM whose
    /// rootless-ness `Backend::is_rootless` reflects; either branch works
    /// because the host↔VM FS share layer translates ownership independently.
    fn transparent_user(&self) -> (rustix::process::Uid, rustix::process::Gid) {
        if self.backend.is_rootless() {
            (rustix::process::Uid::ROOT, rustix::process::Gid::ROOT)
        } else {
            (rustix::process::getuid(), rustix::process::getgid())
        }
    }

    /// Build the shared `ExecCommand` base for transparent-mode operations:
    /// in-container unix-socket PG\* env vars, `--workdir`, and
    /// `--user UID:GID` (picked by [`Self::transparent_user`] so bind-mount
    /// writes come back owned by the host user on all backend modes).
    /// Callers add operation-specific arguments and the terminal
    /// (`.tty_if_terminal().interactive().status()` vs `.build().stdout_capture()`).
    /// `DATABASE_URL` is intentionally omitted; only `run-env` sets it (its
    /// callee may be arbitrary user code that reads the variable).
    #[allow(
        clippy::result_large_err,
        reason = "container::Error aggregates diagnostic-rich variants; this private helper is called once per CLI invocation, not on a hot path where the 128-byte threshold matters"
    )]
    fn exec_transparent(
        &self,
        executable: &str,
        workdir: &crate::definition::TransparentWorkdir,
    ) -> Result<ociman::ExecCommand<'_>, Error> {
        let (uid, gid) = self.transparent_user();
        Ok(self
            .container
            .exec(executable)
            .environment_variables(self.container_unix_socket_config().pg_env()?)
            .workdir(workdir.as_path())
            .user(uid, gid))
    }

    /// Run an interactive (PTY + stdin) transparent exec to completion.
    /// Builds on [`Self::exec_transparent`] and adds the
    /// `.tty_if_terminal().interactive().status()` terminal common to `psql`, `run-env`,
    /// and `shell`.
    async fn exec_transparent_interactive(
        &self,
        executable: &str,
        workdir: &crate::definition::TransparentWorkdir,
        arguments: &[String],
    ) -> Result<(), Error> {
        self.exec_transparent(executable, workdir)?
            .arguments(arguments.iter().cloned())
            .tty_if_terminal()
            .interactive()
            .status()
            .await?;
        Ok(())
    }

    pub async fn exec_transparent_psql(
        &self,
        workdir: &crate::definition::TransparentWorkdir,
    ) -> Result<(), Error> {
        self.exec_transparent_interactive("psql", workdir, &[])
            .await
    }

    pub async fn exec_transparent_run_env(
        &self,
        workdir: &crate::definition::TransparentWorkdir,
        command: &str,
        arguments: &[String],
    ) -> Result<(), Error> {
        let database_url = self
            .container_unix_socket_config()
            .to_url_string()
            .parse::<cmd_proc::EnvVariableValue>()?;
        self.exec_transparent(command, workdir)?
            .arguments(arguments.iter().cloned())
            .environment_variable(crate::ENV_DATABASE_URL, database_url)
            .tty_if_terminal()
            .interactive()
            .status()
            .await?;
        Ok(())
    }

    pub async fn exec_transparent_schema_dump(
        &self,
        workdir: &crate::definition::TransparentWorkdir,
        pg_schema_dump: &pg_client::PgSchemaDump,
    ) -> Result<String, Error> {
        let output = self
            .exec_transparent("pg_dump", workdir)?
            .arguments(pg_schema_dump.arguments())
            .build()
            .stdout_capture()
            .bytes()
            .await?;
        Ok(crate::convert_schema(&output))
    }

    pub async fn exec_transparent_shell(
        &self,
        workdir: &crate::definition::TransparentWorkdir,
    ) -> Result<(), Error> {
        self.exec_transparent_interactive("sh", workdir, &[]).await
    }

    pub async fn exec_transparent_pgbench(
        &self,
        workdir: &crate::definition::TransparentWorkdir,
        arguments: &[String],
    ) -> Result<(), Error> {
        self.exec_transparent_interactive("pgbench", workdir, arguments)
            .await
    }

    fn container_client_config(&self) -> pg_client::Config {
        let mut config = self.client_config.clone();
        if let pg_client::config::Endpoint::Network {
            ref host,
            ref channel_binding,
            ref host_addr,
            ..
        } = config.endpoint
        {
            config.endpoint = pg_client::config::Endpoint::Network {
                host: host.clone(),
                channel_binding: *channel_binding,
                host_addr: host_addr.clone(),
                port: Some(pg_client::config::Port::new(5432)),
            };
        }
        config
    }

    fn container_unix_socket_config(&self) -> pg_client::Config {
        let mut config = self.client_config.clone();
        config.endpoint = pg_client::config::Endpoint::SocketPath(std::path::PathBuf::from(
            "/var/run/postgresql",
        ));
        config.ssl_mode = pg_client::config::SslMode::Disable;
        config.ssl_root_cert = None;
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
            pg_client::config::Endpoint::Network {
                channel_binding, ..
            } => *channel_binding,
            pg_client::config::Endpoint::SocketPath(_) => None,
        };

        let endpoint = pg_client::config::Endpoint::Network {
            host: pg_client::config::Host::IpAddr(ip_address),
            channel_binding,
            host_addr: None,
            port: Some(self.host_port),
        };

        self.client_config.clone().endpoint(endpoint)
    }

    pub fn pg_env(
        &self,
    ) -> Result<
        std::collections::BTreeMap<cmd_proc::EnvVariableName, cmd_proc::EnvVariableValue>,
        cmd_proc::EnvVariableValueError,
    > {
        self.client_config.pg_env()
    }

    #[must_use]
    pub fn database_url(&self) -> String {
        self.client_config.to_url_string()
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.container.stop().await.map_err(Error::ContainerStop)
    }

    async fn terminate_connections(&self) -> Result<(), Error> {
        self.apply_sql(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND backend_type = 'client backend'",
        )
        .await
        .map_err(Error::TerminateConnections)
    }

    async fn checkpoint(&self) -> Result<(), Error> {
        self.apply_sql("CHECKPOINT")
            .await
            .map_err(Error::Checkpoint)
    }

    /// Stop the container (clean PostgreSQL shutdown), commit it to an image,
    /// and remove the stopped container.
    pub(crate) async fn stop_commit_remove(
        &mut self,
        reference: &ociman::Reference,
    ) -> Result<(), Error> {
        self.terminate_connections().await?;
        self.checkpoint().await?;
        self.container.stop().await.map_err(Error::ContainerStop)?;
        self.container.commit(reference, false).await.unwrap();
        self.container
            .remove()
            .await
            .map_err(Error::ContainerRemove)?;
        Ok(())
    }

    async fn wait_for_container_socket(&self) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let max_duration = self.wait_available_timeout;
        let sleep_duration = std::time::Duration::from_millis(100);

        while start.elapsed() <= max_duration {
            if self
                .container
                .exec("pg_isready")
                .argument("--host")
                .argument("localhost")
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
        password: &pg_client::config::Password,
    ) -> Result<(), Error> {
        self.wait_for_container_socket().await?;

        self.container
            .exec("psql")
            .argument("--host")
            .argument("/var/run/postgresql")
            .argument("--username")
            .argument(self.client_config.session.user.as_ref())
            .argument("--dbname")
            .argument("postgres")
            .argument("--variable")
            .argument(format!(
                "target_user={}",
                self.client_config.session.user.as_ref()
            ))
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

fn generate_password() -> pg_client::config::Password {
    let value: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    <pg_client::config::Password as std::str::FromStr>::from_str(&value).unwrap()
}
