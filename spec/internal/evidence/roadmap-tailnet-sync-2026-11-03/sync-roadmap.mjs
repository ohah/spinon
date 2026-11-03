import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-11-03';
const route = 'roadmap.html';
const targetPath = path.join(previewRoot, route);
const backupPath = path.join(backupRoot, route);
const preflightPath = path.join(evidenceDirectory, 'preflight.json');
const copyReportPath = path.join(evidenceDirectory, 'copy.json');
const verifyReportPath = path.join(evidenceDirectory, 'verify.json');
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const [mode, preflightArgument] = process.argv.slice(2);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-roadmap.mjs preflight|copy|verify [preflight.json]');

function readRegularFile(filePath, label) {
  const stat = fs.lstatSync(filePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `${label} must be a regular file: ${filePath}`);
  return fs.readFileSync(filePath);
}

function readOptionalTarget(filePath) {
  try {
    return { exists: true, bytes: readRegularFile(filePath, 'target') };
  } catch (error) {
    if (error.code === 'ENOENT') return { exists: false, bytes: null };
    throw error;
  }
}

function assertDirectory(directoryPath, label) {
  const stat = fs.lstatSync(directoryPath);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `${label} must be a real directory: ${directoryPath}`);
}

function writeNewJson(filePath, value) {
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
}

