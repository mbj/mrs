//! Parallel ANALYZE execution for PostgreSQL tables.

use core::num::NonZeroU16;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::name::{self, QualifiedTable, Schema, Table};
use crate::protocol::frontend;
use crate::session::{self, execute};
use crate::value;

/// An ANALYZE task for a table.
#[derive(Debug)]
struct AnalyzeTask {
    /// The schema-qualified table name.
    qualified_table: QualifiedTable,
    /// The SQL statement to execute.
    statement: frontend::Sql,
}

/// Specifies which schemas to analyze.
#[derive(Debug, Clone)]
pub enum Schemas {
    /// Analyze all schemas.
    All,
    /// Analyze only the specified schemas.
    Specific(BTreeSet<Schema<'static>>),
}

/// Analyze errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection error.
    #[error(transparent)]
    Connection(#[from] session::OpenError),
    /// Worker task panicked.
    #[error("worker task panicked: {0}")]
    WorkerPanic(tokio::task::JoinError),
    #[error(transparent)]
    Execute(#[from] execute::Error),
    #[error(transparent)]
    Column(#[from] execute::ColumnError),
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
    config: Arc<session::Config>,
    schemas: &Schemas,
    jobs: NonZeroU16,
) -> core::result::Result<Result, Error> {
    use std::collections::VecDeque;

    use tokio::sync::Mutex;
    use tokio::task::JoinSet;

    let start = std::time::Instant::now();

    let tasks = fetch_tasks(Arc::clone(&config), schemas).await?;
    let table_count = u64::try_from(tasks.len()).expect("task count fits in u64");

    let shared_queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
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

    Ok(Result {
        elapsed: start.elapsed(),
        table_count,
    })
}

/// Worker that processes ANALYZE tasks from the queue.
async fn worker(
    config: Arc<session::Config>,
    queue: Arc<tokio::sync::Mutex<std::collections::VecDeque<AnalyzeTask>>>,
) -> core::result::Result<(), Error> {
    session::with_session(config, async move |session| {
        loop {
            let task = queue.lock().await.pop_front();

            let Some(task) = task else {
                break;
            };

            log::info!("Analyzing {}", task.qualified_table);

            session
                .statement(&task.statement)
                .execute()?
                .discard_rows()
                .await?;

            log::info!("Analyzed {}", task.qualified_table);
        }

        Ok(())
    })
    .await?
}

/// Fetch ANALYZE tasks for all tables in the specified schemas.
async fn fetch_tasks(
    config: Arc<session::Config>,
    schemas: &Schemas,
) -> core::result::Result<Vec<AnalyzeTask>, Error> {
    const ALL: frontend::Sql = frontend::Sql::from_static_or_panic(indoc::indoc! {"
      SELECT
        pg_tables.schemaname AS schema_name
      , pg_tables.tablename AS table_name
      , format('ANALYZE %I.%I', pg_tables.schemaname, pg_tables.tablename) AS statement
      FROM
        pg_tables
      JOIN
        pg_class ON pg_class.relname = pg_tables.tablename
      JOIN
        pg_namespace ON pg_namespace.oid = pg_class.relnamespace AND pg_namespace.nspname = pg_tables.schemaname
      WHERE
        pg_class.relkind != 'p'
      ORDER BY
        pg_tables.schemaname
      , pg_tables.tablename
    "});

    const SPECIFIC: frontend::Sql = frontend::Sql::from_static_or_panic(indoc::indoc! {"
      SELECT
        pg_tables.schemaname AS schema_name
      , pg_tables.tablename AS table_name
      , format('ANALYZE %I.%I', pg_tables.schemaname, pg_tables.tablename) AS statement
      FROM
        pg_tables
      JOIN
        pg_class ON pg_class.relname = pg_tables.tablename
      JOIN
        pg_namespace ON pg_namespace.oid = pg_class.relnamespace AND pg_namespace.nspname = pg_tables.schemaname
      WHERE
        pg_class.relkind != 'p'
      AND
        pg_tables.schemaname = ANY($1)
      ORDER BY
        pg_tables.schemaname
      , pg_tables.tablename
    "});

    const SCHEMA_NAME: name::Column = name::Column::from_static_or_panic("schema_name");
    const TABLE_NAME: name::Column = name::Column::from_static_or_panic("table_name");
    const STATEMENT: name::Column = name::Column::from_static_or_panic("statement");

    session::with_session(config, async |session| {
        let mut reader = match schemas {
            Schemas::All => session.statement(&ALL).execute()?,
            Schemas::Specific(schema_set) => {
                let names: Vec<&str> = schema_set.iter().map(Schema::as_str).collect();

                session
                    .statement(&SPECIFIC)
                    .bind(value::Array::from(names))
                    .execute()?
            }
        };

        let description = reader.describe().await?;
        let schema = description.column::<Schema<'static>>(SCHEMA_NAME)?;
        let table = description.column::<Table<'static>>(TABLE_NAME)?;
        let statement = description.column::<frontend::Sql>(STATEMENT)?;

        let tasks = reader
            .decode(|row| {
                Ok(AnalyzeTask {
                    qualified_table: QualifiedTable {
                        schema: row.decode(&schema)?,
                        table: row.decode(&table)?,
                    },
                    statement: row.decode(&statement)?,
                })
            })
            .into_vec()
            .await?;

        Ok(tasks)
    })
    .await?
}
