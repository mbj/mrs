//! `pg-ephemeral host <subcommand>` — operations executed on the host.
//!
//! Each subcommand runs as a host process with stdio inherited and PG\* /
//! DATABASE_URL pointing at the container's published TCP port. Use these
//! when the tool must read or write host filesystem, or stream binary data
//! through pipes without PTY corruption.

use super::instance;

/// Run an [`instance::Command`] on the host (TCP, host filesystem).
pub struct Command<'a>(pub &'a instance::Command);

impl Command<'_> {
    pub async fn run(
        &self,
        definition: &crate::definition::Definition,
    ) -> Result<(), super::Error> {
        definition
            .with_container(async |container| run_on_container(self.0, container).await)
            .await??;
        Ok(())
    }
}

/// Dispatch an [`instance::Command`] on the host against an
/// already-running container. Shared by [`Command::run`] (ephemeral path:
/// boot, run, stop) and the `session host` attach path.
pub(super) async fn run_on_container(
    command: &instance::Command,
    container: &crate::container::Container,
) -> Result<(), super::Error> {
    match command {
        instance::Command::Psql => host_psql(container).await,
        instance::Command::RunEnv { command, arguments } => {
            host_run_env(container, command, arguments).await
        }
        instance::Command::SchemaDump => host_schema_dump(container).await,
        instance::Command::Shell => host_shell(container).await,
        instance::Command::Pgbench { arguments } => host_pgbench(container, arguments).await,
    }
}

async fn host_pgbench(
    container: &crate::container::Container,
    arguments: &[String],
) -> Result<(), super::Error> {
    cmd_proc::Command::new("pgbench")
        .arguments(arguments)
        .envs(container.pg_env()?)
        .status()
        .await?;
    Ok(())
}

async fn host_psql(container: &crate::container::Container) -> Result<(), super::Error> {
    cmd_proc::Command::new("psql")
        .envs(container.pg_env()?)
        .status()
        .await?;
    Ok(())
}

async fn host_run_env(
    container: &crate::container::Container,
    command: &str,
    arguments: &[String],
) -> Result<(), super::Error> {
    cmd_proc::Command::new(command)
        .arguments(arguments)
        .envs(container.pg_env()?)
        .env(
            &crate::ENV_DATABASE_URL,
            container
                .database_url()
                .parse::<cmd_proc::EnvVariableValue>()?,
        )
        .status()
        .await?;
    Ok(())
}

async fn host_schema_dump(container: &crate::container::Container) -> Result<(), super::Error> {
    let pg_schema_dump = pg_client::PgSchemaDump::new();
    let output = cmd_proc::Command::new("pg_dump")
        .arguments(pg_schema_dump.arguments())
        .envs(container.pg_env()?)
        .stdout_capture()
        .bytes()
        .await?;
    println!("{}", crate::convert_schema(&output));
    Ok(())
}

async fn host_shell(container: &crate::container::Container) -> Result<(), super::Error> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
    cmd_proc::Command::new(shell)
        .envs(container.pg_env()?)
        .env(
            &crate::ENV_DATABASE_URL,
            container
                .database_url()
                .parse::<cmd_proc::EnvVariableValue>()?,
        )
        .status()
        .await?;
    Ok(())
}
