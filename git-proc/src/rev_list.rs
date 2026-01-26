use std::path::Path;

use crate::CommandError;

/// Create a new `git rev-list` command builder.
#[must_use]
pub fn new() -> RevList<'static> {
    RevList::new()
}

/// Builder for `git rev-list` command.
///
/// See `git rev-list --help` for full documentation.
#[derive(Debug)]
pub struct RevList<'a> {
    repo_path: Option<&'a Path>,
    topo_order: bool,
    reverse: bool,
    max_count: Option<usize>,
    commits: Vec<&'a str>,
}

crate::impl_repo_path!(RevList);

impl<'a> RevList<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            topo_order: false,
            reverse: false,
            max_count: None,
            commits: Vec::new(),
        }
    }

    crate::flag_methods! {
        /// Show commits in topological order.
        ///
        /// Corresponds to `--topo-order`.
        pub fn topo_order / topo_order_if, topo_order, "Conditionally show commits in topological order."
    }

    crate::flag_methods! {
        /// Output commits in reverse order.
        ///
        /// Corresponds to `--reverse`.
        pub fn reverse / reverse_if, reverse, "Conditionally output commits in reverse order."
    }

    /// Limit the number of commits to output.
    ///
    /// Corresponds to `--max-count` or `-n`.
    #[must_use]
    pub fn max_count(mut self, count: usize) -> Self {
        self.max_count = Some(count);
        self
    }

    /// Add a commit or range to list.
    #[must_use]
    pub fn commit(mut self, commit: &'a str) -> Self {
        self.commits.push(commit);
        self
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

impl Default for RevList<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for RevList<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("rev-list")
            .optional_argument(self.topo_order.then_some("--topo-order"))
            .optional_argument(self.reverse.then_some("--reverse"))
            .optional_option("--max-count", self.max_count.map(|c| c.to_string()))
            .arguments(self.commits)
    }
}

#[cfg(feature = "test-utils")]
impl RevList<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            topo_order: self.topo_order,
            reverse: self.reverse,
            max_count: self.max_count,
            commits: self.commits.clone(),
        });
        command.test_eq(other);
    }
}
