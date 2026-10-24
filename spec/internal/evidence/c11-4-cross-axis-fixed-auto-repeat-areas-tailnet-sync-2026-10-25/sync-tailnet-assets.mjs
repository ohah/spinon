import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const manifestPath = path.join(evidenceDirectory, 'selected-static-assets-pass02.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-assets-preflight-pass02.json');
const resultPath = path.join(evidenceDirectory, `tailnet-assets-${mode}-pass02.json`);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-cross-axis-fixed-auto-repeat-areas-tailnet-sync-2026-10-25/pass02';
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet-assets.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(manifestPath).isFile(), 'static asset manifest is required');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, manifest.sourceRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected Rspress build is required');
assert.ok(!fs.existsSync(resultPath), `refusing to overwrite ${path.basename(resultPath)}`);

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
function assertSafeRelative(relative) {
  assert.ok(typeof relative === 'string' && relative.startsWith('static/'), `route is outside selected static output: ${relative}`);
  assert.ok(!relative.startsWith('/') && !relative.includes('\\'), `unsafe route: ${relative}`);
  assert.ok(relative.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${relative}`);
}
function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  const parts = relative.split('/');
  let current = root;
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in path: ${relative}`);
      else assert.ok(stat.isFile(), `non-file path: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current) };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function makePreflight() {
  const routes = manifest.routes.map((route) => route.path);
  assert.equal(routes.length, new Set(routes).size, 'duplicate static asset route');
  const files = manifest.routes.map((entry) => {
    assertSafeRelative(entry.path);
    const source = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(source.byteLength, entry.bytes, `manifest source size mismatch: ${entry.path}`);
    assert.equal(sha256(source), entry.sha256, `manifest source hash mismatch: ${entry.path}`);
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
    selected_static_route_count: files.length,
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
  const preflight = makePreflight();
  fs.writeFileSync(preflightPath, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...preflight, files: undefined }, null, 2)}\n`);
  process.exit(0);
}
assert.ok(preflightArgument, `${mode} mode requires a preflight JSON path`);
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
assert.equal(expected.preview_root_relative, 'SpinonPreview', 'preflight preview root mismatch');
assert.equal(expected.backup_root_relative, path.relative('/Users/yoonhb/Library/Application Support', backupRoot), 'preflight backup root mismatch');
assert.equal(expected.files.length, manifest.routes.length, 'static asset route count changed after preflight');
assert.deepEqual(expected.files.map((entry) => entry.path), manifest.routes.map((entry) => entry.path), 'static asset route set changed after preflight');

if (mode === 'copy') {
  for (const entry of expected.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size changed: ${entry.path}`);
    assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed: ${entry.path}`);
    const previous = inspectPath(targetRoot, entry.path);
    assert.equal(previous.exists, entry.target_exists, `target existence changed: ${entry.path}`);
    if (previous.exists) {
      assert.equal(previous.bytes.byteLength, entry.previous_bytes, `target size changed: ${entry.path}`);
      assert.equal(sha256(previous.bytes), entry.previous_sha256, `target hash changed: ${entry.path}`);
    }
    if (entry.status === 'same') continue;
    if (entry.status === 'replace') {
      const backup = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.writeFileSync(backup, previous.bytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backup)), entry.previous_sha256, `backup hash differs: ${entry.path}`);
    }
    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = `${destination}.c114-crossaxis-assets-pass02-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), entry.source_sha256, `copied hash differs: ${entry.path}`);
  }
  const report = { copied_or_already_current: true, selected_static_route_count: manifest.routes.length, no_deletes: true };
  fs.writeFileSync(resultPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set');
const localTargets = [];
const backups = [];
const remoteTargets = [];
for (const entry of expected.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size mismatch: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash mismatch: ${entry.path}`);
  const local = inspectPath(targetRoot, entry.path);
  const localVerified = local.exists && local.bytes.byteLength === entry.source_bytes && sha256(local.bytes) === entry.source_sha256;
  localTargets.push({ path: entry.path, bytes: local.bytes?.byteLength ?? null, sha256: local.exists ? sha256(local.bytes) : null, verified: localVerified });
  assert.ok(localVerified, `local preview hash mismatch: ${entry.path}`);
  if (entry.status === 'replace') {
    const backup = inspectPath(backupRoot, entry.path);
    const backupVerified = backup.exists && backup.bytes.byteLength === entry.previous_bytes && sha256(backup.bytes) === entry.previous_sha256;
    backups.push({ path: entry.path, backup_path: entry.backup_path, verified: backupVerified });
    assert.ok(backupVerified, `backup hash mismatch: ${entry.path}`);
  }
  const response = await fetch(`${origin}/spinon/${entry.path}?sync=c114-crossaxis-assets-pass02-20261025`, { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `remote HTTP status for ${entry.path}: ${response.status}`);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  const remoteHash = sha256(remoteBytes);
  const remoteVerified = remoteBytes.byteLength === entry.source_bytes && remoteHash === entry.source_sha256;
  remoteTargets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.byteLength, sha256: remoteHash, verified: remoteVerified });
  assert.ok(remoteVerified, `Tailnet static asset hash mismatch: ${entry.path}`);
}
const report = {
  selected_static_route_count: manifest.routes.length,
  local_targets_verified: localTargets.length,
  backups_verified: backups.length,
  remote_targets_verified: remoteTargets.length,
  all_local_targets_verified: localTargets.every((entry) => entry.verified),
  all_backups_verified: backups.every((entry) => entry.verified),
  all_remote_targets_http_200_and_hash_verified: remoteTargets.every((entry) => entry.verified),
  no_deletes: true,
  local_targets: localTargets,
  backups,
  remote_targets: remoteTargets,
};
fs.writeFileSync(resultPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, local_targets: undefined, backups: undefined, remote_targets: undefined }, null, 2)}\n`);
