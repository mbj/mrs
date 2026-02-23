# Changelog

## 0.0.1

- Initial release

### Added

- PostgreSQL schema migration management with filesystem-based migrations
- Migration state stored as `COMMENT ON TABLE` in schema DDL
- Strict consecutive indexing with `{index}_{name}.sql` naming
- `SchemaDump` and `SchemaNormalizer` traits for schema version control
- `apply_pending` and `create_new_pending` operations
- Optional schema dump/normalization for git-friendly version control
