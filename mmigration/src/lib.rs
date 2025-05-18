pub mod cli;
pub mod defined_migrations;
pub mod types;

pub use defined_migrations::*;
pub use types::*;

#[derive(Debug)]
pub struct Config {
    pub migration_dir: std::path::PathBuf,
    pub schema_file: file_buf::FileBuf,
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

    pub async fn create_new_pending(&self) {
        let next_index = self
            .with_inner(async |mut inner| inner.next_index().await)
            .await;

        let next_path = self.config.migration_dir.join(format!("{next_index}.sql"));

        log::info!("Creating new migration: {}", next_path.display());

        std::fs::write(next_path, format!("// Migration {next_index}")).unwrap();
    }

    pub async fn dump_schema(&self) {
        log::info!("Writing schema to: {}", self.config.schema_file.display());

        std::fs::write(
            &self.config.schema_file,
            self.dump_schema.dump_schema().await,
        )
        .unwrap()
    }

    pub async fn apply_pending(&self) {
        self.apply_pending_no_schema_dump().await;

        self.dump_schema().await
    }

    pub async fn find_pending_migrations(&self) -> Vec<PendingMigration> {
        self.with_inner(async |mut inner| inner.find_pending_migrations().await)
            .await
    }

    async fn with_inner<T, F: AsyncFnMut(Inner) -> T>(&self, mut action: F) -> T {
        let defined_migrations = DefinedMigrations::load(&self.config.migration_dir);

        self.client_config
            .with_sqlx_connection(async |connection| {
                action(Inner {
                    connection,
                    defined_migrations: &defined_migrations,
                })
                .await
            })
            .await
    }
}

struct Inner<'a> {
    connection: &'a mut sqlx::postgres::PgConnection,
    defined_migrations: &'a DefinedMigrations,
}

impl Inner<'_> {
    async fn next_index(&mut self) -> Index {
        match self.defined_migrations.next_index() {
            Some(next_index) => next_index,
            None => self
                .find_last_applied_index()
                .await
                .unwrap_or(Index::initial()),
        }
    }

    pub async fn apply_pending(&mut self) {
        for migration in self.find_pending_migrations().await {
            self.apply_migration(migration).await;
        }
    }

    async fn apply_migration(&mut self, migration: PendingMigration) {
        self.begin_serializable_transaction().await;
        log::info!("Appying migration: {}", migration.index());

        sqlx::query("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *self.connection)
            .await
            .unwrap();

        sqlx::raw_sql(migration.raw_sql().as_ref())
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
        .bind(migration.index())
        .bind(migration.digest())
        .execute(&mut *self.connection)
        .await
        .unwrap();

        self.set_applied_migrations_comment(&format!(
            "Last applied migration: {}",
            migration.index()
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

        let last_applied_index = self.find_last_applied_index().await;

        self.defined_migrations.select_pending(last_applied_index)
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
