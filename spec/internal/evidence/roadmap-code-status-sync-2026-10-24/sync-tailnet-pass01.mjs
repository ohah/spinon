import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass01-selected');
const applicationSupportRoot = path.join(os.homedir(), 'Library', 'Application Support');
const targetRoot = path.join(applicationSupportRoot, 'SpinonPreview');
const backupRoot = path.join(applicationSupportRoot, 'SpinonPreviewBackups/roadmap-code-status-sync-2026-10-24/pass01');
const origin = process.env.SPINON_PREVIEW_ORIGIN;
assert.ok(origin?.startsWith('https://'), 'set SPINON_PREVIEW_ORIGIN to the HTTPS preview base ending in /spinon/');
const mode = process.argv[2];
const planNames = [
  'c11-4-auto-fit-named-line-implicit-fallback',
  'c11-4-auto-repeat-named-line-implicit-fallback',
  'c11-4-auto-repeat-named-lines',
  'c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups',
  'c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups',
  'c11-4-cyclic-minmax-calc-cross-axis-span-groups',
  'c11-4-cyclic-minmax-calc-minimum-span',
  'c11-4-cyclic-minmax-calc-overlapping-rows',
  'c11-4-cyclic-minmax-calc-overlapping-spans',
  'c11-4-cyclic-minmax-calc-two-axis-spanning-text',
  'c11-4-cyclic-minmax-max-content-mixed-text-image-span',
  'c11-4-cyclic-minmax-mixed-text-image-span',
  'c11-4-cyclic-minmax-replaced-image-span',
  'c11-4-two-axis-template-areas',
];
const routeSet = new Set([
  'roadmap.html',
  'STATUS.html',
  ...planNames.flatMap((name) => [`plan/${name}.html`, `spec/plan/${name}.html`]),
]);
const routes = [...routeSet].sort();
const reportNames = {
  preflight: 'preflight-pass01.json',
  delta: 'delta-manifest-pass01.json',
  copy: 'copy-and-backup-pass01.json',
  verify: 'remote-verification-pass01.json',
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected RSPress output is missing');
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), 'Tailnet preview root is not a regular directory');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass01.mjs preflight|copy|verify');

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
  let current = root;
  for (const part of relativeDirectory ? relativeDirectory.split(path.sep) : []) {
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
      delta.push({ path: route, operation: 'replace', bytes: sourceStat.size, sha256: sha256(sourceBytes), previousBytes: targetBytes.length, previousSha256: sha256(targetBytes) });
    }
  }
  const report = {
    pass: 'pass01',
    sourceRoot: path.relative(repositoryRoot, sourceRoot),
    targetRootRelative: 'SpinonPreview',
    backupRootRelative: 'SpinonPreviewBackups/roadmap-code-status-sync-2026-10-24/pass01',
    selectedRoutes: routes,
    unchangedFiles: routes.length - delta.length,
    addedFiles: delta.filter((item) => item.operation === 'add').length,
    replacedFiles: delta.filter((item) => item.operation === 'replace').length,
    changedFiles: delta.length,
    backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0),
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
    assert.ok(!path.isAbsolute(relative) && !relative.split('/').includes('..'), `unsafe route: ${relative}`);
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
      assert.equal(sha256(previousBytes), item.previousSha256, `destination changed after preflight: ${relative}`);
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
      fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
      fs.renameSync(temporaryPath, destination);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), item.sha256, `copied route hash mismatch: ${relative}`);
  }
  const report = {
    pass: 'pass01',
    preflightChangedFiles: preflight.changedFiles,
    replacedFiles,
    addedFiles,
    alreadyAppliedFiles,
    backupRootRelative: 'SpinonPreviewBackups/roadmap-code-status-sync-2026-10-24/pass01',
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.copy), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'verify') {
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, reportNames.verify)), 'remote verification already exists');
  const preflight = readReport('preflight');
  const delta = readReport('delta');
  const results = [];
  for (const route of routes) {
    const expected = assertRegularFile(path.join(sourceRoot, route));
    const expectedBytes = fs.readFileSync(path.join(sourceRoot, route));
    const localBytes = fs.readFileSync(path.join(targetRoot, route));
    assert.equal(sha256(localBytes), sha256(expectedBytes), `local preview differs: ${route}`);
    const response = await fetch(new URL(route, origin), { cache: 'no-store' });
    assert.equal(response.status, 200, `HTTP ${response.status}: ${route}`);
    const remoteBytes = Buffer.from(await response.arrayBuffer());
    assert.equal(remoteBytes.length, expected.size, `remote size differs: ${route}`);
    assert.equal(sha256(remoteBytes), sha256(expectedBytes), `remote hash differs: ${route}`);
    results.push({ path: route, status: response.status, bytes: remoteBytes.length, sha256: sha256(remoteBytes) });
  }
  const report = {
    pass: 'pass01',
    selectedRouteCount: routes.length,
    changedRouteCount: delta.length,
    allHttp200: true,
    allServedHashesMatchSelectedBuild: true,
    allLocalPreviewHashesMatchSelectedBuild: true,
    noDeletes: preflight.noDeletes,
    routes: results,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.verify), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
