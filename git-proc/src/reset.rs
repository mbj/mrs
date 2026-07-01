//! `git reset` command builder.
//!
//! Uses the same typestate as `checkout` for the optional commit-ish target,
//! but `Build`/`status` are available in **both** states because `git reset`
//! without a target is valid (defaults to `HEAD`).

use std::marker::PhantomData;
use std::path::Path;

use crate::commit_ish::{CommitIsh, NoTarget, WithTarget};

/// Create a new `git reset` command builder.
#[must_use]
pub fn new() -> Reset<'static, NoTarget> {
    Reset {
        base: crate::RepoBase::default(),
        hard: false,
        state: NoTarget,
        _phantom: PhantomData,
    }
}

/// Builder for `git reset` command.
///
/// See `git reset --help` for full documentation.
#[derive(Debug)]
pub struct Reset<'a, S> {
    base: crate::RepoBase<'a>,
    hard: bool,
    state: S,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, S> Reset<'a, S> {
    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.base.repo_path = Some(path);
        self
    }

    /// Set the [`EnvPolicy`](crate::EnvPolicy) controlling which inherited
    /// `GIT_*` variables are cleared before spawning `git`.
    #[must_use]
    pub fn env_policy(mut self, policy: crate::EnvPolicy) -> Self {
        self.base.env_policy = policy;
        self
    }

    crate::flag_methods! {
        /// Reset the index and working tree.
        ///
        /// Corresponds to `--hard`.
        pub fn hard / hard_if, hard, "Conditionally reset the index and working tree."
    }
}

impl<'a, S> crate::RepoPath<'a> for Reset<'a, S> {
    fn repo_path(self, path: &'a Path) -> Self {
        self.repo_path(path)
    }
}

impl<'a> Reset<'a, NoTarget> {
    /// Select the commit-ish to reset to.
    ///
    /// Calling this once locks the target in — selecting a second is a
    /// compile error.
    #[must_use]
    pub fn commit_ish(self, commit_ish: impl Into<CommitIsh<'a>>) -> Reset<'a, WithTarget<'a>> {
        Reset {
            base: self.base,
            hard: self.hard,
            state: WithTarget {
                target: commit_ish.into(),
            },
            _phantom: PhantomData,
        }
    }
}

impl<'a, S> Reset<'a, S>
where
    Self: crate::Build,
{
    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl crate::Build for Reset<'_, NoTarget> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("reset")
            .optional_flag(self.hard, "--hard"))
    }
}

impl<'a> crate::Build for Reset<'a, WithTarget<'a>> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("reset")
            .optional_flag(self.hard, "--hard")
            .argument(self.state.target))
    }
}

#[cfg(feature = "test-utils")]
impl<'a> Reset<'a, NoTarget> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            hard: self.hard,
            state: NoTarget,
            _phantom: PhantomData,
        })
        .unwrap();
        command.test_eq(other);
    }
}

#[cfg(feature = "test-utils")]
impl<'a> Reset<'a, WithTarget<'a>> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            hard: self.hard,
            state: WithTarget {
                target: self.state.target,
            },
            _phantom: PhantomData,
        })
        .unwrap();
        command.test_eq(other);
    }
}
