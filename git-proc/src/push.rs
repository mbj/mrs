use std::path::Path;

use crate::CommandError;
use crate::url::Remote;

/// Create a new `git push` command builder.
#[must_use]
pub fn new() -> Push<'static> {
    Push::new()
}

/// Builder for `git push` command.
///
/// See `git push --help` for full documentation.
#[derive(Debug)]
pub struct Push<'a> {
    repo_path: Option<&'a Path>,
    force: bool,
    porcelain: bool,
    remote: Option<&'a Remote>,
    refspec: Option<&'a str>,
}

crate::impl_repo_path!(Push);
crate::impl_porcelain!(Push);

impl<'a> Push<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            force: false,
            porcelain: false,
            remote: None,
            refspec: None,
        }
    }

    crate::flag_methods! {
        /// Force push (overwrite remote refs).
        ///
        /// Corresponds to `--force`.
        pub fn force / force_if, force, "Conditionally force push."
    }

    /// Set the remote to push to.
    #[must_use]
    pub fn remote(mut self, remote: &'a Remote) -> Self {
        self.remote = Some(remote);
        self
    }

    /// Set the refspec to push.
    #[must_use]
    pub fn refspec(mut self, refspec: &'a str) -> Self {
        self.refspec = Some(refspec);
        self
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status().await
    }
}

impl Default for Push<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Push<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("push")
            .optional_flag(self.force, "--force")
            .optional_flag(self.porcelain, "--porcelain")
            .optional_argument(self.remote)
            .optional_argument(self.refspec)
    }
}

#[cfg(feature = "test-utils")]
impl Push<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            force: self.force,
            porcelain: self.porcelain,
            remote: self.remote,
            refspec: self.refspec,
        });
        command.test_eq(other);
    }
}
