# mmigration - PostgreSQL Migration Manager

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

PostgreSQL schema migration management.

## Why not sqlx/diesel/refinery migrations?

- **State in DDL, not data**: Migration state is stored as a `COMMENT ON TABLE` in the schema DDL itself - when you pg_dump, the migration state comes with it
- **Strict consecutive indexing**: Migrations use `{index}_{name}.sql` naming with enforced consecutive indexes - no gaps, no timestamp drift
- **Optional schema dumps**: Can automatically dump and normalize schema after migrations for version control

## Git Integration

Designed for version-controlled schemas:

- Commit `schema.sql` alongside migration files - PRs show the actual schema diff
- Normalized dumps produce identical output across environments - clean git history
- Migration state visible in the schema file itself - no hidden tracking tables to query
- **Rebase-friendly**: Index collisions are obvious filename conflicts, forcing linear migration ordering - no silent timestamp-based race conditions
