use std::num::NonZeroU16;

use indoc::indoc;
use sqlx::Row as _;
use sqlx::SqlSafeStr as _;

const TEST_DATABASE: pg_client::Database =
    pg_client::Database::from_static_or_panic("some-database");

async fn setup_partitioned_events(config: &pg_client::Config, cross_schema: bool) {
    let schema_q2 = if cross_schema { "analytics" } else { "public" };

    setup_partitioned_events_with_partitions(
        config,
        ("public", "events_2024q1"),
        (schema_q2, "events_2024q2"),
    )
    .await;
}

async fn setup_partitioned_events_with_partitions(
    config: &pg_client::Config,
    partition_1: (&str, &str),
    partition_2: (&str, &str),
) {
    config
        .with_sqlx_connection(async |connection| {
            for schema in [partition_1.0, partition_2.0] {
                if schema != "public" {
                    let statement: String = sqlx::query_scalar(indoc! {"
                        SELECT
                          format('CREATE SCHEMA IF NOT EXISTS %I', $1)
                    "})
                    .bind(schema)
                    .fetch_one(&mut *connection)
                    .await?;
                    sqlx::raw_sql(sqlx::AssertSqlSafe(statement).into_sql_str())
                        .execute(&mut *connection)
                        .await?;
                }
            }

            sqlx::query(indoc! {"
                CREATE TABLE public.events
                  ( id int
                  , created_at date
                  , payload text
                  )
                PARTITION BY RANGE (created_at)
            "})
                .execute(&mut *connection)
                .await?;

            let statement: String = sqlx::query_scalar(indoc! {"
                SELECT
                  format(
                    'CREATE TABLE %I.%I PARTITION OF public.events FOR VALUES FROM (''2024-01-01'') TO (''2024-04-01'')',
                    $1,
                    $2
                  )
            "})
            .bind(partition_1.0)
            .bind(partition_1.1)
            .fetch_one(&mut *connection)
            .await?;
            sqlx::raw_sql(sqlx::AssertSqlSafe(statement).into_sql_str())
                .execute(&mut *connection)
                .await?;

            let statement: String = sqlx::query_scalar(indoc! {"
                SELECT
                  format(
                    'CREATE TABLE %I.%I PARTITION OF public.events FOR VALUES FROM (''2024-04-01'') TO (''2024-07-01'')',
                    $1,
                    $2
                  )
            "})
            .bind(partition_2.0)
            .bind(partition_2.1)
            .fetch_one(&mut *connection)
            .await?;
            sqlx::raw_sql(sqlx::AssertSqlSafe(statement).into_sql_str())
                .execute(&mut *connection)
                .await?;

            Ok::<(), sqlx::Error>(())
        })
        .await
        .unwrap()
        .unwrap();
}

async fn run_partitioned_index_addition(
    config: &pg_client::Config,
) -> Result<
    pg_client::sqlx::partitioned_index::create::Result,
    pg_client::sqlx::partitioned_index::Error,
> {
    let input = pg_client::sqlx::partitioned_index::create::Input {
        schema: pg_client::identifier::Schema::PUBLIC,
        table: "events".parse().unwrap(),
        index: "idx_events_created_at".parse().unwrap(),
        key_expression: "created_at".parse().unwrap(),
        unique: false,
        method: "btree".parse().unwrap(),
        where_clause: None,
        concurrently: true,
    };

    pg_client::sqlx::partitioned_index::create::run(
        config,
        &input,
        NonZeroU16::new(2).unwrap(),
        false,
    )
    .await
}

async fn assert_parent_index_valid(config: &pg_client::Config) {
    let is_valid = get_parent_index_validity(config).await;
    assert_eq!(is_valid, Some(true), "Parent index should be valid");
}

async fn assert_parent_index_invalid(config: &pg_client::Config) {
    let is_valid = get_parent_index_validity(config).await;
    assert_eq!(is_valid, Some(false), "Parent index should be invalid");
}

async fn get_parent_index_validity(config: &pg_client::Config) -> Option<bool> {
    config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query(indoc! {"
                SELECT
                  indisvalid
                FROM
                  pg_index
                JOIN
                  pg_class
                ON
                  pg_class.oid = pg_index.indexrelid
                WHERE
                  pg_class.relname = $1
            "})
            .bind("idx_events_created_at")
            .fetch_optional(&mut *connection)
            .await?;

            Ok::<Option<bool>, sqlx::Error>(row.map(|r| r.get("indisvalid")))
        })
        .await
        .unwrap()
        .unwrap()
}

async fn assert_index_exists(
    config: &pg_client::Config,
    schema: &pg_client::identifier::Schema,
    index: &pg_client::identifier::Index,
) {
    let count = count_index(config, schema, index).await;
    assert_eq!(count, 1, "Expected {schema}.{index} to exist");
}

async fn assert_index_not_exists(
    config: &pg_client::Config,
    schema: &pg_client::identifier::Schema,
    index: &pg_client::identifier::Index,
) {
    let count = count_index(config, schema, index).await;
    assert_eq!(count, 0, "Expected {schema}.{index} to not exist");
}

async fn count_index(
    config: &pg_client::Config,
    schema: &pg_client::identifier::Schema,
    index: &pg_client::identifier::Index,
) -> i64 {
    config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query(indoc! {"
                SELECT
                  count(*) AS index_count
                FROM
                  pg_class
                JOIN
                  pg_namespace
                ON
                  pg_namespace.oid = pg_class.relnamespace
                WHERE
                  pg_class.relkind = 'i'
                AND
                  pg_class.relname = $1
                AND
                  pg_namespace.nspname = $2
            "})
            .bind(index.as_str())
            .bind(schema.as_str())
            .fetch_one(&mut *connection)
            .await?;
            let count: i64 = row.get("index_count");

            Ok::<i64, sqlx::Error>(count)
        })
        .await
        .unwrap()
        .unwrap()
}

fn find_partition_index_name(
    result: &pg_client::sqlx::partitioned_index::create::Result,
    schema: &pg_client::identifier::Schema,
    table: &pg_client::identifier::Table,
) -> pg_client::identifier::Index {
    result
        .partitions
        .iter()
        .find(|partition| &partition.schema == schema && &partition.table == table)
        .unwrap_or_else(|| panic!("missing partition for table {schema}.{table}"))
        .index
        .clone()
}

fn definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    // CI environments may be slow, use 30s instead of default 10s
    pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default())
        .wait_available_timeout(std::time::Duration::from_secs(30))
}

const TEST_USER: pg_client::User = pg_client::User::from_static_or_panic("some-user");

#[tokio::test]
async fn test_with_sqlx_connection() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let result = container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    let row = sqlx::query("SELECT true AS ok")
                        .fetch_one(connection)
                        .await
                        .unwrap();

                    let ok: bool = row.get("ok");
                    ok
                })
                .await;

            assert!(result.is_ok(), "Connection should succeed: {result:?}");
            assert!(result.unwrap(), "Query should return true");
        })
        .await
}

#[tokio::test]
async fn test_with_sqlx_connection_error_on_unavailable_database() {
    let config = pg_client::Config {
        application_name: None,
        database: TEST_DATABASE,
        endpoint: pg_client::Endpoint::Network {
            host: "localhost".parse().unwrap(),
            channel_binding: None,
            host_addr: None,
            port: Some(pg_client::Port::new(0)), // Port 0 is reserved and never available
        },
        password: Some("test".parse().unwrap()),
        ssl_mode: pg_client::SslMode::Disable,
        ssl_root_cert: None,
        user: TEST_USER,
    };

    let result = config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query("SELECT true AS ok")
                .fetch_one(connection)
                .await
                .unwrap();

            let ok: bool = row.get("ok");
            ok
        })
        .await;

    assert!(result.is_err(), "Connection should fail");

    let error = result.unwrap_err();
    match error {
        pg_client::sqlx::ConnectionError::Connect(_) => {
            // Expected error variant
        }
        other => panic!("Expected Connect error, got: {other:?}"),
    }
}

