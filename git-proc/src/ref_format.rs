//! Shared validation for git reference names.
//!
//! Branches and tags are both git refs and share the rules enforced by
//! `git check-ref-format`. This module is the single source of truth for those
//! rules; type-specific newtypes (`Branch`, `Tag`) delegate to [`validate`].

const fn is_forbidden_char(byte: u8) -> bool {
    matches!(byte, b'~' | b'^' | b':' | b'?' | b'*' | b'[' | b'\\')
}

/// Validate a git reference name against `git check-ref-format` rules.
///
/// Single-level names (no `/`) are accepted, equivalent to git's
/// `--allow-onelevel`. The leading `-` rejection guards against shell
/// argument confusion.
pub const fn validate(input: &str) -> Result<(), RefFormatError> {
    if input.is_empty() {
        return Err(RefFormatError::Empty);
    }

    let bytes = input.as_bytes();

    if bytes.len() == 1 && bytes[0] == b'@' {
        return Err(RefFormatError::SingleAt);
    }

    if bytes[0] == b'-' {
        return Err(RefFormatError::StartsWithDash);
    }
    if bytes[0] == b'.' {
        return Err(RefFormatError::StartsWithDot);
    }
    if bytes[0] == b'/' {
        return Err(RefFormatError::StartsWithSlash);
    }

    if bytes[bytes.len() - 1] == b'/' {
        return Err(RefFormatError::EndsWithSlash);
    }
    if bytes[bytes.len() - 1] == b'.' {
        return Err(RefFormatError::EndsWithDot);
    }

    // ".lock" suffix; byte-by-byte because array == is not const.
    if bytes.len() >= 5
        && bytes[bytes.len() - 5] == b'.'
        && bytes[bytes.len() - 4] == b'l'
        && bytes[bytes.len() - 3] == b'o'
        && bytes[bytes.len() - 2] == b'c'
        && bytes[bytes.len() - 1] == b'k'
    {
        return Err(RefFormatError::EndsWithLock);
    }

    let mut index = 0;
    // Index loop because iterators are not const-compatible.
    while index < bytes.len() {
        let byte = bytes[index];

        if byte < 0x20 || byte == 0x7f {
            return Err(RefFormatError::ContainsControlCharacter);
        }

        if byte == b' ' {
            return Err(RefFormatError::ContainsSpace);
        }

        if is_forbidden_char(byte) {
            return Err(RefFormatError::ContainsForbiddenCharacter);
        }

        if byte == b'.' && index + 1 < bytes.len() && bytes[index + 1] == b'.' {
            return Err(RefFormatError::ContainsDoubleDot);
        }

        if byte == b'/' && index + 1 < bytes.len() && bytes[index + 1] == b'/' {
            return Err(RefFormatError::ContainsDoubleSlash);
        }

        if byte == b'@' && index + 1 < bytes.len() && bytes[index + 1] == b'{' {
            return Err(RefFormatError::ContainsAtBrace);
        }

        if byte == b'/' && index + 1 < bytes.len() && bytes[index + 1] == b'.' {
            return Err(RefFormatError::ComponentStartsWithDot);
        }

        if byte == b'.'
            && index + 5 < bytes.len()
            && bytes[index + 1] == b'l'
            && bytes[index + 2] == b'o'
            && bytes[index + 3] == b'c'
            && bytes[index + 4] == b'k'
            && bytes[index + 5] == b'/'
        {
            return Err(RefFormatError::ComponentEndsWithLock);
        }

        index += 1;
    }

    Ok(())
}

