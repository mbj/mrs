//! `pg-ephemeral <subcommand>` (bare top-level) — transparent mode.
//!
//! "Transparent" because the container is invisible to the user: their cwd
//! is bind-mounted at the same path inside the container, the exec'd
//! process runs with file ownership that matches the host user, and PG\* /
//! DATABASE_URL point at the in-container unix socket. The effect is that
//! `psql`, `pg_dump`, etc. behave as if locally installed but use the
//! container's tooling and connect to the ephemeral instance.
//!
//! The `--user UID:GID` value chosen per backend rootlessness so bind-mount
//! writes come back owned by the host user on all platforms; see
//! [`crate::container::Container::transparent_user`] for the rationale.

use super::instance;
use crate::definition::TransparentWorkdir;

/// Run an [`instance::Command`] in transparent mode (bind-mounted cwd,
/// host-uid-equivalent file ownership, in-container unix socket).
pub struct Command<'a> {
    pub command: &'a instance::Command,
    pub workdir: &'a TransparentWorkdir,
}

impl Command<'_> {
    pub async fn run(
        &self,
        definition: &crate::definition::Definition,
    ) -> Result<(), super::Error> {
        match self.command {
            instance::Command::Psql => {
                let workdir = self.workdir.clone();
                definition
                    .with_container(async |container| {
                        container.exec_transparent_psql(&workdir).await
                    })
                    .await??;
            }
            instance::Command::RunEnv { command, arguments } => {
                let workdir = self.workdir.clone();
                definition
                    .with_container(async |container| {
                        container
                            .exec_transparent_run_env(&workdir, command, arguments)
                            .await
                    })
                    .await??;
            }
            instance::Command::SchemaDump => {
                let workdir = self.workdir.clone();
                let output = definition
                    .with_container(async |container| {
                        container
                            .exec_transparent_schema_dump(&workdir, &pg_client::PgSchemaDump::new())
                            .await
                    })
                    .await??;
                println!("{output}");
            }
            instance::Command::Shell => {
                let workdir = self.workdir.clone();
                definition
                    .with_container(async |container| {
                        container.exec_transparent_shell(&workdir).await
                    })
                    .await??;
            }
            instance::Command::Pgbench { arguments } => {
                let workdir = self.workdir.clone();
                definition
                    .with_container(async |container| {
                        container
                            .exec_transparent_pgbench(&workdir, arguments)
                            .await
                    })
                    .await??;
            }
        }
        Ok(())
    }
}
