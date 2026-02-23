# Changelog

## 0.3.0

### Breaking Changes

- `TcpServer::new` now requires a `PeerFilter` parameter to control which incoming
  connections are allowed through the proxy

### Added

- `PeerFilter` enum with variants `All`, `Subnet(IpNet)`, and `Custom(Box<dyn Fn>)`
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
