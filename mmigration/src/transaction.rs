use crate::types::*;
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::context,
};
use nom_language::error::VerboseError;
use sqlx::AssertSqlSafe;
use sqlx::Row as _;

/// Whether `apply_migration` actually applied the migration or found it
/// already applied (by a concurrent runner) and skipped it.
pub(crate) enum MigrationOutcome {
    Applied,
    Skipped,
}

pub enum AppliedMigrationsComment {
    LastAppliedMigration {
        epoch: Epoch,
        index: Index,
        name: MigrationName,
        chain: Chain,
    },
    NoAppliedMigrations {
        epoch: Epoch,
    },
}

impl AppliedMigrationsComment {
    fn render(&self) -> String {
        match self {
            Self::NoAppliedMigrations { epoch } => {
                format!("Epoch {epoch}: no applied migrations")
            }
            Self::LastAppliedMigration {
                epoch,
                index,
                name,
                chain,
            } => format!("Epoch {epoch}: last applied migration {index}, {name}, chain {chain}"),
        }
    }

    /// The last-applied index. `NoAppliedMigrations` is the baseline (0): the
    /// tracking table exists but no migration has been applied beyond it.
    fn index(&self) -> Index {
        match self {
            Self::LastAppliedMigration { index, .. } => *index,
            Self::NoAppliedMigrations { .. } => Index::baseline(),
        }
    }

    fn epoch(&self) -> Epoch {
        match self {
            Self::LastAppliedMigration { epoch, .. } | Self::NoAppliedMigrations { epoch } => {
                *epoch
            }
        }
    }

    /// The chain value recorded so far. `NoAppliedMigrations` is the epoch's genesis
    /// seed.
    fn chain(&self) -> Chain {
        match self {
            Self::LastAppliedMigration { chain, .. } => *chain,
            Self::NoAppliedMigrations { epoch } => Chain::seed(*epoch),
        }
    }
}

impl std::str::FromStr for AppliedMigrationsComment {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

        fn parser(input: &str) -> ParseResult<'_, AppliedMigrationsComment> {
            let (input, _) = tag("Epoch ").parse(input)?;
            let (input, epoch) = context(
                "epoch",
                map_res(digit1, <Epoch as std::str::FromStr>::from_str),
            )
            .parse(input)?;
            let (input, _) = tag(": ").parse(input)?;

