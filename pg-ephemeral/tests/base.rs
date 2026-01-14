mod common;

#[tokio::test]
async fn test_base_feature() {
    let backend = ociman::test_backend_setup!();

    common::test_definition(backend)
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
}

#[tokio::test]
async fn test_ssl_generated() {
    let backend = ociman::test_backend_setup!();

    common::test_definition(backend)
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
}

#[test]
fn test_config_file() {
    assert_eq!(
        pg_ephemeral::InstanceMap::from([
            (
                pg_ephemeral::InstanceName("a".to_string()),
                pg_ephemeral::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::database!("postgres"),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "17.1".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Podman,
                    database: pg_client::database!("postgres"),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "17.2".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )
        ]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database.toml",
            &pg_ephemeral::config::InstanceDefinition::empty()
        )
        .unwrap()
    );

    assert_eq!(
        pg_ephemeral::InstanceMap::from([
            (
                pg_ephemeral::InstanceName("a".to_string()),
                pg_ephemeral::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::database!("postgres"),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::database!("postgres"),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )
        ]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(ociman::backend::Selection::Docker),
                image: Some("18.0".parse().unwrap()),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                wait_available_timeout: None,
            }
        )
        .unwrap()
    )
}

#[test]
fn test_config_file_no_explicit_instance() {
    assert_eq!(
        pg_ephemeral::InstanceMap::from([(
            pg_ephemeral::InstanceName("main".to_string()),
            pg_ephemeral::Instance {
                application_name: None,
                backend: ociman::backend::Selection::Docker,
                database: pg_client::database!("postgres"),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                superuser: pg_client::username!("postgres"),
                image: "17.1".parse().unwrap(),
                cross_container_access: false,
                wait_available_timeout: std::time::Duration::from_secs(10),
            }
        ),]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database_no_explicit_instance.toml",
            &pg_ephemeral::config::InstanceDefinition::empty()
        )
        .unwrap()
    );

    assert_eq!(
        pg_ephemeral::InstanceMap::from([(
            pg_ephemeral::InstanceName("main".to_string()),
            pg_ephemeral::Instance {
                application_name: None,
                backend: ociman::backend::Selection::Podman,
                database: pg_client::database!("postgres"),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                superuser: pg_client::username!("postgres"),
                image: "18.0".parse().unwrap(),
                cross_container_access: false,
                wait_available_timeout: std::time::Duration::from_secs(10),
            }
        ),]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database_no_explicit_instance.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(ociman::backend::Selection::Podman),
                image: Some("18.0".parse().unwrap()),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                wait_available_timeout: None,
            }
        )
        .unwrap()
    )
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
        pg_ephemeral::InstanceMap::from([(
            pg_ephemeral::InstanceName("main".to_string()),
            pg_ephemeral::Instance {
                application_name: None,
                backend: ociman::backend::Selection::Docker,
                database: pg_client::database!("postgres"),
                seeds: indexmap::IndexMap::new(),
                ssl_config: Some(pg_ephemeral::definition::SslConfig::Generated {
                    hostname: "postgresql.example.com".parse().unwrap(),
                }),
                superuser: pg_client::username!("postgres"),
                image: "18.0".parse().unwrap(),
                cross_container_access: false,
                wait_available_timeout: std::time::Duration::from_secs(10),
            }
        )]),
        pg_ephemeral::Config::load_toml(config_str)
            .unwrap()
            .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
            .unwrap()
    )
}

#[tokio::test]
async fn test_run_env() {
    let backend = ociman::test_backend_setup!();

    common::test_definition(backend)
        .with_container(async |container| {
            // Use sh -c to emit both PG* and DATABASE_URL
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg("(env | grep '^PG' | sort) && echo DATABASE_URL=$DATABASE_URL")
                .envs(container.pg_env())
                .env("DATABASE_URL", container.database_url())
                .output()
                .unwrap();

            let actual = String::from_utf8(output.stdout).unwrap();

            // Generate expected output from config
            let pg_env = container.pg_env();
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

    let config = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = config
        .get(&pg_ephemeral::InstanceName("main".to_string()))
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

    let config = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = config
        .get(&pg_ephemeral::InstanceName("main".to_string()))
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
                cache: pg_ephemeral::CommandCacheConfig::CommandHash,
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

    let config = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = config
        .get(&pg_ephemeral::InstanceName("main".to_string()))
        .unwrap();

    let expected_seeds: indexmap::IndexMap<pg_ephemeral::SeedName, pg_ephemeral::Seed> = [(
        "initialize".parse().unwrap(),
        pg_ephemeral::Seed::Script {
            script: "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'".to_string(),
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

    let config = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = config
        .get(&pg_ephemeral::InstanceName("main".to_string()))
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
                cache: pg_ephemeral::CommandCacheConfig::CommandHash,
            },
        ),
        (
            "verify".parse().unwrap(),
            pg_ephemeral::Seed::Script {
                script: "psql -c 'SELECT COUNT(*) FROM users'".to_string(),
            },
        ),
    ]
    .into();

    assert_eq!(definition.seeds, expected_seeds);
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

    let config = pg_ephemeral::Config::load_toml(toml)
        .unwrap()
        .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
        .unwrap();

    let definition = config
        .get(&pg_ephemeral::InstanceName("main".to_string()))
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
        pg_ephemeral::InstanceMap::from([(
            pg_ephemeral::InstanceName("main".to_string()),
            pg_ephemeral::Instance {
                application_name: None,
                backend: ociman::backend::Selection::Docker,
                database: pg_client::database!("postgres"),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                superuser: pg_client::username!("postgres"),
                image: expected_image.clone(),
                cross_container_access: false,
                wait_available_timeout: std::time::Duration::from_secs(10),
            }
        )]),
        pg_ephemeral::Config::load_toml(config_str)
            .unwrap()
            .instance_map(&pg_ephemeral::config::InstanceDefinition::empty())
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
