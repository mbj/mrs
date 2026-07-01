use crate::repository::Remote;

/// Create a new `git push` command builder.
#[must_use]
pub fn new() -> Push<'static> {
    Push::new()
}

/// Builder for `git push` command.
///
/// See `git push --help` for full documentation.
#[derive(Debug)]
pub struct Push<'a> {
    base: crate::RepoBase<'a>,
    force: bool,
    porcelain: bool,
    remote: Option<&'a Remote>,
    refspec: Option<&'a str>,
}

crate::impl_repo_base!(Push);
crate::impl_porcelain!(Push);

impl<'a> Push<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
            force: false,
            porcelain: false,
            remote: None,
            refspec: None,
        }
    }

    crate::flag_methods! {
        /// Force push (overwrite remote refs).
        ///
        /// Corresponds to `--force`.
        pub fn force / force_if, force, "Conditionally force push."
    }

    /// Set the remote to push to.
    #[must_use]
    pub fn remote(mut self, remote: &'a Remote) -> Self {
        self.remote = Some(remote);
        self
    }

    /// Set the refspec to push.
    #[must_use]
    pub fn refspec(mut self, refspec: &'a str) -> Self {
        self.refspec = Some(refspec);
        self
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl Default for Push<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Push<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("push")
            .optional_flag(self.force, "--force")
            .optional_flag(self.porcelain, "--porcelain")
            .optional_argument(self.remote)
            .optional_argument(self.refspec))
    }
}

#[cfg(feature = "test-utils")]
impl Push<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            force: self.force,
            porcelain: self.porcelain,
            remote: self.remote,
            refspec: self.refspec,
        })
        .unwrap();
        command.test_eq(other);
    }
}
