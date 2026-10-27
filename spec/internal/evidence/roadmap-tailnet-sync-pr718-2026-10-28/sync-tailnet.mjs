import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const publicationPath = path.join(evidenceDirectory, 'selected-publication.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-copy.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr718-2026-10-28/pass01';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);
assert.equal(preflight.allReplacementTargetsClosed, true, 'preflight replacement target is open');
assert.equal(preflight.noRoutesDeleted, true);
assert.equal(publication.sourceHead, preflight.sourceHead);
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
fs.mkdirSync(backupRoot, { recursive: true, mode: 0o755 });
assert.ok(fs.lstatSync(backupRoot).isDirectory() && !fs.lstatSync(backupRoot).isSymbolicLink());

function inspectPath(relative) {
  let current = targetRoot;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `unsafe target parent: ${relative}`);
      else {
        assert.ok(stat.isFile(), `target is not a regular file: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function ensureParent(relative) {
  let current = targetRoot;
  for (const part of relative.split('/').slice(0, -1)) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current, { mode: 0o755 });
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe parent directory: ${relative}`);
  }
  return path.join(targetRoot, relative);
}
function writeSynced(file, bytes, mode) {
  const fd = fs.openSync(file, 'wx', mode);
  try {
    fs.writeFileSync(fd, bytes);
    fs.fsyncSync(fd);
  } finally {
    fs.closeSync(fd);
  }
}
function syncDirectory(directory) {
  try {
    const fd = fs.openSync(directory, 'r');
    try { fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
  } catch { /* 일부 파일 시스템은 디렉터리 fsync를 지원하지 않는다. */ }
}

let created = 0;
let replaced = 0;
let same = 0;
let backupBytes = 0;
const updates = [];
const backups = [];
for (const source of publication.routes) {
  const entry = preflight.entries.find((candidate) => candidate.path === source.path);
  assert.ok(entry, `preflight entry missing: ${source.path}`);
  const bytes = fs.readFileSync(path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr718-publication-selected', source.path));
  assert.equal(bytes.length, source.bytes, `publication size changed: ${source.path}`);
  assert.equal(sha256(bytes), source.sha256, `publication hash changed: ${source.path}`);
  const destination = ensureParent(source.path);
  const current = inspectPath(source.path);
  assert.equal(current.exists, entry.target_exists, `target existence changed after preflight: ${source.path}`);
  if (current.exists) {
    assert.equal(current.bytes.length, entry.previous_bytes, `target size changed after preflight: ${source.path}`);
    assert.equal(sha256(current.bytes), entry.previous_sha256, `target hash changed after preflight: ${source.path}`);
  }
  if (entry.status === 'same') {
    same += 1;
    updates.push({ path: source.path, result: 'same', bytes: bytes.length, sha256: source.sha256 });
    continue;
  }
  if (entry.status === 'replace') {
    const open = spawnSync('lsof', ['-nP', destination], { encoding: 'utf8' });
    assert.notEqual(open.status, 0, `replacement target is open: ${source.path}\n${open.stdout}`);
    const backup = path.join(backupRoot, source.path);
    fs.mkdirSync(path.dirname(backup), { recursive: true, mode: 0o755 });
    writeSynced(backup, current.bytes, current.mode);
    assert.deepEqual(fs.readFileSync(backup), current.bytes, `backup bytes differ: ${source.path}`);
    backups.push({ path: source.path, bytes: current.bytes.length, sha256: sha256(current.bytes), mode: current.mode });
    backupBytes += current.bytes.length;
    replaced += 1;
  } else {
    assert.equal(entry.status, 'create', `unknown preflight status: ${entry.status}`);
    created += 1;
  }

  const mode = entry.previous_mode ?? source.mode ?? 0o644;
  const temporary = path.join(path.dirname(destination), `.${path.basename(destination)}.sync-${randomUUID()}`);
  writeSynced(temporary, bytes, mode);
  fs.renameSync(temporary, destination);
  syncDirectory(path.dirname(destination));
  const servedLocal = fs.readFileSync(destination);
  assert.equal(servedLocal.length, bytes.length, `written size differs: ${source.path}`);
  assert.equal(sha256(servedLocal), source.sha256, `written hash differs: ${source.path}`);
  updates.push({ path: source.path, result: entry.status, bytes: bytes.length, sha256: source.sha256 });
}

const backupManifest = {
  sourceHead: publication.sourceHead,
  backupRoot,
  fileCount: backups.length,
  totalBytes: backupBytes,
  files: backups.sort((a, b) => a.path.localeCompare(b.path)),
};
fs.writeFileSync(path.join(backupRoot, 'backup-manifest.json'), `${JSON.stringify(backupManifest, null, 2)}\n`, { flag: 'wx' });
const report = {
  sourceHead: publication.sourceHead,
  sourceTree: publication.sourceTree,
  targetRoot,
  backupRoot,
  routeCount: updates.length,
  created,
  replaced,
  unchanged: same,
  backupBytes,
  backups: backupManifest.files,
  updates: updates.sort((a, b) => a.path.localeCompare(b.path)),
  noRoutesDeleted: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, backups: undefined, updates: undefined }, null, 2)}\n`);
