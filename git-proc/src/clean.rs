/// Create a new `git clean` command builder.
#[must_use]
pub fn new() -> Clean<'static> {
    Clean::new()
}

/// Builder for `git clean` command.
///
/// See `git clean --help` for full documentation.
#[derive(Debug)]
pub struct Clean<'a> {
    base: crate::RepoBase<'a>,
    directories: bool,
    force: bool,
    include_ignored: bool,
}

crate::impl_repo_base!(Clean);

impl<'a> Clean<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
            directories: false,
            force: false,
            include_ignored: false,
        }
    }

    crate::flag_methods! {
        /// Recurse into untracked directories.
        ///
        /// Corresponds to `-d` (no long form).
        pub fn directories / directories_if, directories, "Conditionally recurse into untracked directories."
    }

    crate::flag_methods! {
        /// Force removal even when `clean.requireForce` is set.
        ///
        /// Corresponds to `--force`.
        pub fn force / force_if, force, "Conditionally force removal."
    }

    crate::flag_methods! {
        /// Also remove files ignored by `.gitignore`.
        ///
        /// Corresponds to `-x` (no long form).
        pub fn include_ignored / include_ignored_if, include_ignored, "Conditionally also remove ignored files."
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl Default for Clean<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Clean<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("clean")
            .optional_flag(self.directories, "-d")
            .optional_flag(self.force, "--force")
            .optional_flag(self.include_ignored, "-x"))
    }
}

#[cfg(feature = "test-utils")]
impl Clean<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            directories: self.directories,
            force: self.force,
            include_ignored: self.include_ignored,
        })
        .unwrap();
        command.test_eq(other);
    }
}
