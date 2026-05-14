mod common;

use pg_ephemeral::label;

#[tokio::test]
async fn test_labels_written_minimal_container() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "test-labels".parse().unwrap();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let labels = container.labels().await.unwrap();

            // Always-present keys.
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

            // Conditional keys NOT set in this minimal Definition.
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

            // Spot-check values.
            assert_eq!(
                labels.get(&label::VERSION_KEY).unwrap().as_str(),
                pg_ephemeral::version().to_string(),
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

            // Password label matches the Container's actual configured password.
            let stored_password = labels.get(&label::SUPERUSER_PASSWORD_KEY).unwrap().as_str();
            let actual_password = container
                .client_config()
                .session
                .password
                .as_ref()
                .unwrap()
                .as_ref();
            assert_eq!(stored_password, actual_password);

            // Seeds label parses back to an empty list (no seeds in this Definition).
            let seeds_json = labels.get(&label::SEEDS_KEY).unwrap().as_str();
            let seed_entries: Vec<label::SeedEntry> = serde_json::from_str(seeds_json).unwrap();
            assert!(seed_entries.is_empty());
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_session_name_writes_label() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-session-name".parse().unwrap();

    // Unique per-run session name so parallel test processes don't collide
    // on the derived OCI container name.
    let raw_session_name = format!("test-{}", ociman::testing::run_id());
    let session_name: pg_ephemeral::session::Name = raw_session_name.parse().unwrap();

    // Clone for use inside the closure: the Definition consumes one backend
    // by value, but we also call Session::list which needs its own handle.
    let backend_for_list = backend.clone();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .session_name(session_name.clone())
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            // (1) Label is on the running container.
            let labels = container.labels().await.unwrap();
            let stored = labels
                .get(&label::SESSION_KEY)
                .expect("session container should carry SESSION_KEY label");
            assert_eq!(stored.as_str(), session_name.as_str());

            // (2) Session::list discovers it via the SESSION_KEY filter and
            // decodes its name. Parallel test processes may also have
            // sessions running, so we filter the list by our unique name
            // rather than asserting on the full vector.
            let sessions = pg_ephemeral::session::Session::list(&backend_for_list)
                .await
                .unwrap();
            let found = sessions
                .iter()
                .find(|session| session.name() == &session_name)
                .expect("Session::list should return our running session");
            assert_eq!(found.name().as_str(), session_name.as_str());
            assert_eq!(
                found.name().container_name().as_str(),
                session_name.container_name().as_str(),
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_no_session_name_omits_label() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-no-session".parse().unwrap();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let labels = container.labels().await.unwrap();
            assert!(
                !labels.contains_key(&label::SESSION_KEY),
                "anonymous (non-session) container should not carry SESSION_KEY",
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_container_metadata_round_trip() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-container-round-trip".parse().unwrap();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let labels = container.labels().await.unwrap();
            let metadata = label::read_container(&labels).unwrap();

            assert_eq!(&metadata.version, pg_ephemeral::version());
            assert_eq!(metadata.instance, instance_name);
            assert_eq!(metadata.superuser.user, pg_client::User::POSTGRES);
            assert_eq!(metadata.superuser.database, pg_client::Database::POSTGRES);
            assert!(metadata.superuser.application.is_none());
            assert!(metadata.ssl.is_none());
            assert!(metadata.seeds.is_empty());

            // Decoded password matches the Container's actual configured one.
            assert_eq!(
                metadata.superuser.password.as_str(),
                container
                    .client_config()
                    .session
                    .password
                    .as_ref()
                    .unwrap()
                    .as_str(),
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_prepare_config_round_trip_ssl() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-prepare-config-ssl".parse().unwrap();
    let hostname: pg_client::config::HostName = "postgresql.example.com".parse().unwrap();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
        hostname: hostname.clone(),
    })
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let labels = container.labels().await.unwrap();
            let metadata = label::read_container(&labels).unwrap();

            assert_eq!(
                metadata.ssl.as_ref().unwrap().hostname,
                hostname,
                "metadata SSL hostname should round-trip from the Definition",
            );

            // Extract the runtime-allocated host port from the container's
            // own client_config rather than re-inspecting.
            let port = match &container.client_config().endpoint {
                pg_client::config::Endpoint::Network {
                    port: Some(port), ..
                } => *port,
                other => panic!("expected network endpoint, got {other:?}"),
            };

            let config = metadata
                .prepare_config(
                    pg_client::config::Host::HostName(hostname.clone()),
                    Some(pg_client::config::HostAddr::new(
                        std::net::Ipv4Addr::LOCALHOST.into(),
                    )),
                    port,
                )
                .unwrap();

            // Connect using the rebuilt config and round-trip a query.
            config
                .with_sqlx_connection(async |connection| {
                    let row = sqlx::query("SELECT 1").fetch_one(connection).await.unwrap();
                    let value: i32 = sqlx::Row::get(&row, 0);
                    assert_eq!(value, 1);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

/// Builds a pg-ephemeral cache image for the given backend so the raw-label
/// and decoded-metadata tests below can each verify a fresh image.
///
/// `instance_name` doubles as the cache image namespace, so each caller passes
/// a distinct value to keep parallel test runs from racing on a shared image
/// reference.
async fn populate_cache_image(
    backend: &ociman::Backend,
    instance_name: &pg_ephemeral::InstanceName,
) -> ociman::Reference {
    let definition = pg_ephemeral::Definition::new(
        backend.clone(),
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .apply_script(
        "create-table".parse().unwrap(),
        r#"psql -c "CREATE TABLE labelled (id INT)""#,
        pg_ephemeral::SeedCacheConfig::CommandHash,
    )
    .unwrap();

    let loaded_seeds = definition.load_seeds(instance_name).await.unwrap();
    let (last_cache_hit, uncached) = definition.populate_cache(&loaded_seeds).await.unwrap();
    assert!(uncached.is_empty(), "all seeds should be cacheable");
    last_cache_hit.expect("populate_cache should produce a cache image")
}

#[tokio::test]
async fn test_cache_image_raw_labels_present() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-cache-image-raw".parse().unwrap();
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
        pg_ephemeral::version().to_string(),
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
}

#[tokio::test]
async fn test_cache_image_metadata_round_trip() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "labels-cache-image-decoded".parse().unwrap();
    let cache_reference = populate_cache_image(&backend, &instance_name).await;

    let labels = backend.image_labels(&cache_reference).await.unwrap();
    let metadata = label::read_image(&labels).unwrap();

    assert_eq!(&metadata.version, pg_ephemeral::version());
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
}
