use super::*;

/// The command surface for deployed binaries.
///
/// Database operations only: nothing here writes into a checkout, so mounting this
/// set does not hand a deployed binary the machinery for authoring migrations.
#[derive(Clone, Debug, clap::Subcommand)]
pub enum Command {
    Database {
        #[command(subcommand)]
        command: crate::database::Command,
    },
}

impl Command {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            Self::Database { command } => command.run(context).await,
        }
    }
}
