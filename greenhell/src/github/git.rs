use super::Repository;
use git_proc::Build;
use git_proc::url::{Remote, RemoteName};
use nom::{IResult, Parser, bytes::complete::take_till, character::complete::char};
use url::Url;

/// A git config key (e.g., `branch.main.remote`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigKey(String);

impl ConfigKey {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A git config value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigValue(String);

impl ConfigValue {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A git config entry (key-value pair).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigEntry {
    key: ConfigKey,
    value: ConfigValue,
}

impl ConfigEntry {
    #[must_use]
    pub fn key(&self) -> &ConfigKey {
        &self.key
    }

    #[must_use]
    pub fn value(&self) -> &ConfigValue {
        &self.value
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, key) = take_till(|c| c == '=').parse(input)?;
        let (remaining, _) = char('=').parse(remaining)?;
        let value = remaining;

        Ok((
            "",
            Self {
                key: ConfigKey(key.to_string()),
                value: ConfigValue(value.to_string()),
            },
        ))
    }
}

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
pub fn get_github_repository(remote_name: &RemoteName) -> Result<Repository, Error> {
    let output = git_proc::remote::get_url(remote_name).build().output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    let remote_url = String::from_utf8(output.stdout)?.trim().to_string();

    parse_github_remote(&remote_url).ok_or(Error::NotGitHub(remote_url))
}

/// Gets the current branch name.
pub fn get_current_branch() -> Result<super::Branch, Error> {
    let output = git_proc::rev_parse::new()
        .abbrev_ref()
        .rev("HEAD")
        .build()
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
    let output = git_proc::rev_list::new()
        .topo_order()
        .reverse()
        .commit(&format!("{base}..HEAD"))
        .build()
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
    remote: &Remote,
    sha: &super::Sha,
    branch: &super::Branch,
) -> Result<(), Error> {
    let refspec = format!("{sha}:refs/heads/{branch}");

    let output = git_proc::push::new()
        .force()
        .remote(remote)
        .refspec(&refspec)
        .build()
        .output()?;

    if !output.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::GitCommandFailed(stderr.trim().to_string()));
    }

    Ok(())
}

/// Gets the upstream tracking ref for the current branch.
///
/// Reads git config `branch.<name>.remote` and `branch.<name>.merge` directly.
/// If the upstream ref doesn't exist yet, falls back to the remote's default branch.
///
/// For example, if on branch `feature` with config:
/// - `branch.feature.remote` = `origin`
/// - `branch.feature.merge` = `refs/heads/feature`
///
/// This returns `origin/feature` if that ref exists, otherwise `origin/main`.
///
/// Returns `None` if no upstream is configured for the current branch.
pub fn get_upstream() -> Result<Option<super::Ref>, Error> {
    let branch = get_current_branch()?;
    let config = get_config()?;

    let remote_key = format!("branch.{branch}.remote");
    let merge_key = format!("branch.{branch}.merge");

    let remote = config
        .iter()
        .find(|entry| entry.key().as_str() == remote_key)
        .map(ConfigEntry::value);

    let merge = config
        .iter()
        .find(|entry| entry.key().as_str() == merge_key)
        .map(ConfigEntry::value);

    let (Some(remote), Some(merge)) = (remote, merge) else {
        return Ok(None);
    };

    let branch_name = merge
        .as_str()
        .strip_prefix("refs/heads/")
        .ok_or_else(|| Error::GitCommandFailed(format!("Invalid merge ref: {merge}")))?;

    let upstream = format!("{remote}/{branch_name}");

    if remote_ref_exists(&upstream)? {
        return upstream
            .parse()
            .map(Some)
            .map_err(|_| Error::GitCommandFailed(format!("Invalid ref: {upstream}")));
    }

    get_remote_head(remote.as_str())
}

/// Gets the HEAD ref for a remote (e.g., `origin/main`).
fn get_remote_head(remote: &str) -> Result<Option<super::Ref>, Error> {
    let symbolic_ref = format!("refs/remotes/{remote}/HEAD");

    let output = cmd_proc::Command::new("git")
        .arguments(["symbolic-ref", &symbolic_ref])
        .output()?;

    if !output.success() {
        return Ok(None);
    }

    let full_ref = String::from_utf8(output.stdout)?.trim().to_string();

    let branch = full_ref
        .strip_prefix(&format!("refs/remotes/{remote}/"))
        .ok_or_else(|| {
            Error::GitCommandFailed(format!("Invalid default branch ref: {full_ref}"))
        })?;

    let upstream = format!("{remote}/{branch}");

    upstream
        .parse()
        .map(Some)
        .map_err(|_| Error::GitCommandFailed(format!("Invalid ref: {upstream}")))
}

/// Checks if a remote ref exists.
fn remote_ref_exists(ref_name: &str) -> Result<bool, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments(["rev-parse", "--verify", "--quiet", ref_name])
        .output()?;

    Ok(output.success())
}

/// Reads the git config.
fn get_config() -> Result<Vec<ConfigEntry>, Error> {
    let output = cmd_proc::Command::new("git")
        .arguments(["config", "--list"])
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
            ConfigEntry::parse(line)
                .map(|(_, entry)| entry)
                .map_err(|err| {
                    Error::GitCommandFailed(format!("Failed to parse config line '{line}': {err}"))
                })
        })
        .collect()
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
