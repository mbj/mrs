use crate::types::*;
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::context,
};
use nom_language::error::VerboseError;
use sqlx::AssertSqlSafe;
use sqlx::Row as _;

/// Whether `apply_pending_migration` actually applied the migration or found it
/// already applied (by a concurrent runner) and skipped it.
pub(crate) enum MigrationOutcome {
    Applied,
    Skipped,
}

pub enum AppliedMigrationsComment {
    LastAppliedMigration { index: Index, name: MigrationName },
    NoAppliedMigrations,
}

impl AppliedMigrationsComment {
    const NO_APPLIED_MIGRATIONS: &str = "No applied migrations";

    fn from_pending_migration(pending_migration: &PendingMigration) -> AppliedMigrationsComment {
        AppliedMigrationsComment::LastAppliedMigration {
            index: pending_migration.index,
            name: pending_migration.name.clone(),
        }
    }

    fn render(&self) -> String {
        match self {
            Self::NoAppliedMigrations => Self::NO_APPLIED_MIGRATIONS.to_string(),
            Self::LastAppliedMigration { index, name } => {
                format!("Last applied migration: {index}, {name}")
            }
        }
    }

    /// The last-applied index. `NoAppliedMigrations` is the baseline (0): the
    /// tracking table exists but no migration has been applied beyond it.
    fn index(&self) -> Index {
        match self {
            Self::LastAppliedMigration { index, .. } => *index,
            Self::NoAppliedMigrations => Index::baseline(),
        }
    }
}

impl std::str::FromStr for AppliedMigrationsComment {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

        fn parser(input: &str) -> ParseResult<'_, AppliedMigrationsComment> {
            alt((
                context(
                    "no applied migrations comment",
                    tag(AppliedMigrationsComment::NO_APPLIED_MIGRATIONS)
                        .map(|_| AppliedMigrationsComment::NoAppliedMigrations),
                ),
                context(
                    "last applied migration comment",
                    (
                        tag("Last applied migration: "),
                        context(
                            "migration index",
                            map_res(digit1, <Index as std::str::FromStr>::from_str),
                        ),
                        tag(", "),
                        context("migration name", migration_name_parser),
                    )
                        .map(|(_, index, _, name)| {
                            AppliedMigrationsComment::LastAppliedMigration {
                                index,
                                name: MigrationName::from_validated(name),
                            }
                        }),
                ),
            ))
            .parse(input)
        }

        match all_consuming(parser).parse(input).finish() {
            Ok((_, parsed)) => Ok(parsed),
            Err(error) => Err(nom_language::error::convert_error(input, error)),
        }
    }
}

pub(crate) struct Transaction<'a> {
    connection: &'a mut sqlx::postgres::PgConnection,
    qualified_table_identifier: &'a str,
    qualified_table_name: &'a crate::QualifiedTableName,
}

