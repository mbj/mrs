# pg-ephemeral Ruby Gem

Ruby wrapper for [pg-ephemeral](https://github.com/mbj/mrs/tree/main/pg-ephemeral). Bundles the platform-specific binary
and provides a native API for ephemeral PostgreSQL instances.

Published on RubyGems: [pg-ephemeral](https://rubygems.org/gems/pg-ephemeral)

## Installation

```ruby
gem 'pg-ephemeral'
```

The gem ships with prebuilt binaries for `x86_64-linux`, `aarch64-linux`, and `arm64-darwin`.

## Usage

### Direct connection

```ruby
PgEphemeral.with_connection do |conn|
  conn.exec("SELECT 1 AS value")
end
```

`with_connection` yields a `PG::Connection` and closes it after the block.

### Server handle

```ruby
PgEphemeral.with_server do |server|
  puts server.url  # => "postgres://postgres:...@127.0.0.1:54321/postgres"
end
```

`with_server` yields a `Server` with a `.url` accessor. The container shuts down
after the block.

### Schema dumps

`Server#schema_dump` runs the container's `pg_dump --schema-only` against the
running server and returns the SQL. It includes schema changes made after the
server starts. Additional arguments are passed directly to `pg_dump`:

```ruby
PgEphemeral.with_server do |server|
  sql = server.schema_dump('--no-owner', '--no-privileges', '--exclude-schema=pghero')
  File.write('structure.sql', sql)
end
```

This dumps the server's default database. Pass an existing database name as the
last argument, such as `server.schema_dump('--no-owner', 'app')`, to dump another
database on the same server. Dump output is returned to Ruby; write the returned
string to a host file rather than passing `--file`, which refers to the container
filesystem.

### Options

Both `with_connection` and `with_server` accept:

| Option          | Description                                      | Default    |
|-----------------|--------------------------------------------------|------------|
| `instance_name` | Target instance from `database.toml`             | `"main"`   |
| `config`        | Path to a `database.toml` config file            | auto-detect |

```ruby
PgEphemeral.with_connection(instance_name: "analytics", config: "path/to/database.toml") do |conn|
  conn.exec("SELECT 1")
end
```

### Manual lifecycle

For cases where the block form doesn't fit:

```ruby
server = PgEphemeral.start
connection = PG.connect(server.url)
# ...
connection.close
server.shutdown
```

### Utilities

```ruby
PgEphemeral.version             # => "0.2.0"
PgEphemeral.platform_supported? # => true
PgEphemeral.binary_path         # => "/path/to/pg-ephemeral"
```

## Test Framework Integration

### RSpec

```ruby
RSpec.describe UserRepository do
  before(:all) do
    @server = PgEphemeral.start
    @connection = PG.connect(@server.url)
  end

  after(:all) do
    @connection.close
    @server.shutdown
  end

  it 'inserts a user' do
    @connection.exec("INSERT INTO users (name) VALUES ('alice')")
    result = @connection.exec("SELECT name FROM users")
    expect(result.first['name']).to eq('alice')
  end
end
```

### Rails

Configure pg-ephemeral in `spec/support/pg_ephemeral.rb` so ActiveRecord
connects to the ephemeral instance:

```ruby
RSpec.configure do |config|
  config.before(:suite) do
    @server = PgEphemeral.start
    ENV['DATABASE_URL'] = @server.url
    ActiveRecord::Base.establish_connection
  end

  config.after(:suite) do
    @server.shutdown
  end
end
```

Schema and seed data are applied via `database.toml` seeds, so the database
is ready before the suite starts.

### Minitest

```ruby
# test/test_helper.rb
SERVER = PgEphemeral.start
CONNECTION = PG.connect(SERVER.url)

Minitest.after_run do
  CONNECTION.close
  SERVER.shutdown
end
```

```ruby
class UserRepositoryTest < Minitest::Test
  def test_inserts_a_user
    CONNECTION.exec("INSERT INTO users (name) VALUES ('alice')")
    result = CONNECTION.exec("SELECT name FROM users")
    assert_equal 'alice', result.first['name']
  end
end
```

## Requirements

- Ruby >= 3.3
- Docker Engine 20.10+ / Docker Desktop 4.34+, or Podman 5.3+
