use crate::{BareCloneNotFound, Config, RepoName};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum DetectError {
    #[error("Failed to get current working directory: {0}")]
    CurrentDir(std::io::Error),

    #[error("Current directory '{}' is not under worktree directory '{}'", .path.display(), .worktree_dir.display())]
    NotInWorktreeDir {
        path: PathBuf,
        worktree_dir: PathBuf,
    },

    #[error("{0}")]
    BareCloneNotFound(#[from] BareCloneNotFound),
}

pub fn detect_repo_from_cwd(config: &Config) -> Result<RepoName, DetectError> {
    let cwd = std::env::current_dir().map_err(DetectError::CurrentDir)?;
    detect_repo_from_path(config, &cwd)
}

fn detect_repo_from_path(config: &Config, path: &Path) -> Result<RepoName, DetectError> {
    let repo_name =
        extract_repo_name(config, path).ok_or_else(|| DetectError::NotInWorktreeDir {
            path: path.to_path_buf(),
            worktree_dir: config.worktree_dir.clone(),
        })?;

    config.bare_clone(&repo_name)?;

    Ok(repo_name)
}

fn extract_repo_name(config: &Config, path: &Path) -> Option<RepoName> {
    let relative = path.strip_prefix(&config.worktree_dir).ok()?;
    let repo_component = relative.components().next()?;
    let repo_str = repo_component.as_os_str().to_str()?;
    repo_str.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_config() -> Config {
        Config {
            bare_clone_dir: PathBuf::from("/tmp/test-bare"),
            worktree_dir: PathBuf::from("/tmp/test-worktrees"),
        }
    }

    #[test]
    fn test_extract_repo_from_simple_branch() {
        let config = test_config();
        let path = PathBuf::from("/tmp/test-worktrees/my-repo/main/src");

        let result = extract_repo_name(&config, &path);

        assert_eq!(
            result.map(|repo| repo.as_str().to_string()),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_extract_repo_from_nested_branch() {
        let config = test_config();
        let path = PathBuf::from("/tmp/test-worktrees/my-repo/feature/login/src/lib.rs");

        let result = extract_repo_name(&config, &path);

        assert_eq!(
            result.map(|repo| repo.as_str().to_string()),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_extract_repo_from_deeply_nested_branch() {
        let config = test_config();
        let path = PathBuf::from(
            "/tmp/test-worktrees/my-repo/feature/deeply/nested/branch/src/module/file.rs",
        );

        let result = extract_repo_name(&config, &path);

        assert_eq!(
            result.map(|repo| repo.as_str().to_string()),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_no_match_outside_worktree_dir() {
        let config = test_config();
        let path = PathBuf::from("/home/user/other/project/src");

        let result = extract_repo_name(&config, &path);

        assert_eq!(result, None);
    }
}
