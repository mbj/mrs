use std::ffi::OsStr;
use std::path::Path;

use crate::CommandError;

/// Create a new `git commit` command builder.
#[must_use]
pub fn new() -> Commit<'static> {
    Commit::new()
}

/// Builder for `git commit` command.
///
/// See `git commit --help` for full documentation.
#[derive(Debug)]
pub struct Commit<'a> {
    repo_path: Option<&'a Path>,
    message: Option<&'a str>,
    author: Option<&'a str>,
    date: Option<&'a str>,
    allow_empty: bool,
    allow_empty_message: bool,
    env_vars: Vec<(cmd_proc::EnvVariableName<'a>, &'a OsStr)>,
}

impl<'a> Commit<'a> {
    #[must_use]
    fn new() -> Self {
        Self {
            repo_path: None,
            message: None,
            author: None,
            date: None,
            allow_empty: false,
            allow_empty_message: false,
            env_vars: Vec::new(),
        }
    }

    /// Set the repository path (`-C <path>`).
    #[must_use]
    pub fn repo_path(mut self, path: &'a Path) -> Self {
        self.repo_path = Some(path);
        self
    }

    /// Set the commit message.
    ///
    /// Corresponds to `--message` or `-m`.
    #[must_use]
    pub fn message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    /// Set the commit author.
    ///
    /// Corresponds to `--author`. Format: `Name <email>`.
    #[must_use]
    pub fn author(mut self, author: &'a str) -> Self {
        self.author = Some(author);
        self
    }

    /// Set the author date.
    ///
    /// Corresponds to `--date`.
    #[must_use]
    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    crate::flag_methods! {
        /// Allow creating a commit with no changes.
        ///
        /// Corresponds to `--allow-empty`.
        pub fn allow_empty / allow_empty_if, allow_empty, "Conditionally allow creating a commit with no changes."
    }

    crate::flag_methods! {
        /// Allow creating a commit with an empty message.
        ///
        /// Corresponds to `--allow-empty-message`.
        pub fn allow_empty_message / allow_empty_message_if, allow_empty_message, "Conditionally allow creating a commit with an empty message."
    }

    /// Set an environment variable for the command.
    #[must_use]
    pub fn env(mut self, key: cmd_proc::EnvVariableName<'a>, value: &'a OsStr) -> Self {
        self.env_vars.push((key, value));
        self
    }

    /// Execute the command and return the exit status.
    pub fn status(self) -> Result<(), CommandError> {
        crate::Build::build(self).status()
    }
}

impl Default for Commit<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Build for Commit<'_> {
    fn build(self) -> cmd_proc::Command {
        crate::base_command(self.repo_path)
            .argument("commit")
            .optional_option("--message", self.message)
            .optional_option("--author", self.author)
            .optional_option("--date", self.date)
            .optional_argument(self.allow_empty.then_some("--allow-empty"))
            .optional_argument(self.allow_empty_message.then_some("--allow-empty-message"))
            .envs(self.env_vars)
    }
}

#[cfg(feature = "test-utils")]
impl Commit<'_> {
    /// Compare the built command with another command using debug representation.
    pub fn test_eq(&self, other: &cmd_proc::Command) {
        let command = crate::Build::build(Self {
            repo_path: self.repo_path,
            message: self.message,
            author: self.author,
            date: self.date,
            allow_empty: self.allow_empty,
            allow_empty_message: self.allow_empty_message,
            env_vars: self.env_vars.clone(),
        });
        command.test_eq(other);
    }
}
