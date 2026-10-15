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
use pg_client::protocol::frontend;
use pg_client::session::{self, execute};

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error(transparent)]
    Connection(#[from] session::OpenError),
    #[error(transparent)]
    Execute(#[from] execute::Error),
    #[error(transparent)]
    Column(#[from] execute::ColumnError),
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
    session: &'a mut session::Session,
    qualified_table_name: &'a crate::QualifiedTableName,
}

impl Transaction<'_> {
    pub(crate) async fn with_transaction<T, F: AsyncFnMut(&mut Transaction) -> T>(
        session_config: std::sync::Arc<session::Config>,
        qualified_table_name: &crate::QualifiedTableName,
        mut action: F,
    ) -> Result<T, TransactionError> {
        session::with_session(session_config, async |session| {
            execute(session, &BEGIN_SERIALIZABLE).await?;

            let mut transaction = Transaction {
                session: &mut *session,
                qualified_table_name,
            };

            let result = action(&mut transaction).await;

            execute(session, &COMMIT).await?;

            Ok::<T, TransactionError>(result)
        })
        .await?
    }

    async fn generate(
        &mut self,
        generator: &frontend::Sql,
        argument: Option<&str>,
    ) -> Result<frontend::Sql, TransactionError> {
        let mut builder = self
            .session
            .statement(generator)
            .bind(self.qualified_table_name.schema_name.as_str())
            .bind(self.qualified_table_name.table_name.as_str());

        if let Some(argument) = argument {
            builder = builder.bind(argument);
        }

        let mut reader = builder.execute()?;
        let description = reader.describe().await?;
        let statement = description.column::<frontend::Sql>(0)?;

        Ok(reader.decode(|row| row.decode(&statement)).one().await?)
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

        self.session
            .statements(pending_migration.raw_sql.as_sql())
            .await?;

        let insert = self.generate(&GENERATE_INSERT, None).await?;

        self.session
            .statement(&insert)
            .bind(pending_migration.index)
            .bind(pending_migration.digest().as_slice())
            .bind(&pending_migration.name)
            .execute()?
            .discard_rows()
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
        let comment = self
            .generate(&GENERATE_COMMENT, Some(&comment.render()))
            .await?;

        execute(self.session, &comment).await
    }

    async fn create_applied_migrations_table(&mut self) -> Result<(), TransactionError> {
        if !self.does_applied_migrations_table_exist().await? {
            log::info!("Applied migrations table does not exist, creating it!");

            let create = self.generate(&GENERATE_CREATE_TABLE, None).await?;

            execute(self.session, &create).await?;

            self.set_applied_migrations_comment(AppliedMigrationsComment::NoAppliedMigrations)
                .await?;
        }

        Ok(())
    }

    async fn does_applied_migrations_table_exist(&mut self) -> Result<bool, TransactionError> {
        const EXISTS: frontend::Sql = frontend::Sql::from_static_or_panic(
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
        );

        let mut reader = self
            .session
            .statement(&EXISTS)
            .bind(self.qualified_table_name.schema_name.as_str())
            .bind(self.qualified_table_name.table_name.as_str())
            .execute()?;

        let description = reader.describe().await?;
        let exists = description.column::<bool>(0)?;

        Ok(reader.decode(|row| row.decode(&exists)).one().await?)
    }

    async fn read_applied_migrations_comment(
        &mut self,
    ) -> Result<AppliedMigrationsComment, TransactionError> {
        const DESCRIPTION: frontend::Sql = frontend::Sql::from_static_or_panic(
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
        );

        let mut reader = self
            .session
            .statement(&DESCRIPTION)
            .bind(self.qualified_table_name.schema_name.as_str())
            .bind(self.qualified_table_name.table_name.as_str())
            .execute()?;

        let description = reader.describe().await?;
        let column = description.column::<String>(0)?;

        let comment = reader.decode(|row| row.decode(&column)).one().await?;

        <AppliedMigrationsComment as std::str::FromStr>::from_str(&comment)
            .map_err(|report| TransactionError::ParseAppliedMigrationsComment { comment, report })
    }
}

const BEGIN_SERIALIZABLE: frontend::Sql =
    frontend::Sql::from_static_or_panic("BEGIN ISOLATION LEVEL SERIALIZABLE");

const COMMIT: frontend::Sql = frontend::Sql::from_static_or_panic("COMMIT");

const GENERATE_INSERT: frontend::Sql = frontend::Sql::from_static_or_panic(
    r#"
    SELECT
      format(
        'INSERT INTO %I.%I (index, digest, name) VALUES ($1, $2, $3)'
      , $1::text
      , $2::text
      )
    "#,
);

const GENERATE_COMMENT: frontend::Sql = frontend::Sql::from_static_or_panic(
    r#"
    SELECT
      format('COMMENT ON TABLE %I.%I IS %L', $1::text, $2::text, $3::text)
    "#,
);

const GENERATE_CREATE_TABLE: frontend::Sql = frontend::Sql::from_static_or_panic(
    r#"
    SELECT
      format(
        'CREATE TABLE
           %I.%I
           ( index int8                    PRIMARY KEY
           , applied_by text               NOT NULL DEFAULT current_role
           , digest bytea                  NOT NULL CHECK (octet_length(digest) = 32)
           , elapsed interval              NOT NULL DEFAULT (clock_timestamp() - transaction_timestamp())
           , name text                     NOT NULL CHECK (char_length(name) BETWEEN 1 AND 128)
           , transaction_id bigint         NOT NULL DEFAULT txid_current()
           , transaction_start timestamptz NOT NULL DEFAULT transaction_timestamp()
           )'
      , $1::text
      , $2::text
      )
    "#,
);

async fn execute(
    session: &mut session::Session,
    sql: &frontend::Sql,
) -> Result<(), TransactionError> {
    Ok(session.statement(sql).execute()?.discard_rows().await?)
}
