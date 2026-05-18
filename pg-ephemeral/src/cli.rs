pub mod cache;
pub mod container;
pub mod host;
pub mod instance;
pub mod meta;
pub mod platform;
pub mod session;
pub mod transparent;

use crate::config::Config;
use crate::seed::SeedName;
use crate::{InstanceMap, InstanceName};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Command(#[from] cmd_proc::CommandError),
    #[error(transparent)]
    Config(#[from] crate::config::Error),
    #[error(transparent)]
    Container(#[from] crate::container::Error),
    #[error(transparent)]
    AttachSession(#[from] crate::container::AttachSessionError),
    #[error(transparent)]
    EnvVariableValue(#[from] cmd_proc::EnvVariableValueError),
    #[error("Unknown instance: {0}")]
    UnknownInstance(InstanceName),
    #[error("Instance {instance} has no seeds; cache credentials requires a cacheable seed")]
    NoSeedsDefined { instance: InstanceName },
    #[error("Instance {instance} has no seed named {seed}")]
    UnknownSeed {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error(
        "Seed {seed} on instance {instance} is uncacheable; cache credentials requires a cacheable seed"
    )]
    SeedUncacheable {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error(
        "Seed {seed} on instance {instance} is not yet cached; run `pg-ephemeral cache populate` first"
    )]
    SeedNotCached {
        instance: InstanceName,
        seed: SeedName,
    },
    #[error("Failed to resolve container backend")]
    BackendResolve(#[source] ociman::backend::resolve::Error),
    #[error(transparent)]
    Session(crate::session::ListError),
    #[error(transparent)]
    SessionFind(crate::session::FindError),
    #[error(transparent)]
    SessionStop(crate::session::StopError),
    #[error(transparent)]
    SessionMetadata(#[from] crate::session::MetadataError),
    #[error("Unknown session: {name}")]
    UnknownSession { name: crate::session::Name },
    #[error("Failed to read current working directory")]
    CurrentDir(#[source] std::io::Error),
    #[error(transparent)]
    TransparentWorkdir(#[from] crate::definition::TransparentWorkdirError),
}

#[derive(Clone, Debug, Default)]
pub enum ConfigFileSource {
    #[default]
    Implicit,
    Explicit(std::path::PathBuf),
    None,
}

impl ConfigFileSource {
    fn from_arguments(config_file: Option<std::path::PathBuf>, no_config_file: bool) -> Self {
        match (config_file, no_config_file) {
            (Some(path), false) => Self::Explicit(path),
            (None, true) => Self::None,
            (None, false) => Self::Implicit,
            (Some(_), true) => unreachable!("clap conflicts_with prevents this"),
        }
    }
}

#[derive(Clone, Debug, clap::Parser)]
#[command(after_help = "EXECUTION CONTEXT:
    Bare commands (psql, run-env, schema-dump, shell) run in transparent mode:
    the current working directory is bind-mounted into the container at the
    same path, the command executes inside the container as the host user,
    and PG* / DATABASE_URL point at the in-container unix socket.

    Use `host <sub>` for an explicit host-side process (TCP to published port),
    or `container <sub>` for an explicit in-container exec without the cwd
    bind mount.

INSTANCE SELECTION:
    All commands target the \"main\" instance by default.
    Use --instance <NAME> to target a different instance.")]
#[command(version = crate::VERSION_STR)]
pub struct App {
    /// Config file to use, defaults to attempt to load database.toml
    ///
    /// If absent on default location a single "main" database is assumed on
    /// autodetected backend with latest postgres and no other configuration.
    #[arg(long, conflicts_with = "no_config_file")]
    config_file: Option<std::path::PathBuf>,
    /// Do not load any config file, use default instance map
    #[arg(long, conflicts_with = "config_file")]
    no_config_file: bool,
    /// Overwrite backend
    ///
    /// If not specified on the CLI and not in the config file will be autodetected:
    /// first based on env variable OCIMAN_BACKEND, then on installed tools.
    /// If the autodetection fails exits with an error.
    #[arg(long)]
    backend: Option<ociman::backend::Selection>,
    /// Overwrite image
    #[arg(long)]
    image: Option<crate::image::Image>,
    /// Enable SSL with the specified hostname
    #[arg(long)]
    ssl_hostname: Option<pg_client::config::HostName>,
    #[clap(subcommand)]
    command: Option<Command>,
}

impl App {
    pub async fn run(&self) -> Result<(), Error> {
        let overwrites = crate::config::InstanceDefinition {
            image: self.image.clone(),
            parameters: pg_client::parameter::Map::new(),
            seeds: indexmap::IndexMap::new(),
            ssl_config: self
                .ssl_hostname
                .clone()
                .map(|hostname| crate::config::SslConfigDefinition { hostname }),
            wait_available_timeout: None,
        };

        let config_file_source =
            ConfigFileSource::from_arguments(self.config_file.clone(), self.no_config_file);

        let resolved = match config_file_source {
            ConfigFileSource::Explicit(config_file) => {
                Config::load_toml_file(&config_file, self.backend, &overwrites)?
            }
            ConfigFileSource::None => {
                log::debug!("--no-config-file specified, using default instance map");
                crate::Config::default().resolve(self.backend, &overwrites)?
            }
            ConfigFileSource::Implicit => {
                log::debug!("No config file specified, trying to load from default location");

                match Config::load_toml_file("database.toml", self.backend, &overwrites) {
                    Ok(value) => value,
                    Err(crate::config::Error::IO(crate::config::IoError(
                        std::io::ErrorKind::NotFound,
                    ))) => {
                        log::debug!(
                            "Config file does not exist in default location, using default instance map"
                        );
                        crate::Config::default().resolve(self.backend, &overwrites)?
                    }
                    Err(error) => return Err(error.into()),
                }
            }
        };

        self.command
            .clone()
            .unwrap_or_default()
            .run(resolved.backend_selection, &resolved.instances)
            .await?;

        Ok(())
    }
}

async fn resolve_backend(selection: ociman::backend::Selection) -> Result<ociman::Backend, Error> {
    selection.resolve().await.map_err(Error::BackendResolve)
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Cache related commands
    Cache {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        #[clap(subcommand)]
        command: cache::Command,
    },
    /// Operations executed inside the running container
    ///
    /// Each subcommand `podman exec`s the target inside the booted
    /// PostgreSQL container: the executable resolves against the image's
    /// $PATH, sees the container filesystem, and connects to PostgreSQL
    /// via the in-container unix socket (`/var/run/postgresql`). Use these
    /// when you need container-side semantics — version-matched `pg_dump`,
    /// scripts that depend on container-installed extensions, etc.
    Container {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        #[clap(subcommand)]
        command: instance::Command,
    },
    /// Operations executed on the host against the running container
    ///
    /// Each subcommand runs as a host process with stdio inherited and
    /// PG* / DATABASE_URL pointing at the container's published TCP port.
    /// Use these when the tool must read or write host filesystem, or
    /// must stream binary data through pipes without PTY corruption.
    Host {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        #[clap(subcommand)]
        command: instance::Command,
    },
    /// Run integration server
    ///
    /// Intent to be used for automation with other languages wrapping pg-ephemeral.
    ///
    /// After successful boot connects to the inherited pipe file descriptors,
    /// writes a single JSON line with connection details to --result-fd,
    /// then waits for EOF on --control-fd before shutting down.
    #[command(name = "integration-server")]
    IntegrationServer {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// File descriptor for writing the result JSON
        #[arg(long)]
        result_fd: std::os::fd::RawFd,
        /// File descriptor for reading the control signal (EOF = shutdown)
        #[arg(long)]
        control_fd: std::os::fd::RawFd,
    },
    /// List defined instances
    List,
    /// Backend introspection (kind, version, rootless status)
    Meta {
        #[clap(subcommand)]
        command: meta::Command,
    },
    /// Platform related commands
    #[command(name = "platform")]
    Platform {
        #[clap(subcommand)]
        command: platform::Command,
    },
    /// Run interactive psql
    Psql {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// Run a command with PostgreSQL connection environment
    #[command(name = "run-env")]
    RunEnv {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// The command to run
        command: String,
        /// Arguments to pass to the command
        arguments: Vec<String>,
    },
    /// Named-session management
    Session {
        #[clap(subcommand)]
        command: session::Command,
    },
    /// Dump schema to stdout
    #[command(name = "schema-dump")]
    SchemaDump {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
    /// Run interactive shell
    Shell {
        /// Target instance name
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Psql {
            instance_name: InstanceName::default(),
        }
    }
}

impl Command {
    pub async fn run(
        &self,
        backend_selection: ociman::backend::Selection,
        instance_map: &InstanceMap,
    ) -> Result<(), Error> {
        match self {
            Self::Cache {
                instance_name,
                command,
            } => {
                let backend = resolve_backend(backend_selection).await?;
                command.run(&backend, instance_map, instance_name).await?
            }
            Self::Container {
                instance_name,
                command,
            } => {
                let backend = resolve_backend(backend_selection).await?;
                let definition =
                    get_instance(instance_map, instance_name)?.definition(backend, instance_name);
                container::Command(command).run(&definition).await?
            }
            Self::Host {
                instance_name,
                command,
            } => {
                let backend = resolve_backend(backend_selection).await?;
                let definition =
                    get_instance(instance_map, instance_name)?.definition(backend, instance_name);
                host::Command(command).run(&definition).await?
            }
            Self::IntegrationServer {
                instance_name,
                result_fd,
                control_fd,
            } => {
                let backend = resolve_backend(backend_selection).await?;
                let definition =
                    get_instance(instance_map, instance_name)?.definition(backend, instance_name);
                definition
                    .run_integration_server(*result_fd, *control_fd)
                    .await?
            }
            Self::List => {
                for instance_name in instance_map.keys() {
                    println!("{instance_name}")
                }
            }
            Self::Meta { command } => {
                let backend = resolve_backend(backend_selection).await?;
                command.run(&backend).await?
            }
            Self::Platform { command } => command.run(),
            Self::Psql { instance_name } => {
                let backend = resolve_backend(backend_selection).await?;
                run_transparent(
                    backend,
                    instance_map,
                    instance_name,
                    &instance::Command::Psql,
                )
                .await?
            }
            Self::RunEnv {
                instance_name,
                command,
                arguments,
            } => {
                let backend = resolve_backend(backend_selection).await?;
                run_transparent(
                    backend,
                    instance_map,
                    instance_name,
                    &instance::Command::RunEnv {
                        command: command.clone(),
                        arguments: arguments.clone(),
                    },
                )
                .await?
            }
            Self::Session { command } => {
                let backend = resolve_backend(backend_selection).await?;
                command.run(&backend, instance_map).await?
            }
            Self::SchemaDump { instance_name } => {
                let backend = resolve_backend(backend_selection).await?;
                run_transparent(
                    backend,
                    instance_map,
                    instance_name,
                    &instance::Command::SchemaDump,
                )
                .await?
            }
            Self::Shell { instance_name } => {
                let backend = resolve_backend(backend_selection).await?;
                run_transparent(
                    backend,
                    instance_map,
                    instance_name,
                    &instance::Command::Shell,
                )
                .await?
            }
        }

        Ok(())
    }
}

async fn run_transparent(
    backend: ociman::Backend,
    instance_map: &InstanceMap,
    instance_name: &InstanceName,
    command: &instance::Command,
) -> Result<(), Error> {
    let cwd = std::env::current_dir().map_err(Error::CurrentDir)?;
    let workdir = crate::definition::TransparentWorkdir::try_from(cwd)?;
    let definition = get_instance(instance_map, instance_name)?
        .definition(backend, instance_name)
        .transparent_workdir(workdir.clone());
    transparent::Command {
        command,
        workdir: &workdir,
    }
    .run(&definition)
    .await
}

#[allow(
    clippy::result_large_err,
    reason = "cli::Error aggregates container/seed errors that intentionally carry diagnostic context; the 128-byte threshold targets async-server hot paths that don't apply here"
)]
pub(super) fn get_instance<'a>(
    instance_map: &'a InstanceMap,
    instance_name: &InstanceName,
) -> Result<&'a crate::config::Instance, Error> {
    instance_map
        .get(instance_name)
        .ok_or_else(|| Error::UnknownInstance(instance_name.clone()))
}
