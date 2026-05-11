# 06 — Installing extensions with `container-script` + `parameters`

The official PostgreSQL Docker images ship contrib extensions but not
third-party ones like `pg_cron`, PostGIS, or pgvector. Some of those
extensions also need `shared_preload_libraries` set before the server
starts. This example demonstrates the clean split:

- **`container-script`** for the OS-side install (package needs to be on disk
  before PG can load its shared library).
- **`parameters`** table for the PG-side configuration (`shared_preload_libraries`
  becomes a `-c` flag on the postgres command).

## Four declarations, three roles

1. `[instances.main.parameters]` — sets `shared_preload_libraries = "pg_cron"`.
   pg-ephemeral injects this as `-c shared_preload_libraries=pg_cron` at
   `postgres` start. No `postgresql.conf` editing involved.
2. `install-pg-cron` — `container-script`, runs **in** the container while
   PostgreSQL is stopped. Installs the Debian package so the shared library
   file is on disk by the time PG tries to load it.
3. `enable-pg-cron` — `script`, runs **on the host** after PostgreSQL is up
   (so `shared_preload_libraries` is already loaded). Calls
   `CREATE EXTENSION pg_cron`.
4. `schedule-job` — `sql-statement`, runs **on the host** via `psql` and
   schedules a cron job to prove the extension is live.

## Run

```sh
cd examples/06-container-script-pg-cron

# First run downloads the package; rebuilds the cached image.
pg-ephemeral

# Subsequent runs hit the cache and start in <1s with pg_cron preinstalled.
pg-ephemeral
```

```
postgres=# SELECT jobname, schedule FROM cron.job;
 jobname |  schedule
---------+-------------
 cleanup | 0 3 * * *
```

## What this demonstrates

- `container-script` for image customizations that must happen with the
  database stopped (package install).
- `[instances.main.parameters]` for PG settings — no `postgresql.conf`
  editing or `/docker-entrypoint-initdb.d/` staging needed.
- Cache layering: the heavy `apt-get install` only runs the first time;
  every later seed step builds on the cached image.

## Caching note

`parameters` are folded into the cache key chain — changing a parameter
value invalidates every cache layer for the instance, same as changing
the base image or SSL config. `container-script` seeds are cached by
their script content. See `07-seed-cache` for cache mechanics.
