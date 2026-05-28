//! `pg-ephemeral cache <subcommand>` — image-cache management.

use crate::seed::{CacheStatus, SeedName};
use crate::{InstanceMap, InstanceName};

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Print cache status for seeds
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove cached images for the instance
    Reset {
        /// Force removal even if images are in use by stopped containers
        #[arg(long)]
        force: bool,
    },
    /// Populate cache by running seeds and committing at each cacheable point
    Populate,
    /// Print full pg-ephemeral metadata for a cached image as JSON
    Inspect {
        /// Image reference, e.g. pg-ephemeral/main:abc123...
        reference: ociman::Reference,
    },
    /// Print connection credentials baked into a cached seed image as JSON.
    ///
    /// Reads the user, database, password, and (when configured) SSL CA cert
    /// from the cached image's labels — no container is booted. Emits no
    /// host/port, since those are runtime artifacts of an actual container,
    /// not properties of the cache image.
    ///
    /// Fails when the targeted seed is uncacheable, when it is cacheable
    /// but not yet built, or when the instance has no seeds at all.
    Credentials {
        /// Seed to read credentials from. Defaults to the last declared seed
        /// in the chain.
        #[arg(long = "seed-name")]
        seed_name: Option<SeedName>,
    },
    /// Pull cache images from the configured registry.
    ///
    /// Walks the seed chain from tip backwards and pulls the newest stage
    /// that exists remotely. Requires `cache_registry` to be set.
    Pull,
    /// Push all locally-cached stages to the configured registry.
    ///
    /// Pushes every stage currently stored locally (status "hit"). Stages
    /// not yet populated locally are skipped. Requires `cache_registry` to
    /// be set.
    Push,
}

impl Command {
    pub async fn run(
        &self,
        backend: &ociman::Backend,
        instance_map: &InstanceMap,
        instance_name: &InstanceName,
    ) -> Result<(), super::Error> {
        let definition = super::get_instance(instance_map, instance_name)?
            .definition(backend.clone(), instance_name);
        match self {
            Self::Status { json } => {
                definition.print_cache_status(instance_name, *json).await?;
            }
            Self::Reset { force } => {
                // Local image names mirror whatever `cache_registry` is
                // set to, so reset must target the same prefixed name —
                // otherwise the unprefixed lookup finds nothing and the
                // prefixed images survive.
                let name = crate::seed::cache_image_name(
                    definition.cache_registry.as_ref(),
                    instance_name,
                );
                let references = definition.backend.image_references_by_name(&name).await;
                for reference in &references {
                    if *force {
                        definition.backend.remove_image_force(reference).await;
                    } else {
                        definition.backend.remove_image(reference).await;
                    }
                    println!("Removed: {reference}");
                }
            }
            Self::Populate => {
                let loaded_seeds = definition
                    .load_seeds(instance_name)
                    .await
                    .map_err(crate::container::Error::from)?;
                definition.populate_cache(&loaded_seeds).await?;
                definition.print_cache_status(instance_name, false).await?;
            }
            Self::Inspect { reference } => {
                let labels =
                    definition
                        .backend
                        .image_labels(reference)
                        .await
                        .map_err(|source| crate::container::Error::InspectImage {
                            reference: reference.clone(),
                            source,
                        })?;
                let metadata =
                    crate::label::read_image(&labels).map_err(crate::container::Error::from)?;
                let json = serde_json::to_string_pretty(&inspect_output(&metadata))
                    .map_err(crate::container::Error::SerializeMetadata)?;
                println!("{json}");
            }
            Self::Credentials { seed_name } => {
                let loaded_seeds = definition
                    .load_seeds(instance_name)
                    .await
                    .map_err(crate::container::Error::from)?;

                let target = match seed_name {
                    Some(name) => loaded_seeds
                        .iter_seeds()
                        .find(|seed| seed.name() == name)
                        .ok_or_else(|| super::Error::UnknownSeed {
                            instance: instance_name.clone(),
                            seed: name.clone(),
                        })?,
                    None => loaded_seeds.iter_seeds().last().ok_or_else(|| {
                        super::Error::NoSeedsDefined {
                            instance: instance_name.clone(),
                        }
                    })?,
                };

                let (reference, labels) = match target.cache_status() {
                    CacheStatus::Hit {
                        reference, labels, ..
                    } => (reference, labels),
                    CacheStatus::Miss { .. } => {
                        return Err(super::Error::SeedNotCached {
                            instance: instance_name.clone(),
                            seed: target.name().clone(),
                        });
                    }
                    CacheStatus::Uncacheable => {
                        return Err(super::Error::SeedUncacheable {
                            instance: instance_name.clone(),
                            seed: target.name().clone(),
                        });
                    }
                };

                let metadata =
                    crate::label::read_image(labels).map_err(crate::container::Error::from)?;
                let json = serde_json::to_string_pretty(&credentials_output(reference, &metadata))
                    .map_err(crate::container::Error::SerializeMetadata)?;
                println!("{json}");
            }
            Self::Pull => {
                definition.pull_cache(instance_name).await?;
            }
            Self::Push => {
                definition.push_cache(instance_name).await?;
            }
        }
        Ok(())
    }
}