const sourceBytes = readRegularFile(sourcePath, 'roadmap source');
const sourceHash = sha256(sourceBytes);
const sourceCommit = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mergedSourceBytes = execFileSync('git', ['show', `${sourceCommit}:spec/public/roadmap.html`], { cwd: repositoryRoot });
assert.deepEqual(sourceBytes, mergedSourceBytes, 'refusing to publish a roadmap that differs from origin/main');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(preflightPath), 'refusing to overwrite preflight.json');
  assert.ok(!fs.existsSync(copyReportPath) && !fs.existsSync(verifyReportPath), 'sync reports already exist');
  assertDirectory(previewRoot, 'preview root');
  assertDirectory(path.dirname(backupRoot), 'backup parent');
  if (fs.existsSync(backupRoot)) assertDirectory(backupRoot, 'backup root');
  const previous = readOptionalTarget(targetPath);
  const backupExists = fs.existsSync(backupPath);
  assert.ok(!backupExists, `refusing to reuse backup path: ${backupPath}`);
  const report = {
    mode: 'preflight',
    source_commit: sourceCommit,
    route,
    source_bytes: sourceBytes.byteLength,
    source_sha256: sourceHash,
    preview_root: previewRoot,
    target_exists: previous.exists,
    target_bytes: previous.bytes?.byteLength ?? null,
    target_sha256: previous.bytes ? sha256(previous.bytes) : null,
    backup_root: backupRoot,
    backup_route: route,
    no_deletes: true,
  };
  writeNewJson(preflightPath, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(preflightArgument, `${mode} requires the preflight JSON path`);
const preflight = JSON.parse(readRegularFile(path.resolve(evidenceDirectory, preflightArgument), 'preflight report').toString('utf8'));
assert.equal(preflight.mode, 'preflight', 'preflight mode mismatch');
assert.equal(preflight.route, route, 'route mismatch');
assert.equal(preflight.source_bytes, sourceBytes.byteLength, 'source size changed after preflight');
assert.equal(preflight.source_sha256, sourceHash, 'source hash changed after preflight');

if (mode === 'copy') {
  assert.ok(!fs.existsSync(copyReportPath), 'refusing to overwrite copy.json');
  assert.ok(!fs.existsSync(verifyReportPath), 'verify.json already exists');
  assertDirectory(previewRoot, 'preview root');
  assertDirectory(path.dirname(backupRoot), 'backup parent');
  if (fs.existsSync(backupRoot)) assertDirectory(backupRoot, 'backup root');
  const previous = readOptionalTarget(targetPath);
  assert.equal(previous.exists, preflight.target_exists, 'preview target existence changed after preflight');
  assert.equal(previous.bytes?.byteLength ?? null, preflight.target_bytes, 'preview target size changed after preflight');
  assert.equal(previous.bytes ? sha256(previous.bytes) : null, preflight.target_sha256, 'preview target hash changed after preflight');
  if (previous.exists) {
    fs.mkdirSync(backupRoot, { recursive: true });
    assertDirectory(backupRoot, 'backup root');
    fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
    const backupBytes = readRegularFile(backupPath, 'backup');
    assert.equal(sha256(backupBytes), preflight.target_sha256, 'backup hash mismatch');
  }
  const temporaryPath = `${targetPath}.roadmap-sync-${randomUUID()}.tmp`;
  fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
  try {
    fs.renameSync(temporaryPath, targetPath);
  } catch (error) {
    if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
    throw error;
  }
  const copiedBytes = readRegularFile(targetPath, 'copied roadmap');
  assert.equal(copiedBytes.byteLength, sourceBytes.byteLength, 'copied roadmap size mismatch');
  assert.equal(sha256(copiedBytes), sourceHash, 'copied roadmap hash mismatch');
  const report = {
    mode: 'copy',
    source_commit: sourceCommit,
    route,
    source_bytes: sourceBytes.byteLength,
    source_sha256: sourceHash,
    target_bytes: copiedBytes.byteLength,
    target_sha256: sha256(copiedBytes),
    backup_created: previous.exists,
    backup_sha256: previous.exists ? sha256(readRegularFile(backupPath, 'backup')) : null,
    no_deletes: true,
  };
  writeNewJson(copyReportPath, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(!fs.existsSync(verifyReportPath), 'refusing to overwrite verify.json');
assert.ok(fs.existsSync(copyReportPath), 'copy.json must exist before remote verification');
const copyReport = JSON.parse(readRegularFile(copyReportPath, 'copy report').toString('utf8'));
assert.equal(copyReport.source_sha256, sourceHash, 'copy report source hash mismatch');
assert.equal(copyReport.target_sha256, sourceHash, 'copy report target hash mismatch');
assertDirectory(previewRoot, 'preview root');
assertDirectory(path.dirname(backupRoot), 'backup parent');
if (fs.existsSync(backupRoot)) assertDirectory(backupRoot, 'backup root');
const localBytes = readRegularFile(targetPath, 'preview roadmap');
assert.equal(localBytes.byteLength, sourceBytes.byteLength, 'local preview size mismatch');
assert.equal(sha256(localBytes), sourceHash, 'local preview hash mismatch');
if (preflight.target_exists) {
  const backupBytes = readRegularFile(backupPath, 'backup');
  assert.equal(backupBytes.byteLength, preflight.target_bytes, 'backup size mismatch');
  assert.equal(sha256(backupBytes), preflight.target_sha256, 'backup hash mismatch');
}
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set for remote verification');
const originUrl = new URL(origin);
assert.equal(originUrl.protocol, 'https:', 'remote verification requires HTTPS');
assert.equal(originUrl.hostname, 'macstudio.tailed42f2.ts.net', 'unexpected Tailnet host');
const response = await fetch(`${originUrl.origin}/spinon/${route}?sync=c11-dense-2026-11-03`, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(15_000) });
assert.equal(response.status, 200, `Tailnet HTTP status: ${response.status}`);
const remoteBytes = Buffer.from(await response.arrayBuffer());
assert.equal(remoteBytes.byteLength, sourceBytes.byteLength, 'Tailnet response size mismatch');
assert.equal(sha256(remoteBytes), sourceHash, 'Tailnet served hash mismatch');
const report = {
  mode: 'verify',
  source_commit: sourceCommit,
  route,
  status: response.status,
  source_bytes: sourceBytes.byteLength,
  source_sha256: sourceHash,
  local_bytes: localBytes.byteLength,
  local_sha256: sha256(localBytes),
  remote_bytes: remoteBytes.byteLength,
  remote_sha256: sha256(remoteBytes),
  local_hash_matches: true,
  remote_http_200_and_hash_matches: true,
  no_deletes: true,
};
writeNewJson(verifyReportPath, report);
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
