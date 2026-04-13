mod common;

use common::{TestDir, TestGitRepo, run_pg_ephemeral};

#[tokio::test]
async fn test_populate_cache() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "populate-cache-test".parse().unwrap();

    // Clean up any leftover images from previous runs
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }

    let definition = pg_ephemeral::Definition::new(backend.clone(), pg_ephemeral::Image::default(), instance_name.clone())
        .wait_available_timeout(std::time::Duration::from_secs(30))
        .apply_script(
            "schema-and-data".parse().unwrap(),
            r##"psql -c "CREATE TABLE test_cache (id INTEGER PRIMARY KEY); INSERT INTO test_cache VALUES (42);""##,
        )
        .unwrap();

    // Verify cache status is Miss initially
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(!seed.cache_status().is_hit());
    }

    // Populate cache
    definition.populate_cache(&instance_name).await.unwrap();

    // Verify cache status is now Hit
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(seed.cache_status().is_hit());
    }

    // Boot from the cached image using with_container (which handles cache hits properly)
    // and verify the seed effect is present
    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row: (i32,) = sqlx::query_as("SELECT id FROM test_cache")
                        .fetch_one(&mut *connection)
                        .await
                        .unwrap();
                    assert_eq!(row.0, 42);
                })
                .await;
        })
        .await
        .unwrap();

    // Clean up images
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }
}

#[tokio::test]
async fn test_cache_status() {
    let _backend = ociman::test_backend_setup!();
    let repo = TestGitRepo::new("cache-test").await;

    repo.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    repo.write_file("data.sql", "INSERT INTO users (id) VALUES (1);");
    let commit_hash = repo.commit("Initial").await;

    let config_content = indoc::formatdoc! {r#"
        image = "17.1"

        [instances.main.seeds.a-schema]
        type = "sql-file"
        path = "schema.sql"

        [instances.main.seeds.b-data-from-git]
        type = "sql-file"
        path = "data.sql"
        git_revision = "{commit_hash}"

        [instances.main.seeds.c-run-command]
        type = "command"
        command = "echo"
        arguments = ["hello"]
        cache.type = "command-hash"

        [instances.main.seeds.d-run-script]
        type = "script"
        script = "echo 'hello world'"
    "#};
    repo.write_file("database.toml", &config_content);

    let expected = indoc::indoc! {r#"
        {
          "instance": "main",
          "image": "17.1",
          "version": "0.2.2",
          "seeds": [
            {
              "name": "a-schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:03e68a0cbf3814e9c0b5e4a801252b65b61e5e1a878d9967267f8ee92ca3b4c1"
            },
            {
              "name": "b-data-from-git",
              "type": "sql-file-git-revision",
              "status": "miss",
              "reference": "pg-ephemeral/main:69bb9ca294bc4953a64be2807394e95d25101090e02a630410be1bb74e65db12"
            },
            {
              "name": "c-run-command",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:47f9803cd158df434114d6a26c910936ea4ce8bc21798a06b5ecb5b670fb6623"
            },
            {
              "name": "d-run-script",
              "type": "script",
              "status": "miss",
              "reference": "pg-ephemeral/main:3091aa7b12ac96577c260a99907a244e139f7477e8f80f5099ccc5457ab707e1"
            }
          ]
        }
    "#};

    let stdout = run_pg_ephemeral(&["cache", "status", "--json"], &repo.path).await;
    assert_eq!(stdout, expected);
}

#[tokio::test]
async fn test_cache_status_deterministic() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-deterministic-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let expected = indoc::indoc! {r#"
        {
          "instance": "main",
          "image": "17.1",
          "version": "0.2.2",
          "seeds": [
            {
              "name": "schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:03e68a0cbf3814e9c0b5e4a801252b65b61e5e1a878d9967267f8ee92ca3b4c1"
            }
          ]
        }
    "#};

    let stdout = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    assert_eq!(stdout, expected);
}

#[tokio::test]
async fn test_cache_status_change_with_content() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-changes-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let stdout1 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;

    dir.write_file(
        "schema.sql",
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
    );

    let stdout2 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    // Cache reference should change when content changes
    assert_ne!(stdout2, stdout1);
}

#[tokio::test]
async fn test_cache_status_change_with_image() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-image-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let stdout1 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.2"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let stdout2 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    // Cache reference should change when image changes
    assert_ne!(stdout2, stdout1);
}

#[tokio::test]
async fn test_cache_status_chain_propagates() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-chain-test");

    dir.write_file("first.sql", "CREATE TABLE first (id INTEGER);");
    dir.write_file("second.sql", "CREATE TABLE second (id INTEGER);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.a-first]
            type = "sql-file"
            path = "first.sql"

            [instances.main.seeds.b-second]
            type = "sql-file"
            path = "second.sql"
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        {
          "instance": "main",
          "image": "17.1",
          "version": "0.2.2",
          "seeds": [
            {
              "name": "a-first",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:a34cce90fc0e3c3f140ef38f0c325f181985c8a35ec97525ec14f73e038e3428"
            },
            {
              "name": "b-second",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:f32399cdac852a06627fba2c6163e24d007f72f0b84b46b86804308cb2505d6f"
            }
          ]
        }
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    assert_eq!(stdout1, expected_before);

    dir.write_file("first.sql", "CREATE TABLE first (id INTEGER, name TEXT);");

    let stdout2 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    // Cache reference should change when first seed changes, and propagate to second seed
    assert_ne!(stdout2, expected_before);
}

