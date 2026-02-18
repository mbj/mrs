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

## TODO

* Cache connect settings (IP address, server CA cert) in the `Dialer` instead
  of fetching them on every `dial()` call. These are stable per instance and
  only change on failover or CA rotation. The Go connector refreshes them
  every ~30 minutes.
