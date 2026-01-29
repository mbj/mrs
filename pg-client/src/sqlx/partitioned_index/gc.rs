//! Garbage collection for incomplete partitioned index creation.

use core::num::NonZeroU16;

use sqlx::Row as _;
use sqlx::SqlSafeStr as _;

use crate::identifier::{Index, Schema};

/// Input parameters for garbage collection.
pub struct Input {
    /// The schema containing the index.
    pub schema: Schema,
    /// The parent index name to clean up.
    pub index: Index,
}

/// A partition index to be dropped.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PartitionIndex {
    /// Partition index schema name.
    pub schema: Schema,
    /// Partition index name.
    pub index: Index,
    /// `DROP INDEX` statement.
    #[serde(deserialize_with = "super::sql_str_serde::deserialize")]
    pub drop_statement: sqlx::SqlStr,
}

/// Result of a successful garbage collection.
#[derive(Debug)]
pub struct Result {
    /// Time elapsed during the operation.
    pub elapsed: std::time::Duration,
    /// Whether the parent index was dropped.
    pub parent_dropped: bool,
    /// Partition indexes that were dropped.
    pub partition_indexes: Vec<PartitionIndex>,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent_msg = if self.parent_dropped {
            "1 parent index"
        } else {
            "0 parent indexes"
        };
        write!(
            f,
            "Dropped {} and {} partition indexes in {:.2}s",
            parent_msg,
            self.partition_indexes.len(),
            self.elapsed.as_secs_f64()
        )
    }
}

/// Worker that drops partition indexes from the queue.
async fn worker(
    config: std::sync::Arc<crate::Config>,
    queue: std::sync::Arc<tokio::sync::Mutex<std::collections::VecDeque<PartitionIndex>>>,
) -> core::result::Result<(), super::Error> {
    Ok(config
        .as_ref()
        .with_sqlx_connection(async move |connection| {
            loop {
                let partition_index = queue.lock().await.pop_front();

                let Some(partition_index) = partition_index else {
                    break;
                };

                log::info!(
                    "Dropping partition index {}.{}",
                    partition_index.schema,
                    partition_index.index
                );

                sqlx::raw_sql(partition_index.drop_statement.clone())
                    .execute(&mut *connection)
                    .await?;

                log::info!(
                    "Dropped partition index {}.{}",
                    partition_index.schema,
                    partition_index.index
                );
            }

            Ok::<(), sqlx::Error>(())
        })
        .await??)
}

