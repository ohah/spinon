import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-manifest-pass03.json'), 'utf8'));
const buildBase = path.join(repo, manifest.buildBase);
const previewRoot = manifest.previewRoot;
const backupRoot = manifest.backupRoot;
const reportPath = path.join(evidence, 'selected-publication-result-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), `preserve existing publish report: ${reportPath}`);
assert.ok(!fs.existsSync(backupRoot), `preserve existing backup root: ${backupRoot}`);

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}

function ensureDirectory(directory, root) {
  const relative = path.relative(root, directory);
  assert.ok(relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative)), `directory outside root: ${directory}`);
  let current = root;
  for (const part of relative ? relative.split(path.sep) : []) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current);
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${current}`);
  }
}

function inspectTarget(route) {
  safeRoute(route);
  let current = previewRoot;
  const parts = route.split('/');
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink rejected: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent is not a directory: ${route}`);
      else {
        assert.ok(stat.isFile(), `target is not a regular file: ${route}`);
        const bytes = fs.readFileSync(current);
        return { exists: true, bytes, sha256: sha256(bytes), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function openPids(target) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof failed: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}

function syncFile(file) {
  const fd = fs.openSync(file, 'r');
  try { fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
}

function syncDirectory(directory) {
  const fd = fs.openSync(directory, 'r');
  try { fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
}

function addAtomically(target, bytes, mode) {
  const directory = path.dirname(target);
  const temporary = path.join(directory, `.${path.basename(target)}.tailnet-${randomUUID()}.tmp`);
  const fd = fs.openSync(temporary, 'wx', mode);
  try {
    fs.writeFileSync(fd, bytes);
    fs.fchmodSync(fd, mode);
    fs.fsyncSync(fd);
  } finally { fs.closeSync(fd); }
  try {
    fs.linkSync(temporary, target);
    fs.unlinkSync(temporary);
  } catch (error) {
    throw new Error(`exclusive atomic add failed for ${target}; temp retained at ${temporary}: ${error.message}`);
  }
  syncDirectory(directory);
}

function replaceAtomically(target, bytes, mode) {
  const directory = path.dirname(target);
  const temporary = path.join(directory, `.${path.basename(target)}.tailnet-${randomUUID()}.tmp`);
  const fd = fs.openSync(temporary, 'wx', mode);
  try {
    fs.writeFileSync(fd, bytes);
    fs.fchmodSync(fd, mode);
    fs.fsyncSync(fd);
  } finally { fs.closeSync(fd); }
  fs.renameSync(temporary, target);
  syncDirectory(directory);
}

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale must remain Running');
const rootStat = fs.lstatSync(previewRoot);
assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'preview root safety check failed');
fs.mkdirSync(path.dirname(backupRoot), { recursive: true });
ensureDirectory(path.dirname(backupRoot), path.dirname(path.dirname(backupRoot)));
fs.mkdirSync(backupRoot, { recursive: false });

const results = [];
for (const item of manifest.entries) {
  const sourceRoot = path.join(repo, manifest.buildBase, item.kind === 'rspress-generated' ? 'site' : 'root');
  const source = path.join(sourceRoot, item.route);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source safety check failed: ${item.route}`);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `source size changed after preflight: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `source hash changed after preflight: ${item.route}`);
  const target = path.join(previewRoot, item.route);
  const current = inspectTarget(item.route);
  if (item.state === 'same') {
    assert.ok(current.exists && current.sha256 === item.sha256, `same target changed after preflight: ${item.route}`);
    results.push({ route: item.route, state: 'unchanged-identical', sha256: item.sha256 });
    continue;
  }
  if (item.state === 'add') {
    assert.ok(!current.exists, `new target appeared after preflight: ${item.route}`);
    ensureDirectory(path.dirname(target), previewRoot);
    addAtomically(target, bytes, 0o644);
    assert.equal(inspectTarget(item.route).sha256, item.sha256, `added target hash mismatch: ${item.route}`);
    results.push({ route: item.route, state: 'added-exclusive-atomic-link', bytes: item.bytes, sha256: item.sha256 });
    continue;
  }
  assert.equal(item.state, 'replace', `unknown preflight state: ${item.state}`);
  assert.ok(manifest.replaceAllowlist.includes(item.route), `replacement not allowlisted: ${item.route}`);
  assert.ok(current.exists, `replacement target disappeared: ${item.route}`);
  assert.equal(current.sha256, item.targetSha256, `replacement target changed after preflight: ${item.route}`);
  const handles = openPids(target);
  const backup = path.join(backupRoot, item.route);
  ensureDirectory(path.dirname(backup), backupRoot);
  fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
  syncFile(backup);
  assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash mismatch: ${item.route}`);
  replaceAtomically(target, bytes, current.stat.mode & 0o777);
  assert.equal(inspectTarget(item.route).sha256, item.sha256, `replaced target hash mismatch: ${item.route}`);
  results.push({
    route: item.route,
    state: 'replaced-same-directory-atomic-rename',
    bytes: item.bytes,
    sha256: item.sha256,
    previousSha256: item.targetSha256,
    backup: path.relative(os.homedir(), backup).split(path.sep).join('/'),
    backupSha256: sha256(fs.readFileSync(backup)),
    openReaderPidsBeforeRename: handles,
  });
}

const report = {
  publicationManifest: 'selected-publication-manifest-pass03.json',
  fileCount: results.length,
  added: results.filter((item) => item.state.startsWith('added')).length,
  unchangedIdentical: results.filter((item) => item.state === 'unchanged-identical').length,
  replacedWithBackup: results.filter((item) => item.state.startsWith('replaced')).length,
  backupRoot,
  atomicReplacementKeepsExistingReaderFileDescriptorsValid: true,
  noDeletes: true,
  serverRestarted: false,
  entries: results,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, entries: undefined }, null, 2)}\n`);
