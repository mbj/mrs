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
          "version": "0.0.1-pre2",
          "seeds": [
            {
              "name": "a-schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
            },
            {
              "name": "b-data-from-git",
              "type": "sql-file-git-revision",
              "status": "miss",
              "reference": "pg-ephemeral/main:9d1fe3c033a5353478c44008c8c34a1223ab208664d43a1c63b22172c9d9c645"
            },
            {
              "name": "c-run-command",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:9c76bdb2e278bff90fa66ea86693deaeab85d2b84ff009d8c31d5d8b0c857e88"
            },
            {
              "name": "d-run-script",
              "type": "script",
              "status": "miss",
              "reference": "pg-ephemeral/main:2204e587eb3ffecb4d3c372f127cbaf26a3c4df88a63b1fa86ec26036d36ecdc"
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
          "version": "0.0.1-pre2",
          "seeds": [
            {
              "name": "schema",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
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
          "version": "0.0.1-pre2",
          "seeds": [
            {
              "name": "a-first",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:f8b223da0e4cf8d8db2add4450845e7563bf9800e27773f3301cdf7740cf6176"
            },
            {
              "name": "b-second",
              "type": "sql-file",
              "status": "miss",
              "reference": "pg-ephemeral/main:96482a6449ac57c34b002ba936158de8ab89819238bd18863c37450d6915cd8e"
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
          "version": "0.0.1-pre2",
          "seeds": [
            {
              "name": "run-migrations",
              "type": "command",
              "status": "miss",
              "reference": "pg-ephemeral/main:7f881e9f75f7767a96ff07d2ee9649c0754a228cd69517e31928107abb11b256"
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
