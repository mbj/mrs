use std::path::PathBuf;

use crate::Branch;

#[derive(Clone, Debug)]
pub struct Config {
    pub bare_clone_dir: PathBuf,
    pub worktree_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        Self {
            bare_clone_dir: PathBuf::from(&home).join(".wtt/bare"),
            worktree_dir: PathBuf::from(&home).join("devel"),
        }
    }
}

impl Config {
    #[must_use]
    pub fn bare_repo_path(&self, repo: &crate::RepoName) -> PathBuf {
        self.bare_clone_dir.join(format!("{}.git", repo.as_str()))
    }

    #[must_use]
    pub fn worktree_base_path(&self, repo: &crate::RepoName) -> PathBuf {
        self.worktree_dir.join(repo.as_str())
    }

    #[must_use]
    pub fn worktree_path(&self, repo: &crate::RepoName, branch: &Branch) -> PathBuf {
        self.worktree_base_path(repo).join(branch.as_str())
    }
}
