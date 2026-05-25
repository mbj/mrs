use crate::types::{Index, IndexError, MigrationName, PendingMigration, migration_name_parser};
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::context,
};
use nom_language::error::VerboseError;

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AddError {
    #[error("Expected next defined migration index: {expected} got: {got}")]
    NonConsecutive { expected: Index, got: Index },
    #[error(transparent)]
    IndexError {
        #[from]
        source: IndexError,
    },
    #[error(transparent)]
    PendingError {
        #[from]
        source: PendingError,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PendingError {
    #[error("Last migration {last} needs to be followed by {expected}, got: {got}!")]
    ExpectedSuccessor {
        last: Index,
        expected: Index,
        got: Index,
    },
    #[error(
        "Migration index {got} is reserved for the baseline; migrations must start at index 1!"
    )]
    InitialIndex { got: Index },
    #[error(transparent)]
    IndexError {
        #[from]
        source: IndexError,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("I/O error while {operation} on {path}: {source}")]
    IoError {
        operation: &'static str,
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Migration dir entry is not a file: {path}")]
    NonFileEntry { path: std::path::PathBuf },
    #[error("Migration file name is invalid: {path}\n{report}")]
    InvalidFileName {
        path: std::path::PathBuf,
        report: String,
    },
    #[error("Migration file has no file name: {path}")]
    MissingFileName { path: std::path::PathBuf },
    #[error("Migration file name is not valid UTF-8: {path}")]
    NonUtf8FileName { path: std::path::PathBuf },
    #[error(transparent)]
    AddError {
        #[from]
        source: AddError,
    },
}

fn parse_migration_file_name(file_name: &str) -> Result<(Index, MigrationName), String> {
    fn parser(input: &str) -> ParseResult<'_, (Index, MigrationName)> {
        context(
            "migration filename ({index}_{name}.sql)",
            (
                context(
                    "migration index",
                    map_res(digit1, <Index as std::str::FromStr>::from_str),
                ),
                tag("_"),
                context("migration name", migration_name_parser),
                tag(".sql"),
            )
                .map(|(index, _, name, _)| (index, MigrationName::from_validated(name))),
        )
        .parse(input)
    }

    match all_consuming(parser).parse(file_name).finish() {
        Ok((_, parsed)) => Ok(parsed),
        Err(error) => Err(nom_language::error::convert_error(file_name, error)),
    }
}

#[derive(Debug)]
pub struct DefinedMigrations(std::collections::BTreeMap<Index, PendingMigration>);

impl Default for DefinedMigrations {
    fn default() -> Self {
        Self::new()
    }
}

