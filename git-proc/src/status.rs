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
    base: crate::RepoBase<'a>,
    porcelain: bool,
}

crate::impl_repo_base!(Status);
crate::impl_porcelain!(Status);

impl<'a> Status<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
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
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("status")
            .optional_flag(self.porcelain, "--porcelain"))
    }
}

#[cfg(feature = "test-utils")]
impl Status<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            porcelain: self.porcelain,
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
    async fn test_status() {
        let output = Status::new()
            .build()
            .unwrap()
            .stdout_capture()
            .string()
            .await
            .unwrap();
        // Just verify it runs without error
        let _ = output;
    }

    #[tokio::test]
    async fn test_status_porcelain() {
        let output = Status::new()
            .porcelain()
            .build()
            .unwrap()
            .stdout_capture()
            .string()
            .await
            .unwrap();
        // Porcelain output is empty if repo is clean
        let _ = output;
    }
}
