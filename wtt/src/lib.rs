#![doc = include_str!("../README.md")]

mod base;
pub mod commands;
mod config;
mod detect;
mod git;
mod repo_name;

pub use base::{Base, BaseError};
pub use config::Config;
pub use detect::detect_repo_from_cwd;
pub use ociman::command::{Command, CommandError};
pub use repo_name::{RepoName, RepoNameError};

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Repository not found: {0}")]
    RepoNotFound(RepoName),

    #[error("Repository already exists: {0}")]
    RepoAlreadyExists(RepoName),

    #[error("Cannot detect repository from current directory")]
    RepoDetectionFailed,

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
}
