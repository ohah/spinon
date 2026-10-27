import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceRoot = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceRoot, '../../../..');
const appSupport = path.join(os.homedir(), 'Library', 'Application Support');
const previewRoot = path.join(appSupport, 'SpinonPreview');
const target = path.join(previewRoot, 'roadmap.html');
const backupParent = path.join(appSupport, 'SpinonPreviewBackups');
const backupTaskRoot = path.join(backupParent, 'c11-4-pr688-postmerge-tailnet-sync-2026-10-27');
const backupRoot = path.join(backupTaskRoot, 'pass01');
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const serverPath = path.join(previewRoot, 'tailnet-preview-server.mjs');
const remoteUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const localUrl = 'http://127.0.0.1:8768/spinon/roadmap.html';
const expectedPreviousSha256 = '74f084cd679efbd0f480c5a72806d7f8fdfbd3f886577aa6e0d19d3ab4e93728';
const expectedSourceSha256 = '2a3c75b4e77318f3828beb9be72131217f80f792f96a9eaed116fb6b06f3f425';
const preflightPath = path.join(evidenceRoot, 'preflight.json');
const resultPath = path.join(evidenceRoot, 'sync-result.json');

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runText = (command, args) => execFileSync(command, args, { encoding: 'utf8' });
const readHttp = async (url) => {
  const response = await fetch(url, { cache: 'no-store' });
  return { status: response.status, headers: Object.fromEntries(response.headers), bytes: Buffer.from(await response.arrayBuffer()) };
};
const safeDirectory = (directory) => {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
  assert.equal(fs.realpathSync(directory), directory, `directory resolves outside expected path: ${directory}`);
};

for (const reportPath of [preflightPath, resultPath]) {
  assert.ok(!fs.existsSync(reportPath), `refusing to overwrite ${reportPath}`);
}
const sourceBytes = fs.readFileSync(sourcePath);
assert.equal(sha256(sourceBytes), expectedSourceSha256, 'unexpected source roadmap hash');
const sourceText = sourceBytes.toString('utf8');
assert.ok(sourceText.includes('pull/688') && sourceText.includes('0b434e16ac66d0a742057e6e1e5749151bd3e08f'), 'source is missing merged PR #688 card');
assert.ok(sourceText.includes('72/213'), 'source is missing the official completion count');

const status = JSON.parse(runText('tailscale', ['status', '--json']));
assert.equal(status.BackendState, 'Running', 'Tailscale backend is not running');
safeDirectory(previewRoot);
safeDirectory(backupParent);
assert.ok(!fs.existsSync(backupTaskRoot), `backup task path already exists: ${backupTaskRoot}`);
const targetStat = fs.lstatSync(target);
assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), 'roadmap target must be a regular file');
const previousBytes = fs.readFileSync(target);
assert.equal(sha256(previousBytes), expectedPreviousSha256, 'existing roadmap target changed since preflight');

const serverSource = fs.readFileSync(serverPath, 'utf8');
assert.ok(serverSource.includes('fs.stat(file,') && serverSource.includes('fs.createReadStream(file)'), 'preview server does not resolve and stream each request by pathname');
const listenerRows = runText('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN', '-Fpcn']).split('\n').filter(Boolean);
const previewPid = Number(listenerRows.find((line) => line.startsWith('p'))?.slice(1));
assert.ok(Number.isInteger(previewPid) && previewPid > 0, 'preview server is not listening on port 8768');
const socketRows = runText('lsof', ['-nP', '-a', '-p', String(previewPid), '-iTCP']).split('\n').filter(Boolean).slice(1);
assert.ok(socketRows.length > 0 && socketRows.every((line) => line.includes('(LISTEN)') && !line.includes('->')), 'preview server has an active TCP request; refusing replacement');

const localBefore = await readHttp(localUrl);
assert.equal(localBefore.status, 200, 'local preview route is not available before sync');
assert.equal(sha256(localBefore.bytes), expectedPreviousSha256, 'local route body does not match old roadmap');
const remoteBefore = await readHttp(remoteUrl);
assert.equal(remoteBefore.status, 200, 'Tailnet route is not available before sync');
assert.equal(sha256(remoteBefore.bytes), expectedPreviousSha256, 'Tailnet route does not match old roadmap before sync');

