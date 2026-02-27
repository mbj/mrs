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
          "version": "0.0.8",
          "seeds": [
            {
              "name": "a-schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:a180f0b9fd91c2a05143caa5a615e3828a20f19cb3d60f72171e53063196fe9a"
            },
            {
              "name": "b-data-from-git",
              "type": "sql-file-git-revision",
              "status": "miss",
              "reference": "pg-ephemeral/main:86ee6638a4366eb6c10eb75bb6e65519124737b2e30508488ae76885e99a6597"
            },
            {
              "name": "c-run-command",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:ffec5f3796bb00119e927dd15ed83498474c50bab1c5e40bcfbab79921b67350"
            },
            {
              "name": "d-run-script",
              "type": "script",
              "status": "miss",
              "reference": "pg-ephemeral/main:39d675c99e830899dd847930a0b619aa0ca14405ec11f498c69159cfd480211d"
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
          "version": "0.0.8",
          "seeds": [
            {
              "name": "schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:a180f0b9fd91c2a05143caa5a615e3828a20f19cb3d60f72171e53063196fe9a"
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
          "version": "0.0.8",
          "seeds": [
            {
              "name": "a-first",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:67b1e9a9a8d9401b82d61701125d8425b837ae1689d1f207864e3db8deb80e05"
            },
            {
              "name": "b-second",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:8913678d58c8c05072435d0174c37a8c392046f582e6ba03b3fe43d88de2bed1"
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
          "version": "0.0.8",
          "seeds": [
            {
              "name": "run-migrations",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:dc333d3e5b19b81f823fb8866285f57ca13e318e36dd5406e80abe0a4d064efe"
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
