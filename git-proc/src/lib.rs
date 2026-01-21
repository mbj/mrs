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

/// Create a command builder with optional repository path.
///
/// If `repo_path` is `Some`, adds `-C <path>` to the command.
/// If `repo_path` is `None`, uses current working directory.
fn base_command(repo_path: Option<&Path>) -> cmd_proc::Command {
    cmd_proc::Command::new("git").optional_option("-C", repo_path)
}
