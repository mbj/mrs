mod common;

#[tokio::test]
async fn pull_test_images() {
    let backend = ociman::test_backend_setup!();

    let default_image: ociman::image::Reference = (&pg_ephemeral::Image::default()).into();
    backend.pull_image(&default_image).await.unwrap();

    for image in [
        &*common::POSTGRES_IMAGE,
        &*common::RUBY_IMAGE,
        &*common::NODE_IMAGE,
        &*ociman::testing::ALPINE_LATEST_IMAGE,
    ] {
        backend.pull_image(image).await.unwrap();
    }
}

#[tokio::test]
async fn test_base_feature() {
    let backend = ociman::test_backend_setup!();

    common::test_definition(backend, "base-feature".parse().unwrap())
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row = sqlx::query("SELECT true")
                        .fetch_one(connection)
                        .await
                        .unwrap();
                    assert!(sqlx::Row::get::<bool, usize>(&row, 0))
                })
                .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_ssl_generated() {
    let backend = ociman::test_backend_setup!();

    common::test_definition(backend, "ssl-generated".parse().unwrap())
        .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
            hostname: "postgresql.example.com".parse().unwrap(),
        })
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row = sqlx::query("SELECT true")
                        .fetch_one(connection)
                        .await
                        .unwrap();
                    assert!(sqlx::Row::get::<bool, usize>(&row, 0))
                })
                .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_ssl_parameter_conflict_rejected() {
    let backend = ociman::test_backend_setup!();

    let mut definition =
        common::test_definition(backend, "ssl-parameter-conflict".parse().unwrap()).ssl_config(
            pg_ephemeral::definition::SslConfig::Generated {
                hostname: "postgresql.example.com".parse().unwrap(),
            },
        );
    definition.parameters.insert(
        pg_client::parameter::Name::from_static_or_panic("ssl"),
        pg_client::parameter::Value::from_static_or_panic("off"),
    );

    let result = definition.with_container(async |_| {}).await;

    match result {
        Err(pg_ephemeral::container::Error::ParameterConflictsWithSslConfig { name }) => {
            assert_eq!(name.as_str(), "ssl");
        }
        other => panic!("expected ParameterConflictsWithSslConfig, got: {other:?}"),
    }
}

#[test]
fn test_config_multi_instance() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.a]

        [instances.b]
        image = "17.2"
    "#};

    assert_eq!(
        pg_ephemeral::config::Resolved {
            backend_selection: ociman::backend::Selection::Docker,
            instances: pg_ephemeral::InstanceMap::from([
                (
                    pg_ephemeral::InstanceName::from_static_or_panic("a"),
                    pg_ephemeral::Instance {
                        application_name: None,
                        database: pg_client::Database::POSTGRES,
                        parameters: pg_client::parameter::Map::new(),
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "17.1".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                ),
                (
                    pg_ephemeral::InstanceName::from_static_or_panic("b"),
                    pg_ephemeral::Instance {
                        application_name: None,
                        database: pg_client::Database::POSTGRES,
                        parameters: pg_client::parameter::Map::new(),
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "17.2".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                )
            ]),
        },
        pg_ephemeral::Config::load_toml(toml)
            .unwrap()
            .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
            .unwrap()
    );
}

#[test]
fn test_config_no_explicit_instance() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"
    "#};

    assert_eq!(
        pg_ephemeral::config::Resolved {
            backend_selection: ociman::backend::Selection::Docker,
            instances: pg_ephemeral::InstanceMap::from([(
                pg_ephemeral::InstanceName::MAIN,
                pg_ephemeral::Instance {
                    application_name: None,
                    database: pg_client::Database::POSTGRES,
                    parameters: pg_client::parameter::Map::new(),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::User::POSTGRES,
                    image: "17.1".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            ),]),
        },
        pg_ephemeral::Config::load_toml(toml)
            .unwrap()
            .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
            .unwrap()
    );
}

#[test]
fn test_config_backend_overwrite_wins_over_toml() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(
            Some(ociman::backend::Selection::Podman),
            &pg_ephemeral::config::InstanceDefinition::empty(),
        )
        .unwrap();

    assert_eq!(
        resolved.backend_selection,
        ociman::backend::Selection::Podman
    );
}

