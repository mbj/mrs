use crate::{Config, Error, GitUrl, ORIGIN, RepoName};

#[derive(Debug, clap::Parser)]
pub struct Setup {
    /// Local name for the repository
    repo: RepoName,

    /// Git remote URL to clone
    url: GitUrl,
}

impl Setup {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let bare_path = config.bare_repo_path(&self.repo);
        let worktree_base = config.worktree_base_path(&self.repo);

        if bare_path.exists() {
            return Err(Error::RepoAlreadyExists(self.repo));
        }

        log::info!("Cloning bare repository to {}", bare_path.display());

        git_proc::clone::new(&self.url)
            .bare()
            .directory(&bare_path)
            .status()?;

        log::info!("Configuring remote tracking branches");

        git_proc::config::new("remote.origin.fetch")
            .repo_path(&bare_path)
            .value("+refs/heads/*:refs/remotes/origin/*")
            .status()?;

        git_proc::fetch::new()
            .repo_path(&bare_path)
            .remote(&ORIGIN)
            .status()?;

        log::info!("Creating worktree directory {}", worktree_base.display());

        std::fs::create_dir_all(&worktree_base)?;

        log::info!("Setup complete for repository '{}'", self.repo);

        Ok(())
    }
}
