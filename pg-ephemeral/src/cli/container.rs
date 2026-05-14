//! `pg-ephemeral container <subcommand>` — operations executed inside the
//! running container.
//!
//! Each subcommand `podman exec`s the target inside the booted PostgreSQL
//! container: the executable resolves against the image's `$PATH`, sees the
//! container filesystem, and connects to PostgreSQL via the in-container
//! unix socket (`/var/run/postgresql`).

use super::instance;

/// Run an [`instance::Command`] inside the running container
/// (unix socket, container filesystem).
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

/// Dispatch an [`instance::Command`] inside the running container via
/// `podman exec` against an already-running container. Shared by
/// [`Command::run`] (ephemeral path: boot, run, stop) and the
/// `session container` attach path.
pub(super) async fn run_on_container(
    command: &instance::Command,
    container: &crate::container::Container,
) -> Result<(), super::Error> {
    match command {
        instance::Command::Psql => container_psql(container).await,
        instance::Command::RunEnv { command, arguments } => {
            container.exec_run_env(command, arguments).await?;
            Ok(())
        }
        instance::Command::SchemaDump => container_schema_dump(container).await,
        instance::Command::Shell => container_shell(container).await,
        instance::Command::Pgbench { arguments } => {
            container.exec_pgbench(arguments).await?;
            Ok(())
        }
    }
}

async fn container_psql(container: &crate::container::Container) -> Result<(), super::Error> {
    container.exec_psql().await?;
    Ok(())
}

async fn container_schema_dump(
    container: &crate::container::Container,
) -> Result<(), super::Error> {
    let pg_schema_dump = pg_client::PgSchemaDump::new();
    println!("{}", container.exec_schema_dump(&pg_schema_dump).await?);
    Ok(())
}

async fn container_shell(container: &crate::container::Container) -> Result<(), super::Error> {
    container.exec_container_shell().await?;
    Ok(())
}
