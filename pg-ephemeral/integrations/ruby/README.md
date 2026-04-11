# pg-ephemeral Ruby Gem

Ruby wrapper for [pg-ephemeral](https://github.com/mbj/mrs/tree/main/pg-ephemeral). Bundles the platform-specific binary
and provides a native API for ephemeral PostgreSQL instances.

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
    @server = PgEphemeral.start # prevent GC from closing the control FD
    @connection = PG.connect(@server.url)
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
    # @server prevents GC from closing the control FD
    @server = PgEphemeral.start
    ENV['DATABASE_URL'] = @server.url
    ActiveRecord::Base.establish_connection
  end
end
```

Schema and seed data are applied via `database.toml` seeds, so the database
is ready before the suite starts.

### Minitest

```ruby
class UserRepositoryTest < Minitest::Test
  def setup
    @server = PgEphemeral.start
    @connection = PG.connect(@server.url)
  end

  def test_inserts_a_user
    @connection.exec("INSERT INTO users (name) VALUES ('alice')")
    result = @connection.exec("SELECT name FROM users")
    assert_equal 'alice', result.first['name']
  end
end
```

## Requirements

- Ruby >= 3.3
- Docker Engine 20.10+ / Docker Desktop 4.34+, or Podman 5.3+
