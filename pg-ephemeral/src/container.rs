use rand::RngExt;

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
pub(crate) trait TtyIfTerminal: Sized {
    fn tty_if_terminal(self) -> Self;
}

impl TtyIfTerminal for ociman::ExecCommand<'_> {
    fn tty_if_terminal(self) -> Self {
        if *HOST_HAS_TTY { self.tty() } else { self }
    }
}

impl TtyIfTerminal for ociman::Definition {
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
        source: Option<pg_client::session::StartError>,
    },
    #[error("Failed to execute command in container")]
    ContainerExec(#[from] cmd_proc::CommandError),
    #[error("Failed to apply pg-ephemeral metadata labels")]
    ApplyLabels(#[from] crate::label::ApplyError),
    #[error("Failed to decode pg-ephemeral metadata labels")]
    DecodeMetadataLabels(#[from] crate::label::ReadError),
    #[error("Failed to inspect cache image {reference}")]
    InspectImage {
        reference: Box<ociman::Reference>,
        #[source]
        source: ociman::label::ImageError,
    },
    #[error("Failed to serialize pg-ephemeral metadata as JSON")]
    SerializeMetadata(#[source] serde_json::Error),
    #[error("Failed to materialize pg-ephemeral client config")]
    PrepareConfig(#[from] crate::label::PrepareConfigError),
    #[error("Failed to start container")]
    RunDetached(#[from] ociman::RunDetachedError),
    #[error("Failed to close the availability probe session")]
    CloseProbe(#[source] pg_client::session::TerminateError),
    #[error("Failed to run a session")]
    Session(#[from] pg_client::session::OpenError),
    #[error("Failed to read host TCP port from container")]
    ReadHostTcpPort(#[from] ociman::ReadHostTcpPortError),
    #[error(transparent)]
    SeedApply(#[from] crate::definition::SeedApplyError),
    #[error(transparent)]
    SeedLoad(#[from] crate::seed::LoadError),
    #[error("Failed to terminate backend connections")]
    TerminateConnections(#[source] SqlError),
    #[error("Failed to checkpoint")]
    Checkpoint(#[source] SqlError),
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
    #[error("Failed to acquire cache build lock")]
    CacheLock(#[from] crate::cache_lock::Error),
    #[error("Failed to commit cache image {reference}")]
    Commit {
        reference: Box<ociman::Reference>,
        #[source]
        source: cmd_proc::CommandError,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum SqlError {
    #[error("Failed to connect")]
    Connection(#[from] pg_client::session::OpenError),
    #[error("Failed to execute SQL")]
    Execute(#[from] pg_client::session::execute::Error),
    #[error("Generated COPY statement cannot be read")]
    CopyStatement(#[source] pg_client::session::execute::ColumnError),
}

#[derive(Debug, thiserror::Error)]
pub enum AttachSessionError {
    #[error("Failed to read host TCP port from session container")]
    ReadHostTcpPort(#[from] ociman::ReadHostTcpPortError),
    #[error("Failed to read session container labels")]
    ReadLabels(#[from] ociman::label::ContainerError),
    #[error("Failed to decode pg-ephemeral metadata from session labels")]
    DecodeMetadata(#[from] crate::label::ReadError),
    #[error("Failed to materialize pg-ephemeral client config from session labels")]
    PrepareConfig(#[from] crate::label::PrepareConfigError),
}

/// Port PostgreSQL listens on inside the container — the standard PG
/// default. Used as the container-side endpoint for `--publish` and for
/// every `read_host_tcp_port` lookup that resolves the host-side mapping.
const PG_CONTAINER_PORT: u16 = 5432;

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
    pub(crate) libpq_config: pg_client::libpq::Config,
    session_config: std::sync::Arc<pg_client::session::Config>,
    pub(crate) superuser: crate::label::SuperuserMetadata,
    container: ociman::Container,
    backend: ociman::Backend,
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
            .publish(ociman::Publish::tcp(PG_CONTAINER_PORT).host_ip(host_ip));

        if definition.remove {
            ociman_definition = ociman_definition.remove();
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

        let container = ociman_definition.run_detached().await?;
        let port: pg_client::config::Port = container
            .read_host_tcp_port(PG_CONTAINER_PORT)
            .await?
            .into();

        let superuser = crate::label::SuperuserMetadata {
            user: definition.superuser.clone(),
            database: definition.database.clone(),
            password: password.clone(),
            application: definition.application_name.clone(),
        };

        let ssl = match (&definition.ssl_config, &ssl_bundle) {
            (Some(definition::SslConfig::Generated { hostname }), Some(bundle)) => {
                Some(crate::label::SslMetadata {
                    hostname: hostname.clone(),
                    ca_cert_pem: bundle.ca_cert_pem.clone(),
                })
            }
            _ => None,
        };

        let libpq_config = crate::label::libpq_config(&superuser, ssl.as_ref(), port)?;
        let session_config = crate::label::session_config(&superuser, ssl.as_ref(), port)?;

        Ok(Container {
            host_port: port,
            container,
            backend: definition.backend.clone(),
            libpq_config,
            session_config: std::sync::Arc::new(session_config),
            superuser,
        })
    }

    /// Build a Container view of an already-running named session.
    ///
    /// Reads the published host TCP port from the ociman handle and
    /// decodes the superuser credentials + SSL bundle from container
    /// labels (which [`crate::label::apply`] writes at boot time) to
    /// materialize a [`pg_client::libpq::Config`] equivalent to what
    /// [`Self::run_definition`] would have produced.
    pub async fn attach_session(
        session: crate::session::Session,
        backend: ociman::Backend,
    ) -> Result<Self, AttachSessionError> {
        let container = session.into_ociman_container();
        let port: pg_client::config::Port = container
            .read_host_tcp_port(PG_CONTAINER_PORT)
            .await?
            .into();
        let labels = container.labels().await?;
        let metadata = crate::label::read_container(&labels)?;

        let libpq_config =
            crate::label::libpq_config(&metadata.superuser, metadata.ssl.as_ref(), port)?;
        let session_config =
            crate::label::session_config(&metadata.superuser, metadata.ssl.as_ref(), port)?;

        Ok(Container {
            host_port: port,
            container,
            backend,
            libpq_config,
            session_config: std::sync::Arc::new(session_config),
            superuser: metadata.superuser,
        })
    }

    pub async fn wait_available(&self, timeout: crate::definition::Timeout) -> Result<(), Error> {
        let config = self.session_config();

        let start = std::time::Instant::now();
        let sleep_duration = std::time::Duration::from_millis(100);

        let mut last_error: Option<pg_client::session::StartError> = None;

        loop {
            if let crate::definition::Timeout::After(duration) = timeout
                && start.elapsed() > duration
            {
                return Err(Error::ConnectionTimeout {
                    timeout: duration,
                    source: last_error,
                });
            }

            log::trace!("connection attempt");
            match pg_client::session::start(std::sync::Arc::clone(&config)).await {
                Ok(session) => {
                    session.send_terminate().await.map_err(Error::CloseProbe)?;

                    log::debug!(
                        "pg is available on endpoint: {:#?}",
                        self.session_config.connection.endpoint
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
            .to_cmd_proc_command()
            .stdout_capture()
            .bytes()
            .await
            .unwrap();
        Ok(crate::convert_schema(&output))
    }

    #[must_use]
    pub fn session_config(&self) -> std::sync::Arc<pg_client::session::Config> {
        std::sync::Arc::clone(&self.session_config)
    }

    #[must_use]
    pub const fn libpq_config(&self) -> &pg_client::libpq::Config {
        &self.libpq_config
    }

    pub async fn with_session<T>(
        &self,
        action: impl AsyncFnOnce(&mut pg_client::session::Session) -> T,
    ) -> Result<T, Error> {
        Ok(pg_client::session::with_session(self.session_config(), action).await?)
    }

    pub async fn labels(
        &self,
    ) -> Result<ociman::label::ContainerLabels, ociman::label::ContainerError> {
        self.container.labels().await
    }

    pub async fn apply_sql(
        &self,
        sql: &pg_client::protocol::frontend::Sql,
    ) -> Result<(), SqlError> {
        log::debug!("Executing: {}", sql.as_str());

        Ok(
            pg_client::session::with_session(self.session_config(), async |session| {
                session.statements(sql).await
            })
            .await??,
        )
    }

    /// Apply CSV content to a table using PostgreSQL's COPY protocol.
    ///
    /// The line delimiter is hardcoded to `\n`.
    pub async fn apply_csv(
        &self,
        table: &pg_client::QualifiedTable,
        delimiter: char,
        content: &str,
    ) -> Result<(), SqlError> {
        const GENERATE: pg_client::protocol::frontend::Sql =
            pg_client::protocol::frontend::Sql::from_static_or_panic(
                r#"SELECT 'COPY ' || format('%I.%I', $1, $2)
                    || '(' || (SELECT string_agg(format('%I', "column"), ', ') FROM unnest($3::text[]) AS "column") || ')'
                    || ' FROM STDIN WITH (FORMAT csv, HEADER MATCH, DELIMITER ' || quote_literal($4) || ')'"#,
            );

        let header_line = content.lines().next().unwrap_or_default();

        let columns: Vec<&str> = header_line.split(delimiter).map(str::trim).collect();

        pg_client::session::with_session(self.session_config(), async |session| {
            let mut reader = session
                .statement(&GENERATE)
                .bind(table.schema.as_str())
                .bind(table.table.as_str())
                .bind(pg_client::value::Array::from(columns))
                .bind(delimiter.to_string())
                .execute()?;

            let description = reader.describe().await?;
            let statement = description
                .column::<pg_client::protocol::frontend::Sql>(0)
                .map_err(SqlError::CopyStatement)?;
            let statement = reader.decode(|row| row.decode(&statement)).one().await?;

            log::debug!("Executing: {}", statement.as_str());

            let mut copy = session.copy_in(&statement).await?;
            copy.write(content.as_bytes()).await?;
            copy.finish().await?;

            Ok::<(), SqlError>(())
        })
        .await?
    }

    pub(crate) async fn exec_container_script(
        &self,
        script: &str,
    ) -> Result<(), cmd_proc::CommandError> {
        self.container
            .exec("sh")
            .arguments(["-e", "-c", script])
            .to_cmd_proc_command()
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
                config.to_url().parse::<cmd_proc::EnvVariableValue>()?,
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
    /// (`.tty_if_terminal().interactive().status()` vs `.to_cmd_proc_command().stdout_capture()`).
    /// `DATABASE_URL` is intentionally omitted; only `run-env` sets it (its
    /// callee may be arbitrary user code that reads the variable).
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
            .workdir(workdir.as_str())
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
            .to_url()
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
            .to_cmd_proc_command()
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

    fn container_client_config(&self) -> pg_client::libpq::Config {
        let mut config = self.libpq_config.clone();
        if let pg_client::libpq::Endpoint::Network {
            ref host,
            ref channel_binding,
            ref host_addr,
            ..
        } = config.endpoint
        {
            config.endpoint = pg_client::libpq::Endpoint::Network {
                host: host.clone(),
                channel_binding: *channel_binding,
                host_addr: host_addr.clone(),
                port: Some(pg_client::config::Port::new(PG_CONTAINER_PORT)),
            };
        }
        config
    }

    fn container_unix_socket_config(&self) -> pg_client::libpq::Config {
        let mut config = self.libpq_config.clone();
        config.endpoint = pg_client::libpq::Endpoint::SocketPath {
            path: std::path::PathBuf::from("/var/run/postgresql"),
            port: None,
        };
        config.ssl_mode = pg_client::libpq::SslMode::Disable;
        config.ssl_root_cert = None;
        config
    }

    pub async fn cross_container_database_url(&self) -> String {
        // Resolve the container host from inside a container
        // This DNS name only works from inside containers, not from the host
        let ip_address = self
            .backend
            .resolve_container_host()
            .await
            .expect("Failed to resolve container host from container");

        let channel_binding = match &self.libpq_config.endpoint {
            pg_client::libpq::Endpoint::Network {
                channel_binding, ..
            } => *channel_binding,
            pg_client::libpq::Endpoint::SocketPath { .. } => None,
        };

        let endpoint = pg_client::libpq::Endpoint::Network {
            host: pg_client::config::Host::IpAddr(ip_address),
            channel_binding,
            host_addr: None,
            port: Some(self.host_port),
        };

        self.libpq_config.clone().endpoint(endpoint).to_url()
    }

    pub fn pg_env(
        &self,
    ) -> Result<
        std::collections::BTreeMap<cmd_proc::EnvVariableName, cmd_proc::EnvVariableValue>,
        cmd_proc::EnvVariableValueError,
    > {
        self.libpq_config.pg_env()
    }

    #[must_use]
    pub fn database_url(&self) -> String {
        self.libpq_config.to_url()
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.container.stop().await.map_err(Error::ContainerStop)
    }

    async fn terminate_connections(&self) -> Result<(), Error> {
        const TERMINATE: pg_client::protocol::frontend::Sql =
            pg_client::protocol::frontend::Sql::from_static_or_panic(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND backend_type = 'client backend'",
            );

        self.apply_sql(&TERMINATE)
            .await
            .map_err(Error::TerminateConnections)
    }

    async fn checkpoint(&self) -> Result<(), Error> {
        const CHECKPOINT: pg_client::protocol::frontend::Sql =
            pg_client::protocol::frontend::Sql::from_static_or_panic("CHECKPOINT");

        self.apply_sql(&CHECKPOINT).await.map_err(Error::Checkpoint)
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
        self.container
            .commit(reference, false)
            .await
            .map_err(|source| Error::Commit {
                reference: Box::new(reference.clone()),
                source,
            })?;
        self.container
            .remove()
            .await
            .map_err(Error::ContainerRemove)?;
        Ok(())
    }

    async fn wait_for_container_socket(
        &self,
        timeout: crate::definition::Timeout,
    ) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let sleep_duration = std::time::Duration::from_millis(100);

        loop {
            if let crate::definition::Timeout::After(duration) = timeout
                && start.elapsed() > duration
            {
                return Err(Error::ConnectionTimeout {
                    timeout: duration,
                    source: None,
                });
            }

            if self
                .container
                .exec("pg_isready")
                .argument("--host")
                .argument("localhost")
                .to_cmd_proc_command()
                .stdout_capture()
                .bytes()
                .await
                .is_ok()
            {
                return Ok(());
            }
            tokio::time::sleep(sleep_duration).await;
        }
    }

    /// Set the superuser password using peer authentication via Unix domain socket.
    ///
    /// This is useful when resuming from a cached image where the password
    /// doesn't match the newly generated one.
    pub async fn set_superuser_password(
        &mut self,
        password: &pg_client::config::Password,
        timeout: crate::definition::Timeout,
    ) -> Result<(), Error> {
        self.alter_superuser_password(password, timeout).await?;

        self.superuser.password = password.clone();
        self.libpq_config.session.password = Some(password.clone());

        let mut session_config = (*self.session_config).clone();
        session_config.authentication.password =
            pg_client::authentication::Password::Static(password.clone());
        self.session_config = std::sync::Arc::new(session_config);

        Ok(())
    }

    pub(crate) async fn restore_superuser_password(
        &self,
        timeout: crate::definition::Timeout,
    ) -> Result<(), Error> {
        self.alter_superuser_password(&self.superuser.password, timeout)
            .await
    }

    async fn alter_superuser_password(
        &self,
        password: &pg_client::config::Password,
        timeout: crate::definition::Timeout,
    ) -> Result<(), Error> {
        self.wait_for_container_socket(timeout).await?;

        self.container
            .exec("psql")
            .argument("--host")
            .argument("/var/run/postgresql")
            .argument("--username")
            .argument(self.superuser.user.as_ref())
            .argument("--dbname")
            .argument("postgres")
            .argument("--variable")
            .argument(format!("target_role={}", self.superuser.user.as_ref()))
            .argument("--variable")
            .argument(format!("new_password={}", password.as_ref()))
            .stdin("ALTER ROLE :\"target_role\" WITH PASSWORD :'new_password'")
            .to_cmd_proc_command()
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
