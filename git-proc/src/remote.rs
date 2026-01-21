use std::path::Path;

use crate::CommandError;
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

impl<'a> Remote<'a> {
    #[must_use]
    fn get_url(name: &'a RemoteName) -> Self {
        Self {
            repo_path: None,
            subcommand: RemoteSubcommand::GetUrl { name },
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout(self) -> cmd_proc::Capture {
        self.build().stdout()
    }

    /// Execute and return full output regardless of exit status.
    ///
    /// Use this when you need to inspect stderr on failure.
    pub fn output(self) -> Result<cmd_proc::Output, CommandError> {
        self.build().output()
    }

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
        let command = Self {
            repo_path: self.repo_path,
            subcommand: match &self.subcommand {
                RemoteSubcommand::GetUrl { name } => RemoteSubcommand::GetUrl { name },
            },
        }
        .build();
        command.test_eq(other);
    }
}
