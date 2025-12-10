mod common;

use common::{TestDir, TestGitRepo, run_pg_ephemeral};
use std::str::FromStr;

/// Helper to manage cache test cleanup
struct CacheTestContext {
    backend: ociman::Backend,
    instance_name: String,
    name: ociman::reference::Name,
}

impl CacheTestContext {
    fn new(backend: ociman::Backend, instance_name: &str) -> Self {
        let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
            .parse()
            .unwrap();

        let context = Self {
            backend,
            instance_name: instance_name.to_string(),
            name,
        };
        context.cleanup();
        context
    }

    fn cleanup(&self) {
        for reference in self.backend.image_references_by_name(&self.name) {
            self.backend.remove_image_force(&reference);
        }
    }

    fn definition(&self) -> pg_ephemeral::Definition {
        pg_ephemeral::Definition::new(self.backend.clone(), pg_ephemeral::Image::default())
    }
}

impl Drop for CacheTestContext {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[tokio::test]
async fn test_populate_cache_empty_seeds() {
    let backend = ociman::test_backend_setup!();
    let context = CacheTestContext::new(backend, "populate-cache-empty");

    let definition = context.definition();

    let (last_cache_hit, uncached_seeds) = definition.populate_cache(&context.instance_name).await;

    assert!(last_cache_hit.is_none());
    assert!(uncached_seeds.is_empty());
}

#[tokio::test]
async fn test_populate_cache_multiple_seeds_chain() {
    let backend = ociman::test_backend_setup!();
    let context = CacheTestContext::new(backend, "populate-cache-chain");

    let definition = context
        .definition()
        .apply_script(
            "a-create-table".parse().unwrap(),
            r#"psql -c "CREATE TABLE chain_test (id INTEGER PRIMARY KEY);""#,
        )
        .unwrap()
        .apply_script(
            "b-insert-first".parse().unwrap(),
            r#"psql -c "INSERT INTO chain_test VALUES (1);""#,
        )
        .unwrap()
        .apply_script(
            "c-insert-second".parse().unwrap(),
            r#"psql -c "INSERT INTO chain_test VALUES (2);""#,
        )
        .unwrap();

    // All seeds should be miss initially
    let loaded_seeds = definition.load_seeds(&context.instance_name).unwrap();
    assert_eq!(loaded_seeds.iter_seeds().count(), 3);
    for seed in loaded_seeds.iter_seeds() {
        assert!(!seed.cache_status().is_hit());
    }

    // Populate cache
    let (last_cache_hit, uncached_seeds) = definition.populate_cache(&context.instance_name).await;

    // Should have cached all seeds
    assert!(last_cache_hit.is_some());
    assert!(uncached_seeds.is_empty());

    // All seeds should now be hits
    let loaded_seeds = definition.load_seeds(&context.instance_name).unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(seed.cache_status().is_hit());
    }

    // Verify the final cache image has all seed effects
    let cache_definition = pg_ephemeral::container::Definition {
        image: last_cache_hit.unwrap(),
        password: pg_client::Password::from_str("test_password").unwrap(),
        username: pg_client::Username::from_str("postgres").unwrap(),
        database: pg_client::Database::from_str("postgres").unwrap(),
        backend: context.backend.clone(),
        cross_container_access: false,
        application_name: None,
        ssl_config: None,
    };

    let mut container = pg_ephemeral::Container::run_container_definition(&cache_definition);
    container.set_superuser_password(&cache_definition.password);
    container.wait_available().await;

    container
        .with_connection(async |connection| {
            let rows: Vec<(i32,)> = sqlx::query_as("SELECT id FROM chain_test ORDER BY id")
                .fetch_all(&mut *connection)
                .await
                .unwrap();
            assert_eq!(rows, vec![(1,), (2,)]);
        })
        .await;

    container.stop();
}

