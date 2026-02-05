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
    porcelain: bool,
    remote: Option<&'a Remote>,
}

crate::impl_repo_path!(Fetch);
crate::impl_porcelain!(Fetch);

impl<'a> Fetch<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            all: false,
            porcelain: false,
            remote: None,
        }
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
    pub async fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status().await
    }
}

impl Default for Fetch<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Fetch<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("fetch")
            .optional_flag(self.all, "--all")
            .optional_flag(self.porcelain, "--porcelain")
            .optional_argument(self.remote)
    }
}

#[cfg(feature = "test-utils")]
impl Fetch<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            all: self.all,
            porcelain: self.porcelain,
            remote: self.remote,
        });
        command.test_eq(other);
    }
}
