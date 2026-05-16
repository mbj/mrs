//! `pg-ephemeral meta <subcommand>` — backend introspection.

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Print backend type, version, and rootless status
    Info,
}

impl Command {
    pub async fn run(&self, backend: &ociman::Backend) -> Result<(), super::Error> {
        match self {
            Self::Info => {
                let (kind, version) = match backend {
                    ociman::Backend::Docker { version, .. } => ("docker", version),
                    ociman::Backend::Podman { version, .. } => ("podman", version),
                };
                println!("Backend:  {kind} {version}");
                println!("Rootless: {}", backend.is_rootless());
            }
        }
        Ok(())
    }
}
