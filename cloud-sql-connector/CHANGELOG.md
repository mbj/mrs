# Changelog

## 0.0.1

### Added

- `Dialer` for establishing mTLS connections to Cloud SQL instances via the Admin API.
- Ephemeral certificate generation using RSA 2048-bit keypairs.
- Custom TLS certificate verifier validating the per-instance CA chain.
- `serve` function for proxying connections over a Unix socket.
