#![doc = include_str!("../README.md")]

/// Generate a pair of flag methods: unconditional and conditional.
///
/// The unconditional method calls the conditional one with `true`.
///
/// # Example
///
/// ```ignore
/// flag_methods! {
///     /// Enable foo mode.
///     ///
///     /// Corresponds to `--foo`.
///     pub fn foo / foo_if, foo_field, "Conditionally enable foo mode."
/// }
/// ```
///
/// Generates:
/// - `pub fn foo(self) -> Self` - calls `foo_if(true)`
/// - `pub fn foo_if(mut self, value: bool) -> Self` - sets `self.foo_field = value`
#[doc(hidden)]
#[macro_export]
macro_rules! flag_methods {
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident / $name_if:ident, $field:ident, $doc_if:literal
    ) => {
        $(#[$attr])*
        #[must_use]
        $vis fn $name(self) -> Self {
            self.$name_if(true)
        }

        #[doc = $doc_if]
        #[must_use]
        $vis fn $name_if(mut self, value: bool) -> Self {
            self.$field = value;
            self
        }
    };
}

/// Generate an inherent `repo_path` method and a `RepoPath` trait implementation.
///
/// The inherent method sets the `repo_path` field to `Some(path)`.
/// The trait implementation delegates to the inherent method.
///
/// This ensures callers can use `.repo_path()` without importing the `RepoPath` trait,
/// while the trait remains available for generic bounds.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_repo_path {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            /// Set the repository path (`-C <path>`).
            #[must_use]
            pub fn repo_path(mut self, path: &'a std::path::Path) -> Self {
                self.repo_path = Some(path);
                self
            }
        }

        impl<'a> $crate::RepoPath<'a> for $ty<'a> {
            fn repo_path(self, path: &'a std::path::Path) -> Self {
                self.repo_path(path)
            }
        }
    };
}

pub mod add;
pub mod branch;
pub mod clone;
pub mod commit;
pub mod config;
pub mod fetch;
pub mod init;
pub mod ls_remote;
pub mod push;
pub mod remote;
pub mod rev_list;
pub mod rev_parse;
pub mod show;
pub mod show_ref;
pub mod status;
pub mod url;
pub mod worktree;

use std::path::Path;

pub use cmd_proc::CommandError;

/// Trait for git command builders that support a repository path.
///
/// Provides the `repo_path` method to set `-C <path>`.
#[must_use = "repo_path returns a modified builder; the return value must be used"]
pub trait RepoPath<'a>: Sized {
    /// Set the repository path (`-C <path>`).
    fn repo_path(self, path: &'a Path) -> Self;
}

/// Trait for building a command without executing it.
///
/// All git command builders implement this trait, allowing you to
/// access the underlying `cmd_proc::Command` for custom execution.
///
/// # Example
///
/// ```ignore
/// use git_proc::Build;
/// use cmd_proc::Stdio;
///
/// git_proc::fetch::new()
///     .all()
///     .build()
///     .spawn()
///     .stderr(Stdio::Null)
///     .run()?;
/// ```
pub trait Build {
    /// Build the command without executing it.
    fn build(self) -> cmd_proc::Command;
}

/// Create a command builder with optional repository path.
///
/// If `repo_path` is `Some`, adds `-C <path>` to the command.
/// If `repo_path` is `None`, uses current working directory.
fn base_command(repo_path: Option<&Path>) -> cmd_proc::Command {
    cmd_proc::Command::new("git").optional_option("-C", repo_path)
}