#[test]
fn test_config_ssl() {
    use indoc::indoc;

    let config_str = indoc! {r#"
        backend = "docker"
        image = "18.0"

        [ssl_config]
        hostname = "postgresql.example.com"

        [instances.main]
    "#};

    assert_eq!(
        pg_ephemeral::config::Resolved {
            backend_selection: ociman::backend::Selection::Docker,
            instances: pg_ephemeral::InstanceMap::from([(
                pg_ephemeral::InstanceName::MAIN,
                pg_ephemeral::Instance {
                    application_name: None,
                    database: pg_client::Database::POSTGRES,
                    parameters: pg_client::parameter::Map::new(),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: Some(pg_ephemeral::definition::SslConfig::Generated {
                        hostname: "postgresql.example.com".parse().unwrap(),
                    }),
                    superuser: pg_client::User::POSTGRES,
                    image: "18.0".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )]),
        },
        pg_ephemeral::Config::load_toml(config_str)
            .unwrap()
            .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
            .unwrap()
    )
}

#[tokio::test]
async fn test_run_env() {
    const DATABASE_URL: cmd_proc::EnvVariableName =
        cmd_proc::EnvVariableName::from_static_or_panic("DATABASE_URL");

    let backend = ociman::test_backend_setup!();

    common::test_definition(backend, "run-env".parse().unwrap())
        .with_container(async |container| {
            // Use sh -c to emit both PG* and DATABASE_URL
            let output = cmd_proc::Command::new("sh")
                .argument("-c")
                .argument("(env | grep '^PG' | sort) && echo DATABASE_URL=$DATABASE_URL")
                .envs(container.pg_env().unwrap())
                .env(
                    &DATABASE_URL,
                    container
                        .database_url()
                        .parse::<cmd_proc::EnvVariableValue>()
                        .unwrap(),
                )
                .stdout_capture()
                .stderr_capture()
                .run()
                .await
                .unwrap();

            let actual = String::from_utf8(output.stdout).unwrap();

            // Generate expected output from config
            let pg_env = container.pg_env().unwrap();
            let mut expected_lines: Vec<String> = pg_env
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect();
            expected_lines.sort();
            expected_lines.push(format!("DATABASE_URL={}", container.database_url()));
            let expected = format!("{}\n", expected_lines.join("\n"));

            assert_eq!(
                expected, actual,
                "Environment variables mismatch.\nExpected:\n{expected}\nActual:\n{actual}"
            );
        })
        .await
        .unwrap()
}

#[test]
fn test_config_seeds_basic() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.create-users-table]
        type = "sql-file"
        path = "tests/fixtures/create_users.sql"

        [instances.main.seeds.insert-test-data]
        type = "sql-file"
        path = "tests/fixtures/insert_users.sql"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [
        (
            "create-users-table".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile {
                path: "tests/fixtures/create_users.sql".into(),
            },
        ),
        (
            "insert-test-data".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile {
                path: "tests/fixtures/insert_users.sql".into(),
            },
        ),
    ]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
}

#[test]
fn test_config_seeds_command() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.setup-schema]
        type = "sql-file"
        path = "tests/fixtures/schema.sql"

        [instances.main.seeds.run-migration]
        type = "command"
        command = "migrate"
        arguments = ["up"]
        cache.type = "command-hash"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [
        (
            "setup-schema".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile {
                path: "tests/fixtures/schema.sql".into(),
            },
        ),
        (
            "run-migration".parse().unwrap(),
            pg_ephemeral::Seed::Command {
                command: pg_ephemeral::Command::new("migrate", ["up"]),
                cache: pg_ephemeral::SeedCacheConfig::CommandHash,
            },
        ),
    ]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
}

#[test]
fn test_config_seeds_script() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.initialize]
        type = "script"
        script = "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [(
        "initialize".parse().unwrap(),
        pg_ephemeral::Seed::Script {
            script: "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'".to_string(),
            cache: pg_ephemeral::SeedCacheConfig::CommandHash,
        },
    )]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
}

#[test]
fn test_config_seeds_mixed() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.schema]
        type = "sql-file"
        path = "tests/fixtures/schema.sql"

        [instances.main.seeds.migrate]
        type = "command"
        command = "migrate"
        arguments = ["up", "--verbose"]
        cache.type = "command-hash"

        [instances.main.seeds.verify]
        type = "script"
        script = "psql -c 'SELECT COUNT(*) FROM users'"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [
        (
            "schema".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile {
                path: "tests/fixtures/schema.sql".into(),
            },
        ),
        (
            "migrate".parse().unwrap(),
            pg_ephemeral::Seed::Command {
                command: pg_ephemeral::Command::new("migrate", ["up", "--verbose"]),
                cache: pg_ephemeral::SeedCacheConfig::CommandHash,
            },
        ),
        (
            "verify".parse().unwrap(),
            pg_ephemeral::Seed::Script {
                script: "psql -c 'SELECT COUNT(*) FROM users'".to_string(),
                cache: pg_ephemeral::SeedCacheConfig::CommandHash,
            },
        ),
    ]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
}

