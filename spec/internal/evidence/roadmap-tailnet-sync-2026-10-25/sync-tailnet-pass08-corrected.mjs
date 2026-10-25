import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass08-corrected.json');
const handleReportPath = path.join(evidenceDirectory, 'target-open-handles-pass08-corrected.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-copy-pass08-corrected.json');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-verification-pass08-corrected.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08-corrected';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const required of [preflightPath, handleReportPath, selectedBuildPath]) assert.ok(fs.lstatSync(required).isFile(), `required pass08-corrected input is missing: ${required}`);
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite pass08-corrected copy report');
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
const handleReport = JSON.parse(fs.readFileSync(handleReportPath, 'utf8'));
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
assert.equal(preflight.pass, 'pass08-corrected');
assert.equal(handleReport.pass, 'pass08-corrected');
assert.equal(handleReport.all_replacements_closed, true, 'replacement files are not all closed');
assert.equal(preflight.selected_source_head, selectedBuild.sourceHead);
assert.equal(preflight.selected_source_tree, selectedBuild.sourceTree);
assert.equal(preflight.files.length, selectedBuild.selectedRouteCount);
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);

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
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function assertCurrent(entry, bytes) {
  assert.equal(bytes.byteLength, entry.source_bytes, `source size changed since preflight: ${entry.path}`);
  assert.equal(sha256(bytes), entry.source_sha256, `source hash changed since preflight: ${entry.path}`);
}

function syncFile(tempPath, bytes, mode) {
  const fd = fs.openSync(tempPath, 'wx', mode ?? 0o644);
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
  } catch { /* Some filesystems do not allow fsync on directory handles. */ }
}

function ensureSafeParent(root, relative) {
  let current = root;
  for (const part of relative.split('/').slice(0, -1)) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current, { mode: 0o755 });
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory in target path: ${relative}`);
  }
  return path.join(root, relative);
}

function atomicCreate(destination, bytes, mode) {
  const parent = path.dirname(destination);
  const temporary = path.join(parent, `.${path.basename(destination)}.spinon-pass08-corrected-${randomUUID()}.tmp`);
  syncFile(temporary, bytes, mode);
  try {
    fs.linkSync(temporary, destination);
    fs.unlinkSync(temporary);
    syncDirectory(parent);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
}

function atomicReplace(destination, bytes, mode) {
  const parent = path.dirname(destination);
  const temporary = path.join(parent, `.${path.basename(destination)}.spinon-pass08-corrected-${randomUUID()}.tmp`);
  syncFile(temporary, bytes, mode);
  try {
    fs.renameSync(temporary, destination);
    syncDirectory(parent);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
}

assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected pass08-corrected root is missing');
assert.ok(fs.lstatSync(targetRoot).isDirectory() && !fs.lstatSync(targetRoot).isSymbolicLink(), 'unsafe preview root');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse pass08-corrected backup root: ${backupRoot}`);
fs.mkdirSync(backupRoot, { recursive: true, mode: 0o755 });
assert.ok(fs.lstatSync(backupRoot).isDirectory() && !fs.lstatSync(backupRoot).isSymbolicLink(), 'unsafe pass08-corrected backup root');

for (const entry of preflight.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assertCurrent(entry, sourceBytes);
  const current = inspectPath(targetRoot, entry.path);
  assert.equal(current.exists, entry.target_exists, `preview route existence changed since preflight: ${entry.path}`);
  if (current.exists) {
    assert.equal(current.bytes.byteLength, entry.previous_bytes, `preview route size changed since preflight: ${entry.path}`);
    assert.equal(sha256(current.bytes), entry.previous_sha256, `preview route hash changed since preflight: ${entry.path}`);
  }
  if (entry.status === 'same') continue;

  const destination = ensureSafeParent(targetRoot, entry.path);
  if (entry.status === 'replace') {
    const lsof = spawnSync('lsof', ['-nP', destination], { encoding: 'utf8' });
    assert.equal(lsof.error, undefined, `lsof failed for ${entry.path}: ${lsof.error}`);
    assert.equal(lsof.status, 1, `replacement target is open immediately before copy: ${entry.path}\n${lsof.stdout}`);

    const backupRelative = entry.backup_path.replace(/^SpinonPreviewBackups\/roadmap-tailnet-sync-2026-10-25\/pass08-corrected\//, '');
    assert.ok(backupRelative !== entry.backup_path, `invalid backup route: ${entry.path}`);
    const backupDestination = ensureSafeParent(backupRoot, backupRelative);
    if (fs.existsSync(backupDestination)) {
      const existing = fs.lstatSync(backupDestination);
      assert.ok(existing.isFile() && !existing.isSymbolicLink(), `unsafe existing pass08-corrected backup: ${entry.path}`);
      const existingBytes = fs.readFileSync(backupDestination);
      assert.equal(existingBytes.byteLength, entry.previous_bytes, `existing backup size mismatch: ${entry.path}`);
      assert.equal(sha256(existingBytes), entry.previous_sha256, `existing backup hash mismatch: ${entry.path}`);
    } else {
      const backupTemporary = path.join(path.dirname(backupDestination), `.${path.basename(backupDestination)}.spinon-pass08-corrected-${randomUUID()}.tmp`);
      syncFile(backupTemporary, current.bytes, entry.previous_mode ?? 0o644);
      try {
        fs.linkSync(backupTemporary, backupDestination);
        fs.unlinkSync(backupTemporary);
        syncDirectory(path.dirname(backupDestination));
      } catch (error) {
        if (fs.existsSync(backupTemporary)) fs.unlinkSync(backupTemporary);
        throw error;
      }
    }
    const verifiedBackup = fs.readFileSync(backupDestination);
    assert.equal(verifiedBackup.byteLength, entry.previous_bytes, `backup size mismatch after write: ${entry.path}`);
    assert.equal(sha256(verifiedBackup), entry.previous_sha256, `backup hash mismatch after write: ${entry.path}`);
    atomicReplace(destination, sourceBytes, entry.previous_mode ?? 0o644);
  } else {
    assert.equal(entry.status, 'create', `unexpected status: ${entry.status}`);
    assert.ok(!fs.existsSync(destination), `create route appeared after preflight: ${entry.path}`);
    atomicCreate(destination, sourceBytes, 0o644);
  }
  const copied = fs.readFileSync(destination);
  assert.equal(copied.byteLength, entry.source_bytes, `copied size mismatch: ${entry.path}`);
  assert.equal(sha256(copied), entry.source_sha256, `copied hash mismatch: ${entry.path}`);
}

const report = {
  pass: 'pass08-corrected',
  selected_source_head: selectedBuild.sourceHead,
  selected_source_tree: selectedBuild.sourceTree,
  copied_or_already_current: true,
  selected_route_count: preflight.files.length,
  copied_replacements: preflight.counts.replace,
  created_routes: preflight.counts.create,
  unchanged_routes: preflight.counts.same,
  byte_exact_backups: preflight.counts.replace,
  backup_root: preflight.backup_root_relative,
  no_deletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
