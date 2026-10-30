import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const publicationPath = path.join(evidenceDirectory, 'selected-publication-pass02.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass01.json');
const resultFilename = process.env.SPINON_PREVIEW_REPORT ?? `tailnet-${mode}-pass01.json`;
assert.match(resultFilename, /^tailnet-[a-z0-9-]+\.json$/, 'report name must be a local evidence filename');
const resultPath = path.join(evidenceDirectory, resultFilename);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-finite-ratio-tailnet-roadmap-sync-2026-10-30/pass01';
const targetRootRelative = 'SpinonPreview';
const backupRootRelative = 'SpinonPreviewBackups/c11-4-finite-ratio-tailnet-roadmap-sync-2026-10-30/pass01';
const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '') ?? '';
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repositoryRoot, encoding: 'utf8' }).trim();

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify [PREFLIGHT_JSON]');
assert.ok(fs.lstatSync(publicationPath).isFile(), 'selected publication manifest is required');
const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
assert.equal(git(['rev-parse', 'HEAD']), publication.sourceHead);
assert.equal(git(['rev-parse', 'origin/main']), publication.sourceHead);
assert.equal(publication.routeCount, publication.files.length);
assert.ok(fs.lstatSync(targetRoot).isDirectory() && !fs.lstatSync(targetRoot).isSymbolicLink(), 'unsafe preview target root');
assert.ok(!fs.existsSync(resultPath), `refusing to overwrite ${path.basename(resultPath)}`);

