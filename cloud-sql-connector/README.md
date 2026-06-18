# cloud-sql-connector

Cloud SQL Auth Proxy connector for Rust.

Implements the Cloud SQL connector protocol: generates an RSA keypair, calls
the Cloud SQL Admin API for ephemeral certificates, and establishes TLS 1.3
connections directly to Cloud SQL instances.

## Usage

### Direct TLS connection

```rust,no_run
use std::sync::Arc;
use cloud_sql_connector::Dialer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dialer = Arc::new(Dialer::new("my-project", "my-instance").await?);
    let mut stream = dialer.dial().await?;
    Ok(())
}
```

### Unix socket proxy

```rust,no_run
use std::path::Path;
use std::sync::Arc;
use cloud_sql_connector::{Dialer, UnixSocketServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dialer = Arc::new(Dialer::new("my-project", "my-instance").await?);
    let server = UnixSocketServer::new(dialer, Path::new("/tmp/cloud-sql.sock"))?;
    // Socket is bound and ready to accept connections.
    server.serve().await?;
    Ok(())
}
```

### Passing credentials

`Dialer::new` resolves Application Default Credentials itself. To supply your own
instead — for example to share one credential set across several dialers — pass
them to `new_with_credentials`. The credentials refresh their own access tokens,
so one instance lasts the dialer's lifetime.

```rust,no_run
use std::sync::Arc;
use cloud_sql_connector::Dialer;
use google_cloud_auth::credentials::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Builder::default().build()?;
    let dialer = Arc::new(
        Dialer::new_with_credentials("my-project", "my-instance", credentials).await?,
    );
    let mut stream = dialer.dial().await?;
    Ok(())
}
```

## IAM database authentication

Cloud SQL IAM database authentication lets an IAM identity log in with a
short-lived OAuth access token instead of a stored password. This crate exposes
the pieces as engine-agnostic building blocks: it mints the token and parses the
identity, but leaves the database username — which differs by identity kind and
database engine — for the consumer to assemble.

### Minting a login token

A [`LoginToken`](login_token::LoginToken) is a short-lived OAuth access token
(valid for about one hour) used as the database password. Its secret is hidden
from `Debug`; read it deliberately with `expose`.

A [`LoginTokenSource`](login_token::LoginTokenSource) holds the credentials and
mints tokens on demand. Build it once and reuse it: the token is needed on every
new connection, and the held credentials cache and self-refresh the underlying
access token, so steady-state calls read it from memory rather than fetching one
each time.

```rust,no_run
use cloud_sql_connector::login_token::LoginTokenSource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ambient identity: the Cloud Run service account in production, or the
    // developer's `gcloud` identity locally (Application Default Credentials).
    let source = LoginTokenSource::new()?;

    let token = source.login_token().await?;

    // Pass the secret to the database driver as the password.
    let _password = token.expose();
    Ok(())
}
```

To test a service account's database access from a developer machine without a
key file, impersonate it with
[`impersonating`](login_token::LoginTokenSource::impersonating). It authenticates
the impersonation with Application Default Credentials;
[`impersonating_with_source`](login_token::LoginTokenSource::impersonating_with_source)
takes an explicit source identity instead.

```rust,no_run
use cloud_sql_connector::{login_token::LoginTokenSource, service_account};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target: service_account::Email =
        "sa-name@project-id.iam.gserviceaccount.com".parse()?;

    let source = LoginTokenSource::impersonating(&target)?;
    let token = source.login_token().await?;
    let _password = token.expose();
    Ok(())
}
```

### Identities and database usernames

[`IamPrincipal`](principal::IamPrincipal) parses either kind of identity and
discriminates on the `.gserviceaccount.com` suffix. The database username is
derived from the identity's parts — differently per kind and per engine — so
match on the variant and assemble it yourself:

```rust
use cloud_sql_connector::principal::IamPrincipal;

// A user-managed service account.
let principal: IamPrincipal =
    "sa-name@project-id.iam.gserviceaccount.com".parse()?;
let IamPrincipal::ServiceAccount(email) = &principal else {
    unreachable!()
};
// Cloud SQL for PostgreSQL drops the `.gserviceaccount.com` suffix; MySQL uses
// just the account id.
assert_eq!(email.without_domain_suffix(), "sa-name@project-id.iam");
assert_eq!(email.id(), "sa-name");

// A human user.
let principal: IamPrincipal = "operator@example.com".parse()?;
let IamPrincipal::User(email) = &principal else {
    unreachable!()
};
// PostgreSQL uses the full email; MySQL uses the local part.
assert_eq!(email.as_str(), "operator@example.com");
assert_eq!(email.local(), "operator");
# Ok::<(), Box<dyn std::error::Error>>(())
```
