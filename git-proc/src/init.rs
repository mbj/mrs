use std::path::Path;

use crate::CommandError;

/// Create a new `git init` command builder.
#[must_use]
pub fn new() -> Init<'static> {
    Init::new()
}

/// Builder for `git init` command.
///
/// See `git init --help` for full documentation.
#[derive(Debug)]
pub struct Init<'a> {
    directory: Option<&'a Path>,
    bare: bool,
}

impl<'a> Init<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            directory: None,
            bare: false,
        }
    }

    /// Set the directory to initialize.
    #[must_use]
    pub fn directory(mut self, path: &'a Path) -> Self {
        self.directory = Some(path);
        self
    }

    crate::flag_methods! {
        /// Create a bare repository.
        ///
        /// Corresponds to `--bare`.
        pub fn bare / bare_if, bare, "Conditionally create a bare repository."
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }
}

impl Default for Init<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Init<'_> {
    fn build(self) -> cmd_proc::Command {
        cmd_proc::Command::new("git")
            .argument("init")
            .optional_flag(self.bare, "--bare")
            .optional_argument(self.directory)
    }
}

#[cfg(feature = "test-utils")]
impl Init<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            directory: self.directory,
            bare: self.bare,
        });
        command.test_eq(other);
    }
}
