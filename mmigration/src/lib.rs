pub mod cli;

#[derive(Debug)]
pub struct Config {
    pub migration_dir: std::path::PathBuf,
    pub schema_file: file_buf::FileBuf,
}

#[derive(Debug)]
pub struct PendingMigration {
    index: Index,
    sql: std::rc::Rc<RawSql>,
}

impl PendingMigration {
    pub fn digest(&self) -> [u8; 32] {
        <sha2::Sha256 as sha2::Digest>::digest(std::rc::Rc::<RawSql>::as_ref(&self.sql)).into()
    }
}

pub struct SchemaDump(String);

impl AsRef<[u8]> for SchemaDump {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<String> for SchemaDump {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Index(u32);

impl Index {
    /// Return successor of index
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.succ(), b);
    /// ```
    pub fn succ(&self) -> Index {
        Self(self.0.checked_add(1).unwrap())
    }

    /// Test if index is initial one
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.is_initial(), true);
    /// assert_eq!(b.is_initial(), false);
    /// ```
    pub fn is_initial(&self) -> bool {
        self.0 == 0
    }

    /// Test if other is successor of self
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    /// let c: Index = 2_u32.into();
    ///
    /// assert_eq!(a.is_succ_of(a), false);
    /// assert_eq!(a.is_succ_of(b), false);
    /// assert_eq!(b.is_succ_of(a), true);
    /// assert_eq!(c.is_succ_of(a), false);
    /// assert_eq!(c.is_succ_of(b), true);
    /// ```
    pub fn is_succ_of(&self, other: Self) -> bool {
        *self == other.succ()
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for Index {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Index, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <i64 as sqlx::Decode<sqlx::Postgres>>::decode(value).and_then(|value| {
            match value.try_into() {
                Ok(valid) => Ok(Self(valid)),
                Err(_) => Err("out of u32 range".into()),
            }
        })
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for Index {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <i64 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0.into(), buf)
    }
}

impl sqlx::Type<sqlx::Postgres> for Index {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::postgres::PgHasArrayType for Index {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}

#[derive(Clone, Debug)]
struct RawSql(String);

impl AsRef<[u8]> for RawSql {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for RawSql {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for RawSql {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct DefinedMigrations(std::collections::BTreeMap<Index, std::rc::Rc<RawSql>>);

impl DefinedMigrations {
    fn new() -> Self {
        DefinedMigrations(std::collections::BTreeMap::new())
    }

    fn add(&mut self, index: Index, raw_sql: RawSql) {
        if self.0.insert(index, raw_sql.into()).is_some() {
            panic!("Migration index: #{index:#?} is duplicated");
        }
    }

    fn select_pending(&self, last: Option<Index>) -> Vec<PendingMigration> {
        match last {
            None => self.select_initial(),
            Some(index) => self.select_from(index),
        }
    }

    fn select_from(&self, last: Index) -> Vec<PendingMigration> {
        let pending: Vec<_> = self
            .0
            .iter()
            .filter(|(index, _)| **index > last)
            .map(|(index, sql)| PendingMigration {
                index: *index,
                sql: sql.clone(),
            })
            .collect();

        if let Some(migration) = pending.first() {
            if !migration.index.is_succ_of(last) {
                panic!(
                    "Last migration {} needs to be followed by {}, got: {}!",
                    last.to_u32(),
                    last.succ().to_u32(),
                    migration.index.to_u32()
                )
            }
        }

        pending
    }

    fn select_initial(&self) -> Vec<PendingMigration> {
        let pending: Vec<_> = self
            .0
            .iter()
            .map(|(index, sql)| PendingMigration {
                index: *index,
                sql: sql.clone(),
            })
            .collect();

        if let Some(migration) = pending.first() {
            if !migration.index.is_initial() {
                panic!(
                    "Initial migration needs to be indexed at 0, got: {}!",
                    migration.index.to_u32()
                );
            }
        }

        pending
    }
}

pub trait DumpSchema {
    fn dump_schema(&self) -> impl std::future::Future<Output = SchemaDump> + Send;
}

pub struct Context<'a, D: DumpSchema> {
    pub client_config: &'a pg_client::Config,
    pub dump_schema: D,
    pub config: &'a Config,
}

impl<D: DumpSchema> Context<'_, D> {
    pub async fn apply_pending_no_schema_dump(&self) {
        self.with_inner(async |mut inner| inner.apply_pending().await)
            .await;
    }

    pub async fn apply_pending(&self) {
        self.apply_pending_no_schema_dump().await;

        std::fs::write(
            &self.config.schema_file,
            self.dump_schema.dump_schema().await,
        )
        .unwrap()
    }

    pub async fn find_pending_migrations(&self) -> Vec<PendingMigration> {
        self.with_inner(async |mut inner| inner.find_pending_migrations().await)
            .await
    }

