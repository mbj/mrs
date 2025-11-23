# pg-ephemeral

Ephemeral PostgreSQL instances for testing.

## Features

- Spin up isolated PostgreSQL containers with Docker or Podman
- Automatic container lifecycle management
- SSL/TLS support with certificate generation
- Database seeding (SQL files, migrations, scripts, commands)
- Integration server mode for non-Rust language bindings

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Start psql session with ephemeral database (no config needed)
pg-ephemeral psql

# Run command with database environment variables
pg-ephemeral run-env -- pytest

# Start integration server (for language bindings)
pg-ephemeral integration-server --protocol v0

# Run migrations
pg-ephemeral migration apply
```

## Configuration (Optional)

Without a config file, pg-ephemeral uses the latest PostgreSQL image with auto-detected backend.

For more control, create a `database.toml`:

```toml
image = "17"
backend = "docker"  # or "podman", omit for auto-detection

[ssl_config]
hostname = "localhost"

[instances.main]
image = "17.6"

[instances.main.seeds.seed-data]
type = "sql-file"
path = "fixtures/seed.sql"
```

### Seed Types

```toml
# SQL file
[instances.main.seeds.schema]
type = "sql-file"
path = "schema.sql"

# SQL from git revision
[instances.main.seeds.old-schema]
type = "sql-file"
path = "schema.sql"
git_revision = "HEAD~1"

# Shell command
[instances.main.seeds.migrate]
type = "command"
command = "diesel"
arguments = ["migration", "run"]

# Inline script
[instances.main.seeds.setup]
type = "script"
script = "psql -c 'CREATE EXTENSION pgcrypto'"
```

### CLI Options

```sh
pg-ephemeral --help
pg-ephemeral --config-file custom.toml psql
pg-ephemeral --backend podman --image 16 psql
pg-ephemeral --ssl-hostname localhost psql
pg-ephemeral instance --name other-db psql
```

## Requirements

- Docker or Podman

## Library Usage

```rust
use pg_ephemeral::{Definition, BackendSelection, Image};

let definition = Definition::new(BackendSelection::Auto, Image::default());

definition.with_container(async |container| {
    let config = container.client_config();
    // Use the database...
}).await;
```

## License

See workspace root for license information.
