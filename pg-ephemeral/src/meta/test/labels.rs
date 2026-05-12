//! Verifies pg-ephemeral metadata labels round-trip on both running
//! containers and committed cache images.

use libtest_mimic::{Failed, Trial};

use crate::label;

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "labels_written_minimal_container",
            labels_written_minimal_container,
        ),
        Trial::test(
            "cache_image_raw_labels_present",
            cache_image_raw_labels_present,
        ),
        Trial::test(
            "cache_image_metadata_round_trip",
            cache_image_metadata_round_trip,
        ),
    ]
}

fn labels_written_minimal_container() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let instance_name: crate::InstanceName = "test-labels".parse().unwrap();

        let definition =
            crate::Definition::new(backend, crate::Image::default(), instance_name.clone())
                .wait_available_timeout(std::time::Duration::from_secs(30));

        definition
            .with_container(async |container| {
                let labels = container.labels().await.unwrap();

                for key in [
                    &label::VERSION_KEY,
                    &label::INSTANCE_KEY,
                    &label::IMAGE_KEY,
                    &label::SUPERUSER_USER_KEY,
                    &label::SUPERUSER_DATABASE_KEY,
                    &label::SUPERUSER_PASSWORD_KEY,
                    &label::SEEDS_KEY,
                ] {
                    assert!(
                        labels.contains_key(key),
                        "expected label {key} to be present",
                    );
                }

                for key in [
                    &label::SUPERUSER_APPLICATION_KEY,
                    &label::SSL_HOSTNAME_KEY,
                    &label::SSL_CA_CERT_PEM_KEY,
                ] {
                    assert!(
                        !labels.contains_key(key),
                        "label {key} should not be present in minimal Definition",
                    );
                }

                assert_eq!(
                    labels.get(&label::VERSION_KEY).unwrap().as_str(),
                    crate::version().to_string(),
                );
                assert_eq!(
                    labels.get(&label::INSTANCE_KEY).unwrap().as_str(),
                    instance_name.as_str(),
                );
                assert_eq!(
                    labels.get(&label::SUPERUSER_USER_KEY).unwrap().as_str(),
                    pg_client::User::POSTGRES.as_ref(),
                );
                assert_eq!(
                    labels.get(&label::SUPERUSER_DATABASE_KEY).unwrap().as_str(),
                    pg_client::Database::POSTGRES.as_ref(),
                );

                let stored_password = labels.get(&label::SUPERUSER_PASSWORD_KEY).unwrap().as_str();
                let actual_password = container
                    .client_config()
                    .session
                    .password
                    .as_ref()
                    .unwrap()
                    .as_ref();
                assert_eq!(stored_password, actual_password);

                let seeds_json = labels.get(&label::SEEDS_KEY).unwrap().as_str();
                let seed_entries: Vec<label::SeedEntry> = serde_json::from_str(seeds_json).unwrap();
                assert!(seed_entries.is_empty());
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn cache_image_raw_labels_present() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let instance_name: crate::InstanceName = "labels-cache-image-raw".parse().unwrap();
        let cache_reference = populate_cache_image(&backend, &instance_name).await;

        let labels = backend.image_labels(&cache_reference).await.unwrap();

        for key in [
            &label::VERSION_KEY,
            &label::INSTANCE_KEY,
            &label::IMAGE_KEY,
            &label::SUPERUSER_USER_KEY,
            &label::SUPERUSER_DATABASE_KEY,
            &label::SUPERUSER_PASSWORD_KEY,
            &label::SEEDS_KEY,
        ] {
            assert!(
                labels.contains_key(key),
                "expected label {key} on cache image",
            );
        }

        for key in [
            &label::SUPERUSER_APPLICATION_KEY,
            &label::SSL_HOSTNAME_KEY,
            &label::SSL_CA_CERT_PEM_KEY,
        ] {
            assert!(
                !labels.contains_key(key),
                "label {key} should not be present on minimal cache image",
            );
        }

        assert_eq!(
            labels.get(&label::VERSION_KEY).unwrap().as_str(),
            crate::version().to_string(),
        );
        assert_eq!(
            labels.get(&label::INSTANCE_KEY).unwrap().as_str(),
            instance_name.as_str(),
        );

        let seeds_json = labels.get(&label::SEEDS_KEY).unwrap().as_str();
        let seed_entries: Vec<label::SeedEntry> = serde_json::from_str(seeds_json).unwrap();
        assert_eq!(seed_entries.len(), 1);
        assert_eq!(seed_entries[0].name.as_str(), "create-table");
        assert!(
            seed_entries[0].hash.is_some(),
            "cacheable seed should carry a hash on the cache image"
        );

        backend.remove_image_force(&cache_reference).await;

        Ok(())
    })
}

fn cache_image_metadata_round_trip() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let instance_name: crate::InstanceName = "labels-cache-image-decoded".parse().unwrap();
        let cache_reference = populate_cache_image(&backend, &instance_name).await;

        let labels = backend.image_labels(&cache_reference).await.unwrap();
        let metadata = label::read_image(&labels).unwrap();

        assert_eq!(&metadata.version, crate::version());
        assert_eq!(metadata.instance, instance_name);
        assert_eq!(metadata.superuser.user, pg_client::User::POSTGRES);
        assert_eq!(metadata.superuser.database, pg_client::Database::POSTGRES);
        assert!(metadata.superuser.application.is_none());
        assert!(metadata.ssl.is_none());

        assert_eq!(metadata.seeds.len(), 1);
        assert_eq!(metadata.seeds[0].name.as_str(), "create-table");
        assert!(
            metadata.seeds[0].hash.is_some(),
            "cacheable seed should carry a hash on the cache image"
        );

        backend.remove_image_force(&cache_reference).await;

        Ok(())
    })
}

/// Builds a pg-ephemeral cache image so the raw-label and decoded-metadata
/// trials below each verify a fresh image. `instance_name` doubles as the
/// cache image namespace; callers pass distinct values to keep parallel
/// trials from racing on a shared image reference.
async fn populate_cache_image(
    backend: &ociman::Backend,
    instance_name: &crate::InstanceName,
) -> ociman::Reference {
    let definition = crate::Definition::new(
        backend.clone(),
        crate::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .apply_script(
        "create-table".parse().unwrap(),
        r#"psql -c "CREATE TABLE labelled (id INT)""#,
        crate::SeedCacheConfig::CommandHash,
    )
    .unwrap();

    let loaded_seeds = definition.load_seeds(instance_name).await.unwrap();
    let (last_cache_hit, uncached) = definition.populate_cache(&loaded_seeds).await.unwrap();
    assert!(uncached.is_empty(), "all seeds should be cacheable");
    last_cache_hit.unwrap()
}
