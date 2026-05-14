mod common;

use pg_ephemeral::session::{self, Session};

/// Spawn a minimal alpine container labeled as a pg-ephemeral session,
/// bypassing the full pg-ephemeral Definition (which doesn't yet expose a
/// detached-start entry point — see Layer 3 plan). The label is what the
/// `session` module discovers on.
async fn spawn_session_container(
    backend: &ociman::Backend,
    name: &session::Name,
) -> ociman::Container {
    let value: ociman::label::Value = name.as_str().to_string().try_into().unwrap();
    let definition = ociman::Definition::new(
        backend.clone(),
        ociman::testing::ALPINE_LATEST_IMAGE.clone(),
    )
    .container_name(name.container_name())
    .entrypoint("sh")
    .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"])
    .label(&pg_ephemeral::label::SESSION_KEY, &value);

    definition.run_detached().await.unwrap()
}

#[tokio::test]
async fn test_session_find_then_stop() {
    let backend = ociman::test_backend_setup!();

    let raw_name = format!("session-find-stop-{}", ociman::testing::run_id());
    let name: session::Name = raw_name.parse().unwrap();

    let _container = spawn_session_container(&backend, &name).await;

    // (1) find returns Some with the expected name.
    let found = Session::find(&backend, &name)
        .await
        .unwrap()
        .expect("Session::find should return the running session");
    assert_eq!(found.name(), &name);

    // (2) stop force-removes the container.
    found.stop().await.unwrap();

    // (3) find now returns None.
    assert!(
        Session::find(&backend, &name).await.unwrap().is_none(),
        "Session::find should return None after stop",
    );
}

#[tokio::test]
async fn test_attach_session_yields_connectable_container() {
    if ociman::testing::platform_not_supported() {
        return;
    }

    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "attach-session-test".parse().unwrap();

    let raw_session_name = format!("attach-{}", ociman::testing::run_id());
    let session_name: pg_ephemeral::session::Name = raw_session_name.parse().unwrap();

    // Start a real pg-ephemeral session container that survives until stopped.
    let definition = common::test_definition(backend.clone(), instance_name)
        .session_name(session_name.clone())
        .remove(false);
    definition.start().await.unwrap();

    // Look it up and attach.
    let session = Session::find(&backend, &session_name)
        .await
        .unwrap()
        .expect("started session should be findable");

    let attached = pg_ephemeral::Container::attach_session(session, backend.clone())
        .await
        .unwrap();

    // The decoded superuser password + host port must be usable end-to-end:
    // opening a real sqlx connection proves attach_session reconstructed the
    // pg_client::Config equivalently to run_definition.
    attached
        .client_config()
        .with_sqlx_connection(async |_| {})
        .await
        .unwrap();

    // Cleanup: re-find (attach_session consumed the original Session) and stop.
    Session::find(&backend, &session_name)
        .await
        .unwrap()
        .expect("session should still exist before cleanup")
        .stop()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_session_find_absent() {
    let backend = ociman::test_backend_setup!();

    let raw_name = format!("session-absent-{}", ociman::testing::run_id());
    let name: session::Name = raw_name.parse().unwrap();

    // Nothing spawned — find returns None.
    assert!(Session::find(&backend, &name).await.unwrap().is_none());
}

#[tokio::test]
async fn test_compute_seed_status() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName =
        format!("seed-status-{}", ociman::testing::run_id())
            .parse()
            .unwrap();

    let image: pg_ephemeral::Image = "17".parse().unwrap();
    let definition =
        pg_ephemeral::Definition::new(backend.clone(), image.clone(), instance_name.clone())
            .apply_sql_statement(
                "schema".parse().unwrap(),
                "CREATE TABLE users (id INTEGER PRIMARY KEY)",
            )
            .unwrap()
            .apply_sql_statement("data".parse().unwrap(), "INSERT INTO users VALUES (1)")
            .unwrap();

    let loaded = definition.load_seeds(&instance_name).await.unwrap();
    let stored_image: ociman::image::Reference = (&image).into();

    // Build the SeedEntry vec that would have been written to the
    // session's labels at start time. compute_seed_status reads only
    // name + hash from each entry, so the `config` field is filled with
    // a placeholder.
    let stored: Vec<pg_ephemeral::label::SeedEntry> = loaded
        .iter_seeds()
        .map(|seed| pg_ephemeral::label::SeedEntry {
            name: seed.name().clone(),
            config: pg_ephemeral::config::SeedConfig::SqlStatement {
                statement: String::new(),
            },
            hash: seed.cache_status().hash().cloned(),
        })
        .collect();

    // (1) Identical chain → Sync.
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(&stored_image, &stored, &image, &loaded),
        pg_ephemeral::session::SeedStatus::Sync,
    );

    // (2) Different base image → Diverged.
    let other_image_reference: ociman::image::Reference =
        "docker.io/library/postgres:18".parse().unwrap();
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(
            &other_image_reference,
            &stored,
            &image,
            &loaded
        ),
        pg_ephemeral::session::SeedStatus::Diverged,
    );

    // (3) Stored chain has extra trailing seed → Diverged (length mismatch).
    let mut longer = stored.clone();
    longer.push(pg_ephemeral::label::SeedEntry {
        name: "extra".parse().unwrap(),
        config: pg_ephemeral::config::SeedConfig::SqlStatement {
            statement: String::new(),
        },
        hash: stored[0].hash.clone(),
    });
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(&stored_image, &longer, &image, &loaded),
        pg_ephemeral::session::SeedStatus::Diverged,
    );

    // (4) Stored chain is shorter → Diverged.
    let shorter = vec![stored[0].clone()];
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(&stored_image, &shorter, &image, &loaded),
        pg_ephemeral::session::SeedStatus::Diverged,
    );

    // (5) Name mismatch at position → Diverged.
    let mut renamed = stored.clone();
    renamed[1].name = "renamed".parse().unwrap();
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(&stored_image, &renamed, &image, &loaded),
        pg_ephemeral::session::SeedStatus::Diverged,
    );

    // (6) Hash mismatch at position → Diverged.
    let mut tweaked = stored.clone();
    tweaked[1].hash = Some(
        "0000000000000000000000000000000000000000000000000000000000000000"
            .parse()
            .unwrap(),
    );
    assert_eq!(
        pg_ephemeral::session::compute_seed_status(&stored_image, &tweaked, &image, &loaded),
        pg_ephemeral::session::SeedStatus::Diverged,
    );
}
