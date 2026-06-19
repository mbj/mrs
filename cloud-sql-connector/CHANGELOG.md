# Changelog

## 0.7.0

### Fixed

- Ephemeral certificate expiry is now read from the certificate's own `notAfter`
  field instead of the `expiration_time` field of the `generateEphemeralCert`
  API response. That field is not reliably populated, and when it came back
  unset the dial aborted with `ephemeral certificate missing expiration_time in
  generateEphemeralCert response`, resetting the connection. Reading `notAfter`
  from the issued certificate — the source of truth used by the official Cloud
  SQL connectors — removes the dependency on the optional API field.

### Changed

- `Error::EphemeralCertNoExpiration` is **removed** and `Error::EphemeralCertParse`
  is added, reflecting that the expiry now comes from parsing the certificate
  rather than the API response. `Error::EphemeralCertExpiration` is unchanged but
  its message now refers to the certificate's `notAfter` being out of range.

## 0.6.0

### Added

- `login_token::LoginTokenSource`, a reusable source that holds the login-scoped
  credentials and mints a `LoginToken` on demand via `login_token`. Constructors
  mirror `Dialer`: `new` (ambient identity), `with_credentials` (caller-supplied
  login-scoped credentials), `impersonating` (impersonate a target from the
  ambient identity), and `impersonating_with_source` (impersonate from an
  explicit source identity rather than Application Default Credentials).

### Changed

- IAM login tokens are now minted through `LoginTokenSource` instead of the
  free `login_token::login_token` and `login_token::login_token_target_principal`
  functions, which are **removed**. The free functions rebuilt the credentials on
  every call — rediscovering Application Default Credentials and spawning a fresh
  background token-refresh task that was dropped after a single read. Holding the
  credentials in a `LoginTokenSource` lets the token cache serve repeated calls
  from memory and refresh itself, which matters because the token is the database
  password fetched on every new connection. Migrate `login_token().await?` to
  `LoginTokenSource::new()?.login_token().await?` (building the source once), and
  `login_token_target_principal(&t).await?` to
  `LoginTokenSource::impersonating(&t)?.login_token().await?`.

## 0.5.0

### Added

- `Dialer::new_with_credentials`, letting the caller pass the credentials the
  dialer authenticates with instead of resolving Application Default Credentials
  itself — e.g. to share one credential set across several dialers.

### Changed

- `Dialer` now caches the per-instance connect settings and ephemeral
  certificate, fetching them from the Cloud SQL Admin API only when the cache is
  empty or the certificate nears expiry instead of on every `dial()`. This cuts
  the steady state to a single TCP connect and TLS handshake per dial, reducing
  dial latency and Admin API traffic for proxies serving many connections.
- A failed `dial()` now refreshes the cached connect info and retries once, so a
  failover or CA rotation that changes the instance IP or server CA is recovered
  rather than failing until the certificate would have expired. Concurrent dials
  that hit the same failure share a single refresh instead of stampeding the
  Admin API.

## 0.4.0

### Added

- IAM database authentication support, exposed as engine-agnostic building blocks:
  - `login_token` module with `LoginToken` (a secret-redacting access token),
    `login_token` for the ambient identity (Application Default Credentials), and
    `login_token_target_principal` for service-account impersonation.
  - `service_account::Email` parsing user-managed service account emails, with
    `id`, `project`, and `without_domain_suffix` accessors.
  - `user::Email` parsing human user emails, with `local` and `domain` accessors.
  - `principal::IamPrincipal` unifying the two identity kinds.
- `google-cloud-auth` dependency for minting login tokens.

## 0.3.1

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.3.0

### Breaking Changes

- `TcpServer::new` now requires a `PeerFilter` parameter to control which incoming
  connections are allowed through the proxy

### Added

- `PeerFilter` enum with variants `All`, `Subnets(Vec<IpNet>)`, and `Custom(Box<dyn Fn>)`
  for filtering incoming connections by peer address
- `PeerFilter::is_allowed` method to check whether a peer address passes the filter
- `ipnet` dependency for subnet-based filtering
- Documentation on `new_localhost_v4` / `new_localhost_v6` noting that they accept
  all connections since they are bound to loopback

## 0.2.0

### Changed

- `TcpServer::local_addr` now returns `SocketAddr` directly instead of `Result<SocketAddr, Error>`.

### Fixed

- `TcpServer` log message now shows the actual bound address instead of the requested address, which previously displayed port 0 when using kernel-assigned ports.

## 0.1.0

### Added

- `TcpServer` for proxying connections over TCP.
- `TcpServer::new` for binding to a caller-provided address and port.
- `TcpServer::new_localhost_v4` for binding to IPv4 localhost with an OS-assigned port.
- `TcpServer::new_localhost_v6` for binding to IPv6 localhost with an OS-assigned port.
- `TcpServer::local_addr` for discovering the bound address and port.

## 0.0.1

### Added

- `Dialer` for establishing mTLS connections to Cloud SQL instances via the Admin API.
- Ephemeral certificate generation using RSA 2048-bit keypairs.
- Custom TLS certificate verifier validating the per-instance CA chain.
- `serve` function for proxying connections over a Unix socket.
