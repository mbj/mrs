use std::path::Path;

use crate::CommandError;

/// Create a new `git config` command builder for getting/setting a key.
#[must_use]
pub fn new(key: &str) -> Config<'_> {
    Config::new(key)
}

/// Builder for `git config` command.
///
/// See `git config --help` for full documentation.
#[derive(Debug)]
pub struct Config<'a> {
    repo_path: Option<&'a Path>,
    key: &'a str,
    value: Option<&'a str>,
}

crate::impl_repo_path!(Config);

impl<'a> Config<'a> {
    #[must_use]
    fn new(key: &'a str) -> Self {
        Self {
            repo_path: None,
            key,
            value: None,
        }
    }

    /// Set the value for the configuration key.
    #[must_use]
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }
}

impl crate::Build for Config<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("config")
            .argument(self.key)
            .optional_argument(self.value)
    }
}

#[cfg(feature = "test-utils")]
impl Config<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            key: self.key,
            value: self.value,
        });
        command.test_eq(other);
    }
}
