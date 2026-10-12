# Changelog

## 0.7.0

### Breaking Changes

- Connection URLs now follow libpq's connection URI syntax instead of generic
  RFC 3986 URL syntax, so a URL means the same to pg-client as to `psql` and
  other libpq clients. The naming reflects this: the `url` module is now
  `libpq_url`, `Config::from_str_url` is now `Config::from_libpq_url`, and
  `Config::to_url_string` is now `Config::to_libpq_url`. `Config::to_url()` is
  removed; `Config::to_libpq_url()` produces the same URL text.
- Parsing differences that follow from the libpq syntax:
  - `#` is an ordinary character: `postgres://user@host/db#x` names the
    database `db#x` instead of being rejected as a fragment.
  - The user part extends to the first `@` that is not preceded by a `/`, also
    inside the query string. `postgres://?host=@abstract&…` therefore reads
    `?host=` as the user, exactly as libpq does; percent-encode the `@` as
    `%40` instead.
  - Every query parameter needs exactly one `=`: `?sslmode` and `?key=a=b`
    are rejected. An empty query (`postgres://host/db?`) and a trailing `&`
    are accepted.
  - Spaces around each component are ignored; spaces inside a component must
    be percent-encoded as `%20`. `%00` is rejected.
  - A query parameter given with an empty value, such as `?user=`, counts as
    given: it conflicts with a user in the URL and otherwise leaves the
    setting unset. When a query parameter is repeated, the last one wins.
  - The scheme must be followed by `//`: `postgres:?host=…` is rejected.
  - Multiple hosts (`host1,host2`) are rejected with the new
    `libpq_url::ParseError::MultipleHosts`.
- `libpq_url::ParseError` changes: `InvalidUrl` now carries the byte `offset`
  where the URL stopped matching the syntax instead of a
  `fluent_uri::ParseError`. `InvalidFragment` is removed. `InvalidPort` is
  removed; an invalid port is now a `FieldError` for the new `Field::Port`.
  Percent-decoding failures are reported as the new `DecodeError`, in
  `FieldErrorCause::InvalidEncoding` (replacing `FieldErrorCause::InvalidUtf8`)
  and in `ParseError::InvalidQueryParameterEncoding`.
- `config::Endpoint::SocketPath` is now `SocketPath { path, port }`. The port
  selects the socket file (`.s.PGSQL.<port>`), as in libpq, and is passed on
  as `PGPORT`, to sqlx, and in `Config::to_libpq_url()`.

### Added

- `parameter::Assignment`, a `NAME=VALUE` parameter assignment with `FromStr`
  and `Display`. This is PG's own textual assignment syntax, shared by the `-c`
  command-line flag, `postgresql.conf` entries, and `PGOPTIONS`. `Name` rejects
  `=`, so the first `=` is unambiguously the separator and everything after it
  is the value.
- Connection URLs accept the `port` query parameter.
- Connection URLs accept a socket path as the host, percent-encoded:
  `postgres://user@%2Fvar%2Frun%2Fpostgresql/db`. Any host starting with `/`
  or `@` is a socket path, in the URL as well as in the `host` query parameter.
  The `host` query parameter may also name a network host.
- Connection URLs accept `ssl=true` as `sslmode=require`, as libpq does for
  JDBC compatibility.

### Fixed

- The port of a socket connection URL, such as
  `postgres://user@:5433/db?host=/var/run/postgresql`, was silently dropped,
  so the connection used the default port's socket.

### Changed

- Raised minimum supported Rust version to 1.99.
- Dropped the `hostname-validator` dependency. `config::HostName` is now
  validated by a built-in parser and accepts the same names as before.
  `nom` is now a required dependency instead of only coming with the `clap`
  feature.
- Dropped the `fluent-uri` dependency. Connection URLs are parsed and
  produced by a built-in implementation of the libpq syntax.

## 0.6.0

### Breaking Changes

- `Config::to_sqlx_connect_options()` no longer inspects process environment
  variables. Connection options are now built solely from `Config`, so ambient
  `PG*` variables (`PGHOST`, `PGPORT`, `PGPASSWORD`, …) can no longer influence
  or conflict with the produced options. Previously a contradicting variable
  produced an error.
- `OptionsError::EnvConflict` is removed, and `OptionsError::UnsupportedFeature`
  no longer carries an `env_key` field (now `{ field_name }`). Both reported
  conflicts with environment variables that are no longer consulted;
  `UnsupportedFeature` now reports only unsupported `Config` settings such as
  channel binding.

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.5.0

### Breaking Changes

- `Config::to_pg_env()` is renamed to `Config::pg_env()` and is now fallible:
  returns `Result<BTreeMap<EnvVariableName, EnvVariableValue>,
  cmd_proc::EnvVariableValueError>`. The map's value type changes from `String`
  to the validated `cmd_proc::EnvVariableValue`. The rename follows the Rust
  convention that `to_*` is infallible; the fallibility comes from the
  per-value env validation now applied at construction.
- `EnvVariableName` constants exposed from this crate (`PGUSER`, `PGHOST`, etc.)
  no longer carry a `<'static>` parameter — the lifetime was dropped at the
  `cmd_proc` level. Update type annotations to `cmd_proc::EnvVariableName`.

## 0.4.0

