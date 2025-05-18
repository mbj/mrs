pub mod cli;
pub mod defined_migrations;
pub mod transaction;
pub mod types;

pub use defined_migrations::*;
pub use types::*;

use transaction::Transaction;

#[derive(Debug)]
pub struct Config {
    pub migration_dir: std::path::PathBuf,
    pub schema_file: file_buf::FileBuf,
}

pub trait DumpSchema {
    fn dump_schema(&self) -> impl std::future::Future<Output = SchemaDump> + Send;
}

pub struct Context<'a, D: DumpSchema> {
    client_config: &'a pg_client::Config,
    config: &'a Config,
    defined_migrations: DefinedMigrations,
    dump_schema: D,
}

impl<'a, D: DumpSchema> Context<'a, D> {
    pub fn load(config: &'a Config, client_config: &'a pg_client::Config, dump_schema: D) -> Self {
        let defined_migrations = DefinedMigrations::load(&config.migration_dir);

        Context {
            config,
            client_config,
            defined_migrations,
            dump_schema,
        }
    }

    pub async fn apply_pending_no_schema_dump(&self) {
        self.with_transaction(async |transaction| {
            for pending_migration in self.find_pending_migrations_transaction(transaction).await {
                transaction.apply_pending_migration(pending_migration).await
            }
        })
        .await;
    }

    pub async fn apply_pending(&self) {
        self.apply_pending_no_schema_dump().await;

        self.dump_schema().await
    }

    pub async fn create_new_pending(&self, name: &MigrationName) {
        let next_index = self
            .with_transaction(async |transaction| self.next_index(transaction).await)
            .await;

        let next_path = self
            .config
            .migration_dir
            .join(format!("{next_index}_{name}.sql"));

        log::info!("Creating new migration: {}", next_path.display());

        std::fs::write(next_path, format!("-- Migration {next_index} {name}")).unwrap();
    }

    pub async fn dump_schema(&self) {
        log::info!("Writing schema to: {}", self.config.schema_file.display());

        std::fs::write(
            &self.config.schema_file,
            self.dump_schema.dump_schema().await,
        )
        .unwrap()
    }

    pub async fn find_pending_migrations(&self) -> Vec<&PendingMigration> {
        self.with_transaction(async |transaction| {
            self.find_pending_migrations_transaction(transaction).await
        })
        .await
    }

    async fn with_transaction<T, F: AsyncFnMut(&mut Transaction) -> T>(&self, mut action: F) -> T {
        Transaction::with_transaction(self.client_config, async |transaction| {
            action(transaction).await
        })
        .await
    }

    async fn next_index(&self, transaction: &mut Transaction<'_>) -> Index {
        match self.defined_migrations.next_index() {
            Some(next_index) => next_index,
            None => transaction
                .find_last_applied_index()
                .await
                .unwrap_or(Index::initial()),
        }
    }

    async fn find_pending_migrations_transaction(
        &self,
        transaction: &mut Transaction<'_>,
    ) -> Vec<&PendingMigration> {
        self.defined_migrations
            .select_pending(transaction.find_last_applied_index().await)
    }
}
