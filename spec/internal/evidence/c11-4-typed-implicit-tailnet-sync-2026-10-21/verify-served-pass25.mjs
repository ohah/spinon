import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const sourceRoot = path.join(repoRoot, 'build/spinon/c11-4-typed-implicit-tailnet-site-pass25');
const baseUrl = new URL('https://macstudio.tailed42f2.ts.net/spinon/');
const pass = 'pass25';

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected build: ${relative}`);
    if (entry.isDirectory()) paths.push(...listFiles(fullPath, relative));
    else if (entry.isFile()) paths.push(relative);
    else throw new Error(`unsupported selected build entry: ${relative}`);
  }
  return paths;
}

function fileUrl(relative) {
  const encodedPath = relative.split('/').map((part) => encodeURIComponent(part)).join('/');
  return new URL(encodedPath, baseUrl);
}

const paths = listFiles(sourceRoot);
const results = new Array(paths.length);
let nextIndex = 0;
const worker = async () => {
  while (true) {
    const index = nextIndex;
    nextIndex += 1;
    if (index >= paths.length) return;
    const relative = paths[index];
    const localBytes = fs.readFileSync(path.join(sourceRoot, relative));
    try {
      const response = await fetch(fileUrl(relative));
      const servedBytes = Buffer.from(await response.arrayBuffer());
      const localHash = sha256(localBytes);
      const servedHash = sha256(servedBytes);
      results[index] = {
        path: relative,
        status: response.status,
        content_type: response.headers.get('content-type'),
        local_bytes: localBytes.byteLength,
        served_bytes: servedBytes.byteLength,
        local_sha256: localHash,
        served_sha256: servedHash,
        matched: response.status === 200 && localBytes.byteLength === servedBytes.byteLength && localHash === servedHash,
      };
    } catch (error) {
      results[index] = { path: relative, error: String(error), matched: false };
    }
  }
};

await Promise.all(Array.from({ length: Math.min(6, paths.length) }, worker));
const matchedCount = results.filter((result) => result.matched).length;
const output = {
  pass,
  preview_root_relative: 'SpinonPreview',
  base_path: '/spinon/',
  target_count: results.length,
  matched_count: matchedCount,
  mismatch_count: results.length - matchedCount,
  all_verified: matchedCount === results.length,
  files: results,
};
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
if (!output.all_verified) process.exitCode = 1;
