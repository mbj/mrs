# pg-ephemeral - Ephemeral PostgreSQL for Testing

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

Spin up throwaway PostgreSQL containers for development and testing. Supports Docker and
Podman with automatic backend detection.

## Quick Start

```sh
# Launch psql against an ephemeral database (default command)
pg-ephemeral

# Run a command with PG* environment variables set
pg-ephemeral run-env -- pytest

# Run an interactive shell on the container
pg-ephemeral container-shell
```

Without a config file pg-ephemeral creates a single `main` instance using the latest
supported PostgreSQL image on the auto-detected container backend.

## Configuration

Place a `database.toml` in the working directory (or pass `--config-file <path>`):

```toml
image = "17.1"

[instances.main.seeds.a-schema]
type = "sql-file"
path = "schema.sql"

[instances.main.seeds.b-data]
type = "script"
script = "psql -c \"INSERT INTO users (name) VALUES ('alice'), ('bob')\""

[instances.main.seeds.c-indexes]
type = "sql-file"
path = "indexes.sql"

[instances.main.seeds.d-dynamic]
type = "command"
command = "sh"
arguments = ["-c", "psql -c \"INSERT INTO users (name) VALUES ('dynamic-$RANDOM')\""]
cache = { type = "none" }
```

### Top-level fields

| Field                    | Description                                                          |
|--------------------------|----------------------------------------------------------------------|
| `image`                  | PostgreSQL version / image tag (e.g. `"17.1"`)                       |
| `backend`                | `"docker"`, `"podman"`, or omit for auto-detection                   |
| `ssl_config`             | SSL configuration with `hostname` field                              |
| `wait_available_timeout` | How long to wait for PostgreSQL to accept connections (e.g. `"30s"`) |

### Seed types

Seeds run in declaration order inside the container. Each seed has a `type`:

| Type       | Fields                          | Description                                                                 |
|------------|---------------------------------|-----------------------------------------------------------------------------|
| `sql-file` | `path`, optional `git_revision` | Apply a SQL file. With `git_revision`, reads the file from that git commit. |
| `script`   | `script`                        | Run a shell script with `sh -e -c`.                                         |
| `command`  | `command`, `arguments`, `cache` | Run an arbitrary command.                                                   |

### Multiple instances

Define multiple named instances under `[instances.<name>]`. Top-level fields serve as
defaults for all instances. Use `--instance <name>` on the CLI to target a specific one.

## Seed Caching

pg-ephemeral caches seed results as container images so repeated runs skip already-applied
seeds. Each seed's cache key is a SHA-256 chain of:

- pg-ephemeral version
- base image
- SSL configuration
- all preceding seeds' content

When the cache key matches an existing image the seed is a **hit** and the container boots
from that image directly. Seeds are cached in order; an uncacheable seed (e.g.
`cache = { type = "none" }`) breaks the chain and all subsequent seeds run without caching.

### Cache commands

```sh
# Show cache status for all seeds
pg-ephemeral cache status

# JSON output with full details (references, etc.)
pg-ephemeral cache status --json

# Pre-populate the cache without running an interactive session
pg-ephemeral cache populate

# Remove cached images
pg-ephemeral cache reset

# Force-remove cached images (even if referenced by stopped containers)
pg-ephemeral cache reset --force
```

### Command cache strategies

For `command` type seeds, the `cache` field controls how the cache key is computed:

| Strategy                                                       | Description                                                                |
|----------------------------------------------------------------|----------------------------------------------------------------------------|
| `{ type = "command-hash" }`                                    | Hash the command and arguments (default).                                  |
| `{ type = "key-command", command = "...", arguments = [...] }` | Run a separate command whose stdout is hashed as the cache key.            |
| `{ type = "key-script", script = "..." }`                      | Run a script whose stdout is hashed as the cache key.                      |
| `{ type = "none" }`                                            | Disable caching. Breaks the cache chain for this and all subsequent seeds. |

## Rust Library

pg-ephemeral can be used as a Rust library for integration tests or any code that needs
a throwaway PostgreSQL instance.

### Basic usage

```rust,no_run
async fn example() {
    let backend = ociman::backend::resolve::auto().await.unwrap();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .apply_file(
        "schema".parse().unwrap(),
        "schema.sql".into(),
    )
    .unwrap()
    .apply_script(
        "seed-data".parse().unwrap(),
        r#"psql -c "INSERT INTO users (name) VALUES ('alice')""#,
    )
    .unwrap();

    definition
        .with_container(async |container| {
            container
                .with_connection(async |conn| {
                    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM users")
                        .fetch_one(&mut *conn)
                        .await
                        .unwrap();
                    assert_eq!(row.0, 1);
                })
                .await;
        })
        .await
        .unwrap();
}
```

`with_container` handles the full lifecycle: populate the seed cache, boot a container
(from the latest cache hit if available), apply any remaining uncached seeds, run the
closure, and stop the container.

### Seed types

Seeds are added to a `Definition` via builder methods:

