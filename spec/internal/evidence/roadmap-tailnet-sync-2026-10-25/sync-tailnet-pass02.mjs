import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-verification-pass02.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass02.json');
const resultPath = path.join(evidenceDirectory, `tailnet-${mode}-pass02.json`);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass02';
const targetRootRelative = 'SpinonPreview';
const backupRootRelative = 'SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass02';
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'selected build verification report is required');
assert.ok(!fs.existsSync(resultPath), `refusing to overwrite ${path.basename(resultPath)}`);
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected documentation output is required');

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
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in target path: ${relative}`);
      else assert.ok(stat.isFile(), `non-file target path: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current) };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function listSelectedRoutes() {
  const routes = selectedBuild.selectedRoutes.map((entry) => entry.path);
  assert.equal(routes.length, new Set(routes).size, 'selected build has duplicate routes');
  for (const route of routes) assertSafeRelative(route);
  return routes;
}

function makePreflight() {
  const routes = listSelectedRoutes();
  const files = routes.map((route) => {
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
      backup_path: status === 'replace' ? `${backupRootRelative}/${route}` : null,
    };
  });
  return {
    pass: 'pass02',
    selected_source_head: selectedBuild.sourceHead,
    selected_route_count: files.length,
    preview_root_relative: targetRootRelative,
    backup_root_relative: backupRootRelative,
    source_bytes_total: files.reduce((sum, entry) => sum + entry.source_bytes, 0),
    backup_bytes_total: files.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
    counts: Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((entry) => entry.status === status).length])),
    no_deletes: true,
    files,
  };
}

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(preflightPath), 'refusing to overwrite preflight report');
  const preflight = makePreflight();
  fs.writeFileSync(preflightPath, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...preflight, files: undefined }, null, 2)}\n`);
  process.exit(0);
}

assert.ok(preflightArgument, `${mode} mode requires a preflight JSON path`);
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
assert.equal(expected.pass, 'pass02', 'preflight pass mismatch');
assert.equal(expected.preview_root_relative, targetRootRelative, 'preflight target mismatch');
assert.equal(expected.backup_root_relative, backupRootRelative, 'preflight backup mismatch');
const routes = listSelectedRoutes();
assert.equal(expected.files.length, routes.length, 'selected route count changed after preflight');
assert.deepEqual(expected.files.map((entry) => entry.path), routes, 'selected route set changed after preflight');

if (mode === 'copy') {
  for (const entry of expected.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size changed: ${entry.path}`);
    assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed: ${entry.path}`);
    const target = inspectPath(targetRoot, entry.path);
    assert.equal(target.exists, entry.target_exists, `target existence changed after preflight: ${entry.path}`);
    if (target.exists) {
      assert.equal(target.bytes.byteLength, entry.previous_bytes, `target size changed after preflight: ${entry.path}`);
      assert.equal(sha256(target.bytes), entry.previous_sha256, `target hash changed after preflight: ${entry.path}`);
    }
    if (entry.status === 'same') continue;

    if (entry.status === 'replace') {
      const backup = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      if (fs.existsSync(backup)) {
        const existingBackup = fs.readFileSync(backup);
        assert.equal(existingBackup.byteLength, entry.previous_bytes, `existing backup size differs: ${entry.path}`);
        assert.equal(sha256(existingBackup), entry.previous_sha256, `existing backup hash differs: ${entry.path}`);
      } else {
        fs.writeFileSync(backup, target.bytes, { flag: 'wx' });
      }
    }

    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = `${destination}.spinon-roadmap-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), entry.source_sha256, `copied hash differs: ${entry.path}`);
  }
  const report = { pass: 'pass02', copied_or_already_current: true, selected_route_count: routes.length, no_deletes: true };
  fs.writeFileSync(resultPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set for remote verification');
const localTargets = [];
const backups = [];
const remoteTargets = [];
for (const entry of expected.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  const target = inspectPath(targetRoot, entry.path);
  const localVerified = target.exists && target.bytes.byteLength === entry.source_bytes
    && sha256(target.bytes) === entry.source_sha256;
  localTargets.push({ path: entry.path, bytes: target.bytes?.byteLength ?? null, sha256: target.exists ? sha256(target.bytes) : null, verified: localVerified });
  assert.ok(localVerified, `local preview hash mismatch: ${entry.path}`);

  if (entry.status === 'replace') {
    const backup = inspectPath(backupRoot, entry.path);
    const backupVerified = backup.exists && backup.bytes.byteLength === entry.previous_bytes
      && sha256(backup.bytes) === entry.previous_sha256;
    backups.push({ path: entry.path, backup_path: `${backupRootRelative}/${entry.path}`, verified: backupVerified });
    assert.ok(backupVerified, `backup hash mismatch: ${entry.path}`);
  }

  assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size changed after preflight: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed after preflight: ${entry.path}`);
  const url = `${origin}/spinon/${entry.path}?sync=roadmap-20261025-pass02`;
  const response = await fetch(url, { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `remote HTTP status for ${entry.path}: ${response.status}`);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  const remoteHash = sha256(remoteBytes);
  const remoteVerified = remoteBytes.byteLength === entry.source_bytes && remoteHash === entry.source_sha256;
  remoteTargets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.byteLength, sha256: remoteHash, verified: remoteVerified });
  assert.ok(remoteVerified, `Tailnet served hash mismatch: ${entry.path}`);
}

const report = {
  pass: 'pass02',
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