#[test]
fn test_config_seeds_preserve_declaration_order() {
    // Seed names are intentionally in reverse-alphabetic order so that a
    // declaration-order-preserving parser produces [z, m, a] while a
    // TOML parser that materializes tables through a sorted map produces
    // [a, m, z]. `IndexMap::PartialEq` is order-insensitive, so we compare
    // the key sequence directly via iter().
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.z-first]
        type = "sql-file"
        path = "first.sql"

        [instances.main.seeds.m-second]
        type = "sql-file"
        path = "second.sql"

        [instances.main.seeds.a-third]
        type = "sql-file"
        path = "third.sql"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let seed_names: Vec<&str> = definition.seeds.keys().map(|name| name.as_ref()).collect();

    assert_eq!(seed_names, vec!["z-first", "m-second", "a-third"]);
}

#[test]
fn test_config_seeds_duplicate_name() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.duplicate]
        type = "sql-file"
        path = "first.sql"

        [instances.main.seeds.duplicate]
        type = "sql-file"
        path = "second.sql"
    "#};

    let error = pg_ephemeral::Config::load_toml(toml).unwrap_err();

    assert_eq!(
        error.to_string(),
        indoc::indoc! {"
            Decoding as toml failed: TOML parse error at line 8, column 23
              |
            8 | [instances.main.seeds.duplicate]
              |                       ^^^^^^^^^
            duplicate key
        "}
    );
}

#[test]
fn test_config_seeds_with_git_revision() {
    let toml = indoc::indoc! {r#"
        backend = "docker"
        image = "17.1"

        [instances.main.seeds.from-git]
        type = "sql-file"
        path = "tests/fixtures/schema.sql"
        git_revision = "main"

        [instances.main.seeds.from-filesystem]
        type = "sql-file"
        path = "tests/fixtures/create_users.sql"
    "#};

    let resolved = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = resolved
        .instances
        .get(&pg_ephemeral::InstanceName::MAIN)
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [
        (
            "from-git".parse().unwrap(),
            pg_ephemeral::Seed::SqlFileGitRevision {
                git_revision: "main".to_string(),
                path: "tests/fixtures/schema.sql".into(),
            },
        ),
        (
            "from-filesystem".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile {
                path: "tests/fixtures/create_users.sql".into(),
            },
        ),
    ]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
}

#[test]
fn test_config_image_with_sha256_digest() {
    use indoc::indoc;

    let config_str = indoc! {r#"
        backend = "docker"
        image = "17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

        [instances.main]
    "#};

    let expected_image: pg_ephemeral::Image =
        "17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .parse()
            .unwrap();

    assert_eq!(
        pg_ephemeral::config::Resolved {
            backend_selection: ociman::backend::Selection::Docker,
            instances: pg_ephemeral::InstanceMap::from([(
                pg_ephemeral::InstanceName::MAIN,
                pg_ephemeral::Instance {
                    application_name: None,
                    database: pg_client::Database::POSTGRES,
                    parameters: pg_client::parameter::Map::new(),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::User::POSTGRES,
                    image: expected_image.clone(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )]),
        },
        pg_ephemeral::Config::load_toml(config_str)
            .unwrap()
            .resolve(None, &pg_ephemeral::config::InstanceDefinition::empty())
            .unwrap()
    );

    // Verify the ociman::image::Reference conversion includes the digest
    let reference: ociman::image::Reference = (&expected_image).into();
    assert_eq!(
        reference.to_string(),
        "registry.hub.docker.com/library/postgres:17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
}

#[test]
fn test_config_invalid_image_format() {
    use indoc::indoc;

    let config_str = indoc! {r#"
        backend = "docker"
        image = "17.6@sha256:tooshort"

        [instances.main]
    "#};

    let error = pg_ephemeral::Config::load_toml(config_str)
        .unwrap_err()
        .to_string();

    let expected = indoc! {"
        Decoding as toml failed: TOML parse error at line 2, column 9
          |
        2 | image = \"17.6@sha256:tooshort\"
          |         ^^^^^^^^^^^^^^^^^^^^^^
        0: at line 1, in TakeWhileMN:
        17.6@sha256:tooshort
                    ^

        1: at line 1, in digest:
        17.6@sha256:tooshort
            ^

        2: at line 1, in official release image:
        17.6@sha256:tooshort
        ^


    "};

    assert_eq!(error, expected);
}

#[test]
fn test_config_invalid_image_nom_error() {
    use indoc::indoc;

    // This tests an image format that triggers nom's detailed error with caret
    let config_str = indoc! {r#"
        backend = "docker"
        image = "INVALID"

        [instances.main]
    "#};

    let error = pg_ephemeral::Config::load_toml(config_str)
        .unwrap_err()
        .to_string();

    let expected = indoc! {"
        Decoding as toml failed: TOML parse error at line 2, column 9
          |
        2 | image = \"INVALID\"
          |         ^^^^^^^^^
        0: at line 1, in TakeWhileMN:
        INVALID
        ^

        1: at line 1, in OS name:
        INVALID
        ^

        2: at line 1, in OS-only image:
        INVALID
        ^

        3: at line 1, in Alt:
        INVALID
        ^


    "};

    assert_eq!(error, expected);
}
