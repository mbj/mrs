//! Verifies base feature surface: image pulls, container boot with and without
//! generated SSL, the `pg_env` / `database_url` shape exposed to spawned
//! children, and the `Config` TOML parser across single-instance, multi-instance,
//! seed-variant, SSL, image-digest, and image-error inputs.

use libtest_mimic::{Failed, Trial};

use super::common::TestDir;

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![
        Trial::test("pull_test_images", pull_test_images),
        Trial::test("base_feature", base_feature),
        Trial::test("ssl_generated", ssl_generated),
        Trial::test("config_file", config_file),
        Trial::test(
            "config_file_no_explicit_instance",
            config_file_no_explicit_instance,
        ),
        Trial::test("config_ssl", config_ssl),
        Trial::test("run_env", run_env),
        Trial::test("config_seeds_basic", config_seeds_basic),
        Trial::test("config_seeds_command", config_seeds_command),
        Trial::test("config_seeds_script", config_seeds_script),
        Trial::test("config_seeds_mixed", config_seeds_mixed),
        Trial::test(
            "config_seeds_preserve_declaration_order",
            config_seeds_preserve_declaration_order,
        ),
        Trial::test("config_seeds_duplicate_name", config_seeds_duplicate_name),
        Trial::test(
            "config_seeds_with_git_revision",
            config_seeds_with_git_revision,
        ),
        Trial::test(
            "config_image_with_sha256_digest",
            config_image_with_sha256_digest,
        ),
        Trial::test("config_invalid_image_format", config_invalid_image_format),
        Trial::test(
            "config_invalid_image_nom_error",
            config_invalid_image_nom_error,
        ),
    ]
}

fn pull_test_images() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        let default_image: ociman::image::Reference = (&crate::Image::default()).into();
        backend.pull_image(&default_image).await;

        for image in [
            &*super::common::POSTGRES_IMAGE,
            &*super::common::RUBY_IMAGE,
            &*super::common::NODE_IMAGE,
            &*ociman::testing::ALPINE_LATEST_IMAGE,
        ] {
            backend.pull_image(image).await;
        }

        Ok(())
    })
}

fn base_feature() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        super::common::test_definition(backend)
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let row = sqlx::query("SELECT true")
                            .fetch_one(connection)
                            .await
                            .unwrap();
                        assert!(sqlx::Row::get::<bool, usize>(&row, 0));
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn ssl_generated() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        super::common::test_definition(backend)
            .ssl_config(crate::definition::SslConfig::Generated {
                hostname: "postgresql.example.com".parse().unwrap(),
            })
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let row = sqlx::query("SELECT true")
                            .fetch_one(connection)
                            .await
                            .unwrap();
                        assert!(sqlx::Row::get::<bool, usize>(&row, 0));
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

const DATABASE_TOML: &str = include_str!("../../../tests/database.toml");
const DATABASE_NO_EXPLICIT_INSTANCE_TOML: &str =
    include_str!("../../../tests/database_no_explicit_instance.toml");

fn config_file() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let dir = TestDir::new("config-file");
        dir.write_file("database.toml", DATABASE_TOML);
        let path = dir.path.join("database.toml");

        assert_eq!(
            crate::InstanceMap::from([
                (
                    crate::InstanceName::from_static_or_panic("a"),
                    crate::Instance {
                        application_name: None,
                        backend: ociman::backend::Selection::Docker,
                        database: pg_client::Database::POSTGRES,
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "17.1".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                ),
                (
                    crate::InstanceName::from_static_or_panic("b"),
                    crate::Instance {
                        application_name: None,
                        backend: ociman::backend::Selection::Podman,
                        database: pg_client::Database::POSTGRES,
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "17.2".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                )
            ]),
            crate::Config::load_toml_file(&path, &crate::config::InstanceDefinition::empty())
                .unwrap()
        );

        assert_eq!(
            crate::InstanceMap::from([
                (
                    crate::InstanceName::from_static_or_panic("a"),
                    crate::Instance {
                        application_name: None,
                        backend: ociman::backend::Selection::Docker,
                        database: pg_client::Database::POSTGRES,
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "18.0".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                ),
                (
                    crate::InstanceName::from_static_or_panic("b"),
                    crate::Instance {
                        application_name: None,
                        backend: ociman::backend::Selection::Docker,
                        database: pg_client::Database::POSTGRES,
                        seeds: indexmap::IndexMap::new(),
                        ssl_config: None,
                        superuser: pg_client::User::POSTGRES,
                        image: "18.0".parse().unwrap(),
                        cross_container_access: false,
                        wait_available_timeout: std::time::Duration::from_secs(10),
                    }
                )
            ]),
            crate::Config::load_toml_file(
                &path,
                &crate::config::InstanceDefinition {
                    backend: Some(ociman::backend::Selection::Docker),
                    image: Some("18.0".parse().unwrap()),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    wait_available_timeout: None,
                }
            )
            .unwrap()
        );

        Ok(())
    })
}

