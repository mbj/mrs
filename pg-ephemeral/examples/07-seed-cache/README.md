# 07 — Seed cache mechanics

Every cacheable seed produces a content-addressed OCI image. The cache key
of seed N is a SHA-256 chain over:

- the pg-ephemeral version
- the base PostgreSQL image
- the SSL configuration
- the content of seeds 1..N

When the key for seed N already exists as an image, that seed is a cache
**hit** and the container can boot directly from it, skipping every preceding
seed. An uncacheable seed (`cache = { type = "none" }`, or anything after one)
is always re-run on every boot.

## The seeds in this example

| #   | Seed         | Cacheable                                     |
|-----|--------------|-----------------------------------------------|
| 1   | `schema`     | yes                                           |
| 2   | `fixtures`   | yes                                           |
| 3   | `timestamp`  | **no** — `cache = { type = "none" }`          |
| 4   | `afterwards` | no — chain broken by `timestamp`              |

## Walk-through

```sh
cd examples/07-seed-cache

# 1. First call: nothing cached.
pg-ephemeral cache status
# (all seeds reported as "miss")

# 2. Pre-populate the cache without opening psql.
pg-ephemeral cache populate

# 3. Status now shows hits for the cacheable seeds.
pg-ephemeral cache status

# 4. JSON output for tooling.
pg-ephemeral cache status --json | jq
```

JSON shape (after `cache populate`):

```json
{
  "instance": "main",
  "base_image": "17.1",
  "version": "0.4.0",
  "summary": {
    "total": 4,
    "hits": 2,
    "misses": 0,
    "uncacheable": 2
  },
  "seeds": [
    {
      "name": "schema",
      "type": "sql-file",
      "status": "hit",
      "cache_image": "pg-ephemeral/main:..."
    },
    {
      "name": "fixtures",
      "type": "sql-statement",
      "status": "hit",
      "cache_image": "pg-ephemeral/main:..."
    },
    {
      "name": "timestamp",
      "type": "script",
      "status": "uncacheable",
      "reason": "cache_strategy_none"
    },
    {
      "name": "afterwards",
      "type": "sql-statement",
      "status": "uncacheable",
      "reason": "chain_broken_by_predecessor",
      "broken_by": "timestamp"
    }
  ]
}
```

`cache credentials` (after `cache populate`) reads the credentials baked
into the deepest cached seed image — no container is booted:

```sh
pg-ephemeral cache credentials
# {
#   "cache_image": "pg-ephemeral/main:...",
#   "superuser": {
#     "user": "postgres",
#     "database": "postgres",
#     "password": "..."
#   }
# }

# Address an earlier layer by name:
pg-ephemeral cache credentials --seed-name schema
```

Useful when a CI script needs to know the password before booting a
container itself, or when you want to attach to an earlier baseline layer.
Fails with a clear message if the targeted seed is uncacheable, not yet
cached, or doesn't exist.

```sh
# 5. Open a session — boots from the cached fixtures image, only
#    `timestamp` and `afterwards` actually execute.
pg-ephemeral

# 6. Inspect the full pg-ephemeral metadata stored on a cached image
#    (superuser, seed chain, SSL, …). Use the reference shown in
#    `cache status --json`.
pg-ephemeral cache inspect pg-ephemeral/main:<sha>

# 7. Wipe the cached images.
pg-ephemeral cache reset

# 8. If a stopped container still references a cached image:
pg-ephemeral cache reset --force
```

`cache status` is intentionally lean — it answers "what's cached?" and
nothing more. Reach for `cache inspect <reference>` when you need the full
manifest (image labels, seed chain, generated CA cert, etc.).

## Cache strategies for `command` / `script` seeds

The seed's own content is always part of the cache key. The `cache` field
chooses what *additional* input gets folded in:

```toml
# Default: hash only the seed content.
cache = { type = "command-hash" }

# Run a separate command and fold its stdout into the key.
# Useful when the seed depends on files or git state.
cache = { type = "key-command", command = "git", arguments = ["rev-parse", "HEAD:db/structure.sql"] }

# Same idea, but with an inline shell script.
cache = { type = "key-script", script = "sha256sum migrations/*.sql | sha256sum" }

# Disable caching for this seed AND every seed after it.
cache = { type = "none" }
```

## What this demonstrates

- `cache status`, `cache populate`, `cache reset` (and `--force`).
- `cache status --json` for programmatic use, including the `summary`
  rollup and per-seed `reason` / `broken_by` fields for uncacheable entries.
- `cache credentials [--seed-name <SEED>]` for reading the credentials
  baked into a cached seed image without booting a container.
- `cache inspect <reference>` for the full image manifest.
- `cache = { type = "none" }` and how it breaks the chain.
- How to think about the cache key when designing seeds: anything that should
  invalidate the cache must end up in the key, either via the seed's own
  content or via `key-command` / `key-script`.
