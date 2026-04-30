# 02 — SQL file + inline statement seeds

Two seeds run in declaration order:

1. `schema` — applies `schema.sql` (a `sql-file` seed). File paths are
   resolved relative to `database.toml`, not to the current working directory.
2. `fixtures` — runs an inline `INSERT` (a `sql-statement` seed). Use this
   when the SQL is short enough that a separate file is overkill.

## Run

```sh
cd examples/02-sql-file-seed
pg-ephemeral
```

You'll land in `psql`. Both seeds have already run:

```
postgres=# SELECT * FROM users ORDER BY id;
 id | name
----+-------
  1 | alice
  2 | bob
  3 | carol
```

You can run from a sibling directory just as well — paths are config-relative:

```sh
pg-ephemeral --config-file examples/02-sql-file-seed/database.toml
```

## What this demonstrates

- `sql-file` seeds and config-relative path resolution.
- `sql-statement` seeds for short embedded SQL.
- Seed declaration order = execution order.
- Both seeds are cached automatically; see `07-seed-cache` for cache mechanics.
