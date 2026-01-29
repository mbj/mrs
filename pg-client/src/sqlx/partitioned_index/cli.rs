//! CLI interface for partitioned index operations.

use core::num::NonZeroU16;

use super::{Error, SqlFragment};
use crate::identifier::{AccessMethod, Index, Schema, Table};

/// Partitioned index operations.
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Create an index on a partitioned table.
    Create(Create),
    /// Garbage collect incomplete index creation state.
    Gc(Gc),
}

/// Output of a partitioned index operation.
#[derive(Debug)]
pub enum Output {
    /// Output of index creation.
    Create(super::create::Result),
    /// Output of garbage collection.
    Gc(super::gc::Result),
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create(result) => result.fmt(f),
            Self::Gc(result) => result.fmt(f),
        }
    }
}

impl Command {
    /// Run the partitioned index operation using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<Output, Error> {
        let output = match self {
            Self::Create(create) => create.run(config).await.map(Output::Create),
            Self::Gc(gc) => gc.run(config).await.map(Output::Gc),
        }?;

        log::info!("{output}");

        Ok(output)
    }
}

/// Create an index on a partitioned PostgreSQL table.
#[derive(Debug, clap::Args)]
pub struct Create {
    /// Parent partitioned table name.
    #[arg(long)]
    table: Table,
    /// Index name for the parent index.
    #[arg(long)]
    index: Index,
    /// Index key expression without surrounding parentheses (e.g. "created_at" or "lower(email), account_id").
    #[arg(long)]
    key_expression: SqlFragment,
    /// Schema name.
    #[arg(long, default_value = "public")]
    schema: Schema,
    /// Create a unique index.
    #[arg(long)]
    unique: bool,
    /// Index access method (e.g. btree, hash).
    #[arg(long, default_value = "btree")]
    method: AccessMethod,
    /// WHERE clause filter (without the WHERE keyword).
    #[arg(long)]
    where_clause: Option<SqlFragment>,
    /// Use CREATE INDEX CONCURRENTLY on partitions.
    #[arg(long)]
    concurrently: bool,
    /// Number of parallel workers for partition index creation.
    #[arg(long, default_value = "1")]
    jobs: NonZeroU16,
    /// Print SQL statements without executing them.
    #[arg(long)]
    dry_run: bool,
}

impl Create {
    /// Run the partitioned index creation using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<super::create::Result, Error> {
        let input = super::create::Input {
            schema: self.schema,
            table: self.table,
            index: self.index,
            key_expression: self.key_expression,
            unique: self.unique,
            method: self.method,
            where_clause: self.where_clause,
            concurrently: self.concurrently,
        };

        super::create::run(config, &input, self.jobs, self.dry_run).await
    }
}

/// Garbage collect incomplete partitioned index creation state.
#[derive(Debug, clap::Args)]
pub struct Gc {
    /// Index name for the parent index.
    #[arg(long)]
    index: Index,
    /// Schema name.
    #[arg(long, default_value = "public")]
    schema: Schema,
    /// Number of parallel workers for partition index deletion.
    #[arg(long, default_value = "1")]
    jobs: NonZeroU16,
    /// Print SQL statements without executing them.
    #[arg(long)]
    dry_run: bool,
}

impl Gc {
    /// Run the garbage collection using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<super::gc::Result, Error> {
        let input = super::gc::Input {
            schema: self.schema,
            index: self.index,
        };

        super::gc::run(config, &input, self.jobs, self.dry_run).await
    }
}
