//! Git tag name type with validation, plus `git tag` command builders.

use std::path::Path;

use crate::CommandError;
use crate::ref_format::{self, RefFormatError};

crate::cow_str_newtype! {
    /// A validated git tag name.
    ///
    /// Tag names follow git's reference naming rules; see
    /// [`crate::ref_format`] for the full ruleset.
    pub struct Tag, TagError(RefFormatError), "invalid tag name"
}

impl Tag {
    const fn validate(input: &str) -> Result<(), TagError> {
        match ref_format::validate(input) {
            Ok(()) => Ok(()),
            Err(error) => Err(TagError(error)),
        }
    }
}

/// Create a new `git tag <name>` command builder.
#[must_use]
pub fn create(name: &Tag) -> Create<'_> {
    Create::new(name)
}

/// Builder for `git tag <name>` (lightweight tag at HEAD).
///
/// See `git tag --help` for full documentation.
#[derive(Debug)]
pub struct Create<'a> {
    repo_path: Option<&'a Path>,
    name: &'a Tag,
}

crate::impl_repo_path!(Create);

impl<'a> Create<'a> {
    #[must_use]
    fn new(name: &'a Tag) -> Self {
        Self {
            repo_path: None,
            name,
        }
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status().await
    }
}

impl crate::Build for Create<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("tag")
            .argument(self.name)
    }
}

#[cfg(feature = "test-utils")]
impl Create<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            name: self.name,
        });
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tag() {
        assert!("v1.0.0".parse::<Tag>().is_ok());
        assert!("release-2024".parse::<Tag>().is_ok());
        assert!("hotfix/v1.2.3".parse::<Tag>().is_ok());
    }

    #[test]
    fn test_invalid_passes_through() {
        assert!(matches!(
            "".parse::<Tag>(),
            Err(TagError(RefFormatError::Empty))
        ));
        assert!(matches!(
            "-tag".parse::<Tag>(),
            Err(TagError(RefFormatError::StartsWithDash))
        ));
        assert!(matches!(
            "v1.0.0.lock".parse::<Tag>(),
            Err(TagError(RefFormatError::EndsWithLock))
        ));
    }

    #[test]
    fn test_from_static_or_panic() {
        let tag = Tag::from_static_or_panic("v1.0.0");
        assert_eq!(tag.as_str(), "v1.0.0");
    }

    #[test]
    fn test_display() {
        let tag: Tag = "v1.0.0".parse().unwrap();
        assert_eq!(format!("{tag}"), "v1.0.0");
    }

    #[test]
    fn test_as_ref_os_str() {
        use std::ffi::OsStr;
        let tag: Tag = "v1.0.0".parse().unwrap();
        let os_str: &OsStr = tag.as_ref();
        assert_eq!(os_str, "v1.0.0");
    }

    #[test]
    fn test_serialize() {
        let tag: Tag = "v1.0.0".parse().unwrap();
        assert_eq!(serde_json::to_string(&tag).unwrap(), "\"v1.0.0\"");
    }

    #[test]
    fn test_deserialize() {
        let tag: Tag = serde_json::from_str("\"v1.0.0\"").unwrap();
        assert_eq!(tag.as_str(), "v1.0.0");
    }

    #[test]
    fn test_deserialize_invalid() {
        assert!(serde_json::from_str::<Tag>("\"-bad\"").is_err());
    }
}
