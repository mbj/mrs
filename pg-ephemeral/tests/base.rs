mod common;

#[tokio::test]
async fn test_base_feature() {
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Image::default());

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
                    backend: pg_ephemeral::cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    steps: vec![],
                    superuser: pg_client::username!("postgres"),
                    image: "17.1".parse().unwrap()
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: pg_ephemeral::cbt::Backend::Podman,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    steps: vec![],
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
                    backend: pg_ephemeral::cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    steps: vec![],
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap()
                }
            ),
            (
                pg_ephemeral::InstanceName("b".to_string()),
                pg_ephemeral::Definition {
                    application_name: None,
                    backend: pg_ephemeral::cbt::Backend::Docker,
                    database: pg_client::database!("postgres"),
                    migration_config: None,
                    steps: vec![],
                    superuser: pg_client::username!("postgres"),
                    image: "18.0".parse().unwrap()
                }
            )
        ]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(pg_ephemeral::cbt::Backend::Docker),
                image: Some("18.0".parse().unwrap())
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
                backend: pg_ephemeral::cbt::Backend::Docker,
                database: pg_client::database!("postgres"),
                migration_config: None,
                steps: vec![],
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
                backend: pg_ephemeral::cbt::Backend::Podman,
                database: pg_client::database!("postgres"),
                migration_config: None,
                steps: vec![],
                superuser: pg_client::username!("postgres"),
                image: "18.0".parse().unwrap()
            }
        ),]),
        pg_ephemeral::Config::load_toml_file(
            "tests/database_no_explicit_instance.toml",
            &pg_ephemeral::config::InstanceDefinition {
                backend: Some(pg_ephemeral::cbt::Backend::Podman),
                image: Some("18.0".parse().unwrap())
            }
        )
        .unwrap()
    )
}

#[tokio::test]
async fn test_run_env() {
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Image::default());

    definition
        .with_container(async |container| {
            // Use sh -c to emit all PG* environment variables
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg("env | grep '^PG' | sort")
                .envs(container.pg_env())
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
            let expected = format!("{}\n", expected_lines.join("\n"));

            assert_eq!(
                expected, actual,
                "PG* environment variables mismatch.\nExpected:\n{}\nActual:\n{}",
                expected, actual
            );
        })
        .await
}

#[tokio::test]
async fn test_run_env_database_url() {
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Image::default());

    definition
        .with_container(async |container| {
            // Use sh -c to emit DATABASE_URL environment variable
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg("echo $DATABASE_URL")
                .env("DATABASE_URL", container.database_url())
                .output()
                .unwrap();

            let actual = String::from_utf8(output.stdout).unwrap().trim().to_string();
            let expected = container.database_url();

            assert_eq!(
                expected, actual,
                "DATABASE_URL mismatch.\nExpected: {}\nActual: {}",
                expected, actual
            );
        })
        .await
}

#[tokio::test]
async fn test_run_env_multiple_flavors() {
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Image::default());

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
                "Multiple flavors mismatch.\nExpected:\n{}\nActual:\n{}",
                expected, actual
            );
        })
        .await
}
