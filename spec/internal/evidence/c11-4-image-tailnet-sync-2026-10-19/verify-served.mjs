import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const [preflightArg, targetArg] = process.argv.slice(2);
const origin = process.env.SPINON_TAILNET_ORIGIN;
if (!preflightArg || !targetArg || !origin) {
  throw new Error('usage: SPINON_TAILNET_ORIGIN=https://<tailnet-host> node verify-served.mjs PREFLIGHT_JSON PREVIEW_ROOT');
}

const preflight = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
const targetRoot = path.resolve(targetArg);
const originUrl = new URL(origin);
if (originUrl.protocol !== 'https:' || originUrl.pathname !== '/' || originUrl.search || originUrl.hash) {
  throw new Error('Tailnet origin must be an HTTPS origin without a path, query, or fragment');
}

function hash(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function verify(entry) {
  const localBytes = fs.readFileSync(path.join(targetRoot, entry.path));
  const relativeUrl = entry.path.split('/').map(encodeURIComponent).join('/');
  const response = await fetch(new URL(`/spinon/${relativeUrl}`, originUrl), { cache: 'no-store' });
  const servedBytes = Buffer.from(await response.arrayBuffer());
  const servedHash = hash(servedBytes);
  const localHash = hash(localBytes);
  return {
    path: entry.path,
    status: response.status,
    content_type: response.headers.get('content-type'),
    local_bytes: localBytes.byteLength,
    served_bytes: servedBytes.byteLength,
    local_sha256: localHash,
    served_sha256: servedHash,
    matched: response.status === 200 && localBytes.byteLength === servedBytes.byteLength && localHash === servedHash,
  };
}

const pending = [...preflight.files];
const results = [];
const concurrency = 6;
await Promise.all(Array.from({ length: concurrency }, async () => {
  while (pending.length > 0) {
    const entry = pending.shift();
    results.push(await verify(entry));
  }
}));
results.sort((a, b) => a.path.localeCompare(b.path));
const mismatches = results.filter((entry) => !entry.matched);
process.stdout.write(`${JSON.stringify({
  pass: preflight.pass,
  preview_root_relative: preflight.preview_root_relative,
  base_path: '/spinon/',
  target_count: results.length,
  matched_count: results.length - mismatches.length,
  mismatch_count: mismatches.length,
  all_verified: mismatches.length === 0,
  files: results,
}, null, 2)}\n`);
if (mismatches.length > 0) process.exitCode = 1;