#[tokio::test]
async fn test_populate_cache_partial_hit() {
    let backend = ociman::test_backend_setup!();
    let context = CacheTestContext::new(backend, "populate-cache-partial");

    // First, populate cache with only the first seed
    let definition_first = context
        .definition()
        .apply_script(
            "a-create-table".parse().unwrap(),
            r#"psql -c "CREATE TABLE partial_test (id INTEGER PRIMARY KEY);""#,
        )
        .unwrap();

    definition_first
        .populate_cache(&context.instance_name)
        .await;

    // Now create a definition with two seeds - first should be hit, second should be miss
    let definition_two = context
        .definition()
        .apply_script(
            "a-create-table".parse().unwrap(),
            r#"psql -c "CREATE TABLE partial_test (id INTEGER PRIMARY KEY);""#,
        )
        .unwrap()
        .apply_script(
            "b-insert-data".parse().unwrap(),
            r#"psql -c "INSERT INTO partial_test VALUES (99);""#,
        )
        .unwrap();

    // Verify first is hit, second is miss
    let loaded_seeds: Vec<_> = definition_two
        .load_seeds(&context.instance_name)
        .unwrap()
        .iter_seeds()
        .cloned()
        .collect();

    assert_eq!(loaded_seeds.len(), 2);
    assert!(loaded_seeds[0].cache_status().is_hit());
    assert!(!loaded_seeds[1].cache_status().is_hit());

    // Populate cache - should only create one new image (for second seed)
    let (last_cache_hit, uncached_seeds) =
        definition_two.populate_cache(&context.instance_name).await;

    assert!(last_cache_hit.is_some());
    assert!(uncached_seeds.is_empty());

    // Now both should be hits
    let loaded_seeds: Vec<_> = definition_two
        .load_seeds(&context.instance_name)
        .unwrap()
        .iter_seeds()
        .cloned()
        .collect();

    assert!(loaded_seeds[0].cache_status().is_hit());
    assert!(loaded_seeds[1].cache_status().is_hit());

    // Verify the final cache image has both seed effects
    let cache_definition = pg_ephemeral::container::Definition {
        image: last_cache_hit.unwrap(),
        password: pg_client::Password::from_str("test_password").unwrap(),
        username: pg_client::Username::from_str("postgres").unwrap(),
        database: pg_client::Database::from_str("postgres").unwrap(),
        backend: context.backend.clone(),
        cross_container_access: false,
        application_name: None,
        ssl_config: None,
    };

    let mut container = pg_ephemeral::Container::run_container_definition(&cache_definition);
    container.set_superuser_password(&cache_definition.password);
    container.wait_available().await;

    container
        .with_connection(async |connection| {
            let row: (i32,) = sqlx::query_as("SELECT id FROM partial_test")
                .fetch_one(&mut *connection)
                .await
                .unwrap();
            assert_eq!(row.0, 99);
        })
        .await;

    container.stop();
}

#[tokio::test]
async fn test_populate_cache_all_hits() {
    let backend = ociman::test_backend_setup!();
    let context = CacheTestContext::new(backend, "populate-cache-allhits");

    let definition = context
        .definition()
        .apply_script(
            "a-create-table".parse().unwrap(),
            r#"psql -c "CREATE TABLE allhits_test (id INTEGER PRIMARY KEY);""#,
        )
        .unwrap()
        .apply_script(
            "b-insert-data".parse().unwrap(),
            r#"psql -c "INSERT INTO allhits_test VALUES (1);""#,
        )
        .unwrap();

    // First populate to fill the cache
    let (first_cache_hit, _) = definition.populate_cache(&context.instance_name).await;
    assert!(first_cache_hit.is_some());

    // All seeds should now be hits
    let loaded_seeds: Vec<_> = definition
        .load_seeds(&context.instance_name)
        .unwrap()
        .iter_seeds()
        .cloned()
        .collect();

    for seed in &loaded_seeds {
        assert!(seed.cache_status().is_hit());
    }

    // Call populate_cache again - should return immediately with same reference
    let (second_cache_hit, uncached_seeds) =
        definition.populate_cache(&context.instance_name).await;

    assert_eq!(first_cache_hit, second_cache_hit);
    assert!(uncached_seeds.is_empty());
}

