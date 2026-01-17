use crate::github::{
    GetPullRequest, ListPullRequestCommits, ListPullRequests, PullRequestNumber, Repository,
};
use futures_util::StreamExt;
use tower_service::Service;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// List open pull requests
    List,
    /// Get a specific pull request
    Get {
        /// Pull request number
        #[clap(long)]
        number: PullRequestNumber,
    },
    /// List commits on a pull request
    Commits {
        /// Pull request number
        #[clap(long)]
        number: PullRequestNumber,
    },
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
        repository: &Repository,
    ) -> Result<()> {
        match self {
            Self::List => {
                let request = ListPullRequests {
                    repository: repository.clone(),
                };
                let mut stream = std::pin::pin!(mhttp::link::paginate(client, request));

                while let Some(result) = stream.next().await {
                    let pull_requests = result?;
                    for pr in pull_requests {
                        println!("#{} {} ({:?})", pr.number, pr.title, pr.state);
                    }
                }

                Ok(())
            }
            Self::Get { number } => {
                let request = GetPullRequest {
                    repository: repository.clone(),
                    pull_request: *number,
                };
                let response = client.call(request).await?;
                println!("{response:#?}");
                Ok(())
            }
            Self::Commits { number } => {
                let request = ListPullRequestCommits {
                    repository: repository.clone(),
                    pull_request: *number,
                };
                let mut stream = std::pin::pin!(mhttp::link::paginate(client, request));

                while let Some(result) = stream.next().await {
                    let commits = result?;
                    for commit in commits {
                        println!(
                            "{} {}",
                            commit.sha.abbrev(),
                            commit.commit.message.lines().next().unwrap_or("")
                        );
                    }
                }

                Ok(())
            }
        }
    }
}
