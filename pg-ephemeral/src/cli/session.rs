//! `pg-ephemeral session <subcommand>` — operations on named, detached
//! pg-ephemeral containers.
//!
//! Lifecycle: `list`, `start`, `stop`. Attached forms: `psql`, `shell`,
//! `run-env`, `schema-dump`, `pgbench` — these mirror the top-level bare
//! commands but target an existing session container by `--name`, exec'ing
//! against it in transparent mode (cwd bind-mounted, host-UID, unix socket).

use super::container;
use super::host;
use super::instance;
use super::transparent;
use crate::definition::TransparentWorkdir;
use crate::session::{self, Session};
use crate::{InstanceMap, InstanceName};

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// List running named sessions
    List,
    /// Start a new named, detached session
    ///
    /// Seeds and credentials are taken from the given instance config; the
    /// resulting container is named `pg-ephemeral-session-<NAME>` and
    /// survives until explicitly stopped.
    Start {
        /// Instance to source seeds, image, and SSL config from
        #[arg(long = "instance", default_value_t)]
        instance_name: InstanceName,
        /// User-facing session name (the OCI container is named
        /// `pg-ephemeral-session-<NAME>`)
        #[arg(long = "name")]
        name: session::Name,
    },
    /// Stop and remove a named session
    Stop {
        /// User-facing session name to stop
        #[arg(long = "name")]
        name: session::Name,
    },
    /// Attach interactive psql to a named session (transparent mode)
    Psql {
        #[arg(long = "name")]
        name: session::Name,
    },
    /// Attach an interactive shell to a named session with PG\* /
    /// DATABASE_URL pointing at the in-container unix socket
    Shell {
        #[arg(long = "name")]
        name: session::Name,
    },
    /// Run a command in a named session with PostgreSQL connection
    /// environment (PG\* + DATABASE_URL)
    #[command(name = "run-env")]
    RunEnv {
        #[arg(long = "name")]
        name: session::Name,
        /// The command to run
        command: String,
        /// Arguments to pass to the command
        arguments: Vec<String>,
    },
    /// Dump schema (`pg_dump --schema-only`) from a named session to stdout
    #[command(name = "schema-dump")]
    SchemaDump {
        #[arg(long = "name")]
        name: session::Name,
    },
    /// Run pgbench against a named session
    Pgbench {
        #[arg(long = "name")]
        name: session::Name,
        /// Arguments forwarded to pgbench
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        arguments: Vec<String>,
    },
    /// Run a subcommand on the host against a named session: host
    /// process with PG* / DATABASE_URL pointing at the container's
    /// published TCP port
    Host {
        #[arg(long = "name")]
        name: session::Name,
        #[command(subcommand)]
        command: instance::Command,
    },
    /// Run a subcommand inside a named session via `podman exec`:
    /// container filesystem, in-container unix socket
    /// (`/var/run/postgresql`)
    Container {
        #[arg(long = "name")]
        name: session::Name,
        #[command(subcommand)]
        command: instance::Command,
    },
    /// Report seed-chain status of running sessions against the
    /// current instance config (each session's stored seed-hash chain
    /// is compared to what the current config would produce today;
    /// reported as `sync` or `diverged`)
    Status {
        /// Restrict to a single session by name; defaults to all
        /// running sessions
        #[arg(long = "name")]
        name: Option<session::Name>,
    },
}

impl Command {
    pub async fn run(
        &self,
        backend: &ociman::Backend,
        instance_map: &InstanceMap,
    ) -> Result<(), super::Error> {
        match self {
            Self::List => {
                let sessions = Session::list(backend)
                    .await
                    .map_err(super::Error::Session)?;
                for session in sessions {
                    println!("{}", session.name());
                }
            }
            Self::Start {
                instance_name,
                name,
            } => {
                // Bake a bind mount of the start-time cwd into the session
                // container so attached commands (which run transparent) can
                // chdir into it. Mounts are immutable post-start, so this
                // path is fixed for the session's lifetime; attaches from a
                // cwd outside (or unrelated to) this tree will fail at the
                // chdir layer.
                let cwd = std::env::current_dir().map_err(super::Error::CurrentDir)?;
                let workdir = TransparentWorkdir::try_from(cwd)?;
                let definition = super::get_instance(instance_map, instance_name)?
                    .definition(backend.clone(), instance_name)
                    .session_name(name.clone())
                    .transparent_workdir(workdir)
                    .remove(false);
                definition.start().await?;
                println!("Started session: {name}");
            }
            Self::Stop { name } => {
                let session = Session::find(backend, name)
                    .await
                    .map_err(super::Error::SessionFind)?
                    .ok_or_else(|| super::Error::UnknownSession { name: name.clone() })?;
                session.stop().await.map_err(super::Error::SessionStop)?;
                println!("Stopped session: {name}");
            }
            Self::Psql { name } => {
                run_attached(backend, name, &instance::Command::Psql).await?;
            }
            Self::Shell { name } => {
                run_attached(backend, name, &instance::Command::Shell).await?;
            }
            Self::RunEnv {
                name,
                command,
                arguments,
            } => {
                run_attached(
                    backend,
                    name,
                    &instance::Command::RunEnv {
                        command: command.clone(),
                        arguments: arguments.clone(),
                    },
                )
                .await?;
            }
            Self::SchemaDump { name } => {
                run_attached(backend, name, &instance::Command::SchemaDump).await?;
            }
            Self::Pgbench { name, arguments } => {
                run_attached(
                    backend,
                    name,
                    &instance::Command::Pgbench {
                        arguments: arguments.clone(),
                    },
                )
                .await?;
            }
            Self::Host { name, command } => {
                let container = attach(backend, name).await?;
                host::run_on_container(command, &container).await?;
            }
            Self::Container { name, command } => {
                let container = attach(backend, name).await?;
                container::run_on_container(command, &container).await?;
            }
            Self::Status { name } => {
                run_status(backend, instance_map, name.as_ref()).await?;
            }
        }
        Ok(())
    }
}

