#!/usr/bin/env node
const { binaryPath } = require('../lib/index.js');

process.execve(binaryPath(), [binaryPath(), ...process.argv.slice(2)], process.env);