fn config_file_no_explicit_instance() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let dir = TestDir::new("config-file-no-explicit-instance");
        dir.write_file("database.toml", DATABASE_NO_EXPLICIT_INSTANCE_TOML);
        let path = dir.path.join("database.toml");

        assert_eq!(
            crate::InstanceMap::from([(
                crate::InstanceName::MAIN,
                crate::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::Database::POSTGRES,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::User::POSTGRES,
                    image: "17.1".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            ),]),
            crate::Config::load_toml_file(&path, &crate::config::InstanceDefinition::empty())
                .unwrap()
        );

        assert_eq!(
            crate::InstanceMap::from([(
                crate::InstanceName::MAIN,
                crate::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Podman,
                    database: pg_client::Database::POSTGRES,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::User::POSTGRES,
                    image: "18.0".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            ),]),
            crate::Config::load_toml_file(
                &path,
                &crate::config::InstanceDefinition {
                    backend: Some(ociman::backend::Selection::Podman),
                    image: Some("18.0".parse().unwrap()),
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    wait_available_timeout: None,
                }
            )
            .unwrap()
        );

        Ok(())
    })
}

fn config_ssl() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let config_str = indoc::indoc! {r#"
            backend = "docker"
            image = "18.0"

            [ssl_config]
            hostname = "postgresql.example.com"

            [instances.main]
        "#};

        assert_eq!(
            crate::InstanceMap::from([(
                crate::InstanceName::MAIN,
                crate::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::Database::POSTGRES,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: Some(crate::definition::SslConfig::Generated {
                        hostname: "postgresql.example.com".parse().unwrap(),
                    }),
                    superuser: pg_client::User::POSTGRES,
                    image: "18.0".parse().unwrap(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )]),
            crate::Config::load_toml(config_str)
                .unwrap()
                .instance_map(&crate::config::InstanceDefinition::empty())
                .unwrap()
        );

        Ok(())
    })
}

fn run_env() -> Result<(), Failed> {
    const DATABASE_URL: cmd_proc::EnvVariableName<'static> =
        cmd_proc::EnvVariableName::from_static_or_panic("DATABASE_URL");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        super::common::test_definition(backend)
            .with_container(async |container| {
                // Use sh -c to emit both PG* and DATABASE_URL
                let output = cmd_proc::Command::new("sh")
                    .argument("-c")
                    .argument("(env | grep '^PG' | sort) && echo DATABASE_URL=$DATABASE_URL")
                    .envs(container.pg_env())
                    .env(&DATABASE_URL, container.database_url())
                    .stdout_capture()
                    .stderr_capture()
                    .run()
                    .await
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
            .unwrap();

        Ok(())
    })
}

fn config_seeds_basic() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let expected_seeds: indexmap::IndexMap<crate::SeedName, crate::Seed> = [
            (
                "create-users-table".parse().unwrap(),
                crate::Seed::SqlFile {
                    path: "tests/fixtures/create_users.sql".into(),
                },
            ),
            (
                "insert-test-data".parse().unwrap(),
                crate::Seed::SqlFile {
                    path: "tests/fixtures/insert_users.sql".into(),
                },
            ),
        ]
        .into();

        assert_eq!(definition.seeds, expected_seeds);

        Ok(())
    })
}

fn config_seeds_command() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let expected_seeds: indexmap::IndexMap<crate::SeedName, crate::Seed> = [
            (
                "setup-schema".parse().unwrap(),
                crate::Seed::SqlFile {
                    path: "tests/fixtures/schema.sql".into(),
                },
            ),
            (
                "run-migration".parse().unwrap(),
                crate::Seed::Command {
                    command: crate::Command::new("migrate", ["up"]),
                    cache: crate::SeedCacheConfig::CommandHash,
                },
            ),
        ]
        .into();

        assert_eq!(definition.seeds, expected_seeds);

        Ok(())
    })
}

