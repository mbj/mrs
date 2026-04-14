import { describe, it } from 'node:test';
import * as assert from 'node:assert';
import * as path from 'node:path';
import { execFileSync } from 'node:child_process';
import * as pgEphemeral from 'pg-ephemeral';
import type { Server, StartOptions } from 'pg-ephemeral';
import type { Client } from 'pg';

// Compile-time type assertions validating the exported interface
const _typeCheckBinaryPath: () => string = pgEphemeral.binaryPath;
const _typeCheckVersion: () => string = pgEphemeral.version;
const _typeCheckPlatformSupported: () => boolean = pgEphemeral.platformSupported;
const _typeCheckStart: (opts?: StartOptions) => Promise<Server> = pgEphemeral.start;
const _typeCheckWithServer: <T>(fn: (server: Server) => Promise<T>, opts?: StartOptions) => Promise<T> = pgEphemeral.withServer;
const _typeCheckWithConnection: <T>(fn: (client: Client) => Promise<T>, opts?: StartOptions) => Promise<T> = pgEphemeral.withConnection;

describe('version', () => {
  it('returns the expected version', () => {
    const expected = process.env['EXPECTED_PG_EPHEMERAL_VERSION'];
    assert.strictEqual(pgEphemeral.version(), expected);
  });

  it('returns a semantic version format', () => {
    assert.match(pgEphemeral.version(), /^\d+\.\d+\.\d+(-.+)?$/);
  });
});

describe('bin wrapper', () => {
  it('is shipped in the main package tarball and prints the expected version', () => {
    const expected = process.env['EXPECTED_PG_EPHEMERAL_VERSION'];
    const binPath = require.resolve('pg-ephemeral/bin/pg-ephemeral.js');
    const output = execFileSync(process.execPath, [binPath, '--version'], { encoding: 'utf-8' });
    assert.strictEqual(output, `pg-ephemeral ${expected}\n`);
  });
});

describe('platformSupported', () => {
  it('returns a boolean', () => {
    assert.strictEqual(typeof pgEphemeral.platformSupported(), 'boolean');
  });
});

describe('start', { skip: !pgEphemeral.platformSupported() }, () => {
  it('returns a server with a url', async () => {
    const server: Server = await pgEphemeral.start();
    assert.match(server.url, /^postgres:\/\//);
    await server.shutdown();
  });

  it('supports custom instance names', async () => {
    const server: Server = await pgEphemeral.start({ instanceName: 'custom' });
    assert.ok(typeof server.url === 'string');
    await server.shutdown();
  });

  it('accepts a custom config file path', async () => {
    const configPath = path.join(__dirname, '..', '..', 'database.toml');
    const server: Server = await pgEphemeral.start({ config: configPath });
    assert.match(server.url, /^postgres:\/\//);
    await server.shutdown();
  });
});

describe('withServer', { skip: !pgEphemeral.platformSupported() }, () => {
  it('yields a server with url', async () => {
    await pgEphemeral.withServer(async (server: Server) => {
      assert.match(server.url, /^postgres:\/\//);
    });
  });

  it('returns the callback result', async () => {
    const result: number = await pgEphemeral.withServer(async () => 42);
    assert.strictEqual(result, 42);
  });
});

describe('withConnection', { skip: !pgEphemeral.platformSupported() }, () => {
  it('yields a connected pg client', async () => {
    await pgEphemeral.withConnection(async (client: Client) => {
      const result = await client.query('SELECT 1 AS value');
      assert.strictEqual(result.rows[0].value, 1);
    });
  });

  it('returns the callback result', async () => {
    const result: number = await pgEphemeral.withConnection(async () => 42);
    assert.strictEqual(result, 42);
  });
});
