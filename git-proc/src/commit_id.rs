//! Git commit object ID type with validation.

crate::cow_str_newtype! {
    /// A validated git commit object ID.
    ///
    /// Accepts full SHA-1 (40 hex characters) or SHA-256 (64 hex characters)
    /// strings. Both lowercase and uppercase hex digits are allowed and the
    /// input case is preserved.
    ///
    /// Abbreviated SHAs are deliberately rejected: this type is intended for
    /// pinned-commit use cases where the caller has the full object ID.
    pub struct CommitId, CommitIdError(InvalidCommitId), "invalid commit id"
}

impl CommitId {
    const fn validate(input: &str) -> Result<(), CommitIdError> {
        let bytes = input.as_bytes();
        let len = bytes.len();

        if len == 0 {
            return Err(CommitIdError(InvalidCommitId::Empty));
        }
        if len != 40 && len != 64 {
            return Err(CommitIdError(InvalidCommitId::InvalidLength));
        }

        let mut index = 0;
        while index < bytes.len() {
            if !bytes[index].is_ascii_hexdigit() {
                return Err(CommitIdError(InvalidCommitId::ContainsNonHexCharacter));
            }
            index += 1;
        }

        Ok(())
    }
}

/// Reasons a string fails to parse as a commit ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum InvalidCommitId {
    #[error("commit id cannot be empty")]
    Empty,
    #[error("commit id must be 40 (SHA-1) or 64 (SHA-256) hex characters")]
    InvalidLength,
    #[error("commit id must contain only hex characters")]
    ContainsNonHexCharacter,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA1: &str = "0123456789abcdef0123456789abcdef01234567";
    const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn test_valid_sha1() {
        let id: CommitId = SHA1.parse().unwrap();
        assert_eq!(id.as_str(), SHA1);
    }

    #[test]
    fn test_valid_sha256() {
        let id: CommitId = SHA256.parse().unwrap();
        assert_eq!(id.as_str(), SHA256);
    }

    #[test]
    fn test_uppercase_accepted() {
        let id: CommitId = "DEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF".parse().unwrap();
        assert_eq!(id.as_str(), "DEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF");
    }

    #[test]
    fn test_empty() {
        assert!(matches!(
            "".parse::<CommitId>(),
            Err(CommitIdError(InvalidCommitId::Empty))
        ));
    }

    #[test]
    fn test_abbreviated_rejected() {
        assert!(matches!(
            "abc1234".parse::<CommitId>(),
            Err(CommitIdError(InvalidCommitId::InvalidLength))
        ));
    }

    #[test]
    fn test_wrong_length() {
        assert!(matches!(
            "abc".parse::<CommitId>(),
            Err(CommitIdError(InvalidCommitId::InvalidLength))
        ));
        // 41 chars
        assert!(matches!(
            "0123456789abcdef0123456789abcdef012345670".parse::<CommitId>(),
            Err(CommitIdError(InvalidCommitId::InvalidLength))
        ));
    }

    #[test]
    fn test_non_hex() {
        // 40 chars but contains 'g'
        let bad = "g123456789abcdef0123456789abcdef01234567";
        assert_eq!(bad.len(), 40);
        assert!(matches!(
            bad.parse::<CommitId>(),
            Err(CommitIdError(InvalidCommitId::ContainsNonHexCharacter))
        ));
    }

    #[test]
    fn test_from_static_or_panic() {
        let id = CommitId::from_static_or_panic(SHA1);
        assert_eq!(id.as_str(), SHA1);
    }

    #[test]
    fn test_serde_roundtrip() {
        let id: CommitId = SHA1.parse().unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, format!("\"{SHA1}\""));
        let back: CommitId = serde_json::from_str(&json).unwrap();
        assert_eq!(back.as_str(), SHA1);
    }
}
