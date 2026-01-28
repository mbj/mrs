//! Coordinated index addition for partitioned PostgreSQL tables.
//!
//! Protocol:
//! 1. Discover partitions from `pg_catalog`
//! 2. `CREATE INDEX [CONCURRENTLY]` on each partition (parallel workers)
//! 3. `CREATE INDEX ON ONLY parent_table` (invalid stub on parent)
//! 4. `ALTER INDEX parent_index ATTACH PARTITION partition_index` for each partition

#[cfg(feature = "clap")]
pub mod cli;

use core::num::NonZeroU16;
use core::str::FromStr as _;

use sqlx::Row as _;
use sqlx::SqlSafeStr as _;

use crate::identifier::{AccessMethod, Index, Schema, Table};

/// Error returned when parsing an empty SQL fragment.
#[derive(Debug, thiserror::Error)]
#[error("SQL fragment must not be empty")]
pub struct EmptySqlFragment;

/// A raw SQL fragment embedded verbatim in generated statements.
/// No escaping or quoting is applied. The caller is responsible for correctness.
#[derive(Debug, Clone)]
pub struct SqlFragment(String);

impl SqlFragment {
    /// Returns the fragment as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::str::FromStr for SqlFragment {
    type Err = EmptySqlFragment;

    fn from_str(value: &str) -> core::result::Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(EmptySqlFragment);
        }

        Ok(Self(value.to_owned()))
    }
}

/// Input parameters for adding an index to a partitioned table.
pub struct Input {
    /// The schema containing the partitioned table.
    pub schema: Schema,
    /// The parent partitioned table.
    pub table: Table,
    /// The desired parent index name.
    pub index: Index,
    /// The index key expression (e.g. `"lower(email), account_id"`).
    pub key_expression: SqlFragment,
    /// Whether the index should enforce uniqueness.
    pub unique: bool,
    /// The index access method (e.g. btree, hash).
    pub method: AccessMethod,
    /// An optional `WHERE` clause (without the `WHERE` keyword).
    pub where_clause: Option<SqlFragment>,
    /// Whether to use `CREATE INDEX CONCURRENTLY` on partitions.
    pub concurrently: bool,
}

/// Result of a successful index addition.
#[derive(Debug)]
pub struct Result {
    /// Time elapsed during the operation.
    pub elapsed: std::time::Duration,
    /// Number of partitions indexed.
    pub partition_count: u64,
    /// Partitions that were indexed.
    pub partitions: Vec<Partition>,
}

