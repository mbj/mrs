#![cfg(feature = "tools")]

use std::num::NonZeroU16;

use indoc::indoc;
use pg_client::protocol::frontend;
use pg_client::session;

async fn execute(session: &mut session::Session, sql: &'static str) {
    session
        .statement(&frontend::Sql::from_static_or_panic(sql))
        .execute()
        .unwrap()
        .discard_rows()
        .await
        .unwrap();
}

async fn execute_generated(
    session: &mut session::Session,
    generator: &'static str,
    arguments: &[&str],
) {
    let mut builder = session.statement(&frontend::Sql::from_static_or_panic(generator));

    for argument in arguments {
        builder = builder.bind(*argument);
    }

    let mut reader = builder.execute().unwrap();
    let description = reader.describe().await.unwrap();
    let statement = description.column::<frontend::Sql>(0).unwrap();
    let generated = reader
        .decode(|row| row.decode(&statement))
        .one()
        .await
        .unwrap();

    session
        .statement(&generated)
        .execute()
        .unwrap()
        .discard_rows()
        .await
        .unwrap();
}

async fn setup_partitioned_events(container: &pg_ephemeral::Container, cross_schema: bool) {
    let schema_q2 = if cross_schema { "analytics" } else { "public" };

    setup_partitioned_events_with_partitions(
        container,
        ("public", "events_2024q1"),
        (schema_q2, "events_2024q2"),
    )
    .await;
}

