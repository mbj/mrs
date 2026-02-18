use crate::{Config, Error, ORIGIN, RepoName, repository};

#[derive(Debug, clap::Parser)]
pub struct Setup {
    /// Git repository address to clone
    address: repository::Address,

    /// Local name for the repository (defaults to repo name extracted from address)
    #[clap(long)]
    repo: Option<RepoName>,
}

impl Setup {
    pub async fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(name) => name,
            None => RepoName::from_repository_address(&self.address)?,
        };

        let repo_path = config.repo_path(&repo);

        if repo_path.exists() {
            return Err(Error::RepoAlreadyExists(repo));
        }

        log::info!("Cloning bare repository to {}", repo_path.display());

        git_proc::clone::new(&self.address)
            .bare()
            .directory(&repo_path)
            .status()
            .await?;

        log::info!("Configuring remote tracking branches");

        git_proc::config::new("remote.origin.fetch")
            .repo_path(&repo_path)
            .value("+refs/heads/*:refs/remotes/origin/*")
            .status()
            .await?;

        git_proc::fetch::new()
            .repo_path(&repo_path)
            .remote(&ORIGIN)
            .status()
            .await?;

        log::info!("Setup complete for repository '{repo}'");

        Ok(())
    }
}
