import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr979-2026-11-01/pass01');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));
const publication = path.join(repo, manifest.publicationRoot);
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function assertDir(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${path.basename(directory)}`);
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function inspect(root, route) {
  assertDir(root);
  safeRoute(route);
  let current = root;
  const parts = route.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink 대상 거부: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `route parent가 디렉터리가 아님: ${route}`);
      else {
        assert.ok(stat.isFile(), `route가 일반 파일이 아님: ${route}`);
        return { exists: true, bytes: fs.readFileSync(current), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function listFiles(directory, relative = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `publication symlink 거부: ${child}`);
    if (entry.isDirectory()) result.push(...listFiles(directory, child));
    else if (entry.isFile()) result.push(child.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication 항목: ${child}`);
  }
  return result;
}
function ensureDirectory(directory) {
  if (fs.existsSync(directory)) {
    assertDir(directory);
    return;
  }
  const parent = path.dirname(directory);
  if (parent !== directory) ensureDirectory(parent);
  fs.mkdirSync(directory, { recursive: false });
  assertDir(directory);
}
function openPids(filename) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', filename], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, 'lsof가 대상 파일을 검사하지 못함');
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}
async function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assertDir(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink 또는 alias 경로임');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, '8768 preview listener가 하나여야 함');
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '8768 listener가 예상된 preview server가 아님');
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1);
  assert.equal(sockets.filter((line) => line.includes('127.0.0.1:8768') && line.includes('(LISTEN)')).length, 1, 'preview listener 주소·port 불일치');
  const response = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
  const body = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, 'local preview roadmap가 HTTP 200이 아님');
  return { serverPid, localStatus: response.status, localBytes: body.length, localSha256: sha256(body) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDir(publication);
const sourcePaths = listFiles(publication);
const routeManifest = new Map(manifest.routes.map((item) => [item.path, item]));
assert.equal(sourcePaths.length, routeManifest.size, 'publication과 manifest route 수 불일치');
for (const route of sourcePaths) assert.ok(routeManifest.has(route), `manifest에 없는 publication route: ${route}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight 증거 보존');
  assert.ok(!fs.existsSync(backupRoot), '고유 backup 경로가 이미 존재해 덮어쓰지 않음');
  const server = await checkServer();
  const targets = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publication, route));
    const expected = routeManifest.get(route);
    assert.equal(source.length, expected.bytes, `publication size 변경: ${route}`);
    assert.equal(sha256(source), expected.sha256, `publication hash 변경: ${route}`);
    const target = inspect(previewRoot, route);
    const targetSha256 = target.exists ? sha256(target.bytes) : null;
    const action = !target.exists ? 'create' : targetSha256 === expected.sha256 ? 'same' : 'replace';
    const pids = action === 'replace' ? openPids(path.join(previewRoot, route)) : [];
    assert.equal(pids.length, 0, `바꿀 route가 현재 열려 있음: ${route}`);
    targets.push({ route, action, sourceBytes: source.length, sourceSha256: expected.sha256, targetExists: target.exists, targetBytes: target.exists ? target.bytes.length : null, targetSha256, openHandleCount: pids.length });
  }
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    tailscaleBackendRunning: true,
    previewServerPid: server.serverPid,
    localRoadmapStatus: server.localStatus,
    localRoadmapBytes: server.localBytes,
    localRoadmapSha256: server.localSha256,
    selectedRouteCount: targets.length,
    selectedBytes: targets.reduce((sum, item) => sum + item.sourceBytes, 0),
    counts,
    changedRouteOpenHandleCount: targets.reduce((sum, item) => sum + item.openHandleCount, 0),
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr979-2026-11-01/pass01',
    noDeletes: true,
    targets,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy 증거 보존');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, manifest.sourceHead);
  assert.equal(preflight.sourceTree, manifest.sourceTree);
  assert.equal(preflight.trackedDiffSha256, manifest.trackedDiffSha256);
  assert.equal(preflight.selectedRouteCount, sourcePaths.length);

  for (const target of preflight.targets) {
    const current = inspect(previewRoot, target.route);
    assert.equal(current.exists, target.targetExists, `preflight 이후 target 존재 상태 변경: ${target.route}`);
    if (current.exists) assert.equal(sha256(current.bytes), target.targetSha256, `preflight 이후 target 수정됨: ${target.route}`);
    assert.equal(sha256(fs.readFileSync(path.join(publication, target.route))), target.sourceSha256, `preflight 이후 source 수정됨: ${target.route}`);
  }

  const replacements = preflight.targets.filter((item) => item.action === 'replace');
  if (replacements.length) {
    assert.ok(!fs.existsSync(backupRoot), 'backup 경로를 덮어쓰지 않음');
    ensureDirectory(path.dirname(backupRoot));
    fs.mkdirSync(backupRoot, { recursive: false });
    assertDir(backupRoot);
    for (const item of replacements) {
      const backup = path.join(backupRoot, item.route);
      ensureDirectory(path.dirname(backup));
      fs.copyFileSync(path.join(previewRoot, item.route), backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${item.route}`);
    }
  }

  const created = [];
  const replaced = [];
  const identical = [];
  for (const item of preflight.targets) {
    const sourcePath = path.join(publication, item.route);
    const source = fs.readFileSync(sourcePath);
    if (item.action === 'same') {
      identical.push({ route: item.route, sha256: item.sourceSha256 });
      continue;
    }
    const destination = path.join(previewRoot, item.route);
    ensureDirectory(path.dirname(destination));
    const current = inspect(previewRoot, item.route);
    assert.equal(current.exists, item.targetExists, `preflight 이후 target 존재 상태 변경: ${item.route}`);
    if (item.action === 'replace') {
      assert.equal(sha256(current.bytes), item.targetSha256, `copy 직전 target 변경: ${item.route}`);
      assert.equal(openPids(destination).length, 0, `copy 직전 열린 target 거부: ${item.route}`);
    }
    const temporary = `${destination}.roadmap-sync-${randomUUID()}.tmp`;
    const modeBits = item.action === 'replace' ? current.stat.mode & 0o777 : 0o644;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: modeBits });
    fs.chmodSync(temporary, modeBits);
    fs.renameSync(temporary, destination);
    assert.equal(sha256(fs.readFileSync(destination)), item.sourceSha256, `복사 후 hash 불일치: ${item.route}`);
    if (item.action === 'replace') replaced.push({ route: item.route, previousSha256: item.targetSha256, sha256: item.sourceSha256 });
    else created.push({ route: item.route, sha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr979-2026-11-01/pass01',
    noDeletes: true,
    created,
    replaced,
    identical,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy가 필요함');
  assert.ok(!fs.existsSync(reports.verify), '기존 served-verification 증거 보존');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  const local = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publication, route));
    const target = inspect(previewRoot, route);
    assert.ok(target.exists, `local preview route 누락: ${route}`);
    assert.equal(target.bytes.length, source.length, `local preview 크기 불일치: ${route}`);
    assert.equal(sha256(target.bytes), sha256(source), `local preview hash 불일치: ${route}`);
    local.push({ route, bytes: source.length, sha256: sha256(source), verified: true });
  }
  const backupChecks = [];
  for (const item of copied.replaced) {
    const backup = inspect(backupRoot, item.route);
    assert.ok(backup.exists, `이전 파일 백업 누락: ${item.route}`);
    assert.equal(sha256(backup.bytes), item.previousSha256, `이전 파일 backup hash 불일치: ${item.route}`);
    backupChecks.push({ route: item.route, sha256: item.previousSha256, verified: true });
  }

  const remote = new Array(sourcePaths.length);
  let next = 0;
  async function worker() {
    while (true) {
      const index = next++;
      if (index >= sourcePaths.length) return;
      const route = sourcePaths[index];
      const response = await fetch(new URL(route.split('/').map(encodeURIComponent).join('/'), remoteBase), { signal: AbortSignal.timeout(20000) });
      assert.equal(response.status, 200, `HTTPS preview route HTTP ${response.status}: ${route}`);
      const bytes = Buffer.from(await response.arrayBuffer());
      const expected = routeManifest.get(route);
      assert.equal(bytes.length, expected.bytes, `HTTPS preview 크기 불일치: ${route}`);
      assert.equal(sha256(bytes), expected.sha256, `HTTPS preview hash 불일치: ${route}`);
      remote[index] = { route, bytes: bytes.length, sha256: expected.sha256, httpStatus: response.status, verified: true };
    }
  }
  await Promise.all(Array.from({ length: 6 }, () => worker()));
  const server = await checkServer();
  assert.equal(server.serverPid, preflight.previewServerPid, '동기화 중 preview server PID가 바뀜');
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length,
    localHashVerified: local.length,
    httpsHttp200AndHashVerified: remote.length,
    backupsVerified: backupChecks.length,
    tailscaleBackendRunning: true,
    observedPreviewServerPid: server.serverPid,
    postSyncServerObservation: {
      observedAtUtc: new Date().toISOString(),
      tailscaleBackendRunning: true,
      preflightPreviewServerPid: preflight.previewServerPid,
      postSyncPreviewServerPid: server.serverPid,
      sameProcessObserved: true,
    },
    previewServerRestarted: false,
    noDeletes: true,
    local,
    https: remote,
    backupChecks,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, local: undefined, https: undefined, backupChecks: undefined }, null, 2)}\n`);
}
