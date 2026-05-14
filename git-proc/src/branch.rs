//! Git branch name type with validation.

use crate::ref_format::{self, RefFormatError};

crate::cow_str_newtype! {
    /// A validated git branch name.
    ///
    /// Branch names follow git's reference naming rules; see
    /// [`crate::ref_format`] for the full ruleset.
    pub struct Branch, BranchError(RefFormatError), "invalid branch name"
}

impl Branch {
    /// Returns true if the branch name contains path separators.
    #[must_use]
    pub fn has_parents(&self) -> bool {
        self.0.contains('/')
    }

    const fn validate(input: &str) -> Result<(), BranchError> {
        match ref_format::validate(input) {
            Ok(()) => Ok(()),
            Err(error) => Err(BranchError(error)),
        }
    }
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
    fn test_invalid_passes_through() {
        // Spot-check that validation is wired up; full coverage lives in ref_format.
        assert!(matches!(
            "".parse::<Branch>(),
            Err(BranchError(RefFormatError::Empty))
        ));
        assert!(matches!(
            "-branch".parse::<Branch>(),
            Err(BranchError(RefFormatError::StartsWithDash))
        ));
        assert!(matches!(
            "feature/.hidden".parse::<Branch>(),
            Err(BranchError(RefFormatError::ComponentStartsWithDot))
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

    #[test]
    fn test_serialize() {
        let branch: Branch = "feature/login".parse().unwrap();
        assert_eq!(serde_json::to_string(&branch).unwrap(), "\"feature/login\"");
    }

    #[test]
    fn test_deserialize() {
        let branch: Branch = serde_json::from_str("\"feature/login\"").unwrap();
        assert_eq!(branch.as_str(), "feature/login");
    }

    #[test]
    fn test_deserialize_invalid() {
        assert!(serde_json::from_str::<Branch>("\"-bad\"").is_err());
    }
}
