//! Shared subcommand parser used by both `host` and `container` parents.
//!
//! Each variant describes *what* to do; the parent decides *where* it runs
//! (host process + TCP vs `podman exec` + unix socket). See
//! [`super::host::Command`] and [`super::container::Command`] for the
//! per-context dispatch.

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Run interactive psql
    Psql,
    /// Run a command with PostgreSQL connection environment
    ///
    /// Sets PG\* and DATABASE_URL. On host, runs as a host process with
    /// stdio inherited and the TCP-published endpoint. In the container,
    /// runs via `podman exec` with PTY against the unix socket
    /// (`/var/run/postgresql`); use the host parent for binary stream
    /// piping.
    #[command(name = "run-env")]
    RunEnv {
        /// The command to run
        command: String,
        /// Arguments to pass to the command
        arguments: Vec<String>,
    },
    /// Dump schema (`pg_dump --schema-only`) to stdout
    #[command(name = "schema-dump")]
    SchemaDump,
    /// Run interactive shell with PostgreSQL connection environment
    ///
    /// On host, runs `$SHELL` (falling back to `sh`) with PG\* and
    /// DATABASE_URL set. In the container, runs `sh` via `podman exec`.
    Shell,
    /// Run pgbench against the instance
    ///
    /// Forwards trailing arguments to `pgbench`. Recommended via the
    /// `container` parent (uses the in-container `pgbench` with the local
    /// unix socket — see memory note benchmark_in_container).
    Pgbench {
        /// Arguments forwarded to pgbench
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        arguments: Vec<String>,
    },
}
