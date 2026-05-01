# 04 — Multiple instances

`database.toml` defines two named instances, `app` and `legacy`. Top-level
fields (`image`, `wait_available_timeout`) are defaults inherited by every
instance; per-instance fields override them.

Without `--instance`, commands target the conventional `main` instance — but
this config has no `main`, so commands without `--instance` will fail with
`Unknown instance: main`. That's intentional: pick the instance you mean.

## Run

```sh
cd examples/04-multi-instance

# List the configured instances.
pg-ephemeral list
# app
# legacy

# psql against the app instance (PostgreSQL 17.1, schema + fixtures).
pg-ephemeral --instance app

# psql against the legacy instance (PostgreSQL 16.6, schema only).
pg-ephemeral --instance legacy

# Cache management is also per-instance.
pg-ephemeral cache status --instance app
pg-ephemeral cache reset  --instance legacy
```

## What this demonstrates

- Multiple named instances under `[instances.<name>]`.
- Top-level fields acting as defaults (image, timeout).
- Per-instance overrides (`legacy` pins a different image).
- `--instance <NAME>` selecting which instance a command operates on.
- That `--instance` is **per-subcommand**, e.g. `cache status --instance app`.

## When to use

- A service that needs both a primary DB and a separate analytics DB in tests.
- Compatibility testing across PostgreSQL versions.
- Comparing the same migration applied to two seed states.