impl Transaction<'_> {
    pub(crate) async fn with_transaction<T, F>(
        client_config: &pg_client::Config,
        qualified_table_name: &crate::QualifiedTableName,
        mut action: F,
    ) -> Result<T, crate::ContextError>
    where
        F: AsyncFnMut(&mut Transaction) -> Result<T, crate::ContextError>,
    {
        client_config
            .with_sqlx_connection(async |connection| {
                let qualified_table_identifier =
                    Self::read_qualified_table_identifier(&mut *connection, qualified_table_name)
                        .await?;

                pg_client::sqlx::transaction::with_transaction(
                    &mut *connection,
                    pg_client::sqlx::transaction::IsolationLevel::Serializable,
                    async |connection| {
                        let mut transaction = Transaction {
                            connection,
                            qualified_table_identifier: &qualified_table_identifier,
                            qualified_table_name,
                        };
                        action(&mut transaction).await
                    },
                )
                .await
                .map_err(crate::ContextError::from)
            })
            .await
            .map_err(crate::ContextError::from)?
    }

    async fn read_qualified_table_identifier(
        connection: &mut sqlx::postgres::PgConnection,
        qualified_table_name: &crate::QualifiedTableName,
    ) -> Result<String, crate::ContextError> {
        let row = sqlx::query(r#"SELECT format('%I.%I', $1, $2) table_identifier"#)
            .bind(&qualified_table_name.schema_name)
            .bind(&qualified_table_name.table_name)
            .fetch_one(&mut *connection)
            .await?;

        Ok(row.try_get("table_identifier")?)
    }

    /// Take an exclusive lock on the tracking table for this transaction.
    ///
    /// Uses `NOWAIT`: if another runner already holds the lock we do not block but
    /// surface [`crate::ContextError::MigrationLockUnavailable`] so the caller can
    /// decide whether a concurrent run is benign or fatal. The lock is released at
    /// commit, so it is held only for the current migration.
    async fn lock_applied_migrations_table(&mut self) -> Result<(), crate::ContextError> {
        let result = sqlx::raw_sql(AssertSqlSafe(format!(
            "LOCK TABLE {} IN ACCESS EXCLUSIVE MODE NOWAIT",
            self.qualified_table_identifier
        )))
        .execute(&mut *self.connection)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(error)
                if pg_client::sqlx::sqlstate::sqlstate(&error)
                    == Some(pg_client::sqlx::sqlstate::SqlState::LOCK_NOT_AVAILABLE) =>
            {
                Err(crate::ContextError::MigrationLockUnavailable {
                    schema: self.qualified_table_name.schema_name.clone(),
                    table: self.qualified_table_name.table_name.clone(),
                })
            }
            Err(error) => Err(error.into()),
        }
    }

    /// Create the tracking table, erroring if it already exists.
    pub(crate) async fn bootstrap(&mut self) -> Result<(), crate::ContextError> {
        if self.does_applied_migrations_table_exist().await? {
            return Err(crate::ContextError::AlreadyBootstrapped {
                schema: self.qualified_table_name.schema_name.clone(),
                table: self.qualified_table_name.table_name.clone(),
            });
        }

        self.create_applied_migrations_table().await
    }

    pub(crate) async fn find_last_applied_index(&mut self) -> Result<Index, crate::ContextError> {
        if !self.does_applied_migrations_table_exist().await? {
            return Err(crate::ContextError::NotBootstrapped {
                schema: self.qualified_table_name.schema_name.clone(),
                table: self.qualified_table_name.table_name.clone(),
            });
        }

        Ok(self.read_applied_migrations_comment().await?.index())
    }

    pub(crate) async fn apply_pending_migration(
        &mut self,
        pending_migration: &PendingMigration,
    ) -> Result<MigrationOutcome, crate::ContextError> {
        // Coordinate concurrent runners: take an exclusive lock on the tracking
        // table for the duration of this transaction, then re-read the committed
        // last-applied index. A peer that applied this migration first commits
        // (and releases the lock) before we acquire it, so we observe its result
        // and skip rather than re-running the migration.
        self.lock_applied_migrations_table().await?;

        let last_applied = self.read_applied_migrations_comment().await?.index();
        let expected = last_applied.succ()?;

        match pending_migration.index.cmp(&expected) {
            std::cmp::Ordering::Less => {
                log::info!(
                    "Migration {} already applied; skipping",
                    pending_migration.index
                );
                return Ok(MigrationOutcome::Skipped);
            }
            std::cmp::Ordering::Greater => {
                return Err(crate::ContextError::Pending(
                    crate::PendingError::ExpectedSuccessor {
                        last: last_applied,
                        expected,
                        got: pending_migration.index,
                    },
                ));
            }
            std::cmp::Ordering::Equal => {}
        }

        log::info!("Applying migration: {}", pending_migration.index);

        sqlx::raw_sql(&pending_migration.raw_sql)
            .execute(&mut *self.connection)
            .await
            .map_err(|source| crate::ContextError::ApplyMigration {
                index: pending_migration.index,
                source,
            })?;

        sqlx::query(AssertSqlSafe(format!(
            r#"
                INSERT INTO
                  {}
                  ( index
                  , digest
                  , name
                  )
                VALUES
                  ( $1
                  , $2
                  , $3
                  )
            "#,
            self.qualified_table_identifier
        )))
        .bind(pending_migration.index)
        .bind(pending_migration.digest())
        .bind(&pending_migration.name)
        .execute(&mut *self.connection)
        .await?;

        self.set_applied_migrations_comment(AppliedMigrationsComment::from_pending_migration(
            pending_migration,
        ))
        .await?;

        Ok(MigrationOutcome::Applied)
    }

    async fn set_applied_migrations_comment(
        &mut self,
        comment: AppliedMigrationsComment,
    ) -> Result<(), crate::ContextError> {
        // we use a temporary function to generate the SQL string literal for the comment safely PG
        // server side. PG does not support binds in place the string literal.
        sqlx::raw_sql(AssertSqlSafe(format!(
            r#"
                    CREATE FUNCTION
                      pg_temp.set_applied_migrations_comment(arg_comment text)
                    RETURNS
                      void
                    LANGUAGE
                      plpgsql
                    AS $$
                      BEGIN
                        EXECUTE format('COMMENT ON TABLE {} IS %L', arg_comment);
                      END;
                    $$
                "#,
            self.qualified_table_identifier
        )))
        .execute(&mut *self.connection)
        .await?;

        sqlx::query("SELECT pg_temp.set_applied_migrations_comment($1)")
            .bind(comment.render())
            .execute(&mut *self.connection)
            .await?;

        sqlx::raw_sql("DROP FUNCTION pg_temp.set_applied_migrations_comment")
            .execute(&mut *self.connection)
            .await?;

        Ok(())
    }

    async fn create_applied_migrations_table(&mut self) -> Result<(), crate::ContextError> {
        log::info!("Creating applied migrations table");

        sqlx::query(AssertSqlSafe(format!(
            r#"
                CREATE TABLE
                  {}
                  ( index int8                    PRIMARY KEY
                  , applied_by text               NOT NULL DEFAULT current_role
                  , digest bytea                  NOT NULL CHECK (octet_length(digest) = 32)
                  , elapsed interval              NOT NULL DEFAULT (clock_timestamp() - transaction_timestamp())
                  , name text                     NOT NULL CHECK (char_length(name) BETWEEN 1 AND 128)
                  , transaction_id bigint         NOT NULL DEFAULT txid_current()
                  , transaction_start timestamptz NOT NULL DEFAULT transaction_timestamp()
                  )
            "#,
            self.qualified_table_identifier
        )))
        .execute(&mut *self.connection)
        .await?;

        self.set_applied_migrations_comment(AppliedMigrationsComment::NoAppliedMigrations)
            .await?;

        Ok(())
    }

    async fn does_applied_migrations_table_exist(&mut self) -> Result<bool, crate::ContextError> {
        let row = sqlx::query(
            r#"
            SELECT
              EXISTS(
                SELECT
                FROM
                  information_schema.tables
                WHERE
                  (table_schema, table_name) = ($1, $2)
             )
        "#,
        )
        .bind(&self.qualified_table_name.schema_name)
        .bind(&self.qualified_table_name.table_name)
        .fetch_one(&mut *self.connection)
        .await?;

        Ok(row.try_get(0)?)
    }

    async fn read_applied_migrations_comment(
        &mut self,
    ) -> Result<AppliedMigrationsComment, crate::ContextError> {
        let row = sqlx::query(
            r#"
            SELECT
              description
            FROM
              pg_class
            JOIN
              pg_description
            ON
              pg_class.oid = pg_description.objoid
            WHERE
              relkind = 'r'
            AND
              relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = $1)
            AND
              relname = $2
            ;
        "#,
        )
        .bind(&self.qualified_table_name.schema_name)
        .bind(&self.qualified_table_name.table_name)
        .fetch_one(&mut *self.connection)
        .await?;

        let comment: String = row.try_get(0)?;

        <AppliedMigrationsComment as std::str::FromStr>::from_str(&comment).map_err(|report| {
            crate::ContextError::ParseAppliedMigrationsComment { comment, report }
        })
    }
}
