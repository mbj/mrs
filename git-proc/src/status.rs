use std::path::Path;

/// Create a new `git status` command builder.
#[must_use]
pub fn new() -> Status<'static> {
    Status::new()
}

/// Builder for `git status` command.
///
/// See `git status --help` for full documentation.
#[derive(Debug)]
pub struct Status<'a> {
    repo_path: Option<&'a Path>,
    porcelain: bool,
}

impl<'a> Status<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            porcelain: false,
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    crate::flag_methods! {
        /// Give output in machine-parseable format.
        ///
        /// Corresponds to `--porcelain`.
        pub fn porcelain / porcelain_if, porcelain, "Conditionally enable porcelain output."
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        self.build().stdout()
    }

    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("status")
            .optional_argument(self.porcelain.then_some("--porcelain"))
    }
}

impl Default for Status<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "test-utils")]
impl Status<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = Self {
            repo_path: self.repo_path,
            porcelain: self.porcelain,
        }
        .build();
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let output = Status::new().stdout().string().unwrap();
        // Just verify it runs without error
        let _ = output;
    }

    #[test]
    fn test_status_porcelain() {
        let output = Status::new().porcelain().stdout().string().unwrap();
        // Porcelain output is empty if repo is clean
        let _ = output;
    }
}
