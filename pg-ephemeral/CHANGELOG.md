# Changelog

## 0.3.1

### Added

- New `sql-statement` seed type for embedding SQL directly in `database.toml`,
  without needing a separate file on disk.

## 0.3.0

### Fixed

- `key-script` and `key-command` cache strategies now correctly invalidate when
  the seed's own command or script body changes. Previously the key script's
  output replaced the command hash entirely, so edits to the command being run
  were silently ignored by the cache.
- `key-script` and `key-command` caching can now be attached to `type =
  "script"` seeds. Previously the `cache = { ... }` field on a script seed
  was silently dropped during config parsing and had no effect at all.
- `cache` field on seed types that don't support it (`sql-file`, `csv-file`,
  `container-script`, ...) is now a loud config error instead of being silently
  ignored.
- CLI errors now print the full thiserror-formatted cause chain instead of a
  raw `Debug` dump of nested enum variants.
- Suppress spurious `PID N is not a PostgreSQL backend process` warnings when
  committing a container. Connection termination now targets only client
  backends instead of every entry in `pg_stat_activity`, which also included
  background workers and auxiliary processes.

### Breaking Changes

- `CommandCacheConfig` renamed to `SeedCacheConfig` since it now applies to
  both `command` and `script` seeds.
- `Definition::apply_script` now takes an explicit `SeedCacheConfig` argument
  to match `apply_command`.
- `Seed::Script` variant gained a `cache: SeedCacheConfig` field.
- `LoadError::KeyCommand` and `LoadError::KeyScript` now carry the underlying
  `cmd_proc::CommandError` via `#[source]` instead of a stringified `message`,
  so walkers can drill into the real cause.

## 0.2.3

### Breaking Changes

- npm package now requires Node.js >= 22.15.0 (was >= 20). The CLI wrapper
  relies on `process.execve`, which was backported to Node 22.15.0 and added in
  Node 23.11.0.

### Added

- Restore `csv-file` seed type for importing CSV data into PostgreSQL tables
  using the native COPY protocol. The first row must be column headers
  matching the target table; columns may appear in any order and omitted
  columns use their table defaults. The column delimiter defaults to `,` and
  can be overridden with `delimiter`. Also restores `Definition::apply_csv_file`
  and `Container::apply_csv`.

### Fixed

- Seeds defined in `database.toml` now run in the order they appear in the
  file. The `toml` crate deserialized tables through a sorted map, so seeds
  were silently executed in alphabetic order even though the README promised
  declaration order. The builder API (`apply_file`, `apply_script`, ...) was
  unaffected.
- Ship the CLI wrapper with the published npm package so `npx pg-ephemeral`
  works out of the box (previously the `bin` entry pointed to a file that was
  never included in the tarball)
- Ship README with the published npm package so it renders on npmjs.com
- Ship README with the published Ruby gem platform packages so it renders on
  rubygems.org

## 0.2.2

### Breaking Changes

- Track `pg-client` 0.2.0: session parameters now live in
  `pg_client::config::Session`, and connection types (`Endpoint`, `Host`,
  `HostName`, `HostAddr`, `Port`, `Password`, `ApplicationName`, `SslMode`,
  `SslRootCert`) have moved into the `pg_client::config` module. Public
  `Instance`, `Definition`, and `container::Definition` fields holding these
  types have been re-typed accordingly.

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
