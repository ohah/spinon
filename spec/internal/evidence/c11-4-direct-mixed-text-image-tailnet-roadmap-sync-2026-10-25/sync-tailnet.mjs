import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-pass02.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass02.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-direct-mixed-text-image-tailnet-roadmap-sync-2026-10-25/pass02';
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
const mode = process.argv[2];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet.mjs preflight|copy|verify');
assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'selected build manifest is required');
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected site build output is required');

function assertSafeRelative(relative) {
  assert.ok(typeof relative === 'string' && relative.length > 0, 'empty route');
  assert.ok(!relative.startsWith('/') && !relative.includes('\\'), 'unsafe route: ' + relative);
  assert.ok(relative.split('/').every((part) => part && part !== '.' && part !== '..'), 'unsafe route: ' + relative);
}
function inspectPath(root, relative) {
  if (!fs.existsSync(root)) return { exists: false };
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'unsafe root: ' + root);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), 'symlink in route: ' + relative);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), 'non-directory parent: ' + relative);
      else {
        assert.ok(stat.isFile(), 'non-file target: ' + relative);
        return { exists: true, bytes: fs.readFileSync(current) };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function routes() {
  const result = selectedBuild.selectedRoutes.map((entry) => entry.path);
  assert.equal(result.length, new Set(result).size, 'duplicate selected route');
  for (const route of result) assertSafeRelative(route);
  return result;
}
function preflight() {
  const files = selectedBuild.selectedRoutes.map((entry) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.bytes, 'selected source size mismatch: ' + entry.path);
    assert.equal(sha256(sourceBytes), entry.sha256, 'selected source hash mismatch: ' + entry.path);
    const previous = inspectPath(targetRoot, entry.path);
    const previousHash = previous.exists ? sha256(previous.bytes) : null;
    const status = !previous.exists ? 'create' : previousHash === entry.sha256 ? 'same' : 'replace';
    return {
      path: entry.path,
      status,
      source_bytes: entry.bytes,
      source_sha256: entry.sha256,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.byteLength : null,
      previous_sha256: previousHash,
      backup_path: status === 'replace' ? path.relative('/Users/yoonhb/Library/Application Support', path.join(backupRoot, entry.path)) : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((entry) => entry.status === status).length]));
  return {
    selected_route_count: files.length,
    selected_build_relative: path.relative(repositoryRoot, sourceRoot),
    preview_root_relative: 'SpinonPreview',
    backup_root_relative: path.relative('/Users/yoonhb/Library/Application Support', backupRoot),
    source_bytes_total: files.reduce((sum, entry) => sum + entry.source_bytes, 0),
    backup_bytes_total: files.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
    counts,
    files,
    no_deletes: true,
  };
}
if (mode === 'preflight') {
  const report = preflight();
  fs.writeFileSync(preflightPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, files: undefined }, null, 2) + '\n');
  process.exit(0);
}

const expected = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
assert.equal(expected.preview_root_relative, 'SpinonPreview', 'preview target mismatch');
assert.equal(expected.backup_root_relative, path.relative('/Users/yoonhb/Library/Application Support', backupRoot), 'backup target mismatch');
const selectedRoutes = routes();
assert.deepEqual(expected.files.map((entry) => entry.path), selectedRoutes, 'selected route set changed');

if (mode === 'copy') {
  for (const entry of expected.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.source_bytes, 'source size changed: ' + entry.path);
    assert.equal(sha256(sourceBytes), entry.source_sha256, 'source hash changed: ' + entry.path);
    const previous = inspectPath(targetRoot, entry.path);
    assert.equal(previous.exists, entry.target_exists, 'target existence changed: ' + entry.path);
    if (previous.exists) {
      assert.equal(previous.bytes.byteLength, entry.previous_bytes, 'target size changed: ' + entry.path);
      assert.equal(sha256(previous.bytes), entry.previous_sha256, 'target hash changed: ' + entry.path);
    }
    if (entry.status === 'same') continue;
    if (entry.status === 'replace') {
      const backup = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.writeFileSync(backup, previous.bytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backup)), entry.previous_sha256, 'backup hash mismatch: ' + entry.path);
    }
    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = destination + '.c114-roadmap-' + randomUUID() + '.tmp';
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), entry.source_sha256, 'copied hash mismatch: ' + entry.path);
  }
  const report = { copied_or_already_current: true, selected_route_count: selectedRoutes.length, no_deletes: true };
  fs.writeFileSync(path.join(evidenceDirectory, 'tailnet-copy-pass02.json'), JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
  process.exit(0);
}

assert.ok(origin, 'SPINON_PREVIEW_ORIGIN is required');
const localTargets = [];
const backups = [];
const remoteTargets = [];
for (const entry of expected.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assert.equal(sha256(sourceBytes), entry.source_sha256, 'source hash mismatch: ' + entry.path);
  const local = inspectPath(targetRoot, entry.path);
  const localHash = local.exists ? sha256(local.bytes) : null;
  const localOk = local.exists && local.bytes.byteLength === entry.source_bytes && localHash === entry.source_sha256;
  localTargets.push({ path: entry.path, bytes: local.exists ? local.bytes.byteLength : null, sha256: localHash, verified: localOk });
  assert.ok(localOk, 'local preview hash mismatch: ' + entry.path);
  if (entry.status === 'replace') {
    const backup = inspectPath(backupRoot, entry.path);
    const backupOk = backup.exists && backup.bytes.byteLength === entry.previous_bytes && sha256(backup.bytes) === entry.previous_sha256;
    backups.push({ path: entry.path, backup_path: entry.backup_path, verified: backupOk });
    assert.ok(backupOk, 'backup hash mismatch: ' + entry.path);
  }
  const response = await fetch(origin + '/spinon/' + entry.path + '?sync=c114-direct-mixed-roadmap-pass02-20261025', { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, 'remote HTTP status for ' + entry.path + ': ' + response.status);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  const remoteHash = sha256(remoteBytes);
  const remoteOk = remoteBytes.byteLength === entry.source_bytes && remoteHash === entry.source_sha256;
  remoteTargets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.byteLength, sha256: remoteHash, verified: remoteOk });
  assert.ok(remoteOk, 'Tailnet content hash mismatch: ' + entry.path);
}
const report = {
  selected_route_count: selectedRoutes.length,
  local_targets_verified: localTargets.filter((entry) => entry.verified).length,
  backups_verified: backups.filter((entry) => entry.verified).length,
  remote_targets_verified: remoteTargets.filter((entry) => entry.verified).length,
  all_local_targets_verified: localTargets.every((entry) => entry.verified),
  all_backups_verified: backups.every((entry) => entry.verified),
  all_remote_targets_http_200_and_hash_verified: remoteTargets.every((entry) => entry.verified),
  no_deletes: true,
  local_targets: localTargets,
  backups,
  remote_targets: remoteTargets,
};
fs.writeFileSync(path.join(evidenceDirectory, 'tailnet-verification-pass02.json'), JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({
  selected_route_count: report.selected_route_count,
  local_targets_verified: report.local_targets_verified,
  backups_verified: report.backups_verified,
  remote_targets_verified: report.remote_targets_verified,
  no_deletes: true,
}, null, 2) + '\n');