#[tokio::test]
async fn test_populate_cache_uncacheable_breaks_chain() {
    let backend = ociman::test_backend_setup!();
    let context = CacheTestContext::new(backend, "populate-cache-uncacheable");

    // Create definition with: cacheable -> uncacheable -> cacheable
    // The uncacheable seed should break the chain
    let definition = context
        .definition()
        .apply_script(
            "a-create-table".parse().unwrap(),
            r#"psql -c "CREATE TABLE uncacheable_test (id INTEGER PRIMARY KEY);""#,
        )
        .unwrap()
        .apply_command(
            "b-uncacheable".parse().unwrap(),
            pg_ephemeral::Command::new(
                "psql",
                ["-c", "INSERT INTO uncacheable_test VALUES (1);"],
            ),
            pg_ephemeral::CommandCacheConfig::None, // This makes it uncacheable
        )
        .unwrap()
        .apply_script(
            "c-after-uncacheable".parse().unwrap(),
            r#"psql -c "INSERT INTO uncacheable_test VALUES (2);""#,
        )
        .unwrap();

    // Verify the second seed is uncacheable
    let loaded_seeds: Vec<_> = definition
        .load_seeds(&context.instance_name)
        .unwrap()
        .iter_seeds()
        .cloned()
        .collect();

    assert_eq!(loaded_seeds.len(), 3);
    assert!(loaded_seeds[0].cache_status().reference().is_some()); // cacheable
    assert!(loaded_seeds[1].cache_status().reference().is_none()); // uncacheable
    assert!(loaded_seeds[2].cache_status().reference().is_none()); // chain broken

    // Populate cache - should cache first seed, then return remaining as uncached
    let (last_cache_hit, uncached_seeds) =
        definition.populate_cache(&context.instance_name).await;

    // Should have cached only the first seed
    assert!(last_cache_hit.is_some());

    // Should return the uncacheable seed and all seeds after it
    assert_eq!(uncached_seeds.len(), 2);
    assert_eq!(uncached_seeds[0].name().as_str(), "b-uncacheable");
    assert_eq!(uncached_seeds[1].name().as_str(), "c-after-uncacheable");

    // First seed should now be a hit
    let loaded_seeds: Vec<_> = definition
        .load_seeds(&context.instance_name)
        .unwrap()
        .iter_seeds()
        .cloned()
        .collect();

    assert!(loaded_seeds[0].cache_status().is_hit());
}

#[tokio::test]
async fn test_populate_cache() {
    let backend = ociman::test_backend_setup!();
    let instance_name = "populate-cache-test";

    // Clean up any leftover images from previous runs
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name) {
        backend.remove_image_force(&reference);
    }

    let definition = pg_ephemeral::Definition::new(backend.clone(), pg_ephemeral::Image::default())
        .apply_script(
            "schema-and-data".parse().unwrap(),
            r#"psql -c "CREATE TABLE test_cache (id INTEGER PRIMARY KEY); INSERT INTO test_cache VALUES (42);""#,
        )
        .unwrap();

    // Verify cache status is Miss initially
    let loaded_seeds = definition.load_seeds(instance_name).unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(!seed.cache_status().is_hit());
    }

    // Populate cache
    definition.populate_cache(instance_name).await;

    // Verify cache status is now Hit
    let loaded_seeds = definition.load_seeds(instance_name).unwrap();
    let mut last_cache_reference = None;
    for seed in loaded_seeds.iter_seeds() {
        assert!(seed.cache_status().is_hit());
        last_cache_reference = seed.cache_status().reference().cloned();
    }

    // Boot from the cached image and verify the seed effect is present
    let cache_definition = pg_ephemeral::container::Definition {
        image: last_cache_reference.clone().unwrap(),
        password: pg_client::Password::from_str("test_password").unwrap(),
        username: pg_client::Username::from_str("postgres").unwrap(),
        database: pg_client::Database::from_str("postgres").unwrap(),
        backend: backend.clone(),
        cross_container_access: false,
        application_name: None,
        ssl_config: None,
    };

    let mut container = pg_ephemeral::Container::run_container_definition(&cache_definition);
    container.set_superuser_password(&cache_definition.password);
    container.wait_available().await;

    container
        .with_connection(async |connection| {
            let row: (i32,) = sqlx::query_as("SELECT id FROM test_cache")
                .fetch_one(&mut *connection)
                .await
                .unwrap();
            assert_eq!(row.0, 42);
        })
        .await;

    container.stop();

    // Clean up images
    for reference in backend.image_references_by_name(&name) {
        backend.remove_image_force(&reference);
    }
}

