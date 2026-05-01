# pg-ephemeral examples

Self-contained examples of common pg-ephemeral workflows. Each subdirectory
holds a runnable `database.toml` (where applicable), the supporting files it
references, and a `README.md` that walks through the workflow and the
relevant CLI semantics.

The examples build on each other but each one stands alone — pick the one
closest to what you need and copy it.

## Index

| Example                               | Demonstrates                                                                |
|---------------------------------------|-----------------------------------------------------------------------------|
| [01-default](01-default/)             | Zero-config: `pg-ephemeral` with no `database.toml`. Default subcommand.    |
| [02-sql-file-seed](02-sql-file-seed/) | `sql-file` and `sql-statement` seeds; config-relative path resolution.      |
| [03-csv-load](03-csv-load/)           | `csv-file` seeds for tabular data; default and custom delimiters.           |
| [04-multi-instance](04-multi-instance/) | Multiple named instances, top-level defaults, `--instance <NAME>`.        |
| [05-run-env](05-run-env/)             | `run-env` as the universal integration for any test runner / migrator.     |
| [06-container-script-pg-cron](06-container-script-pg-cron/) | Installing `pg_cron` (or any extension) via `container-script`. |
| [07-seed-cache](07-seed-cache/)       | Cache mechanics: `status`, `populate`, `reset`, and the cache-key chain.    |
| [08-ssl](08-ssl/)                     | SSL with auto-generated CA + verify-full.                                   |

## Prerequisites

All examples assume:

- `pg-ephemeral` is on your `PATH` (`cargo install pg-ephemeral`, or build
  this workspace with `cargo build -p pg-ephemeral` and put `target/debug/`
  on `PATH`).
- A working Docker or Podman installation. pg-ephemeral auto-detects; see
  the top-level [README](../README.md#backend-selection) for overrides.
- Local `psql` for any host-side commands.

## Running an example

Each example has a `database.toml` next to its supporting files. Either `cd`
into the directory:

```sh
cd examples/02-sql-file-seed
pg-ephemeral
```

…or point `--config-file` at it from anywhere:

```sh
pg-ephemeral --config-file examples/02-sql-file-seed/database.toml
```

File paths inside the TOML are resolved relative to the config file, so both
forms behave identically.

## CLI cheatsheet

The full reference lives in the top-level [README](../README.md#cli-reference).
The subcommands these examples actually use:

| Command                              | What it does                                                    |
|--------------------------------------|-----------------------------------------------------------------|
| `pg-ephemeral`                       | Default. Boots an instance and opens host `psql`.               |
| `pg-ephemeral run-env -- <cmd>`      | Boots an instance, runs `<cmd>` with PG\* and DATABASE_URL set. |
| `pg-ephemeral container-psql`        | `psql` from inside the container instead of the host.           |
| `pg-ephemeral container-shell`       | Interactive shell in the container.                             |
| `pg-ephemeral container-schema-dump` | Dump schema from the container to stdout.                       |
| `pg-ephemeral list`                  | Print configured instance names.                                |
| `pg-ephemeral cache status`          | Show cache state per seed (`--json` for tooling).               |
| `pg-ephemeral cache credentials`     | Print credentials baked into a cached seed image (no boot).     |
| `pg-ephemeral cache inspect <ref>`   | Print full pg-ephemeral metadata for a cached image.            |
| `pg-ephemeral cache populate`        | Build the cache without opening a session.                      |
| `pg-ephemeral cache reset`           | Remove cached images (`--force` for in-use ones).               |

Global flags worth knowing:

| Flag                         | Effect                                                |
|------------------------------|-------------------------------------------------------|
| `--config-file <PATH>`       | Use `<PATH>` instead of `./database.toml`.            |
| `--no-config-file`           | Ignore any config file; use defaults.                 |
| `--instance <NAME>`          | Per-subcommand: target this named instance.           |
| `--backend docker\|podman`   | Override auto-detection.                              |
| `--image <TAG>`              | Override the PostgreSQL image.                        |
| `--ssl-hostname <HOST>`      | Enable SSL with the given hostname.                   |
