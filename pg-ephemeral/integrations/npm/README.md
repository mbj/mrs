# pg-ephemeral npm Package

Node.js wrapper for [pg-ephemeral](https://github.com/mbj/mrs/tree/main/pg-ephemeral). Installs the platform-specific binary
via optional dependencies and provides a TypeScript API for ephemeral PostgreSQL instances.

## Installation

```sh
npm install pg-ephemeral
```

Platform binaries are installed automatically for `linux-x64`, `linux-arm64`, and `darwin-arm64`.

## Usage

### Direct connection

```typescript
import { withConnection } from 'pg-ephemeral';

await withConnection(async (client) => {
  const result = await client.query('SELECT 1 AS value');
  console.log(result.rows[0].value);
});
```

`withConnection` yields a connected `pg.Client` and closes it after the callback.

### Server handle

```typescript
import { withServer } from 'pg-ephemeral';

await withServer(async (server) => {
  console.log(server.url);  // => "postgres://postgres:...@127.0.0.1:54321/postgres"
});
```

`withServer` yields a `Server` with a `.url` property. The container shuts down
after the callback.

### Options

Both `withConnection` and `withServer` accept an options object:

| Option         | Description                                      | Default    |
|----------------|--------------------------------------------------|------------|
| `instanceName` | Target instance from `database.toml`             | `"main"`   |
| `config`       | Path to a `database.toml` config file            | `null`     |

```typescript
await withConnection(async (client) => {
  await client.query('SELECT 1');
}, { instanceName: 'analytics', config: 'path/to/database.toml' });
```

### Manual lifecycle

For cases where the callback form doesn't fit:

```typescript
import { start } from 'pg-ephemeral';

const server = await start();
// ... use server.url ...
await server.shutdown();
```

### Utilities

```typescript
import { version, platformSupported, binaryPath } from 'pg-ephemeral';

version();            // => "0.2.0"
platformSupported();  // => true
binaryPath();         // => "/path/to/pg-ephemeral"
```

## Test Framework Integration

### Jest

Use `globalSetup` to start the container and share the URL via `process.env`.
The container shuts down automatically when the control process exits.

```typescript
// jest.globalSetup.ts
import { start } from 'pg-ephemeral';

export default async function globalSetup() {
  const server = await start();
  process.env.DATABASE_URL = server.url;
  // Anchor to globalThis to prevent GC from closing the control FD
  (globalThis as any).__pgEphemeralServer = server;
}
```

Then connect via the environment variable in your tests:

```typescript
import { Client } from 'pg';

let client: Client;

beforeAll(async () => {
  client = new Client({ connectionString: process.env.DATABASE_URL });
  await client.connect();
});

afterAll(async () => {
  await client.end();
});

it('inserts a user', async () => {
  await client.query("INSERT INTO users (name) VALUES ('alice')");
  const result = await client.query('SELECT name FROM users');
  expect(result.rows[0].name).toBe('alice');
});
```

### Prisma

Use `globalSetup` to start the container and deploy migrations before tests run:

```typescript
// jest.globalSetup.ts
import { start } from 'pg-ephemeral';
import { execSync } from 'node:child_process';

export default async function globalSetup() {
  const server = await start();
  process.env.DATABASE_URL = server.url;
  execSync('npx prisma migrate deploy', { env: process.env });
  // Anchor to globalThis to prevent GC from closing the control FD
  (globalThis as any).__pgEphemeralServer = server;
}
```

The container shuts down automatically when the control process exits. Then use
`PrismaClient` in your tests as usual:

```typescript
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

afterAll(async () => {
  await prisma.$disconnect();
});

it('creates a user', async () => {
  const user = await prisma.user.create({ data: { name: 'alice' } });
  expect(user.name).toBe('alice');
});
```

## Requirements

- Node.js >= 20
- Docker Engine 20.10+ / Docker Desktop 4.34+, or Podman 5.3+
