#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitUrl(String);

impl GitUrl {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for GitUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<std::ffi::OsStr> for GitUrl {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl std::str::FromStr for GitUrl {
    type Err = GitUrlError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(GitUrlError::Empty);
        }

        Ok(Self(string.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitUrlError {
    #[error("Git URL cannot be empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        assert!("git@github.com:user/repo.git".parse::<GitUrl>().is_ok());
        assert!("https://github.com/user/repo.git".parse::<GitUrl>().is_ok());
    }

    #[test]
    fn test_empty() {
        assert!(matches!("".parse::<GitUrl>(), Err(GitUrlError::Empty)));
    }
}
