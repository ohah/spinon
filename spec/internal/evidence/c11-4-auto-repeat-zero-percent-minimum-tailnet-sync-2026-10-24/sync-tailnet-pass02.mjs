import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-zero-percent-auto-repeat-minimum-site-pass02-selected');
const applicationSupportRoot = path.join(os.homedir(), 'Library', 'Application Support');
const targetRoot = path.join(applicationSupportRoot, 'SpinonPreview');
const backupRoot = path.join(applicationSupportRoot, 'SpinonPreviewBackups/c11-4-auto-repeat-zero-percent-minimum-tailnet-sync-2026-10-24/pass02');
const origin = process.env.SPINON_PREVIEW_ORIGIN;
assert.ok(origin && origin.startsWith('https://') && origin.endsWith('/spinon/'), 'SPINON_PREVIEW_ORIGIN must be the HTTPS preview base ending in /spinon/');
const mode = process.argv[2];
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass02.mjs preflight|copy|verify');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const relativePath = (root, file) => path.relative(root, file).split(path.sep).join('/');
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected build output is missing');
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), 'Tailnet preview root is not a regular directory');

const reportPaths = {
  preflight: path.join(evidenceDirectory, 'preflight-pass02.json'),
  delta: path.join(evidenceDirectory, 'delta-manifest-pass02.json'),
  copy: path.join(evidenceDirectory, 'copy-and-backup-pass02.json'),
  verify: path.join(evidenceDirectory, 'remote-verification-pass02.json'),
};
const reportNames = Object.fromEntries(Object.entries(reportPaths).map(([key, value]) => [key, path.basename(value)]));

function assertRegularFile(filePath, mayBeMissing = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe file: ' + filePath);
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
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), 'unsafe directory: ' + current);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      fs.mkdirSync(current);
    }
  }
}

function walkFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in selected build: ' + relative);
    if (entry.isDirectory()) files.push(...walkFiles(fullPath, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error('unsupported selected-build entry: ' + relative);
  }
  return files;
}

const fixedRoutes = [
  'roadmap.html',
  'STATUS.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum.html',
  'spec/plan/c11-4-auto-repeat-zero-percent-minimum.html',
  'internal/0059-c11-grid-layout.html',
  'spec/internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/pr-diff-review-30.html',
];
const publicEvidenceRoots = [
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24',
];
const routes = [...new Set([
  ...fixedRoutes,
  ...walkFiles(path.join(sourceRoot, 'static'), 'static'),
  ...publicEvidenceRoots.flatMap((relativeRoot) => walkFiles(path.join(sourceRoot, relativeRoot), relativeRoot)),
  ...['icon.svg', 'logo.svg', 'roadmap.svg'].filter((name) => fs.existsSync(path.join(sourceRoot, name))),
])].sort();
const verificationRoutes = [
  'roadmap.html',
  'STATUS.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/chrome156-zero-percent-auto-repeat-agent-browser-pass02.png',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/pr-diff-review-30.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/android-avd-c114-auto-repeat-zero-percent-minimum-android-avd-20261023T170458589Z-c0537217/result.json',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/android-avd-c114-auto-repeat-zero-percent-minimum-android-avd-20261023T170458589Z-c0537217/fit-col-first.png',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/ios-simulator-c114-auto-repeat-zero-percent-minimum-ios-simulator-20261023T170458589Z-b80ae17b/result.json',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/ios-simulator-c114-auto-repeat-zero-percent-minimum-ios-simulator-20261023T170458589Z-b80ae17b/fit-col-first.png',
];

function assertPreviewServer() {
  const listeners = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean);
  assert.equal(listeners.length, 1, 'expected exactly one Tailnet preview listener on port 8768');
  const command = execFileSync('ps', ['-p', listeners[0], '-o', 'command='], { encoding: 'utf8' }).trim();
  assert.ok(command.includes('tailnet-preview-server.mjs'), 'unexpected process owns Tailnet preview listener');
  return { pid: Number(listeners[0]), command: path.basename(command.split(/\s+/)[0]) };
}

function assertPreviewConnectionsIdle(previewServer) {
  try {
    const output = execFileSync('lsof', ['-nP', '-a', '-p', String(previewServer.pid), '-iTCP', '-sTCP:ESTABLISHED', '-Ff'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] });
    const activeConnections = output.split(/\r?\n/).filter((row) => row.startsWith('f')).length;
    assert.equal(activeConnections, 0, 'preview server has active TCP requests; retry after they close');
    return activeConnections;
  } catch (error) {
    if (error.status === 1 && !String(error.stdout ?? '').trim()) return 0;
    throw error;
  }
}

function readReport(key) {
  return JSON.parse(fs.readFileSync(reportPaths[key], 'utf8'));
}

