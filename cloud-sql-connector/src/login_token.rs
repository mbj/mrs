//! Cloud SQL IAM database authentication login tokens.

use google_cloud_auth::credentials::{AccessTokenCredentials, Builder, impersonated};

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

/// Mint a Cloud SQL IAM login token for the ambient identity.
///
/// The ambient identity comes from Application Default Credentials: the Cloud
/// Run service account in production, or the developer's `gcloud` identity
/// locally. This is the production path.
///
/// # Errors
///
/// Returns an error if credentials cannot be discovered or the token cannot be
/// fetched.
pub async fn login_token() -> Result<LoginToken, Error> {
    let credentials = Builder::default()
        .with_scopes([LOGIN_SCOPE])
        .build_access_token_credentials()?;
    token_from(credentials).await
}

/// Mint a Cloud SQL IAM login token by impersonating `target_principal`.
///
/// The ambient identity (Application Default Credentials) impersonates the
/// target — useful for testing a service account's database access from a
/// developer machine without a key file.
///
/// # Errors
///
/// Returns an error if credentials cannot be discovered or the token cannot be
/// fetched.
pub async fn login_token_target_principal(
    target_principal: &service_account::Email,
) -> Result<LoginToken, Error> {
    let source = Builder::default().build()?;
    let credentials = impersonated::Builder::from_source_credentials(source)
        .with_target_principal(target_principal.as_str())
        .with_scopes([LOGIN_SCOPE])
        .build_access_token_credentials()?;
    token_from(credentials).await
}

/// Fetch an access token from built credentials and wrap it as a [`LoginToken`].
async fn token_from(credentials: AccessTokenCredentials) -> Result<LoginToken, Error> {
    let token = credentials.access_token().await?;
    Ok(LoginToken(token.token))
}
