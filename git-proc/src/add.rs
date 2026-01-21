use std::path::Path;

use crate::CommandError;

/// Create a new `git add` command builder.
#[must_use]
pub fn new() -> Add<'static> {
    Add::new()
}

/// Builder for `git add` command.
///
/// See `git add --help` for full documentation.
#[derive(Debug)]
pub struct Add<'a> {
    repo_path: Option<&'a Path>,
    all: bool,
    pathspecs: Vec<&'a str>,
}

impl<'a> Add<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            all: false,
            pathspecs: Vec::new(),
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    crate::flag_methods! {
        /// Add all changes (new, modified, deleted).
        ///
        /// Corresponds to `--all` or `-A`.
        pub fn all / all_if, all, "Conditionally add all changes."
    }

    /// Add a pathspec to stage.
    #[must_use]
    pub fn pathspec(mut self, pathspec: &'a str) -> Self {
        self.pathspecs.push(pathspec);
        self
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        self.build().status()
    }

    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("add")
            .optional_argument(self.all.then_some("--all"))
            .arguments(self.pathspecs)
    }
}

impl Default for Add<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "test-utils")]
impl Add<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = Self {
            repo_path: self.repo_path,
            all: self.all,
            pathspecs: self.pathspecs.clone(),
        }
        .build();
        command.test_eq(other);
    }
}
