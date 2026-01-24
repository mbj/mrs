use crate::{
    Base, Branch, Config, Error, RepoName, detect_repo_from_cwd,
};

#[derive(Debug, clap::Parser)]
pub struct Add {
    /// Branch name for the new worktree
    branch: Branch,

    /// Base ref for new branches [default: remote default branch]
    #[clap(long)]
    base: Option<Base>,

    /// Repository name [default: auto-detected from current directory]
    #[clap(long)]
    repo: Option<RepoName>,
}

impl Add {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(repo) => repo,
            None => detect_repo_from_cwd(config)?,
        };

        let bare_clone = config.bare_clone(&repo)?;
        let bare_path = bare_clone.path();

        let worktree_path = config.worktree_path(&repo, &self.branch);

        if worktree_path.exists() {
            return Err(Error::WorktreeExists(worktree_path));
        }

        bare_clone.fetch()?;

        if bare_clone.branch_exists(&self.branch)? {
            log::info!(
                "Creating worktree for existing branch '{}' at {}",
                self.branch,
                worktree_path.display()
            );

            git_proc::worktree::add(&worktree_path)
                .repo_path(bare_path)
                .branch(self.branch.as_str())
                .status()?;
        } else {
            let base = match self.base {
                Some(base) => base,
                None => bare_clone.remote_default_branch()?,
            };

            log::info!(
                "Creating worktree for new branch '{}' from '{}' at {}",
                self.branch,
                base,
                worktree_path.display()
            );

            git_proc::worktree::add(&worktree_path)
                .repo_path(bare_path)
                .new_branch(self.branch.as_str())
                .commit_ish(base.as_str())
                .status()?;
        }

        config.worktree(&repo, &self.branch)?.set_upstream()?;

        log::info!("Worktree created at {}", worktree_path.display());

        Ok(())
    }
}

