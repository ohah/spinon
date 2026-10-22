import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), '../../../..');
const evidenceDir = path.dirname(new URL(import.meta.url).pathname);
const preflight = JSON.parse(fs.readFileSync(path.join(evidenceDir, process.argv[2]), 'utf8'));
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const baseUrl = 'http://127.0.0.1:8768/spinon/';
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const results = new Array(preflight.files.length);
let cursor = 0;
async function worker() {
  while (true) {
    const index = cursor++;
    if (index >= preflight.files.length) return;
    const entry = preflight.files[index];
    const local = fs.readFileSync(path.join(targetRoot, entry.path));
    const urlPath = entry.path.split('/').map(encodeURIComponent).join('/');
    const response = await fetch(new URL(urlPath, baseUrl));
    const served = Buffer.from(await response.arrayBuffer());
    const localHash = digest(local);
    const servedHash = digest(served);
    results[index] = {
      path: entry.path,
      status: response.status,
      content_type: response.headers.get('content-type'),
      local_bytes: local.byteLength,
      served_bytes: served.byteLength,
      local_sha256: localHash,
      served_sha256: servedHash,
      matched: response.status === 200 && local.byteLength === served.byteLength && localHash === servedHash,
    };
  }
}
await Promise.all(Array.from({ length: 32 }, () => worker()));
const matchedCount = results.filter((result) => result.matched).length;
const output = {
  pass: preflight.pass,
  preview_root_relative: preflight.preview_root_relative,
  base_path: '/spinon/',
  target_count: results.length,
  matched_count: matchedCount,
  mismatch_count: results.length - matchedCount,
  all_verified: matchedCount === results.length,
  files: results,
};
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
if (matchedCount !== results.length) process.exitCode = 1;
