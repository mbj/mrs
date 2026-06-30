mod common;

use common::{TestDir, TestGitRepo, run_pg_ephemeral};
use typed_reqwest::Request;

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
            pg_ephemeral::SeedCacheConfig::CommandHash,
        )
        .unwrap();

    // Verify cache status is Miss initially
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    for seed in loaded_seeds.iter_seeds() {
        assert!(!seed.cache_status().is_hit());
    }

    // Populate cache
    definition.populate_cache(&loaded_seeds).await.unwrap();

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
async fn test_populate_cache_runs_seeds_in_declaration_order() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "populate-cache-order-test".parse().unwrap();

    // Clean up any leftover images from previous runs
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }

    // Seed names are declared in reverse alphabetic order (z -> m -> a) and each
    // seed depends on the previous one having executed. If populate_cache ever
    // sorts by name instead of honoring declaration order, the "m-insert" step
    // would run before "z-create-table" and fail because the table does not
    // exist yet.
    let definition = pg_ephemeral::Definition::new(
        backend.clone(),
        pg_ephemeral::Image::default(),
        instance_name.clone(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .apply_script(
        "z-create-table".parse().unwrap(),
        r#"psql -c "CREATE TABLE order_test (value INTEGER)""#,
        pg_ephemeral::SeedCacheConfig::CommandHash,
    )
    .unwrap()
    .apply_script(
        "m-insert-row".parse().unwrap(),
        r#"psql -c "INSERT INTO order_test VALUES (1)""#,
        pg_ephemeral::SeedCacheConfig::CommandHash,
    )
    .unwrap()
    .apply_script(
        "a-update-row".parse().unwrap(),
        r#"psql -c "UPDATE order_test SET value = 2 WHERE value = 1""#,
        pg_ephemeral::SeedCacheConfig::CommandHash,
    )
    .unwrap();

    // Populate cache - this will fail if seeds run in alphabetic order because
    // a-update-row references a table that z-create-table has not yet created.
    let loaded_seeds = definition.load_seeds(&instance_name).await.unwrap();
    definition.populate_cache(&loaded_seeds).await.unwrap();

    // Boot from the cached image and verify all three seeds ran in declaration
    // order: table created, row inserted with value 1, row updated to value 2.
    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row: (i32,) = sqlx::query_as("SELECT value FROM order_test")
                        .fetch_one(&mut *connection)
                        .await
                        .unwrap();
                    assert_eq!(row.0, 2);
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
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "cache-status".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;
    let repo = TestGitRepo::new("cache-test").await;

    repo.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    repo.write_file("data.sql", "INSERT INTO users (id) VALUES (1);");
    let commit_hash = repo.commit("Initial").await;

    let config_content = indoc::formatdoc! {r#"
        image = "17.1"

        [instances.cache-status.seeds.a-schema]
        type = "sql-file"
        path = "schema.sql"

        [instances.cache-status.seeds.b-data-from-git]
        type = "sql-file"
        path = "data.sql"
        git_revision = "{commit_hash}"

        [instances.cache-status.seeds.c-run-command]
        type = "command"
        command = "echo"
        arguments = ["hello"]
        cache.type = "command-hash"

        [instances.cache-status.seeds.d-run-script]
        type = "script"
        script = "echo 'hello world'"
    "#};
    repo.write_file("database.toml", &config_content);

    let expected = indoc::indoc! {r#"
        {
          "instance": "cache-status",
          "base_image": "17.1",
          "version": "0.6.0",
          "summary": {
            "total": 4,
            "hits": 0,
            "misses": 4,
            "uncacheable": 0
          },
          "seeds": [
            {
              "name": "a-schema",
              "type": "sql-file",
              "status": "miss",
              "cache_image": "pg-ephemeral/cache-status:8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27"
            },
            {
              "name": "b-data-from-git",
              "type": "sql-file-git-revision",
              "status": "miss",
              "cache_image": "pg-ephemeral/cache-status:f93452df939346a67214ba6a806183355300b3c83c7643ab583cb4b2666eace9"
            },
            {
              "name": "c-run-command",
              "type": "command",
              "status": "miss",
              "cache_image": "pg-ephemeral/cache-status:29844e778da525054d197a676576c4c8192b81eab3b602f2599f6af25eb7e1af"
            },
            {
              "name": "d-run-script",
              "type": "script",
              "status": "miss",
              "cache_image": "pg-ephemeral/cache-status:83f2fd61f5f26943afed556d999475b28e501091b261a9c330604d6123b46325"
            }
          ]
        }
    "#};

    let stdout = run_pg_ephemeral(
        &[
            "cache",
            "--instance",
            instance_name.as_ref(),
            "status",
            "--json",
        ],
        &repo.path,
    )
    .await;
    assert_eq!(stdout, expected);
}

#[tokio::test]
async fn test_cache_status_deterministic() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "cache-deterministic".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;
    let dir = TestDir::new("cache-deterministic-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.cache-deterministic.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let expected = indoc::indoc! {r#"
        {
          "instance": "cache-deterministic",
          "base_image": "17.1",
          "version": "0.6.0",
          "summary": {
            "total": 1,
            "hits": 0,
            "misses": 1,
            "uncacheable": 0
          },
          "seeds": [
            {
              "name": "schema",
              "type": "sql-file",
              "status": "miss",
              "cache_image": "pg-ephemeral/cache-deterministic:8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27"
            }
          ]
        }
    "#};

    let stdout = run_pg_ephemeral(
        &[
            "cache",
            "--instance",
            instance_name.as_ref(),
            "status",
            "--json",
        ],
        &dir.path,
    )
    .await;
    assert_eq!(stdout, expected);
}

#[tokio::test]
async fn test_cache_status_uncacheable_reason() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "cache-uncacheable-reason".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;
    let dir = TestDir::new("cache-uncacheable-reason-test");

    // Same schema.sql + image as `test_cache_status_deterministic`, so the
    // schema seed's reference hash is fixed and we can assert against an
    // exact JSON.
    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    // chain: cacheable schema -> chain-breaker `nope` (cache=none)
    //        -> chain-broken `tail` (uncacheable because predecessor broke chain)
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.cache-uncacheable-reason.seeds.schema]
            type = "sql-file"
            path = "schema.sql"

            [instances.cache-uncacheable-reason.seeds.nope]
            type = "script"
            script = "true"
            cache = { type = "none" }

            [instances.cache-uncacheable-reason.seeds.tail]
            type = "sql-statement"
            statement = "SELECT 1"
        "#},
    );

    let expected = serde_json::json!({
        "instance": "cache-uncacheable-reason",
        "base_image": "17.1",
        "version": "0.6.0",
        "summary": {
            "total": 3,
            "hits": 0,
            "misses": 1,
            "uncacheable": 2,
        },
        "seeds": [
            {
                "name": "schema",
                "type": "sql-file",
                "status": "miss",
                "cache_image": "pg-ephemeral/cache-uncacheable-reason:8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27",
            },
            {
                "name": "nope",
                "type": "script",
                "status": "uncacheable",
                "reason": "cache_strategy_none",
            },
            {
                "name": "tail",
                "type": "sql-statement",
                "status": "uncacheable",
                "reason": "chain_broken_by_predecessor",
                "broken_by": "nope",
            },
        ],
    });

    let stdout = run_pg_ephemeral(
        &[
            "cache",
            "--instance",
            instance_name.as_ref(),
            "status",
            "--json",
        ],
        &dir.path,
    )
    .await;
    let actual: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(actual, expected);
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
async fn test_cache_registry_prefixes_reference_without_changing_hash() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-registry-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    // Baseline: no cache_registry.
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );
    let baseline = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    let baseline: serde_json::Value = serde_json::from_str(&baseline).unwrap();
    let baseline_reference = baseline["seeds"][0]["cache_image"].as_str().unwrap();
    assert!(baseline_reference.starts_with("pg-ephemeral/main:"));

    // Same config plus cache_registry.
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"
            cache_registry = "ghcr.io/mbj"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );
    let prefixed = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
    let prefixed: serde_json::Value = serde_json::from_str(&prefixed).unwrap();
    let prefixed_reference = prefixed["seeds"][0]["cache_image"].as_str().unwrap();

    // The prefixed reference should be the baseline reference with the registry prepended,
    // proving (a) the registry prefix is applied and (b) the hash is unaffected.
    assert_eq!(
        prefixed_reference,
        format!("ghcr.io/mbj/{baseline_reference}")
    );
}