const targetHandles = spawnSync('lsof', ['-nP', '--', target], { encoding: 'utf8' });
assert.ok(targetHandles.status === 0 || targetHandles.status === 1, `lsof failed: ${targetHandles.stderr}`);
const handleRows = targetHandles.stdout.split('\n').filter(Boolean).slice(1);
const parsedHandles = handleRows.map((line) => {
  const columns = line.trim().split(/\s+/);
  return { command: columns[0], pid: Number(columns[1]), descriptor: columns[3] };
});
assert.ok(parsedHandles.every((handle) => handle.pid === previewPid && /r$/.test(handle.descriptor)), 'a non-preview or write handle uses the roadmap target');

const preflight = {
  sourceBaseHead: runText('git', ['rev-parse', 'HEAD']).trim(),
  sourceWorkingTreePaths: runText('git', ['diff', '--name-only', '--', 'spec/public/roadmap.html']).trim().split('\n').filter(Boolean),
  sourcePath: 'spec/public/roadmap.html',
  sourceBytes: sourceBytes.length,
  sourceSha256: expectedSourceSha256,
  tailscaleBackendState: status.BackendState,
  tailnetHostName: status.Self?.HostName ?? null,
  previewRoot,
  previewPid,
  previewServerRestarted: false,
  targetPath: target,
  targetWasRegularFile: true,
  previousBytes: previousBytes.length,
  previousSha256: expectedPreviousSha256,
  targetReadHandles: parsedHandles,
  activeClientConnections: 0,
  localRouteBefore: { status: localBefore.status, bytes: localBefore.bytes.length, sha256: sha256(localBefore.bytes) },
  tailnetRouteBefore: { status: remoteBefore.status, bytes: remoteBefore.bytes.length, sha256: sha256(remoteBefore.bytes) },
  backupRoot,
  backupRootWasAbsent: true,
  selectedRouteCount: 1,
};
fs.writeFileSync(preflightPath, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });

fs.mkdirSync(backupTaskRoot, { recursive: false });
fs.mkdirSync(backupRoot, { recursive: false });
const backupPath = path.join(backupRoot, 'roadmap.html');
fs.copyFileSync(target, backupPath, fs.constants.COPYFILE_EXCL);
const backupBytes = fs.readFileSync(backupPath);
assert.deepEqual(backupBytes, previousBytes, 'backup is not byte-exact');

const temporaryPath = path.join(previewRoot, `.roadmap.html.codex-sync-${randomUUID()}.tmp`);
const temporaryFd = fs.openSync(temporaryPath, 'wx', targetStat.mode & 0o777);
try {
  fs.writeFileSync(temporaryFd, sourceBytes);
  fs.fsyncSync(temporaryFd);
} finally {
  fs.closeSync(temporaryFd);
}
fs.chmodSync(temporaryPath, targetStat.mode & 0o777);
fs.renameSync(temporaryPath, target);
const installedBytes = fs.readFileSync(target);
assert.deepEqual(installedBytes, sourceBytes, 'installed target differs from source');

const localAfter = await readHttp(localUrl);
const remoteAfter = await readHttp(remoteUrl);
for (const [name, response] of [['local', localAfter], ['Tailnet', remoteAfter]]) {
  assert.equal(response.status, 200, `${name} route did not return HTTP 200`);
  assert.equal(response.bytes.length, sourceBytes.length, `${name} route byte count mismatch`);
  assert.equal(sha256(response.bytes), expectedSourceSha256, `${name} route hash mismatch`);
}
const result = {
  sourceBaseHead: preflight.sourceBaseHead,
  sourceWorkingTreePaths: preflight.sourceWorkingTreePaths,
  sourcePath: preflight.sourcePath,
  sourceBytes: sourceBytes.length,
  sourceSha256: expectedSourceSha256,
  targetPath: target,
  backupPath,
  backupBytes: backupBytes.length,
  backupSha256: sha256(backupBytes),
  replacement: 'same-directory atomic rename after byte-exact backup',
  replacedRoutes: ['roadmap.html'],
  otherRoutesChanged: false,
  filesDeleted: false,
  previewServerRestarted: false,
  activeClientConnectionsDuringReplacement: 0,
  localRouteAfter: { status: localAfter.status, bytes: localAfter.bytes.length, sha256: sha256(localAfter.bytes) },
  tailnetRouteAfter: { status: remoteAfter.status, bytes: remoteAfter.bytes.length, sha256: sha256(remoteAfter.bytes) },
  allRoutesHttp200: true,
  allRouteHashesMatch: true,
};
fs.writeFileSync(resultPath, `${JSON.stringify(result, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ preflight, result }, null, 2)}\n`);
