import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-site-pass01-selected');
const applicationSupportRoot = '/Users/yoonhb/Library/Application Support';
const targetRoot = path.join(applicationSupportRoot, 'SpinonPreview');
const backupRoot = path.join(applicationSupportRoot, 'SpinonPreviewBackups/c11-4-roadmap-code-sync-2026-10-23/pass02');
const origin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const mode = process.argv[2];
const routes = [
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
];
const names = {
  preflight: 'preflight-pass02.json',
  delta: 'delta-manifest-pass02.json',
  copy: 'copy-and-backup-pass02.json',
  verify: 'remote-verification-pass02.json',
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
if (!['preflight', 'copy', 'verify'].includes(mode)) throw new Error('usage: node sync-evidence-pages-pass02.mjs preflight|copy|verify');

function regularFile(filePath, optional = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
    return stat;
  } catch (error) {
    if (optional && error.code === 'ENOENT') return null;
    throw error;
  }
}

function ensureDirectoryChain(root, relative) {
  let current = root;
  for (const part of relative ? relative.split(path.sep) : []) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${current}`);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      fs.mkdirSync(current);
    }
  }
}

function read(name) {
  return JSON.parse(fs.readFileSync(path.join(evidenceDirectory, names[name]), 'utf8'));
}

assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected RSPress output is missing');
const previewStat = fs.lstatSync(targetRoot);
assert.ok(previewStat.isDirectory() && !previewStat.isSymbolicLink(), 'Tailnet preview root is unsafe');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, names.preflight)), 'pass02 preflight exists');
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, names.delta)), 'pass02 delta exists');
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse pass02 backup root');
  const delta = [];
  for (const relative of routes) {
    assert.ok(!path.isAbsolute(relative) && !relative.split('/').includes('..'), `unsafe route: ${relative}`);
    const sourcePath = path.join(sourceRoot, relative);
    const sourceStat = regularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    const targetPath = path.join(targetRoot, relative);
    const previous = regularFile(targetPath, true);
    if (!previous) {
      delta.push({ path: relative, operation: 'add', bytes: sourceStat.size, sha256: sha256(sourceBytes) });
      continue;
    }
    const previousBytes = fs.readFileSync(targetPath);
    if (previousBytes.length !== sourceBytes.length || sha256(previousBytes) !== sha256(sourceBytes)) {
      delta.push({
        path: relative, operation: 'replace', bytes: sourceBytes.length, sha256: sha256(sourceBytes),
        previousBytes: previousBytes.length, previousSha256: sha256(previousBytes),
      });
    }
  }
  const report = {
    pass: 'pass02', selectedRoutes: routes, unchangedFiles: routes.length - delta.length,
    addedFiles: delta.filter(item => item.operation === 'add').length,
    replacedFiles: delta.filter(item => item.operation === 'replace').length,
    changedFiles: delta.length, backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0), noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, names.preflight), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  fs.writeFileSync(path.join(evidenceDirectory, names.delta), `${JSON.stringify(delta, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = read('preflight');
  const delta = read('delta');
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse pass02 backup root');
  const backupRelative = path.relative(applicationSupportRoot, backupRoot);
  assert.ok(!path.isAbsolute(backupRelative) && !backupRelative.split(path.sep).includes('..'), 'backup path escapes Application Support');
  if (delta.length) ensureDirectoryChain(applicationSupportRoot, backupRelative);
  let added = 0;
  let replaced = 0;
  let alreadyApplied = 0;
  for (const item of delta) {
    const sourcePath = path.join(sourceRoot, item.path);
    const sourceStat = regularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceStat.size, item.bytes, `source size changed: ${item.path}`);
    assert.equal(sha256(sourceBytes), item.sha256, `source hash changed: ${item.path}`);
    const targetPath = path.join(targetRoot, item.path);
    ensureDirectoryChain(targetRoot, path.dirname(item.path));
    const previous = regularFile(targetPath, true);
    if (previous) {
      const previousBytes = fs.readFileSync(targetPath);
      if (previousBytes.length === item.bytes && sha256(previousBytes) === item.sha256) {
        alreadyApplied += 1;
        continue;
      }
      assert.equal(item.operation, 'replace', `new target appeared unexpectedly: ${item.path}`);
      assert.equal(previousBytes.length, item.previousBytes, `target changed after preflight: ${item.path}`);
      assert.equal(sha256(previousBytes), item.previousSha256, `target hash changed after preflight: ${item.path}`);
      const backupPath = path.join(backupRoot, item.path);
      ensureDirectoryChain(backupRoot, path.dirname(item.path));
      fs.copyFileSync(targetPath, backupPath, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backupPath)), item.previousSha256, `backup hash mismatch: ${item.path}`);
      replaced += 1;
    } else {
      assert.equal(item.operation, 'add', `target disappeared after preflight: ${item.path}`);
      added += 1;
    }
    const temporaryPath = `${targetPath}.spinon-pass02-${randomUUID()}.tmp`;
    try {
      fs.copyFileSync(sourcePath, temporaryPath, fs.constants.COPYFILE_EXCL);
      fs.chmodSync(temporaryPath, sourceStat.mode & 0o777);
      fs.renameSync(temporaryPath, targetPath);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(targetPath)), item.sha256, `target hash mismatch: ${item.path}`);
  }
  const report = {
    pass: 'pass02', added, replaced, alreadyApplied,
    backupRootRelative: path.relative(applicationSupportRoot, backupRoot),
    replacementMethod: 'unique sibling temp file followed by atomic rename', noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, names.copy), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'verify') {
  const delta = read('delta');
  const fetched = [];
  for (const relative of routes) {
    const expected = fs.readFileSync(path.join(sourceRoot, relative));
    const local = fs.readFileSync(path.join(targetRoot, relative));
    assert.equal(sha256(local), sha256(expected), `local preview mismatch: ${relative}`);
    const response = await fetch(new URL(relative, origin));
    const remote = Buffer.from(await response.arrayBuffer());
    assert.equal(response.status, 200, `HTTP ${response.status}: ${relative}`);
    assert.equal(sha256(remote), sha256(expected), `remote preview mismatch: ${relative}`);
    fetched.push({ path: relative, status: response.status, bytes: remote.length, sha256: sha256(remote) });
  }
  const report = { pass: 'pass02', changedFiles: delta.length, routes: fetched, allHttp200AndHashesMatch: true, noDeletes: true };
  fs.writeFileSync(path.join(evidenceDirectory, names.verify), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
