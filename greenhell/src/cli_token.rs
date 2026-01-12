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

/// Known GitHub token prefixes.
const TOKEN_PREFIXES: &[&str] = &[
    "ghp_", // Personal access tokens (fine-grained or classic)
    "gho_", // OAuth access tokens
    "ghu_", // GitHub App user-to-server tokens
    "ghs_", // GitHub App installation tokens
    "ghr_", // GitHub App refresh tokens
];

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
/// This type validates that the token matches one of these known formats.
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

/// Check if token has a known prefix (ghp_, ghs_, etc.) with valid suffix.
fn is_prefixed_token(string: &str) -> bool {
    TOKEN_PREFIXES.iter().any(|prefix| {
        string.starts_with(prefix)
            && string.len() > prefix.len()
            && string[prefix.len()..]
                .chars()
                .all(|c| c.is_ascii_alphanumeric())
    })
}

/// Check if token is a legacy 40-character hex token.
fn is_legacy_token(string: &str) -> bool {
    string.len() == 40 && string.chars().all(|c| c.is_ascii_hexdigit())
}

impl Parse for Token {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        context(
            "token: must be ghp_/gho_/ghu_/ghs_/ghr_ prefixed or 40-char hex",
            verify(
                take_while1(|character: char| {
                    character.is_ascii_alphanumeric() || character == '_'
                }),
                |string: &str| is_prefixed_token(string) || is_legacy_token(string),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ghp_prefixed_token() {
        let token: Token = "ghp_abcdefghij1234567890abcdefghij1234".parse().unwrap();
        assert_eq!(token.as_str(), "ghp_abcdefghij1234567890abcdefghij1234");
    }

    #[test]
    fn accepts_ghs_prefixed_token() {
        let token: Token = "ghs_installationToken12345678901234567".parse().unwrap();
        assert_eq!(token.as_str(), "ghs_installationToken12345678901234567");
    }

    #[test]
    fn accepts_gho_prefixed_token() {
        let token: Token = "gho_oauthToken123456789012345678901234".parse().unwrap();
        assert_eq!(token.as_str(), "gho_oauthToken123456789012345678901234");
    }

    #[test]
    fn accepts_ghu_prefixed_token() {
        let token: Token = "ghu_userToken1234567890123456789012345".parse().unwrap();
        assert_eq!(token.as_str(), "ghu_userToken1234567890123456789012345");
    }

    #[test]
    fn accepts_ghr_prefixed_token() {
        let token: Token = "ghr_refreshToken12345678901234567890123".parse().unwrap();
        assert_eq!(token.as_str(), "ghr_refreshToken12345678901234567890123");
    }

    #[test]
    fn accepts_legacy_40_char_hex_token() {
        let token: Token = "0123456789abcdef0123456789abcdef01234567".parse().unwrap();
        assert_eq!(
            token.as_str(),
            "0123456789abcdef0123456789abcdef01234567"
        );
    }

    #[test]
    fn rejects_unknown_prefix() {
        let result = "xyz_sometoken123456789012345678901234".parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_prefix_only() {
        let result = "ghp_".parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_short_hex_token() {
        let result = "0123456789abcdef".parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_long_hex_token() {
        let result = "0123456789abcdef0123456789abcdef012345678".parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_underscore_in_suffix() {
        let result = "ghp_token_with_underscores".parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_token() {
        let result = "".parse::<Token>();
        assert!(result.is_err());
    }
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
