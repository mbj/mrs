use std::path::Path;

/// Create a new `git show-ref` command builder.
#[must_use]
pub fn new() -> ShowRef<'static> {
    ShowRef::new()
}

/// Builder for `git show-ref` command.
///
/// See `git show-ref --help` for full documentation.
#[derive(Debug)]
pub struct ShowRef<'a> {
    repo_path: Option<&'a Path>,
    verify: bool,
    pattern: Option<&'a str>,
}

crate::impl_repo_path!(ShowRef);

impl<'a> ShowRef<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            verify: false,
            pattern: None,
        }
    }

    crate::flag_methods! {
        /// Enable strict reference checking.
        ///
        /// Corresponds to `--verify`. When used, requires an exact ref path.
        pub fn verify / verify_if, verify, "Conditionally enable strict reference checking."
    }

    /// Set the pattern to match references against.
    #[must_use]
    pub fn pattern(mut self, pattern: &'a str) -> Self {
        self.pattern = Some(pattern);
        self
    }
}

impl Default for ShowRef<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for ShowRef<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("show-ref")
            .optional_flag(self.verify, "--verify")
            .optional_argument(self.pattern)
    }
}

#[cfg(feature = "test-utils")]
impl ShowRef<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            verify: self.verify,
            pattern: self.pattern,
        });
        command.test_eq(other);
    }
}
