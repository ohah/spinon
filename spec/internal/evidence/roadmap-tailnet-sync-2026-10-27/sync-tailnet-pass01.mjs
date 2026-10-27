import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass01.json');
const handleReportPath = path.join(evidenceDirectory, 'target-open-handles-pass01.json');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-copy-pass01.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-27/pass01';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const required of [preflightPath, handleReportPath, selectedBuildPath]) assert.ok(fs.lstatSync(required).isFile(), `required input is missing: ${required}`);
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite pass01 copy report');
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
const handleReport = JSON.parse(fs.readFileSync(handleReportPath, 'utf8'));
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
assert.equal(preflight.pass, 'pass01');
assert.equal(handleReport.pass, 'pass01');
assert.equal(selectedBuild.pass, 'pass01');
assert.equal(handleReport.all_replacements_closed, true, 'replacement targets are not all closed');
assert.equal(preflight.selected_source_tree_matches_merged_main, true, 'selected tree does not match merged main');
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
      assert.ok(!stat.isSymbolicLink(), `symlink in path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in path: ${relative}`);
      else assert.ok(stat.isFile(), `non-file in path: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
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

function syncFile(temporary, bytes, mode) {
  const fd = fs.openSync(temporary, 'wx', mode ?? 0o644);
  try { fs.writeFileSync(fd, bytes); fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
}

function syncDirectory(directory) {
  try { const fd = fs.openSync(directory, 'r'); try { fs.fsyncSync(fd); } finally { fs.closeSync(fd); } } catch { /* 일부 파일시스템은 디렉터리 fsync를 지원하지 않는다. */ }
}

assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
fs.mkdirSync(backupRoot, { recursive: true, mode: 0o755 });
assert.ok(fs.lstatSync(backupRoot).isDirectory() && !fs.lstatSync(backupRoot).isSymbolicLink(), 'unsafe backup root');

for (const entry of preflight.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assert.equal(sourceBytes.byteLength, entry.source_bytes, `source size changed since preflight: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed since preflight: ${entry.path}`);
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
    const backupDestination = ensureSafeParent(backupRoot, entry.path);
    const backupTemporary = path.join(path.dirname(backupDestination), `.${path.basename(backupDestination)}.spinon-pass01-${randomUUID()}.tmp`);
    syncFile(backupTemporary, current.bytes, entry.previous_mode ?? 0o644);
    fs.linkSync(backupTemporary, backupDestination);
    fs.unlinkSync(backupTemporary);
    syncDirectory(path.dirname(backupDestination));
    const backup = fs.readFileSync(backupDestination);
    assert.equal(backup.byteLength, entry.previous_bytes, `backup size mismatch: ${entry.path}`);
    assert.equal(sha256(backup), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);

    const temporary = path.join(path.dirname(destination), `.${path.basename(destination)}.spinon-pass01-${randomUUID()}.tmp`);
    syncFile(temporary, sourceBytes, entry.previous_mode ?? 0o644);
    fs.renameSync(temporary, destination);
    syncDirectory(path.dirname(destination));
  } else {
    assert.equal(entry.status, 'create', `unexpected route status: ${entry.path}`);
    assert.ok(!fs.existsSync(destination), `create route appeared after preflight: ${entry.path}`);
    const temporary = path.join(path.dirname(destination), `.${path.basename(destination)}.spinon-pass01-${randomUUID()}.tmp`);
    syncFile(temporary, sourceBytes, 0o644);
    fs.linkSync(temporary, destination);
    fs.unlinkSync(temporary);
    syncDirectory(path.dirname(destination));
  }
  const copied = fs.readFileSync(destination);
  assert.equal(copied.byteLength, entry.source_bytes, `copied size mismatch: ${entry.path}`);
  assert.equal(sha256(copied), entry.source_sha256, `copied hash mismatch: ${entry.path}`);
}

const report = {
  pass: 'pass01', selected_source_head: selectedBuild.sourceHead, selected_source_tree: selectedBuild.sourceTree,
  copied_or_already_current: true, selected_route_count: preflight.files.length,
  copied_replacements: preflight.counts.replace, created_routes: preflight.counts.create,
  unchanged_routes: preflight.counts.same, byte_exact_backups: preflight.counts.replace,
  backup_root: 'SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-27/pass01', no_deletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
