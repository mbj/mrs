# Changelog

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
