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

        let bare_path = config.bare_repo_path(&repo);
        let worktree_base = config.worktree_base_path(&repo);

        if bare_path.exists() {
            return Err(Error::RepoAlreadyExists(repo));
        }

        log::info!("Cloning bare repository to {}", bare_path.display());

        git_proc::clone::new(&self.address)
            .bare()
            .directory(&bare_path)
            .status()
            .await?;

        log::info!("Configuring remote tracking branches");

        git_proc::config::new("remote.origin.fetch")
            .repo_path(&bare_path)
            .value("+refs/heads/*:refs/remotes/origin/*")
            .status()
            .await?;

        git_proc::fetch::new()
            .repo_path(&bare_path)
            .remote(&ORIGIN)
            .status()
            .await?;

        log::info!("Creating worktree directory {}", worktree_base.display());

        std::fs::create_dir_all(&worktree_base)?;

        log::info!("Setup complete for repository '{repo}'");

        Ok(())
    }
}
