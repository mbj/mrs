use super::*;

/// Operations on the database reached through the configured connection.
///
/// Every command here reads or mutates the database and nothing else: none of
/// them write into a checkout, so this set is safe to mount in a deployed binary.
#[derive(Clone, Debug, clap::Subcommand)]
pub enum Command {
    /// Apply every pending migration
    Apply,
    /// Create the migration tracking table, failing if it already exists
    Bootstrap,
    /// Write the current schema to stdout
    DumpSchema,
    /// List the migrations that are not yet applied
    Pending,
    /// Verify the recorded migration state against the migration files
    Verify,
}

impl Command {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            Self::Apply => {
                context.apply_pending().await?;
                Ok(())
            }
            Self::Bootstrap => context.bootstrap().await,
            // Deliberately stdout rather than the configured schema path: a deployed
            // binary must never write over the schema file it shipped with.
            Self::DumpSchema => {
                print!(
                    "{}",
                    <Schema as AsRef<str>>::as_ref(&context.read_schema().await)
                );
                Ok(())
            }
            Self::Pending => {
                for pending_migration in context.find_pending_migrations().await? {
                    println!("{}", pending_migration.index);
                }
                Ok(())
            }
            Self::Verify => context.verify().await,
        }
    }
}

/// [`Command`], plus the database operations that maintain a checkout.
#[derive(Clone, Debug, clap::Subcommand)]
pub enum DevelopCommand {
    #[command(flatten)]
    Operate(Command),
    /// Apply every pending migration, then refresh the schema file
    Sync,
}

impl DevelopCommand {
    pub async fn run<D: SchemaSource>(&self, context: Context<'_, D>) -> Result<(), ContextError> {
        match self {
            Self::Operate(command) => command.run(context).await,
            // The only writer of the configured schema path.
            Self::Sync => {
                context.apply_pending().await?;
                context.write_schema().await
            }
        }
    }
}
