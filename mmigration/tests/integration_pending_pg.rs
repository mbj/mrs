use std::path::PathBuf;

use mmigration::{
    Config, Context, DefinedMigrations, PendingMigration, QualifiedTableName, Schema, SchemaDump,
    SchemaNormalizer,
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
                        "COMMENT ON TABLE public.applied_migrations_test IS 'Last applied migration: 1, init_schema'",
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
                index: 2_u32.into(),
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

            let migration_path = temp_dir.join("1_bad.sql");
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

            context.bootstrap().await.unwrap();

            let error = context.apply_pending_no_schema_dump().await.unwrap_err();

            assert!(matches!(
                error,
                mmigration::ContextError::ApplyMigration {
                    source: sqlx::Error::Database(_),
                    ..
                }
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

            let expected_path = migration_dir.join("1_add_users.sql");
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

#[tokio::test]
async fn apply_pending_keeps_committed_migrations_when_a_later_one_fails() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            // Two pending migrations built in memory: index 0 applies cleanly and has
            // an observable effect; index 1 makes a change then fails. Per-migration
            // transactions mean 0 must stay committed while 1 is rolled back.
            let mut defined_migrations = DefinedMigrations::new();
            defined_migrations
                .add(PendingMigration {
                    index: 1_u32.into(),
                    name: "init_schema".parse().unwrap(),
                    raw_sql: "CREATE SCHEMA app;".into(),
                })
                .unwrap();
            defined_migrations
                .add(PendingMigration {
                    index: 2_u32.into(),
                    name: "create_then_fail".parse().unwrap(),
                    raw_sql: "CREATE TABLE app.created_by_failed_migration (id int8);\nSELECT FROM;"
                        .into(),
                })
                .unwrap();

            let config = Config {
                migration_dir: PathBuf::new(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: PathBuf::new(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_partial".to_string(),
                },
            };

            let context =
                Context::new(&config, client_config, StaticSchemaDump, defined_migrations);

            context.bootstrap().await.unwrap();

            let error = context.apply_pending_no_schema_dump().await.unwrap_err();

            assert!(
                matches!(
                    error,
                    mmigration::ContextError::ApplyMigration { index, .. }
                        if index == 2_u32.into()
                ),
                "expected ApplyMigration for index 1, got: {error:?}"
            );

            client_config
                .with_sqlx_connection(async |connection| {
                    let schema_present: bool = sqlx::query_scalar(
                        "SELECT EXISTS(SELECT FROM information_schema.schemata WHERE schema_name = 'app')",
                    )
                    .fetch_one(&mut *connection)
                    .await?;
                    assert!(
                        schema_present,
                        "migration 0 (CREATE SCHEMA app) must stay committed after migration 1 fails"
                    );

                    let failed_table_present: bool = sqlx::query_scalar(
                        "SELECT EXISTS(SELECT FROM information_schema.tables WHERE table_schema = 'app' AND table_name = 'created_by_failed_migration')",
                    )
                    .fetch_one(&mut *connection)
                    .await?;
                    assert!(
                        !failed_table_present,
                        "migration 1's table must be rolled back when migration 1 fails"
                    );

                    Ok::<(), sqlx::Error>(())
                })
                .await
                .unwrap()
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn bootstrap_is_required_and_single_use() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            let mut defined_migrations = DefinedMigrations::new();
            defined_migrations
                .add(PendingMigration {
                    index: 1_u32.into(),
                    name: "init_schema".parse().unwrap(),
                    raw_sql: "CREATE SCHEMA app;".into(),
                })
                .unwrap();

            let config = Config {
                migration_dir: PathBuf::new(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: PathBuf::new(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_lifecycle".to_string(),
                },
            };

            let context =
                Context::new(&config, client_config, StaticSchemaDump, defined_migrations);

            // Applying before bootstrap surfaces the missing tracking table rather
            // than silently treating the database as empty.
            let error = context.apply_pending_no_schema_dump().await.unwrap_err();
            assert!(
                matches!(error, mmigration::ContextError::NotBootstrapped { .. }),
                "expected NotBootstrapped, got: {error:?}"
            );

            context.bootstrap().await.unwrap();

            // Bootstrapping an already-bootstrapped database errors rather than
            // silently succeeding.
            let error = context.bootstrap().await.unwrap_err();
            assert!(
                matches!(error, mmigration::ContextError::AlreadyBootstrapped { .. }),
                "expected AlreadyBootstrapped, got: {error:?}"
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn apply_yields_when_migration_lock_is_held() {
    use sqlx::ConnectOptions as _;

    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            let mut defined_migrations = DefinedMigrations::new();
            defined_migrations
                .add(PendingMigration {
                    index: 1_u32.into(),
                    name: "init_schema".parse().unwrap(),
                    raw_sql: "CREATE SCHEMA app;".into(),
                })
                .unwrap();

            let config = Config {
                migration_dir: PathBuf::new(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: PathBuf::new(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_lock_held".to_string(),
                },
            };

            let context =
                Context::new(&config, client_config, StaticSchemaDump, defined_migrations);
            context.bootstrap().await.unwrap();

            // Hold the tracking-table lock in a separate connection, standing in for
            // a concurrent migration run.
            let mut holder = client_config
                .to_sqlx_connect_options()
                .unwrap()
                .connect()
                .await
                .unwrap();
            sqlx::raw_sql(
                "BEGIN; LOCK TABLE public.applied_migrations_lock_held IN ACCESS EXCLUSIVE MODE",
            )
            .execute(&mut holder)
            .await
            .unwrap();

            // apply can't take the lock and yields instead of blocking.
            let error = context.apply_pending_no_schema_dump().await.unwrap_err();
            assert!(
                matches!(
                    error,
                    mmigration::ContextError::MigrationLockUnavailable { .. }
                ),
                "expected MigrationLockUnavailable, got: {error:?}"
            );

            // Release the lock; the same apply now succeeds.
            sqlx::raw_sql("ROLLBACK")
                .execute(&mut holder)
                .await
                .unwrap();
            context.apply_pending_no_schema_dump().await.unwrap();

            let schema_present: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT FROM information_schema.schemata WHERE schema_name = 'app')",
            )
            .fetch_one(&mut holder)
            .await
            .unwrap();
            assert!(
                schema_present,
                "migration should have applied once unblocked"
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn concurrent_runners_apply_each_migration_exactly_once() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            let mut defined_migrations = DefinedMigrations::new();
            // Re-running this migration would fail ("schema app already exists"), so a
            // double-apply surfaces as an error rather than passing silently.
            defined_migrations
                .add(PendingMigration {
                    index: 1_u32.into(),
                    name: "init_schema".parse().unwrap(),
                    raw_sql: "CREATE SCHEMA app;".into(),
                })
                .unwrap();

            let config = Config {
                migration_dir: PathBuf::new(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: PathBuf::new(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_concurrent".to_string(),
                },
            };

            let context =
                Context::new(&config, client_config, StaticSchemaDump, defined_migrations);
            context.bootstrap().await.unwrap();

            // Two runs race against the same tracking table.
            let (left, right) = tokio::join!(
                context.apply_pending_no_schema_dump(),
                context.apply_pending_no_schema_dump(),
            );

            let results = [left, right];

            // Each run either succeeded (applying or finding nothing to apply) or
            // yielded to the other; neither double-applies, which would surface as
            // some other error.
            for result in &results {
                assert!(
                    matches!(
                        result,
                        Ok(_) | Err(mmigration::ContextError::MigrationLockUnavailable { .. })
                    ),
                    "unexpected runner result: {result:?}"
                );
            }
            assert!(
                results.iter().any(Result::is_ok),
                "at least one run must succeed: {results:?}"
            );

            // Across both runs, migration 1 is applied exactly once — the other run
            // either skipped it (empty `applied`) or yielded.
            let applied_across_runs: Vec<mmigration::Index> = results
                .iter()
                .filter_map(|result| result.as_ref().ok())
                .flat_map(|report| report.applied.clone())
                .collect();
            assert_eq!(vec![mmigration::Index::from(1_u32)], applied_across_runs);

            let schema_present: bool = client_config
                .with_sqlx_connection(async |connection| {
                    sqlx::query_scalar(
                        "SELECT EXISTS(SELECT FROM information_schema.schemata WHERE schema_name = 'app')",
                    )
                    .fetch_one(&mut *connection)
                    .await
                })
                .await
                .unwrap()
                .unwrap();
            assert!(schema_present, "the migration must have applied exactly once");
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn second_apply_run_on_applied_state_noops() {
    let Some(backend) = ociman::testing::setup_backend().await else {
        return;
    };
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let client_config = container.client_config();

            let mut defined_migrations = DefinedMigrations::new();
            // Re-running either migration would error (duplicate schema / table), so a
            // non-noop second run surfaces as an error rather than passing silently.
            defined_migrations
                .add(PendingMigration {
                    index: 1_u32.into(),
                    name: "init_schema".parse().unwrap(),
                    raw_sql: "CREATE SCHEMA app;".into(),
                })
                .unwrap();
            defined_migrations
                .add(PendingMigration {
                    index: 2_u32.into(),
                    name: "add_users".parse().unwrap(),
                    raw_sql: "CREATE TABLE app.users (id int8 PRIMARY KEY);".into(),
                })
                .unwrap();

            let config = Config {
                migration_dir: PathBuf::new(),
                schema_normalizer: Box::new(NoopNormalizer),
                schema_path: PathBuf::new(),
                qualified_table_name: QualifiedTableName {
                    schema_name: "public".to_string(),
                    table_name: "applied_migrations_idempotent".to_string(),
                },
            };

            let context =
                Context::new(&config, client_config, StaticSchemaDump, defined_migrations);
            context.bootstrap().await.unwrap();

            // First run applies both migrations, from the baseline (0) to 2.
            let first = context.apply_pending_no_schema_dump().await.unwrap();
            assert_eq!(
                mmigration::ApplyReport {
                    before: mmigration::Index::baseline(),
                    after: 2_u32.into(),
                    applied: vec![1_u32.into(), 2_u32.into()],
                },
                first
            );

            // Second run on the same state is a clean no-op: nothing applied, the
            // level unchanged. (If it re-ran a migration the duplicate DDL would error.)
            let second = context.apply_pending_no_schema_dump().await.unwrap();
            assert_eq!(
                mmigration::ApplyReport {
                    before: 2_u32.into(),
                    after: 2_u32.into(),
                    applied: vec![],
                },
                second
            );
        })
        .await
        .unwrap();
}
