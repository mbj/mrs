use std::path::PathBuf;

use mmigration::{
    Config, Context, PendingMigration, QualifiedTableName, Schema, SchemaDump, SchemaNormalizer,
};
use pretty_assertions::assert_eq;

fn fixture_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

#[derive(Debug, PartialEq)]
struct NoopNormalizer;

impl SchemaNormalizer for NoopNormalizer {
    fn normalize(&self, schema: &Schema) -> Schema {
        schema.clone()
    }
}

struct StaticSchemaDump;

impl SchemaDump for StaticSchemaDump {
    async fn schema_dump(&self) -> Schema {
        "".into()
    }
}

#[tokio::test]
async fn find_pending_migrations_uses_database_last_applied_index() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            client_config
                .with_sqlx_connection(async |connection| {
                    sqlx::query("CREATE TABLE public.applied_migrations_test(index int8 PRIMARY KEY)")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::query(
                        "COMMENT ON TABLE public.applied_migrations_test IS 'Last applied migration: 0, init_schema'",
                    )
                    .execute(&mut *connection)
                    .await?;

                    Ok::<(), sqlx::Error>(())
                })
                .await
                .unwrap()
                .unwrap();

            let config = Config {
                migration_dir: fixture_dir("basic"),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: fixture_dir("basic").join("schema.sql"),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_test".to_string(),
                },
            };

            let context = Context::load(&config, client_config, StaticSchemaDump).unwrap();

            let actual: Vec<PendingMigration> = context
                .find_pending_migrations()
                .await
                .unwrap()
                .into_iter()
                .cloned()
                .collect();

            let expected = vec![PendingMigration {
                index: 1_u32.into(),
                name: "add_users".parse().unwrap(),
                raw_sql: "CREATE TABLE app.users (id bigint PRIMARY KEY);\n".into(),
            }];

            assert_eq!(expected, actual);
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn apply_pending_no_schema_dump_propagates_sql_error() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();
            let temp_dir = std::env::temp_dir().join(format!(
                "mmigration-invalid-sql-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&temp_dir).unwrap();

            let migration_path = temp_dir.join("0_bad.sql");
            std::fs::write(&migration_path, "SELECT FROM;\n").unwrap();

            let config = Config {
                migration_dir: temp_dir.clone(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: temp_dir.join("schema.sql"),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_invalid_sql".to_string(),
                },
            };

            let context = Context::load(&config, client_config, StaticSchemaDump).unwrap();

            let error = context.apply_pending_no_schema_dump().await.unwrap_err();

            assert!(matches!(
                error,
                mmigration::ContextError::Transaction(
                    mmigration::transaction::TransactionError::Sqlx(sqlx::Error::Database(_))
                )
            ));

            std::fs::remove_file(&migration_path).unwrap();
            std::fs::remove_dir(&temp_dir).unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn schema_dump_propagates_io_error() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();
            let base = std::env::temp_dir().join(format!(
                "mmigration-schema-dump-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&base).unwrap();

            let schema_path = base.join("missing").join("schema.sql");
            let config = Config {
                migration_dir: fixture_dir("basic"),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: schema_path.clone(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_schema_dump_error".to_string(),
                },
            };

            let context = Context::load(&config, client_config, StaticSchemaDump).unwrap();
            let error = context.schema_dump().await.unwrap_err();

            assert!(matches!(
                error,
                mmigration::ContextError::IoError {
                    operation,
                    path,
                    source
                } if operation == "write_schema_dump"
                    && path == schema_path
                    && source.kind() == std::io::ErrorKind::NotFound
            ));

            std::fs::remove_dir(&base).unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn create_new_pending_propagates_io_error() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();
            let base = std::env::temp_dir().join(format!(
                "mmigration-new-pending-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&base).unwrap();

            let migration_dir = base.join("missing").join("migrations");
            let config = Config {
                migration_dir: migration_dir.clone(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: base.join("schema.sql"),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_new_pending_error".to_string(),
                },
            };

            let context = Context::load(&config, client_config, StaticSchemaDump).unwrap();
            let migration_name: mmigration::MigrationName = "add_users".parse().unwrap();
            let error = context
                .create_new_pending(&migration_name)
                .await
                .unwrap_err();

            let expected_path = migration_dir.join("0_add_users.sql");
            assert!(matches!(
                error,
                mmigration::ContextError::IoError {
                    operation,
                    path,
                    source
                } if operation == "write_new_pending_migration"
                    && path == expected_path
                    && source.kind() == std::io::ErrorKind::NotFound
            ));

            std::fs::remove_dir(&base).unwrap();
        })
        .await
        .unwrap();
}
