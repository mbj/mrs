use std::path::Path;

use crate::url::RemoteName;

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
    repo_path: Option<&'a Path>,
    subcommand: RemoteSubcommand<'a>,
}

#[derive(Debug)]
enum RemoteSubcommand<'a> {
    GetUrl { name: &'a RemoteName },
}

crate::impl_repo_path!(Remote);

impl<'a> Remote<'a> {
    #[must_use]
    fn get_url(name: &'a RemoteName) -> Self {
        Self {
            repo_path: None,
            subcommand: RemoteSubcommand::GetUrl { name },
        }
    }
}

impl crate::Build for Remote<'_> {
    fn build(self) -> cmd_proc::Command {
        let RemoteSubcommand::GetUrl { name } = self.subcommand;

        crate::base_command(self.repo_path)
            .argument("remote")
            .argument("get-url")
            .argument(name)
    }
}

#[cfg(feature = "test-utils")]
impl Remote<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            subcommand: match &self.subcommand {
                RemoteSubcommand::GetUrl { name } => RemoteSubcommand::GetUrl { name },
            },
        });
        command.test_eq(other);
    }
}
