/// Create a new `git symbolic-ref <name>` command builder.
#[must_use]
pub fn new(name: &str) -> SymbolicRef<'_> {
    SymbolicRef::new(name)
}

/// Builder for `git symbolic-ref` command.
///
/// Reads (or, with a target, sets) the symbolic ref `name` — e.g.
/// `refs/remotes/origin/HEAD`. See `git symbolic-ref --help` for full
/// documentation.
#[derive(Debug)]
pub struct SymbolicRef<'a> {
    base: crate::RepoBase<'a>,
    name: &'a str,
}

crate::impl_repo_base!(SymbolicRef);

impl<'a> SymbolicRef<'a> {
    #[must_use]
    fn new(name: &'a str) -> Self {
        Self {
            base: crate::RepoBase::default(),
            name,
        }
    }
}

impl crate::Build for SymbolicRef<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("symbolic-ref")
            .argument(self.name))
    }
}

#[cfg(feature = "test-utils")]
impl SymbolicRef<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            name: self.name,
        })
        .unwrap();
        command.test_eq(other);
    }
}

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use super::*;

    #[test]
    fn test_symbolic_ref_command() {
        let expected = cmd_proc::Command::new("git")
            .argument("symbolic-ref")
            .argument("refs/remotes/origin/HEAD");

        new("refs/remotes/origin/HEAD").test_eq(&expected);
    }
}
