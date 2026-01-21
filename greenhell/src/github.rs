mod api;
mod client;
pub mod git;

pub use api::*;
pub use client::{Client, Error};
pub use git_proc::branch::Branch;

use nom::{
    IResult, Parser, bytes::complete::take_while1, character::complete::char, combinator::verify,
    error::context,
};
use nom_language::error::VerboseError;

use crate::impl_from_str;
use crate::parse::Parse;

impl_from_str!(Repository, PullRequestNumber, Ref, Sha);

/// A GitHub repository identifier.
///
/// Format: `owner/repo`
///
/// Constraints (for both owner and repo):
/// - Minimum 1 character
/// - Maximum 100 characters
/// - No whitespace
///
/// Note: Exact constraints for owner and repo names are not explicitly documented
/// in official GitHub API documentation. The constraints above are a reasonable
/// approximation based on observed behavior.
///
/// # Examples
///
/// ```
/// use greenhell::github::Repository;
///
/// let repo: Repository = "mbj/mrs".parse().unwrap();
/// assert_eq!(repo.owner(), "mbj");
/// assert_eq!(repo.repo(), "mrs");
///
/// // Rejects empty owner
/// assert!("".parse::<Repository>().is_err());
///
/// // Rejects missing repo
/// assert!("owner".parse::<Repository>().is_err());
///
/// // Rejects empty repo
/// assert!("owner/".parse::<Repository>().is_err());
///
/// // Rejects whitespace
/// assert!("owner/repo name".parse::<Repository>().is_err());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repository {
    owner: String,
    repo: String,
}

impl Repository {
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    #[must_use]
    pub fn repo(&self) -> &str {
        &self.repo
    }
}

impl Parse for Repository {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        fn parse_segment<'a>(
            remaining: &'a str,
            context_name: &'static str,
        ) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
            context(
                context_name,
                verify(
                    take_while1(|character: char| !character.is_whitespace() && character != '/'),
                    |string: &str| string.len() <= 100,
                ),
            )
            .parse(remaining)
        }

        let (remaining, owner) = parse_segment(remaining, "owner: 1-100 chars, no whitespace")?;
        let (remaining, _) = context("separator: /", char('/')).parse(remaining)?;
        let (remaining, repo) = parse_segment(remaining, "repo: 1-100 chars, no whitespace")?;

        Ok((
            remaining,
            Self {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
        ))
    }
}

impl std::fmt::Display for Repository {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}/{}", self.owner, self.repo)
    }
}

/// A GitHub pull request number.
///
/// Constraints:
/// - Must be a valid positive integer
/// - Maximum 10 digits
///
/// # Examples
///
/// ```
/// use greenhell::github::PullRequestNumber;
///
/// let pr: PullRequestNumber = "123".parse().unwrap();
/// assert_eq!(pr.get(), 123);
///
/// let pr: PullRequestNumber = "1".parse().unwrap();
/// assert_eq!(pr.get(), 1);
///
/// // Rejects empty
/// assert!("".parse::<PullRequestNumber>().is_err());
///
/// // Rejects non-digits
/// assert!("abc".parse::<PullRequestNumber>().is_err());
/// assert!("123a".parse::<PullRequestNumber>().is_err());
///
/// // Rejects too long (>10 chars)
/// assert!("12345678901".parse::<PullRequestNumber>().is_err());
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PullRequestNumber(u64);

impl PullRequestNumber {
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for PullRequestNumber {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Parse for PullRequestNumber {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        context(
            "pull request number: 1-10 digits",
            verify(
                take_while1(|character: char| character.is_ascii_digit()),
                |string: &str| string.len() <= 10,
            ),
        )
        .map(|string: &str| Self(string.parse().unwrap()))
        .parse(remaining)
    }
}

impl std::fmt::Display for PullRequestNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A git ref (branch, tag, or SHA).
///
/// Used in GitHub API endpoints that accept a ref parameter.
///
/// Constraints:
/// - Minimum 1 character
/// - Maximum 256 characters
/// - No whitespace
///
/// # Examples
///
/// ```
/// use greenhell::github::Ref;
///
/// let git_ref: Ref = "main".parse().unwrap();
/// assert_eq!(git_ref.as_str(), "main");
/// assert!("".parse::<Ref>().is_err());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ref(String);

impl Ref {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parse for Ref {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        context(
            "ref: 1-256 chars, no whitespace",
            verify(
                take_while1(|character: char| !character.is_whitespace()),
                |string: &str| string.len() <= 256,
            ),
        )
        .map(|string: &str| Self(string.to_string()))
        .parse(remaining)
    }
}

impl std::fmt::Display for Ref {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<Branch> for Ref {
    fn from(branch: Branch) -> Self {
        Self(branch.as_str().to_string())
    }
}

impl From<&Sha> for Ref {
    fn from(sha: &Sha) -> Self {
        Self(sha.as_str().to_string())
    }
}

impl From<Sha> for Ref {
    fn from(sha: Sha) -> Self {
        Self(sha.as_str().to_string())
    }
}

/// A git commit SHA.
///
/// Constraints:
/// - Exactly 40 lowercase hexadecimal characters
///
/// # Examples
///
/// ```
/// use greenhell::github::Sha;
///
/// let sha: Sha = "abc123def456abc123def456abc123def456abc1".parse().unwrap();
/// assert_eq!(sha.as_str(), "abc123def456abc123def456abc123def456abc1");
///
/// // Rejects empty
/// assert!("".parse::<Sha>().is_err());
///
/// // Rejects short SHAs
/// assert!("abc123".parse::<Sha>().is_err());
///
/// // Rejects non-hex characters
/// assert!("ghijklmnopqrstuvwxyzghijklmnopqrstuvwxyz".parse::<Sha>().is_err());
/// ```
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Sha(String);

impl Sha {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn abbrev(&self) -> &str {
        &self.0[..7]
    }
}

impl Parse for Sha {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        context(
            "sha: exactly 40 lowercase hex characters",
            verify(take_while1(|c: char| c.is_ascii_hexdigit()), |s: &str| {
                s.len() == 40
            }),
        )
        .map(|s: &str| Self(s.to_ascii_lowercase()))
        .parse(remaining)
    }
}

impl std::fmt::Display for Sha {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
