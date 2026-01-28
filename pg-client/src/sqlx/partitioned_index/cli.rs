//! CLI interface for partitioned index addition.

use core::num::NonZeroU16;

use super::{Error, Input, SqlFragment};
use crate::identifier::{AccessMethod, Index, Schema, Table};

/// Add an index to a partitioned PostgreSQL table.
#[derive(Debug, clap::Args)]
pub struct Cli {
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
}

impl Cli {
    /// Run the partitioned index addition using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<super::Result, Error> {
        let input = Input {
            schema: self.schema,
            table: self.table,
            index: self.index,
            key_expression: self.key_expression,
            unique: self.unique,
            method: self.method,
            where_clause: self.where_clause,
            concurrently: self.concurrently,
        };

        super::run(config, &input, self.jobs).await
    }
}
