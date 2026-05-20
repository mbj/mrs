//! Run an action inside a Postgres transaction.
//!
//! [`with_transaction`] opens a `BEGIN`, runs the supplied action, then either
//! `COMMIT`s on `Ok` or `ROLLBACK`s on `Err`. The action receives `&mut PgConnection`
//! so it can issue any SQL it likes.
//!
//! Errors are separated by source:
//!
//! * [`TransactionError::Action`] - the action returned `Err`; the transaction was
//!   rolled back successfully.
//! * [`TransactionError::ActionRollback`] - the action returned `Err` and the
//!   subsequent `ROLLBACK` also failed; both errors are carried.
//! * [`TransactionError::Begin`] - `BEGIN` itself failed; no transaction was opened.
//! * [`TransactionError::Commit`] - `COMMIT` itself failed; transaction state is
//!   unknown (it may or may not have durably committed).
//!
//! To distinguish retry-safe commit failures (SQLSTATE `40001`
//! `serialization_failure`, `40P01` `deadlock_detected`, …) from hard failures,
//! inspect the inner `sqlx::Error` with
//! [`super::sqlstate::sqlstate`] and the [`super::sqlstate::SqlState`] constants.

/// Postgres transaction isolation level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsolationLevel {
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// Postgres transaction access mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AccessMode {
    #[default]
    ReadWrite,
    ReadOnly,
}

/// Full set of transaction options.
///
/// `deferrable` is only meaningful when [`IsolationLevel::Serializable`] is combined
/// with [`AccessMode::ReadOnly`]; Postgres accepts the keyword in other combinations
/// as a no-op.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Options {
    pub isolation: IsolationLevel,
    pub access_mode: AccessMode,
    pub deferrable: bool,
}

impl From<IsolationLevel> for Options {
    fn from(isolation: IsolationLevel) -> Self {
        Self {
            isolation,
            access_mode: AccessMode::ReadWrite,
            deferrable: false,
        }
    }
}

impl Options {
    fn begin_sql(self) -> &'static str {
        use AccessMode::{ReadOnly, ReadWrite};
        use IsolationLevel::{ReadCommitted, RepeatableRead, Serializable};

        match (self.isolation, self.access_mode, self.deferrable) {
            (ReadCommitted, ReadWrite, false) => "BEGIN ISOLATION LEVEL READ COMMITTED READ WRITE",
            (ReadCommitted, ReadWrite, true) => {
                "BEGIN ISOLATION LEVEL READ COMMITTED READ WRITE DEFERRABLE"
            }
            (ReadCommitted, ReadOnly, false) => "BEGIN ISOLATION LEVEL READ COMMITTED READ ONLY",
            (ReadCommitted, ReadOnly, true) => {
                "BEGIN ISOLATION LEVEL READ COMMITTED READ ONLY DEFERRABLE"
            }
            (RepeatableRead, ReadWrite, false) => {
                "BEGIN ISOLATION LEVEL REPEATABLE READ READ WRITE"
            }
            (RepeatableRead, ReadWrite, true) => {
                "BEGIN ISOLATION LEVEL REPEATABLE READ READ WRITE DEFERRABLE"
            }
            (RepeatableRead, ReadOnly, false) => "BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY",
            (RepeatableRead, ReadOnly, true) => {
                "BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY DEFERRABLE"
            }
            (Serializable, ReadWrite, false) => "BEGIN ISOLATION LEVEL SERIALIZABLE READ WRITE",
            (Serializable, ReadWrite, true) => {
                "BEGIN ISOLATION LEVEL SERIALIZABLE READ WRITE DEFERRABLE"
            }
            (Serializable, ReadOnly, false) => "BEGIN ISOLATION LEVEL SERIALIZABLE READ ONLY",
            (Serializable, ReadOnly, true) => {
                "BEGIN ISOLATION LEVEL SERIALIZABLE READ ONLY DEFERRABLE"
            }
        }
    }
}

