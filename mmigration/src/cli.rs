use super::*;

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run<D: DumpSchema>(&self, context: Context<'_, D>) {
        self.command.run(context).await
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    ApplyPending,
    ListPending,
}

impl Command {
    pub async fn run<D: DumpSchema>(&self, context: Context<'_, D>) {
        match self {
            Self::ApplyPending => context.apply_pending().await,
            Self::ListPending => list_pending(context).await,
        }
    }
}

async fn list_pending<D: DumpSchema>(context: Context<'_, D>) {
    for pending_migration in context.find_pending_migrations().await {
        println!("{}", pending_migration.index.to_u32());
    }
}
