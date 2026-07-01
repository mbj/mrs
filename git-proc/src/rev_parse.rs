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
    base: crate::RepoBase<'a>,
    abbrev_ref: bool,
    symbolic_full_name: bool,
    verify: bool,
    quiet: bool,
    rev: Option<&'a str>,
}

crate::impl_repo_base!(RevParse);

impl<'a> RevParse<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
            abbrev_ref: false,
            symbolic_full_name: false,
            verify: false,
            quiet: false,
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

    crate::flag_methods! {
        /// Verify that the revision resolves, failing (non-zero exit) if it does not.
        ///
        /// Corresponds to `--verify`.
        pub fn verify / verify_if, verify, "Conditionally verify that the revision resolves."
    }

    crate::flag_methods! {
        /// Suppress error output; combined with `--verify`, signals a missing
        /// revision through the exit status alone.
        ///
        /// Corresponds to `--quiet`.
        pub fn quiet / quiet_if, quiet, "Conditionally suppress error output."
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
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("rev-parse")
            .optional_flag(self.abbrev_ref, "--abbrev-ref")
            .optional_flag(self.symbolic_full_name, "--symbolic-full-name")
            .optional_flag(self.verify, "--verify")
            .optional_flag(self.quiet, "--quiet")
            .optional_argument(self.rev))
    }
}

#[cfg(feature = "test-utils")]
impl RevParse<'_> {
    /// Compare the built command with another command using debug representation.
    ///
    /// This is useful for testing command construction without executing.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            abbrev_ref: self.abbrev_ref,
            symbolic_full_name: self.symbolic_full_name,
            verify: self.verify,
            quiet: self.quiet,
            rev: self.rev,
        })
        .unwrap();
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Build;

    #[tokio::test]
    async fn test_rev_parse_head() {
        let output = RevParse::new()
            .rev("HEAD")
            .build()
            .unwrap()
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert!(!output.trim().is_empty());
    }

    #[tokio::test]
    async fn test_rev_parse_abbrev_ref() {
        let output = RevParse::new()
            .abbrev_ref()
            .rev("HEAD")
            .build()
            .unwrap()
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert!(!output.trim().is_empty());
    }

    #[cfg(feature = "test-utils")]
    #[test]
    fn test_rev_parse_verify_quiet_command() {
        let expected = cmd_proc::Command::new("git")
            .argument("rev-parse")
            .argument("--verify")
            .argument("--quiet")
            .argument("refs/remotes/origin/HEAD");

        RevParse::new()
            .verify()
            .quiet()
            .rev("refs/remotes/origin/HEAD")
            .test_eq(&expected);
    }
}