/// Errors that can occur during index addition.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection error.
    #[error(transparent)]
    Connection(#[from] crate::sqlx::ConnectionError),
    /// Worker task panicked.
    #[error("worker task panicked: {0}")]
    WorkerPanic(tokio::task::JoinError),
    /// SQL error.
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
    /// No partitions found for the given table.
    #[error("no partitions found for {schema}.{table}")]
    NoPartitions {
        /// The schema name.
        schema: String,
        /// The table name.
        table: String,
    },
    /// Invalid identifier.
    #[error("invalid identifier: {0}")]
    Identifier(#[from] crate::identifier::ParseError),
}

/// A partition with its derived index name and `CREATE INDEX` statement.
#[derive(Debug, Clone)]
pub struct Partition {
    /// Partition schema name.
    pub schema: Schema,
    /// Partition table name.
    pub table: Table,
    /// Partition index name.
    pub index: Index,
    /// Partition `CREATE INDEX` statement.
    pub create_statement: sqlx::SqlStr,
}

/// All SQL statements needed for the index addition protocol.
struct Statements {
    parent_create: sqlx::SqlStr,
    partitions: Vec<Partition>,
    attach_statements: Vec<sqlx::SqlStr>,
}

/// Fetch all SQL statements for the index addition protocol.
///
/// Returns the parent `CREATE INDEX ON ONLY` statement, per-partition
/// `CREATE INDEX` statements, and `ALTER INDEX ATTACH PARTITION` statements.
/// All statements are generated server-side via `format()` with `%I` identifier quoting.
async fn fetch_statements(
    config: &crate::Config,
    input: &Input,
) -> core::result::Result<Statements, Error> {
    let method = input.method.as_str();
    let unique_keyword = if input.unique { "UNIQUE " } else { "" };
    let concurrently_keyword = if input.concurrently {
        "CONCURRENTLY "
    } else {
        ""
    };
    let key_expression = input.key_expression.as_str();
    let where_clause = input
        .where_clause
        .as_ref()
        .map(|w| w.as_str())
        .unwrap_or_default();

    let (parent_create, schema_names, table_names, index_names, create_statements, attach_statements) = config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query(indoc::indoc! {"
                WITH
                  params
                  ( parent_table
                  , schema_name
                  , parent_index
                  , access_method
                  , unique_keyword
                  , concurrently_keyword
                  , key_expression
                  , where_clause
                  ) AS (
                    VALUES
                      ( $1::text
                      , $2::text
                      , $3::text
                      , $4::text
                      , $5::text
                      , $6::text
                      , $7::text
                      , $8::text
                      )
                  )
                , fragments AS (
                    SELECT
                      derived_where.where_clause
                    , format
                      ( 'CREATE %sINDEX %I ON ONLY %I.%I USING %I (%s)%s'
                      , params.unique_keyword
                      , params.parent_index
                      , params.schema_name
                      , params.parent_table
                      , params.access_method
                      , params.key_expression
                      , derived_where.where_clause
                      ) AS parent_create_statement
                    , format
                      ( 'CREATE %sINDEX %s%%I ON %%I.%%I USING %I (%s)%s'
                      , params.unique_keyword
                      , params.concurrently_keyword
                      , params.access_method
                      , params.key_expression
                      , derived_where.where_clause
                      ) AS create_index_template
                    , format
                      ( 'ALTER INDEX %I.%I ATTACH PARTITION %%I.%%I'
                      , params.schema_name
                      , params.parent_index
                      ) AS attach_index_template
                    FROM
                      params
                    CROSS JOIN LATERAL (
                      SELECT
                        CASE WHEN params.where_clause = '' THEN '' ELSE ' WHERE ' || params.where_clause END
                    ) AS derived_where(where_clause)
                  )
                , partitions AS (
                    SELECT
                      child_namespace.nspname AS child_schema
                    , child_class.relname AS table_name
                    , derived.partition_index_name AS index_name
                    , format
                      ( fragments.create_index_template
                      , derived.partition_index_name
                      , child_namespace.nspname
                      , child_class.relname
                      ) AS create_statement
                    , format
                      ( fragments.attach_index_template
                      , child_namespace.nspname
                      , derived.partition_index_name
                      ) AS attach_statement
                    FROM
                      params
                    CROSS JOIN
                      fragments
                    CROSS JOIN
                      pg_inherits
                    JOIN
                      pg_class AS parent_class
                    ON
                      parent_class.oid = pg_inherits.inhparent
                    JOIN
                      pg_class AS child_class
                    ON
                      child_class.oid = pg_inherits.inhrelid
                    CROSS JOIN LATERAL (
                      SELECT
                        CASE
                          WHEN octet_length(base_name) <= 63 THEN base_name
                          ELSE left(base_name, 54) || '_' || substr(md5(base_name), 1, 8)
                        END AS partition_index_name
                      FROM (
                        SELECT params.parent_index || '_' || child_class.relname AS base_name
                      ) AS base
                    ) AS derived(partition_index_name)
                    JOIN
                      pg_namespace AS parent_namespace
                    ON
                      parent_namespace.oid = parent_class.relnamespace
                    JOIN
                      pg_namespace AS child_namespace
                    ON
                      child_namespace.oid = child_class.relnamespace
                    WHERE
                      parent_class.relkind = 'p'
                    AND
                      parent_class.relname = params.parent_table
                    AND
                      parent_namespace.nspname = params.schema_name
                  )
                SELECT
                  fragments.parent_create_statement
                , (SELECT array_agg(partitions.child_schema ORDER BY partitions.child_schema, partitions.table_name) FROM partitions)
                    AS partition_schema_names
                , (SELECT array_agg(partitions.table_name ORDER BY partitions.child_schema, partitions.table_name) FROM partitions)
                    AS partition_table_names
                , (SELECT array_agg(partitions.index_name ORDER BY partitions.child_schema, partitions.table_name) FROM partitions)
                    AS partition_index_names
                , (SELECT array_agg(partitions.create_statement ORDER BY partitions.child_schema, partitions.table_name) FROM partitions)
                    AS create_statements
                , (SELECT array_agg(partitions.attach_statement ORDER BY partitions.child_schema, partitions.table_name) FROM partitions)
                    AS attach_statements
                FROM
                  fragments
            "})
            .bind(input.table.as_str())
            .bind(input.schema.as_str())
            .bind(input.index.as_str())
            .bind(method)
            .bind(unique_keyword)
            .bind(concurrently_keyword)
            .bind(key_expression)
            .bind(where_clause)
            .fetch_one(connection)
            .await?;

            let parent_create: String = row.get("parent_create_statement");
            let schema_names: Option<Vec<String>> = row.get("partition_schema_names");
            let table_names: Option<Vec<String>> = row.get("partition_table_names");
            let index_names: Option<Vec<String>> = row.get("partition_index_names");
            let create_statements: Option<Vec<String>> = row.get("create_statements");
            let attach_statements: Option<Vec<String>> = row.get("attach_statements");

            Ok::<_, sqlx::Error>((
                parent_create,
                schema_names,
                table_names,
                index_names,
                create_statements,
                attach_statements,
            ))
        })
        .await??;

    let Some(table_names) = table_names else {
        return Err(Error::NoPartitions {
            schema: input.schema.to_string(),
            table: input.table.to_string(),
        });
    };

    let schema_names = schema_names.expect("schema names present when partitions exist");
    let index_names = index_names.expect("index names present when partitions exist");
    let create_statements =
        create_statements.expect("create statements present when partitions exist");
    let attach_statements =
        attach_statements.expect("attach statements present when partitions exist");

    let partitions = schema_names
        .into_iter()
        .zip(table_names)
        .zip(index_names)
        .zip(create_statements)
        .map(
            |(((schema_name, table_name), index_name), create_statement)| Partition {
                schema: Schema::from_str(&schema_name)
                    .expect("schema name from database should be valid"),
                table: Table::from_str(&table_name)
                    .expect("partition name from database should be valid"),
                index: Index::from_str(&index_name).expect("derived index name should be valid"),
                create_statement: sqlx::AssertSqlSafe(create_statement).into_sql_str(),
            },
        )
        .collect();

    let attach_statements = attach_statements
        .into_iter()
        .map(|statement| sqlx::AssertSqlSafe(statement).into_sql_str())
        .collect();

    Ok(Statements {
        parent_create: sqlx::AssertSqlSafe(parent_create).into_sql_str(),
        partitions,
        attach_statements,
    })
}

/// Worker that creates indexes on partitions from the queue.
async fn worker(
    config: std::sync::Arc<crate::Config>,
    queue: std::sync::Arc<tokio::sync::Mutex<std::collections::VecDeque<Partition>>>,
) -> core::result::Result<(), Error> {
    Ok(config
        .as_ref()
        .with_sqlx_connection(async move |connection| {
            loop {
                let partition = queue.lock().await.pop_front();

                let Some(partition) = partition else {
                    break;
                };

                log::info!("Creating index {} on {}", partition.index, partition.table);

                sqlx::raw_sql(partition.create_statement.clone())
                    .execute(&mut *connection)
                    .await?;

                log::info!("Created index {} on {}", partition.index, partition.table);
            }

            Ok::<(), sqlx::Error>(())
        })
        .await??)
}

/// Add an index to a partitioned table and all its partitions.
///
/// This function coordinates the multi-step protocol:
/// 1. Discover partitions from `pg_catalog`
/// 2. `CREATE INDEX [CONCURRENTLY]` on each partition (parallel workers)
/// 3. `CREATE INDEX ON ONLY parent_table` (invalid stub on parent)
/// 4. `ALTER INDEX parent_index ATTACH PARTITION partition_index` for each partition
pub async fn run(
    config: &crate::Config,
    input: &Input,
    jobs: NonZeroU16,
) -> core::result::Result<Result, Error> {
    use std::collections::VecDeque;
    use std::sync::Arc;

    use tokio::sync::Mutex;
    use tokio::task::JoinSet;

    let start = std::time::Instant::now();

    let Statements {
        parent_create,
        partitions,
        attach_statements,
    } = fetch_statements(config, input).await?;

    let partition_count = u64::try_from(partitions.len()).expect("partition count fits in u64");
    let partitions_snapshot = partitions.clone();

    // Step 2: Create indexes on partitions via parallel workers
    let shared_config = Arc::new(config.clone());
    let shared_queue = Arc::new(Mutex::new(VecDeque::from(partitions)));
    let mut join_set = JoinSet::new();

    for _ in 0..jobs.get() {
        let worker_config = Arc::clone(&shared_config);
        let worker_queue = Arc::clone(&shared_queue);

        join_set.spawn(async move { worker(worker_config, worker_queue).await });
    }

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(worker_result) => worker_result?,
            Err(join_error) => return Err(Error::WorkerPanic(join_error)),
        }
    }

    // Steps 3-4: Create parent index stub and attach partition indexes
    config
        .with_sqlx_connection(async |connection| {
            log::info!("Creating parent index {}", input.index);

            sqlx::raw_sql(parent_create.clone())
                .execute(&mut *connection)
                .await?;

            log::info!("Created parent index {}", input.index);

            for attach_statement in &attach_statements {
                sqlx::raw_sql(attach_statement.clone())
                    .execute(&mut *connection)
                    .await?;
            }

            Ok::<(), sqlx::Error>(())
        })
        .await??;

    Ok(Result {
        elapsed: start.elapsed(),
        partition_count,
        partitions: partitions_snapshot,
    })
}
