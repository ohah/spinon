import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-verification.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const resultPath = path.join(evidenceDirectory, `tailnet-${mode}.json`);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-fixed-auto-repeat-template-areas-tailnet-sync-2026-10-25/pass01';
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'selected build verification report is required');
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected documentation build is required');
assert.ok(!fs.existsSync(resultPath), `refusing to overwrite ${path.basename(resultPath)}`);

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
function assertSafeRelative(relative) {
  assert.ok(typeof relative === 'string' && relative.length > 0, 'empty route');
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
function listRoutes() {
  const routes = selectedBuild.selectedRoutes.map((entry) => entry.path);
  assert.equal(routes.length, new Set(routes).size, 'selected build has duplicate routes');
  for (const route of routes) assertSafeRelative(route);
  return routes;
}
function makePreflight() {
  const files = listRoutes().map((route) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, route));
    const previous = inspectPath(targetRoot, route);
    const sourceHash = sha256(sourceBytes);
    const previousHash = previous.exists ? sha256(previous.bytes) : null;
    const status = !previous.exists ? 'create' : previousHash === sourceHash ? 'same' : 'replace';
    return {
      path: route,
      status,
      source_bytes: sourceBytes.byteLength,
      source_sha256: sourceHash,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.byteLength : null,
      previous_sha256: previousHash,
      backup_path: status === 'replace' ? path.relative('/Users/yoonhb/Library/Application Support', path.join(backupRoot, route)) : null,
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
  const preflight = makePreflight();
  fs.writeFileSync(preflightPath, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...preflight, files: undefined }, null, 2)}\n`);
  process.exit(0);
}
assert.ok(preflightArgument, `${mode} mode requires a preflight JSON path`);
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
assert.equal(expected.preview_root_relative, 'SpinonPreview', 'preflight preview root mismatch');
assert.equal(expected.backup_root_relative, path.relative('/Users/yoonhb/Library/Application Support', backupRoot), 'preflight backup root mismatch');
const routes = listRoutes();
assert.equal(expected.files.length, routes.length, 'selected route count changed after preflight');
assert.deepEqual(expected.files.map((entry) => entry.path), routes, 'selected route set changed after preflight');

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
    const temporary = `${destination}.c114-pass01-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), entry.source_sha256, `copied hash differs: ${entry.path}`);
  }
  const report = { copied_or_already_current: true, selected_route_count: routes.length, no_deletes: true };
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
  const sourceVerified = sourceBytes.byteLength === entry.source_bytes && sha256(sourceBytes) === entry.source_sha256;
  assert.ok(sourceVerified, `source hash mismatch: ${entry.path}`);
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
  const url = `${origin}/spinon/${entry.path}?sync=c114-merge-20261025`;
  const response = await fetch(url, { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `remote HTTP status for ${entry.path}: ${response.status}`);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  const remoteHash = sha256(remoteBytes);
  const remoteVerified = remoteBytes.byteLength === entry.source_bytes && remoteHash === entry.source_sha256;
  remoteTargets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.byteLength, sha256: remoteHash, verified: remoteVerified });
  assert.ok(remoteVerified, `Tailnet served hash mismatch: ${entry.path}`);
}
const report = {
  selected_route_count: routes.length,
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
