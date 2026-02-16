use std::path::Path;

/// Create a new `git diff` command builder.
#[must_use]
pub fn new() -> Diff<'static> {
    Diff::new()
}

/// Builder for `git diff` command.
///
/// See `git diff --help` for full documentation.
#[derive(Debug)]
pub struct Diff<'a> {
    repo_path: Option<&'a Path>,
    exit_code: bool,
}

crate::impl_repo_path!(Diff);

impl<'a> Diff<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            exit_code: false,
        }
    }

    /// Make the program exit with codes similar to diff(1).
    ///
    /// Exits with 1 if there were differences and 0 means no differences.
    ///
    /// Corresponds to `--exit-code`.
    #[must_use]
    pub fn exit_code(mut self) -> Self {
        self.exit_code = true;
        self
    }
}

impl Default for Diff<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Diff<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("diff")
            .optional_flag(self.exit_code, "--exit-code")
    }
}