function discoverPreviewServerPid() {
  const output = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim();
  const pids = [...new Set(output.split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, 'expected one Tailnet preview listener on port 8768');
  const pid = pids[0];
  const args = execFileSync('ps', ['-p', pid, '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `unexpected preview listener: ${args}`);
  const listeners = execFileSync('lsof', ['-nP', '-a', '-p', pid, '-iTCP', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  assert.equal(listeners.length, 1, 'preview server has unexpected listening sockets');
  assert.ok(listeners[0].includes('127.0.0.1:8768') && listeners[0].includes('(LISTEN)'), 'preview listener is unexpected');
  return pid;
}
const previewServerPid = discoverPreviewServerPid();

function safeRelative(relative) {
  assert.ok(typeof relative === 'string' && relative.length > 0);
  assert.ok(!relative.startsWith('/') && !relative.includes('\\'), `unsafe route: ${relative}`);
  assert.ok(relative.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${relative}`);
}
function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory path component: ${relative}`);
      else {
        assert.ok(stat.isFile(), `non-file target: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current) };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function openHandles(filePath) {
  try {
    const output = execFileSync('lsof', ['-nP', '-t', filePath], { encoding: 'utf8' }).trim();
    return [...new Set(output.split(/\s+/).filter(Boolean))];
  } catch (error) {
    if (error.status === 1) return [];
    throw error;
  }
}

function makePreflight() {
  const files = publication.files.map((entry) => {
    safeRelative(entry.path);
    const source = path.join(repositoryRoot, 'build/spinon/c11-4-finite-ratio-tailnet-sync-2026-10-30/pass02/publication', entry.path);
    const sourceBytes = fs.readFileSync(source);
    assert.equal(sourceBytes.length, entry.bytes, `publication size changed: ${entry.path}`);
    assert.equal(hash(sourceBytes), entry.sha256, `publication hash changed: ${entry.path}`);
    const previous = inspectPath(targetRoot, entry.path);
    const previousHash = previous.exists ? hash(previous.bytes) : null;
    return {
      path: entry.path,
      status: !previous.exists ? 'create' : previousHash === entry.sha256 ? 'same' : 'replace',
      source_bytes: entry.bytes,
      source_sha256: entry.sha256,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.length : null,
      previous_sha256: previousHash,
      backup_path: previous.exists && previousHash !== entry.sha256 ? `${backupRootRelative}/${entry.path}` : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((entry) => entry.status === status).length]));
  return {
    sourceHead: publication.sourceHead,
    sourceTree: publication.sourceTree,
    previewServerPid,
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
  assert.ok(!fs.existsSync(preflightPath), 'refusing to overwrite tailnet-preflight-pass01.json');
  const preflight = makePreflight();
  fs.writeFileSync(preflightPath, JSON.stringify(preflight, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...preflight, files: undefined }, null, 2) + '\n');
  process.exit(0);
}

assert.ok(preflightArgument, `${mode} mode requires a preflight JSON path`);
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDirectory, preflightArgument), 'utf8'));
assert.equal(expected.sourceHead, publication.sourceHead);
assert.equal(expected.preview_root_relative, targetRootRelative);
assert.equal(expected.backup_root_relative, backupRootRelative);
assert.equal(expected.files.length, publication.files.length);
assert.deepEqual(expected.files.map((entry) => entry.path), publication.files.map((entry) => entry.path));

if (mode === 'copy') {
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse existing backup directory');
  const copied = [];
  const backups = [];
  for (let index = 0; index < expected.files.length; index += 1) {
    const entry = expected.files[index];
    const manifestEntry = publication.files[index];
    const sourcePath = path.join(repositoryRoot, 'build/spinon/c11-4-finite-ratio-tailnet-sync-2026-10-30/pass02/publication', entry.path);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceBytes.length, entry.source_bytes, `source size changed after preflight: ${entry.path}`);
    assert.equal(hash(sourceBytes), entry.source_sha256, `source hash changed after preflight: ${entry.path}`);
    assert.equal(hash(sourceBytes), manifestEntry.sha256, `manifest mismatch after preflight: ${entry.path}`);
    const previous = inspectPath(targetRoot, entry.path);
    assert.equal(previous.exists, entry.target_exists, `target existence changed after preflight: ${entry.path}`);
    if (previous.exists) assert.equal(hash(previous.bytes), entry.previous_sha256, `target changed after preflight: ${entry.path}`);
    if (entry.status === 'same') continue;
    if (entry.status === 'replace') {
      const holders = openHandles(path.join(targetRoot, entry.path));
      assert.ok(holders.every((pid) => pid === previewServerPid), `unexpected process holds target: ${entry.path} pids=${holders.join(',')}`);
      const backupPath = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
      const backupBytes = fs.readFileSync(backupPath);
      assert.equal(backupBytes.length, entry.previous_bytes, `backup length mismatch: ${entry.path}`);
      assert.equal(hash(backupBytes), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
      backups.push({ path: entry.path, backup_path: `${backupRootRelative}/${entry.path}`, bytes: backupBytes.length, sha256: hash(backupBytes) });
    }
    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = `${destination}.spinon-tailnet-${process.pid}-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporary, destination);
    const copiedBytes = fs.readFileSync(destination);
    assert.equal(copiedBytes.length, entry.source_bytes, `copied length mismatch: ${entry.path}`);
    assert.equal(hash(copiedBytes), entry.source_sha256, `copied hash mismatch: ${entry.path}`);
    copied.push({ path: entry.path, status: entry.status, bytes: copiedBytes.length, sha256: hash(copiedBytes) });
  }
  const report = { sourceHead: publication.sourceHead, copied_count: copied.length, backup_count: backups.length, no_deletes: true, copied, backups };
  fs.writeFileSync(resultPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, copied: undefined, backups: undefined }, null, 2) + '\n');
  process.exit(0);
}

assert.equal(mode, 'verify');
assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set for remote verification');
const parsedOrigin = new URL(origin);
assert.equal(parsedOrigin.protocol, 'https:', 'remote verification requires HTTPS');
assert.equal(parsedOrigin.origin, origin, 'SPINON_PREVIEW_ORIGIN must be an origin without a path');
const targets = [];
const backups = [];
for (const entry of expected.files) {
  const local = inspectPath(targetRoot, entry.path);
  assert.ok(local.exists && local.bytes.length === entry.source_bytes && hash(local.bytes) === entry.source_sha256, `local target hash mismatch: ${entry.path}`);
  if (entry.status === 'replace') {
    const backup = inspectPath(backupRoot, entry.path);
    assert.ok(backup.exists && backup.bytes.length === entry.previous_bytes && hash(backup.bytes) === entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
    backups.push({ path: entry.path, backup_verified: true });
  }
  const requestedUrl = new URL(`/spinon/${entry.path}?sync=c11-4-finite-ratio-pr843-pass01`, origin);
  const response = await fetch(requestedUrl, { redirect: 'manual', headers: { 'cache-control': 'no-cache' } });
  assert.equal(response.status, 200, `remote HTTP status for ${entry.path}: ${response.status}`);
  assert.equal(response.url, requestedUrl.href, `remote verification redirected: ${entry.path}`);
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  assert.equal(remoteBytes.length, entry.source_bytes, `remote byte count differs: ${entry.path}`);
  assert.equal(hash(remoteBytes), entry.source_sha256, `remote hash differs: ${entry.path}`);
  targets.push({ path: entry.path, http_status: response.status, bytes: remoteBytes.length, sha256: hash(remoteBytes), local_verified: true, remote_verified: true });
}
const report = {
  sourceHead: publication.sourceHead,
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
