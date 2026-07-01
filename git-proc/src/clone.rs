use std::path::Path;

use crate::repository::Address;

/// Create a new `git clone` command builder.
#[must_use]
pub fn new(address: &Address) -> Clone<'_> {
    Clone::new(address)
}

/// Builder for `git clone` command.
///
/// See `git clone --help` for full documentation.
#[derive(Debug)]
pub struct Clone<'a> {
    base: crate::RepoBase<'a>,
    address: &'a Address,
    directory: Option<&'a Path>,
    bare: bool,
}

crate::impl_env_policy!(Clone);

impl<'a> Clone<'a> {
    #[must_use]
    fn new(address: &'a Address) -> Self {
        Self {
            base: crate::RepoBase::default(),
            address,
            directory: None,
            bare: false,
        }
    }

    /// Set the destination directory.
    #[must_use]
    pub fn directory(mut self, path: &'a Path) -> Self {
        self.directory = Some(path);
        self
    }

    crate::flag_methods! {
        /// Make a bare clone.
        ///
        /// Corresponds to `--bare`.
        pub fn bare / bare_if, bare, "Conditionally make a bare clone."
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl crate::Build for Clone<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("clone")
            .optional_flag(self.bare, "--bare")
            .argument(self.address)
            .optional_argument(self.directory))
    }
}

#[cfg(feature = "test-utils")]
impl Clone<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            address: self.address,
            directory: self.directory,
            bare: self.bare,
        })
        .unwrap();
        command.test_eq(other);
    }
}
