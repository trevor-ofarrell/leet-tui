#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const REPO = 'trevor/leet-tui';
const VERSION = require('../package.json').version;

const PLATFORMS = {
  'darwin-arm64': 'leet-tui-darwin-arm64',
  'darwin-x64': 'leet-tui-darwin-x64',
  'linux-x64': 'leet-tui-linux-x64',
  'linux-arm64': 'leet-tui-linux-arm64',
  'win32-x64': 'leet-tui-win32-x64.exe',
};

const platformKey = `${process.platform}-${process.arch}`;
const binaryName = PLATFORMS[platformKey];

if (!binaryName) {
  console.warn(`\n[leet-tui] No prebuilt binary for ${platformKey}`);
  console.warn(`Supported: ${Object.keys(PLATFORMS).join(', ')}`);
  console.warn('Build from source: https://github.com/' + REPO + '\n');
  process.exit(0);
}

const binDir = path.join(__dirname, '..', 'bin');
const binPath = path.join(binDir, process.platform === 'win32' ? 'leet-tui.exe' : 'leet-tui-binary');
const url = `https://github.com/${REPO}/releases/download/v${VERSION}/${binaryName}`;

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    https.get(url, (response) => {
      // Handle redirects (GitHub releases redirect to S3)
      if (response.statusCode === 302 || response.statusCode === 301) {
        file.close();
        fs.unlinkSync(dest);
        return download(response.headers.location, dest).then(resolve).catch(reject);
      }

      if (response.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        reject(new Error(`Download failed: HTTP ${response.statusCode}`));
        return;
      }

      const total = parseInt(response.headers['content-length'], 10);
      let downloaded = 0;

      response.on('data', (chunk) => {
        downloaded += chunk.length;
        if (total) {
          const pct = Math.round((downloaded / total) * 100);
          process.stdout.write(`\r[leet-tui] Downloading... ${pct}%`);
        }
      });

      response.pipe(file);

      file.on('finish', () => {
        file.close();
        console.log('\r[leet-tui] Download complete.      ');
        resolve();
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

async function main() {
  console.log(`[leet-tui] Installing binary for ${platformKey}...`);

  // Ensure bin directory exists
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  try {
    await download(url, binPath);

    // Make executable on Unix
    if (process.platform !== 'win32') {
      fs.chmodSync(binPath, 0o755);
    }

    console.log('[leet-tui] Installation successful!');
  } catch (err) {
    console.error(`\n[leet-tui] Failed to download binary: ${err.message}`);
    console.error(`URL: ${url}`);
    console.error('\nYou can build from source:');
    console.error('  git clone https://github.com/' + REPO);
    console.error('  cd leet-tui && cargo build --release');
    process.exit(1);
  }
}

main();
