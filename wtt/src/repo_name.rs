#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepoName(String);

impl RepoName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract a repository name from a repository address.
    ///
    /// Takes the last path component, removes the `.git` suffix if present,
    /// and validates it as a repository name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wtt::{repository, RepoName};
    /// let address: repository::Address = "git@github.com:user/repo.git".parse().unwrap();
    /// let name = RepoName::from_repository_address(&address).unwrap();
    /// assert_eq!(name.as_str(), "repo");
    ///
    /// let address: repository::Address = "https://github.com/user/my-repo".parse().unwrap();
    /// let name = RepoName::from_repository_address(&address).unwrap();
    /// assert_eq!(name.as_str(), "my-repo");
    /// ```
    pub fn from_repository_address(
        address: &crate::repository::Address,
    ) -> Result<Self, RepoNameError> {
        use crate::repository::Address;

        let path = match address {
            Address::Ssh(ssh) => ssh.path(),
            Address::Https(https) => https.path(),
            Address::Git(git) => git.path(),
            Address::Path(path_address) => path_address
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
    use crate::repository;

    #[test]
    fn test_from_repository_address_ssh_scp_style() {
        let address: repository::Address = "git@github.com:user/repo.git".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_repository_address_ssh_scp_style_no_git_suffix() {
        let address: repository::Address = "git@github.com:user/my-repo".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_repository_address_ssh_url_style() {
        let address: repository::Address = "ssh://git@github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_repository_address_https() {
        let address: repository::Address = "https://github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_repository_address_https_no_git_suffix() {
        let address: repository::Address =
            "https://github.com/user/my-awesome-repo".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "my-awesome-repo");
    }

    #[test]
    fn test_from_repository_address_git_protocol() {
        let address: repository::Address = "git://github.com/user/repo.git".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "repo");
    }

    #[test]
    fn test_from_repository_address_path() {
        let address: repository::Address = "/home/user/my-repo.git".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_repository_address_path_no_git_suffix() {
        let address: repository::Address = "/home/user/my-repo".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn test_from_repository_address_trailing_slash() {
        let address: repository::Address = "https://github.com/user/repo.git/".parse().unwrap();
        let name = RepoName::from_repository_address(&address).unwrap();
        assert_eq!(name.as_str(), "repo");
    }
}
