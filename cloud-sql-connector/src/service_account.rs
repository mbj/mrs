//! Service account email parsing for Cloud SQL IAM database authentication.

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::all_consuming,
    error::Error,
};

/// Domain a user-managed service account email carries: the project id is
/// followed by `.iam.gserviceaccount.com`.
const IAM_DOMAIN: &str = ".iam.gserviceaccount.com";

/// Suffix shared by all service account emails, stripped by
/// [`Email::without_domain_suffix`].
pub(crate) const DOMAIN_SUFFIX: &str = ".gserviceaccount.com";

/// A user-managed GCP service account email,
/// `<id>@<project>.iam.gserviceaccount.com`.
///
/// This is the service-account form that can be a Cloud SQL IAM database user.
/// Only the exact `<id>@<project>.iam.gserviceaccount.com` grammar is accepted,
/// so forms with a different domain — e.g. the default compute account
/// `…@developer.gserviceaccount.com` or the truncated `…@<project>.iam` — are
/// rejected.
///
/// Holds the full email; [`as_str`](Self::as_str), [`id`](Self::id),
/// [`project`](Self::project), and [`without_domain_suffix`](Self::without_domain_suffix)
/// return borrowed slices of it. The engine-specific database username is the
/// consumer's to assemble from these parts — e.g. for PostgreSQL
/// [`without_domain_suffix`](Self::without_domain_suffix) (`<id>@<project>.iam`),
/// for MySQL just [`id`](Self::id).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email {
    /// The full email, e.g. `sa-name@project-id.iam.gserviceaccount.com`.
    email: String,
    /// Byte offset of the `@` separating `<id>` from `<project>`, recorded by
    /// the parser so the accessors need not rescan.
    at: usize,
}

impl Email {
    /// The full email, `<id>@<project>.iam.gserviceaccount.com`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.email
    }

    /// The account id (local part before `@`), e.g. `sa-name`.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.email[..self.at]
    }

    /// The GCP project id, e.g. `project-id`.
    #[must_use]
    pub fn project(&self) -> &str {
        &self.email[self.at + 1..self.email.len() - IAM_DOMAIN.len()]
    }

    /// The email with the shared `.gserviceaccount.com` suffix removed, e.g.
    /// `sa-name@project-id.iam`. This is the form Cloud SQL for PostgreSQL uses
    /// as the database username (the full email exceeds the username length
    /// limit), but it is offered here as a structural part — the engine policy is
    /// the consumer's.
    #[must_use]
    pub fn without_domain_suffix(&self) -> &str {
        &self.email[..self.email.len() - DOMAIN_SUFFIX.len()]
    }
}

/// A character permitted in a service-account id or project id: lowercase ASCII
/// letters, digits, and hyphens.
fn is_label_char(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
}

/// Parse `<id>@<project>.iam.gserviceaccount.com`, returning the byte offset of
/// the `@`. Requires the whole input to be consumed.
fn parse(input: &str) -> IResult<&str, usize, Error<&str>> {
    all_consuming((
        take_while1(is_label_char),
        char('@'),
        take_while1(is_label_char),
        tag(IAM_DOMAIN),
    ))
    .parse(input)
    .map(|(remaining, (id, _at, _project, _domain))| (remaining, id.len()))
}

/// Error parsing an [`Email`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("not a user-managed service account email (<id>@<project>.iam.gserviceaccount.com)")]
pub struct ParseError;

impl std::str::FromStr for Email {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (_, at) = parse(value).map_err(|_| ParseError)?;
        Ok(Self {
            email: value.to_owned(),
            at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<Email, ParseError> {
        value.parse()
    }

    #[test]
    fn parses_user_managed_email() {
        let email = parse("sa-name@project-id.iam.gserviceaccount.com").expect("valid email");
        assert_eq!(email.as_str(), "sa-name@project-id.iam.gserviceaccount.com");
        assert_eq!(email.id(), "sa-name");
        assert_eq!(email.project(), "project-id");
        assert_eq!(email.without_domain_suffix(), "sa-name@project-id.iam");
    }

    #[test]
    fn rejects_compute_default_service_account() {
        assert_eq!(
            parse("123456789-compute@developer.gserviceaccount.com"),
            Err(ParseError),
        );
    }

    #[test]
    fn rejects_truncated_username_form() {
        assert_eq!(parse("sa-name@project-id.iam"), Err(ParseError));
    }

    #[test]
    fn rejects_uppercase() {
        assert_eq!(
            parse("SA-Name@project-id.iam.gserviceaccount.com"),
            Err(ParseError),
        );
    }

    #[test]
    fn rejects_empty_id_or_project() {
        assert_eq!(
            parse("@project-id.iam.gserviceaccount.com"),
            Err(ParseError)
        );
        assert_eq!(parse("sa-name@.iam.gserviceaccount.com"), Err(ParseError));
    }

    #[test]
    fn rejects_trailing_data() {
        assert_eq!(
            parse("sa-name@project-id.iam.gserviceaccount.com.evil"),
            Err(ParseError),
        );
    }
}
