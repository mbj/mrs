pub(crate) mod cache_registry;

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Cache registry end-to-end tests.
    CacheRegistry {
        #[clap(subcommand)]
        command: cache_registry::Command,
    },
}

impl Command {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::CacheRegistry { command } => command.run().await,
        }
    }
}
