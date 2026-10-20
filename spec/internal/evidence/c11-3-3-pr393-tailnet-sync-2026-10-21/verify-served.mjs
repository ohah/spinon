import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const [preflightArg, targetArg] = process.argv.slice(2);
const origin = process.env.SPINON_TAILNET_ORIGIN;
if (!preflightArg || !targetArg || !origin) throw new Error('usage: SPINON_TAILNET_ORIGIN=https://<tailnet-host> node verify-served.mjs PREFLIGHT_JSON PREVIEW_ROOT');
const preflight = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
const targetRoot = path.resolve(targetArg); const originUrl = new URL(origin);
if (originUrl.protocol !== 'https:' || originUrl.pathname !== '/' || originUrl.search || originUrl.hash) throw new Error('Tailnet origin must be an HTTPS origin without a path, query, or fragment');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
async function verify(entry) {
  if (path.posix.isAbsolute(entry.path) || entry.path.split('/').some((part) => !part || part === '.' || part === '..')) throw new Error(`unsafe relative preview path: ${entry.path}`);
  const local = fs.readFileSync(path.join(targetRoot, entry.path)); const urlPath = entry.path.split('/').map(encodeURIComponent).join('/');
  const response = await fetch(new URL(`/spinon/${urlPath}`, originUrl), { cache: 'no-store' }); const served = Buffer.from(await response.arrayBuffer());
  return { path: entry.path, status: response.status, content_type: response.headers.get('content-type'), local_bytes: local.byteLength,
    served_bytes: served.byteLength, local_sha256: hash(local), served_sha256: hash(served), matched: response.status === 200 && local.byteLength === served.byteLength && hash(local) === hash(served) };
}
const pending = [...preflight.files]; const results = [];
await Promise.all(Array.from({ length: 6 }, async () => { while (pending.length) results.push(await verify(pending.shift())); }));
results.sort((a, b) => a.path.localeCompare(b.path)); const mismatches = results.filter((entry) => !entry.matched);
process.stdout.write(`${JSON.stringify({ pass: preflight.pass, preview_root_relative: preflight.preview_root_relative, base_path: '/spinon/', target_count: results.length,
  matched_count: results.length - mismatches.length, mismatch_count: mismatches.length, all_verified: mismatches.length === 0, files: results }, null, 2)}\n`);
if (mismatches.length) process.exitCode = 1;
