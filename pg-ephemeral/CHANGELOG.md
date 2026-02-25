# Changelog

## 0.0.6

### Changed

- Make `Container::exec_schema_dump` public

## 0.0.5

- Bump pg-client dependency to 0.1.0

## 0.0.4

### Breaking Changes

- `Definition::schema_dump` now accepts `&PgSchemaDump` builder instead of `&[String]`
  for configurable `pg_dump` arguments

### Changed

- `Container::exec_schema_dump` now accepts `&PgSchemaDump` parameter

## 0.0.3

### Changed

- Switch integration protocol from stdin/stdout to inherited pipe FDs.
  Seeds can now freely print to stdout/stderr without corrupting the protocol.

## 0.0.2

### Changed

- Resolve `sql-file` paths and relative-path `command` executables relative to the
  config file's directory instead of the process working directory

## 0.0.1

### Breaking Changes

- Switch to async `cmd-proc` 0.4.0 and `git-proc` 0.2.1: all command execution is now async
- Switch `sqlx` dependency to `msqlx` fork
- Remove `Drop` implementations that used blocking IO
- Change to composable container exec API via `ociman`
- Change to composable command builder API
- Change to `ociman::backend::Selection` based config
- Change to `instance-list` subcommand (renamed)
- Change to toplevel instance commands
- Remove `--flavor` option (use explicit backend autodetection)

### Added

- Boot from snapshot support
- Configurable `wait_available_timeout`
- Seed caching: populate and boot from cached container images (`populate_cache`, `cache status`)
- Cache seed infrastructure
- Config file support for git seeds
- Config file support for command and script seeds
- GCP Cloud SQL URL support
- Ruby gem integration with cross-platform builds
- Optional SSL/TLS support
- Container password reset API
- `run-env` subcommand
- `--no-config-file` option
- Platform support APIs (amd64, arm64)
- Explicit image support with digest verification

### Changed

- Change to `PGDATA` outside of volume
- Change to minimal backtrace support builds
- Upgrade to `nom` 8
- Upgrade to `rcgen` 0.14
- Change to consolidated ruby build
- Change to consistent instance name type use

### Fixed

- Fix local test concurrency on ociman dependencies
- Fix hostname resolution
