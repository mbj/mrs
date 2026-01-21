//! Git branch name type with validation.

use std::borrow::Cow;

/// A validated git branch name.
///
/// Branch names follow git's reference naming rules:
/// - Cannot be empty
/// - Cannot start with `-`, `.`, or `/`
/// - Cannot end with `/`, `.`, or `.lock`
/// - Cannot contain `..`, `//`, `@{`, or control characters
/// - Cannot contain spaces or forbidden characters: `~^:?*[\`
/// - Cannot be single `@`
/// - No component can start with `.` or end with `.lock`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch(Cow<'static, str>);

impl Branch {
    /// Returns the branch name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if the branch name contains path separators.
    #[must_use]
    pub fn has_parents(&self) -> bool {
        self.0.contains('/')
    }

    const fn is_forbidden_char(byte: u8) -> bool {
        matches!(byte, b'~' | b'^' | b':' | b'?' | b'*' | b'[' | b'\\')
    }

    const fn validate(input: &str) -> Result<(), BranchError> {
        if input.is_empty() {
            return Err(BranchError::Empty);
        }

        let bytes = input.as_bytes();

        // Single @ is not allowed
        if bytes.len() == 1 && bytes[0] == b'@' {
            return Err(BranchError::SingleAt);
        }

        // Check first character
        if bytes[0] == b'-' {
            return Err(BranchError::StartsWithDash);
        }
        if bytes[0] == b'.' {
            return Err(BranchError::StartsWithDot);
        }
        if bytes[0] == b'/' {
            return Err(BranchError::StartsWithSlash);
        }

        // Check last character
        if bytes[bytes.len() - 1] == b'/' {
            return Err(BranchError::EndsWithSlash);
        }
        if bytes[bytes.len() - 1] == b'.' {
            return Err(BranchError::EndsWithDot);
        }

        // Check for .lock suffix (need at least 5 chars).
        // Using byte-by-byte comparison because array == is not const-compatible.
        if bytes.len() >= 5
            && bytes[bytes.len() - 5] == b'.'
            && bytes[bytes.len() - 4] == b'l'
            && bytes[bytes.len() - 3] == b'o'
            && bytes[bytes.len() - 2] == b'c'
            && bytes[bytes.len() - 1] == b'k'
        {
            return Err(BranchError::EndsWithLock);
        }

        // Check character-by-character constraints
        // Using index loop because iterators are not const-compatible.
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];

            // Control characters
            if byte < 0x20 || byte == 0x7f {
                return Err(BranchError::ContainsControlCharacter);
            }

            // Space
            if byte == b' ' {
                return Err(BranchError::ContainsSpace);
            }

            // Forbidden characters
            if Self::is_forbidden_char(byte) {
                return Err(BranchError::ContainsForbiddenCharacter);
            }

            // Check for .. sequence
            if byte == b'.' && index + 1 < bytes.len() && bytes[index + 1] == b'.' {
                return Err(BranchError::ContainsDoubleDot);
            }

            // Check for // sequence
            if byte == b'/' && index + 1 < bytes.len() && bytes[index + 1] == b'/' {
                return Err(BranchError::ContainsDoubleSlash);
            }

            // Check for @{ sequence
            if byte == b'@' && index + 1 < bytes.len() && bytes[index + 1] == b'{' {
                return Err(BranchError::ContainsAtBrace);
            }

            // Check for component starting with . (after /)
            if byte == b'/' && index + 1 < bytes.len() && bytes[index + 1] == b'.' {
                return Err(BranchError::ComponentStartsWithDot);
            }

            // Check for component ending with .lock (before /)
            // Pattern: ".lock/" at position where index points to '.'
            if byte == b'.'
                && index + 5 < bytes.len()
                && bytes[index + 1] == b'l'
                && bytes[index + 2] == b'o'
                && bytes[index + 3] == b'c'
                && bytes[index + 4] == b'k'
                && bytes[index + 5] == b'/'
            {
                return Err(BranchError::ComponentEndsWithLock);
            }

            index += 1;
        }

        Ok(())
    }

    /// Creates a branch name from a static string, panicking if invalid.
    ///
    /// This is useful for compile-time constants.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        assert!(Self::validate(input).is_ok(), "invalid branch name");
        Self(Cow::Borrowed(input))
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<std::ffi::OsStr> for Branch {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

impl std::str::FromStr for Branch {
    type Err = BranchError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::validate(input)?;
        Ok(Self(Cow::Owned(input.to_string())))
    }
}