#[test]
fn test_cache_status() {
    let _backend = ociman::test_backend_setup!();
    let repo = TestGitRepo::new("cache-test");

    repo.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    repo.write_file("data.sql", "INSERT INTO users (id) VALUES (1);");
    let commit_hash = repo.commit("Initial");

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
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "a-schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"

        [[seeds]]
        name = "b-data-from-git"
        type = "sql-file-git-revision"
        cache_image = "pg-ephemeral/main:9d1fe3c033a5353478c44008c8c34a1223ab208664d43a1c63b22172c9d9c645"
        status = "miss"

        [[seeds]]
        name = "c-run-command"
        type = "command"
        cache_image = "pg-ephemeral/main:9c76bdb2e278bff90fa66ea86693deaeab85d2b84ff009d8c31d5d8b0c857e88"
        status = "miss"

        [[seeds]]
        name = "d-run-script"
        type = "script"
        cache_image = "pg-ephemeral/main:2204e587eb3ffecb4d3c372f127cbaf26a3c4df88a63b1fa86ec26036d36ecdc"
        status = "miss"
    "#};

    let stdout = run_pg_ephemeral(&["cache", "status"], &repo.path);
    assert_eq!(stdout, expected);
}

#[test]
fn test_cache_status_deterministic() {
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
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let stdout = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout, expected);
}

#[test]
fn test_cache_status_change_with_content() {
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

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout1, expected_before);

    dir.write_file(
        "schema.sql",
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
    );

    let stdout2 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    // Cache image should change when content changes - just verify it's different
    assert_ne!(stdout2, expected_before);
}

#[test]
fn test_cache_status_change_with_image() {
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

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout1, expected_before);

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.2"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let stdout2 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    // Cache image should change when image changes - just verify it's different
    assert_ne!(stdout2, expected_before);
}

#[test]
fn test_cache_status_chain_propagates() {
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
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "a-first"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:f8b223da0e4cf8d8db2add4450845e7563bf9800e27773f3301cdf7740cf6176"
        status = "miss"

        [[seeds]]
        name = "b-second"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:96482a6449ac57c34b002ba936158de8ab89819238bd18863c37450d6915cd8e"
        status = "miss"
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout1, expected_before);

    dir.write_file("first.sql", "CREATE TABLE first (id INTEGER, name TEXT);");

    let stdout2 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    // Cache image should change when first seed changes, and propagate to second seed
    assert_ne!(stdout2, expected_before);
}

#[test]
fn test_cache_status_key_command() {
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
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:7f881e9f75f7767a96ff07d2ee9649c0754a228cd69517e31928107abb11b256"
        status = "miss"
        cache_key_output = "1.0.0"
    "#};

    let stdout1 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout1, expected_before);

    // Change the version file - cache image should change
    dir.write_file("version.txt", "2.0.0");

    let stdout2 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    // Cache image should change when key command output changes
    assert_ne!(stdout2, expected_before);
}

#[test]
fn test_cache_status_output_truncation_and_verbose() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-truncation-test");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.run-migrations]
            type = "command"
            command = "migrate"
            arguments = ["up"]

            [instances.main.seeds.run-migrations.cache]
            type = "key-script"
            script = "echo 'line1'; echo 'line2'; echo 'line3'"
        "#},
    );

    let expected_truncated = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:c0bb68b81534ae662400ea38803ee43940c9a96b20c625bbd517dac8018511e8"
        status = "miss"
        cache_key_output = "line1 [...2 more lines]"
    "#};

    let expected_verbose = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:c0bb68b81534ae662400ea38803ee43940c9a96b20c625bbd517dac8018511e8"
        status = "miss"
        cache_key_output = """
        line1
        line2
        line3
        """
    "#};

    // Without --verbose: truncated output
    let stdout1 = run_pg_ephemeral(&["cache", "status"], &dir.path);
    assert_eq!(stdout1, expected_truncated);

    // With --verbose: full output
    let stdout2 = run_pg_ephemeral(&["cache", "status", "--verbose"], &dir.path);
    assert_eq!(stdout2, expected_verbose);
}

#[test]
fn test_cache_status_change_with_ssl() {
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

    let output_no_ssl = run_pg_ephemeral(&["cache", "status"], &dir.path);

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

    let output_with_ssl = run_pg_ephemeral(&["cache", "status"], &dir.path);

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

    let output_different_ssl = run_pg_ephemeral(&["cache", "status"], &dir.path);

    // Cache key should change when SSL hostname changes
    assert_ne!(output_with_ssl, output_different_ssl);
}
