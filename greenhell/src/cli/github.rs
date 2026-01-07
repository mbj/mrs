pub mod check_runs;
pub mod commits;
pub mod pull_requests;
pub mod repository;

use crate::github::Repository;

/// Direct GitHub API operations
#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// Repository operations
    Repository {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,
        #[clap(subcommand)]
        command: repository::Command,
    },
    /// Pull request operations
    #[command(name = "pull-requests")]
    PullRequests {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,
        #[clap(subcommand)]
        command: pull_requests::Command,
    },
    /// Check runs operations
    #[command(name = "check-runs")]
    CheckRuns {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,
        #[clap(subcommand)]
        command: check_runs::Command,
    },
    /// Commits operations
    Commits {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,
        #[clap(subcommand)]
        command: commits::Command,
    },
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::Repository {
                repository,
                command,
            } => command.run(client, repository).await,
            Self::PullRequests {
                repository,
                command,
            } => command.run(client, repository).await,
            Self::CheckRuns {
                repository,
                command,
            } => command.run(client, repository).await,
            Self::Commits {
                repository,
                command,
            } => command.run(client, repository).await,
        }
    }
}
