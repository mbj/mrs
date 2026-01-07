use crate::github::{ListRepositoryEvents, Repository};
use tower_service::Service;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// List repository events
    List,
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
        repository: &Repository,
    ) -> Result<()> {
        match self {
            Self::List => {
                let request = ListRepositoryEvents {
                    repository: repository.clone(),
                    etag: None,
                };
                let response = client.call(request).await?;
                println!("{response:#?}");
                Ok(())
            }
        }
    }
}