/// Garbage collect incomplete partitioned index state.
///
/// This function removes partition indexes and the parent index stub
/// left behind by a failed or interrupted index creation.
///
/// Returns an error if the parent index exists and is already valid.
pub async fn run(
    config: &crate::Config,
    input: &Input,
    jobs: NonZeroU16,
    dry_run: bool,
) -> core::result::Result<Result, super::Error> {
    let start = std::time::Instant::now();

    let (parent_is_valid, parent_drop, partition_indexes) = config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query(indoc::indoc! {"
                WITH
                  params(schema_name, parent_index) AS (
                    VALUES ($1::text, $2::text)
                  )
                , parent_index_info AS (
                    SELECT
                      pg_index.indisvalid AS is_valid
                    , format('DROP INDEX %I.%I', params.schema_name, params.parent_index)
                        AS drop_statement
                    FROM
                      params
                    LEFT JOIN
                      pg_class
                    ON
                      pg_class.relname = params.parent_index
                    LEFT JOIN
                      pg_namespace
                    ON
                      pg_namespace.oid = pg_class.relnamespace
                    AND
                      pg_namespace.nspname = params.schema_name
                    LEFT JOIN
                      pg_index
                    ON
                      pg_index.indexrelid = pg_class.oid
                  )
                , partition_indexes AS (
                    SELECT
                      pg_namespace.nspname AS schema
                    , pg_class_index.relname AS index
                    , format
                      ( 'DROP INDEX %I.%I'
                      , pg_namespace.nspname
                      , pg_class_index.relname
                      ) AS drop_statement
                    FROM
                      params
                    JOIN
                      pg_class AS pg_class_index
                    ON
                      pg_class_index.relname LIKE params.parent_index || '_%'
                    JOIN
                      pg_namespace
                    ON
                      pg_namespace.oid = pg_class_index.relnamespace
                    JOIN
                      pg_index
                    ON
                      pg_index.indexrelid = pg_class_index.oid
                    JOIN
                      pg_class AS pg_class_table
                    ON
                      pg_class_table.oid = pg_index.indrelid
                    WHERE
                      pg_class_index.relkind = 'i'
                    AND
                      EXISTS (
                        SELECT 1 FROM pg_inherits
                        WHERE pg_inherits.inhrelid = pg_class_table.oid
                      )
                  )
                SELECT
                  parent_index_info.is_valid AS parent_is_valid
                , parent_index_info.drop_statement AS parent_drop_statement
                , (
                    SELECT
                      COALESCE(jsonb_agg(
                        jsonb_build_object
                          ( 'schema', partition_indexes.schema
                          , 'index', partition_indexes.index
                          , 'drop_statement', partition_indexes.drop_statement
                          )
                        ORDER BY partition_indexes.schema, partition_indexes.index
                      ), '[]'::jsonb)
                    FROM
                      partition_indexes
                  ) AS partition_indexes
                FROM
                  parent_index_info
            "})
            .bind(input.schema.as_str())
            .bind(input.index.as_str())
            .fetch_one(connection)
            .await?;

            let parent_is_valid: Option<bool> = row.get("parent_is_valid");
            let parent_drop: String = row.get("parent_drop_statement");
            let partition_indexes_json: serde_json::Value = row.get("partition_indexes");
            let partition_indexes: Vec<PartitionIndex> =
                serde_json::from_value(partition_indexes_json)
                    .expect("valid partition index JSON from database");

            Ok::<_, sqlx::Error>((parent_is_valid, parent_drop, partition_indexes))
        })
        .await??;

    // Bail out if the parent index is already valid
    if parent_is_valid == Some(true) {
        return Err(super::Error::IndexAlreadyValid {
            schema: input.schema.clone(),
            index: input.index.clone(),
        });
    }

    // Bail out if there's nothing to clean up
    if parent_is_valid.is_none() && partition_indexes.is_empty() {
        return Err(super::Error::IndexNotFound {
            schema: input.schema.clone(),
            index: input.index.clone(),
        });
    }

    let parent_drop = sqlx::AssertSqlSafe(parent_drop).into_sql_str();
    let parent_exists = parent_is_valid.is_some();

    if dry_run {
        for partition_index in &partition_indexes {
            log::info!("[dry-run] {};", partition_index.drop_statement.as_str());
        }
        if parent_exists {
            log::info!("[dry-run] {};", parent_drop.as_str());
        }
    } else {
        use std::collections::VecDeque;
        use std::sync::Arc;

        use tokio::sync::Mutex;
        use tokio::task::JoinSet;

        // Drop partition indexes via parallel workers
        let partition_indexes_for_workers = partition_indexes.clone();
        let shared_config = Arc::new(config.clone());
        let shared_queue = Arc::new(Mutex::new(VecDeque::from(partition_indexes_for_workers)));
        let mut join_set = JoinSet::new();

        for _ in 0..jobs.get() {
            let worker_config = Arc::clone(&shared_config);
            let worker_queue = Arc::clone(&shared_queue);

            join_set.spawn(async move { worker(worker_config, worker_queue).await });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(worker_result) => worker_result?,
                Err(join_error) => return Err(super::Error::WorkerPanic(join_error)),
            }
        }

        // Drop parent index after all partition indexes are dropped
        if parent_exists {
            config
                .with_sqlx_connection(async |connection| {
                    log::info!("Dropping parent index {}.{}", input.schema, input.index);
                    sqlx::raw_sql(parent_drop.clone())
                        .execute(&mut *connection)
                        .await?;
                    Ok::<(), sqlx::Error>(())
                })
                .await??;
        }
    }

    Ok(Result {
        elapsed: start.elapsed(),
        parent_dropped: parent_exists,
        partition_indexes,
    })
}
