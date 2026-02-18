use std::path::PathBuf;

use crate::{Config, Error, RepoName, git};
use git_proc::Build;

use super::remove::remove_worktree;

#[derive(Debug, clap::Parser)]
pub struct Teardown {
    /// Repository name to remove completely
    repo: RepoName,

    /// Force removal of worktrees with uncommitted changes
    #[clap(long)]
    force: bool,
}

impl Teardown {
    pub async fn run(self, config: &Config) -> Result<(), Error> {
        let repo_path = config.repo_path(&self.repo);

        if !repo_path.exists() {
            return Err(Error::RepoNotFound(self.repo));
        }

        let output = git_proc::worktree::list()
            .repo_path(&repo_path)
            .build()
            .stdout_capture()
            .string()
            .await?;

        let worktree_paths: Vec<PathBuf> = git::parse_worktree_list(&output)
            .into_iter()
            .filter_map(|line| line.split_whitespace().next())
            .map(PathBuf::from)
            .collect();

        for worktree_path in worktree_paths {
            remove_worktree(&repo_path, &worktree_path, self.force).await?;
        }

        log::info!("Removing repository at {}", repo_path.display());
        std::fs::remove_dir_all(&repo_path)?;

        log::info!("Teardown complete for repository '{}'", self.repo);

        Ok(())
    }
}
