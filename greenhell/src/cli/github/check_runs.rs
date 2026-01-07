use crate::github::{ListCheckRuns, Ref, Repository};
use futures_util::StreamExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// List check runs for a ref
    List {
        /// Git ref (branch, tag, or SHA)
        #[clap(long)]
        git_ref: Ref,
    },
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
        repository: &Repository,
    ) -> Result<()> {
        match self {
            Self::List { git_ref } => {
                let request = ListCheckRuns {
                    repository: repository.clone(),
                    git_ref: git_ref.clone(),
                };
                let mut stream = std::pin::pin!(mhttp::link::paginate(client, request));

                while let Some(result) = stream.next().await {
                    let check_run_list = result?;
                    println!("Total: {}", check_run_list.total_count);
                    for check_run in check_run_list.check_runs {
                        let conclusion = check_run
                            .conclusion
                            .map(|c| format!("{c:?}"))
                            .unwrap_or_else(|| "-".to_string());
                        println!("  {} {:?} {}", check_run.name, check_run.status, conclusion);
                    }
                }

                Ok(())
            }
        }
    }
}