async fn setup_partitioned_events_with_partitions(
    container: &pg_ephemeral::Container,
    partition_1: (&str, &str),
    partition_2: (&str, &str),
) {
    container
        .with_session(async |session| {
            for schema in [partition_1.0, partition_2.0] {
                if schema != "public" {
                    execute_generated(
                        session,
                        "SELECT format('CREATE SCHEMA IF NOT EXISTS %I', $1::text)",
                        &[schema],
                    )
                    .await;
                }
            }

            execute(
                session,
                indoc! {"
                    CREATE TABLE public.events
                      ( id int
                      , created_at date
                      , payload text
                      )
                    PARTITION BY RANGE (created_at)
                "},
            )
            .await;

            execute_generated(
                session,
                indoc! {"
                    SELECT
                      format(
                        'CREATE TABLE %I.%I PARTITION OF public.events FOR VALUES FROM (''2024-01-01'') TO (''2024-04-01'')',
                        $1::text,
                        $2::text
                      )
                "},
                &[partition_1.0, partition_1.1],
            )
            .await;

            execute_generated(
                session,
                indoc! {"
                    SELECT
                      format(
                        'CREATE TABLE %I.%I PARTITION OF public.events FOR VALUES FROM (''2024-04-01'') TO (''2024-07-01'')',
                        $1::text,
                        $2::text
                      )
                "},
                &[partition_2.0, partition_2.1],
            )
            .await;
        })
        .await
        .unwrap();
}

async fn run_partitioned_index_addition(
    config: std::sync::Arc<pg_client::session::Config>,
) -> Result<
    pg_client::tools::partitioned_index::create::Result,
    pg_client::tools::partitioned_index::Error,
> {
    let input = pg_client::tools::partitioned_index::create::Input {
        qualified_table: pg_client::name::QualifiedTable {
            schema: pg_client::name::Schema::PUBLIC,
            table: "events".parse().unwrap(),
        },
        index: "idx_events_created_at".parse().unwrap(),
        key_expression: "created_at".parse().unwrap(),
        unique: false,
        method: "btree".parse().unwrap(),
        include: None,
        where_clause: None,
        fillfactor: None,
        concurrently: pg_client::tools::partitioned_index::ConcurrentlyConfig::All,
    };

    pg_client::tools::partitioned_index::create::run(
        config,
        &input,
        NonZeroU16::new(2).unwrap(),
        false,
    )
    .await
}

async fn assert_parent_index_valid(container: &pg_ephemeral::Container) {
    let is_valid = get_parent_index_validity(container).await;
    assert_eq!(is_valid, Some(true), "Parent index should be valid");
}

async fn assert_parent_index_invalid(container: &pg_ephemeral::Container) {
    let is_valid = get_parent_index_validity(container).await;
    assert_eq!(is_valid, Some(false), "Parent index should be invalid");
}

async fn get_parent_index_validity(container: &pg_ephemeral::Container) -> Option<bool> {
    container
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(indoc! {"
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
                "}))
                .bind("idx_events_created_at")
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let is_valid = description.column::<bool>(0).unwrap();

            reader
                .decode(|row| row.decode(&is_valid))
                .max_one()
                .await
                .unwrap()
        })
        .await
        .unwrap()
}

async fn assert_index_exists(
    container: &pg_ephemeral::Container,
    schema: &pg_client::name::Schema<'_>,
    index: &pg_client::name::Index<'_>,
) {
    let count = count_index(container, schema, index).await;
    assert_eq!(count, 1, "Expected {schema}.{index} to exist");
}

async fn assert_index_not_exists(
    container: &pg_ephemeral::Container,
    schema: &pg_client::name::Schema<'_>,
    index: &pg_client::name::Index<'_>,
) {
    let count = count_index(container, schema, index).await;
    assert_eq!(count, 0, "Expected {schema}.{index} to not exist");
}

async fn count_index(
    container: &pg_ephemeral::Container,
    schema: &pg_client::name::Schema<'_>,
    index: &pg_client::name::Index<'_>,
) -> i64 {
    container
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(indoc! {"
                    SELECT
                      count(*)
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
                "}))
                .bind(index.as_str())
                .bind(schema.as_str())
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let count = description.column::<i64>(0).unwrap();

            reader.decode(|row| row.decode(&count)).one().await.unwrap()
        })
        .await
        .unwrap()
}

fn find_partition_index_name(
    result: &pg_client::tools::partitioned_index::create::Result,
    qualified_table: &pg_client::name::QualifiedTable,
) -> pg_client::name::Index<'static> {
    result
        .partitions
        .iter()
        .find(|partition| &partition.qualified_table == qualified_table)
        .unwrap_or_else(|| panic!("missing partition for table {qualified_table}"))
        .index
        .clone()
}

fn definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    // CI environments may be slow, use 30s instead of default 10s
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

#[tokio::test]
async fn test_analyze_all_tables() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Create a test table to analyze
            container
                .with_session(async |session| {
                    execute(
                        session,
                        indoc! {"
                            CREATE TABLE test_table
                              ( id int PRIMARY KEY
                              , name text
                              )
                        "},
                    )
                    .await;
                })
                .await
                .unwrap();

            // Run analyze on public schema
            let result = pg_client::tools::analyze::run_all(
                container.session_config(),
                &pg_client::tools::analyze::Schemas::Specific(
                    [pg_client::name::Schema::PUBLIC].into(),
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
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_addition() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Setup: create partitioned table with 2 range partitions
            setup_partitioned_events(container, false).await;

            // Run index addition
            let result = run_partitioned_index_addition(container.session_config()).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            // Verify parent index is valid
            assert_parent_index_valid(container).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_addition_cross_schema() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Setup: create partitioned table with partitions across schemas.
            setup_partitioned_events(container, true).await;

            // Run index addition
            let result = run_partitioned_index_addition(container.session_config()).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            // Verify parent index is valid
            assert_parent_index_valid(container).await;
            let public_qualified = pg_client::name::QualifiedTable {
                schema: pg_client::name::Schema::PUBLIC,
                table: "events_2024q1".parse().unwrap(),
            };
            let analytics_qualified = pg_client::name::QualifiedTable {
                schema: "analytics".parse().unwrap(),
                table: "events_2024q2".parse().unwrap(),
            };
            let public_index = find_partition_index_name(&result, &public_qualified);
            let analytics_index = find_partition_index_name(&result, &analytics_qualified);
            assert_index_exists(container, &pg_client::name::Schema::PUBLIC, &public_index).await;
            assert_index_exists(container, &analytics_qualified.schema, &analytics_index).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_addition_truncation() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let long_suffix_a = "a".repeat(49);
            let long_suffix_b = "b".repeat(49);
            let partition_1 = format!("events_2024q1_{long_suffix_a}");
            let partition_2 = format!("events_2024q1_{long_suffix_b}");

            setup_partitioned_events_with_partitions(
                container,
                ("public", partition_1.as_str()),
                ("public", partition_2.as_str()),
            )
            .await;

            let result = run_partitioned_index_addition(container.session_config()).await;
            assert!(result.is_ok(), "Index addition failed: {result:?}");
            let result = result.unwrap();
            assert_eq!(result.partitions.len(), 2);

            assert_parent_index_valid(container).await;

            let qualified_1 = pg_client::name::QualifiedTable {
                schema: pg_client::name::Schema::PUBLIC,
                table: partition_1.parse().unwrap(),
            };
            let qualified_2 = pg_client::name::QualifiedTable {
                schema: pg_client::name::Schema::PUBLIC,
                table: partition_2.parse().unwrap(),
            };
            let index_1 = find_partition_index_name(&result, &qualified_1);
            let index_2 = find_partition_index_name(&result, &qualified_2);

            assert_ne!(index_1, index_2, "Index names should be distinct");
            assert_index_exists(container, &pg_client::name::Schema::PUBLIC, &index_1).await;
            assert_index_exists(container, &pg_client::name::Schema::PUBLIC, &index_2).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_concurrently_except_unknown_partition_fails() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Setup: create partitioned table with 2 range partitions
            setup_partitioned_events(container, false).await;

            let missing_partition: pg_client::name::Table = "events_2024q3".parse().unwrap();
            let mut excluded_tables = std::collections::BTreeSet::new();
            excluded_tables.insert(missing_partition.clone());

            let input = pg_client::tools::partitioned_index::create::Input {
                qualified_table: pg_client::name::QualifiedTable {
                    schema: pg_client::name::Schema::PUBLIC,
                    table: "events".parse().unwrap(),
                },
                index: "idx_events_created_at".parse().unwrap(),
                key_expression: "created_at".parse().unwrap(),
                unique: false,
                method: "btree".parse().unwrap(),
                include: None,
                where_clause: None,
                fillfactor: None,
                concurrently: pg_client::tools::partitioned_index::ConcurrentlyConfig::Except(
                    excluded_tables,
                ),
            };

            let result = pg_client::tools::partitioned_index::create::fetch_statements(
                container.session_config(),
                &input,
            )
            .await;

            match result {
                Err(pg_client::tools::partitioned_index::Error::UnknownPartitionTables {
                    tables,
                }) => {
                    assert!(
                        tables.contains(&missing_partition),
                        "Missing partition should be reported"
                    );
                }
                Err(other_error) => {
                    panic!("Expected UnknownPartitionTables error, got {other_error}")
                }
                Ok(_) => panic!("Expected UnknownPartitionTables error, got Ok"),
            }
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_gc() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Setup: create partitioned table with 2 range partitions
            setup_partitioned_events(container, false).await;

            // Fetch statements but only partially apply them
            let input = pg_client::tools::partitioned_index::create::Input {
                qualified_table: pg_client::name::QualifiedTable {
                    schema: pg_client::name::Schema::PUBLIC,
                    table: "events".parse().unwrap(),
                },
                index: "idx_events_created_at".parse().unwrap(),
                key_expression: "created_at".parse().unwrap(),
                unique: false,
                method: "btree".parse().unwrap(),
                include: None,
                where_clause: None,
                fillfactor: None,
                concurrently: pg_client::tools::partitioned_index::ConcurrentlyConfig::None, // Non-concurrent for simpler partial state
            };

            let statements = pg_client::tools::partitioned_index::create::fetch_statements(
                container.session_config(),
                &input,
            )
            .await
            .unwrap();

            // Create partition indexes and parent stub, but don't attach
            container
                .with_session(async |session| {
                    // Create partition indexes
                    for partition in &statements.partitions {
                        session
                            .statement(&partition.create_index_statement)
                            .execute()
                            .unwrap()
                            .discard_rows()
                            .await
                            .unwrap();
                    }

                    // Create parent index stub (will be invalid)
                    session
                        .statement(&statements.parent_create)
                        .execute()
                        .unwrap()
                        .discard_rows()
                        .await
                        .unwrap();
                })
                .await
                .unwrap();

            // Verify we have an invalid parent index
            assert_parent_index_invalid(container).await;

            // Verify partition indexes exist
            for partition in &statements.partitions {
                assert_index_exists(
                    container,
                    &partition.qualified_table.schema,
                    &partition.index,
                )
                .await;
            }

            // Run GC
            let gc_input = pg_client::tools::partitioned_index::gc::Input {
                schema: pg_client::name::Schema::PUBLIC,
                index: "idx_events_created_at".parse().unwrap(),
            };

            let gc_result = pg_client::tools::partitioned_index::gc::run(
                container.session_config(),
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
            let parent_index: pg_client::name::Index = "idx_events_created_at".parse().unwrap();
            assert_index_not_exists(container, &pg_client::name::Schema::PUBLIC, &parent_index)
                .await;

            for partition in &statements.partitions {
                assert_index_not_exists(
                    container,
                    &partition.qualified_table.schema,
                    &partition.index,
                )
                .await;
            }
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_partitioned_index_gc_refuses_valid_index() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Setup and create a valid index
            setup_partitioned_events(container, false).await;
            run_partitioned_index_addition(container.session_config())
                .await
                .expect("index creation should succeed");

            // Verify index is valid
            assert_parent_index_valid(container).await;

            // Try to GC - should fail
            let gc_input = pg_client::tools::partitioned_index::gc::Input {
                schema: pg_client::name::Schema::PUBLIC,
                index: "idx_events_created_at".parse().unwrap(),
            };

            let gc_result = pg_client::tools::partitioned_index::gc::run(
                container.session_config(),
                &gc_input,
                NonZeroU16::new(1).unwrap(),
                false,
            )
            .await;

            assert!(gc_result.is_err(), "GC should fail on valid index");
            assert!(
                matches!(
                    gc_result.unwrap_err(),
                    pg_client::tools::partitioned_index::Error::IndexAlreadyValid { .. }
                ),
                "Should be IndexAlreadyValid error"
            );
        })
        .await
        .unwrap()
}
