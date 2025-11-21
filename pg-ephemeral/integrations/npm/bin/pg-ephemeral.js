#!/usr/bin/env node

const { execFileSync } = require('child_process');
const { join } = require('path');
const { existsSync } = require('fs');

function getPlatformPackage() {
  const platform = process.platform;
  const arch = process.arch;

  // Map Node.js platform/arch to package names
  const platformMap = {
    'linux-x64': '@pg-ephemeral/linux-x64',
    'linux-arm64': '@pg-ephemeral/linux-arm64',
    'darwin-arm64': '@pg-ephemeral/darwin-arm64',
  };

  const key = `${platform}-${arch}`;
  const packageName = platformMap[key];

  if (!packageName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error(`Supported platforms: ${Object.keys(platformMap).join(', ')}`);
    process.exit(1);
  }

  return packageName;
}

function getBinaryPath() {
  const packageName = getPlatformPackage();

  // Try to resolve the platform-specific package
  try {
    const packagePath = require.resolve(`${packageName}/package.json`);
    const binaryDir = join(packagePath, '..');
    const binaryPath = join(binaryDir, 'pg-ephemeral');

    if (!existsSync(binaryPath)) {
      console.error(`Binary not found at: ${binaryPath}`);
      console.error(`The ${packageName} package may not be installed correctly.`);
      process.exit(1);
    }

    return binaryPath;
  } catch (error) {
    console.error(`Failed to locate ${packageName} package.`);
    console.error(`Please ensure the package is installed.`);
    console.error(`Error: ${error.message}`);
    process.exit(1);
  }
}

function main() {
  const binaryPath = getBinaryPath();

  try {
    execFileSync(binaryPath, process.argv.slice(2), {
      stdio: 'inherit',
    });
  } catch (error) {
    process.exit(error.status || 1);
  }
}

main();