fn credentials_output(
    reference: &ociman::Reference,
    metadata: &crate::label::Metadata,
) -> serde_json::Value {
    let mut superuser = serde_json::json!({
        "user": metadata.superuser.user.as_ref(),
        "database": metadata.superuser.database.as_ref(),
        "password": metadata.superuser.password.as_ref(),
    });
    if let Some(application) = metadata.superuser.application.as_ref() {
        superuser["application"] = serde_json::Value::String(application.as_ref().to_string());
    }

    let mut output = serde_json::json!({
        "cache_image": reference.to_string(),
        "superuser": superuser,
    });
    if let Some(ssl) = metadata.ssl.as_ref() {
        output["ssl"] = serde_json::json!({
            "hostname": ssl.hostname.as_str(),
            "ca_cert_pem": ssl.ca_cert_pem,
        });
    }
    output
}

fn inspect_output(metadata: &crate::label::Metadata) -> serde_json::Value {
    let mut superuser = serde_json::Map::new();
    superuser.insert(
        "user".to_string(),
        serde_json::Value::String(metadata.superuser.user.as_ref().to_string()),
    );
    superuser.insert(
        "database".to_string(),
        serde_json::Value::String(metadata.superuser.database.as_ref().to_string()),
    );
    superuser.insert(
        "password".to_string(),
        serde_json::Value::String(metadata.superuser.password.as_ref().to_string()),
    );
    if let Some(application) = metadata.superuser.application.as_ref() {
        superuser.insert(
            "application".to_string(),
            serde_json::Value::String(application.as_ref().to_string()),
        );
    }

    let mut output = serde_json::Map::new();
    output.insert(
        "version".to_string(),
        serde_json::Value::String(metadata.version.to_string()),
    );
    output.insert(
        "instance".to_string(),
        serde_json::Value::String(metadata.instance.as_ref().to_string()),
    );
    output.insert(
        "image".to_string(),
        serde_json::Value::String(metadata.image.to_string()),
    );
    output.insert(
        "superuser".to_string(),
        serde_json::Value::Object(superuser),
    );
    output.insert(
        "seeds".to_string(),
        serde_json::to_value(&metadata.seeds).unwrap(),
    );
    if let Some(ssl) = metadata.ssl.as_ref() {
        let mut ssl_map = serde_json::Map::new();
        ssl_map.insert(
            "hostname".to_string(),
            serde_json::Value::String(ssl.hostname.as_str().to_string()),
        );
        ssl_map.insert(
            "ca_cert_pem".to_string(),
            serde_json::Value::String(ssl.ca_cert_pem.clone()),
        );
        output.insert("ssl".to_string(), serde_json::Value::Object(ssl_map));
    }

    serde_json::Value::Object(output)
}
