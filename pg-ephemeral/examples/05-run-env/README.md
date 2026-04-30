# 05 — `run-env`: hand the DB to another tool

`pg-ephemeral run-env` boots an instance, sets every connection-related
environment variable, and execs the command you give it. When the command
exits, the container is stopped and removed.

This is the universal integration: any language's test runner, migration
tool, or CLI that reads libpq env vars or `DATABASE_URL` just works.

## Variables that get set

- libpq-style: `PGHOST`, `PGPORT`, `PGUSER`, `PGDATABASE`, `PGPASSWORD`,
  `PGSSLMODE` (and `PGSSLROOTCERT` etc. when SSL is enabled)
- `DATABASE_URL` — full PostgreSQL URL form

## Run

```sh
cd examples/05-run-env

# Run the bundled smoke script.
pg-ephemeral run-env -- ./run-tests.sh

# Run any other tool the same way.
pg-ephemeral run-env -- pytest
pg-ephemeral run-env -- npx prisma migrate deploy
pg-ephemeral run-env -- cargo test --features integration
pg-ephemeral run-env -- go test ./...
```

The `--` is the standard shell separator; pg-ephemeral itself doesn't require
it, but it's good practice to make sure flags after it go to your command,
not to pg-ephemeral.

## Exit code

The exit code of `run-env` is the exit code of the wrapped command. CI just
treats it like running the command directly.

## What this demonstrates

- Using pg-ephemeral as a fixture wrapper around any process.
- The full set of env vars exposed to child processes.
- That seeds run before the wrapped command starts — `run-tests.sh` sees a
  fully-populated database immediately.
