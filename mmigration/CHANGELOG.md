# Changelog

## 0.0.1

- Initial release

### Added

- PostgreSQL schema migration management with filesystem-based migrations
- Migration state stored as `COMMENT ON TABLE` in schema DDL (travels with `pg_dump --schema-only`)
- Strict consecutive `{index}_{name}.sql` naming; migrations start at 1, with index 0 as the bootstrap baseline
- Commands grouped by what they operate on: `database apply`, `database bootstrap`, `database dump-schema`, `database pending`, `database sync`, `database verify`, and `repository new <name>`
- Two mountable command surfaces: `operate::Command` for deployed binaries, offering database operations only, and `develop::Command`, which adds the commands that read and write a checkout, so a deployed binary need not expose migration authoring
- `database verify` detects when an already-applied migration file has been edited or removed, by checking a tamper-evident hash chain over the applied migrations that is recorded in the schema dump
- `database sync` applies every pending migration and then refreshes the schema file, the everyday development loop in one step, and is the only command that writes it
- `database dump-schema` writes to stdout, so it is safe to run against a deployed database
- Each migration applied in its own transaction, with concurrent runs coordinated by an exclusive lock on the tracking table
- Pluggable schema source with optional normalization for git-friendly schema dumps
- Requires Rust 1.95
