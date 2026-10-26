import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const build = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-build-pr666-pass02.json'), 'utf8'));
const copy = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'tailnet-copy-pr666-pass01.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'tailnet-verify-pr666-pass01.json');
const logPath = path.join(evidenceDirectory, 'tailnet-verification-run-pr666-pass01.log');
for (const candidate of [reportPath, logPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const selectedRoot = path.join(repositoryRoot, build.selectedOutputRoot);
assert.equal(copy.sourceHead, build.sourceHead);
assert.equal(copy.sourceTree, build.sourceTree);
assert.equal(copy.selectedRouteCount, build.selectedRoutes.length);
assert.equal(copy.noDeletes, true);

const local = [];
for (const route of build.selectedRoutes) {
  const bytes = fs.readFileSync(path.join(selectedRoot, route.path));
  assert.equal(bytes.length, route.bytes, `selected build size mismatch: ${route.path}`);
  assert.equal(sha256(bytes), route.sha256, `selected build hash mismatch: ${route.path}`);
  const served = fs.readFileSync(path.join(previewRoot, route.path));
  assert.equal(served.length, route.bytes, `local preview size mismatch: ${route.path}`);
  assert.equal(sha256(served), route.sha256, `local preview hash mismatch: ${route.path}`);
  local.push({ path: route.path, bytes: served.length, sha256: sha256(served) });
}

const backupRoot = copy.backupRoot;
const backedUp = [];
for (const record of copy.replaced) {
  const backupBytes = fs.readFileSync(path.join(backupRoot, record.backupPath));
  assert.equal(backupBytes.length, record.backupBytes, `backup size mismatch: ${record.path}`);
  assert.equal(sha256(backupBytes), record.backupSha256, `backup hash mismatch: ${record.path}`);
  backedUp.push({ path: record.path, bytes: backupBytes.length, sha256: sha256(backupBytes) });
}
function walkFiles(root, relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(root, relative), { withFileTypes: true })) {
    const child = path.posix.join(relative, entry.name);
    const absolute = path.join(root, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in backup: ${child}`);
    if (entry.isDirectory()) files.push(...walkFiles(root, child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`unsupported backup entry: ${child}`);
  }
  return files;
}
assert.equal(walkFiles(backupRoot).length, copy.replacedCount, 'backup contains unexpected or missing files');

const baseUrl = 'https://macstudio.tailed42f2.ts.net/spinon/';
const remoteResults = [];
let cursor = 0;
const workers = Array.from({ length: 8 }, async () => {
  while (true) {
    const index = cursor++;
    const route = build.selectedRoutes[index];
    if (!route) return;
    const url = baseUrl + route.path.split('/').map(encodeURIComponent).join('/');
    const response = await fetch(url, { cache: 'no-store', redirect: 'follow' });
    const bytes = Buffer.from(await response.arrayBuffer());
    const digest = sha256(bytes);
    assert.equal(response.status, 200, `Tailnet HTTP ${response.status}: ${route.path}`);
    assert.equal(bytes.length, route.bytes, `Tailnet size mismatch: ${route.path}`);
    assert.equal(digest, route.sha256, `Tailnet hash mismatch: ${route.path}`);
    remoteResults[index] = { path: route.path, httpStatus: response.status, bytes: bytes.length, sha256: digest };
  }
});
await Promise.all(workers);

const externalExisting = [];
for (const asset of build.externalExistingAssets) {
  const url = baseUrl + asset.path.split('/').map(encodeURIComponent).join('/');
  const response = await fetch(url, { cache: 'no-store', redirect: 'follow' });
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, `Tailnet HTTP ${response.status}: ${asset.path}`);
  assert.equal(bytes.length, asset.bytes, `Tailnet size mismatch: ${asset.path}`);
  assert.equal(sha256(bytes), asset.sha256, `Tailnet hash mismatch: ${asset.path}`);
  externalExisting.push({ path: asset.path, httpStatus: response.status, bytes: bytes.length, sha256: sha256(bytes) });
}

const report = {
  sourceHead: build.sourceHead,
  sourceTree: build.sourceTree,
  selectedRouteCount: build.selectedRoutes.length,
  selectedRouteBytes: build.selectedRouteBytes,
  localRoutesVerified: local.length,
  backupFilesVerified: backedUp.length,
  remoteRoutesVerified: remoteResults.length,
  externalExistingAssetsVerified: externalExisting.length,
  createdCount: copy.createdCount,
  replacedCount: copy.replacedCount,
  identicalCount: copy.identicalCount,
  openHandleRoutesSkipped: copy.openHandleSkipped,
  noDeletes: true,
  remoteResults,
  backupResults: backedUp,
  externalExistingResults: externalExisting,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(logPath, `sourceHead=${build.sourceHead}\nselectedRoutes=${report.remoteRoutesVerified}/${build.selectedRoutes.length}\nlocalRoutes=${report.localRoutesVerified}/${build.selectedRoutes.length}\nbackups=${report.backupFilesVerified}/${copy.replacedCount}\nexternalExistingAssets=${report.externalExistingAssetsVerified}/${build.externalExistingAssets.length}\nnoDeletes=true\n` , { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, remoteResults: undefined, backupResults: undefined, externalExistingResults: undefined }, null, 2)}\n`);
