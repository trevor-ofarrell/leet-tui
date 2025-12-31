#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const PLATFORMS = {
  'darwin-arm64': '@leet-tui/darwin-arm64',
  'darwin-x64': '@leet-tui/darwin-x64',
  'linux-x64': '@leet-tui/linux-x64',
  'linux-arm64': '@leet-tui/linux-arm64',
  'win32-x64': '@leet-tui/win32-x64',
};

const platformKey = `${process.platform}-${process.arch}`;
const packageName = PLATFORMS[platformKey];

if (!packageName) {
  console.warn(`\n[leet-tui] Warning: No prebuilt binary available for ${platformKey}`);
  console.warn(`Supported platforms: ${Object.keys(PLATFORMS).join(', ')}`);
  console.warn('You may need to build from source: https://github.com/trevor/leet-tui\n');
  process.exit(0);
}

try {
  const packagePath = require.resolve(`${packageName}/package.json`);
  const packageDir = path.dirname(packagePath);
  const binName = process.platform === 'win32' ? 'leet-tui.exe' : 'leet-tui';
  const binaryPath = path.join(packageDir, 'bin', binName);

  if (fs.existsSync(binaryPath)) {
    console.log(`[leet-tui] Binary installed successfully for ${platformKey}`);
  } else {
    console.warn(`[leet-tui] Warning: Binary not found at ${binaryPath}`);
  }
} catch (err) {
  console.warn(`[leet-tui] Warning: Could not verify binary installation`);
  console.warn(err.message);
}
