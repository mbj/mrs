# Changelog

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
