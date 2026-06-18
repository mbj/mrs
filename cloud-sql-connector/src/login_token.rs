//! Cloud SQL IAM database authentication login tokens.

use google_cloud_auth::credentials::{AccessTokenCredentials, Builder, Credentials, impersonated};

use crate::service_account;

/// OAuth scope a token must carry to authenticate as a Cloud SQL IAM database
/// user. The token serves as the database password.
const LOGIN_SCOPE: &str = "https://www.googleapis.com/auth/sqlservice.login";

/// A Cloud SQL IAM login token: a short-lived OAuth access token used as the
/// database password. Valid for about one hour.
///
/// The secret is hidden from `Debug` so it cannot leak into logs; read it
/// deliberately with [`expose`](Self::expose).
#[derive(Clone)]
pub struct LoginToken(String);

impl LoginToken {
    /// The raw token. Pass it to the database driver as the password.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Debug for LoginToken {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("LoginToken(***)")
    }
}

/// Error obtaining a [`LoginToken`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Building the credentials failed (e.g. no Application Default Credentials).
    #[error(transparent)]
    Build(#[from] google_cloud_auth::build_errors::Error),
    /// Fetching the access token failed.
    #[error(transparent)]
    Token(#[from] google_cloud_auth::errors::CredentialsError),
}

/// A reusable source of Cloud SQL IAM [`LoginToken`]s for one identity.
///
/// Holds the login-scoped credentials and mints a token on demand via
/// [`login_token`](Self::login_token). The token is the database password, so it
/// is needed on every new connection; holding the credentials means the
/// underlying [`google_cloud_auth`] token cache services those repeated calls
/// from memory and refreshes itself in the background, rather than rediscovering
/// credentials and fetching a token from scratch each time.
///
/// Build it once and share it (e.g. behind an `Arc`) for the process lifetime.
#[derive(Clone, Debug)]
pub struct LoginTokenSource {
    /// Login-scoped credentials; self-refreshing and cheap to read repeatedly.
    credentials: AccessTokenCredentials,
}

impl LoginTokenSource {
    /// A source minting tokens for the ambient identity.
    ///
    /// The ambient identity comes from Application Default Credentials: the Cloud
    /// Run service account in production, or the developer's `gcloud` identity
    /// locally. This is the production path.
    ///
    /// # Errors
    ///
    /// Returns an error if credentials cannot be discovered.
    pub fn new() -> Result<Self, Error> {
        let credentials = Builder::default()
            .with_scopes([LOGIN_SCOPE])
            .build_access_token_credentials()?;
        Ok(Self { credentials })
    }

    /// A source minting tokens from caller-supplied credentials.
    ///
    /// Reuses `credentials` instead of resolving Application Default Credentials,
    /// for a caller who already holds login-scoped credentials. The credentials
    /// must carry the [`sqlservice.login`](LOGIN_SCOPE) scope — a token built for
    /// another scope (e.g. the Admin API scope a [`Dialer`] uses) is not a valid
    /// database password.
    ///
    /// [`Dialer`]: crate::Dialer
    #[must_use]
    pub fn with_credentials(credentials: AccessTokenCredentials) -> Self {
        Self { credentials }
    }

    /// A source minting tokens by impersonating `target_principal` from the
    /// ambient identity.
    ///
    /// Application Default Credentials authenticate the impersonation — useful for
    /// testing a service account's database access from a developer machine
    /// without a key file. To impersonate from an explicit identity instead of
    /// ADC, use [`impersonating_with_source`](Self::impersonating_with_source).
    ///
    /// # Errors
    ///
    /// Returns an error if credentials cannot be discovered.
    pub fn impersonating(target_principal: &service_account::Email) -> Result<Self, Error> {
        let source = Builder::default().build()?;
        Self::impersonating_with_source(target_principal, source)
    }

    /// A source minting tokens by impersonating `target_principal` from a
    /// caller-supplied identity.
    ///
    /// `source` authenticates the impersonation in place of Application Default
    /// Credentials, so it must be permitted to mint tokens for the target (the
    /// `roles/iam.serviceAccountTokenCreator` role) and carry a scope allowing the
    /// IAM Credentials API. The resulting login token is scoped for
    /// [`sqlservice.login`](LOGIN_SCOPE) regardless of `source`'s own scope.
    ///
    /// # Errors
    ///
    /// Returns an error if the impersonating credentials cannot be built.
    pub fn impersonating_with_source(
        target_principal: &service_account::Email,
        source: Credentials,
    ) -> Result<Self, Error> {
        let credentials = impersonated::Builder::from_source_credentials(source)
            .with_target_principal(target_principal.as_str())
            .with_scopes([LOGIN_SCOPE])
            .build_access_token_credentials()?;
        Ok(Self { credentials })
    }

    /// Mint a login token from the held credentials.
    ///
    /// The credentials cache and self-refresh the underlying access token, so in
    /// steady state this reads a still-valid token from memory rather than
    /// fetching one.
    ///
    /// # Errors
    ///
    /// Returns an error if the access token cannot be fetched.
    pub async fn login_token(&self) -> Result<LoginToken, Error> {
        let token = self.credentials.access_token().await?;
        Ok(LoginToken(token.token))
    }
}
