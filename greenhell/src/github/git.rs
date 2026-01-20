use super::Repository;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to run git command: {0}")]
    GitCommand(#[from] cmd_proc::CommandError),
    #[error("Git command failed: {0}")]
    GitCommandFailed(String),
    #[error("Git output is not valid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Remote '{0}' is not a GitHub repository")]
    NotGitHub(String),
}

/// Gets the GitHub repository from the specified git remote.
pub fn get_github_repository(remote_name: &str) -> Result<Repository, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments(["remote", "get-url", remote_name])
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    let remote_url = String::from_utf8(output.stdout)?.trim().to_string();

    parse_github_remote(&remote_url).ok_or(Error::NotGitHub(remote_url))
}

/// Gets the current branch name.
pub fn get_current_branch() -> Result<super::Branch, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    let branch_name = String::from_utf8(output.stdout)?.trim().to_string();

    branch_name
        .parse()
        .map_err(|_| Error::GitCommandFailed(format!("Invalid branch name: {branch_name}")))
}

/// Lists commit SHAs from base (exclusive) to HEAD (inclusive).
///
/// Returns commits in topological order with parents before children.
pub fn list_commits(base: &super::Ref) -> Result<Vec<super::Sha>, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments([
            "rev-list",
            "--topo-order",
            "--reverse",
            &format!("{base}..HEAD"),
        ])
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    let stdout = String::from_utf8(output.stdout)?;

    stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse()
                .map_err(|_| Error::GitCommandFailed(format!("Invalid SHA: {line}")))
        })
        .collect()
}

/// Force pushes a specific commit to a remote branch.
///
/// Uses `git push --force <remote> <sha>:refs/heads/<branch>`.
pub fn force_push_commit(
    remote: &str,
    sha: &super::Sha,
    branch: &super::Branch,
) -> Result<(), Error> {
    let refspec = format!("{sha}:refs/heads/{branch}");

    let output = cmd_proc::Command::new("git")
        .arguments(["push", "--force", remote, &refspec])
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    Ok(())
}

/// Gets the upstream tracking ref for the current branch.
///
/// Uses `@{u}` (shorthand for `@{upstream}`) which resolves to the upstream
/// tracking branch. For example, if on branch `feature` tracking `origin/feature`,
/// this returns `origin/feature`.
///
/// Returns `None` if no upstream is configured for the current branch.
pub fn get_upstream() -> Result<Option<super::Ref>, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        if stderr.contains("no upstream") || stderr.contains("unknown revision") {
            return Ok(None);
        }
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    let upstream = String::from_utf8(output.stdout)?.trim().to_string();

    upstream
        .parse()
        .map(Some)
        .map_err(|_| Error::GitCommandFailed(format!("Invalid ref: {upstream}")))
}

/// Parses a GitHub SSH remote URL.
///
/// Format: `git@github.com:owner/repo.git`
///
/// Note: SSH URLs use SCP-style syntax which is not a valid URL format,
/// so we parse them manually.
fn parse_github_ssh_remote(remote: &str) -> Option<Repository> {
    let (user_host, path) = remote.split_once(':')?;

    if user_host != "git@github.com" {
        return None;
    }

    let repo_path = path.strip_suffix(".git").unwrap_or(path);
    repo_path.parse().ok()
}

/// Parses a GitHub HTTPS remote URL.
///
/// Format: `https://github.com/owner/repo.git`
fn parse_github_https_remote(remote: &str) -> Option<Repository> {
    let url = Url::parse(remote).ok()?;

    if url.scheme() != "https" {
        return None;
    }

    if url.host_str() != Some("github.com") {
        return None;
    }

    if url.query().is_some() || url.fragment().is_some() {
        return None;
    }

    let path = url.path().strip_prefix('/')?;
    let repo_path = path.strip_suffix(".git").unwrap_or(path);
    repo_path.parse().ok()
}

/// Parses a git remote URL and returns the GitHub repository if it's a GitHub remote.
///
/// Supports:
/// - SSH: `git@github.com:owner/repo.git`
/// - HTTPS: `https://github.com/owner/repo.git`
#[must_use]
pub fn parse_github_remote(remote: &str) -> Option<Repository> {
    parse_github_ssh_remote(remote).or_else(|| parse_github_https_remote(remote))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_ssh_remote() {
        let repo = parse_github_ssh_remote("git@github.com:owner/repo.git").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_ssh_remote_no_git_suffix() {
        let repo = parse_github_ssh_remote("git@github.com:owner/repo").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_ssh_remote_not_github() {
        assert!(parse_github_ssh_remote("git@gitlab.com:owner/repo.git").is_none());
    }

    #[test]
    fn test_parse_github_https_remote() {
        let repo = parse_github_https_remote("https://github.com/owner/repo.git").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_https_remote_no_git_suffix() {
        let repo = parse_github_https_remote("https://github.com/owner/repo").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_https_remote_not_github() {
        assert!(parse_github_https_remote("https://gitlab.com/owner/repo.git").is_none());
    }

    #[test]
    fn test_parse_github_https_remote_with_query() {
        assert!(parse_github_https_remote("https://github.com/owner/repo.git?foo=bar").is_none());
    }

    #[test]
    fn test_parse_github_https_remote_with_fragment() {
        assert!(parse_github_https_remote("https://github.com/owner/repo.git#readme").is_none());
    }

    #[test]
    fn test_parse_github_remote_ssh() {
        let repo = parse_github_remote("git@github.com:owner/repo.git").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_remote_https() {
        let repo = parse_github_remote("https://github.com/owner/repo.git").unwrap();
        assert_eq!(repo.owner(), "owner");
        assert_eq!(repo.repo(), "repo");
    }

    #[test]
    fn test_parse_github_remote_http_not_supported() {
        assert!(parse_github_remote("http://github.com/owner/repo.git").is_none());
    }

    #[test]
    fn test_parse_github_remote_not_github() {
        assert!(parse_github_remote("git@gitlab.com:owner/repo.git").is_none());
        assert!(parse_github_remote("https://gitlab.com/owner/repo.git").is_none());
    }

    #[test]
    fn test_parse_github_remote_garbage() {
        assert!(parse_github_remote("not a url at all").is_none());
        assert!(parse_github_remote("").is_none());
    }
}