impl DefinedMigrations {
    /// Load migrations from migration dir
    pub fn load(migration_dir: &std::path::Path) -> Result<Self, LoadError> {
        let reader = match std::fs::read_dir(migration_dir) {
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    return Ok(DefinedMigrations::new());
                }

                return Err(LoadError::IoError {
                    operation: "read_dir",
                    path: migration_dir.to_path_buf(),
                    source: error,
                });
            }
            Ok(iterator) => iterator,
        };

        let mut tuples = Vec::new();

        for entry in reader {
            let dir_entry = entry.map_err(|source| LoadError::IoError {
                operation: "read_dir_entry",
                path: migration_dir.to_path_buf(),
                source,
            })?;

            let path = dir_entry.path();
            let is_file = dir_entry
                .file_type()
                .map_err(|source| LoadError::IoError {
                    operation: "read_entry_file_type",
                    path: path.clone(),
                    source,
                })?
                .is_file();
            if !is_file {
                return Err(LoadError::NonFileEntry { path });
            }

            let Some(file_name_os) = path.file_name() else {
                return Err(LoadError::MissingFileName { path });
            };
            let Some(file_name) = file_name_os.to_str() else {
                return Err(LoadError::NonUtf8FileName { path });
            };
            let (index, name) = parse_migration_file_name(file_name).map_err(|report| {
                LoadError::InvalidFileName {
                    path: path.clone(),
                    report,
                }
            })?;

            let raw_sql = std::fs::read_to_string(&path).map_err(|source| LoadError::IoError {
                operation: "read_to_string",
                path,
                source,
            })?;

            tuples.push(PendingMigration {
                index,
                name,
                raw_sql: raw_sql.into(),
            });
        }

        tuples.sort_by_key(|pending_migration| pending_migration.index);

        let mut defined_migrations = DefinedMigrations::new();

        for pending_migration in tuples {
            defined_migrations.add(pending_migration)?;
        }

        Ok(defined_migrations)
    }

    /// Return next index
    ///
    /// This method only returns an index if there is already a defined migration
    /// Else this method returns None.
    pub fn next_index(&self) -> Result<Option<Index>, IndexError> {
        self.0
            .last_key_value()
            .map(|(index, _)| index.succ())
            .transpose()
    }

    /// Return an empty migration set.
    #[must_use]
    pub fn new() -> Self {
        DefinedMigrations(std::collections::BTreeMap::new())
    }

    /// Add new defined migration
    ///
    /// Migrations are indexed consecutively starting at 1; index 0 is reserved for
    /// the baseline (the presence of the tracking table).
    pub fn add(&mut self, pending_migration: PendingMigration) -> Result<(), AddError> {
        if pending_migration.index.is_baseline() {
            return Err(AddError::PendingError {
                source: PendingError::InitialIndex {
                    got: pending_migration.index,
                },
            });
        }

        if let Some(expected) = self.next_index()?
            && pending_migration.index != expected
        {
            return Err(AddError::NonConsecutive {
                expected,
                got: pending_migration.index,
            });
        }

        let index = pending_migration.index;
        let previous = self.0.insert(index, pending_migration);

        if let Err(source) = self.select_pending(Index::baseline()) {
            if let Some(previous) = previous {
                self.0.insert(index, previous);
            } else {
                self.0.remove(&index);
            }

            return Err(AddError::PendingError { source });
        }

        Ok(())
    }

    /// Select pending migrations after the given last-applied index.
    ///
    /// The lowest pending migration must be the successor of `last` (after the
    /// baseline 0 that is the first migration, index 1).
    pub fn select_pending(&self, last: Index) -> Result<Vec<&PendingMigration>, PendingError> {
        let pending: Vec<_> = self
            .0
            .iter()
            .filter_map(|(index, pending_migration)| (*index > last).then_some(pending_migration))
            .collect();

        if let Some(migration) = pending.first()
            && !migration.index.is_succ_of(last)
        {
            return Err(PendingError::ExpectedSuccessor {
                last,
                expected: last.succ()?,
                got: migration.index,
            });
        }

        Ok(pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pending(index: u32, name: &str, raw_sql: &str) -> PendingMigration {
        PendingMigration {
            index: index.into(),
            name: name.parse().unwrap(),
            raw_sql: raw_sql.into(),
        }
    }

    #[test]
    fn new_is_empty() {
        let defined = DefinedMigrations::new();

        assert_eq!(Ok(None), defined.next_index());
        assert_eq!(
            Vec::<&PendingMigration>::new(),
            defined.select_pending(Index::baseline()).unwrap()
        );
    }

    #[test]
    fn add_accepts_consecutive_from_one() {
        let mut defined = DefinedMigrations::new();

        defined.add(pending(1, "example", "SELECT 1")).unwrap();
        defined.add(pending(2, "example", "SELECT 2")).unwrap();

        assert_eq!(Ok(Some(3_u32.into())), defined.next_index());
    }

    #[test]
    fn add_rejects_baseline_index_zero() {
        let mut defined = DefinedMigrations::new();

        assert_eq!(
            Err(AddError::PendingError {
                source: PendingError::InitialIndex { got: 0_u32.into() },
            }),
            defined.add(pending(0, "example", "SELECT 0"))
        );
    }

    #[test]
    fn add_rejects_first_migration_above_one() {
        let mut defined = DefinedMigrations::new();

        assert_eq!(
            Err(AddError::PendingError {
                source: PendingError::ExpectedSuccessor {
                    last: 0_u32.into(),
                    expected: 1_u32.into(),
                    got: 2_u32.into(),
                },
            }),
            defined.add(pending(2, "example", "SELECT 2"))
        );
    }

    #[test]
    fn add_rejects_non_consecutive() {
        let mut defined = DefinedMigrations::new();
        defined.add(pending(1, "example", "SELECT 1")).unwrap();

        assert_eq!(
            Err(AddError::NonConsecutive {
                expected: 2_u32.into(),
                got: 3_u32.into(),
            }),
            defined.add(pending(3, "example", "SELECT 3"))
        );
    }

    #[test]
    fn add_rejects_duplicate_index() {
        let mut defined = DefinedMigrations::new();
        defined.add(pending(1, "example", "SELECT 1")).unwrap();

        assert_eq!(
            Err(AddError::NonConsecutive {
                expected: 2_u32.into(),
                got: 1_u32.into(),
            }),
            defined.add(pending(1, "example", "SELECT other"))
        );
    }

    #[test]
    fn select_pending_for_partially_applied() {
        let mut defined = DefinedMigrations::new();
        defined.add(pending(1, "example", "SELECT 1")).unwrap();
        defined.add(pending(2, "example", "SELECT 2")).unwrap();

        assert_eq!(
            vec![&pending(2, "example", "SELECT 2")],
            defined.select_pending(1_u32.into()).unwrap()
        );
    }

    #[test]
    fn select_pending_from_baseline_returns_all() {
        let mut defined = DefinedMigrations::new();
        defined.add(pending(1, "example", "SELECT 1")).unwrap();
        defined.add(pending(2, "example", "SELECT 2")).unwrap();

        assert_eq!(
            vec![
                &pending(1, "example", "SELECT 1"),
                &pending(2, "example", "SELECT 2"),
            ],
            defined.select_pending(Index::baseline()).unwrap()
        );
    }

    #[test]
    fn select_pending_rejects_non_successor_when_state_is_invalid() {
        let defined = DefinedMigrations(std::collections::BTreeMap::from([(
            3_u32.into(),
            pending(3, "example", "SELECT 3"),
        )]));

        assert_eq!(
            Err(PendingError::ExpectedSuccessor {
                last: 1_u32.into(),
                expected: 2_u32.into(),
                got: 3_u32.into(),
            }),
            defined.select_pending(1_u32.into())
        );
    }

    #[test]
    fn next_index_overflow_returns_error() {
        let defined = DefinedMigrations(std::collections::BTreeMap::from([(
            u32::MAX.into(),
            pending(u32::MAX, "example", "SELECT 1"),
        )]));

        assert_eq!(
            Err(IndexError::Overflow {
                index: u32::MAX.into()
            }),
            defined.next_index()
        );
    }
}
