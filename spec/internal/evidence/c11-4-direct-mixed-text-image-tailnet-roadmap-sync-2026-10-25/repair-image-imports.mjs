import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-root-20261025');
const sourceManifest = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'selected-root-image-import-repair.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite image repair report');
const imageExtensions = new Set(['.png', '.jpg', '.jpeg', '.webp', '.gif', '.svg']);
const copies = [];
for (const entry of sourceManifest.sourceFiles) {
  if (!entry.path.startsWith('public/')) continue;
  if (!imageExtensions.has(path.extname(entry.path).toLowerCase())) continue;
  const publicPath = path.join(selectedRoot, entry.path);
  const rootRelative = entry.path.slice('public/'.length);
  const rootPath = path.join(selectedRoot, rootRelative);
  const bytes = fs.readFileSync(publicPath);
  assert.equal(sha256(bytes), entry.sha256, 'public image source hash mismatch: ' + entry.path);
  assert.ok(!fs.existsSync(rootPath), 'refusing to overwrite doc-root image: ' + rootRelative);
  fs.mkdirSync(path.dirname(rootPath), { recursive: true });
  fs.writeFileSync(rootPath, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(rootPath)), entry.sha256, 'doc-root image hash mismatch: ' + rootRelative);
  copies.push({ source: entry.path, path: rootRelative, bytes: bytes.byteLength, sha256: entry.sha256 });
}
const report = { copied_image_imports: copies.length, copies, no_deletes: true };
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ copied_image_imports: copies.length, no_deletes: true }, null, 2) + '\n');
