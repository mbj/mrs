#[tokio::test]
async fn test_base_feature() {
    if platform_not_supported() {
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

fn platform_not_supported() -> bool {
    std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok()
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
