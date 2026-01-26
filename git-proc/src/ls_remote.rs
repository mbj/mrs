use std::path::Path;

use crate::url::Remote;

/// Create a new `git ls-remote` command builder.
#[must_use]
pub fn new() -> LsRemote<'static> {
    LsRemote::new()
}

/// Builder for `git ls-remote` command.
///
/// See `git ls-remote --help` for full documentation.
#[derive(Debug)]
pub struct LsRemote<'a> {
    repo_path: Option<&'a Path>,
    heads: bool,
    symref: bool,
    remote: Option<&'a Remote>,
    pattern: Option<&'a str>,
}

crate::impl_repo_path!(LsRemote);

impl<'a> LsRemote<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            heads: false,
            symref: false,
            remote: None,
            pattern: None,
        }
    }

    crate::flag_methods! {
        /// Limit to refs/heads (branches only).
        ///
        /// Corresponds to `--heads`.
        pub fn heads / heads_if, heads, "Conditionally limit to refs/heads."
    }

    crate::flag_methods! {
        /// Show underlying ref in addition to the object.
        ///
        /// Corresponds to `--symref`. Useful for finding the default branch.
        pub fn symref / symref_if, symref, "Conditionally show underlying ref."
    }

    /// Set the remote repository to query.
    #[must_use]
    pub fn remote(mut self, remote: &'a Remote) -> Self {
        self.remote = Some(remote);
        self
    }

    /// Set the pattern to filter refs.
    #[must_use]
    pub fn pattern(mut self, pattern: &'a str) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        crate::Build::build(self).stdout()
    }
}

impl Default for LsRemote<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for LsRemote<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("ls-remote")
            .optional_flag(self.heads, "--heads")
            .optional_flag(self.symref, "--symref")
            .optional_argument(self.remote)
            .optional_argument(self.pattern)
    }
}

#[cfg(feature = "test-utils")]
impl LsRemote<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            heads: self.heads,
            symref: self.symref,
            remote: self.remote,
            pattern: self.pattern,
        });
        command.test_eq(other);
    }
}
