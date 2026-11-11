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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1351-2026-11-11/pass01');
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
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
function assertNoSymlinkPath(target) {
  const absolute = path.resolve(target);
  const parsed = path.parse(absolute);
  let current = parsed.root;
  for (const part of absolute.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `symlink 경로 거부: ${current}`);
  }
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `위험한 route: ${route}`);
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
      assert.ok(!stat.isSymbolicLink(), `symlink target 거부: ${route}`);
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
  assert.ok(parent !== directory, `디렉터리 경로가 잘못됨: ${directory}`);
  ensureDirectory(parent);
  fs.mkdirSync(directory, { recursive: false });
  assertDir(directory);
}
function openPids(filename) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', filename], { encoding: 'utf8', maxBuffer: 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}
function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assert.equal(tailscale.Self?.DNSName, 'macstudio.tailed42f2.ts.net.', '기대한 Tailnet host인지 확인');
  assertDir(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 alias 또는 symlink임');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 preview listener가 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview server: ${args}`);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  assert.ok(sockets.some((line) => line.includes('127.0.0.1:8768') && line.includes('(LISTEN)')), 'preview listener 주소·port 불일치');
  return { backend: tailscale.BackendState, host: tailscale.Self.DNSName, serverPid, args, sockets };
}
async function fetchHash(url) {
  const response = await fetch(url, { cache: 'no-store', signal: AbortSignal.timeout(20000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  return { url, status: response.status, bytes: bytes.length, sha256: sha256(bytes), body: bytes };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet-roadmap.mjs preflight|copy|verify');
assertDir(publication);
assertNoSymlinkPath(publication);
assertNoSymlinkPath(previewRoot);
const sourcePaths = listFiles(publication);
const routeManifest = new Map(manifest.routes.map((item) => [item.path, item]));
assert.equal(sourcePaths.length, routeManifest.size, 'publication과 manifest route 수 불일치');
for (const route of sourcePaths) assert.ok(routeManifest.has(route), `manifest에 없는 publication route: ${route}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight evidence 보존');
  assert.ok(!fs.existsSync(reports.copy), 'copy evidence가 먼저 없어야 함');
  assert.ok(!fs.existsSync(reports.verify), 'verify evidence가 먼저 없어야 함');
  assert.ok(!fs.existsSync(backupRoot), '기존 backup을 덮어쓰지 않음');
  const server = checkServer();
  const local = await fetchHash('http://127.0.0.1:8768/spinon/roadmap.html');
  assert.equal(local.status, 200, 'local preview roadmap가 HTTP 200이 아님');
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
    assert.equal(pids.length, 0, `교체할 route가 열려 있음: ${route}`);
    targets.push({ route, action, sourceBytes: source.length, sourceSha256: expected.sha256, targetExists: target.exists, targetBytes: target.exists ? target.bytes.length : null, targetSha256, openHandlePids: pids });
  }
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const report = {
    recordedAt: new Date().toISOString(),
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    publicationRouteCount: sourcePaths.length,
    publicationBytes: manifest.totalBytes,
    sourceRoadmapSha256: routeManifest.get('roadmap.html').sha256,
    tailscaleBackend: server.backend,
    tailnetHost: server.host,
    previewServerPid: server.serverPid,
    previewServerArgs: server.args,
    localRoadmapStatus: local.status,
    localRoadmapBytes: local.bytes,
    localRoadmapSha256: local.sha256,
    targetOpenHandleCount: targets.reduce((sum, item) => sum + item.openHandlePids.length, 0),
    backupRoot,
    counts,
    noDeletes: true,
    targets,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy evidence 보존');
  assert.ok(!fs.existsSync(reports.verify), 'verify evidence가 먼저 없어야 함');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, manifest.sourceHead);
  assert.equal(preflight.sourceTree, manifest.sourceTree);
  assert.equal(preflight.publicationRouteCount, sourcePaths.length);
  const server = checkServer();
  assert.equal(server.serverPid, preflight.previewServerPid, '기존 preview server가 바뀌었음');

  for (const target of preflight.targets) {
    const current = inspect(previewRoot, target.route);
    assert.equal(current.exists, target.targetExists, `preflight 이후 target 존재 상태 변경: ${target.route}`);
    if (current.exists) assert.equal(sha256(current.bytes), target.targetSha256, `preflight 이후 target 변경: ${target.route}`);
    assert.equal(sha256(fs.readFileSync(path.join(publication, target.route))), target.sourceSha256, `preflight 이후 source 변경: ${target.route}`);
    if (target.action === 'replace') assert.equal(openPids(path.join(previewRoot, target.route)).length, 0, `교체 직전 열린 target 거부: ${target.route}`);
  }

  const replacements = preflight.targets.filter((item) => item.action === 'replace');
  if (replacements.length) {
    assert.ok(!fs.existsSync(backupRoot), `backup 경로를 덮어쓰지 않음: ${backupRoot}`);
    ensureDirectory(path.dirname(backupRoot));
    fs.mkdirSync(backupRoot, { recursive: false });
    assertDir(backupRoot);
  }
  const backups = [];
  for (const item of replacements) {
    const backup = path.join(backupRoot, item.route);
    ensureDirectory(path.dirname(backup));
    fs.copyFileSync(path.join(previewRoot, item.route), backup, fs.constants.COPYFILE_EXCL);
    assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${item.route}`);
    backups.push({ route: item.route, sha256: item.targetSha256, verified: true });
  }

  const created = [];
  const replaced = [];
  const identical = [];
  for (const item of preflight.targets) {
    if (item.action === 'same') {
      identical.push({ route: item.route, sha256: item.sourceSha256 });
      continue;
    }
    const source = fs.readFileSync(path.join(publication, item.route));
    const destination = path.join(previewRoot, item.route);
    ensureDirectory(path.dirname(destination));
    const target = inspect(previewRoot, item.route);
    if (item.action === 'replace') assert.equal(sha256(target.bytes), item.targetSha256, `copy 직전 target 변경: ${item.route}`);
    const modeBits = item.action === 'replace' ? target.stat.mode & 0o777 : 0o644;
    const temporary = `${destination}.roadmap-sync-${randomUUID()}.tmp`;
    const fd = fs.openSync(temporary, 'wx', modeBits);
    try {
      fs.writeFileSync(fd, source);
      fs.fsyncSync(fd);
    } finally {
      fs.closeSync(fd);
    }
    fs.renameSync(temporary, destination);
    assert.equal(sha256(fs.readFileSync(destination)), item.sourceSha256, `복사 후 hash 불일치: ${item.route}`);
    const record = { route: item.route, sha256: item.sourceSha256 };
    if (item.action === 'replace') replaced.push({ ...record, previousSha256: item.targetSha256 });
    else created.push(record);
  }
  const afterServer = checkServer();
  assert.equal(afterServer.serverPid, preflight.previewServerPid, 'preview server PID가 복사 중 바뀌었음');
  const report = {
    recordedAt: new Date().toISOString(),
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    publicationRouteCount: sourcePaths.length,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    backupRoot: replacements.length ? backupRoot : null,
    backups,
    created,
    replaced,
    identical,
    noDeletes: true,
    serverPid: afterServer.serverPid,
    serverRestarted: false,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, backups: backups.length, created, replaced, identical: identical.length }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy가 필요함');
  assert.ok(!fs.existsSync(reports.verify), '기존 served evidence 보존');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const server = checkServer();
  assert.equal(server.serverPid, copied.serverPid, 'preview server PID가 변경됨');
  const local = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publication, route));
    const target = inspect(previewRoot, route);
    assert.ok(target.exists, `local preview route 누락: ${route}`);
    assert.equal(target.bytes.length, source.length, `local preview size 불일치: ${route}`);
    assert.equal(sha256(target.bytes), sha256(source), `local preview hash 불일치: ${route}`);
    local.push({ route, bytes: source.length, sha256: sha256(source), verified: true });
  }
  const remote = new Array(sourcePaths.length);
  let next = 0;
  async function worker() {
    while (true) {
      const index = next++;
      if (index >= sourcePaths.length) return;
      const route = sourcePaths[index];
      const url = new URL(route.split('/').map(encodeURIComponent).join('/'), remoteBase);
      const response = await fetchHash(url);
      const expected = routeManifest.get(route);
      assert.equal(response.status, 200, `HTTPS preview HTTP ${response.status}: ${route}`);
      assert.equal(response.bytes, expected.bytes, `HTTPS preview 크기 불일치: ${route}`);
      assert.equal(response.sha256, expected.sha256, `HTTPS preview hash 불일치: ${route}`);
      remote[index] = { route, url: response.url, bytes: response.bytes, sha256: response.sha256, httpStatus: response.status, verified: true };
    }
  }
  await Promise.all(Array.from({ length: 6 }, () => worker()));
  const backupChecks = [];
  for (const item of copied.backups) {
    const backup = inspect(backupRoot, item.route);
    assert.ok(backup.exists, `기존 파일 백업 누락: ${item.route}`);
    assert.equal(sha256(backup.bytes), item.sha256, `이전 파일 backup hash 불일치: ${item.route}`);
    backupChecks.push({ route: item.route, sha256: item.sha256, verified: true });
  }
  const finalServer = checkServer();
  assert.equal(finalServer.serverPid, copied.serverPid, '최종 확인 중 preview server PID가 변경됨');
  const report = {
    recordedAt: new Date().toISOString(),
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    selectedRouteCount: sourcePaths.length,
    localHashVerified: local.length,
    httpsHttp200AndHashVerified: remote.length,
    backupsVerified: backupChecks.length,
    previewServerPid: finalServer.serverPid,
    previewServerRestarted: false,
    deletions: 0,
    checks: { local, remote, backups: backupChecks },
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, checks: undefined }, null, 2)}\n`);
}
