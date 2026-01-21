use std::path::Path;

use crate::CommandError;
use crate::url::Remote;

/// Create a new `git fetch` command builder.
#[must_use]
pub fn new() -> Fetch<'static> {
    Fetch::new()
}

/// Builder for `git fetch` command.
///
/// See `git fetch --help` for full documentation.
#[derive(Debug)]
pub struct Fetch<'a> {
    repo_path: Option<&'a Path>,
    all: bool,
    remote: Option<&'a Remote>,
}

impl<'a> Fetch<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            all: false,
            remote: None,
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    crate::flag_methods! {
        /// Fetch all remotes.
        ///
        /// Corresponds to `--all`.
        pub fn all / all_if, all, "Conditionally fetch all remotes."
    }

    /// Set the remote to fetch from.
    #[must_use]
    pub fn remote(mut self, remote: &'a Remote) -> Self {
        self.remote = Some(remote);
        self
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        self.build().status()
    }

    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("fetch")
            .optional_argument(self.all.then_some("--all"))
            .optional_argument(self.remote)
    }
}

impl Default for Fetch<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "test-utils")]
impl Fetch<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = Self {
            repo_path: self.repo_path,
            all: self.all,
            remote: self.remote,
        }
        .build();
        command.test_eq(other);
    }
}
