import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-site-pass01-selected');
const applicationSupportRoot = '/Users/yoonhb/Library/Application Support';
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-roadmap-code-sync-2026-10-23/pass01';
const origin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const mode = process.argv[2];
const routes = [
  'roadmap.html', 'STATUS.html', 'internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html', 'plan/c11-4-auto-repeat-template-areas.html',
  'plan/c11-4-auto-repeat-calc-minimum.html', 'plan/c11-4-auto-repeat-positive-percent-minimum.html',
  'internal/evidence/c11-4-auto-repeat-calc-minimum-runtime-2026-10-23/README.html',
  'internal/evidence/c11-4-template-area-auto-repeat-runtime-2026-10-23/README.html',
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
];
const reportNames = {
  preflight: 'preflight-pass01.json',
  delta: 'delta-manifest-pass01.json',
  copy: 'copy-and-backup-pass01.json',
  verify: 'remote-verification-pass01.json',
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
if (!['preflight', 'copy', 'verify'].includes(mode)) throw new Error('usage: node sync-tailnet-pass01.mjs preflight|copy|verify');

function assertRegularFile(filePath, mayBeMissing = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
    return stat;
  } catch (error) {
    if (mayBeMissing && error.code === 'ENOENT') return null;
    throw error;
  }
}

function ensureDirectoryChain(root, relativeDirectory) {
  const parts = relativeDirectory ? relativeDirectory.split(path.sep) : [];
  let current = root;
  for (const part of parts) {
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

function readReport(key) {
  return JSON.parse(fs.readFileSync(path.join(evidenceDirectory, reportNames[key]), 'utf8'));
}

assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected RSPress output is missing');
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), 'Tailnet preview root is not a regular directory');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, reportNames.preflight)), 'preflight already exists');
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, reportNames.delta)), 'delta manifest already exists');
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const delta = [];
  for (const route of routes) {
    assert.ok(route && !path.isAbsolute(route) && !route.split('/').includes('..'), `unsafe route: ${route}`);
    const sourcePath = path.join(sourceRoot, route);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    const targetPath = path.join(targetRoot, route);
    const targetFile = assertRegularFile(targetPath, true);
    if (!targetFile) {
      delta.push({ path: route, operation: 'add', bytes: sourceStat.size, sha256: sha256(sourceBytes) });
      continue;
    }
    const targetBytes = fs.readFileSync(targetPath);
    if (sourceStat.size !== targetBytes.length || sha256(sourceBytes) !== sha256(targetBytes)) {
      delta.push({
        path: route, operation: 'replace', bytes: sourceStat.size, sha256: sha256(sourceBytes),
        previousBytes: targetBytes.length, previousSha256: sha256(targetBytes),
      });
    }
  }
  const report = {
    pass: 'pass01', sourceRoot: path.relative(repositoryRoot, sourceRoot),
    targetRootRelative: 'SpinonPreview', backupRootRelative: 'SpinonPreviewBackups/c11-4-roadmap-code-sync-2026-10-23/pass01',
    selectedRoutes: routes, unchangedFiles: routes.length - delta.length,
    addedFiles: delta.filter((item) => item.operation === 'add').length,
    replacedFiles: delta.filter((item) => item.operation === 'replace').length,
    changedFiles: delta.length, backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0),
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.preflight), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.delta), `${JSON.stringify(delta, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readReport('preflight');
  const delta = readReport('delta');
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const backupRelative = path.relative(applicationSupportRoot, backupRoot);
  assert.ok(!path.isAbsolute(backupRelative) && !backupRelative.split(path.sep).includes('..'), 'backup path escaped Application Support');
  let addedFiles = 0;
  let replacedFiles = 0;
  let alreadyAppliedFiles = 0;
  if (delta.length) ensureDirectoryChain(applicationSupportRoot, backupRelative);
  for (const item of delta) {
    const relative = item.path;
    assert.ok(!path.isAbsolute(relative) && !relative.split('/').includes('..'), `unsafe path: ${relative}`);
    const sourcePath = path.join(sourceRoot, relative);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceStat.size, item.bytes, `source size changed: ${relative}`);
    assert.equal(sha256(sourceBytes), item.sha256, `source hash changed: ${relative}`);
    const destination = path.join(targetRoot, relative);
    ensureDirectoryChain(targetRoot, path.dirname(relative));
    const previous = assertRegularFile(destination, true);
    if (previous) {
      const previousBytes = fs.readFileSync(destination);
      if (previousBytes.length === item.bytes && sha256(previousBytes) === item.sha256) {
        alreadyAppliedFiles += 1;
        continue;
      }
      assert.equal(item.operation, 'replace', `new destination appeared: ${relative}`);
      assert.equal(previousBytes.length, item.previousBytes, `destination changed after preflight: ${relative}`);
      assert.equal(sha256(previousBytes), item.previousSha256, `destination hash changed after preflight: ${relative}`);
      const backupPath = path.join(backupRoot, relative);
      ensureDirectoryChain(backupRoot, path.dirname(relative));
      fs.copyFileSync(destination, backupPath, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backupPath)), item.previousSha256, `backup hash mismatch: ${relative}`);
      replacedFiles += 1;
    } else {
      assert.equal(item.operation, 'add', `destination disappeared after preflight: ${relative}`);
      addedFiles += 1;
    }
    const temporaryPath = `${destination}.spinon-pass01-${randomUUID()}.tmp`;
    try {
      fs.copyFileSync(sourcePath, temporaryPath, fs.constants.COPYFILE_EXCL);
      fs.chmodSync(temporaryPath, sourceStat.mode & 0o777);
      fs.renameSync(temporaryPath, destination);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), item.sha256, `destination hash mismatch: ${relative}`);
  }
  const report = {
    pass: 'pass01', addedFiles, replacedFiles, alreadyAppliedFiles,
    backupRootRelative: preflight.backupRootRelative,
    backupMethod: 'copy only each replaced selected page to a unique backup path',
    replacementMethod: 'write a unique sibling temporary file then atomic rename', noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.copy), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'verify') {
  const preflight = readReport('preflight');
  const delta = readReport('delta');
  const fetched = [];
  for (const route of routes) {
    const sourcePath = path.join(sourceRoot, route);
    const expectedBytes = fs.readFileSync(sourcePath);
    const targetPath = path.join(targetRoot, route);
    const localBytes = fs.readFileSync(targetPath);
    assert.equal(sha256(localBytes), sha256(expectedBytes), `local preview hash mismatch: ${route}`);
    const response = await fetch(new URL(route, origin));
    const remoteBytes = Buffer.from(await response.arrayBuffer());
    const remoteHash = sha256(remoteBytes);
    assert.equal(response.status, 200, `Tailnet HTTP status ${response.status}: ${route}`);
    assert.equal(remoteHash, sha256(expectedBytes), `Tailnet served hash mismatch: ${route}`);
    fetched.push({ path: route, status: response.status, bytes: remoteBytes.length, sha256: remoteHash });
  }
  const report = {
    pass: 'pass01', selectedRoutes: routes, verifiedChangedFiles: delta.length,
    unchangedFiles: preflight.unchangedFiles, remoteRoutes: fetched,
    allHttp200AndHashesMatch: true, noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.verify), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
