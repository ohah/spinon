import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const selectedBuildPath = path.join(evidenceDirectory, 'postmerge-selected-build-verification.json');
const preflightPath = path.join(evidenceDirectory, 'postmerge-tailnet-preflight.json');
const resultPath = path.join(evidenceDirectory, `postmerge-tailnet-${mode}.json`);
const applicationSupport = path.join(os.homedir(), 'Library', 'Application Support');
const targetRoot = path.join(applicationSupport, 'SpinonPreview');
const backupRoot = path.join(applicationSupport, 'SpinonPreviewBackups/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/pass02');
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-postmerge-preview.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'post-merge selected build report is required');
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'post-merge selected output is required');
assert.ok(!fs.existsSync(resultPath), `refusing to overwrite ${path.basename(resultPath)}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function inspect(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  const parts = relative.split('/');
  let current = root;
  for (let i = 0; i < parts.length; i += 1) {
    current = path.join(current, parts[i]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in route: ${relative}`);
      if (i < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory route parent: ${relative}`);
      else assert.ok(stat.isFile(), `non-file route: ${relative}`);
      if (i === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current) };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function routes() {
  const selected = selectedBuild.selectedRoutes.map((entry) => entry.path);
  assert.equal(selected.length, new Set(selected).size, 'duplicate selected route');
  for (const route of selected) {
    assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
    assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route segment: ${route}`);
  }
  return selected;
}

function createPreflight() {
  const files = routes().map((route) => {
    const bytes = fs.readFileSync(path.join(sourceRoot, route));
    const previous = inspect(targetRoot, route);
    const sourceHash = sha256(bytes);
    const previousHash = previous.exists ? sha256(previous.bytes) : null;
    return {
      path: route,
      status: !previous.exists ? 'create' : previousHash === sourceHash ? 'same' : 'replace',
      source_bytes: bytes.length,
      source_sha256: sourceHash,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.length : null,
      previous_sha256: previousHash,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((file) => file.status === status).length]));
  return {
    selected_route_count: files.length,
    source_bytes_total: files.reduce((sum, file) => sum + file.source_bytes, 0),
    backup_bytes_total: files.filter((file) => file.status === 'replace').reduce((sum, file) => sum + file.previous_bytes, 0),
    counts,
    backup_root_relative: 'SpinonPreviewBackups/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/pass02',
    no_deletes: true,
    files,
  };
}

if (mode === 'preflight') {
  const report = createPreflight();
  fs.writeFileSync(preflightPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, files: undefined }, null, 2)}\n`);
  process.exit(0);
}

assert.ok(preflightArgument, `${mode} requires a preflight report`);
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
const selectedRoutes = routes();
assert.equal(expected.files.length, selectedRoutes.length, 'route count changed after preflight');
assert.deepEqual(expected.files.map((file) => file.path), selectedRoutes, 'route set changed after preflight');

if (mode === 'copy') {
  for (const entry of expected.files) {
    const source = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(source.length, entry.source_bytes, `source size changed: ${entry.path}`);
    assert.equal(sha256(source), entry.source_sha256, `source hash changed: ${entry.path}`);
    const previous = inspect(targetRoot, entry.path);
    assert.equal(previous.exists, entry.target_exists, `preview existence changed: ${entry.path}`);
    if (previous.exists) {
      assert.equal(previous.bytes.length, entry.previous_bytes, `preview size changed: ${entry.path}`);
      assert.equal(sha256(previous.bytes), entry.previous_sha256, `preview hash changed: ${entry.path}`);
    }
    if (entry.status === 'same') continue;
    if (entry.status === 'replace') {
      const backupPath = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backupPath)), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
    }
    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = `${destination}.spinon-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, source, { flag: 'wx' });
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), entry.source_sha256, `copied hash mismatch: ${entry.path}`);
  }
  const report = { copied_or_already_current: true, selected_route_count: selectedRoutes.length, no_deletes: true };
  fs.writeFileSync(resultPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(origin, 'SPINON_PREVIEW_ORIGIN is required for HTTP verification');
const local = [];
const backups = [];
const remote = [];
for (const entry of expected.files) {
  const localFile = inspect(targetRoot, entry.path);
  const localOk = localFile.exists && localFile.bytes.length === entry.source_bytes && sha256(localFile.bytes) === entry.source_sha256;
  local.push({ path: entry.path, verified: localOk });
  assert.ok(localOk, `local preview hash mismatch: ${entry.path}`);
  if (entry.status === 'replace') {
    const saved = inspect(backupRoot, entry.path);
    const backupOk = saved.exists && saved.bytes.length === entry.previous_bytes && sha256(saved.bytes) === entry.previous_sha256;
    backups.push({ path: entry.path, verified: backupOk });
    assert.ok(backupOk, `backup mismatch: ${entry.path}`);
  }
  const response = await fetch(`${origin}/spinon/${entry.path}?sync=postmerge-c114`, { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `HTTP status for ${entry.path}`);
  const bytes = Buffer.from(await response.arrayBuffer());
  const verified = bytes.length === entry.source_bytes && sha256(bytes) === entry.source_sha256;
  remote.push({ path: entry.path, http_status: response.status, bytes: bytes.length, sha256: sha256(bytes), verified });
  assert.ok(verified, `remote hash mismatch: ${entry.path}`);
}
const report = {
  selected_route_count: selectedRoutes.length,
  local_targets_verified: local.length,
  backups_verified: backups.length,
  remote_targets_verified: remote.length,
  all_local_targets_verified: local.every((item) => item.verified),
  all_backups_verified: backups.every((item) => item.verified),
  all_remote_targets_http_200_and_hash_verified: remote.every((item) => item.verified),
  no_deletes: true,
  local,
  backups,
  remote,
};
fs.writeFileSync(resultPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, local: undefined, backups: undefined, remote: undefined }, null, 2)}\n`);
