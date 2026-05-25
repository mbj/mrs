pub mod cli;
pub mod defined_migrations;
pub mod schema;
pub mod transaction;
pub mod types;

pub use defined_migrations::*;
pub use types::*;

use transaction::Transaction;

#[derive(Debug, PartialEq)]
pub struct QualifiedTableName {
    pub schema_name: String,
    pub table_name: String,
}

#[derive(Debug)]
pub struct Config {
    pub migration_dir: std::path::PathBuf,
    pub schema_normalizer: Box<dyn SchemaNormalizer>,
    pub schema_path: std::path::PathBuf,
    pub qualified_table_name: QualifiedTableName,
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            migration_dir: left_migration_dir,
            schema_normalizer: left_schema_normalizer,
            schema_path: left_schema_path,
            qualified_table_name: left_qualified_table_name,
        } = self;
        let Self {
            migration_dir: right_migration_dir,
            schema_normalizer: right_schema_normalizer,
            schema_path: right_schema_path,
            qualified_table_name: right_qualified_table_name,
        } = other;

        left_migration_dir == right_migration_dir
            && left_schema_normalizer.as_ref() == right_schema_normalizer.as_ref()
            && left_schema_path == right_schema_path
            && left_qualified_table_name == right_qualified_table_name
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait DynEq {
    fn dyn_eq(&self, other: &dyn std::any::Any) -> bool;
}

impl<T: PartialEq + std::any::Any> DynEq for T {
    fn dyn_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>() == Some(self)
    }
}

impl PartialEq for dyn SchemaNormalizer {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}

impl AsAny for dyn SchemaNormalizer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait SchemaDump {
    fn schema_dump(&self) -> impl std::future::Future<Output = Schema> + Send;
}

pub trait SchemaNormalizer: std::fmt::Debug + std::any::Any + DynEq + Sync {
    fn normalize(&self, schema: &Schema) -> Schema;
}

#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("I/O error while {operation} on {path}: {source}")]
    IoError {
        operation: &'static str,
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Connection(#[from] pg_client::sqlx::ConnectionError),
    #[error(transparent)]
    Transaction(Box<pg_client::sqlx::transaction::TransactionError<ContextError>>),
    #[error("failed to apply migration {index}: {source}")]
    ApplyMigration {
        index: Index,
        #[source]
        source: sqlx::Error,
    },
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Failed to parse applied migrations table comment: {comment}\n{report}")]
    ParseAppliedMigrationsComment { comment: String, report: String },
    #[error("migration tracking table {schema}.{table} does not exist; run bootstrap first")]
    NotBootstrapped { schema: String, table: String },
    #[error(
        "migration tracking table {schema}.{table} already exists; database is already bootstrapped"
    )]
    AlreadyBootstrapped { schema: String, table: String },
    #[error(transparent)]
    Pending(#[from] PendingError),
    #[error(transparent)]
    Index(#[from] IndexError),
}

impl From<pg_client::sqlx::transaction::TransactionError<ContextError>> for ContextError {
    fn from(error: pg_client::sqlx::transaction::TransactionError<ContextError>) -> Self {
        use pg_client::sqlx::transaction::TransactionError;
        match error {
            TransactionError::Action(context_error) => context_error,
            other => Self::Transaction(Box::new(other)),
        }
    }
}

pub struct Context<'a, D: SchemaDump> {
    client_config: &'a pg_client::Config,
    config: &'a Config,
    defined_migrations: DefinedMigrations,
    schema_dump: D,
}

impl<'a, D: SchemaDump> Context<'a, D> {
    #[must_use]
    pub fn new(
        config: &'a Config,
        client_config: &'a pg_client::Config,
        schema_dump: D,
        defined_migrations: DefinedMigrations,
    ) -> Self {
        Context {
            config,
            client_config,
            defined_migrations,
            schema_dump,
        }
    }

    pub fn load(
        config: &'a Config,
        client_config: &'a pg_client::Config,
        schema_dump: D,
    ) -> Result<Self, LoadError> {
        Ok(Self::new(
            config,
            client_config,
            schema_dump,
            DefinedMigrations::load(&config.migration_dir)?,
        ))
    }

    /// Create the migration tracking table.
    ///
    /// This is a deliberate, one-time setup step: it errors with
    /// [`ContextError::AlreadyBootstrapped`] if the table already exists rather than
    /// silently succeeding, so a mistaken re-bootstrap (or a misconfigured table
    /// name) surfaces instead of being shadowed.
    pub async fn bootstrap(&self) -> Result<(), ContextError> {
        self.with_transaction(async |transaction| transaction.bootstrap().await)
            .await
    }

    pub async fn apply_pending_no_schema_dump(&self) -> Result<(), ContextError> {
        // Each migration is applied in its own transaction: locks are released at
        // every commit rather than held across the whole batch, and a failure
        // partway through leaves the already-applied migrations committed so a
        // re-run resumes from the first unapplied one.
        for pending_migration in self.find_pending_migrations().await? {
            self.with_transaction(async |transaction| {
                transaction.apply_pending_migration(pending_migration).await
            })
            .await?;
        }

        Ok(())
    }

    pub async fn apply_pending(&self) -> Result<(), ContextError> {
        self.apply_pending_no_schema_dump().await?;

        self.schema_dump().await?;
        Ok(())
    }

    pub async fn create_new_pending(&self, name: &MigrationName) -> Result<(), ContextError> {
        // The next index is a filesystem fact (one past the highest defined migration,
        // or the initial index when none are defined), so creating a new migration
        // file needs no database round-trip.
        let next_index = self
            .defined_migrations
            .next_index()?
            .unwrap_or_else(Index::initial);

        let next_path = self
            .config
            .migration_dir
            .join(format!("{next_index}_{name}.sql"));

        log::info!("Creating new migration: {}", next_path.display());

        std::fs::write(&next_path, format!("-- Migration {next_index} {name}")).map_err(
            |source| ContextError::IoError {
                operation: "write_new_pending_migration",
                path: next_path.clone(),
                source,
            },
        )?;
        Ok(())
    }

    pub async fn schema_dump(&self) -> Result<(), ContextError> {
        log::info!("Writing schema to: {}", self.config.schema_path.display());

        std::fs::write(
            &self.config.schema_path,
            self.config
                .schema_normalizer
                .normalize(&self.schema_dump.schema_dump().await),
        )
        .map_err(|source| ContextError::IoError {
            operation: "write_schema_dump",
            path: self.config.schema_path.clone(),
            source,
        })?;

        Ok(())
    }

    pub async fn find_pending_migrations(&self) -> Result<Vec<&PendingMigration>, ContextError> {
        self.with_transaction(async |transaction| {
            self.find_pending_migrations_transaction(transaction).await
        })
        .await
    }

    async fn with_transaction<T, F: AsyncFnMut(&mut Transaction) -> Result<T, ContextError>>(
        &self,
        action: F,
    ) -> Result<T, ContextError> {
        Transaction::with_transaction(
            self.client_config,
            &self.config.qualified_table_name,
            action,
        )
        .await
    }

    async fn find_pending_migrations_transaction(
        &self,
        transaction: &mut Transaction<'_>,
    ) -> Result<Vec<&PendingMigration>, ContextError> {
        self.defined_migrations
            .select_pending(transaction.find_last_applied_index().await?)
            .map_err(ContextError::from)
    }
}
