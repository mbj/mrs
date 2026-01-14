#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch(String);

impl Branch {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn has_parents(&self) -> bool {
        self.0.contains('/')
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<std::ffi::OsStr> for Branch {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl std::str::FromStr for Branch {
    type Err = BranchError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(BranchError::Empty);
        }
        if string.starts_with('-') {
            return Err(BranchError::StartsWithDash);
        }
        if string.starts_with('.') {
            return Err(BranchError::StartsWithDot);
        }
        if string.ends_with('/') {
            return Err(BranchError::EndsWithSlash);
        }
        if string.ends_with(".lock") {
            return Err(BranchError::EndsWithLock);
        }
        if string.contains("..") {
            return Err(BranchError::ContainsDoubleDot);
        }
        if string.contains("//") {
            return Err(BranchError::ContainsDoubleSlash);
        }
        if string.bytes().any(|byte| byte.is_ascii_control()) {
            return Err(BranchError::ContainsControlCharacter);
        }
        if string.contains(' ') {
            return Err(BranchError::ContainsSpace);
        }

        Ok(Self(string.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BranchError {
    #[error("Branch name cannot be empty")]
    Empty,
    #[error("Branch name cannot start with '-'")]
    StartsWithDash,
    #[error("Branch name cannot start with '.'")]
    StartsWithDot,
    #[error("Branch name cannot end with '/'")]
    EndsWithSlash,
    #[error("Branch name cannot end with '.lock'")]
    EndsWithLock,
    #[error("Branch name cannot contain '..'")]
    ContainsDoubleDot,
    #[error("Branch name cannot contain '//'")]
    ContainsDoubleSlash,
    #[error("Branch name cannot contain control characters")]
    ContainsControlCharacter,
    #[error("Branch name cannot contain spaces")]
    ContainsSpace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_branch() {
        assert!("main".parse::<Branch>().is_ok());
        assert!("feature/login".parse::<Branch>().is_ok());
        assert!("feature/deeply/nested/branch".parse::<Branch>().is_ok());
        assert!("fix-123".parse::<Branch>().is_ok());
    }

    #[test]
    fn test_empty() {
        assert!(matches!("".parse::<Branch>(), Err(BranchError::Empty)));
    }

    #[test]
    fn test_starts_with_dash() {
        assert!(matches!(
            "-branch".parse::<Branch>(),
            Err(BranchError::StartsWithDash)
        ));
    }

    #[test]
    fn test_starts_with_dot() {
        assert!(matches!(
            ".branch".parse::<Branch>(),
            Err(BranchError::StartsWithDot)
        ));
    }

    #[test]
    fn test_ends_with_slash() {
        assert!(matches!(
            "branch/".parse::<Branch>(),
            Err(BranchError::EndsWithSlash)
        ));
    }

    #[test]
    fn test_ends_with_lock() {
        assert!(matches!(
            "branch.lock".parse::<Branch>(),
            Err(BranchError::EndsWithLock)
        ));
    }

    #[test]
    fn test_contains_double_dot() {
        assert!(matches!(
            "branch..name".parse::<Branch>(),
            Err(BranchError::ContainsDoubleDot)
        ));
    }

    #[test]
    fn test_contains_double_slash() {
        assert!(matches!(
            "feature//branch".parse::<Branch>(),
            Err(BranchError::ContainsDoubleSlash)
        ));
    }

    #[test]
    fn test_contains_space() {
        assert!(matches!(
            "branch name".parse::<Branch>(),
            Err(BranchError::ContainsSpace)
        ));
    }
}
