import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceRoot = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceRoot, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library', 'Application Support', 'SpinonPreview');
const target = path.join(previewRoot, 'roadmap.html');
const backupParent = path.join(os.homedir(), 'Library', 'Application Support', 'SpinonPreviewBackups');
const backupTaskRoot = path.join(backupParent, 'roadmap-tailnet-sync-pr738-2026-10-28');
const backupRoot = path.join(backupTaskRoot, 'pass01');
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const serverPath = path.join(previewRoot, 'tailnet-preview-server.mjs');
const localUrl = 'http://127.0.0.1:8768/spinon/roadmap.html';
const remoteUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const expectedSourceHead = '23304539ac9d6fab0e79e2bb8cc1154e77cc8469';
const expectedPriorSourceHead = 'bf4ab21eebc2f3739a7ae61de6511b92733b9464';
const expectedPreviousSha256 = 'ecb1845d51e5eff1407f863ff591317df43127479d2b6efd71518198dab86709';
const preflightPath = path.join(evidenceRoot, 'preflight.json');
const resultPath = path.join(evidenceRoot, 'sync-result.json');

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runText = (command, args) => execFileSync(command, args, { encoding: 'utf8' });
const readHttp = async (url) => {
  const response = await fetch(url, { cache: 'no-store' });
  return { status: response.status, bytes: Buffer.from(await response.arrayBuffer()) };
};
const safeDirectory = (directory) => {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
  assert.equal(fs.realpathSync(directory), directory, `directory resolves outside expected path: ${directory}`);
};

for (const reportPath of [preflightPath, resultPath]) {
  assert.ok(!fs.existsSync(reportPath), `refusing to overwrite ${reportPath}`);
}
assert.equal(runText('git', ['rev-parse', 'HEAD']).trim(), expectedSourceHead, 'unexpected local source commit');
assert.equal(runText('git', ['rev-parse', 'origin/main']).trim(), expectedSourceHead, 'origin/main moved after documentation merge');

const sourceBytes = fs.readFileSync(sourcePath);
const mainSourceBytes = Buffer.from(runText('git', ['show', 'origin/main:spec/public/roadmap.html']));
assert.deepEqual(sourceBytes, mainSourceBytes, 'worktree roadmap differs from merged origin/main');
const sourceHash = sha256(sourceBytes);
const priorSourceBytes = Buffer.from(runText('git', ['show', `${expectedPriorSourceHead}:spec/public/roadmap.html`]));
assert.equal(sha256(priorSourceBytes), expectedPreviousSha256, 'the pre-sync main roadmap revision changed');

const tailscale = JSON.parse(runText('tailscale', ['status', '--json']));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend is not running');
assert.equal(tailscale.Self?.Online, true, 'Tailnet host is not online');
safeDirectory(previewRoot);
safeDirectory(backupParent);
assert.ok(!fs.existsSync(backupTaskRoot), `backup task path already exists: ${backupTaskRoot}`);

const targetStat = fs.lstatSync(target);
assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), 'roadmap target must be a regular file');
const previousBytes = fs.readFileSync(target);
assert.equal(sha256(previousBytes), expectedPreviousSha256, 'existing Tailnet roadmap changed since baseline');
assert.deepEqual(previousBytes, priorSourceBytes, 'existing Tailnet file differs from the prior merged roadmap');

const serverSource = fs.readFileSync(serverPath, 'utf8');
assert.ok(serverSource.includes('fs.stat(file,') && serverSource.includes('fs.createReadStream(file)'), 'preview server does not resolve each request by pathname');
const listenerRows = runText('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN', '-Fpcn']).split('\n').filter(Boolean);
const listenerPids = [...new Set(listenerRows.filter((line) => line.startsWith('p')).map((line) => Number(line.slice(1))))];
assert.equal(listenerPids.length, 1, 'expected one local Tailnet preview listener on port 8768');
const previewPid = listenerPids[0];
assert.ok(Number.isInteger(previewPid) && previewPid > 0, 'preview server PID is invalid');
const socketRows = runText('lsof', ['-nP', '-a', '-p', String(previewPid), '-iTCP']).split('\n').filter(Boolean).slice(1);
assert.ok(socketRows.length > 0 && socketRows.every((line) => line.includes('(LISTEN)') && !line.includes('->')), 'preview server has an active TCP request');

