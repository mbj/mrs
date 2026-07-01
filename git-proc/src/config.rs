/// Create a new `git config` command builder for getting/setting a key.
#[must_use]
pub fn new(key: &str) -> Config<'_> {
    Config::new(key)
}

/// Create a `git config --list` command builder that lists all configuration.
#[must_use]
pub fn list() -> List<'static> {
    List::new()
}

/// Builder for `git config` command.
///
/// See `git config --help` for full documentation.
#[derive(Debug)]
pub struct Config<'a> {
    base: crate::RepoBase<'a>,
    key: &'a str,
    value: Option<&'a str>,
}

crate::impl_repo_base!(Config);

impl<'a> Config<'a> {
    #[must_use]
    fn new(key: &'a str) -> Self {
        Self {
            base: crate::RepoBase::default(),
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
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl crate::Build for Config<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("config")
            .argument(self.key)
            .optional_argument(self.value))
    }
}

#[cfg(feature = "test-utils")]
impl Config<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            key: self.key,
            value: self.value,
        })
        .unwrap();
        command.test_eq(other);
    }
}

/// Builder for `git config --list`.
///
/// See `git config --help` for full documentation.
#[derive(Debug)]
pub struct List<'a> {
    base: crate::RepoBase<'a>,
}

crate::impl_repo_base!(List);

impl<'a> List<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
        }
    }
}

impl Default for List<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for List<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self.base.command()?.argument("config").argument("--list"))
    }
}

#[cfg(feature = "test-utils")]
impl List<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self { base: self.base }).unwrap();
        command.test_eq(other);
    }
}

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use super::*;

    #[test]
    fn test_config_list_command() {
        let expected = cmd_proc::Command::new("git")
            .argument("config")
            .argument("--list");

        list().test_eq(&expected);
    }
}
