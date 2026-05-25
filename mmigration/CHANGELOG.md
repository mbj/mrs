# Changelog

## Unreleased

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.0.1

- Initial release

### Added

- PostgreSQL schema migration management with filesystem-based migrations
- Migration state stored as `COMMENT ON TABLE` in schema DDL (travels with `pg_dump --schema-only`)
- Strict consecutive `{index}_{name}.sql` naming; migrations start at 1, with index 0 as the bootstrap baseline
- Bootstrap, apply, schema-dump, list-pending, and new-migration commands
- Each migration applied in its own transaction, with concurrent runs coordinated by an exclusive lock on the tracking table
- Pluggable schema source with optional normalization for git-friendly schema dumps
