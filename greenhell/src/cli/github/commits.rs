use crate::github::{CompareCommits, Ref, Repository};
use futures_util::StreamExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// Compare two commits
    Compare {
        /// Base ref
        #[clap(long)]
        base: Ref,
        /// Head ref
        #[clap(long)]
        head: Ref,
    },
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
        repository: &Repository,
    ) -> Result<()> {
        match self {
            Self::Compare { base, head } => {
                let request = CompareCommits {
                    repository: repository.clone(),
                    base: base.clone(),
                    head: head.clone(),
                };

                let mut stream = std::pin::pin!(typed_reqwest::link::paginate(client, request));
                let mut first_page = true;

                while let Some(result) = stream.next().await {
                    let comparison = result?;

                    if first_page {
                        println!("Status: {:?}", comparison.status);
                        println!("Ahead by: {}", comparison.ahead_by);
                        println!("Behind by: {}", comparison.behind_by);
                        println!("Commits:");
                        first_page = false;
                    }

                    for commit in comparison.commits {
                        println!("  {}", commit.sha);
                    }
                }

                Ok(())
            }
        }
    }
}
