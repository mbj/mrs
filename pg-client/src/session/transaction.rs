use core::ops::DerefMut;

use super::{Session, statement};
use crate::protocol::backend;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsolationLevel {
    Serializable,
    RepeatableRead,
    ReadCommitted,
    ReadUncommitted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    ReadWrite,
    ReadOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deferrable {
    Deferrable,
    NotDeferrable,
}

#[derive(Debug)]
pub struct Builder<S: DerefMut<Target = Session>> {
    session: S,
    options: Options,
}

impl<S: DerefMut<Target = Session>> Builder<S> {
    pub fn new(session: S) -> Self {
        Self {
            session,
            options: Options::default(),
        }
    }

    #[must_use]
    pub const fn with_isolation(mut self, isolation: IsolationLevel) -> Self {
        self.options.isolation = Some(isolation);
        self
    }

    #[must_use]
    pub const fn with_access(mut self, access: Access) -> Self {
        self.options.access = Some(access);
        self
    }

    #[must_use]
    pub const fn with_deferrable(mut self, deferrable: Deferrable) -> Self {
        self.options.deferrable = Some(deferrable);
        self
    }

    pub async fn begin(mut self) -> Result<Transaction<S>, BeginError> {
        // The status is the server's, and only current once nothing is left
        // unanswered.
        self.session
            .drain_abandoned_before(&crate::trace::Span::current(), 0)
            .await
            .map_err(statement::Error::from)?;

        let status = self.session.transaction_status();

        if status != backend::TransactionStatus::Idle {
            return Err(BeginError::InTransaction { status });
        }

        // Armed before BEGIN goes out, so a cancel while waiting for its reply
        // is covered too: the transaction may already be open.
        let mut transaction = Transaction {
            session: self.session,
            open: true,
        };

        transaction
            .session
            .queue_query(self.options.statement())
            .map_err(statement::Error::from)?;

        transaction.session.read_statements_reply().await?;

        Ok(transaction)
    }

    // Commits what the action did when it succeeds, rolls it back when it
    // fails.
    pub async fn run_action<T, E, F>(self, action: F) -> Result<T, Error<E>>
    where
        F: AsyncFnOnce(&mut Session) -> Result<T, E>,
    {
        let mut transaction = self.begin().await.map_err(Error::Begin)?;

        match action(&mut transaction).await {
            Ok(value) => {
                transaction.commit().await.map_err(Error::Commit)?;

                Ok(value)
            }
            Err(cause) => match transaction.rollback().await {
                Ok(()) => Err(Error::Action(cause)),
                Err(rollback) => Err(Error::Rollback { cause, rollback }),
            },
        }
    }
}

// Unset modes take the server's defaults (default_transaction_isolation and
// friends).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Options {
    isolation: Option<IsolationLevel>,
    access: Option<Access>,
    deferrable: Option<Deferrable>,
}

impl Options {
    fn statement(self) -> &'static crate::Sql<'static> {
        match (self.isolation, self.access, self.deferrable) {
            (None, None, None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic("BEGIN");
                &SQL
            }
            (None, None, Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic("BEGIN DEFERRABLE");
                &SQL
            }
            (None, None, Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic("BEGIN NOT DEFERRABLE");
                &SQL
            }
            (None, Some(Access::ReadWrite), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic("BEGIN READ WRITE");
                &SQL
            }
            (None, Some(Access::ReadWrite), Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN READ WRITE, DEFERRABLE");
                &SQL
            }
            (None, Some(Access::ReadWrite), Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN READ WRITE, NOT DEFERRABLE");
                &SQL
            }
            (None, Some(Access::ReadOnly), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic("BEGIN READ ONLY");
                &SQL
            }
            (None, Some(Access::ReadOnly), Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN READ ONLY, DEFERRABLE");
                &SQL
            }
            (None, Some(Access::ReadOnly), Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN READ ONLY, NOT DEFERRABLE");
                &SQL
            }
            (Some(IsolationLevel::Serializable), None, None) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN ISOLATION LEVEL SERIALIZABLE");
                &SQL
            }
            (Some(IsolationLevel::Serializable), None, Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::Serializable), None, Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::Serializable), Some(Access::ReadWrite), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ WRITE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::Serializable),
                Some(Access::ReadWrite),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ WRITE, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::Serializable),
                Some(Access::ReadWrite),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ WRITE, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::Serializable), Some(Access::ReadOnly), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ ONLY",
                );
                &SQL
            }
            (
                Some(IsolationLevel::Serializable),
                Some(Access::ReadOnly),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ ONLY, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::Serializable),
                Some(Access::ReadOnly),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL SERIALIZABLE, READ ONLY, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::RepeatableRead), None, None) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN ISOLATION LEVEL REPEATABLE READ");
                &SQL
            }
            (Some(IsolationLevel::RepeatableRead), None, Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::RepeatableRead), None, Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::RepeatableRead), Some(Access::ReadWrite), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ WRITE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::RepeatableRead),
                Some(Access::ReadWrite),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ WRITE, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::RepeatableRead),
                Some(Access::ReadWrite),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ WRITE, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::RepeatableRead), Some(Access::ReadOnly), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ ONLY",
                );
                &SQL
            }
            (
                Some(IsolationLevel::RepeatableRead),
                Some(Access::ReadOnly),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ ONLY, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::RepeatableRead),
                Some(Access::ReadOnly),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL REPEATABLE READ, READ ONLY, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadCommitted), None, None) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN ISOLATION LEVEL READ COMMITTED");
                &SQL
            }
            (Some(IsolationLevel::ReadCommitted), None, Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadCommitted), None, Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadCommitted), Some(Access::ReadWrite), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ WRITE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadCommitted),
                Some(Access::ReadWrite),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ WRITE, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadCommitted),
                Some(Access::ReadWrite),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ WRITE, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadCommitted), Some(Access::ReadOnly), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ ONLY",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadCommitted),
                Some(Access::ReadOnly),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ ONLY, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadCommitted),
                Some(Access::ReadOnly),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ COMMITTED, READ ONLY, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadUncommitted), None, None) => {
                static SQL: crate::Sql =
                    crate::Sql::from_static_or_panic("BEGIN ISOLATION LEVEL READ UNCOMMITTED");
                &SQL
            }
            (Some(IsolationLevel::ReadUncommitted), None, Some(Deferrable::Deferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadUncommitted), None, Some(Deferrable::NotDeferrable)) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadUncommitted), Some(Access::ReadWrite), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ WRITE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadUncommitted),
                Some(Access::ReadWrite),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ WRITE, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadUncommitted),
                Some(Access::ReadWrite),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ WRITE, NOT DEFERRABLE",
                );
                &SQL
            }
            (Some(IsolationLevel::ReadUncommitted), Some(Access::ReadOnly), None) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ ONLY",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadUncommitted),
                Some(Access::ReadOnly),
                Some(Deferrable::Deferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ ONLY, DEFERRABLE",
                );
                &SQL
            }
            (
                Some(IsolationLevel::ReadUncommitted),
                Some(Access::ReadOnly),
                Some(Deferrable::NotDeferrable),
            ) => {
                static SQL: crate::Sql = crate::Sql::from_static_or_panic(
                    "BEGIN ISOLATION LEVEL READ UNCOMMITTED, READ ONLY, NOT DEFERRABLE",
                );
                &SQL
            }
        }
    }
}