async fn run_pg_ephemeral_expect_failure(
    args: &[&str],
    current_dir: &std::path::Path,
) -> (String, String) {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");
    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(args)
        .working_directory(current_dir)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    assert!(
        !output.status.success(),
        "expected pg-ephemeral {} to fail, but it succeeded\nstdout:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout)
    );

    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[tokio::test]
async fn test_cache_pull_without_registry_errors() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-pull-no-registry");

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

    let (_stdout, stderr) = run_pg_ephemeral_expect_failure(&["cache", "pull"], &dir.path).await;
    assert!(
        stderr.contains("cache_registry must be set"),
        "expected registry-not-set error, got stderr:\n{stderr}"
    );
}

#[tokio::test]
async fn test_cache_push_without_registry_errors() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cache-push-no-registry");

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

    let (_stdout, stderr) = run_pg_ephemeral_expect_failure(&["cache", "push"], &dir.path).await;
    assert!(
        stderr.contains("cache_registry must be set"),
        "expected registry-not-set error, got stderr:\n{stderr}"
    );
}

/// Marker for the OCI distribution API the readiness probe talks to.
struct RegistryV2Api;

/// `GET /v2/` — the OCI distribution base endpoint. A `200 OK` means the
/// registry is up and serving; we only care that it answers, so the
/// response body is discarded.
struct RegistryV2Ping;

