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

/// Generate a `Cow<'static, str>` tuple newtype with validation, plus a
/// type-distinct error wrapper.
///
/// Invocation: `pub struct $name, $error($inner), "$invalid"`.
///
/// Generated:
/// - `$vis struct $name(Cow<'static, str>)` with `as_str` / `from_static_or_panic`
/// - `Display`, `AsRef<OsStr>`, `FromStr`, `TryFrom<String>`
/// - `serde::Serialize` and `serde::Deserialize` (validating via `try_from = "String"`)
/// - `$vis struct $error(pub $inner)` — a transparent `thiserror::Error`
///   wrapper, so distinct newtypes can share an inner validation enum
///   (e.g. `RefFormatError`) yet remain type-distinct
///
/// The user is expected to define
/// `impl $name { const fn validate(input: &str) -> Result<(), $error> { ... } }`
/// in the same module.
#[doc(hidden)]
#[macro_export]
macro_rules! cow_str_newtype {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident, $error:ident($inner:ty), $invalid:literal
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, Eq, PartialEq, ::serde::Deserialize)]
        #[serde(try_from = "String")]
        $vis struct $name(::std::borrow::Cow<'static, str>);

        #[doc = concat!("Error returned when parsing an invalid `", stringify!($name), "`.")]
        #[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
        #[error(transparent)]
        $vis struct $error(pub $inner);

        impl $name {
            /// Returns the inner string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Constructs from a static string, panicking if invalid.
            #[must_use]
            pub const fn from_static_or_panic(input: &'static str) -> Self {
                assert!(Self::validate(input).is_ok(), $invalid);
                Self(::std::borrow::Cow::Borrowed(input))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }

        impl ::std::convert::AsRef<::std::ffi::OsStr> for $name {
            fn as_ref(&self) -> &::std::ffi::OsStr {
                self.as_str().as_ref()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $error;

            fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
                Self::validate(input)?;
                Ok(Self(::std::borrow::Cow::Owned(input.to_string())))
            }
        }

        impl ::std::convert::TryFrom<String> for $name {
            type Error = $error;

            fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}

pub mod add;
pub mod branch;
pub mod checkout;
pub mod clean;
pub mod clone;
pub mod commit;
pub mod commit_id;
pub mod commit_ish;
pub mod config;
pub mod diff;
pub mod fetch;
pub mod init;
pub mod ls_remote;
pub mod push;
pub mod ref_format;
pub mod remote;
pub mod repository;
pub mod reset;
pub mod rev_list;
pub mod rev_parse;
pub mod show;
pub mod show_ref;
pub mod status;
pub mod tag;
pub mod worktree;

use std::path::Path;

pub use cmd_proc::CommandError;

/// Trait for git command builders that support porcelain output.
///
/// Provides the `porcelain` and `porcelain_if` methods to set `--porcelain`.
pub trait Porcelain: Sized {
    /// Conditionally enable porcelain output (`--porcelain`).
    fn porcelain_if(self, value: bool) -> Self;

    /// Enable porcelain output (`--porcelain`).
    fn porcelain(self) -> Self {
        self.porcelain_if(true)
    }
}

/// Generate inherent `porcelain` / `porcelain_if` methods and a `Porcelain` trait implementation.
///
/// The inherent methods set the `porcelain` field.
/// The trait implementation delegates to the inherent methods.
///
/// This ensures callers can use `.porcelain()` without importing the `Porcelain` trait,
/// while the trait remains available for generic bounds.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_porcelain {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            /// Give output in machine-parseable format.
            ///
            /// Corresponds to `--porcelain`.
            #[must_use]
            pub fn porcelain(self) -> Self {
                self.porcelain_if(true)
            }

            /// Conditionally enable porcelain output.
            #[must_use]
            pub fn porcelain_if(mut self, value: bool) -> Self {
                self.porcelain = value;
                self
            }
        }

        impl<'a> $crate::Porcelain for $ty<'a> {
            fn porcelain_if(self, value: bool) -> Self {
                self.porcelain_if(value)
            }
        }
    };
}

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
///
/// git_proc::fetch::new()
///     .all()
///     .build()
///     .stderr_null()
///     .status()
///     .await?;
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
