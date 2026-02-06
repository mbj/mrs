use std::path::Path;

use crate::CommandError;
use crate::url::GitUrl;

/// Create a new `git clone` command builder.
#[must_use]
pub fn new(url: &GitUrl) -> Clone<'_> {
    Clone::new(url)
}

/// Builder for `git clone` command.
///
/// See `git clone --help` for full documentation.
#[derive(Debug)]
pub struct Clone<'a> {
    url: &'a GitUrl,
    directory: Option<&'a Path>,
    bare: bool,
}

impl<'a> Clone<'a> {
    #[must_use]
    fn new(url: &'a GitUrl) -> Self {
        Self {
            url,
            directory: None,
            bare: false,
        }
    }

    /// Set the destination directory.
    #[must_use]
    pub fn directory(mut self, path: &'a Path) -> Self {
        self.directory = Some(path);
        self
    }

    crate::flag_methods! {
        /// Make a bare clone.
        ///
        /// Corresponds to `--bare`.
        pub fn bare / bare_if, bare, "Conditionally make a bare clone."
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status().await
    }
}

impl crate::Build for Clone<'_> {
    fn build(self) -> cmd_proc::Command {
        cmd_proc::Command::new("git")
            .argument("clone")
            .optional_flag(self.bare, "--bare")
            .argument(self.url)
            .optional_argument(self.directory)
    }
}

#[cfg(feature = "test-utils")]
impl Clone<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            url: self.url,
            directory: self.directory,
            bare: self.bare,
        });
        command.test_eq(other);
    }
}