/// Error returned by [`with_transaction`].
#[derive(Debug, thiserror::Error)]
pub enum TransactionError<E> {
    /// The action returned `Err`; `ROLLBACK` succeeded.
    #[error(transparent)]
    Action(E),
    /// The action returned `Err` and the subsequent `ROLLBACK` also failed.
    /// The transaction and connection state is unknown.
    #[error("action failed and ROLLBACK also failed (action: {action}, rollback: {rollback})")]
    ActionRollback {
        #[source]
        action: E,
        rollback: sqlx::Error,
    },
    /// `BEGIN` itself failed; no transaction was opened.
    #[error("BEGIN failed: {0}")]
    Begin(#[source] sqlx::Error),
    /// `COMMIT` itself failed; the transaction's state is unknown.
    #[error("COMMIT failed: {0}")]
    Commit(#[source] sqlx::Error),
}

/// Run `action` inside a Postgres transaction.
///
/// On `Ok(value)`, `COMMIT` is issued and `value` is returned.
/// On `Err(action)`, `ROLLBACK` is issued: if it succeeds the action error is
/// returned as [`TransactionError::Action`]; if the `ROLLBACK` itself fails both
/// errors are returned as [`TransactionError::ActionRollback`].
///
/// Failures of `BEGIN` or `COMMIT` surface as [`TransactionError::Begin`] or
/// [`TransactionError::Commit`] respectively.
pub async fn with_transaction<T, E, F, O>(
    connection: &mut sqlx::postgres::PgConnection,
    options: O,
    mut action: F,
) -> Result<T, TransactionError<E>>
where
    O: Into<Options>,
    F: AsyncFnMut(&mut sqlx::postgres::PgConnection) -> Result<T, E>,
{
    let options: Options = options.into();

    sqlx::raw_sql(options.begin_sql())
        .execute(&mut *connection)
        .await
        .map_err(TransactionError::Begin)?;

    match action(&mut *connection).await {
        Ok(value) => {
            sqlx::raw_sql("COMMIT")
                .execute(&mut *connection)
                .await
                .map_err(TransactionError::Commit)?;
            Ok(value)
        }
        Err(action) => match sqlx::raw_sql("ROLLBACK").execute(&mut *connection).await {
            Ok(_) => Err(TransactionError::Action(action)),
            Err(rollback) => Err(TransactionError::ActionRollback { action, rollback }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_sql_read_committed_default() {
        assert_eq!(
            "BEGIN ISOLATION LEVEL READ COMMITTED READ WRITE",
            Options::from(IsolationLevel::ReadCommitted).begin_sql()
        );
    }

    #[test]
    fn begin_sql_repeatable_read_default() {
        assert_eq!(
            "BEGIN ISOLATION LEVEL REPEATABLE READ READ WRITE",
            Options::from(IsolationLevel::RepeatableRead).begin_sql()
        );
    }

    #[test]
    fn begin_sql_serializable_default() {
        assert_eq!(
            "BEGIN ISOLATION LEVEL SERIALIZABLE READ WRITE",
            Options::from(IsolationLevel::Serializable).begin_sql()
        );
    }

    #[test]
    fn begin_sql_read_only() {
        let options = Options {
            isolation: IsolationLevel::Serializable,
            access_mode: AccessMode::ReadOnly,
            deferrable: false,
        };
        assert_eq!(
            "BEGIN ISOLATION LEVEL SERIALIZABLE READ ONLY",
            options.begin_sql()
        );
    }

    #[test]
    fn begin_sql_serializable_read_only_deferrable() {
        let options = Options {
            isolation: IsolationLevel::Serializable,
            access_mode: AccessMode::ReadOnly,
            deferrable: true,
        };
        assert_eq!(
            "BEGIN ISOLATION LEVEL SERIALIZABLE READ ONLY DEFERRABLE",
            options.begin_sql()
        );
    }
}
