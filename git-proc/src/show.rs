use std::path::Path;

use crate::CommandError;

/// Create a new `git show` command builder.
///
/// The object can be a commit, tree, blob, or tag reference.
/// For file contents at a specific revision, use format: `revision:path`
#[must_use]
pub fn new(object: &str) -> Show<'_> {
    Show::new(object)
}

/// Builder for `git show` command.
///
/// See `git show --help` for full documentation.
#[derive(Debug)]
pub struct Show<'a> {
    repo_path: Option<&'a Path>,
    object: &'a str,
}

impl<'a> Show<'a> {
    #[must_use]
    fn new(object: &'a str) -> Self {
        Self {
            repo_path: None,
            object,
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        self.build().stdout()
    }

    /// Execute and return full output regardless of exit status.
    ///
    /// Use this when you need to inspect stderr on failure.
    pub fn output(self) -> Result<cmd_proc::Output, CommandError> {
        self.build().output()
    }

    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("show")
            .argument(self.object)
    }
}

#[cfg(feature = "test-utils")]
impl Show<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = Self {
            repo_path: self.repo_path,
            object: self.object,
        }
        .build();
        command.test_eq(other);
    }
}
