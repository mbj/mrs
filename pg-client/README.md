# pg-client

**Status: Highly experimental - currently only intended for use within [pg-ephemeral](../pg-ephemeral)**

A PostgreSQL client configuration library providing strongly-typed connection parameters with accurate representation of PostgreSQL connection options.

## Motivation

`sqlx::postgres::PgConnectOptions` has several limitations:

- No `PartialEq` implementation, making testing difficult
- Incomplete getter methods (e.g., no access to password or SSL root cert)
- No way to construct an instance without environment variable inference
- No API to unset fields that were inferred from environment variables
- Conflates configuration with connection state

`pg-client` addresses these issues by providing a pure configuration type with complete control over all connection parameters.

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

- PostgreSQL environment variables (`to_pg_env()`) - `PGHOST`, `PGPORT`, `PGDATABASE`, etc.
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

## License

MIT
