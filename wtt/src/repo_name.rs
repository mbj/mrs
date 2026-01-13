#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepoName(String);

impl RepoName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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
