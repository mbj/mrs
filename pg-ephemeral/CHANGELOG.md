# Changelog

## 0.2.2

### Fixed

- Checkpoint PostgreSQL after terminating seed connections so cached images
  always commit from a deterministic on-disk state
- Fail seeding hard when terminate-connections or checkpoint errors, instead of
  silently committing a potentially broken cache image

## 0.2.1

### Added

- README for Ruby gem integration
- README for npm package integration

### Fixed

- Terminate stale database connections before stopping containers

### Changed

- Lower config file resolution log messages from info to debug level

## 0.2.0

### Breaking Changes

- `InstanceName` field is now private. Use `from_static_or_panic`, `FromStr`,
  `TryFrom<String>`, or the `MAIN` constant to construct values.
- `InstanceName` now validates: lowercase ASCII alphanumeric and dashes only,
  no leading or trailing dashes.
- `InstanceName::FromStr::Err` changed from `Infallible` to `InstanceNameError`.
- `SeedNameError` changed from unit struct to enum with `Empty`, `TooLong`,
  `InvalidCharacter`, `StartsWithDash`, and `EndsWithDash` variants.
- `SeedName` now validates: lowercase ASCII alphanumeric and dashes only,
  no leading or trailing dashes, maximum 63 bytes.

### Added

- `InstanceName::from_static_or_panic` const constructor.
- `InstanceName::MAIN` constant for the default `"main"` instance name.
- `InstanceName::as_str` and `AsRef<str>` for `InstanceName`.
- `InstanceNameError` enum for `InstanceName` validation errors.
- `SeedName::from_static_or_panic` const constructor.
- `SeedName::AsRef<str>` implementation.
- `SEED_NAME_MAX_LENGTH` constant (63).

## 0.1.3

### Changed

- CSV seeds now use the header line to generate an explicit column list in the
  COPY statement, allowing columns in any order and omitted columns to use
  their table defaults.
- Added configurable column `delimiter` for CSV seeds (defaults to `,`).

## 0.1.2

### Added

- `csv-file` seed type for importing CSV data into PostgreSQL tables using the
  native COPY protocol via sqlx.
- `Definition::apply_csv_file` builder method
- `Container::apply_csv` method

## 0.1.1

### Changed

- Lower config file resolution log messages from info to debug level

## 0.1.0

- Bump version to 0.1.0
- Remove pre-1.0 stability warning from README

## 0.0.8

### Added (Ruby gem)

- `PgEphemeral.start` method that returns a `Server` directly without
  requiring a block, enabling use in frameworks that cannot wrap their
  lifecycle in a block.

### Changed (Ruby gem)

- Refactored `PgEphemeral.with_server` to delegate to `start`

## 0.0.7

### Added

- `container-script` seed type for running scripts inside the container without
  starting PostgreSQL. Useful for installing extensions (`apt-get install`) or
  other image customizations. Cached via `docker build` with a generated Dockerfile.
- `Definition::apply_container_script` builder method
- `SeedApplyError` error type for seed application failures

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
