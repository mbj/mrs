//! Human user email parsing for Cloud SQL IAM database authentication.

use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{all_consuming, recognize, verify},
    error::Error,
    multi::separated_list1,
};

/// A human Google account email used as a Cloud SQL IAM database user.
///
/// Cloud SQL requires the username to be lowercase, which is enforced. The
/// structure is only validated as `<local>@<domain>` — the precise character set
/// is Google's concern, already enforced at account creation, so it is not
/// re-litigated here. No length limit is imposed: the maximum username length is
/// engine-specific (e.g. PostgreSQL's 63-char `NAMEDATALEN`), so it belongs to
/// the consuming engine, which rejects an over-long name itself.
///
/// Holds the full email; [`as_str`](Self::as_str), [`local`](Self::local), and
/// [`domain`](Self::domain) return borrowed slices of it. The engine-specific
/// database username is the consumer's to assemble — e.g. the full email for
/// PostgreSQL, just the `local` part for MySQL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email {
    /// The full email, e.g. `operator@example.com`.
    email: String,
    /// Byte offset of the `@` separating `<local>` from `<domain>`, recorded by
    /// the parser so the accessors need not rescan.
    at: usize,
}

impl Email {
    /// The full email, `<local>@<domain>`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.email
    }

    /// The local part, before `@`, e.g. `operator`. This is the Cloud SQL for
    /// MySQL IAM database username, which truncates the `@<domain>`; PostgreSQL
    /// uses the full email ([`as_str`](Self::as_str)) instead.
    #[must_use]
    pub fn local(&self) -> &str {
        &self.email[..self.at]
    }

    /// The domain, after `@`, e.g. `example.com`.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.email[self.at + 1..]
    }
}

/// A character permitted in a local-part label: lowercase ASCII letters, digits,
/// and the common mailbox punctuation `-`, `_`, `+`. The precise account charset
/// is Google's concern (validated upstream); this rejects uppercase (Cloud SQL's
/// lowercase rule), whitespace, and the `@`/`.` delimiters.
fn is_local_char(character: char) -> bool {
    character.is_ascii_lowercase()
        || character.is_ascii_digit()
        || matches!(character, '-' | '_' | '+')
}

/// A character permitted in a domain label: lowercase ASCII letters, digits, and
/// `-` (the DNS label set). Unlike the local part, `_` and `+` are not valid in a
/// hostname, so they are rejected here.
fn is_domain_char(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
}

/// A dot-separated sequence of labels whose characters satisfy `is_char`,
/// returning the label count.
fn labels(is_char: impl Fn(char) -> bool) -> impl FnMut(&str) -> IResult<&str, usize, Error<&str>> {
    move |input| {
        separated_list1(char('.'), take_while1(&is_char))
            .parse(input)
            .map(|(remaining, parts)| (remaining, parts.len()))
    }
}

/// Parse `<local>@<domain>` where the local part and domain are dot-separated
/// labels (with their respective character sets) and the domain has at least two
/// labels. Returns the byte offset of the `@`. Requires the whole input to be
/// consumed and rejects uppercase, enforcing Cloud SQL's lowercase rule.
fn parse(input: &str) -> IResult<&str, usize, Error<&str>> {
    all_consuming((
        recognize(labels(is_local_char)),
        char('@'),
        verify(labels(is_domain_char), |&domain_labels| domain_labels >= 2),
    ))
    .parse(input)
    .map(|(remaining, (local, _at, _domain))| (remaining, local.len()))
}

/// Error parsing an [`Email`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("not a lowercase user email (<local>@<domain>)")]
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
    fn parses_user_email() {
        let email = parse("operator@example.com").expect("valid email");
        assert_eq!(email.as_str(), "operator@example.com");
        assert_eq!(email.local(), "operator");
        assert_eq!(email.domain(), "example.com");
    }

    #[test]
    fn parses_managed_domain_and_punctuation() {
        let email = parse("first.last@gcp.venice.ai").expect("valid email");
        assert_eq!(email.as_str(), "first.last@gcp.venice.ai");
    }

    #[test]
    fn exposes_local_and_domain() {
        let email = parse("first.last@gcp.venice.ai").expect("valid email");
        assert_eq!(email.local(), "first.last");
        assert_eq!(email.domain(), "gcp.venice.ai");
    }

    #[test]
    fn rejects_uppercase() {
        assert_eq!(parse("Operator@example.com"), Err(ParseError));
        assert_eq!(parse("operator@Example.com"), Err(ParseError));
    }

    #[test]
    fn allows_local_part_punctuation() {
        let email = parse("first-l_ast+tag@example.com").expect("valid email");
        assert_eq!(email.local(), "first-l_ast+tag");
    }

    #[test]
    fn rejects_domain_punctuation() {
        assert_eq!(parse("operator@ex_ample.com"), Err(ParseError));
        assert_eq!(parse("operator@ex+ample.com"), Err(ParseError));
    }

    #[test]
    fn rejects_missing_domain_dot() {
        assert_eq!(parse("operator@localhost"), Err(ParseError));
    }

    #[test]
    fn rejects_empty_parts() {
        assert_eq!(parse("@example.com"), Err(ParseError));
        assert_eq!(parse("operator@"), Err(ParseError));
        assert_eq!(parse("operator@.com"), Err(ParseError));
    }

    #[test]
    fn allows_long_email() {
        // No length limit is imposed here — the maximum username length is
        // engine-specific, so an over-63-char email (rejected by PostgreSQL but
        // valid for e.g. SQL Server) still parses.
        let long_local = "a".repeat(60);
        let email = format!("{long_local}@example.com");
        assert!(email.len() > 63);
        let parsed = parse(&email).expect("valid email");
        assert_eq!(parsed.as_str(), email);
    }

    #[test]
    fn rejects_trailing_data() {
        assert_eq!(parse("operator@example.com extra"), Err(ParseError));
    }
}