#[tokio::test]
async fn test_analyze_all_tables() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Create a test table to analyze
            config
                .with_sqlx_connection(async |connection| {
                    sqlx::query(indoc! {"
                        CREATE TABLE test_table
                          ( id int PRIMARY KEY
                          , name text
                          )
                    "})
                    .execute(connection)
                    .await
                    .unwrap();
                })
                .await
                .unwrap();

            // Run analyze on public schema
            let result = pg_client::sqlx::analyze::run_all(
                config,
                &pg_client::sqlx::analyze::Schemas::Specific(
                    [pg_client::identifier::Schema::PUBLIC].into(),
                ),
                NonZeroU16::new(1).unwrap(),
            )
            .await;

            assert!(result.is_ok(), "Analyze should succeed: {result:?}");

            let result = result.unwrap();
            assert_eq!(result.table_count, 1, "Should have 1 table to analyze");
            assert!(!result.elapsed.is_zero(), "Elapsed time should be non-zero");
        })
        .await
}

#[tokio::test]
async fn test_partitioned_index_addition() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Setup: create partitioned table with 2 range partitions
            setup_partitioned_events(config, false).await;

            // Run index addition
            let result = run_partitioned_index_addition(config).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            // Verify parent index is valid
            assert_parent_index_valid(config).await;
        })
        .await
}

#[tokio::test]
async fn test_partitioned_index_addition_cross_schema() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Setup: create partitioned table with partitions across schemas.
            setup_partitioned_events(config, true).await;

            // Run index addition
            let result = run_partitioned_index_addition(config).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            // Verify parent index is valid
            assert_parent_index_valid(config).await;
            let public_table: pg_client::identifier::Table = "events_2024q1".parse().unwrap();
            let analytics_table: pg_client::identifier::Table = "events_2024q2".parse().unwrap();
            let public_index = find_partition_index_name(
                &result,
                &pg_client::identifier::Schema::PUBLIC,
                &public_table,
            );
            let analytics_schema: pg_client::identifier::Schema = "analytics".parse().unwrap();
            let analytics_index =
                find_partition_index_name(&result, &analytics_schema, &analytics_table);
            assert_index_exists(
                config,
                &pg_client::identifier::Schema::PUBLIC,
                &public_index,
            )
            .await;
            assert_index_exists(config, &analytics_schema, &analytics_index).await;
        })
        .await
}