### Breaking Changes

- `FromStr` implementations for `Password`, `ApplicationName`, `HostName`,
  `Host`, `HostAddr`, and `Port` no longer use `String` or `&'static str` as
  their `Err` type. Each now has a dedicated parse-error type implementing
  `Debug + Display + std::error::Error`:
  - `PasswordParseError` and `ApplicationNameParseError`: per-type enums
    generated by `from_str_impl!` with `TooShort { min, actual }`,
    `TooLong { max, actual }`, and `ContainsNul` variants.
  - `HostNameParseError`, `HostParseError`, `HostAddrParseError`,
    `PortParseError`: unit error structs.
  - `Display` output is preserved verbatim, so format-only assertions and
    user-visible messages are unchanged. Callers that named the old `Err =
    String` / `Err = &'static str` types must update to the new types;
    `error.to_string()` continues to work for stringification-style usage.

## 0.3.0

### Breaking Changes

- Update to `cmd-proc` 0.5.0. `cmd_proc::EnvVariableName` is re-exposed through
  the public `PG*` constants (`PGAPPNAME`, `PGHOST`, `PGPORT`, ...) and the
  `sqlx` module API, so the `cmd-proc` 0.5.0 upgrade is a breaking change for
  pg-client consumers: any caller that also depends on `cmd-proc` 0.4.0 will
  see type-identity mismatches on these constants.

## 0.2.0

Separate transport configuration from session/auth configuration.

`Config` previously mixed transport concerns (endpoint, SSL) with session
concerns (user, password, database, application name). There are scenarios
where the raw connection (TCP socket, Unix socket) is already established
externally and only the session parameters for the PostgreSQL startup message
are needed. Extracting `Session` makes it possible to pass around just the
auth/session data independently of transport.

### Breaking Changes

- Introduce `pg_client::config::Session` struct grouping session-level fields:
  `user`, `password`, `database`, `application_name`
- Move connection/session types into `pg_client::config` module:
  `Endpoint`, `Host`, `HostName`, `HostAddr`, `Port`, `SslMode`, `SslRootCert`,
  `ChannelBinding`, `Password`, `ApplicationName`
- `Config` fields restructured: `user`, `password`, `database`, `application_name`
  replaced by a single `session: config::Session` field
- No backwards-compatible re-exports; all downstream code must update type paths
- Add `sqlx` field to `Config` (behind `sqlx` feature flag) with `sqlx::Settings`
  exposing `statement_cache_capacity`, `log_statements`, and `log_slow_statements`

## 0.1.0

- Version bump signalling pg-client is on the path to stabilization.

## 0.0.5

### Breaking Changes

- Change `PgSchemaDump::restrict_key` to accept `&RestrictKey` instead of `RestrictKey`

### Added

- `From<&RestrictKey>` implementation for `RestrictKey`

## 0.0.4

### Breaking Changes

- Replace separate `schema`/`table` fields with `QualifiedTable` in `AnalyzeTask`,
  partitioned index `Input`, `Partition`, and `Error::NoPartitions`

### Added

- `QualifiedTable` struct combining `Schema` and `Table` with `Display` implementation
- `pg_dump` module with `PgSchemaDump` builder for configurable `pg_dump` commands
- `RestrictKey` type for `pg_dump` restrict/unrestrict key (CVE-2025-8714)
- Re-export `QualifiedTable`, `PgSchemaDump`, and `RestrictKey` from crate root

### Fixed

- Fix circular dev-dependency on `pg-ephemeral` by using path-only reference

## 0.0.3

### Breaking Changes

- Switch to async `cmd-proc`: command execution methods are now `async fn`
- Switch `sqlx` dependency to `msqlx` fork
- Change to `u16` non-zero interface for port types
- Change to `clap` args for CLI integration
- Change to consistent instance name types

### Added

- GCP Cloud SQL URL support
- Partitioned index addition helper
- Support for partially concurrent indexes
- `INCLUDE` support for covering indexes
- Fillfactor support
- `gc` subcommand for incomplete index cleanup
- `analyze all` support
- Channel binding URL parameter support
- `host_addr` support
- Comprehensive PostgreSQL object identifier types: column, index, constraint,
  extension, sequence, function, trigger, domain, type, view, relation,
  materialized view, operator, aggregate, collation, tablespace, policy, rule,
  publication, subscription, foreign server, foreign data wrapper, foreign table,
  event trigger, language, text search configuration, text search dictionary,
  conversion, operator class, operator family, access method, statistics object

### Fixed

- Reject NUL bytes in identifiers
- Fix channel binding URL parsing query parameter
- Fix duplicated constants
- Fix conflicting user/hostname/database values
- Fix to not analyze partition parents
- Fix to allow analyze statements concurrently
- Fix to not attempt to set unsupported `system` pgsslroot (sqlx/msqlx integration)

## 0.0.2

- URL parsing now accepts a `&str` and uses RFC 3986 semantics; `+` is treated as a literal plus
  (use `%20` for spaces). The prior behavior treated `+` as space, which was incorrect for URIs.
- URL parsing errors now report field-specific detail via `ParseError::Field`.

## 0.0.1

- Initial release
- URL parsing support
- `FromStr` instance for `SslMode`
- Optional `sqlx` feature flag