fn config_seeds_script() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let toml = indoc::indoc! {r#"
            backend = "docker"
            image = "17.1"

            [instances.main.seeds.initialize]
            type = "script"
            script = "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'"
        "#};

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let expected_seeds: indexmap::IndexMap<crate::SeedName, crate::Seed> = [(
            "initialize".parse().unwrap(),
            crate::Seed::Script {
                script: "echo 'Starting setup' && psql -c 'CREATE TABLE test (id INT)'".to_string(),
                cache: crate::SeedCacheConfig::CommandHash,
            },
        )]
        .into();

        assert_eq!(definition.seeds, expected_seeds);

        Ok(())
    })
}

fn config_seeds_mixed() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let expected_seeds: indexmap::IndexMap<crate::SeedName, crate::Seed> = [
            (
                "schema".parse().unwrap(),
                crate::Seed::SqlFile {
                    path: "tests/fixtures/schema.sql".into(),
                },
            ),
            (
                "migrate".parse().unwrap(),
                crate::Seed::Command {
                    command: crate::Command::new("migrate", ["up", "--verbose"]),
                    cache: crate::SeedCacheConfig::CommandHash,
                },
            ),
            (
                "verify".parse().unwrap(),
                crate::Seed::Script {
                    script: "psql -c 'SELECT COUNT(*) FROM users'".to_string(),
                    cache: crate::SeedCacheConfig::CommandHash,
                },
            ),
        ]
        .into();

        assert_eq!(definition.seeds, expected_seeds);

        Ok(())
    })
}

fn config_seeds_preserve_declaration_order() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let seed_names: Vec<&str> = definition.seeds.keys().map(|name| name.as_ref()).collect();

        assert_eq!(seed_names, vec!["z-first", "m-second", "a-third"]);

        Ok(())
    })
}

fn config_seeds_duplicate_name() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let error = crate::Config::load_toml(toml).unwrap_err();

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

        Ok(())
    })
}

fn config_seeds_with_git_revision() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
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

        let config = crate::Config::load_toml(toml)
            .unwrap()
            .instance_map(&crate::config::InstanceDefinition::empty())
            .unwrap();

        let definition = config.get(&crate::InstanceName::MAIN).unwrap();

        let expected_seeds: indexmap::IndexMap<crate::SeedName, crate::Seed> = [
            (
                "from-git".parse().unwrap(),
                crate::Seed::SqlFileGitRevision {
                    git_revision: "main".to_string(),
                    path: "tests/fixtures/schema.sql".into(),
                },
            ),
            (
                "from-filesystem".parse().unwrap(),
                crate::Seed::SqlFile {
                    path: "tests/fixtures/create_users.sql".into(),
                },
            ),
        ]
        .into();

        assert_eq!(definition.seeds, expected_seeds);

        Ok(())
    })
}

fn config_image_with_sha256_digest() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let config_str = indoc::indoc! {r#"
            backend = "docker"
            image = "17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

            [instances.main]
        "#};

        let expected_image: crate::Image =
            "17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse()
                .unwrap();

        assert_eq!(
            crate::InstanceMap::from([(
                crate::InstanceName::MAIN,
                crate::Instance {
                    application_name: None,
                    backend: ociman::backend::Selection::Docker,
                    database: pg_client::Database::POSTGRES,
                    seeds: indexmap::IndexMap::new(),
                    ssl_config: None,
                    superuser: pg_client::User::POSTGRES,
                    image: expected_image.clone(),
                    cross_container_access: false,
                    wait_available_timeout: std::time::Duration::from_secs(10),
                }
            )]),
            crate::Config::load_toml(config_str)
                .unwrap()
                .instance_map(&crate::config::InstanceDefinition::empty())
                .unwrap()
        );

        // Verify the ociman::image::Reference conversion includes the digest
        let reference: ociman::image::Reference = (&expected_image).into();
        assert_eq!(
            reference.to_string(),
            "registry.hub.docker.com/library/postgres:17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );

        Ok(())
    })
}

fn config_invalid_image_format() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let config_str = indoc::indoc! {r#"
            backend = "docker"
            image = "17.6@sha256:tooshort"

            [instances.main]
        "#};

        let error = crate::Config::load_toml(config_str)
            .unwrap_err()
            .to_string();

        let expected = indoc::indoc! {"
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

        Ok(())
    })
}

fn config_invalid_image_nom_error() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        // This tests an image format that triggers nom's detailed error with caret
        let config_str = indoc::indoc! {r#"
            backend = "docker"
            image = "INVALID"

            [instances.main]
        "#};

        let error = crate::Config::load_toml(config_str)
            .unwrap_err()
            .to_string();

        let expected = indoc::indoc! {"
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

        Ok(())
    })
}
