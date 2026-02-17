#![doc = include_str!("../README.md")]

mod base;
pub mod commands;
mod config;
mod detect;
mod git;
mod repo_name;

pub use base::{Base, BaseError};
pub use config::{Config, Error as ConfigError, Source as ConfigSource};
pub use detect::{DetectError, detect_repo_from_cwd};
pub use git_proc::CommandError;
pub use git_proc::branch::{Branch, BranchError};
pub use git_proc::repository::{self, Remote, RemoteName};
pub use repo_name::{RepoName, RepoNameError};

use std::path::PathBuf;

pub const ORIGIN: Remote = Remote::Name(RemoteName::from_static_or_panic("origin"));

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Repository not found: {0}")]
    RepoNotFound(RepoName),

    #[error("Repository already exists: {0}")]
    RepoAlreadyExists(RepoName),

    #[error("{0}")]
    Detect(#[from] DetectError),

    #[error("Cannot determine default branch from remote")]
    DefaultBranchNotFound,

    #[error("Worktree already exists: {}", .0.display())]
    WorktreeExists(PathBuf),

    #[error("Worktree not found: {}", .0.display())]
    WorktreeNotFound(PathBuf),

    #[error("Command failed: {0}")]
    Command(#[from] CommandError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid repository name: {0}")]
    RepoName(#[from] RepoNameError),
}