if (mode === 'preflight') {
  for (const key of Object.keys(reportPaths)) assert.ok(!fs.existsSync(reportPaths[key]), 'report already exists: ' + reportNames[key]);
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse backup root');
  const previewServer = assertPreviewServer();
  const activeTcpConnectionCount = assertPreviewConnectionsIdle(previewServer);
  const delta = [];
  for (const route of routes) {
    assert.ok(!path.isAbsolute(route) && !route.split('/').includes('..'), 'unsafe route: ' + route);
    const sourcePath = path.join(sourceRoot, route);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    const targetPath = path.join(targetRoot, route);
    const previousStat = assertRegularFile(targetPath, true);
    if (!previousStat) {
      delta.push({ path: route, operation: 'add', bytes: sourceStat.size, sha256: sha256(sourceBytes) });
      continue;
    }
    const previousBytes = fs.readFileSync(targetPath);
    if (previousBytes.length !== sourceStat.size || sha256(previousBytes) !== sha256(sourceBytes)) {
      delta.push({ path: route, operation: 'replace', bytes: sourceStat.size, sha256: sha256(sourceBytes), previousBytes: previousBytes.length, previousSha256: sha256(previousBytes) });
    }
  }
  const report = {
    pass: 'pass02',
    sourceRoot: relativePath(repositoryRoot, sourceRoot),
    targetRootRelative: 'SpinonPreview',
    backupRootRelative: 'SpinonPreviewBackups/c11-4-auto-repeat-zero-percent-minimum-tailnet-sync-2026-10-24/pass02',
    previewServer,
    selectedRouteCount: routes.length,
    activeTcpConnectionCount,
    selectedRoutes: routes,
    unchangedFiles: routes.length - delta.length,
    addedFiles: delta.filter((item) => item.operation === 'add').length,
    replacedFiles: delta.filter((item) => item.operation === 'replace').length,
    changedFiles: delta.length,
    backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0),
    noDeletes: true,
  };
  fs.writeFileSync(reportPaths.preflight, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  fs.writeFileSync(reportPaths.delta, JSON.stringify(delta, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
}

if (mode === 'copy') {
  const preflight = readReport('preflight');
  const delta = readReport('delta');
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse backup root');
  const previewServer = assertPreviewServer();
  assert.equal(previewServer.pid, preflight.previewServer.pid, 'Tailnet preview listener changed after preflight');
  assert.equal(assertPreviewConnectionsIdle(previewServer), 0, 'preview server has active TCP requests; retry after they close');
  const backupRelative = path.relative(applicationSupportRoot, backupRoot);
  assert.ok(!path.isAbsolute(backupRelative) && !backupRelative.split(path.sep).includes('..'), 'backup path escaped Application Support');
  if (delta.length) ensureDirectoryChain(applicationSupportRoot, backupRelative);
  let addedFiles = 0;
  let replacedFiles = 0;
  let alreadyAppliedFiles = 0;
  for (const item of delta) {
    const relative = item.path;
    const sourcePath = path.join(sourceRoot, relative);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceStat.size, item.bytes, 'source size changed: ' + relative);
    assert.equal(sha256(sourceBytes), item.sha256, 'source hash changed: ' + relative);
    const destination = path.join(targetRoot, relative);
    ensureDirectoryChain(targetRoot, path.dirname(relative));
    const previousStat = assertRegularFile(destination, true);
    if (previousStat) {
      const previousBytes = fs.readFileSync(destination);
      if (previousBytes.length === item.bytes && sha256(previousBytes) === item.sha256) {
        alreadyAppliedFiles += 1;
        continue;
      }
      assert.equal(item.operation, 'replace', 'new destination appeared: ' + relative);
      assert.equal(previousBytes.length, item.previousBytes, 'destination size changed after preflight: ' + relative);
      assert.equal(sha256(previousBytes), item.previousSha256, 'destination changed after preflight: ' + relative);
      const backupPath = path.join(backupRoot, relative);
      ensureDirectoryChain(backupRoot, path.dirname(relative));
      fs.copyFileSync(destination, backupPath, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backupPath)), item.previousSha256, 'backup hash mismatch: ' + relative);
      replacedFiles += 1;
    } else {
      assert.equal(item.operation, 'add', 'destination disappeared after preflight: ' + relative);
      addedFiles += 1;
    }
    const temporaryPath = destination + '.spinon-pass02-' + randomUUID() + '.tmp';
    try {
      fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
      fs.renameSync(temporaryPath, destination);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), item.sha256, 'copied route hash mismatch: ' + relative);
  }
  const report = {
    pass: 'pass02',
    preflightChangedFiles: preflight.changedFiles,
    replacedFiles,
    addedFiles,
    alreadyAppliedFiles,
    backupRootRelative: preflight.backupRootRelative,
    noDeletes: true,
  };
  fs.writeFileSync(reportPaths.copy, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
}

if (mode === 'verify') {
  assert.ok(!fs.existsSync(reportPaths.verify), 'remote verification report already exists');
  const preflight = readReport('preflight');
  const results = [];
  for (const route of verificationRoutes) {
    const expectedPath = path.join(sourceRoot, route);
    const expectedStat = assertRegularFile(expectedPath);
    const expectedBytes = fs.readFileSync(expectedPath);
    const localBytes = fs.readFileSync(path.join(targetRoot, route));
    assert.equal(sha256(localBytes), sha256(expectedBytes), 'local preview hash differs: ' + route);
    const response = await fetch(new URL(route, origin), { cache: 'no-store' });
    assert.equal(response.status, 200, 'Tailnet HTTP status differs: ' + route);
    const remoteBytes = Buffer.from(await response.arrayBuffer());
    assert.equal(remoteBytes.length, expectedStat.size, 'remote size differs: ' + route);
    assert.equal(sha256(remoteBytes), sha256(expectedBytes), 'remote hash differs: ' + route);
    results.push({ path: route, status: response.status, bytes: remoteBytes.length, sha256: sha256(remoteBytes) });
  }
  const report = {
    pass: 'pass02',
    selectedRouteCount: preflight.selectedRouteCount,
    changedRouteCount: preflight.changedFiles,
    verifiedRouteCount: results.length,
    allVerifiedRoutesHttp200: true,
    allVerifiedLocalAndRemoteHashesMatchSelectedBuild: true,
    noDeletes: preflight.noDeletes,
    routes: results,
  };
  fs.writeFileSync(reportPaths.verify, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
}
