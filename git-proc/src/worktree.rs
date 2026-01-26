use std::path::Path;

use crate::CommandError;

/// Create a new `git worktree list` command builder.
#[must_use]
pub fn list() -> List<'static> {
    List::new()
}

/// Create a new `git worktree add` command builder.
#[must_use]
pub fn add(path: &Path) -> Add<'_> {
    Add::new(path)
}

/// Create a new `git worktree remove` command builder.
#[must_use]
pub fn remove(worktree: &Path) -> Remove<'_> {
    Remove::new(worktree)
}

/// Builder for `git worktree list` command.
///
/// See `git worktree --help` for full documentation.
#[derive(Debug)]
pub struct List<'a> {
    repo_path: Option<&'a Path>,
}

crate::impl_repo_path!(List);

impl<'a> List<'a> {
    #[must_use]
    fn new() -> Self {
        Self { repo_path: None }
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        crate::Build::build(self).stdout()
    }
}

impl Default for List<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for List<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("worktree")
            .argument("list")
    }
}

#[cfg(feature = "test-utils")]
impl List<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
        });
        command.test_eq(other);
    }
}

/// Builder for `git worktree add` command.
///
/// See `git worktree --help` for full documentation.
#[derive(Debug)]
pub struct Add<'a> {
    repo_path: Option<&'a Path>,
    path: &'a Path,
    branch: Option<&'a str>,
    new_branch: Option<&'a str>,
    commit_ish: Option<&'a str>,
}

crate::impl_repo_path!(Add);

impl<'a> Add<'a> {
    #[must_use]
    fn new(path: &'a Path) -> Self {
        Self {
            repo_path: None,
            path,
            branch: None,
            new_branch: None,
            commit_ish: None,
        }
    }

    /// Checkout existing branch in the new worktree.
    #[must_use]
    pub fn branch(mut self, branch: &'a str) -> Self {
        self.branch = Some(branch);
        self
    }

    /// Create a new branch and checkout in the new worktree.
    ///
    /// Corresponds to `-b <branch>`.
    #[must_use]
    pub fn new_branch(mut self, branch: &'a str) -> Self {
        self.new_branch = Some(branch);
        self
    }

    /// Set the commit-ish to create the worktree from.
    #[must_use]
    pub fn commit_ish(mut self, commit_ish: &'a str) -> Self {
        self.commit_ish = Some(commit_ish);
        self
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }
}

impl crate::Build for Add<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("worktree")
            .argument("add")
            .optional_option("-b", self.new_branch)
            .argument(self.path)
            .optional_argument(self.branch)
            .optional_argument(self.commit_ish)
    }
}

#[cfg(feature = "test-utils")]
impl Add<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            path: self.path,
            branch: self.branch,
            new_branch: self.new_branch,
            commit_ish: self.commit_ish,
        });
        command.test_eq(other);
    }
}

/// Builder for `git worktree remove` command.
///
/// See `git worktree --help` for full documentation.
#[derive(Debug)]
pub struct Remove<'a> {
    repo_path: Option<&'a Path>,
    worktree: &'a Path,
    force: bool,
}

crate::impl_repo_path!(Remove);

impl<'a> Remove<'a> {
    #[must_use]
    fn new(worktree: &'a Path) -> Self {
        Self {
            repo_path: None,
            worktree,
            force: false,
        }
    }

    crate::flag_methods! {
        /// Force removal even if worktree is dirty.
        ///
        /// Corresponds to `--force`.
        pub fn force / force_if, force, "Conditionally force removal."
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }
}

impl crate::Build for Remove<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("worktree")
            .argument("remove")
            .optional_argument(self.force.then_some("--force"))
            .argument(self.worktree)
    }
}

#[cfg(feature = "test-utils")]
impl Remove<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            worktree: self.worktree,
            force: self.force,
        });
        command.test_eq(other);
    }
}
