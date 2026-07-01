/// Create a new `git show` command builder.
///
/// The object can be a commit, tree, blob, or tag reference.
/// For file contents at a specific revision, use format: `revision:path`
#[must_use]
pub fn new(object: &str) -> Show<'_> {
    Show::new(object)
}

/// Builder for `git show` command.
///
/// See `git show --help` for full documentation.
#[derive(Debug)]
pub struct Show<'a> {
    base: crate::RepoBase<'a>,
    object: &'a str,
}

crate::impl_repo_base!(Show);

impl<'a> Show<'a> {
    #[must_use]
    fn new(object: &'a str) -> Self {
        Self {
            base: crate::RepoBase::default(),
            object,
        }
    }
}

impl crate::Build for Show<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self.base.command()?.argument("show").argument(self.object))
    }
}

#[cfg(feature = "test-utils")]
impl Show<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            object: self.object,
        })
        .unwrap();
        command.test_eq(other);
    }
}
