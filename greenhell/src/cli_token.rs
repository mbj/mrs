//! CLI token discovery for GitHub API access.
//!
//! Discovers GitHub tokens using the same resolution order as the `gh` CLI:
//!
//! 1. `GH_TOKEN` environment variable
//! 2. `GITHUB_TOKEN` environment variable
//! 3. `gh auth token` command output
//!
//! This module is only for CLI/user context. For GitHub App authentication
//! (used in hosted/Lambda deployments), see the app authentication module.

use std::process::Command;

use nom::{IResult, Parser, bytes::complete::take_while1, combinator::verify, error::context};
use nom_language::error::VerboseError;

use crate::impl_from_str;
use crate::parse::Parse;

/// A validated GitHub token.
///
/// GitHub tokens come in several formats:
/// - `ghp_*` - Personal access tokens (fine-grained or classic)
/// - `gho_*` - OAuth access tokens
/// - `ghu_*` - GitHub App user-to-server tokens
/// - `ghs_*` - GitHub App installation tokens
/// - `ghr_*` - GitHub App refresh tokens
/// - 40 hex characters - Classic personal access tokens (legacy)
///
/// This type validates that the token is non-empty and contains only
/// valid token characters (alphanumeric and underscore).
#[derive(Clone)]
pub struct Token(String);

impl Token {
    /// Returns the token as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Token(***)")
    }
}

impl Parse for Token {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        context(
            "token: alphanumeric and underscore only",
            verify(
                take_while1(|character: char| {
                    character.is_ascii_alphanumeric() || character == '_'
                }),
                |string: &str| string.len() >= 4,
            ),
        )
        .map(|string: &str| Self(string.to_string()))
        .parse(remaining)
    }
}

impl_from_str!(Token);

/// The source from which a token was discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// Token from `GH_TOKEN` environment variable.
    GhToken,
    /// Token from `GITHUB_TOKEN` environment variable.
    GithubToken,
    /// Token from `gh auth token` command.
    GhAuthToken,
}

impl std::fmt::Display for Source {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GhToken => write!(formatter, "GH_TOKEN"),
            Self::GithubToken => write!(formatter, "GITHUB_TOKEN"),
            Self::GhAuthToken => write!(formatter, "gh auth token"),
        }
    }
}

/// Result of token discovery.
#[derive(Debug, Clone)]
pub struct Discovery {
    /// The discovered token.
    pub token: Token,
    /// Where the token was found.
    pub source: Source,
}

/// Error when no token could be discovered.
#[derive(Debug, Clone)]
pub struct NotFound {
    /// Error from `gh auth token` if it was attempted.
    pub gh_error: Option<String>,
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "No GitHub token found. Checked:\n  \
             1. GH_TOKEN env var (not set)\n  \
             2. GITHUB_TOKEN env var (not set)\n  \
             3. gh auth token"
        )?;

        if let Some(error) = &self.gh_error {
            write!(formatter, " (failed: {error})")?;
        } else {
            write!(formatter, " (not available)")?;
        }

        Ok(())
    }
}

impl std::error::Error for NotFound {}

/// Discovers a GitHub token using CLI resolution order.
///
/// Checks sources in order:
/// 1. `GH_TOKEN` environment variable
/// 2. `GITHUB_TOKEN` environment variable
/// 3. `gh auth token` command
///
/// Returns the first token found, along with its source.
///
/// # Errors
///
/// Returns `NotFound` if no token could be discovered from any source.
pub fn discover() -> Result<Discovery, NotFound> {
    if let Some(discovery) = try_env_var("GH_TOKEN", Source::GhToken) {
        return Ok(discovery);
    }

    if let Some(discovery) = try_env_var("GITHUB_TOKEN", Source::GithubToken) {
        return Ok(discovery);
    }

    match gh_auth_token() {
        Ok(token) => Ok(Discovery {
            token,
            source: Source::GhAuthToken,
        }),
        Err(error) => Err(NotFound {
            gh_error: Some(error),
        }),
    }
}

fn try_env_var(name: &str, source: Source) -> Option<Discovery> {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<Token>().ok())
        .map(|token| Discovery { token, source })
}

/// Executes `gh auth token` and returns the token.
fn gh_auth_token() -> Result<Token, String> {
    let output = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|error| format!("failed to execute gh: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.trim().to_string());
    }

    let token_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    token_str
        .parse::<Token>()
        .map_err(|error| format!("invalid token from gh: {error}"))
}
