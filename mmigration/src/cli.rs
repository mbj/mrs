use super::*;

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        self.command.run(context).await
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    ApplyPending {
        /// Apply migrations without writing the schema file afterwards.
        #[arg(long)]
        no_schema_dump: bool,
    },
    Bootstrap,
    DumpSchema,
    ListPending,
    NewPending {
        #[arg(long)]
        name: MigrationName,
    },
}

impl Command {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            // Apply, then refresh the schema file by default; `--no-schema-dump` skips
            // the refresh.
            Self::ApplyPending { no_schema_dump } => {
                context.apply_pending().await?;
                if !no_schema_dump {
                    context.dump_schema().await?;
                }
                Ok(())
            }
            Self::Bootstrap => context.bootstrap().await,
            Self::DumpSchema => context.dump_schema().await,
            Self::ListPending => list_pending(context).await,
            Self::NewPending { name } => context.create_new_pending(name).await,
        }
    }
}

async fn list_pending<D: SchemaSource>(context: Context<'_, D>) -> Result<(), ContextError> {
    for pending_migration in context.find_pending_migrations().await? {
        println!("{}", pending_migration.index);
    }

    Ok(())
}
