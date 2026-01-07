use crate::github::{GetRepository, Repository};
use tower_service::Service;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// Get repository information
    Get,
}

impl Command {
    pub async fn run(
        &self,
        client: &mut crate::github::Client,
        repository: &Repository,
    ) -> Result<()> {
        match self {
            Self::Get => {
                let request = GetRepository {
                    repository: repository.clone(),
                };
                let response = client.call(request).await?;
                println!("{response:#?}");
                Ok(())
            }
        }
    }
}
