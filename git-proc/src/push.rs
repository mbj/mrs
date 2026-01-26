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
    remote: Option<&'a Remote>,
    refspec: Option<&'a str>,
}

crate::impl_repo_path!(Push);

impl<'a> Push<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            force: false,
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
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        crate::Build::build(self).stdout()
    }

    /// Execute and return full output regardless of exit status.
    ///
    /// Use this when you need to inspect stderr on failure.
    pub fn output(self) -> Result<cmd_proc::Output, CommandError> {
        crate::Build::build(self).output()
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
            .optional_argument(self.force.then_some("--force"))
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
            remote: self.remote,
            refspec: self.refspec,
        });
        command.test_eq(other);
    }
}
