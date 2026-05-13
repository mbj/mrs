//! `pg-ephemeral meta <subcommand>` — backend introspection.

use crate::{InstanceMap, InstanceName};

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Print backend type, version, and rootless status
    Info,
}

impl Command {
    pub async fn run(
        &self,
        instance_map: &InstanceMap,
        instance_name: &InstanceName,
    ) -> Result<(), super::Error> {
        match self {
            Self::Info => {
                let definition = super::get_instance(instance_map, instance_name)?
                    .definition(instance_name)
                    .await
                    .map_err(|source| super::Error::BackendResolve {
                        instance: instance_name.clone(),
                        source,
                    })?;

                let backend = &definition.backend;
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
