//! IAM principal identities that can be Cloud SQL IAM database users.

use crate::{service_account, user};

/// An IAM identity usable as a Cloud SQL IAM database user.
///
/// Either a service account or a human user. The `.gserviceaccount.com` suffix
/// discriminates: an email carrying it must be a valid user-managed service
/// account (`<id>@<project>.iam.gserviceaccount.com`) or parsing fails — it does
/// not fall through to a human user. Any other email is parsed as a human user.
///
/// The engine-specific database username is derived differently per kind (a
/// service account truncates its domain, a human user does not) and differently
/// per database engine, so it is left to the consumer: match on the variant and
/// assemble it from the inner email's parts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IamPrincipal {
    /// A service account identity.
    ServiceAccount(service_account::Email),
    /// A human user identity.
    User(user::Email),
}

impl IamPrincipal {
    /// The full identity email.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ServiceAccount(email) => email.as_str(),
            Self::User(email) => email.as_str(),
        }
    }
}

/// Error parsing an [`IamPrincipal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("not a service account or user email")]
pub struct ParseError;

impl std::str::FromStr for IamPrincipal {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.ends_with(service_account::DOMAIN_SUFFIX) {
            value
                .parse::<service_account::Email>()
                .map(Self::ServiceAccount)
                .map_err(|_| ParseError)
        } else {
            value
                .parse::<user::Email>()
                .map(Self::User)
                .map_err(|_| ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_service_account() {
        let principal: IamPrincipal = "sa-name@project-id.iam.gserviceaccount.com"
            .parse()
            .expect("valid service account");
        assert_eq!(
            principal.as_str(),
            "sa-name@project-id.iam.gserviceaccount.com"
        );
        let IamPrincipal::ServiceAccount(email) = &principal else {
            panic!("expected service account");
        };
        assert_eq!(email.id(), "sa-name");
        assert_eq!(email.project(), "project-id");
    }

    #[test]
    fn parses_user() {
        let principal: IamPrincipal = "operator@gcp.venice.ai".parse().expect("valid user");
        assert_eq!(principal.as_str(), "operator@gcp.venice.ai");
        let IamPrincipal::User(email) = &principal else {
            panic!("expected user");
        };
        assert_eq!(email.local(), "operator");
        assert_eq!(email.domain(), "gcp.venice.ai");
    }

    #[test]
    fn rejects_non_user_managed_service_account() {
        // Carries the `.gserviceaccount.com` suffix, so it is routed to the
        // service-account parser and rejected — it must not fall through to a
        // human user even though its parts satisfy the user grammar.
        assert_eq!(
            "123456789-compute@developer.gserviceaccount.com".parse::<IamPrincipal>(),
            Err(ParseError),
        );
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!("not-an-email".parse::<IamPrincipal>(), Err(ParseError));
    }
}
