import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-copy.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr710-2026-10-28/pass01';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(preflightPath).isFile(), 'preflight report is missing');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite copy report');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
assert.equal(preflight.allReplacementTargetsClosed, true, 'preflight found an open replacement target');
assert.equal(preflight.noRoutesDeleted, true);
const sourceRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-publication');

function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  const parts = relative.split('/');
  let current = root;
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory path component: ${relative}`);
      else {
        assert.ok(stat.isFile(), `non-file destination: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function ensureSafeParent(root, relative) {
  let current = root;
  for (const part of relative.split('/').slice(0, -1)) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current, { mode: 0o755 });
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory in path: ${relative}`);
  }
  return path.join(root, relative);
}

function writeSyncedFile(temporary, bytes, mode) {
  const fd = fs.openSync(temporary, 'wx', mode);
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
  } catch { /* 일부 파일시스템은 디렉터리 fsync를 지원하지 않는다. */ }
}

fs.mkdirSync(backupRoot, { recursive: true, mode: 0o755 });
assert.ok(fs.lstatSync(backupRoot).isDirectory() && !fs.lstatSync(backupRoot).isSymbolicLink(), 'unsafe backup root');
let created = 0;
let replaced = 0;
let same = 0;
let backupBytes = 0;
const backups = [];

for (const entry of preflight.entries) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size changed: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed: ${entry.path}`);
  const current = inspectPath(targetRoot, entry.path);
  assert.equal(current.exists, entry.target_exists, `target existence changed: ${entry.path}`);
  if (current.exists) {
    assert.equal(current.bytes.byteLength, entry.previous_bytes, `target size changed: ${entry.path}`);
    assert.equal(sha256(current.bytes), entry.previous_sha256, `target hash changed: ${entry.path}`);
  }
  const destination = ensureSafeParent(targetRoot, entry.path);
  if (entry.status === 'same') {
    same += 1;
    continue;
  }

  if (entry.status === 'replace') {
    assert.equal(entry.openHandleCheck?.noOpenHandles, true, `preflight replacement was open: ${entry.path}`);
    const lsof = spawnSync('lsof', ['-nP', destination], { encoding: 'utf8' });
    assert.equal(lsof.error, undefined, `lsof failed for ${entry.path}: ${lsof.error}`);
    assert.equal(lsof.status, 1, `replacement target is open immediately before sync: ${entry.path}\n${lsof.stdout}`);

    const backupDestination = ensureSafeParent(backupRoot, entry.path);
    const backupTemporary = path.join(path.dirname(backupDestination), `.${path.basename(backupDestination)}.pr710-${randomUUID()}.tmp`);
    writeSyncedFile(backupTemporary, current.bytes, current.mode);
    fs.linkSync(backupTemporary, backupDestination);
    fs.unlinkSync(backupTemporary);
    syncDirectory(path.dirname(backupDestination));
    const saved = fs.readFileSync(backupDestination);
    assert.equal(saved.byteLength, entry.previous_bytes, `backup size mismatch: ${entry.path}`);
    assert.equal(sha256(saved), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
    backupBytes += saved.byteLength;
    backups.push({ path: entry.path, bytes: saved.byteLength, sha256: sha256(saved) });

    const temporary = path.join(path.dirname(destination), `.${path.basename(destination)}.pr710-${randomUUID()}.tmp`);
    writeSyncedFile(temporary, sourceBytes, current.mode);
    fs.renameSync(temporary, destination);
    syncDirectory(path.dirname(destination));
    replaced += 1;
  } else {
    assert.equal(entry.status, 'create', `unexpected route status: ${entry.path}`);
    assert.ok(!fs.existsSync(destination), `create route appeared after preflight: ${entry.path}`);
    const temporary = path.join(path.dirname(destination), `.${path.basename(destination)}.pr710-${randomUUID()}.tmp`);
    writeSyncedFile(temporary, sourceBytes, 0o644);
    fs.linkSync(temporary, destination);
    fs.unlinkSync(temporary);
    syncDirectory(path.dirname(destination));
    created += 1;
  }

  const copied = fs.readFileSync(destination);
  assert.equal(copied.byteLength, entry.source_bytes, `copied size mismatch: ${entry.path}`);
  assert.equal(sha256(copied), entry.source_sha256, `copied hash mismatch: ${entry.path}`);
}

const report = {
  sourceHead: preflight.sourceHead,
  sourceTree: preflight.sourceTree,
  targetRoot,
  backupRoot,
  routeCount: preflight.entries.length,
  created,
  replaced,
  unchanged: same,
  backupCount: backups.length,
  backupBytes,
  noRoutesDeleted: true,
  backups,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
