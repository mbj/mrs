# 01 — Default (no config)

The simplest possible workflow: no `database.toml`, no flags. pg-ephemeral
starts a single instance named `main` with the default PostgreSQL image on the
auto-detected backend.

## Run

From any directory that does **not** contain a `database.toml`:

```sh
pg-ephemeral
```

This is shorthand for `pg-ephemeral psql`: a container is booted, an
interactive `psql` session is opened on the host (talking to the container),
and the container is removed when you exit psql.

You can also force the no-config path explicitly, even from a directory that
*does* contain a `database.toml`:

```sh
pg-ephemeral --no-config-file
```

## Inspect what got resolved

```sh
pg-ephemeral --no-config-file list
# main
```

## Override defaults from the CLI

```sh
# Pin a specific PostgreSQL image
pg-ephemeral --no-config-file --image 17.2

# Force a backend
pg-ephemeral --no-config-file --backend podman
```

## What this demonstrates

- The "zero-config" entry point.
- That `psql` is the default subcommand.
- That `--image` and `--backend` are CLI overrides usable without any config
  file.
