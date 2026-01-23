//! Parallel ANALYZE execution for PostgreSQL tables.

use core::num::NonZeroUsize;
use core::str::FromStr as _;
use std::collections::BTreeSet;

use sqlx::Row as _;
use sqlx::SqlSafeStr as _;

use crate::identifier::{Schema, Table};

/// An ANALYZE task for a table.
#[derive(Debug)]
struct AnalyzeTask {
    /// The schema name.
    schema: Schema,
    /// The SQL statement to execute.
    statement: sqlx::SqlStr,
    /// The table name.
    table: Table,
}

/// Specifies which schemas to analyze.
#[derive(Debug, Clone)]
pub enum Schemas {
    /// Analyze all schemas.
    All,
    /// Analyze only the specified schemas.
    Specific(BTreeSet<Schema>),
}

/// Analyze errors.
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
}

/// Result of running ANALYZE on all tables.
#[derive(Debug)]
pub struct Result {
    /// Time elapsed during the analyze operation.
    pub elapsed: std::time::Duration,
    /// Number of tables analyzed.
    pub table_count: u64,
}

/// Run ANALYZE on all tables in the specified schemas in parallel.
pub async fn run_all(
    config: &crate::Config,
    schemas: &Schemas,
    jobs: NonZeroUsize,
) -> core::result::Result<Result, Error> {
    use std::collections::VecDeque;
    use std::sync::Arc;

    use tokio::sync::Mutex;
    use tokio::task::JoinSet;

    let start = std::time::Instant::now();

    let tasks = fetch_tasks(config, schemas).await?;
    let table_count = u64::try_from(tasks.len()).expect("task count fits in u64");

    let shared_config = Arc::new(config.clone());
    let shared_queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
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

    Ok(Result {
        elapsed: start.elapsed(),
        table_count,
    })
}

/// Worker that processes ANALYZE tasks from the queue.
async fn worker(
    config: std::sync::Arc<crate::Config>,
    queue: std::sync::Arc<tokio::sync::Mutex<std::collections::VecDeque<AnalyzeTask>>>,
) -> core::result::Result<(), Error> {
    config
        .as_ref()
        .with_sqlx_connection(async move |connection| {
            loop {
                let task = queue.lock().await.pop_front();

                let Some(task) = task else {
                    break;
                };

                let schema = task.schema.to_string();
                let table = task.table.to_string();

                log::info!("Analyzing {schema}.{table}");

                sqlx::raw_sql(task.statement.clone())
                    .execute(&mut *connection)
                    .await?;

                log::info!("Analyzed {schema}.{table}");
            }

            Ok(())
        })
        .await?
}

/// Fetch ANALYZE tasks for all tables in the specified schemas.
async fn fetch_tasks(
    config: &crate::Config,
    schemas: &Schemas,
) -> core::result::Result<Vec<AnalyzeTask>, Error> {
    config
        .with_sqlx_connection(async |connection| {
            let rows = match schemas {
                Schemas::All => {
                    sqlx::query(indoc::indoc! {"
                      SELECT
                        schemaname AS schema_name
                      , tablename AS table_name
                      , format('ANALYZE %I.%I', schemaname, tablename) AS statement
                      FROM
                        pg_tables
                      ORDER BY
                        schemaname
                      , tablename
                    "})
                    .fetch_all(connection)
                    .await?
                }
                Schemas::Specific(schema_set) => {
                    let schema_names: Vec<&str> = schema_set.iter().map(Schema::as_ref).collect();

                    sqlx::query(indoc::indoc! {"
                      SELECT
                        schemaname AS schema_name
                      , tablename AS table_name
                      , format('ANALYZE %I.%I', schemaname, tablename) AS statement
                      FROM
                        pg_tables
                      WHERE
                        schemaname = ANY($1)
                      ORDER BY
                        schemaname
                      , tablename
                    "})
                    .bind(&schema_names)
                    .fetch_all(connection)
                    .await?
                }
            };

            let tasks = rows
                .into_iter()
                .map(|row| {
                    let schema: String = row.get("schema_name");
                    let table: String = row.get("table_name");
                    let statement: String = row.get("statement");

                    AnalyzeTask {
                        schema: Schema::from_str(&schema)
                            .expect("schema name from database should be valid"),
                        statement: sqlx::AssertSqlSafe(statement).into_sql_str(),
                        table: Table::from_str(&table)
                            .expect("table name from database should be valid"),
                    }
                })
                .collect();

            Ok(tasks)
        })
        .await?
}
