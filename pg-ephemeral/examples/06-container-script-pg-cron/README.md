# 06 — Installing extensions with `container-script`

The official PostgreSQL Docker images ship contrib extensions but not
third-party ones like `pg_cron`, PostGIS, or pgvector. The `container-script`
seed type runs a shell script **inside** the container, with PostgreSQL
**stopped**, so installed packages get baked into the seed-cache image as
regular layers and persist across runs.

Some extensions (pg_cron, pgaudit, etc.) require `shared_preload_libraries`,
which has to be in `postgresql.conf` *before* the server starts. The trick:
write a shell snippet to `/docker-entrypoint-initdb.d/` from the
container-script — the official entrypoint runs everything in that directory
on first boot, and that's still before the server takes connections.

## Three seeds, three contexts

1. `install-pg-cron` — `container-script`, runs **in** the container while
   PostgreSQL is stopped. Installs the package, drops in the init script.
2. `enable-pg-cron` — `script`, runs **on the host** after PostgreSQL is up
   (so `shared_preload_libraries` is already loaded). Calls
   `CREATE EXTENSION pg_cron`.
3. `schedule-job` — `sql-statement`, runs **on the host** via `psql` and
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
  database stopped.
- The `shared_preload_libraries` init-script pattern.
- Cache layering: the heavy `apt-get install` only runs the first time;
  every later seed step builds on the cached image.

## Caching note

`container-script` seeds are cached by their script content. Change one byte
of the script and the next run rebuilds. See `07-seed-cache` for cache mechanics.
