import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr515-tailnet-site-pass50');
const preflightPath = path.join(evidenceDirectory, 'preflight-pass50.json');
const origin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const pass = 'pass50';
const source = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
assert.equal(source.pass, pass);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const failures = [];
let completed = 0;
let matching = 0;
const concurrency = 24;
const maxAttempts = 3;

async function verify(item) {
  const local = fs.readFileSync(path.join(sourceRoot, item.path));
  assert.equal(local.byteLength, item.sourceBytes);
  assert.equal(sha256(local), item.sourceSha256);
  const route = `${origin}${item.path.split('/').map(encodeURIComponent).join('/')}`;
  let lastError;
  for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
    try {
      const response = await fetch(route, { signal: AbortSignal.timeout(30000), cache: 'no-store' });
      const bytes = Buffer.from(await response.arrayBuffer());
      const actualHash = sha256(bytes);
      if (response.status === 200 && bytes.byteLength === item.sourceBytes && actualHash === item.sourceSha256) {
        matching += 1;
        completed += 1;
        if (completed % 500 === 0 || completed === source.files.length) {
          process.stdout.write(`verified ${completed}/${source.files.length}\n`);
        }
        return;
      }
      lastError = { status: response.status, actualBytes: bytes.byteLength, actualSha256: actualHash };
    } catch (error) {
      lastError = { error: String(error) };
    }
    if (attempt < maxAttempts) await new Promise((resolve) => setTimeout(resolve, attempt * 300));
  }
  failures.push({ path: item.path, ...lastError });
  completed += 1;
  if (completed % 500 === 0 || completed === source.files.length) {
    process.stdout.write(`verified ${completed}/${source.files.length}\n`);
  }
}

let next = 0;
await Promise.all(Array.from({ length: concurrency }, async () => {
  while (next < source.files.length) {
    const index = next;
    next += 1;
    await verify(source.files[index]);
  }
}));

const output = {
  pass, origin, sourceFileCount: source.files.length,
  http200AndByteHashMatches: matching, failureCount: failures.length,
  failures, concurrency, maxAttempts, allVerified: failures.length === 0,
};
fs.writeFileSync(path.join(evidenceDirectory, 'remote-verification-pass50.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...output, failures: undefined }, null, 2)}\n`);
if (!output.allVerified) process.exitCode = 1;
