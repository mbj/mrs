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
        match self.0 {
            instance::Command::Psql => {
                definition.with_container(host_psql).await??;
            }
            instance::Command::RunEnv { command, arguments } => {
                definition
                    .with_container(async |container| {
                        host_run_env(container, command, arguments).await
                    })
                    .await??;
            }
            instance::Command::SchemaDump => {
                definition.with_container(host_schema_dump).await??;
            }
            instance::Command::Shell => {
                definition.with_container(host_shell).await??;
            }
            instance::Command::Pgbench { arguments } => {
                definition
                    .with_container(async |container| host_pgbench(container, arguments).await)
                    .await??;
            }
        }
        Ok(())
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
