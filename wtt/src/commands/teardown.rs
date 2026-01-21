use std::path::PathBuf;

use crate::{Config, Error, RepoName, git};

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
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let bare_path = config.bare_repo_path(&self.repo);

        if !bare_path.exists() {
            return Err(Error::RepoNotFound(self.repo));
        }

        let output = git_proc::worktree::list()
            .repo_path(&bare_path)
            .stdout()
            .string()?;

        let worktree_paths: Vec<PathBuf> = git::parse_worktree_list(&output)
            .into_iter()
            .filter_map(|line| line.split_whitespace().next())
            .map(PathBuf::from)
            .collect();

        for worktree_path in worktree_paths {
            remove_worktree(&bare_path, &worktree_path, self.force)?;
        }

        log::info!("Removing bare repository at {}", bare_path.display());
        std::fs::remove_dir_all(&bare_path)?;

        let worktree_base = config.worktree_base_path(&self.repo);

        if worktree_base.exists() {
            log::info!("Removing worktree directory at {}", worktree_base.display());
            std::fs::remove_dir_all(&worktree_base)?;
        }

        log::info!("Teardown complete for repository '{}'", self.repo);

        Ok(())
    }
}
