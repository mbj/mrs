use std::path::Path;

use crate::CommandError;

/// Create a new `git rev-parse` command builder.
#[must_use]
pub fn new() -> RevParse<'static> {
    RevParse::new()
}

/// Builder for `git rev-parse` command.
///
/// See `git rev-parse --help` for full documentation.
#[derive(Debug)]
pub struct RevParse<'a> {
    repo_path: Option<&'a Path>,
    abbrev_ref: bool,
    symbolic_full_name: bool,
    rev: Option<&'a str>,
}

impl<'a> RevParse<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            abbrev_ref: false,
            symbolic_full_name: false,
            rev: None,
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    crate::flag_methods! {
        /// Output short ref name (e.g., `main` instead of `refs/heads/main`).
        ///
        /// Corresponds to `--abbrev-ref`.
        pub fn abbrev_ref / abbrev_ref_if, abbrev_ref, "Conditionally output short ref name."
    }

    crate::flag_methods! {
        /// Output full symbolic ref name.
        ///
        /// Corresponds to `--symbolic-full-name`.
        pub fn symbolic_full_name / symbolic_full_name_if, symbolic_full_name, "Conditionally output full symbolic ref name."
    }

    /// Set the revision to parse (e.g., `HEAD`, `@{u}`).
    #[must_use]
    pub fn rev(mut self, rev: &'a str) -> Self {
        self.rev = Some(rev);
        self
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        self.build().stdout()
    }

    /// Execute and return full output regardless of exit status.
    ///
    /// Use this when you need to inspect stderr on failure.
    pub fn output(self) -> Result<cmd_proc::Output, CommandError> {
        self.build().output()
    }

    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("rev-parse")
            .optional_argument(self.abbrev_ref.then_some("--abbrev-ref"))
            .optional_argument(self.symbolic_full_name.then_some("--symbolic-full-name"))
            .optional_argument(self.rev)
    }
}

impl Default for RevParse<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "test-utils")]
impl RevParse<'_> {
    /// Compare the built command with another command using debug representation.
    ///
    /// This is useful for testing command construction without executing.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        // Clone self to build without consuming
        let command = Self {
            repo_path: self.repo_path,
            abbrev_ref: self.abbrev_ref,
            symbolic_full_name: self.symbolic_full_name,
            rev: self.rev,
        }
        .build();
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_parse_head() {
        let output = RevParse::new().rev("HEAD").stdout().string().unwrap();
        assert!(!output.trim().is_empty());
    }

    #[test]
    fn test_rev_parse_abbrev_ref() {
        let output = RevParse::new()
            .abbrev_ref()
            .rev("HEAD")
            .stdout()
            .string()
            .unwrap();
        assert!(!output.trim().is_empty());
    }
}
