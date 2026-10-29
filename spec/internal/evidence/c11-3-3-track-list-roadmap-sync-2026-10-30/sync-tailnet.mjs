import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-verification-pass03.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass03.json');
const resultPath = path.join(evidenceDirectory, 'tailnet-' + mode + '-pass03.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-3-3-track-list-roadmap-sync-2026-10-30/pass03';
const targetRootRelative = 'SpinonPreview';
const backupRootRelative = 'SpinonPreviewBackups/c11-3-3-track-list-roadmap-sync-2026-10-30/pass03';
const origin = process.env.SPINON_PREVIEW_ORIGIN ? process.env.SPINON_PREVIEW_ORIGIN.replace(/\/$/, '') : '';
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'selected build verification report is required');
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected documentation build is required');
assert.ok(!fs.existsSync(resultPath), 'refusing to overwrite ' + path.basename(resultPath));

function sha256(bytes) { return createHash('sha256').update(bytes).digest('hex'); }

function discoverPreviewServerPid() {
  const output = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim();
  const pids = [...new Set(output.split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, 'expected one Tailnet preview listener on port 8768');
  const pid = pids[0];
  const args = execFileSync('ps', ['-p', pid, '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), 'unexpected preview listener: ' + args);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', pid, '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  assert.equal(sockets.length, 1, 'preview server has unexpected network handles');
  assert.ok(sockets[0].includes('127.0.0.1:8768') && sockets[0].includes('(LISTEN)'), 'preview socket is unexpected');
  return pid;
}
const previewServerPid = discoverPreviewServerPid();

function assertSafeRelative(relative) {
  assert.ok(typeof relative === 'string' && relative.length > 0, 'empty route');
  assert.ok(!relative.startsWith('/') && !relative.includes('\\'), 'unsafe route: ' + relative);
  assert.ok(relative.split('/').every((part) => part && part !== '.' && part !== '..'), 'unsafe route: ' + relative);
}

function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'unsafe root: ' + root);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), 'symlink in target path: ' + relative);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), 'non-directory in target path: ' + relative);
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

function selectedFiles() {
  const entries = selectedBuild.files;
  assert.ok(Array.isArray(entries) && entries.length === selectedBuild.fileCount, 'selected build manifest is malformed');
  const names = entries.map((entry) => entry.path);
  assert.equal(names.length, new Set(names).size, 'selected build has duplicate paths');
  for (const entry of entries) assertSafeRelative(entry.path);
  return entries;
}

function makePreflight() {
  const files = selectedFiles().map((entry) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.bytes, 'selected source size changed: ' + entry.path);
    assert.equal(sha256(sourceBytes), entry.sha256, 'selected source hash changed: ' + entry.path);
    const previous = inspectPath(targetRoot, entry.path);
    const previousHash = previous.exists ? sha256(previous.bytes) : null;
    return {
      path: entry.path,
      status: !previous.exists ? 'create' : previousHash === entry.sha256 ? 'same' : 'replace',
      source_bytes: entry.bytes,
      source_sha256: entry.sha256,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.byteLength : null,
      previous_sha256: previousHash,
      backup_path: previous.exists && previousHash !== entry.sha256 ? backupRootRelative + '/' + entry.path : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((entry) => entry.status === status).length]));
  return {
    selected_output_relative: path.relative(repositoryRoot, sourceRoot),
    selected_route_count: files.length,
    preview_root_relative: targetRootRelative,
    backup_root_relative: backupRootRelative,
    source_bytes_total: files.reduce((sum, entry) => sum + entry.source_bytes, 0),
    backup_bytes_total: files.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
    counts,
    no_deletes: true,
    files,
  };
}

if (mode === 'preflight') {
  const preflight = makePreflight();
  fs.writeFileSync(preflightPath, JSON.stringify(preflight, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...preflight, files: undefined }, null, 2) + '\n');
  process.exit(0);
}

assert.ok(preflightArgument, mode + ' mode requires a preflight JSON path');
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
assert.equal(expected.preview_root_relative, targetRootRelative, 'preflight target mismatch');
assert.equal(expected.backup_root_relative, backupRootRelative, 'preflight backup mismatch');
const entries = selectedFiles();
assert.equal(expected.files.length, entries.length, 'selected route count changed after preflight');
assert.deepEqual(expected.files.map((entry) => entry.path), entries.map((entry) => entry.path), 'selected path set changed after preflight');

