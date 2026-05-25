use super::*;

/// The command surface for developer tooling.
///
/// Everything [`operate`](crate::operate) offers, plus the operations that read and
/// write the checkout.
#[derive(Clone, Debug, clap::Subcommand)]
pub enum Command {
    Database {
        #[command(subcommand)]
        command: crate::database::DevelopCommand,
    },
    Repository {
        #[command(subcommand)]
        command: crate::repository::Command,
    },
}

impl Command {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            Self::Database { command } => command.run(context).await,
            Self::Repository { command } => command.run(context).await,
        }
    }
}