#[tokio::test]
async fn test_cache_status_key_command() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-key-command-test");

    dir.write_file("version.txt", "1.0.0");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.run-migrations]
            type = "command"
            command = "migrate"
            arguments = ["up"]

            [instances.main.seeds.run-migrations.cache]
            type = "key-command"
            command = "cat"
            arguments = ["version.txt"]
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        {
          "instance": "main",
          "image": "17.1",
          "version": "0.2.2",
          "seeds": [
            {
              "name": "run-migrations",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:351f9789d817a9154b6f8d584dd184097d0dd36e8446090a5df3d470885791f9"
            }
          ]
        }
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    assert_eq!(stdout1, expected_before);

    // Change the version file - cache reference should change
    dir.write_file("version.txt", "2.0.0");

    let stdout2 = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    // Cache reference should change when key command output changes
    assert_ne!(stdout2, expected_before);
}

#[tokio::test]
async fn test_cache_status_change_with_ssl() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-ssl-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output_no_ssl = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;

    // Add SSL config
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [ssl_config]
            hostname = "localhost"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output_with_ssl = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;

    // Cache key should change when SSL config is added
    assert_ne!(output_no_ssl, output_with_ssl);

    // Change SSL hostname
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [ssl_config]
            hostname = "example.com"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output_different_ssl = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;

    // Cache key should change when SSL hostname changes
    assert_ne!(output_with_ssl, output_different_ssl);
}

#[tokio::test]
async fn test_cache_status_container_script() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-container-script-test");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.install-ext]
            type = "container-script"
            script = "touch /container-script-marker"
        "#},
    );

    let stdout = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    let output: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    assert_eq!(output["seeds"][0]["name"], "install-ext");
    assert_eq!(output["seeds"][0]["type"], "container-script");
    assert_eq!(output["seeds"][0]["status"], "miss");
    assert!(output["seeds"][0]["reference"].is_string());
}

#[tokio::test]
async fn test_populate_cache_container_script() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName =
        "populate-cache-container-script-test".parse().unwrap();

    // Clean up any leftover images from previous runs
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }

    let definition = pg_ephemeral::Definition::new(
        backend.clone(),
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .apply_container_script(
        "create-marker".parse().unwrap(),
        "touch /container-script-marker",
    )
    .unwrap();

    // Verify cache status is Miss initially
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(!seed.cache_status().is_hit());
    }

    // Populate cache
    definition.populate_cache(&instance_name).await.unwrap();

    // Verify cache status is now Hit
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(seed.cache_status().is_hit());
    }

    // Boot from the cached image and verify PG starts cleanly
    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row: (bool,) = sqlx::query_as("SELECT true")
                        .fetch_one(&mut *connection)
                        .await
                        .unwrap();
                    assert!(row.0);
                })
                .await;
        })
        .await
        .unwrap();

    // Clean up images
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }
}

#[tokio::test]
async fn test_container_script_with_pg_cron() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName =
        "container-script-pg-cron-test".parse().unwrap();

    // Clean up any leftover images from previous runs
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }

    let definition = pg_ephemeral::Definition::new(
        backend.clone(),
        "17".parse().unwrap(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .apply_container_script(
        "install-pg-cron".parse().unwrap(),
        "apt-get update && apt-get install -y --no-install-recommends postgresql-17-cron \
         && printf '#!/bin/bash\\necho \"shared_preload_libraries = '\"'\"'pg_cron'\"'\"'\" >> \"$PGDATA/postgresql.conf\"\\n' \
            > /docker-entrypoint-initdb.d/pg-cron.sh \
         && chmod +x /docker-entrypoint-initdb.d/pg-cron.sh",
    )
    .unwrap()
    .apply_script(
        "enable-pg-cron".parse().unwrap(),
        r#"psql -c "CREATE EXTENSION pg_cron""#,
    )
    .unwrap();

    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row: (String,) = sqlx::query_as(
                        "SELECT extname::text FROM pg_extension WHERE extname = 'pg_cron'",
                    )
                    .fetch_one(&mut *connection)
                    .await
                    .unwrap();
                    assert_eq!(row.0, "pg_cron");
                })
                .await;
        })
        .await
        .unwrap();

    // Clean up images
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }
}

// JoinHandle is intentionally returned to be awaited after stop() terminates connections.
#[allow(clippy::async_yields_async)]
#[tokio::test]
async fn test_stale_connection_terminated_before_stop() {
    let backend = ociman::test_backend_setup!();

    let definition = common::test_definition(backend);

    // with_container returns the JoinHandle; stop() runs before it returns.
    let sleep_handle = definition
        .with_container(async |container| {
            let config = container.client_config().to_sqlx_connect_options().unwrap();
            let mut connection = sqlx::ConnectOptions::connect(&config).await.unwrap();

            tokio::spawn(async move {
                sqlx::query("SELECT pg_sleep(3600)")
                    .execute(&mut connection)
                    .await
            })
        })
        .await
        .unwrap();

    // stop() terminated all connections before shutting down.
    // The sleep query must fail with a connection error, not succeed or hang for 3600s.
    let error = sleep_handle.await.unwrap().unwrap_err();

    match error {
        sqlx::Error::Database(ref db_error) => {
            assert_eq!(db_error.code().as_deref(), Some("57P01"));
        }
        _ => panic!("Expected database error 57P01 (admin_shutdown), got: {error}"),
    }
}
