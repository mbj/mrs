//! Coordinated index addition for partitioned PostgreSQL tables.
//!
//! Protocol:
//! 1. Discover partitions from `pg_catalog`
//! 2. `CREATE INDEX [CONCURRENTLY]` on each partition (parallel workers)
//! 3. `CREATE INDEX ON ONLY parent_table` (invalid stub on parent)
//! 4. `ALTER INDEX parent_index ATTACH PARTITION partition_index` for each partition

#[cfg(feature = "clap")]
pub mod cli;
pub mod create;
pub mod gc;

use std::collections::BTreeSet;

use crate::identifier::Table;

pub(crate) mod sql_str_serde {
    use serde::{Deserialize, Deserializer};
    use sqlx::SqlSafeStr as _;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<sqlx::SqlStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(sqlx::AssertSqlSafe(s).into_sql_str())
    }
}

/// Concurrency settings for partition index creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcurrentlyConfig {
    /// Do not use `CREATE INDEX CONCURRENTLY`.
    None,
    /// Use `CREATE INDEX CONCURRENTLY` on all partitions.
    All,
    /// Use `CREATE INDEX CONCURRENTLY` for all partitions except the specified ones.
    Except(BTreeSet<Table>),
}

impl ConcurrentlyConfig {
    /// Returns true if the partition should be created concurrently.
    #[must_use]
    pub fn is_concurrent_for(&self, table: &Table) -> bool {
        match self {
            Self::None => false,
            Self::All => true,
            Self::Except(tables) => !tables.contains(table),
        }
    }
}

/// Error returned when parsing an empty SQL fragment.
#[derive(Debug, thiserror::Error)]
#[error("SQL fragment must not be empty")]
pub struct EmptySqlFragment;

/// Error returned when parsing an invalid fillfactor.
#[derive(Debug, thiserror::Error)]
pub enum FillFactorParseError {
    /// The value was not a valid integer.
    #[error("fillfactor must be an integer between 1 and 100")]
    InvalidFormat,
    /// The value was outside the allowed range.
    #[error("fillfactor must be between 1 and 100, got {0}")]
    OutOfRange(u8),
}

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

/// Storage parameter for index fillfactor (1-100).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillFactor(u8);

impl FillFactor {
    /// Minimum allowed fillfactor.
    pub const MIN: u8 = 1;
    /// Maximum allowed fillfactor.
    pub const MAX: u8 = 100;

    /// Create a fillfactor from a numeric value.
    pub fn new(value: u8) -> Result<Self, FillFactorParseError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(FillFactorParseError::OutOfRange(value));
        }

        Ok(Self(value))
    }

    /// Returns the fillfactor as a numeric value.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl core::str::FromStr for FillFactor {
    type Err = FillFactorParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed: u8 = value
            .parse()
            .map_err(|_| FillFactorParseError::InvalidFormat)?;
        Self::new(parsed)
    }
}

impl core::fmt::Display for FillFactor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use crate::identifier::{Index, Schema};

/// Errors that can occur during index operations.
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
    /// Partition tables referenced in concurrency configuration were not found.
    #[error("unknown partition tables in concurrently selection: {tables:?}")]
    UnknownPartitionTables {
        /// Unknown partition tables.
        tables: BTreeSet<Table>,
    },
    /// Index is already valid, gc should not run.
    #[error("index {schema}.{index} is already valid")]
    IndexAlreadyValid {
        /// The schema name.
        schema: Schema,
        /// The index name.
        index: Index,
    },
    /// Index not found, nothing to gc.
    #[error("index {schema}.{index} not found")]
    IndexNotFound {
        /// The schema name.
        schema: Schema,
        /// The index name.
        index: Index,
    },
}
