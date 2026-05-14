//! `git checkout` command builder using a typestate to enforce that a target
//! is chosen before the command can be executed.
//!
//! # Example
//!
//! ```
//! use git_proc::{Build, branch::Branch, checkout};
//! let main = Branch::from_static_or_panic("main");
//! let _cmd = checkout::new().commit_ish(&main).build();
//! ```
//!
//! # Typestate guarantees
//!
//! Executing without a target is a compile error:
//!
//! ```compile_fail
//! use git_proc::{Build, checkout};
//! let _cmd = checkout::new().build();
//! ```
//!
//! Selecting more than one target is a compile error:
//!
//! ```compile_fail
//! use git_proc::{branch::Branch, checkout, tag::Tag};
//! let b = Branch::from_static_or_panic("main");
//! let t = Tag::from_static_or_panic("v1.0.0");
//! let _ = checkout::new().commit_ish(&b).commit_ish(&t);
//! ```

use std::marker::PhantomData;
use std::path::Path;

use crate::CommandError;
use crate::commit_ish::{CommitIsh, NoTarget, WithTarget};

/// Create a new `git checkout` command builder.
#[must_use]
pub fn new() -> Checkout<'static, NoTarget> {
    Checkout {
        repo_path: None,
        state: NoTarget,
        _phantom: PhantomData,
    }
}

/// Builder for `git checkout`.
///
/// `S` encodes whether a target has been chosen. Only
/// `Checkout<_, WithTarget>` exposes `status()` / `Build`, so failing to
/// specify a target is a compile error.
///
/// See `git checkout --help` for full documentation.
#[derive(Debug)]
pub struct Checkout<'a, S> {
    repo_path: Option<&'a Path>,
    state: S,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, S> Checkout<'a, S> {
    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }
}

impl<'a, S> crate::RepoPath<'a> for Checkout<'a, S> {
    fn repo_path(self, path: &'a Path) -> Self {
        self.repo_path(path)
    }
}

impl<'a> Checkout<'a, NoTarget> {
    /// Select the commit-ish to check out.
    #[must_use]
    pub fn commit_ish(self, commit_ish: impl Into<CommitIsh<'a>>) -> Checkout<'a, WithTarget<'a>> {
        Checkout {
            repo_path: self.repo_path,
            state: WithTarget {
                target: commit_ish.into(),
            },
            _phantom: PhantomData,
        }
    }
}

impl<'a> Checkout<'a, WithTarget<'a>> {
    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status().await
    }
}

impl<'a> crate::Build for Checkout<'a, WithTarget<'a>> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("checkout")
            .argument(self.state.target)
    }
}

#[cfg(feature = "test-utils")]
impl<'a> Checkout<'a, WithTarget<'a>> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            state: WithTarget {
                target: self.state.target,
            },
            _phantom: PhantomData,
        });
        command.test_eq(other);
    }
}
