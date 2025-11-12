mod common;

#[tokio::test]
async fn test_base_feature() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(
        pg_ephemeral::BackendSelection::Auto,
        pg_ephemeral::Image::default(),
    );

    definition
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
    env_logger::init();

    if cbt::testing::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(
        pg_ephemeral::BackendSelection::Auto,
        pg_ephemeral::Image::default(),
    )
    .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
        hostname: "postgresql.example.com".parse().unwrap(),
    });

    definition
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
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "17.1".parse().unwrap()
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: cbt::Backend::Podman,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "17.2".parse().unwrap()
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
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap()
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap()
                }
            )
        ]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(cbt::Backend::Docker),
                image: Some("18.0".parse().unwrap()),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
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
            pg_ephemeral::Definition {
                application_name: None,
                backend: cbt::Backend::Docker,
                database: pg_client::database!("postgres"),
                migration_config: None,
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                superuser: pg_client::username!("postgres"),
                image: "17.1".parse().unwrap()
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
            pg_ephemeral::Definition {
                application_name: None,
                backend: cbt::Backend::Podman,
                database: pg_client::database!("postgres"),
                migration_config: None,
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
                superuser: pg_client::username!("postgres"),
                image: "18.0".parse().unwrap()
            }
        ),]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database_no_explicit_instance.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(cbt::Backend::Podman),
                image: Some("18.0".parse().unwrap()),
                seeds: indexmap::IndexMap::new(),
                ssl_config: None,
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
            pg_ephemeral::Definition {
                application_name: None,
                backend: cbt::Backend::Docker,
                database: pg_client::database!("postgres"),
                migration_config: None,
                seeds: indexmap::IndexMap::new(),
                ssl_config: Some(pg_ephemeral::definition::SslConfig::Generated {
                    hostname: "postgresql.example.com".parse().unwrap(),
                }),
                superuser: pg_client::username!("postgres"),
                image: "18.0".parse().unwrap(),
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
    if cbt::testing::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(
        pg_ephemeral::BackendSelection::Auto,
        pg_ephemeral::Image::default(),
    );

    definition
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
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();
            expected_lines.sort();
            expected_lines.push(format!("DATABASE_URL={}", container.database_url()));
            let expected = format!("{}\n", expected_lines.join("\n"));

            assert_eq!(
                expected, actual,
                "Environment variables mismatch.\nExpected:\n{}\nActual:\n{}",
                expected, actual
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
            pg_ephemeral::Seed::SqlFile("tests/fixtures/create_users.sql".into()),
        ),
        (
            "insert-test-data".parse().unwrap(),
            pg_ephemeral::Seed::SqlFile("tests/fixtures/insert_users.sql".into()),
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
            pg_ephemeral::Seed::SqlFile("tests/fixtures/schema.sql".into()),
        ),
        (
            "run-migration".parse().unwrap(),
            pg_ephemeral::Seed::Command(pg_ephemeral::Command::new(
                "migrate".to_string(),
                vec!["up".to_string()],
            )),
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
        pg_ephemeral::Seed::Script(
            "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'".to_string(),
        ),
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
            pg_ephemeral::Seed::SqlFile("tests/fixtures/schema.sql".into()),
        ),
        (
            "migrate".parse().unwrap(),
            pg_ephemeral::Seed::Command(pg_ephemeral::Command::new(
                "migrate".to_string(),
                vec!["up".to_string(), "--verbose".to_string()],
            )),
        ),
        (
            "verify".parse().unwrap(),
            pg_ephemeral::Seed::Script("psql -c 'SELECT COUNT(*) FROM users'".to_string()),
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
