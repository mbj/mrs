use std::path::Path;

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

crate::impl_repo_path!(RevParse);

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
}

impl Default for RevParse<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for RevParse<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("rev-parse")
            .optional_flag(self.abbrev_ref, "--abbrev-ref")
            .optional_flag(self.symbolic_full_name, "--symbolic-full-name")
            .optional_argument(self.rev)
    }
}

#[cfg(feature = "test-utils")]
impl RevParse<'_> {
    /// Compare the built command with another command using debug representation.
    ///
    /// This is useful for testing command construction without executing.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            abbrev_ref: self.abbrev_ref,
            symbolic_full_name: self.symbolic_full_name,
            rev: self.rev,
        });
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Build;

    #[test]
    fn test_rev_parse_head() {
        let output = RevParse::new()
            .rev("HEAD")
            .build()
            .capture_stdout()
            .string()
            .unwrap();
        assert!(!output.trim().is_empty());
    }

    #[test]
    fn test_rev_parse_abbrev_ref() {
        let output = RevParse::new()
            .abbrev_ref()
            .rev("HEAD")
            .build()
            .capture_stdout()
            .string()
            .unwrap();
        assert!(!output.trim().is_empty());
    }
}
