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

/// Generate inherent `repo_path` / `env_policy` methods and a `RepoPath` trait
/// implementation for a builder that stores its repository context in a
/// `base: RepoBase<'a>` field.
///
/// `repo_path` sets `-C <path>`; `env_policy` selects the [`EnvPolicy`] applied
/// before spawning `git`. The inherent methods let callers use `.repo_path()` /
/// `.env_policy()` without importing a trait, while the `RepoPath` trait remains
/// available for generic bounds.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_repo_base {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            /// Set the repository path (`-C <path>`).
            #[must_use]
            pub fn repo_path(mut self, path: &'a std::path::Path) -> Self {
                self.base.repo_path = Some(path);
                self
            }

            /// Set the [`EnvPolicy`](crate::EnvPolicy) controlling which inherited
            /// `GIT_*` variables are cleared before spawning `git`.
            #[must_use]
            pub fn env_policy(mut self, policy: $crate::EnvPolicy) -> Self {
                self.base.env_policy = policy;
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

/// Generate an inherent `env_policy` method for a builder that stores a
/// `base: RepoBase<'a>` but takes no `-C <path>` (e.g. `init`, `clone`, which
/// create rather than operate on a repository yet still must not be hijacked by
/// an ambient git environment).
#[doc(hidden)]
#[macro_export]
macro_rules! impl_env_policy {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            /// Set the [`EnvPolicy`](crate::EnvPolicy) controlling which inherited
            /// `GIT_*` variables are cleared before spawning `git`.
            #[must_use]
            pub fn env_policy(mut self, policy: $crate::EnvPolicy) -> Self {
                self.base.env_policy = policy;
                self
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
pub mod symbolic_ref;
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
    ///
    /// # Errors
    ///
    /// Returns [`EnvError`] if git's repository-local environment variables
    /// could not be resolved (see [`EnvPolicy::ClearLocal`]).
    fn build(self) -> Result<cmd_proc::Command, EnvError>;
}

/// Which inherited `GIT_*` environment variables a command clears before
/// spawning `git`.
///
/// Composite operations (`git commit`, `git rebase`) and hooks export `GIT_*`
/// variables to pin child git processes to *their* repository and index. A
/// `git_proc` command spawned in that context — a hook, a `git rebase -x`, or a
/// test launched from one — would otherwise be hijacked onto the ambient
/// repository instead of the one named by `-C` or the positional directory; most
/// dangerously `GIT_INDEX_FILE`, which no command-line flag can override.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EnvPolicy {
    /// Clear git's repository-local variables — `GIT_DIR`, `GIT_INDEX_FILE`,
    /// `GIT_WORK_TREE`, and the rest of `git rev-parse --local-env-vars`, the
    /// exact set `git` clears when entering another repository. Fixes the hijack
    /// while leaving global config/auth (`GIT_SSH_COMMAND`, `GIT_ASKPASS`,
    /// `GIT_CONFIG_GLOBAL`) intact. The default.
    #[default]
    ClearLocal,
    /// Clear every inherited `GIT_*` variable. Maximally hermetic, but also drops
    /// env-based auth/config.
    ClearAll,
    /// Inherit the ambient git environment unchanged.
    Inherit,
}

/// Failure resolving git's repository-local environment variables via
/// `git rev-parse --local-env-vars`.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EnvError {
    #[error("could not run `git rev-parse --local-env-vars`: {0:?}")]
    Spawn(std::io::ErrorKind),
    #[error("`git rev-parse --local-env-vars` exited unsuccessfully")]
    ExitStatus,
    #[error("`git rev-parse --local-env-vars` reported an invalid variable name: {0}")]
    InvalidName(String),
}

/// Failure running a `git_proc` command: either resolving the git environment
/// ([`EnvError`]) or executing the process ([`cmd_proc::CommandError`]).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The git environment could not be resolved.
    #[error(transparent)]
    Env(#[from] EnvError),
    /// The git process failed.
    #[error(transparent)]
    Command(#[from] cmd_proc::CommandError),
}

/// Git's repository-local environment variable names, queried once from
/// `git rev-parse --local-env-vars` so the set always matches the installed git.
/// The query is run lazily, on the first command built under
/// [`EnvPolicy::ClearLocal`] that actually inherits a `GIT_*` variable.
static LOCAL_GIT_ENV: std::sync::LazyLock<Result<Vec<cmd_proc::EnvVariableName>, EnvError>> =
    std::sync::LazyLock::new(|| {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--local-env-vars"])
            .output()
            .map_err(|error| EnvError::Spawn(error.kind()))?;

        if !output.status.success() {
            return Err(EnvError::ExitStatus);
        }

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.parse::<cmd_proc::EnvVariableName>()
                    .map_err(|error| EnvError::InvalidName(error.to_string()))
            })
            .collect()
    });

/// Create a command builder with optional repository path under an explicit
/// [`EnvPolicy`].
///
/// # Errors
///
/// Returns [`EnvError`] when the policy is [`EnvPolicy::ClearLocal`], an ambient
/// `GIT_*` variable is present, and git's local-env-var set could not be
/// resolved.
fn base_command_with_policy(
    repo_path: Option<&Path>,
    policy: EnvPolicy,
) -> Result<cmd_proc::Command, EnvError> {
    let mut command = cmd_proc::Command::new("git");

    for (name, _) in std::env::vars_os() {
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with("GIT_") {
            continue;
        }
        let Ok(env_name) = name.parse::<cmd_proc::EnvVariableName>() else {
            continue;
        };

        let clear = match policy {
            EnvPolicy::ClearAll => true,
            EnvPolicy::Inherit => false,
            EnvPolicy::ClearLocal => LOCAL_GIT_ENV
                .as_ref()
                .map_err(Clone::clone)?
                .contains(&env_name),
        };

        if clear {
            command = command.env_remove(&env_name);
        }
    }

    Ok(command.optional_option("-C", repo_path))
}

/// Repository context threaded through every command builder: the optional
/// `-C <path>` target and the [`EnvPolicy`] applied before spawning `git`.
///
/// Builders hold this as a `base` field (set via the `impl_repo_base!` /
/// `impl_env_policy!` macros) and call [`RepoBase::command`] to begin their
/// `git` invocation. Defaults to no `-C` and [`EnvPolicy::ClearLocal`].
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RepoBase<'a> {
    pub(crate) repo_path: Option<&'a Path>,
    pub(crate) env_policy: EnvPolicy,
}

impl RepoBase<'_> {
    /// Begin a `git` command: clear the inherited environment per the configured
    /// [`EnvPolicy`], then apply the optional `-C <path>`.
    ///
    /// # Errors
    ///
    /// See [`base_command_with_policy`].
    pub(crate) fn command(self) -> Result<cmd_proc::Command, EnvError> {
        base_command_with_policy(self.repo_path, self.env_policy)
    }
}