            alt((
                context(
                    "no applied migrations comment",
                    tag("no applied migrations")
                        .map(move |_| AppliedMigrationsComment::NoAppliedMigrations { epoch }),
                ),
                context(
                    "last applied migration comment",
                    (
                        tag("last applied migration "),
                        context(
                            "migration index",
                            map_res(digit1, <Index as std::str::FromStr>::from_str),
                        ),
                        tag(", "),
                        context("migration name", migration_name_parser),
                        tag(", chain "),
                        context(
                            "migration chain",
                            map_res(
                                take_while_m_n(64, 64, |character: char| {
                                    character.is_ascii_hexdigit()
                                }),
                                <Chain as std::str::FromStr>::from_str,
                            ),
                        ),
                    )
                        .map(move |(_, index, _, name, _, chain)| {
                            AppliedMigrationsComment::LastAppliedMigration {
                                epoch,
                                index,
                                name: MigrationName::from_validated(name),
                                chain,
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

/// Why [`verify_applied`] rejected the recorded migration state.
///
/// Split by *which* of the two independent digest records diverged, so the caller
/// can tell metadata tampering apart from an edited migration file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum VerifyFailure {
    /// The tracking table's own records are inconsistent: the per-row digests do
    /// not reproduce the chain recorded in the comment, or the rows do not cover
    /// exactly indices `1..=last`. Points at hand-edited metadata.
    HistoryCorruption { detail: String },
    /// An index recorded as applied has no corresponding file on disk.
    MigrationMissing { index: Index },
    /// An applied migration's file digest differs from the digest recorded when it
    /// was applied — the `.sql` was edited after the fact.
    SchemaDrift {
        index: Index,
        recorded: Digest,
        current: Digest,
    },
}

/// Verify the recorded migration state against both independent digest records.
///
/// `comment_chain`/`last` come from the table comment (the value carried by
/// `pg_dump`); `rows` are the `(index, digest)` history rows; `disk` yields the
/// current file digest for an index. Two checks, two failure classes:
///
/// 1. the row digests must reproduce `comment_chain` and cover exactly `1..=last`
///    ([`VerifyFailure::HistoryCorruption`]); then
/// 2. each applied index's file digest must match its recorded row digest
///    ([`VerifyFailure::SchemaDrift`] / [`VerifyFailure::MigrationMissing`]).
fn verify_applied(
    epoch: Epoch,
    comment_chain: Chain,
    last: Index,
    rows: &[(Index, Digest)],
    disk: impl Fn(Index) -> Option<Digest>,
) -> Result<(), VerifyFailure> {
    // 1a. Rows cover exactly 1..=last, consecutively.
    let mut expected = Index::baseline();
    for (row_index, _) in rows {
        expected = expected
            .succ()
            .map_err(|error| VerifyFailure::HistoryCorruption {
                detail: format!("tracking table rows overflow the index space: {error}"),
            })?;
        if *row_index != expected {
            return Err(VerifyFailure::HistoryCorruption {
                detail: format!(
                    "tracking table rows are not consecutive: expected index {expected}, found {row_index}"
                ),
            });
        }
    }
    if expected != last {
        return Err(VerifyFailure::HistoryCorruption {
            detail: format!(
                "table comment records last applied index {last}, but tracking table rows end at {expected}"
            ),
        });
    }

    // 1b. The row digests must reproduce the chain recorded in the comment.
    let recomputed = rows.iter().fold(Chain::seed(epoch), |chain, (_, digest)| {
        chain.extend(*digest)
    });
    if recomputed != comment_chain {
        return Err(VerifyFailure::HistoryCorruption {
            detail: format!(
                "chain recomputed from tracking table rows {recomputed} does not match the chain recorded in the table comment {comment_chain}"
            ),
        });
    }

    // 2. Each applied file must still match the digest recorded for it.
    for (index, recorded) in rows {
        match disk(*index) {
            None => return Err(VerifyFailure::MigrationMissing { index: *index }),
            Some(current) if current != *recorded => {
                return Err(VerifyFailure::SchemaDrift {
                    index: *index,
                    recorded: *recorded,
                    current,
                });
            }
            Some(_) => {}
        }
    }

    Ok(())
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
                    table: self.qualified_table_name.clone(),
                })
            }
            Err(error) => Err(error.into()),
        }
    }

    /// Create the tracking table, erroring if it already exists.
    pub(crate) async fn bootstrap(&mut self) -> Result<(), crate::ContextError> {
        if self.does_applied_migrations_table_exist().await? {
            return Err(crate::ContextError::AlreadyBootstrapped {
                table: self.qualified_table_name.clone(),
            });
        }

        self.create_applied_migrations_table().await
    }

    pub(crate) async fn find_last_applied_index(&mut self) -> Result<Index, crate::ContextError> {
        if !self.does_applied_migrations_table_exist().await? {
            return Err(crate::ContextError::NotBootstrapped {
                table: self.qualified_table_name.clone(),
            });
        }

        Ok(self.read_applied_migrations_comment().await?.index())
    }

    pub(crate) async fn apply_migration(
        &mut self,
        pending_migration: &DefinedMigration,
    ) -> Result<MigrationOutcome, crate::ContextError> {
        // Coordinate concurrent runners: take an exclusive lock on the tracking
        // table for the duration of this transaction, then re-read the committed
        // last-applied index. A peer that applied this migration first commits
        // (and releases the lock) before we acquire it, so we observe its result
        // and skip rather than re-running the migration.
        self.lock_applied_migrations_table().await?;

        let comment = self.read_applied_migrations_comment().await?;
        let last_applied = comment.index();
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
        .bind(*pending_migration.digest().as_bytes())
        .bind(&pending_migration.name)
        .execute(&mut *self.connection)
        .await?;

        // Fold this migration's digest into the chain recorded in the comment.
        self.set_applied_migrations_comment(AppliedMigrationsComment::LastAppliedMigration {
            epoch: comment.epoch(),
            index: pending_migration.index,
            name: pending_migration.name.clone(),
            chain: comment.chain().extend(pending_migration.digest()),
        })
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

        self.set_applied_migrations_comment(AppliedMigrationsComment::NoAppliedMigrations {
            epoch: Epoch::zero(),
        })
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

    /// Verify the recorded migration state against the migration files on disk.
    ///
    /// `disk` yields the current digest of the migration file at an index. The
    /// recorded chain and per-row digests are read inside this (serializable)
    /// transaction for a consistent snapshot; the comparison itself is delegated to
    /// [`verify_applied`].
    pub(crate) async fn verify(
        &mut self,
        disk: impl Fn(Index) -> Option<Digest>,
    ) -> Result<(), crate::ContextError> {
        if !self.does_applied_migrations_table_exist().await? {
            return Err(crate::ContextError::NotBootstrapped {
                table: self.qualified_table_name.clone(),
            });
        }

        let comment = self.read_applied_migrations_comment().await?;
        let rows = self.read_applied_rows().await?;

        verify_applied(
            comment.epoch(),
            comment.chain(),
            comment.index(),
            &rows,
            disk,
        )
        .map_err(|failure| match failure {
            VerifyFailure::HistoryCorruption { detail } => crate::ContextError::HistoryCorruption {
                table: self.qualified_table_name.clone(),
                detail,
            },
            VerifyFailure::MigrationMissing { index } => {
                crate::ContextError::MigrationMissing { index }
            }
            VerifyFailure::SchemaDrift {
                index,
                recorded,
                current,
            } => crate::ContextError::SchemaDrift {
                index,
                recorded,
                current,
            },
        })
    }

    /// Read the applied-migration history rows as `(index, digest)`, ordered by index.
    async fn read_applied_rows(&mut self) -> Result<Vec<(Index, Digest)>, crate::ContextError> {
        let rows = sqlx::query(AssertSqlSafe(format!(
            "SELECT index, digest FROM {} ORDER BY index ASC",
            self.qualified_table_identifier
        )))
        .fetch_all(&mut *self.connection)
        .await?;

        let mut applied = Vec::with_capacity(rows.len());
        for row in rows {
            let index: Index = row.try_get("index")?;
            let bytes: Vec<u8> = row.try_get("digest")?;
            let length = bytes.len();
            let digest: [u8; 32] =
                bytes
                    .try_into()
                    .map_err(|_| crate::ContextError::HistoryCorruption {
                        table: self.qualified_table_name.clone(),
                        detail: format!(
                            "tracking table row {index} has a {length}-byte digest, expected 32"
                        ),
                    })?;
            applied.push((index, Digest::from(digest)));
        }

        Ok(applied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn digest(byte: u8) -> Digest {
        Digest::from([byte; 32])
    }

    fn chain_of(digests: &[Digest]) -> Chain {
        digests
            .iter()
            .fold(Chain::seed(Epoch::zero()), |chain, digest| {
                chain.extend(*digest)
            })
    }

    fn verify(
        comment_chain: Chain,
        last: u32,
        rows: &[(Index, Digest)],
        disk: &BTreeMap<Index, Digest>,
    ) -> Result<(), VerifyFailure> {
        verify_applied(Epoch::zero(), comment_chain, last.into(), rows, |index| {
            disk.get(&index).copied()
        })
    }

    #[test]
    fn verify_applied_accepts_matching_state() {
        let rows = vec![(1_u32.into(), digest(1)), (2_u32.into(), digest(2))];
        let comment_chain = chain_of(&[digest(1), digest(2)]);
        let disk: BTreeMap<Index, Digest> = rows.iter().copied().collect();

        assert_eq!(Ok(()), verify(comment_chain, 2, &rows, &disk));
    }

    #[test]
    fn verify_applied_accepts_empty_baseline() {
        assert_eq!(
            Ok(()),
            verify(Chain::seed(Epoch::zero()), 0, &[], &BTreeMap::new())
        );
    }

    #[test]
    fn verify_applied_rejects_non_consecutive_rows() {
        let rows = vec![(1_u32.into(), digest(1)), (3_u32.into(), digest(3))];
        let disk: BTreeMap<Index, Digest> = rows.iter().copied().collect();

        assert_eq!(
            Err(VerifyFailure::HistoryCorruption {
                detail: "tracking table rows are not consecutive: expected index 2, found 3"
                    .to_owned(),
            }),
            verify(chain_of(&[digest(1), digest(3)]), 3, &rows, &disk)
        );
    }

    #[test]
    fn verify_applied_rejects_last_index_mismatch() {
        let rows = vec![(1_u32.into(), digest(1)), (2_u32.into(), digest(2))];
        let disk: BTreeMap<Index, Digest> = rows.iter().copied().collect();

        assert_eq!(
            Err(VerifyFailure::HistoryCorruption {
                detail:
                    "table comment records last applied index 3, but tracking table rows end at 2"
                        .to_owned(),
            }),
            verify(chain_of(&[digest(1), digest(2)]), 3, &rows, &disk)
        );
    }

    #[test]
    fn verify_applied_rejects_chain_mismatch() {
        let rows = vec![(1_u32.into(), digest(1)), (2_u32.into(), digest(2))];
        let disk: BTreeMap<Index, Digest> = rows.iter().copied().collect();
        let wrong = chain_of(&[digest(1)]);
        let recomputed = chain_of(&[digest(1), digest(2)]);

        assert_eq!(
            Err(VerifyFailure::HistoryCorruption {
                detail: format!(
                    "chain recomputed from tracking table rows {recomputed} does not match the chain recorded in the table comment {wrong}"
                ),
            }),
            verify(wrong, 2, &rows, &disk)
        );
    }

    #[test]
    fn verify_applied_reports_missing_file() {
        let rows = vec![(1_u32.into(), digest(1)), (2_u32.into(), digest(2))];
        let comment_chain = chain_of(&[digest(1), digest(2)]);
        // Index 2's file is absent from disk.
        let disk: BTreeMap<Index, Digest> = [(1_u32.into(), digest(1))].into_iter().collect();

        assert_eq!(
            Err(VerifyFailure::MigrationMissing {
                index: 2_u32.into(),
            }),
            verify(comment_chain, 2, &rows, &disk)
        );
    }

    #[test]
    fn verify_applied_reports_schema_drift() {
        let rows = vec![(1_u32.into(), digest(1)), (2_u32.into(), digest(2))];
        let comment_chain = chain_of(&[digest(1), digest(2)]);
        // Index 2's file was edited after it was applied.
        let disk: BTreeMap<Index, Digest> = [(1_u32.into(), digest(1)), (2_u32.into(), digest(99))]
            .into_iter()
            .collect();

        assert_eq!(
            Err(VerifyFailure::SchemaDrift {
                index: 2_u32.into(),
                recorded: digest(2),
                current: digest(99),
            }),
            verify(comment_chain, 2, &rows, &disk)
        );
    }

    #[test]
    fn applied_migrations_comment_round_trips() {
        // A non-zero epoch and a real chain exercise the full rendered format,
        // including the 64-hex chain field carried through pg_dump.
        let epoch: Epoch = "3".parse().unwrap();

        let last = AppliedMigrationsComment::LastAppliedMigration {
            epoch,
            index: 2_u32.into(),
            name: "add_users".parse().unwrap(),
            chain: chain_of(&[digest(1), digest(2)]),
        };
        let parsed: AppliedMigrationsComment = last.render().parse().unwrap();
        assert_eq!(last.render(), parsed.render());
        assert_eq!(epoch, parsed.epoch());
        assert_eq!(Index::from(2), parsed.index());
        assert_eq!(chain_of(&[digest(1), digest(2)]), parsed.chain());

        let none = AppliedMigrationsComment::NoAppliedMigrations { epoch };
        let parsed: AppliedMigrationsComment = none.render().parse().unwrap();
        assert_eq!(none.render(), parsed.render());
        assert_eq!(epoch, parsed.epoch());
        assert_eq!(Index::baseline(), parsed.index());
        assert_eq!(Chain::seed(epoch), parsed.chain());
    }
}