impl typed_reqwest::Request<RegistryV2Api> for RegistryV2Ping {
    type Response = ();

    typed_reqwest::decoder!(
        typed_reqwest::decoder::Response::build()
            .status_code_constant(http::StatusCode::OK, ())
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &typed_reqwest::BaseUrl,
    ) -> reqwest::RequestBuilder {
        // Trailing empty segment preserves the `/v2/` trailing slash the
        // distribution spec uses for the base endpoint.
        client.get(base_url.set_path_segments(&["v2", ""]))
    }
}

/// Poll the registry's `/v2/` endpoint until it answers, so the round-trip
/// below doesn't race the container's HTTP server coming up. The published
/// host port accepts connections as soon as the runtime sets up its
/// forwarder, which can be before the registry process itself is listening.
async fn wait_registry_ready(port: u16) {
    let client = reqwest::Client::new();
    let base_url = typed_reqwest::BaseUrl::new(
        typed_reqwest::Scheme::Http,
        url::Host::parse("localhost").unwrap(),
        Some(port),
    );
    // Materialize the `const DECODER` (a `LazyLock`) into a local once;
    // borrowing it directly per call trips `borrow_interior_mutable_const`.
    let decoder = RegistryV2Ping::DECODER;

    for _ in 0..120 {
        if let Ok(response) = RegistryV2Ping
            .request_builder(&client, &base_url)
            .send()
            .await
            && decoder.decode(response).await.is_ok()
        {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
    panic!("registry at localhost:{port} did not become ready in time");
}

const CONTAINERS_REGISTRIES_CONF: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("CONTAINERS_REGISTRIES_CONF");

/// Absolute path to the committed `registries.conf` fixture that marks the
/// local test registry (`localhost:5000`) insecure.
fn insecure_registries_conf() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registries.conf")
}

/// Like [`run_pg_ephemeral`] but points podman at the insecure-registry
/// fixture via `CONTAINERS_REGISTRIES_CONF`, for the steps that actually
/// talk to the plain-HTTP local registry (push/pull). Podman otherwise
/// defaults to HTTPS for `localhost:<port>`; docker ignores the var and
/// trusts loopback natively, so this is a no-op there.
async fn run_pg_ephemeral_insecure(args: &[&str], current_dir: &std::path::Path) -> String {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");
    let conf = insecure_registries_conf();
    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(args)
        .working_directory(current_dir)
        .env(&CONTAINERS_REGISTRIES_CONF, &conf)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    assert!(
        output.status.success(),
        "pg-ephemeral {} failed:\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).unwrap()
}

/// End-to-end push/pull round trip against a throwaway local registry.
///
/// Boots `registry:2` on `localhost:5000`, points `cache_registry` at it,
/// then runs: populate (build local cache) -> push (upload) -> reset
/// --force (clear local) -> assert the stage is a miss -> pull (walk back
/// from the tip) -> assert it's a local hit again. The registry needs no
/// auth; podman is steered to plain HTTP via the insecure-registry fixture
/// (see [`run_pg_ephemeral_insecure`]) and docker trusts loopback natively,
/// so this runs in the normal suite — no credentials, no external
/// dependency.
#[tokio::test]
async fn test_cache_registry_round_trip() {
    // Fixed host port so the committed `registries.conf` fixture can name
    // it; `registry:2` listens on this port inside the container too.
    const REGISTRY_PORT: u16 = 5000;

    let backend = ociman::test_backend_setup!();

    let registry_image = common::REGISTRY_IMAGE.clone();
    backend.pull_image_if_absent(&registry_image).await.unwrap();

    let registry_definition = ociman::Definition::new(backend.clone(), registry_image)
        .remove()
        .publish(
            ociman::Publish::tcp(REGISTRY_PORT)
                .host_ip_port(std::net::Ipv4Addr::LOCALHOST.into(), REGISTRY_PORT),
        );

    registry_definition
        .with_container(async |_container| {
            wait_registry_ready(REGISTRY_PORT).await;

            let dir = TestDir::new("cache-registry-round-trip");
            dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
            dir.write_file(
                "database.toml",
                indoc::indoc! {r#"
                    image = "17.1"
                    cache_registry = "localhost:5000/pg-ephemeral-cache-test"

                    [instances.main.seeds.schema]
                    type = "sql-file"
                    path = "schema.sql"
                "#},
            );

            // populate/reset/status are local-only; push and pull contact
            // the registry, so they get the insecure-registry fixture.
            run_pg_ephemeral(&["cache", "populate"], &dir.path).await;
            run_pg_ephemeral_insecure(&["cache", "push"], &dir.path).await;
            run_pg_ephemeral(&["cache", "reset", "--force"], &dir.path).await;

            // Same schema + image as `test_cache_status_deterministic`, and
            // `cache_registry` does not feed the cache key, so the hash is
            // that test's constant — only the registry prefix differs.
            let cache_image = "localhost:5000/pg-ephemeral-cache-test/pg-ephemeral/main:\
                8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27";

            let after_reset = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
            let after_reset: serde_json::Value = serde_json::from_str(&after_reset).unwrap();
            assert_eq!(
                after_reset,
                serde_json::json!({
                    "instance": "main",
                    "base_image": "17.1",
                    "version": "0.6.0",
                    "summary": { "total": 1, "hits": 0, "misses": 1, "uncacheable": 0 },
                    "seeds": [{
                        "name": "schema",
                        "type": "sql-file",
                        "status": "miss",
                        "cache_image": cache_image,
                    }],
                })
            );

            run_pg_ephemeral_insecure(&["cache", "pull"], &dir.path).await;

            let after_pull = run_pg_ephemeral(&["cache", "status", "--json"], &dir.path).await;
            let after_pull: serde_json::Value = serde_json::from_str(&after_pull).unwrap();
            assert_eq!(
                after_pull,
                serde_json::json!({
                    "instance": "main",
                    "base_image": "17.1",
                    "version": "0.6.0",
                    "summary": { "total": 1, "hits": 1, "misses": 0, "uncacheable": 0 },
                    "seeds": [{
                        "name": "schema",
                        "type": "sql-file",
                        "status": "hit",
                        "cache_image": cache_image,
                    }],
                })
            );
        })
        .await
        .unwrap();
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
          "base_image": "17.1",
          "version": "0.6.0",
          "summary": {
            "total": 2,
            "hits": 0,
            "misses": 2,
            "uncacheable": 0
          },
          "seeds": [
            {
              "name": "a-first",
              "type": "sql-file",
              "status": "miss",
              "cache_image": "pg-ephemeral/main:5781853bea10d8ad4b43421bdd6ac0e85de950560b339282ec0674ac19412d02"
            },
            {
              "name": "b-second",
              "type": "sql-file",
              "status": "miss",
              "cache_image": "pg-ephemeral/main:5af8fd2f60959a383f825f1e025b5d62572724d4d6e7f29dc30ba32799e5c88a"
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
          "base_image": "17.1",
          "version": "0.6.0",
          "summary": {
            "total": 1,
            "hits": 0,
            "misses": 1,
            "uncacheable": 0
          },
          "seeds": [
            {
              "name": "run-migrations",
              "type": "command",
              "status": "miss",
              "cache_image": "pg-ephemeral/main:4b60324001451a60bc1e82bc6c386d8c51bdf0ba07af41fd20813cc9f9112619"
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

/// Parse a TOML config in-line and return the cache reference of each seed in the
/// `main` instance. Panics on any error.
async fn seed_references(toml: &str) -> Vec<String> {
    let instance_name = pg_ephemeral::InstanceName::MAIN;
    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();
    let backend = resolved.backend_selection.resolve().await.unwrap();
    let definition = resolved
        .instances
        .get(&instance_name)
        .unwrap()
        .definition(backend, &instance_name);
    definition
        .load_seeds(&instance_name)
        .await
        .unwrap()
        .iter_seeds()
        .map(|seed| seed.cache_status().reference().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn test_cache_status_key_script_on_command_seed() {
    let _backend = ociman::test_backend_setup!();

    let baseline = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.run-migrations]
        type = "command"
        command = "migrate"
        arguments = ["up"]

        [instances.main.seeds.run-migrations.cache]
        type = "key-script"
        script = "echo version-1"
    "#})
    .await;

    // Changing the key-script output invalidates the cache.
    let after_key_change = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.run-migrations]
        type = "command"
        command = "migrate"
        arguments = ["up"]

        [instances.main.seeds.run-migrations.cache]
        type = "key-script"
        script = "echo version-2"
    "#})
    .await;
    assert_ne!(after_key_change, baseline);

    // Changing command arguments also invalidates the cache, even though the
    // key-script output is unchanged. Regression guard for the bug where
    // key-script output used to replace rather than supplement the command hash.
    let after_args_change = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.run-migrations]
        type = "command"
        command = "migrate"
        arguments = ["down"]

        [instances.main.seeds.run-migrations.cache]
        type = "key-script"
        script = "echo version-1"
    "#})
    .await;
    assert_ne!(after_args_change, baseline);
}

#[tokio::test]
async fn test_cli_key_script_failure_reports_display() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("cli-key-script-failure-display-test");

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
            script = "exit 1"
        "#},
    );

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");
    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(["cache", "status"])
        .working_directory(&dir.path)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    assert!(!output.status.success());

    // main() must print thiserror's Display-formatted source chain, not the Debug tuple-variant dump.
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr,
        indoc::indoc! {"
            Error: Failed to load seed run-migrations: cache key script failed
              caused by: command exited with exit status: 1
        "},
    );
}

#[tokio::test]
async fn test_cache_status_key_script_failure_propagates() {
    let _backend = ociman::test_backend_setup!();

    let instance_name = pg_ephemeral::InstanceName::MAIN;
    let resolved = pg_ephemeral::Config::load_toml(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.run-migrations]
        type = "command"
        command = "migrate"
        arguments = ["up"]

        [instances.main.seeds.run-migrations.cache]
        type = "key-script"
        script = "exit 1"
    "#})
    .unwrap()
    .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
    .unwrap();
    let backend = resolved.backend_selection.resolve().await.unwrap();
    let definition = resolved
        .instances
        .get(&instance_name)
        .unwrap()
        .definition(backend, &instance_name);

    let error = definition.load_seeds(&instance_name).await.unwrap_err();

    assert!(
        matches!(error, pg_ephemeral::LoadError::KeyScript { .. }),
        "expected LoadError::KeyScript, got: {error:?}"
    );
}

#[tokio::test]
async fn test_cache_status_key_script_on_script_seed() {
    let _backend = ociman::test_backend_setup!();

    let baseline = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.seed-data]
        type = "script"
        script = "psql -c 'SELECT 1'"

        [instances.main.seeds.seed-data.cache]
        type = "key-script"
        script = "echo version-1"
    "#})
    .await;

    // Changing the key-script output invalidates the cache.
    let after_key_change = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.seed-data]
        type = "script"
        script = "psql -c 'SELECT 1'"

        [instances.main.seeds.seed-data.cache]
        type = "key-script"
        script = "echo version-2"
    "#})
    .await;
    assert_ne!(after_key_change, baseline);

    // Changing the script body also invalidates the cache, even though the
    // key-script output is unchanged.
    let after_script_change = seed_references(indoc::indoc! {r#"
        image = "17.1"

        [instances.main.seeds.seed-data]
        type = "script"
        script = "psql -c 'SELECT 2'"

        [instances.main.seeds.seed-data.cache]
        type = "key-script"
        script = "echo version-1"
    "#})
    .await;
    assert_ne!(after_script_change, baseline);
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
    assert!(output["seeds"][0]["cache_image"].is_string());
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
    definition.populate_cache(&loaded_seeds).await.unwrap();

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
        pg_ephemeral::SeedCacheConfig::CommandHash,
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

    let definition = common::test_definition(backend, "stale-connection".parse().unwrap());

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

async fn run_cli(args: &[&str], current_dir: &std::path::Path) -> (Option<i32>, String, String) {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(args)
        .working_directory(current_dir)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    (
        output.status.code(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

async fn cleanup_cache_images(
    backend: &ociman::Backend,
    instance_name: &pg_ephemeral::InstanceName,
) {
    let name: ociman::reference::Name = format!("localhost/pg-ephemeral/{instance_name}")
        .parse()
        .unwrap();
    for reference in backend.image_references_by_name(&name).await {
        backend.remove_image_force(&reference).await;
    }
}

#[tokio::test]
async fn test_cache_credentials_default_seed() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "credentials-default-test".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;

    let dir = TestDir::new("credentials-default-test");
    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    dir.write_file(
        "database.toml",
        &indoc::formatdoc! {r#"
            image = "17.1"
            wait_available_timeout = "30s"

            [instances.{instance_name}.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    run_pg_ephemeral(
        &["cache", "--instance", instance_name.as_ref(), "populate"],
        &dir.path,
    )
    .await;

    let (code, stdout, stderr) = run_cli(
        &["cache", "--instance", instance_name.as_ref(), "credentials"],
        &dir.path,
    )
    .await;

    let actual: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let password = actual["superuser"]["password"]
        .as_str()
        .expect("password missing")
        .to_string();
    let cache_image = format!(
        "pg-ephemeral/{instance_name}:8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27",
    );

    assert_eq!(
        (code, actual, stderr),
        (
            Some(0),
            serde_json::json!({
                "cache_image": cache_image,
                "superuser": {
                    "user": "postgres",
                    "database": "postgres",
                    "password": password,
                },
            }),
            String::new(),
        ),
    );

    cleanup_cache_images(&backend, &instance_name).await;
}

#[tokio::test]
async fn test_cache_credentials_explicit_seed_name() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "credentials-explicit-test".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;

    let dir = TestDir::new("credentials-explicit-test");
    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    dir.write_file(
        "database.toml",
        &indoc::formatdoc! {r#"
            image = "17.1"
            wait_available_timeout = "30s"

            [instances.{instance_name}.seeds.schema]
            type = "sql-file"
            path = "schema.sql"

            [instances.{instance_name}.seeds.fixtures]
            type = "sql-statement"
            statement = "INSERT INTO users (id) VALUES (1)"
        "#},
    );

    run_pg_ephemeral(
        &["cache", "--instance", instance_name.as_ref(), "populate"],
        &dir.path,
    )
    .await;

    // Ask for the FIRST seed by name; the cache_image must be the schema-only
    // hash, not the deeper fixtures-applied hash.
    let (code, stdout, stderr) = run_cli(
        &[
            "cache",
            "--instance",
            instance_name.as_ref(),
            "credentials",
            "--seed-name",
            "schema",
        ],
        &dir.path,
    )
    .await;

    let actual: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let password = actual["superuser"]["password"]
        .as_str()
        .expect("password missing")
        .to_string();
    let cache_image = format!(
        "pg-ephemeral/{instance_name}:8732e9629a0030d0773d6b56db16cd6b9eb1b639bef6874f7c18df6094929c27",
    );

    assert_eq!(
        (code, actual, stderr),
        (
            Some(0),
            serde_json::json!({
                "cache_image": cache_image,
                "superuser": {
                    "user": "postgres",
                    "database": "postgres",
                    "password": password,
                },
            }),
            String::new(),
        ),
    );

    cleanup_cache_images(&backend, &instance_name).await;
}

#[tokio::test]
async fn test_cache_credentials_no_seeds() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("credentials-no-seeds-test");
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main]
        "#},
    );

    let actual = run_cli(&["cache", "credentials"], &dir.path).await;
    assert_eq!(
        actual,
        (
            Some(1),
            String::new(),
            "Error: Instance main has no seeds; cache credentials requires a cacheable seed\n"
                .to_string(),
        ),
    );
}

#[tokio::test]
async fn test_cache_credentials_uncacheable_tip() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("credentials-uncacheable-test");
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.tip]
            type = "script"
            script = "true"
            cache = { type = "none" }
        "#},
    );

    let actual = run_cli(&["cache", "credentials"], &dir.path).await;
    assert_eq!(
        actual,
        (
            Some(1),
            String::new(),
            "Error: Seed tip on instance main is uncacheable; cache credentials requires a cacheable seed\n"
                .to_string(),
        ),
    );
}

#[tokio::test]
async fn test_cache_credentials_unknown_seed() {
    let _backend = ociman::test_backend_setup!();
    let dir = TestDir::new("credentials-unknown-seed-test");
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

    let actual = run_cli(
        &["cache", "credentials", "--seed-name", "does-not-exist"],
        &dir.path,
    )
    .await;
    assert_eq!(
        actual,
        (
            Some(1),
            String::new(),
            "Error: Instance main has no seed named does-not-exist\n".to_string(),
        ),
    );
}

#[tokio::test]
async fn test_cache_credentials_miss_tip() {
    let backend = ociman::test_backend_setup!();
    let instance_name: pg_ephemeral::InstanceName = "credentials-miss-test".parse().unwrap();
    cleanup_cache_images(&backend, &instance_name).await;

    let dir = TestDir::new("credentials-miss-test");
    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    dir.write_file(
        "database.toml",
        &indoc::formatdoc! {r#"
            image = "17.1"

            [instances.{instance_name}.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let actual = run_cli(
        &["cache", "--instance", instance_name.as_ref(), "credentials"],
        &dir.path,
    )
    .await;
    assert_eq!(
        actual,
        (
            Some(1),
            String::new(),
            format!(
                "Error: Seed schema on instance {instance_name} is not yet cached; run `pg-ephemeral cache populate` first\n",
            ),
        ),
    );
}
