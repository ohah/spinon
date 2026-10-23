import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.argv[3] ?? 'pass01';
assert.ok(['pass01', 'pass02'].includes(pass), 'pass must be pass01 or pass02');
const sourceRoot = path.join(repositoryRoot, `build/spinon/c11-4-code-roadmap-sync-20261024-site-${pass}-selected`);
const applicationSupportRoot = path.join(os.homedir(), 'Library', 'Application Support');
const targetRoot = path.join(applicationSupportRoot, 'SpinonPreview');
const backupRoot = path.join(applicationSupportRoot, `SpinonPreviewBackups/c11-4-code-roadmap-sync-2026-10-24/${pass}`);
const origin = process.env.SPINON_PREVIEW_ORIGIN;
assert.ok(origin && origin.startsWith('https://') && origin.endsWith('/spinon/'), 'SPINON_PREVIEW_ORIGIN must be the HTTPS preview base ending in /spinon/');
const mode = process.argv[2];
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass01.mjs preflight|copy|verify [pass01|pass02]');

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const relativePath = (root, file) => path.relative(root, file).split(path.sep).join('/');
const reportPaths = {
  preflight: path.join(evidenceDirectory, `preflight-${pass}.json`),
  delta: path.join(evidenceDirectory, `delta-manifest-${pass}.json`),
  copy: path.join(evidenceDirectory, `copy-and-backup-${pass}.json`),
  verify: path.join(evidenceDirectory, `remote-verification-${pass}.json`),
};

const pass01Routes = [
  'STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'spec/plan/c11-4-grid-track-sizing.html',
  'internal/evidence/c11-4-code-roadmap-sync-2026-10-24/README.html',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/README.html',
  'internal/evidence/c11-4-code-roadmap-sync-2026-10-24/pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/pr-diff-review-20.html',
];
const pass02Routes = [
  'internal/evidence/c11-4-code-roadmap-sync-2026-10-24/README.html',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/README.html',
  'internal/evidence/c11-4-code-roadmap-sync-2026-10-24/pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/pr-diff-review-20.html',
  'static/image/tailnet-c114-plan-agent-browser-pass01.65153520e0.png',
];
const routes = pass === 'pass01' ? pass01Routes : pass02Routes;

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

function readReport(name) {
  return JSON.parse(fs.readFileSync(reportPaths[name], 'utf8'));
}

assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected build output is missing');
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), 'Tailnet preview root is not a regular directory');

if (mode === 'preflight') {
  for (const filePath of Object.values(reportPaths)) assert.ok(!fs.existsSync(filePath), 'refusing to overwrite report: ' + filePath);
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
    pass,
    sourceRoot: relativePath(repositoryRoot, sourceRoot),
    targetRootRelative: 'SpinonPreview',
    backupRootRelative: `SpinonPreviewBackups/c11-4-code-roadmap-sync-2026-10-24/${pass}`,
    previewServer,
    selectedRouteCount: routes.length,
    activeTcpConnectionCount,
    changedFiles: delta.length,
    addedFiles: delta.filter((item) => item.operation === 'add').length,
    replacedFiles: delta.filter((item) => item.operation === 'replace').length,
    backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0),
    selectedRoutes: routes,
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
  for (const item of delta) {
    const sourcePath = path.join(sourceRoot, item.path);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceStat.size, item.bytes, 'source size changed: ' + item.path);
    assert.equal(sha256(sourceBytes), item.sha256, 'source hash changed: ' + item.path);
    const destination = path.join(targetRoot, item.path);
    ensureDirectoryChain(targetRoot, path.dirname(item.path));
    const previousStat = assertRegularFile(destination, true);
    if (previousStat) {
      const previousBytes = fs.readFileSync(destination);
      assert.equal(item.operation, 'replace', 'new destination appeared: ' + item.path);
      assert.equal(previousBytes.length, item.previousBytes, 'destination size changed after preflight: ' + item.path);
      assert.equal(sha256(previousBytes), item.previousSha256, 'destination changed after preflight: ' + item.path);
      const backupPath = path.join(backupRoot, item.path);
      ensureDirectoryChain(backupRoot, path.dirname(item.path));
      fs.copyFileSync(destination, backupPath, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backupPath)), item.previousSha256, 'backup hash mismatch: ' + item.path);
      replacedFiles += 1;
    } else {
      assert.equal(item.operation, 'add', 'destination disappeared after preflight: ' + item.path);
      addedFiles += 1;
    }
    const temporaryPath = destination + '.spinon-' + pass + '-' + randomUUID() + '.tmp';
    try {
      fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
      fs.renameSync(temporaryPath, destination);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(destination)), item.sha256, 'copied route hash mismatch: ' + item.path);
  }
  const report = { pass, changedFiles: delta.length, replacedFiles, addedFiles, noDeletes: true, backupRootRelative: preflight.backupRootRelative };
  fs.writeFileSync(reportPaths.copy, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
}

if (mode === 'verify') {
  assert.ok(!fs.existsSync(reportPaths.verify), 'refusing to overwrite remote verification');
  const preflight = readReport('preflight');
  const results = [];
  for (const route of routes) {
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
    pass,
    selectedRouteCount: preflight.selectedRouteCount,
    changedRouteCount: preflight.changedFiles,
    verifiedRouteCount: results.length,
    allVerifiedRoutesHttp200: true,
    allVerifiedLocalAndRemoteHashesMatchSelectedBuild: true,
    noDeletes: true,
    routes: results,
  };
  fs.writeFileSync(reportPaths.verify, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify(report, null, 2) + '\n');
}