    async fn with_inner<T, F: AsyncFnMut(Inner) -> T>(&self, mut action: F) -> T {
        self.client_config
            .with_sqlx_connection(async |connection| {
                action(Inner {
                    config: self.config,
                    connection,
                })
                .await
            })
            .await
    }
}

struct Inner<'a> {
    pub config: &'a Config,
    pub connection: &'a mut sqlx::postgres::PgConnection,
}

impl Inner<'_> {
    fn load_defined_migrations(&self) -> DefinedMigrations {
        let pattern = regex_lite::Regex::new(r#"\A(?<index>\d+)_[^\.]+\.sql\z"#).unwrap();

        let mut migrations = DefinedMigrations::new();

        let reader = std::fs::read_dir(&self.config.migration_dir).unwrap_or_else(|error| {
            panic!(
                "Migration dir: {} not readable: {error}",
                self.config.migration_dir.display()
            )
        });

        for entry in reader {
            let dir_entry = entry.unwrap();

            if !dir_entry.file_type().unwrap().is_file() {
                panic!("Migration dir entry: {dir_entry:#?} is not a file!");
            }

            let path = dir_entry.path();

            let file_name = path.file_name().unwrap().to_str().unwrap();

            match pattern.captures(file_name) {
                None => panic!("Migration file: {file_name} does not match file pattern"),
                Some(captures) => {
                    let index = <u32 as std::str::FromStr>::from_str(&captures["index"])
                        .unwrap()
                        .into();

                    migrations.add(index, std::fs::read_to_string(path).unwrap().into())
                }
            }
        }

        migrations
    }

    pub async fn apply_pending(&mut self) {
        for migration in self.find_pending_migrations().await {
            self.apply_migration(migration).await;
        }
    }

    async fn apply_migration(&mut self, migration: PendingMigration) {
        self.begin_serializable_transaction().await;
        log::info!("Appying migration: {}", migration.index.to_u32());

        sqlx::query("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *self.connection)
            .await
            .unwrap();

        sqlx::raw_sql(migration.sql.as_ref().as_ref())
            .execute(&mut *self.connection)
            .await
            .unwrap();

        sqlx::query(
            r#"
            INSERT INTO
              public.applied_migrations
              ( index
              , digest
              )
            VALUES
              ( $1
              , $2
              )
        "#,
        )
        .bind(migration.index)
        .bind(migration.digest())
        .execute(&mut *self.connection)
        .await
        .unwrap();

        self.set_applied_migrations_comment(&format!(
            "Last applied migration: {}",
            migration.index.to_u32()
        ))
        .await;

        self.commit_transaction().await;
    }

    async fn create_applied_migrations_table(&mut self) {
        self.begin_serializable_transaction().await;

        let row = sqlx::query(
            r#"
            SELECT
              EXISTS(
                SELECT
                FROM
                  information_schema.tables
                WHERE
                  (table_schema, table_name) = ('public', 'applied_migrations')
             )
        "#,
        )
        .fetch_one(&mut *self.connection)
        .await
        .unwrap();

        let exists: bool = sqlx::Row::get(&row, 0);

        if !exists {
            sqlx::query(
                r#"
                CREATE TABLE
                  public.applied_migrations
                  ( index int8                    PRIMARY KEY
                  , applied_by text               NOT NULL DEFAULT current_role
                  , digest bytea                  NOT NULL CHECK (octet_length(digest) = 32)
                  , elapsed interval              NOT NULL DEFAULT (clock_timestamp() - transaction_timestamp())
                  , transaction_id bigint         NOT NULL DEFAULT txid_current()
                  , transaction_start timestamptz NOT NULL DEFAULT transaction_timestamp()
                  )
                "#,
            )
            .execute(&mut *self.connection)
            .await
            .unwrap();

            self.set_applied_migrations_comment("No applied migrations")
                .await
        }

        self.commit_transaction().await;
    }

    async fn find_pending_migrations(&mut self) -> Vec<PendingMigration> {
        self.create_applied_migrations_table().await;

        let defined_migrations = self.load_defined_migrations();
        let last_applied_index = self.find_last_applied_index().await;

        defined_migrations.select_pending(last_applied_index)
    }

    async fn set_applied_migrations_comment(&mut self, comment: &str) {
        // we use a termporary function to generate the SQL string literal for the comment safely PG
        // server side. PG does not support binds in place the string literal.
        sqlx::raw_sql(
            r#"
            CREATE FUNCTION
              pg_temp.set_applied_migrations_comment(arg_comment text)
            RETURNS
              void
            LANGUAGE
              plpgsql
            AS $$
              BEGIN
                EXECUTE format('COMMENT ON TABLE public.applied_migrations IS %L', arg_comment);
              END;
            $$
        "#,
        )
        .execute(&mut *self.connection)
        .await
        .unwrap();

        sqlx::query("SELECT pg_temp.set_applied_migrations_comment($1)")
            .bind(comment)
            .execute(&mut *self.connection)
            .await
            .unwrap();

        sqlx::raw_sql("DROP FUNCTION pg_temp.set_applied_migrations_comment")
            .execute(&mut *self.connection)
            .await
            .unwrap();
    }

    async fn find_last_applied_index(&mut self) -> Option<Index> {
        sqlx::query(r#"SELECT index FROM public.applied_migrations ORDER BY index DESC LIMIT 1"#)
            .fetch_optional(&mut *self.connection)
            .await
            .unwrap()
            .as_ref()
            .map(|row| sqlx::Row::get(row, 0))
    }

    async fn begin_serializable_transaction(&mut self) {
        sqlx::query("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *self.connection)
            .await
            .unwrap();
    }

    async fn commit_transaction(&mut self) {
        sqlx::query("COMMIT")
            .execute(&mut *self.connection)
            .await
            .unwrap();
    }
}
