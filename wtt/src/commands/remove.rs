use crate::{Branch, Config, Error, RepoName, detect_repo_from_cwd};

#[derive(Debug, clap::Parser)]
pub struct Remove {
    /// Branch name of the worktree to remove
    branch: Branch,

    /// Repository name [default: auto-detected from current directory]
    #[clap(long)]
    repo: Option<RepoName>,

    #[clap(long)]
    force: bool,
}

impl Remove {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(repo) => repo,
            None => detect_repo_from_cwd(config)?,
        };

        config.worktree(&repo, &self.branch)?.remove(self.force)?;

        Ok(())
    }
}
