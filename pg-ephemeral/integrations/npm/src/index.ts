import { spawn, execFileSync, type ChildProcess } from 'node:child_process';
import { createInterface } from 'node:readline';
import * as path from 'node:path';
import type { Readable, Writable } from 'node:stream';
import { Client } from 'pg';

export interface StartOptions {
  instanceName?: string;
  config?: string | null;
}

export interface Server {
  url: string;
  shutdown(): Promise<void>;
}

const PLATFORMS: Record<string, string> = {
  'linux-x64': '@pg-ephemeral/linux-x64',
  'linux-arm64': '@pg-ephemeral/linux-arm64',
  'darwin-arm64': '@pg-ephemeral/darwin-arm64',
};

export function binaryPath(): string {
  const key = `${process.platform}-${process.arch}`;
  const packageName = PLATFORMS[key];

  if (!packageName) {
    throw new Error(`Unsupported platform: ${key}`);
  }

  try {
    const packageDir = path.dirname(require.resolve(`${packageName}/package.json`));
    return path.join(packageDir, 'bin', 'pg-ephemeral');
  } catch {
    throw new Error(
      `Platform package ${packageName} is not installed. ` +
      'Ensure pg-ephemeral is installed with optional dependencies enabled.'
    );
  }
}

export function version(): string {
  const output = execFileSync(binaryPath(), ['--version'], { encoding: 'utf-8' });
  const match = output.match(/^pg-ephemeral (\d+\.\d+\.\d+(?:-.+)?)\n$/);

  if (!match?.[1]) {
    throw new Error('Failed to parse version from pg-ephemeral binary');
  }

  return match[1];
}

export function platformSupported(): boolean {
  try {
    execFileSync(binaryPath(), ['platform', 'support']);
    return true;
  } catch {
    return false;
  }
}

export async function start(opts: StartOptions = {}): Promise<Server> {
  const { instanceName = 'main', config = null } = opts;
  const args: string[] = [];

  if (config) {
    args.push('--config-file', config);
  }

  args.push(
    'integration-server',
    '--instance', instanceName,
    '--result-fd', '3',
    '--control-fd', '4',
  );

  const child: ChildProcess = spawn(binaryPath(), args, {
    stdio: ['ignore', 'inherit', 'inherit', 'pipe', 'pipe'],
  });

  const resultStream = child.stdio[3] as Readable;
  const controlStream = child.stdio[4] as Writable;

  const configJson = await new Promise<string>((resolve, reject) => {
    const rl = createInterface({ input: resultStream });
    let resolved = false;

    rl.once('line', (line: string) => {
      resolved = true;
      rl.close();
      resolve(line);
    });

    rl.once('close', () => {
      if (!resolved) {
        reject(new Error('Failed to read server configuration'));
      }
    });

    child.once('error', (error: Error) => {
      rl.close();
      reject(error);
    });
  });

  const { url } = JSON.parse(configJson) as { url: string };

  return {
    url,
    shutdown(): Promise<void> {
      return new Promise((resolve) => {
        if (child.exitCode !== null) {
          resolve();
          return;
        }
        controlStream.end();
        child.once('exit', () => resolve());
      });
    },
  };
}

export async function withServer<T>(
  fn: (server: Server) => Promise<T>,
  opts?: StartOptions,
): Promise<T> {
  const server = await start(opts);
  try {
    return await fn(server);
  } finally {
    await server.shutdown();
  }
}

export async function withConnection<T>(
  fn: (client: Client) => Promise<T>,
  opts?: StartOptions,
): Promise<T> {
  return withServer(async (server) => {
    const client = new Client({ connectionString: server.url });
    await client.connect();
    try {
      return await fn(client);
    } finally {
      await client.end();
    }
  }, opts);
}
