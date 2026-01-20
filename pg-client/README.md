# pg-client - PostgreSQL Client Configuration

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

A PostgreSQL client configuration library providing strongly-typed connection parameters with accurate representation of PostgreSQL connection options.

## Motivation

`sqlx::postgres::PgConnectOptions` has several limitations:

- No `PartialEq` implementation, making testing difficult
- Incomplete getter methods (e.g., no access to password or SSL root cert)
- No way to construct an instance without environment variable inference
- No API to unset fields that were inferred from environment variables
- Conflates configuration with connection state

`pg-client` addresses these issues by providing a pure configuration type with complete control over all connection parameters.

## Feature Flags

### `sqlx`

Enables integration with [sqlx](https://github.com/launchbadge/sqlx):

- `Config::to_sqlx_connect_options()` - converts to `sqlx::postgres::PgConnectOptions`
- `Config::with_sqlx_connection()` - runs an async action with a managed connection

```toml
[dependencies]
pg-client = { version = "0.0.1-pre1", features = ["sqlx"] }
```

## Features

### Strongly-Typed Connection Parameters

All connection parameters use newtype wrappers with validation:

- `HostName` - validated hostname (via `hostname-validator`)
- `Host` - either a hostname or IP address (IPv4/IPv6)
- `HostAddr` - explicit IP address for connection
- `Port` - TCP port number
- `Database`, `Username`, `ApplicationName` - length-validated strings (1-63 bytes, no NUL)
- `Password` - length-validated (0-4096 bytes, no NUL)
- `SslMode` - all PostgreSQL SSL modes (`disable`, `allow`, `prefer`, `require`, `verify-ca`, `verify-full`)
- `SslRootCert` - file path or `system`

### Endpoint Types

Distinguishes between network and Unix socket connections:

```rust,ignore
enum Endpoint {
    Network { host: Host, host_addr: Option<HostAddr>, port: Option<Port> },
    SocketPath(PathBuf),
}
```

### Multiple Output Formats

A single `Config` can be converted to:

- PostgreSQL environment variables (`to_pg_env()`) - returns `BTreeMap<EnvVariableName, String>` with keys like `pg_client::PGHOST`, `pg_client::PGPORT`, `pg_client::PGDATABASE`
- PostgreSQL connection URL (`to_url()`)
- `sqlx::postgres::PgConnectOptions` (`to_sqlx_connect_options()`)
- JSON (via serde)

### Environment Contradiction Detection

When converting to `sqlx::postgres::PgConnectOptions`, the library detects conflicts between your configuration and environment variables that sqlx would silently infer:

```rust,ignore
// If PGPASSWORD is set but config.password is None, this returns an error
// rather than silently using the environment value
let options = config.to_sqlx_connect_options()?;
```

This prevents subtle bugs where environment variables override your intended configuration. The library also rejects unsupported environment variables (`PGSSLKEY`, `PGSSLCERT`, `PGOPTIONS`) that sqlx would pick up.

### Secure Defaults

When parsing connection URLs without an explicit `sslmode` parameter, this library defaults to `verify-full` rather than the `prefer` default used by libpq and sqlx.

libpq's `prefer` mode attempts TLS but silently falls back to unencrypted connections if TLS fails, providing no protection against downgrade attacks. This default made sense in 1996 when PostgreSQL was primarily used on trusted networks, but is inappropriate for modern deployments where databases are frequently accessed over untrusted networks.

By defaulting to `verify-full`, connections require TLS with full certificate verification. Applications that explicitly need weaker security can specify `sslmode=prefer` or `sslmode=disable` in their connection URLs.
