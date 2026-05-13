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
        match self.0 {
            instance::Command::Psql => {
                definition.with_container(container_psql).await??;
            }
            instance::Command::RunEnv { command, arguments } => {
                definition
                    .with_container(async |container| {
                        container.exec_run_env(command, arguments).await
                    })
                    .await??;
            }
            instance::Command::SchemaDump => {
                definition.with_container(container_schema_dump).await??;
            }
            instance::Command::Shell => {
                definition.with_container(container_shell).await??;
            }
            instance::Command::Pgbench { arguments } => {
                definition
                    .with_container(async |container| container.exec_pgbench(arguments).await)
                    .await??;
            }
        }
        Ok(())
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
