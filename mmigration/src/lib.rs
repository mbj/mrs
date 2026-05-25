pub mod cli;
pub mod defined_migrations;
pub mod schema;
pub mod transaction;
pub mod types;

pub use defined_migrations::*;
pub use types::*;

use transaction::Transaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedTableName {
    pub schema_name: String,
    pub table_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub migration_dir: std::path::PathBuf,
    pub schema_path: std::path::PathBuf,
    pub qualified_table_name: QualifiedTableName,
}

/// A source of the database schema (e.g. a `pg_dump` wrapper, or a static value
/// in tests).
pub trait SchemaSource {
    fn read(&self) -> impl std::future::Future<Output = Schema> + Send;

    /// Adapt this source so its [`read`](SchemaSource::read) output is passed
    /// through `normalize` before being returned.
    ///
    /// Normalization (e.g. [`schema::remove_version_details`]) is a
    /// `Schema -> Schema` transform; composing it with a source yields another
    /// source, so callers build the exact source they want rather than the
    /// `Context` holding a separate normalizer.
    fn normalized<F>(self, normalize: F) -> Normalized<Self, F>
    where
        Self: Sized,
        F: Fn(&Schema) -> Schema + Send + Sync,
    {
        Normalized {
            source: self,
            normalize,
        }
    }
}

/// A [`SchemaSource`] that applies a `Schema -> Schema` transform to the schema
/// read from an inner source. Created by [`SchemaSource::normalized`].
pub struct Normalized<S, F> {
    source: S,
    normalize: F,
}

impl<S, F> SchemaSource for Normalized<S, F>
where
    S: SchemaSource + Sync,
    F: Fn(&Schema) -> Schema + Send + Sync,
{
    async fn read(&self) -> Schema {
        (self.normalize)(&self.source.read().await)
    }
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
    #[error(
        "could not acquire the migration lock on {schema}.{table}; another migration run is in progress"
    )]
    MigrationLockUnavailable { schema: String, table: String },
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

/// Outcome of an `apply_pending` run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyReport {
    /// Last-applied index observed before the run (the baseline 0 if no migration
    /// had been applied yet).
    pub before: Index,
    /// Last-applied index observed after the run.
    pub after: Index,
    /// Indices this run applied, in order. Empty when the run was a no-op (every
    /// pending migration was already applied, possibly by a concurrent runner).
    pub applied: Vec<Index>,
}

pub struct Context<'a, D: SchemaSource> {
    client_config: &'a pg_client::Config,
    config: &'a Config,
    defined_migrations: DefinedMigrations,
    schema_source: D,
}

impl<'a, D: SchemaSource> Context<'a, D> {
    #[must_use]
    pub fn new(
        config: &'a Config,
        client_config: &'a pg_client::Config,
        schema_source: D,
        defined_migrations: DefinedMigrations,
    ) -> Self {
        Context {
            config,
            client_config,
            defined_migrations,
            schema_source,
        }
    }

    pub fn load(
        config: &'a Config,
        client_config: &'a pg_client::Config,
        schema_source: D,
    ) -> Result<Self, LoadError> {
        Ok(Self::new(
            config,
            client_config,
            schema_source,
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

    /// Apply every pending migration. Does not touch the schema file — call
    /// [`dump_schema`](Self::dump_schema) separately to refresh it.
    pub async fn apply_pending(&self) -> Result<ApplyReport, ContextError> {
        // Each migration is applied in its own transaction: locks are released at
        // every commit rather than held across the whole batch, and a failure
        // partway through leaves the already-applied migrations committed so a
        // re-run resumes from the first unapplied one.
        let before = self.last_applied_index().await?;

        let mut applied = Vec::new();
        for pending_migration in self.find_pending_migrations().await? {
            match self
                .with_transaction(async |transaction| {
                    transaction.apply_pending_migration(pending_migration).await
                })
                .await?
            {
                transaction::MigrationOutcome::Applied => applied.push(pending_migration.index),
                transaction::MigrationOutcome::Skipped => {}
            }
        }

        let after = self.last_applied_index().await?;

        Ok(ApplyReport {
            before,
            after,
            applied,
        })
    }

    /// The last-applied migration index (the baseline 0 if none applied yet).
    pub async fn last_applied_index(&self) -> Result<Index, ContextError> {
        self.with_transaction(async |transaction| transaction.find_last_applied_index().await)
            .await
    }

    pub async fn create_new_pending(&self, name: &MigrationName) -> Result<(), ContextError> {
        // The next index is a filesystem fact (one past the highest defined migration,
        // or the first migration index 1 when none are defined), so creating a new
        // migration file needs no database round-trip.
        let next_index = match self.defined_migrations.next_index()? {
            Some(next_index) => next_index,
            None => Index::baseline().succ()?,
        };

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

    /// Write the current schema (read from the [`SchemaSource`]) to `schema_path`.
    pub async fn dump_schema(&self) -> Result<(), ContextError> {
        log::info!("Writing schema to: {}", self.config.schema_path.display());

        std::fs::write(&self.config.schema_path, self.schema_source.read().await).map_err(
            |source| ContextError::IoError {
                operation: "dump_schema",
                path: self.config.schema_path.clone(),
                source,
            },
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticSource(&'static str);

    impl SchemaSource for StaticSource {
        async fn read(&self) -> Schema {
            self.0.into()
        }
    }

    #[tokio::test]
    async fn normalized_applies_transform_to_read_output() {
        let source = StaticSource("raw").normalized(|schema| {
            let mut wrapped = String::from("[");
            wrapped.push_str(<Schema as AsRef<str>>::as_ref(schema));
            wrapped.push(']');
            wrapped.into()
        });

        assert_eq!(Schema::from("[raw]"), source.read().await);
    }
}
