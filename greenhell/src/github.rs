mod api;
mod client;

pub use api::*;
pub use client::{Client, Error};

use nom::{
    IResult, Parser, bytes::complete::take_while1, character::complete::char, combinator::verify,
    error::context, multi::separated_list1,
};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::impl_from_str;
use crate::parse::Parse;

impl_from_str!(Repository, Branch, PullRequestNumber, Ref);

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

/// A git branch name.
///
/// Format: `component[/component]*`
///
/// Follows git-check-ref-format rules:
/// <https://git-scm.com/docs/git-check-ref-format>
///
/// Per-component constraints:
/// - Not empty
/// - Cannot start with `.`
/// - Cannot end with `.lock`
/// - Cannot contain `..`
/// - Cannot contain ASCII control characters
/// - Cannot contain space, `~`, `^`, `:`, `?`, `*`, `[`, `\`
/// - Cannot contain `@{` sequence
/// - Cannot contain null character (author's choice: git is implemented in C
///   where null terminates strings, so this is implicitly forbidden)
///
/// Whole branch constraints:
/// - Cannot be single `@`
/// - Cannot end with `.`
///
/// # Examples
///
/// ```
/// use greenhell::github::Branch;
///
/// let branch: Branch = "main".parse().unwrap();
/// assert_eq!(branch.as_str(), "main");
///
/// let branch: Branch = "feature/add-login".parse().unwrap();
/// assert_eq!(branch.as_str(), "feature/add-login");
///
/// let branch: Branch = "feature/level-1/level-2".parse().unwrap();
/// assert_eq!(branch.as_str(), "feature/level-1/level-2");
///
/// // Rejects empty
/// assert!("".parse::<Branch>().is_err());
///
/// // Rejects single @
/// assert!("@".parse::<Branch>().is_err());
///
/// // Rejects component starting with .
/// assert!(".hidden".parse::<Branch>().is_err());
/// assert!("feature/.hidden".parse::<Branch>().is_err());
///
/// // Rejects component ending with .lock
/// assert!("branch.lock".parse::<Branch>().is_err());
/// assert!("feature/branch.lock".parse::<Branch>().is_err());
///
/// // Rejects consecutive dots
/// assert!("feature..branch".parse::<Branch>().is_err());
///
/// // Rejects ending with .
/// assert!("branch.".parse::<Branch>().is_err());
///
/// // Rejects forbidden characters
/// assert!("branch name".parse::<Branch>().is_err());
/// assert!("branch~name".parse::<Branch>().is_err());
/// assert!("branch^name".parse::<Branch>().is_err());
/// assert!("branch:name".parse::<Branch>().is_err());
/// assert!("branch?name".parse::<Branch>().is_err());
/// assert!("branch*name".parse::<Branch>().is_err());
/// assert!("branch[name".parse::<Branch>().is_err());
/// assert!("branch\\name".parse::<Branch>().is_err());
/// assert!("branch\0name".parse::<Branch>().is_err());
///
/// // Rejects @{ sequence
/// assert!("branch@{name".parse::<Branch>().is_err());
///
/// // Rejects consecutive slashes
/// assert!("feature//branch".parse::<Branch>().is_err());
///
/// // Rejects leading/trailing slash
/// assert!("/branch".parse::<Branch>().is_err());
/// assert!("branch/".parse::<Branch>().is_err());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch(String);

impl Branch {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parse for Branch {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        const FORBIDDEN_CHARS: &[char] = &[' ', '~', '^', ':', '?', '*', '[', '\\', '\0'];

        fn is_valid_char(character: char) -> bool {
            !character.is_ascii_control()
                && character != '/'
                && !FORBIDDEN_CHARS.contains(&character)
        }

        fn parse_component(remaining: &str) -> IResult<&str, &str, VerboseError<&str>> {
            context(
                "branch component: no leading dot, no trailing .lock, no .., no @{",
                verify(take_while1(is_valid_char), |string: &str| {
                    !string.starts_with('.')
                        && !string.ends_with(".lock")
                        && !string.contains("..")
                        && !string.contains("@{")
                }),
            )
            .parse(remaining)
        }

        let start = remaining;

        let (remaining, _) = context(
            "branch: one or more components separated by /",
            separated_list1(char('/'), parse_component),
        )
        .parse(remaining)?;

        let parsed_len = start.len() - remaining.len();
        let parsed = &start[..parsed_len];

        if parsed == "@" {
            return Err(nom::Err::Error(VerboseError {
                errors: vec![(
                    start,
                    VerboseErrorKind::Context("branch cannot be single @"),
                )],
            }));
        }

        if parsed.ends_with('.') {
            return Err(nom::Err::Error(VerboseError {
                errors: vec![(start, VerboseErrorKind::Context("branch cannot end with ."))],
            }));
        }

        Ok((remaining, Self(parsed.to_string())))
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
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
