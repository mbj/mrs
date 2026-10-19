//! Index creation for partitioned tables.

use core::num::NonZeroU16;
use std::collections::BTreeSet;
use std::sync::Arc;

use super::{ConcurrentlyConfig, Error, FillFactor, SqlFragment};
use crate::name::{self, AccessMethod, Index, QualifiedTable, Table};
use crate::session;
use crate::value;

/// Input parameters for adding an index to a partitioned table.
pub struct Input {
    /// The schema-qualified parent partitioned table.
    pub qualified_table: QualifiedTable,
    /// The desired parent index name.
    pub index: Index<'static>,
    /// The index key expression without surrounding parentheses (e.g. `"lower(email), account_id"`).
    pub key_expression: SqlFragment,
    /// Whether the index should enforce uniqueness.
    pub unique: bool,
    /// The index access method (e.g. btree, hash).
    pub method: AccessMethod<'static>,
    /// An optional `INCLUDE` clause for covering indexes (without `INCLUDE` keyword or parentheses).
    pub include: Option<SqlFragment>,
    /// An optional `WHERE` clause (without the `WHERE` keyword).
    pub where_clause: Option<SqlFragment>,
    /// An optional index fillfactor (1-100).
    pub fillfactor: Option<FillFactor>,
    /// Concurrency settings for partition index creation.
    pub concurrently: ConcurrentlyConfig,
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
    /// Schema-qualified partition table name.
    #[serde(flatten)]
    pub qualified_table: QualifiedTable,
    /// Partition index name.
    pub index: Index<'static>,
    /// Partition `CREATE INDEX` statement.
    #[serde(deserialize_with = "super::sql_serde::deserialize")]
    pub create_index_statement: crate::Sql<'static>,
    /// `ALTER INDEX ATTACH PARTITION` statement.
    #[serde(deserialize_with = "super::sql_serde::deserialize")]
    pub attach_statement: crate::Sql<'static>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PartitionRow {
    /// Schema-qualified partition table name.
    #[serde(flatten)]
    qualified_table: QualifiedTable,
    /// Partition index name.
    index: Index<'static>,
    /// Partition `CREATE INDEX` statement (non-concurrent).
    #[serde(deserialize_with = "super::sql_serde::deserialize")]
    create_index_statement: crate::Sql<'static>,
    /// Partition `CREATE INDEX CONCURRENTLY` statement.
    #[serde(deserialize_with = "super::sql_serde::deserialize")]
    create_index_statement_concurrently: crate::Sql<'static>,
    /// `ALTER INDEX ATTACH PARTITION` statement.
    #[serde(deserialize_with = "super::sql_serde::deserialize")]
    attach_statement: crate::Sql<'static>,
}

/// All SQL statements needed for the index addition protocol.
pub struct Statements {
    /// The `CREATE INDEX ON ONLY` statement for the parent table.
    pub parent_create: crate::Sql<'static>,
    /// Per-partition information including SQL statements.
    pub partitions: Vec<Partition>,
}

/// Fetch all SQL statements for the index addition protocol.
///
/// Returns the parent `CREATE INDEX ON ONLY` statement, per-partition
/// `CREATE INDEX` statements, and `ALTER INDEX ATTACH PARTITION` statements.
/// All statements are generated server-side via `format()` with `%I` identifier quoting.
pub async fn fetch_statements(
    config: Arc<session::Config>,
    input: &Input,
) -> core::result::Result<Statements, Error> {
    const PARENT_CREATE_STATEMENT: name::Column =
        name::Column::from_static_or_panic("parent_create_statement");
    const PARTITIONS: name::Column = name::Column::from_static_or_panic("partitions");

    let method = input.method.as_str();
    let unique_keyword = if input.unique { "UNIQUE " } else { "" };
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

    let (parent_create, partitions) =
        session::with_session(config, async |session| {
            const STATEMENTS: crate::Sql = crate::Sql::from_static_or_panic(indoc::indoc! {"
                WITH
                  params
                  ( parent_table
                  , schema_name
                  , parent_index
                  , access_method
                  , unique_keyword
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
                      ( 'CREATE %sINDEX %%I ON %%I.%%I USING %I (%s)%s%s%s'
                      , params.unique_keyword
                      , params.access_method
                      , params.key_expression
                      , derived.include_clause
                      , derived.storage_clause
                      , derived.where_clause
                      ) AS create_index_template
                    , format
                      ( 'CREATE %sINDEX CONCURRENTLY %%I ON %%I.%%I USING %I (%s)%s%s%s'
                      , params.unique_keyword
                      , params.access_method
                      , params.key_expression
                      , derived.include_clause
                      , derived.storage_clause
                      , derived.where_clause
                      ) AS create_index_template_concurrently
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
                      ) AS create_index_statement
                    , format
                      ( fragments.create_index_template_concurrently
                      , derived.partition_index_name
                      , child_namespace.nspname
                      , child_class.relname
                      ) AS create_index_statement_concurrently
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
                          , 'create_index_statement', partitions.create_index_statement
                          , 'create_index_statement_concurrently', partitions.create_index_statement_concurrently
                          , 'attach_statement', partitions.attach_statement
                          )
                        ORDER BY partitions.schema, partitions.table
                      ), '[]'::jsonb)
                    FROM
                      partitions
                  ) AS partitions
                FROM
                  fragments
            "});

            let mut reader = session
                .statement(&STATEMENTS)
                .bind(input.qualified_table.table.as_str())?
                .bind(input.qualified_table.schema.as_str())?
                .bind(input.index.as_str())?
                .bind(method)?
                .bind(unique_keyword)?
                .bind(key_expression)?
                .bind(include_clause)?
                .bind(fillfactor.as_str())?
                .bind(where_clause)?
                .execute()?;

            let description = reader.describe().await?;
            let parent_create = description.column::<crate::Sql<'static>>(PARENT_CREATE_STATEMENT)?;
            let partitions = description.column::<value::Jsonb<Vec<PartitionRow>>>(PARTITIONS)?;

            let row = reader
                .decode(|row| {
                    let value::Jsonb(partitions) = row.decode(&partitions)?;

                    Ok((row.decode(&parent_create)?, partitions))
                })
                .one()
                .await?;

            Ok::<_, Error>(row)
        })
        .await??;

    if partitions.is_empty() {
        return Err(Error::NoPartitions {
            qualified_table: input.qualified_table.clone(),
        });
    }

    let partition_tables: BTreeSet<Table> = partitions
        .iter()
        .map(|partition| partition.qualified_table.table.clone())
        .collect();
    validate_concurrently_tables(&input.concurrently, &partition_tables)?;

    let partitions = partitions
        .into_iter()
        .map(|partition| {
            let create_index_statement = if input
                .concurrently
                .is_concurrent_for(&partition.qualified_table.table)
            {
                partition.create_index_statement_concurrently
            } else {
                partition.create_index_statement
            };

            Partition {
                qualified_table: partition.qualified_table,
                index: partition.index,
                create_index_statement,
                attach_statement: partition.attach_statement,
            }
        })
        .collect();

    Ok(Statements {
        parent_create,
        partitions,
    })
}