/// `pg-ephemeral session status [--name NAME]` — render each running
/// session's seed-chain status in a comfy-table grid (or vertically
/// for a single target).
///
/// The library's [`session::SeedStatus`] is binary (`sync` /
/// `diverged`); when the session's instance label no longer resolves
/// in the current config the CLI short-circuits to a synthesized
/// `"unknown-instance"` string rather than calling `compute_seed_status`.
async fn run_status(
    backend: &ociman::Backend,
    instance_map: &InstanceMap,
    name: Option<&session::Name>,
) -> Result<(), super::Error> {
    let sessions = match name {
        Some(name) => vec![
            Session::find(backend, name)
                .await
                .map_err(super::Error::SessionFind)?
                .ok_or_else(|| super::Error::UnknownSession { name: name.clone() })?,
        ],
        None => Session::list(backend)
            .await
            .map_err(super::Error::Session)?,
    };

    let mut rows = Vec::with_capacity(sessions.len());
    for session in &sessions {
        let seed_status = seed_status_for(backend, instance_map, session).await?;
        rows.push((session.name().to_string(), seed_status));
    }

    if name.is_some() {
        let (session_name, seed_status) = &rows[0];
        println!("{session_name}");
        println!("  seed: {seed_status}");
    } else {
        let mut table = comfy_table::Table::new();
        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_header(["NAME", "SEED"]);
        for (session_name, seed_status) in &rows {
            table.add_row([session_name.as_str(), seed_status.as_str()]);
        }
        println!("{table}");
    }
    Ok(())
}

/// Orchestrate the per-session seed-status query: decode metadata via
/// [`Session::metadata`], either short-circuit to `"unknown-instance"`
/// or build a Definition and dispatch to [`session::compute_seed_status`].
async fn seed_status_for(
    backend: &ociman::Backend,
    instance_map: &InstanceMap,
    session: &Session,
) -> Result<String, super::Error> {
    let metadata = session.metadata().await?;
    let Some(instance) = instance_map.get(&metadata.instance) else {
        return Ok("unknown-instance".to_string());
    };
    let definition = instance.definition(backend.clone(), &metadata.instance);
    let current_seeds = definition
        .load_seeds(&metadata.instance)
        .await
        .map_err(crate::container::Error::from)?;
    Ok(session::compute_seed_status(
        &metadata.image,
        &metadata.seeds,
        &definition.image,
        &current_seeds,
    )
    .to_string())
}

/// Resolve a session by name and reconstruct a [`crate::Container`] view
/// of its running container.
async fn attach(
    backend: &ociman::Backend,
    name: &session::Name,
) -> Result<crate::Container, super::Error> {
    let session = Session::find(backend, name)
        .await
        .map_err(super::Error::SessionFind)?
        .ok_or_else(|| super::Error::UnknownSession { name: name.clone() })?;
    Ok(crate::Container::attach_session(session, backend.clone()).await?)
}

/// Attach to a named session and dispatch an [`instance::Command`] in
/// transparent mode against the attach-time cwd. The cwd must lie under
/// a path the session container already bind-mounts; attaches from a
/// cwd outside the session's bound tree fail at the chdir layer.
async fn run_attached(
    backend: &ociman::Backend,
    name: &session::Name,
    command: &instance::Command,
) -> Result<(), super::Error> {
    let container = attach(backend, name).await?;
    let cwd = std::env::current_dir().map_err(super::Error::CurrentDir)?;
    let workdir = TransparentWorkdir::try_from(cwd)?;
    transparent::run_on_container(command, &container, &workdir).await
}