/// Errors that can occur when parsing a branch name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BranchError {
    #[error("branch name cannot be empty")]
    Empty,
    #[error("branch name cannot be single '@'")]
    SingleAt,
    #[error("branch name cannot start with '-'")]
    StartsWithDash,
    #[error("branch name cannot start with '.'")]
    StartsWithDot,
    #[error("branch name cannot start with '/'")]
    StartsWithSlash,
    #[error("branch name cannot end with '/'")]
    EndsWithSlash,
    #[error("branch name cannot end with '.'")]
    EndsWithDot,
    #[error("branch name cannot end with '.lock'")]
    EndsWithLock,
    #[error("branch name cannot contain '..'")]
    ContainsDoubleDot,
    #[error("branch name cannot contain '//'")]
    ContainsDoubleSlash,
    #[error("branch name cannot contain '@{{'")]
    ContainsAtBrace,
    #[error("branch component cannot start with '.'")]
    ComponentStartsWithDot,
    #[error("branch component cannot end with '.lock'")]
    ComponentEndsWithLock,
    #[error("branch name cannot contain control characters")]
    ContainsControlCharacter,
    #[error("branch name cannot contain spaces")]
    ContainsSpace,
    #[error("branch name cannot contain forbidden characters (~^:?*[\\)")]
    ContainsForbiddenCharacter,
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
    fn test_has_parents() {
        assert!(!Branch::from_static_or_panic("main").has_parents());
        assert!(Branch::from_static_or_panic("feature/login").has_parents());
    }

    #[test]
    fn test_empty() {
        assert!(matches!("".parse::<Branch>(), Err(BranchError::Empty)));
    }

    #[test]
    fn test_single_at() {
        assert!(matches!("@".parse::<Branch>(), Err(BranchError::SingleAt)));
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
    fn test_starts_with_slash() {
        assert!(matches!(
            "/branch".parse::<Branch>(),
            Err(BranchError::StartsWithSlash)
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
    fn test_ends_with_dot() {
        assert!(matches!(
            "branch.".parse::<Branch>(),
            Err(BranchError::EndsWithDot)
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
    fn test_contains_at_brace() {
        assert!(matches!(
            "branch@{name".parse::<Branch>(),
            Err(BranchError::ContainsAtBrace)
        ));
    }

    #[test]
    fn test_component_starts_with_dot() {
        assert!(matches!(
            "feature/.hidden".parse::<Branch>(),
            Err(BranchError::ComponentStartsWithDot)
        ));
        assert!(matches!(
            "a/b/.c/d".parse::<Branch>(),
            Err(BranchError::ComponentStartsWithDot)
        ));
    }

    #[test]
    fn test_component_ends_with_lock() {
        assert!(matches!(
            "feature/branch.lock/next".parse::<Branch>(),
            Err(BranchError::ComponentEndsWithLock)
        ));
    }

    #[test]
    fn test_contains_space() {
        assert!(matches!(
            "branch name".parse::<Branch>(),
            Err(BranchError::ContainsSpace)
        ));
    }

    #[test]
    fn test_contains_control_character() {
        assert!(matches!(
            "branch\x00name".parse::<Branch>(),
            Err(BranchError::ContainsControlCharacter)
        ));
        assert!(matches!(
            "branch\tname".parse::<Branch>(),
            Err(BranchError::ContainsControlCharacter)
        ));
    }

    #[test]
    fn test_contains_forbidden_characters() {
        assert!(matches!(
            "branch~name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch^name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch:name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch?name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch*name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch[name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
        assert!(matches!(
            "branch\\name".parse::<Branch>(),
            Err(BranchError::ContainsForbiddenCharacter)
        ));
    }

    #[test]
    fn test_from_static_or_panic() {
        let branch = Branch::from_static_or_panic("main");
        assert_eq!(branch.as_str(), "main");
    }

    #[test]
    fn test_display() {
        let branch: Branch = "feature/test".parse().unwrap();
        assert_eq!(format!("{branch}"), "feature/test");
    }

    #[test]
    fn test_as_ref_os_str() {
        use std::ffi::OsStr;
        let branch: Branch = "main".parse().unwrap();
        let os_str: &OsStr = branch.as_ref();
        assert_eq!(os_str, "main");
    }
}