const COMMIT: crate::Sql = crate::Sql::from_static_or_panic("COMMIT");
const ROLLBACK: crate::Sql = crate::Sql::from_static_or_panic("ROLLBACK");

#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error(transparent)]
    Statement(#[from] statement::Error),
    // A failed transaction answers COMMIT with the ROLLBACK tag and no error.
    #[error("server rolled the failed transaction back instead of committing it")]
    RolledBack,
}

#[derive(Debug, thiserror::Error)]
pub enum BeginError {
    #[error(transparent)]
    Statement(#[from] statement::Error),
    // Nesting would need savepoints; a second BEGIN only draws a warning and
    // its COMMIT would end the outer transaction.
    #[error("session is already in a transaction ({status:?})")]
    InTransaction { status: backend::TransactionStatus },
}

#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    #[error("beginning the transaction")]
    Begin(#[source] BeginError),
    #[error("the transaction's action failed")]
    Action(#[source] E),
    #[error("committing the transaction")]
    Commit(#[source] CommitError),
    #[error("rolling back after the action failed: {rollback}")]
    Rollback {
        #[source]
        cause: E,
        rollback: statement::Error,
    },
}

#[derive(Debug)]
pub struct Transaction<S: DerefMut<Target = Session>> {
    session: S,
    open: bool,
}

impl<S: DerefMut<Target = Session>> Transaction<S> {
    pub async fn commit(mut self) -> Result<(), CommitError> {
        let mut run = self.session.statement(&COMMIT).run()?;

        while run.next().await?.is_some() {}

        let rolled_back = matches!(
            run.outcome(),
            Some(statement::Outcome::Complete(completion))
                if matches!(completion.command(), backend::CommandComplete::Other("ROLLBACK"))
        );

        drop(run);

        self.open = false;

        if rolled_back {
            return Err(CommitError::RolledBack);
        }

        Ok(())
    }

    pub async fn rollback(mut self) -> Result<(), statement::Error> {
        self.session
            .statement(&ROLLBACK)
            .run()?
            .discard_rows()
            .await?;

        self.open = false;

        Ok(())
    }
}

impl<S: DerefMut<Target = Session>> core::ops::Deref for Transaction<S> {
    type Target = Session;

    fn deref(&self) -> &Session {
        &self.session
    }
}

impl<S: DerefMut<Target = Session>> DerefMut for Transaction<S> {
    fn deref_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

// A transaction given up without commit or rollback closes the connection
// rather than queueing a ROLLBACK (as sqlx does): a queued ROLLBACK only goes
// out on the session's next use, and until then the server keeps the
// transaction and its locks. Closing is synchronous, and the backend aborts
// the transaction as soon as it next reads, which an idle one is already
// doing. Savepoints are left out: a guard for them would have to track
// nesting depth client side, which is where sqlx's guards go wrong.
impl<S: DerefMut<Target = Session>> Drop for Transaction<S> {
    fn drop(&mut self) {
        if self.open && !self.session.is_known_idle() {
            self.session.connection.close();
        }
    }
}