function openHandles(pathname) {
  try {
    const output = execFileSync('lsof', ['-nP', '-t', pathname], { encoding: 'utf8' }).trim();
    return [...new Set(output.split(/\s+/).filter(Boolean))];
  } catch (error) {
    if (error.status === 1) return [];
    throw error;
  }
}

if (mode === 'copy') {
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse backup directory');
  const copied = [];
  const backups = [];
  for (const entry of expected.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
    assert.equal(sourceBytes.byteLength, entry.source_bytes, 'source size changed: ' + entry.path);
    assert.equal(sha256(sourceBytes), entry.source_sha256, 'source changed after preflight: ' + entry.path);
    const previous = inspectPath(targetRoot, entry.path);
    assert.equal(previous.exists, entry.target_exists, 'target existence changed: ' + entry.path);
    if (previous.exists) assert.equal(sha256(previous.bytes), entry.previous_sha256, 'target changed after preflight: ' + entry.path);
    if (entry.status === 'same') continue;
    if (entry.status === 'replace') {
      const holders = openHandles(path.join(targetRoot, entry.path));
      assert.ok(holders.every((pid) => pid === previewServerPid), 'unexpected process holds target: ' + entry.path + ' pids=' + holders.join(','));
      const backupPath = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
      const backupBytes = fs.readFileSync(backupPath);
      assert.equal(backupBytes.byteLength, entry.previous_bytes, 'backup verification failed: ' + entry.path);
      assert.equal(sha256(backupBytes), entry.previous_sha256, 'backup verification failed: ' + entry.path);
      backups.push({ path: entry.path, backup_path: backupRootRelative + '/' + entry.path, bytes: backupBytes.byteLength, sha256: sha256(backupBytes) });
    }
    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = destination + '.spinon-roadmap-sync-' + process.pid + '-' + randomUUID() + '.tmp';
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporary, destination);
    const targetBytes = fs.readFileSync(destination);
    assert.equal(sha256(targetBytes), entry.source_sha256, 'copied target hash differs: ' + entry.path);
    copied.push({ path: entry.path, status: entry.status, bytes: targetBytes.byteLength, sha256: sha256(targetBytes) });
  }
  const report = { copied_count: copied.length, backup_count: backups.length, no_deletes: true, copied, backups };
  fs.writeFileSync(resultPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, copied: undefined, backups: undefined }, null, 2) + '\n');
  process.exit(0);
}

assert.equal(mode, 'verify');
assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set for remote verification');
const targets = [];
const backups = [];
for (const entry of expected.files) {
  const local = inspectPath(targetRoot, entry.path);
  assert.ok(local.exists && local.bytes.byteLength === entry.source_bytes && sha256(local.bytes) === entry.source_sha256, 'local preview hash mismatch: ' + entry.path);
  if (entry.status === 'replace') {
    const backup = inspectPath(backupRoot, entry.path);
    assert.ok(backup.exists && backup.bytes.byteLength === entry.previous_bytes && sha256(backup.bytes) === entry.previous_sha256, 'backup hash mismatch: ' + entry.path);
    backups.push({ path: entry.path, backup_verified: true });
  }
  const response = await fetch(origin + '/spinon/' + entry.path + '?sync=c11-3-3-track-list-roadmap-2026-10-30-pass03', { headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, 'remote HTTP status for ' + entry.path + ': ' + response.status);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  assert.equal(remoteBytes.byteLength, entry.source_bytes, 'remote byte count differs: ' + entry.path);
  assert.equal(sha256(remoteBytes), entry.source_sha256, 'remote hash differs: ' + entry.path);
  targets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.byteLength, sha256: sha256(remoteBytes), local_verified: true, remote_verified: true });
}
const report = {
  selected_route_count: expected.files.length,
  local_targets_verified: targets.length,
  backups_verified: backups.length,
  remote_http_200_and_hash_verified: targets.length,
  all_local_targets_verified: targets.every((entry) => entry.local_verified),
  all_backups_verified: backups.every((entry) => entry.backup_verified),
  all_remote_targets_verified: targets.every((entry) => entry.remote_verified),
  no_deletes: true,
  targets,
  backups,
};
fs.writeFileSync(resultPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, targets: undefined, backups: undefined }, null, 2) + '\n');
