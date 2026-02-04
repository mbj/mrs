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

crate::impl_repo_path!(Status);
crate::impl_porcelain!(Status);

impl<'a> Status<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            porcelain: false,
        }
    }
}

impl Default for Status<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Status<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("status")
            .optional_flag(self.porcelain, "--porcelain")
    }
}

#[cfg(feature = "test-utils")]
impl Status<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            porcelain: self.porcelain,
        });
        command.test_eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Build;

    #[test]
    fn test_status() {
        let output = Status::new().build().capture_stdout().string().unwrap();
        // Just verify it runs without error
        let _ = output;
    }

    #[test]
    fn test_status_porcelain() {
        let output = Status::new()
            .porcelain()
            .build()
            .capture_stdout()
            .string()
            .unwrap();
        // Porcelain output is empty if repo is clean
        let _ = output;
    }
}
