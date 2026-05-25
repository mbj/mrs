use super::*;

/// Operations on the migration files in a checkout.
///
/// These need no database connection.
#[derive(Clone, Debug, clap::Subcommand)]
pub enum Command {
    /// Create a new migration file at the next index
    New {
        /// Migration name, used in the `{index}_{name}.sql` file name
        name: MigrationName,
    },
}

impl Command {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            Self::New { name } => context.create_new_pending(name).await,
        }
    }
}