const targetHandles = spawnSync('lsof', ['-nP', '--', target], { encoding: 'utf8' });
assert.ok(targetHandles.status === 0 || targetHandles.status === 1, `lsof failed: ${targetHandles.stderr}`);
const handleRows = targetHandles.stdout.split('\n').filter(Boolean).slice(1);
const parsedHandles = handleRows.map((line) => {
  const columns = line.trim().split(/\s+/);
  return { command: columns[0], pid: Number(columns[1]), descriptor: columns[3] };
});
assert.ok(parsedHandles.every((handle) => handle.pid === previewPid && /r$/.test(handle.descriptor)), 'a non-preview or write handle uses the roadmap target');

const localBefore = await readHttp(localUrl);
assert.equal(localBefore.status, 200, 'local preview route is unavailable before sync');
assert.equal(sha256(localBefore.bytes), expectedPreviousSha256, 'local route differs from previous roadmap');
const remoteBefore = await readHttp(remoteUrl);
assert.equal(remoteBefore.status, 200, 'Tailnet route is unavailable before sync');
assert.equal(sha256(remoteBefore.bytes), expectedPreviousSha256, 'Tailnet route differs from previous roadmap');

const preflight = {
  sourceHead: expectedSourceHead,
  priorSourceHead: expectedPriorSourceHead,
  tailscaleBackendState: tailscale.BackendState,
  tailnetHostName: tailscale.Self?.HostName ?? null,
  previewRoot,
  previewPid,
  previewServerRestarted: false,
  targetPath: target,
  targetWasRegularFile: true,
  previousBytes: previousBytes.length,
  previousSha256: sha256(previousBytes),
  targetReadHandles: parsedHandles,
  activeClientConnections: 0,
  localRouteBefore: { status: localBefore.status, bytes: localBefore.bytes.length, sha256: sha256(localBefore.bytes) },
  tailnetRouteBefore: { status: remoteBefore.status, bytes: remoteBefore.bytes.length, sha256: sha256(remoteBefore.bytes) },
  backupTaskRoot,
  backupTaskRootWasAbsent: true,
  selectedRouteCount: 1,
  selectedBytes: sourceBytes.length,
  sourceSha256: sourceHash,
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
assert.deepEqual(installedBytes, sourceBytes, 'installed route differs from merged roadmap source');

const localAfter = await readHttp(localUrl);
const remoteAfter = await readHttp(remoteUrl);
for (const [name, response] of [['local', localAfter], ['Tailnet', remoteAfter]]) {
  assert.equal(response.status, 200, `${name} route did not return HTTP 200`);
  assert.equal(response.bytes.length, sourceBytes.length, `${name} route byte count mismatch`);
  assert.equal(sha256(response.bytes), sourceHash, `${name} route hash mismatch`);
}

const result = {
  sourceHead: expectedSourceHead,
  sourcePath: 'spec/public/roadmap.html',
  sourceBytes: sourceBytes.length,
  sourceSha256: sourceHash,
  targetPath: target,
  backupPath,
  backupBytes: backupBytes.length,
  backupSha256: sha256(backupBytes),
  replacement: 'same-directory atomic rename after byte-exact backup',
  replacedRoutes: ['roadmap.html'],
  otherRoutesChanged: false,
  filesDeleted: false,
  previewServerRestarted: false,
  previewPid,
  localRouteAfter: { status: localAfter.status, bytes: localAfter.bytes.length, sha256: sha256(localAfter.bytes) },
  tailnetRouteAfter: { status: remoteAfter.status, bytes: remoteAfter.bytes.length, sha256: sha256(remoteAfter.bytes) },
  allRoutesHttp200: true,
  allRouteHashesMatch: true,
};
fs.writeFileSync(resultPath, `${JSON.stringify(result, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ preflight, result }, null, 2)}\n`);
