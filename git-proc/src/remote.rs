use crate::repository::RemoteName;

/// Create a `git remote get-url` command builder.
#[must_use]
pub fn get_url(name: &RemoteName) -> Remote<'_> {
    Remote::get_url(name)
}

/// Builder for `git remote` command.
///
/// See `git remote --help` for full documentation.
#[derive(Debug)]
pub struct Remote<'a> {
    base: crate::RepoBase<'a>,
    subcommand: RemoteSubcommand<'a>,
}

#[derive(Debug)]
enum RemoteSubcommand<'a> {
    GetUrl { name: &'a RemoteName },
}

crate::impl_repo_base!(Remote);

impl<'a> Remote<'a> {
    #[must_use]
    fn get_url(name: &'a RemoteName) -> Self {
        Self {
            base: crate::RepoBase::default(),
            subcommand: RemoteSubcommand::GetUrl { name },
        }
    }
}

impl crate::Build for Remote<'_> {
    fn build(self) -> Result<cmd_proc::Command, crate::EnvError> {
        let RemoteSubcommand::GetUrl { name } = self.subcommand;

        Ok(self
            .base
            .command()?
            .argument("remote")
            .argument("get-url")
            .argument(name))
    }
}

#[cfg(feature = "test-utils")]
impl Remote<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            base: self.base,
            subcommand: match &self.subcommand {
                RemoteSubcommand::GetUrl { name } => RemoteSubcommand::GetUrl { name },
            },
        })
        .unwrap();
        command.test_eq(other);
    }
}
