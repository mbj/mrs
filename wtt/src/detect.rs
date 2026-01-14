use crate::{Config, RepoName};
use std::path::Path;

#[must_use]
pub fn detect_repo_from_cwd(config: &Config) -> Option<RepoName> {
    let cwd = std::env::current_dir().ok()?;
    detect_repo_from_path(config, &cwd)
}

fn detect_repo_from_path(config: &Config, path: &Path) -> Option<RepoName> {
    let worktree_dir = &config.worktree_dir;

    let relative = path.strip_prefix(worktree_dir).ok()?;

    let repo_name = relative.components().next()?;

    let repo_name_str = repo_name.as_os_str().to_str()?;

    let repo_name: RepoName = repo_name_str.parse().ok()?;

    let bare_path = config.bare_repo_path(&repo_name);
    if bare_path.exists() {
        Some(repo_name)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_repo_from_path() {
        let config = Config {
            bare_clone_dir: PathBuf::from("/tmp/test-bare"),
            worktree_dir: PathBuf::from("/tmp/test-worktrees"),
        };

        let path = PathBuf::from("/tmp/test-worktrees/my-repo/main/src");
        let result = detect_repo_from_path(&config, &path);

        assert!(result.is_none());
    }
}
