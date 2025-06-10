use super::*;

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run<D: SchemaDump>(&self, context: Context<'_, D>) {
        self.command.run(context).await
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    ApplyPending,
    ApplyPendingNoSchemaDump,
    ListPending,
    NewPending {
        #[arg(long)]
        name: MigrationName,
    },
    SchemaDump,
}

impl Command {
    pub async fn run<D: SchemaDump>(&self, context: Context<'_, D>) {
        match self {
            Self::ApplyPending => context.apply_pending().await,
            Self::ApplyPendingNoSchemaDump => context.apply_pending_no_schema_dump().await,
            Self::SchemaDump => context.schema_dump().await,
            Self::ListPending => list_pending(context).await,
            Self::NewPending { name } => context.create_new_pending(name).await,
        }
    }
}

async fn list_pending<D: SchemaDump>(context: Context<'_, D>) {
    for pending_migration in context.find_pending_migrations().await {
        println!("{}", pending_migration.index);
    }
}
