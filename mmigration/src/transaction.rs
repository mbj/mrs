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

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error(transparent)]
    Connection(#[from] pg_client::sqlx::ConnectionError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Failed to parse applied migrations table comment: {comment}\n{report}")]
    ParseAppliedMigrationsComment { comment: String, report: String },
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

    fn index(&self) -> Option<Index> {
        match self {
            Self::LastAppliedMigration { index, .. } => Some(*index),
            Self::NoAppliedMigrations => None,
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
    pub(crate) async fn with_transaction<T, F: AsyncFnMut(&mut Transaction) -> T>(
        client_config: &pg_client::Config,
        qualified_table_name: &crate::QualifiedTableName,
        mut action: F,
    ) -> Result<T, TransactionError> {
        client_config
            .with_sqlx_connection(async |connection| {
                let qualified_table_identifier =
                    Self::read_qualified_table_identifier(&mut *connection, qualified_table_name)
                        .await?;
                Self::begin_serializable_transaction(&mut *connection).await?;
                let mut transaction = Transaction {
                    connection: &mut *connection,
                    qualified_table_identifier: &qualified_table_identifier,
                    qualified_table_name,
                };
                let result = action(&mut transaction).await;
                Self::commit_transaction(&mut *connection).await?;
                Ok::<T, TransactionError>(result)
            })
            .await
            .map_err(TransactionError::from)?
    }

    async fn read_qualified_table_identifier(
        connection: &mut sqlx::postgres::PgConnection,
        qualified_table_name: &crate::QualifiedTableName,
    ) -> Result<String, TransactionError> {
        let row = sqlx::query(r#"SELECT format('%I.%I', $1, $2) table_identifier"#)
            .bind(&qualified_table_name.schema_name)
            .bind(&qualified_table_name.table_name)
            .fetch_one(&mut *connection)
            .await?;

        Ok(row.try_get("table_identifier")?)
    }

    pub(crate) async fn find_last_applied_index(
        &mut self,
    ) -> Result<Option<Index>, TransactionError> {
        if !self.does_applied_migrations_table_exist().await? {
            return Ok(None);
        }

        Ok(self.read_applied_migrations_comment().await?.index())
    }

    pub(crate) async fn apply_pending_migration(
        &mut self,
        pending_migration: &PendingMigration,
    ) -> Result<(), TransactionError> {
        self.create_applied_migrations_table().await?;

        log::info!("Appying migration: {}", pending_migration.index);

        sqlx::raw_sql(&pending_migration.raw_sql)
            .execute(&mut *self.connection)
            .await?;

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

        Ok(())
    }

    async fn set_applied_migrations_comment(
        &mut self,
        comment: AppliedMigrationsComment,
    ) -> Result<(), TransactionError> {
        // we use a termporary function to generate the SQL string literal for the comment safely PG
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

    async fn create_applied_migrations_table(&mut self) -> Result<(), TransactionError> {
        if !self.does_applied_migrations_table_exist().await? {
            log::info!("Applied migrations table does not exist, creating it!");

            sqlx::query(
                AssertSqlSafe(
                format!(
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
                ))
            )
            .execute(&mut *self.connection)
            .await?;

            self.set_applied_migrations_comment(AppliedMigrationsComment::NoAppliedMigrations)
                .await?;
        }

        Ok(())
    }

    async fn does_applied_migrations_table_exist(&mut self) -> Result<bool, TransactionError> {
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
    ) -> Result<AppliedMigrationsComment, TransactionError> {
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

        <AppliedMigrationsComment as std::str::FromStr>::from_str(&comment)
            .map_err(|report| TransactionError::ParseAppliedMigrationsComment { comment, report })
    }

    async fn begin_serializable_transaction(
        connection: &mut sqlx::postgres::PgConnection,
    ) -> Result<(), TransactionError> {
        sqlx::query("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .execute(connection)
            .await?;

        Ok(())
    }

    async fn commit_transaction(
        connection: &mut sqlx::postgres::PgConnection,
    ) -> Result<(), TransactionError> {
        sqlx::query("COMMIT").execute(&mut *connection).await?;

        Ok(())
    }
}
