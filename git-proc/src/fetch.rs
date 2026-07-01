use crate::repository::Remote;

/// Create a new `git fetch` command builder.
#[must_use]
pub fn new() -> Fetch<'static> {
    Fetch::new()
}

/// Builder for `git fetch` command.
///
/// See `git fetch --help` for full documentation.
#[derive(Debug)]
pub struct Fetch<'a> {
    base: crate::RepoBase<'a>,
    all: bool,
    porcelain: bool,
    remote: Option<&'a Remote>,
}

crate::impl_repo_base!(Fetch);
crate::impl_porcelain!(Fetch);

impl<'a> Fetch<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            base: crate::RepoBase::default(),
            all: false,
            porcelain: false,
            remote: None,
        }
    }

    crate::flag_methods! {
        /// Fetch all remotes.
        ///
        /// Corresponds to `--all`.
        pub fn all / all_if, all, "Conditionally fetch all remotes."
    }

    /// Set the remote to fetch from.
    #[must_use]
    pub fn remote(mut self, remote: &'a Remote) -> Self {
        self.remote = Some(remote);
        self
    }

    /// Execute the command and return the exit status.
    pub async fn status(self) -> Result<(), crate::Error> {
        Ok(crate::Build::build(self)?.status().await?)
    }
}

impl Default for Fetch<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Fetch<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        Ok(self
            .base
            .command()?
            .argument("fetch")
            .optional_flag(self.all, "--all")
            .optional_flag(self.porcelain, "--porcelain")
            .optional_argument(self.remote))
    }
}

#[cfg(feature = "test-utils")]
impl Fetch<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            all: self.all,
            porcelain: self.porcelain,
            remote: self.remote,
        })
        .unwrap();
        command.test_eq(other);
    }
}