fn validate_concurrently_tables(
    concurrently: &ConcurrentlyConfig,
    partition_tables: &BTreeSet<Table<'static>>,
) -> core::result::Result<(), Error> {
    let requested_tables = match concurrently {
        ConcurrentlyConfig::Except(tables) => Some(tables),
        ConcurrentlyConfig::None | ConcurrentlyConfig::All => None,
    };

    let Some(requested_tables) = requested_tables else {
        return Ok(());
    };

    let unknown_tables: BTreeSet<Table> = requested_tables
        .difference(partition_tables)
        .cloned()
        .collect();

    if unknown_tables.is_empty() {
        return Ok(());
    }

    Err(Error::UnknownPartitionTables {
        tables: unknown_tables,
    })
}

/// Worker that creates indexes on partitions from the queue.
async fn worker(
    config: Arc<session::Config>,
    queue: Arc<tokio::sync::Mutex<std::collections::VecDeque<Partition>>>,
) -> core::result::Result<(), Error> {
    session::with_session(config, async move |session| {
        loop {
            let partition = queue.lock().await.pop_front();

            let Some(partition) = partition else {
                break;
            };

            log::info!(
                "Creating index {} on {}",
                partition.index,
                partition.qualified_table
            );

            session
                .statement(&partition.create_index_statement)
                .execute()?
                .discard_rows()
                .await?;

            log::info!(
                "Created index {} on {}",
                partition.index,
                partition.qualified_table
            );
        }

        Ok(())
    })
    .await?
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
    config: Arc<session::Config>,
    input: &Input,
    jobs: NonZeroU16,
    dry_run: bool,
) -> core::result::Result<Result, Error> {
    use std::collections::VecDeque;

    use tokio::sync::Mutex;
    use tokio::task::JoinSet;

    let start = std::time::Instant::now();

    let Statements {
        parent_create,
        partitions,
        ..
    } = fetch_statements(Arc::clone(&config), input).await?;

    if dry_run {
        for partition in &partitions {
            log::info!("[dry-run] {};", partition.create_index_statement.as_str());
        }
        log::info!("[dry-run] {};", parent_create.as_str());
        for partition in &partitions {
            log::info!("[dry-run] {};", partition.attach_statement.as_str());
        }
    } else {
        let partitions_for_workers = partitions.clone();

        // Step 2: Create indexes on partitions via parallel workers
        let shared_queue = Arc::new(Mutex::new(VecDeque::from(partitions_for_workers)));
        let mut join_set = JoinSet::new();

        for _ in 0..jobs.get() {
            let worker_config = Arc::clone(&config);
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
        session::with_session(config, async |session| {
            log::info!("Creating parent index {}", input.index);

            session
                .statement(&parent_create)
                .execute()?
                .discard_rows()
                .await?;

            log::info!("Created parent index {}", input.index);

            for partition in &partitions {
                session
                    .statement(&partition.attach_statement)
                    .execute()?
                    .discard_rows()
                    .await?;
            }

            Ok::<(), Error>(())
        })
        .await??;
    }

    Ok(Result {
        elapsed: start.elapsed(),
        partitions,
    })
}