```rust,no_run
# async fn example() {
# let backend = ociman::backend::resolve::auto().await.unwrap();
let definition = pg_ephemeral::Definition::new(
    backend,
    pg_ephemeral::Image::default(),
    "test".parse().unwrap(),
)
// Apply a SQL file from disk
.apply_file("schema".parse().unwrap(), "schema.sql".into())
.unwrap()
// Apply a SQL file from a specific git revision
.apply_file_from_git_revision(
    "baseline".parse().unwrap(),
    "schema.sql".into(),
    "abc1234",
)
.unwrap()
// Run an inline shell script
.apply_script(
    "seed-data".parse().unwrap(),
    r#"psql -c "INSERT INTO users (name) VALUES ('alice')""#,
)
.unwrap()
// Run an arbitrary command with explicit cache strategy
.apply_command(
    "migrations".parse().unwrap(),
    pg_ephemeral::Command::new("migrate", ["up"]),
    pg_ephemeral::CommandCacheConfig::CommandHash,
)
.unwrap();
# }
```

### Configuration

The `Definition` builder supports additional options:

```rust,no_run
# async fn example() {
# let backend = ociman::backend::resolve::auto().await.unwrap();
let definition = pg_ephemeral::Definition::new(
    backend,
    "17.1".parse().unwrap(),
    "test".parse().unwrap(),
)
// Extend the timeout for slow CI environments
.wait_available_timeout(std::time::Duration::from_secs(30))
// Enable cross-container access (for testing from other containers)
.cross_container_access(true)
// Enable SSL with a generated certificate
.ssl_config(pg_ephemeral::definition::SslConfig::Generated {
    hostname: "localhost".parse().unwrap(),
});
# }
```

### Accessing connection details

Inside `with_container`, the `Container` provides several ways to connect:

```rust,no_run
# async fn example() {
# let backend = ociman::backend::resolve::auto().await.unwrap();
# let definition = pg_ephemeral::Definition::new(
#     backend, pg_ephemeral::Image::default(), "test".parse().unwrap(),
# );
definition
    .with_container(async |container| {
        // Direct sqlx connection
        container
            .with_connection(async |conn| {
                sqlx::query("SELECT 1").execute(&mut *conn).await.unwrap();
            })
            .await;

        // Get pg_client::Config for custom connection setup
        let _config = container.client_config();

        // Get libpq-style environment variables (PGHOST, PGPORT, etc.)
        let _env = container.pg_env();

        // Get DATABASE_URL string
        let _url = container.database_url();
    })
    .await
    .unwrap();
# }
```

## Language Integrations

### Ruby

The `pg-ephemeral` Ruby gem bundles the binary and provides a native API:

```ruby
# Yields a PG::Connection to an ephemeral database
PgEphemeral.with_connection do |conn|
  conn.exec("SELECT 1")
end

# Or get the server URL for manual connection management
PgEphemeral.with_server do |server|
  puts server.url  # => "postgres://postgres:...@127.0.0.1:54321/postgres"
end
```

The gem is available for `x86_64-linux`, `aarch64-linux`, and `arm64-darwin`.

See [integrations/ruby](integrations/ruby/) for details.

### Other Languages

Any language can integrate via `run-env` or the integration server protocol:

**Environment variables** — run a command with `PG*` and `DATABASE_URL` set:

```sh
pg-ephemeral run-env -- python manage.py test
pg-ephemeral run-env -- npx prisma migrate deploy
```

**Integration server** — for programmatic control over the container lifecycle:

```sh
pg-ephemeral integration-server --protocol v0
```

Boots a container, prints a JSON line with connection details to stdout, then waits for
EOF on stdin before shutting down. Close stdin to stop the server.

## CLI Reference

```
pg-ephemeral [OPTIONS] [COMMAND]

Commands:
  psql                 Run interactive psql on the host (default)
  run-env              Run a command with PG* and DATABASE_URL environment variables
  container-psql       Run interactive psql inside the container
  container-shell      Run interactive shell inside the container
  container-schema-dump  Dump schema from the container
  cache                Cache management (status, populate, reset)
  integration-server   Run integration server (JSON over stdin/stdout)
  list                 List defined instances
  platform             Platform support checks

Options:
  --config-file <PATH>   Config file path (default: database.toml)
  --no-config-file       Use defaults, ignore any config file
  --backend <BACKEND>    Override backend (docker, podman)
  --image <IMAGE>        Override PostgreSQL image
  --ssl-hostname <HOST>  Enable SSL with the specified hostname
  --instance <NAME>      Target instance (default: main)
```

## Requirements

- Docker Engine 20.10+ / Docker Desktop 4.34+, or Podman 5.3+
- PostgreSQL client tools (`psql`) for host-side commands

## Release Build Configuration

Release builds use `split-debuginfo = "packed"` to separate debug information from the binary:

- **Linux**: Debug info stored in `.dwp` file alongside the binary
- **macOS**: Debug info stored in `.dSYM` bundle alongside the binary

This provides smaller binaries while preserving full backtraces with file paths and line numbers.