#[tokio::test]
async fn test_partitioned_index_addition_truncation() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            let long_suffix_a = "a".repeat(49);
            let long_suffix_b = "b".repeat(49);
            let partition_1 = format!("events_2024q1_{long_suffix_a}");
            let partition_2 = format!("events_2024q1_{long_suffix_b}");

            setup_partitioned_events_with_partitions(
                config,
                ("public", partition_1.as_str()),
                ("public", partition_2.as_str()),
            )
            .await;

            let result = run_partitioned_index_addition(config).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            assert_parent_index_valid(config).await;

            let table_1: pg_client::identifier::Table = partition_1.parse().unwrap();
            let table_2: pg_client::identifier::Table = partition_2.parse().unwrap();
            let index_1 = find_partition_index_name(
                &result,
                &pg_client::identifier::Schema::PUBLIC,
                &table_1,
            );
            let index_2 = find_partition_index_name(
                &result,
                &pg_client::identifier::Schema::PUBLIC,
                &table_2,
            );

            assert_ne!(index_1, index_2, "Index names should be distinct");
            assert_index_exists(config, &pg_client::identifier::Schema::PUBLIC, &index_1).await;
            assert_index_exists(config, &pg_client::identifier::Schema::PUBLIC, &index_2).await;
        })
        .await
}

#[tokio::test]
async fn test_partitioned_index_gc() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Setup: create partitioned table with 2 range partitions
            setup_partitioned_events(config, false).await;

            // Fetch statements but only partially apply them
            let input = pg_client::sqlx::partitioned_index::create::Input {
                schema: pg_client::identifier::Schema::PUBLIC,
                table: "events".parse().unwrap(),
                index: "idx_events_created_at".parse().unwrap(),
                key_expression: "created_at".parse().unwrap(),
                unique: false,
                method: "btree".parse().unwrap(),
                where_clause: None,
                concurrently: false, // Non-concurrent for simpler partial state
            };

            let statements =
                pg_client::sqlx::partitioned_index::create::fetch_statements(config, &input)
                    .await
                    .expect("fetch_statements should succeed");

            // Create partition indexes and parent stub, but don't attach
            config
                .with_sqlx_connection(async |connection| {
                    // Create partition indexes
                    for partition in &statements.partitions {
                        sqlx::raw_sql(partition.create_statement.clone())
                            .execute(&mut *connection)
                            .await?;
                    }

                    // Create parent index stub (will be invalid)
                    sqlx::raw_sql(statements.parent_create.clone())
                        .execute(&mut *connection)
                        .await?;

                    Ok::<(), sqlx::Error>(())
                })
                .await
                .unwrap()
                .unwrap();

            // Verify we have an invalid parent index
            assert_parent_index_invalid(config).await;

            // Verify partition indexes exist
            for partition in &statements.partitions {
                assert_index_exists(config, &partition.schema, &partition.index).await;
            }

            // Run GC
            let gc_input = pg_client::sqlx::partitioned_index::gc::Input {
                schema: pg_client::identifier::Schema::PUBLIC,
                index: "idx_events_created_at".parse().unwrap(),
            };

            let gc_result = pg_client::sqlx::partitioned_index::gc::run(
                config,
                &gc_input,
                NonZeroU16::new(2).unwrap(),
                false,
            )
            .await
            .expect("gc should succeed");

            assert!(
                gc_result.parent_dropped,
                "Parent index should have been dropped"
            );
            assert_eq!(
                gc_result.partition_indexes.len(),
                statements.partitions.len(),
                "Should have dropped all partition indexes"
            );

            // Verify all indexes are gone
            let parent_index: pg_client::identifier::Index =
                "idx_events_created_at".parse().unwrap();
            assert_index_not_exists(
                config,
                &pg_client::identifier::Schema::PUBLIC,
                &parent_index,
            )
            .await;

            for partition in &statements.partitions {
                assert_index_not_exists(config, &partition.schema, &partition.index).await;
            }
        })
        .await
}

#[tokio::test]
async fn test_partitioned_index_gc_refuses_valid_index() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Setup and create a valid index
            setup_partitioned_events(config, false).await;
            run_partitioned_index_addition(config)
                .await
                .expect("index creation should succeed");

            // Verify index is valid
            assert_parent_index_valid(config).await;

            // Try to GC - should fail
            let gc_input = pg_client::sqlx::partitioned_index::gc::Input {
                schema: pg_client::identifier::Schema::PUBLIC,
                index: "idx_events_created_at".parse().unwrap(),
            };

            let gc_result = pg_client::sqlx::partitioned_index::gc::run(
                config,
                &gc_input,
                NonZeroU16::new(1).unwrap(),
                false,
            )
            .await;

            assert!(gc_result.is_err(), "GC should fail on valid index");
            assert!(
                matches!(
                    gc_result.unwrap_err(),
                    pg_client::sqlx::partitioned_index::Error::IndexAlreadyValid { .. }
                ),
                "Should be IndexAlreadyValid error"
            );
        })
        .await
}
