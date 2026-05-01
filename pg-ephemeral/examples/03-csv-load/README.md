# 03 — CSV / TSV loading

Loads tabular data via `csv-file` seeds, which use `COPY ... FROM STDIN` under
the hood.

The first row of the file must be column headers matching the target table's
column names. Columns may appear in any order, and any column omitted from the
header (e.g. `users.id`, `users.created_at`) falls back to its table default.

## Files

- `schema.sql` — defines `users` and `events`.
- `users.csv` — comma-separated; supplies `name` and `email` while
  `id` and `created_at` fall back to their table defaults.
- `events.tsv` — tab-separated; `delimiter = "\t"` overrides the default `,`.

## Run

```sh
cd examples/03-csv-load
pg-ephemeral
```

```
postgres=# SELECT u.name, count(e.*) FROM users u
postgres-# LEFT JOIN events e ON e.user_id = u.id
postgres-# GROUP BY u.name ORDER BY u.name;
 name  | count
-------+-------
 alice |     2
 bob   |     1
 carol |     1
```

## What this demonstrates

- `csv-file` seeds with default `,` delimiter.
- `csv-file` with a non-default delimiter (`\t`).
- Header-driven column mapping — columns can be omitted in the file when the
  table has a usable default.
- The line delimiter is hardcoded to `\n`; CRLF files will not parse cleanly.
