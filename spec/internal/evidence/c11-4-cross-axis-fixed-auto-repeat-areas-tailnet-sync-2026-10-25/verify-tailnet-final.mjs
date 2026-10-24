import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const applicationSupport = '/Users/yoonhb/Library/Application Support';
const targetRoot = path.join(applicationSupport, 'SpinonPreview');
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
const outputName = process.argv[2] ?? 'tailnet-final-verification-pass02.json';
assert.match(outputName, /^[a-z0-9-]+\.json$/, 'output must be a simple JSON filename');
const outputPath = path.join(evidenceDirectory, outputName);
assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set');
assert.ok(!fs.existsSync(outputPath), 'refusing to overwrite final verification');
const docs = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-build-verification-pass02.json'), 'utf8'));
const assets = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-static-assets-pass02.json'), 'utf8'));
const sourceRoots = [
  path.join(repositoryRoot, docs.selectedOutputRoot),
  path.join(repositoryRoot, assets.sourceRoot),
];
const routes = [
  ...docs.selectedRoutes.map((entry) => ({ ...entry, sourceRoot: sourceRoots[0], kind: 'document-or-direct-asset' })),
  ...assets.routes.map((entry) => ({ ...entry, sourceRoot: sourceRoots[1], kind: 'rspress-static-asset' })),
];
assert.equal(routes.length, new Set(routes.map((entry) => entry.path)).size, 'duplicate final route');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const targets = [];
for (const entry of routes) {
  const sourceBytes = fs.readFileSync(path.join(entry.sourceRoot, entry.path));
  assert.equal(sourceBytes.byteLength, entry.bytes, `source size mismatch: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.sha256, `source hash mismatch: ${entry.path}`);
  const localPath = path.join(targetRoot, entry.path);
  const localStat = fs.lstatSync(localPath);
  assert.ok(localStat.isFile() && !localStat.isSymbolicLink(), `unsafe local preview file: ${entry.path}`);
  const localBytes = fs.readFileSync(localPath);
  assert.equal(localBytes.byteLength, entry.bytes, `local size mismatch: ${entry.path}`);
  assert.equal(sha256(localBytes), entry.sha256, `local hash mismatch: ${entry.path}`);
  const response = await fetch(`${origin}/spinon/${entry.path}?sync=c114-crossaxis-final-pass02-20261025`, { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `remote status mismatch: ${entry.path}`);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  const remoteHash = sha256(remoteBytes);
  assert.equal(remoteBytes.byteLength, entry.bytes, `remote size mismatch: ${entry.path}`);
  assert.equal(remoteHash, entry.sha256, `remote hash mismatch: ${entry.path}`);
  targets.push({ path: entry.path, kind: entry.kind, bytes: entry.bytes, sha256: entry.sha256, http_status: response.status, verified: true });
}
const documentPreflight = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'tailnet-preflight-pass02.json'), 'utf8'));
const backupTargets = [];
for (const entry of documentPreflight.files.filter((file) => file.status === 'replace')) {
  const backupPath = path.join(applicationSupport, entry.backup_path);
  const stat = fs.lstatSync(backupPath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe backup file: ${entry.backup_path}`);
  const backupBytes = fs.readFileSync(backupPath);
  assert.equal(backupBytes.byteLength, entry.previous_bytes, `backup size mismatch: ${entry.path}`);
  assert.equal(sha256(backupBytes), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
  backupTargets.push({ path: entry.path, bytes: backupBytes.byteLength, sha256: entry.previous_sha256, verified: true });
}
const report = {
  remote_origin: origin,
  remote_path_prefix: '/spinon/',
  documentRouteCount: docs.selectedRouteCount,
  rspressStaticAssetCount: assets.staticRouteCount,
  distinctPreviewRouteCount: targets.length,
  local_targets_verified: targets.length,
  remote_http_200_and_hash_verified: targets.length,
  historical_replacement_backups_verified: backupTargets.length,
  all_targets_verified: targets.every((entry) => entry.verified),
  all_backups_verified: backupTargets.every((entry) => entry.verified),
  no_deletes: true,
  targets,
  backups: backupTargets,
};
fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, targets: undefined, backups: undefined }, null, 2)}\n`);
