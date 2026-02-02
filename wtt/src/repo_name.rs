#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepoName(String);

impl RepoName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract a repository name from a git URL.
    ///
    /// Takes the last path component, removes the `.git` suffix if present,
    /// and validates it as a repository name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wtt::{GitUrl, RepoName};
    /// let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
    /// let name = RepoName::from_git_url(&url).unwrap();
    /// assert_eq!(name.as_str(), "repo");
    ///
    /// let url: GitUrl = "https://github.com/user/my-repo".parse().unwrap();
    /// let name = RepoName::from_git_url(&url).unwrap();
    /// assert_eq!(name.as_str(), "my-repo");
    /// ```
    pub fn from_git_url(url: &crate::GitUrl) -> Result<Self, RepoNameError> {
        use crate::GitUrl;

        let path = match url {
            GitUrl::Ssh(ssh) => ssh.path(),
            GitUrl::Https(https) => https.path(),
            GitUrl::Git(git) => git.path(),
            GitUrl::Path(path_url) => path_url
                .path()
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or(RepoNameError::Empty)?,
        };

        let last_component = path
            .trim_end_matches('/')
            .split('/')
            .next_back()
            .ok_or(RepoNameError::Empty)?;

        let name = last_component
            .strip_suffix(".git")
            .unwrap_or(last_component);

        if name.is_empty() {
            return Err(RepoNameError::Empty);
        }

        name.parse()
    }
}

impl std::fmt::Display for RepoName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for RepoName {
    type Err = RepoNameError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(RepoNameError::Empty);
        }
        if string.contains('/') || string.contains('\\') {
            return Err(RepoNameError::ContainsPathSeparator);
        }
        if string.starts_with('.') {
            return Err(RepoNameError::StartsWithDot);
        }
        Ok(Self(string.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepoNameError {
    #[error("Repository name cannot be empty")]
    Empty,
    #[error("Repository name cannot contain path separators")]
    ContainsPathSeparator,
    #[error("Repository name cannot start with a dot")]
    StartsWithDot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GitUrl;

    #[test]
    fn test_from_git_url_ssh_scp_style() {
        let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_git_url_ssh_scp_style_no_git_suffix() {
        let url: GitUrl = "git@github.com:user/my-repo".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_git_url_ssh_url_style() {
        let url: GitUrl = "ssh://git@github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_git_url_https() {
        let url: GitUrl = "https://github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_git_url_https_no_git_suffix() {
        let url: GitUrl = "https://github.com/user/my-awesome-repo".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "my-awesome-repo");
    }

    #[test]
    fn test_from_git_url_git_protocol() {
        let url: GitUrl = "git://github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_git_url_path() {
        let url: GitUrl = "/home/user/my-repo.git".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_git_url_path_no_git_suffix() {
        let url: GitUrl = "/home/user/my-repo".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_git_url_trailing_slash() {
        let url: GitUrl = "https://github.com/user/repo.git/".parse().unwrap();
        let name = RepoName::from_git_url(&url).unwrap();
        assert_eq!(name.as_str(), "repo");
    }
}
