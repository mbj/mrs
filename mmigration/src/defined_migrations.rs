use crate::types::{Index, MigrationName, PendingMigration};

#[derive(Debug)]
pub struct DefinedMigrations(std::collections::BTreeMap<Index, PendingMigration>);

impl DefinedMigrations {
    /// Load migrations from migration dir
    pub fn load(migration_dir: &std::path::Path) -> Self {
        let pattern = regex_lite::Regex::new(r#"\A(?<index>\d+)_(?<name>[^\.]+)\.sql\z"#).unwrap();

        let reader = std::fs::read_dir(migration_dir).unwrap_or_else(|error| {
            panic!(
                "Migration dir: {} not readable: {error}",
                migration_dir.display()
            )
        });

        let mut tuples = reader
            .map(|entry| {
                let dir_entry = entry.unwrap();

                if !dir_entry.file_type().unwrap().is_file() {
                    panic!("Migration dir entry: {dir_entry:#?} is not a file!");
                }

                let path = dir_entry.path();

                let file_name = path.file_name().unwrap().to_str().unwrap();

                match pattern.captures(file_name) {
                    None => panic!("Migration file: {file_name} does not match file pattern"),
                    Some(captures) => PendingMigration {
                        index: <Index as std::str::FromStr>::from_str(&captures["index"]).unwrap(),
                        name: <MigrationName as std::str::FromStr>::from_str(&captures["name"])
                            .unwrap(),
                        raw_sql: std::fs::read_to_string(path).unwrap().into(),
                    },
                }
            })
            .collect::<Vec<_>>();

        tuples.sort_by_key(|pending_migration| pending_migration.index);

        let mut defined_migrations = DefinedMigrations::new();

        for pending_migration in tuples {
            defined_migrations.add(pending_migration);
        }

        defined_migrations
    }

    /// Return next index
    ///
    /// This method only returns an index if there is already a defined migration
    /// Else this method returns None.
    pub fn next_index(&self) -> Option<Index> {
        self.0.last_key_value().map(|(index, _)| index.succ())
    }

    /// Return new defined migrations
    ///
    /// ### Examples
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let defined = DefinedMigrations::new();
    ///
    /// assert_eq!(
    ///     defined.select_pending(None),
    ///     (vec![] as Vec<&PendingMigration>)
    /// )
    /// ```
    pub fn new() -> Self {
        DefinedMigrations(std::collections::BTreeMap::new())
    }

    /// Add new defined migration
    ///
    /// Its required migration indexes are added consective!
    ///
    /// ### Examples
    ///
    /// Add intial migrations from initial
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 0.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 0".into(),
    /// });
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    /// ```
    ///
    /// Add intial migrations after initial was long deleted
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    /// defined.add(PendingMigration {
    ///     index: 2.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 2".into(),
    /// });
    /// ```
    ///
    /// Add non consecutive
    ///
    /// ```should_panic
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    /// defined.add(PendingMigration {
    ///     index: 3.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 3".into(),
    /// });
    /// ```
    ///
    /// Add duplicate index
    ///
    /// ```should_panic
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT other".into(),
    /// });
    /// ```
    pub fn add(&mut self, pending_migration: PendingMigration) {
        if let Some(expected) = self.next_index() {
            if pending_migration.index != expected {
                panic!(
                    "Expected next defined migration index: {expected} got: {}",
                    pending_migration.index
                );
            }
        }

        self.0.insert(pending_migration.index, pending_migration);
    }

    /// Select pending migrations
    ///
    /// ### Examples
    ///
    /// No migrations defined, no migration applied.
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let empty = DefinedMigrations::new();
    ///
    /// assert_eq!(
    ///     empty.select_pending(None),
    ///     (vec![] as Vec<&PendingMigration>)
    /// )
    /// ```
    ///
    /// No migrations defined, some migration already applied
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let empty = DefinedMigrations::new();
    ///
    /// assert_eq!(
    ///     empty.select_pending(Some(0.into())),
    ///     (vec![] as Vec<&PendingMigration>)
    /// );
    /// ```
    ///
    /// Migration defined, that is already applied
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 0.into(),
    ///     name: migration_name!("exmaple"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    ///
    /// assert_eq!(
    ///     defined.select_pending(Some(0.into())),
    ///     (vec![] as Vec<&PendingMigration>)
    /// );
    /// ```
    ///
    /// Migration defined, that is not applied
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 0.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    ///
    /// assert_eq!(
    ///     defined.select_pending(None),
    ///     vec![&PendingMigration {
    ///         index: 0.into(),
    ///         name: migration_name!("example"),
    ///         raw_sql: "SELECT 1".into()
    ///     }]
    /// );
    /// ```
    ///
    /// Migration defined, that is initial migration at wrong index
    ///
    /// ```should_panic
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    ///
    /// assert_eq!(
    ///     defined.select_pending(None),
    ///     (vec![] as Vec<&PendingMigration>) // expected value is irrelevant
    /// );
    /// ```
    ///
    /// Migration defined, that is not applied but not successor of applied
    ///
    /// ```should_panic
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 2.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    ///
    /// assert_eq!(
    ///     defined.select_pending(Some(0.into())),
    ///     (vec![] as Vec<&PendingMigration>) // expected value is irrelevant
    /// );
    /// ```
    ///
    /// Migrations defined, partially applied
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let mut defined = DefinedMigrations::new();
    ///
    /// defined.add(PendingMigration {
    ///     index: 1.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 1".into(),
    /// });
    /// defined.add(PendingMigration {
    ///     index: 2.into(),
    ///     name: migration_name!("example"),
    ///     raw_sql: "SELECT 2".into(),
    /// });
    ///
    /// assert_eq!(
    ///     defined.select_pending(Some(1.into())),
    ///     vec![&PendingMigration {
    ///         index: 2.into(),
    ///         name: migration_name!("example"),
    ///         raw_sql: "SELECT 2".into()
    ///     }]
    /// );
    /// ```
    pub fn select_pending(&self, last: Option<Index>) -> Vec<&PendingMigration> {
        match last {
            None => self.select_initial(),
            Some(index) => self.select_from(index),
        }
    }

    fn select_from(&self, last: Index) -> Vec<&PendingMigration> {
        let pending: Vec<_> = self
            .0
            .iter()
            .filter_map(|(index, pending_migration)| {
                if *index > last {
                    Some(pending_migration)
                } else {
                    None
                }
            })
            .collect();

        if let Some(migration) = pending.first() {
            if !migration.index.is_succ_of(last) {
                panic!(
                    "Last migration {last} needs to be followed by {}, got: {}!",
                    last.succ(),
                    migration.index
                )
            }
        }

        pending
    }

    fn select_initial(&self) -> Vec<&PendingMigration> {
        let pending: Vec<_> = self.0.values().collect();

        if let Some(migration) = pending.first() {
            if !migration.index.is_initial() {
                panic!(
                    "Initial migration needs to be indexed at 0, got: {}!",
                    migration.index
                );
            }
        }

        pending
    }
}
