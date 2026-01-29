//! Index creation for partitioned tables.

use core::num::NonZeroU16;

use sqlx::Row as _;
use sqlx::SqlSafeStr as _;

use super::{Error, FillFactor, SqlFragment};
use crate::identifier::{AccessMethod, Index, Schema, Table};

/// Input parameters for adding an index to a partitioned table.
pub struct Input {
    /// The schema containing the partitioned table.
    pub schema: Schema,
    /// The parent partitioned table.
    pub table: Table,
    /// The desired parent index name.
    pub index: Index,
    /// The index key expression without surrounding parentheses (e.g. `"lower(email), account_id"`).
    pub key_expression: SqlFragment,
    /// Whether the index should enforce uniqueness.
    pub unique: bool,
    /// The index access method (e.g. btree, hash).
    pub method: AccessMethod,
    /// An optional `INCLUDE` clause for covering indexes (without `INCLUDE` keyword or parentheses).
    pub include: Option<SqlFragment>,
    /// An optional `WHERE` clause (without the `WHERE` keyword).
    pub where_clause: Option<SqlFragment>,
    /// An optional index fillfactor (1-100).
    pub fillfactor: Option<FillFactor>,
    /// Whether to use `CREATE INDEX CONCURRENTLY` on partitions.
    pub concurrently: bool,
}

/// Result of a successful index addition.
#[derive(Debug)]
pub struct Result {
    /// Time elapsed during the operation.
    pub elapsed: std::time::Duration,
    /// Partitions that were processed.
    pub partitions: Vec<Partition>,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Created {} partition indexes in {:.2}s",
            self.partitions.len(),
            self.elapsed.as_secs_f64()
        )
    }
}

/// A partition with its derived index name and SQL statements for creation.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Partition {
    /// Partition schema name.
    pub schema: Schema,
    /// Partition table name.
    pub table: Table,
    /// Partition index name.
    pub index: Index,
    /// Partition `CREATE INDEX` statement.
    #[serde(deserialize_with = "super::sql_str_serde::deserialize")]
    pub create_statement: sqlx::SqlStr,
    /// `ALTER INDEX ATTACH PARTITION` statement.
    #[serde(deserialize_with = "super::sql_str_serde::deserialize")]
    pub attach_statement: sqlx::SqlStr,
}

/// All SQL statements needed for the index addition protocol.
pub struct Statements {
    /// The `CREATE INDEX ON ONLY` statement for the parent table.
    pub parent_create: sqlx::SqlStr,
    /// Per-partition information including SQL statements.
    pub partitions: Vec<Partition>,
}

/// Fetch all SQL statements for the index addition protocol.
///
/// Returns the parent `CREATE INDEX ON ONLY` statement, per-partition
/// `CREATE INDEX` statements, and `ALTER INDEX ATTACH PARTITION` statements.
/// All statements are generated server-side via `format()` with `%I` identifier quoting.
pub async fn fetch_statements(
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
    let include_clause = input
        .include
        .as_ref()
        .map(|i| i.as_str())
        .unwrap_or_default();
    let where_clause = input
        .where_clause
        .as_ref()
        .map(|w| w.as_str())
        .unwrap_or_default();
    let fillfactor = input
        .fillfactor
        .map(|value| value.as_u8().to_string())
        .unwrap_or_default();

    let (parent_create, partitions) = config
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
                  , include_clause
                  , fillfactor
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
                      , $9::text
                      , $10::text
                      )
                  )
                , fragments AS (
                    SELECT
                      derived.include_clause
                    , derived.storage_clause
                    , derived.where_clause
                    , format
                      ( 'CREATE %sINDEX %I ON ONLY %I.%I USING %I (%s)%s%s%s'
                      , params.unique_keyword
                      , params.parent_index
                      , params.schema_name
                      , params.parent_table
                      , params.access_method
                      , params.key_expression
                      , derived.include_clause
                      , derived.storage_clause
                      , derived.where_clause
                      ) AS parent_create_statement
                    , format
                      ( 'CREATE %sINDEX %s%%I ON %%I.%%I USING %I (%s)%s%s%s'
                      , params.unique_keyword
                      , params.concurrently_keyword
                      , params.access_method
                      , params.key_expression
                      , derived.include_clause
                      , derived.storage_clause
                      , derived.where_clause
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
                        CASE WHEN params.include_clause = '' THEN '' ELSE ' INCLUDE (' || params.include_clause || ')' END
                      , CASE WHEN params.fillfactor = '' THEN '' ELSE ' WITH (fillfactor = ' || params.fillfactor || ')' END
                      , CASE WHEN params.where_clause = '' THEN '' ELSE ' WHERE ' || params.where_clause END
                    ) AS derived(include_clause, storage_clause, where_clause)
                  )
                , partitions AS (
                    SELECT
                      child_namespace.nspname AS schema
                    , child_class.relname AS table
                    , derived.partition_index_name AS index
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
                , (
                    SELECT
                      COALESCE(jsonb_agg(
                        jsonb_build_object
                          ( 'schema', partitions.schema
                          , 'table', partitions.table
                          , 'index', partitions.index
                          , 'create_statement', partitions.create_statement
                          , 'attach_statement', partitions.attach_statement
                          )
                        ORDER BY partitions.schema, partitions.table
                      ), '[]'::jsonb)
                    FROM
                      partitions
                  ) AS partitions
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
            .bind(include_clause)
            .bind(fillfactor.clone())
            .bind(where_clause)
            .fetch_one(connection)
            .await?;

            let parent_create: String = row.get("parent_create_statement");
            let partitions_json: serde_json::Value = row.get("partitions");
            let partitions: Vec<Partition> =
                serde_json::from_value(partitions_json).expect("valid partition JSON from database");

            Ok::<_, sqlx::Error>((parent_create, partitions))
        })
        .await??;

    if partitions.is_empty() {
        return Err(Error::NoPartitions {
            schema: input.schema.to_string(),
            table: input.table.to_string(),
        });
    }

    Ok(Statements {
        parent_create: sqlx::AssertSqlSafe(parent_create).into_sql_str(),
        partitions,
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
///
/// If `dry_run` is true, the SQL statements are logged but not executed.
pub async fn run(
    config: &crate::Config,
    input: &Input,
    jobs: NonZeroU16,
    dry_run: bool,
) -> core::result::Result<Result, Error> {
    use std::collections::VecDeque;
    use std::sync::Arc;

    use tokio::sync::Mutex;
    use tokio::task::JoinSet;

    let start = std::time::Instant::now();

    let Statements {
        parent_create,
        partitions,
        ..
    } = fetch_statements(config, input).await?;

    if dry_run {
        for partition in &partitions {
            log::info!("[dry-run] {};", partition.create_statement.as_str());
        }
        log::info!("[dry-run] {};", parent_create.as_str());
        for partition in &partitions {
            log::info!("[dry-run] {};", partition.attach_statement.as_str());
        }
    } else {
        let partitions_for_workers = partitions.clone();

        // Step 2: Create indexes on partitions via parallel workers
        let shared_config = Arc::new(config.clone());
        let shared_queue = Arc::new(Mutex::new(VecDeque::from(partitions_for_workers)));
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

                for partition in &partitions {
                    sqlx::raw_sql(partition.attach_statement.clone())
                        .execute(&mut *connection)
                        .await?;
                }

                Ok::<(), sqlx::Error>(())
            })
            .await??;
    }

    Ok(Result {
        elapsed: start.elapsed(),
        partitions,
    })
}