/// Errors that can occur when validating a git reference name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum RefFormatError {
    #[error("git ref name cannot be empty")]
    Empty,
    #[error("git ref name cannot be single '@'")]
    SingleAt,
    #[error("git ref name cannot start with '-'")]
    StartsWithDash,
    #[error("git ref name cannot start with '.'")]
    StartsWithDot,
    #[error("git ref name cannot start with '/'")]
    StartsWithSlash,
    #[error("git ref name cannot end with '/'")]
    EndsWithSlash,
    #[error("git ref name cannot end with '.'")]
    EndsWithDot,
    #[error("git ref name cannot end with '.lock'")]
    EndsWithLock,
    #[error("git ref name cannot contain '..'")]
    ContainsDoubleDot,
    #[error("git ref name cannot contain '//'")]
    ContainsDoubleSlash,
    #[error("git ref name cannot contain '@{{'")]
    ContainsAtBrace,
    #[error("git ref component cannot start with '.'")]
    ComponentStartsWithDot,
    #[error("git ref component cannot end with '.lock'")]
    ComponentEndsWithLock,
    #[error("git ref name cannot contain control characters")]
    ContainsControlCharacter,
    #[error("git ref name cannot contain spaces")]
    ContainsSpace,
    #[error("git ref name cannot contain forbidden characters (~^:?*[\\)")]
    ContainsForbiddenCharacter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(validate("main").is_ok());
        assert!(validate("v1.0.0").is_ok());
        assert!(validate("feature/login").is_ok());
        assert!(validate("a/b/c").is_ok());
    }

    #[test]
    fn test_empty() {
        assert_eq!(validate(""), Err(RefFormatError::Empty));
    }

    #[test]
    fn test_single_at() {
        assert_eq!(validate("@"), Err(RefFormatError::SingleAt));
    }

    #[test]
    fn test_starts_with_dash() {
        assert_eq!(validate("-x"), Err(RefFormatError::StartsWithDash));
    }

    #[test]
    fn test_starts_with_dot() {
        assert_eq!(validate(".x"), Err(RefFormatError::StartsWithDot));
    }

    #[test]
    fn test_starts_with_slash() {
        assert_eq!(validate("/x"), Err(RefFormatError::StartsWithSlash));
    }

    #[test]
    fn test_ends_with_slash() {
        assert_eq!(validate("x/"), Err(RefFormatError::EndsWithSlash));
    }

    #[test]
    fn test_ends_with_dot() {
        assert_eq!(validate("x."), Err(RefFormatError::EndsWithDot));
    }

    #[test]
    fn test_ends_with_lock() {
        assert_eq!(validate("x.lock"), Err(RefFormatError::EndsWithLock));
    }

    #[test]
    fn test_contains_double_dot() {
        assert_eq!(validate("a..b"), Err(RefFormatError::ContainsDoubleDot));
    }

    #[test]
    fn test_contains_double_slash() {
        assert_eq!(validate("a//b"), Err(RefFormatError::ContainsDoubleSlash));
    }

    #[test]
    fn test_contains_at_brace() {
        assert_eq!(validate("a@{b"), Err(RefFormatError::ContainsAtBrace));
    }

    #[test]
    fn test_component_starts_with_dot() {
        assert_eq!(
            validate("a/.b"),
            Err(RefFormatError::ComponentStartsWithDot)
        );
        assert_eq!(
            validate("a/b/.c/d"),
            Err(RefFormatError::ComponentStartsWithDot)
        );
    }

    #[test]
    fn test_component_ends_with_lock() {
        assert_eq!(
            validate("a/b.lock/c"),
            Err(RefFormatError::ComponentEndsWithLock)
        );
    }

    #[test]
    fn test_contains_space() {
        assert_eq!(validate("a b"), Err(RefFormatError::ContainsSpace));
    }

    #[test]
    fn test_contains_control_character() {
        assert_eq!(
            validate("a\x00b"),
            Err(RefFormatError::ContainsControlCharacter)
        );
        assert_eq!(
            validate("a\tb"),
            Err(RefFormatError::ContainsControlCharacter)
        );
        assert_eq!(
            validate("a\x7fb"),
            Err(RefFormatError::ContainsControlCharacter)
        );
    }

    #[test]
    fn test_contains_forbidden_characters() {
        for bad in ["a~b", "a^b", "a:b", "a?b", "a*b", "a[b", "a\\b"] {
            assert_eq!(
                validate(bad),
                Err(RefFormatError::ContainsForbiddenCharacter)
            );
        }
    }
}
