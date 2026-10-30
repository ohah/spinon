import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = resolve(fileURLToPath(new URL('.', import.meta.url)));
const preflight = JSON.parse(await readFile(resolve(evidenceDirectory, 'preflight-pass01.json'), 'utf8'));
assert.equal(preflight.pass, 'pass01');
assert.equal(preflight.preview_root_relative, 'SpinonPreview');
assert.ok(preflight.files.length > 0);

const origin = new URL('https://macstudio.tailed42f2.ts.net/spinon/');
const concurrency = 8;
const results = new Array(preflight.files.length);
let nextIndex = 0;

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function fileURL(relative) {
  const encodedPath = relative.split('/').map(encodeURIComponent).join('/');
  return new URL(encodedPath, origin);
}

async function verifyOne(index) {
  const expected = preflight.files[index];
  try {
    const response = await fetch(fileURL(expected.path), {
      redirect: 'follow',
      headers: { 'cache-control': 'no-cache' },
      signal: AbortSignal.timeout(30_000),
    });
    const bytes = Buffer.from(await response.arrayBuffer());
    const digest = sha256(bytes);
    const verified = response.status === 200
      && bytes.byteLength === expected.source_bytes
      && digest === expected.source_sha256;
    results[index] = {
      path: expected.path,
      status: response.status,
      final_url: response.url,
      bytes: bytes.byteLength,
      sha256: digest,
      expected_bytes: expected.source_bytes,
      expected_sha256: expected.source_sha256,
      verified,
    };
  } catch (error) {
    results[index] = {
      path: expected.path,
      status: null,
      error: error instanceof Error ? error.message : String(error),
      verified: false,
    };
  }
}

await Promise.all(Array.from({ length: concurrency }, async () => {
  while (nextIndex < preflight.files.length) {
    const index = nextIndex;
    nextIndex += 1;
    await verifyOne(index);
    if ((index + 1) % 250 === 0 || index + 1 === preflight.files.length) {
      process.stdout.write(`verified ${index + 1}/${preflight.files.length}\n`);
    }
  }
}));

const output = {
  pass: 'pass01',
  origin: origin.origin,
  base_path: origin.pathname,
  concurrency,
  file_count: results.length,
  http_200_count: results.filter((result) => result.status === 200).length,
  matching_hash_count: results.filter((result) => result.verified).length,
  failures: results.filter((result) => !result.verified),
  all_verified: results.every((result) => result.verified),
  results,
};
await writeFile(resolve(evidenceDirectory, 'served-verification-pass01.json'), `${JSON.stringify(output, null, 2)}\n`);
if (!output.all_verified) process.exitCode = 1;
process.stdout.write(`${JSON.stringify({
  pass: output.pass,
  origin: output.origin,
  file_count: output.file_count,
  http_200_count: output.http_200_count,
  matching_hash_count: output.matching_hash_count,
  failure_count: output.failures.length,
  all_verified: output.all_verified,
}, null, 2)}\n`);
