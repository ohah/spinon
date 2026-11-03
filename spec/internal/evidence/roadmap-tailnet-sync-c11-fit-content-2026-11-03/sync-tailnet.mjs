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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-fit-content-2026-11-03/pass01');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));
const publication = path.join(repo, manifest.publicationRoot);
const preflightPath = path.join(evidence, 'preflight.json');
const copyPath = path.join(evidence, 'copy-and-backup.json');
const verifyPath = path.join(evidence, 'served-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function assertDir(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 directory: ${directory}`);
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 route: ${route}`);
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
function openPids(filename) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', filename], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, 'lsof 대상 확인 실패');
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}
function safeEnsureDirectory(directory) {
  const absolute = path.resolve(directory);
  const parsed = path.parse(absolute);
  let current = parsed.root;
  for (const part of absolute.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, part);
    if (fs.existsSync(current)) {
      const stat = fs.lstatSync(current);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `directory symlink/non-directory 거부: ${current}`);
    } else fs.mkdirSync(current, { recursive: false });
  }
  assertDir(absolute);
}
function listFiles(directory, relative = '') {
  const results = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `publication symlink 거부: ${child}`);
    if (entry.isDirectory()) results.push(...listFiles(directory, child));
    else if (entry.isFile()) results.push(child.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication 항목: ${child}`);
  }
  return results;
}
function encodedRoute(route) {
  return route.split('/').map(encodeURIComponent).join('/');
}
async function fetchBytes(url) {
  const response = await fetch(url, { signal: AbortSignal.timeout(20000), redirect: 'follow' });
  const bytes = Buffer.from(await response.arrayBuffer());
  return { status: response.status, bytes, url: response.url };
}
async function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assertDir(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink/alias 경로임');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, '8768 listener가 하나여야 함');
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '8768 listener가 예상한 Tailnet preview server가 아님');
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1);
  assert.equal(sockets.filter((line) => line.includes('127.0.0.1:8768') && line.includes('(LISTEN)')).length, 1, 'preview server bind 주소가 예상과 다름');
  const local = await fetchBytes('http://127.0.0.1:8768/spinon/roadmap.html');
  assert.equal(local.status, 200, '기존 local roadmap가 HTTP 200이 아님');
  return { serverPid, localStatus: local.status, localBytes: local.bytes.length, localSha256: sha256(local.bytes) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDir(publication);
const sourcePaths = listFiles(publication);
const byRoute = new Map(manifest.routes.map((item) => [item.path, item]));
assert.equal(sourcePaths.length, byRoute.size, 'publication/manifest route 수 불일치');
for (const route of sourcePaths) assert.ok(byRoute.has(route), `manifest에 없는 route: ${route}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(preflightPath), '기존 preflight evidence 보존');
  assert.ok(!fs.existsSync(backupRoot), 'backup 경로가 이미 있어 덮어쓰지 않음');
  const server = await checkServer();
  const remote = await fetchBytes(`${remoteBase}roadmap.html`);
  assert.equal(remote.status, 200, '기존 HTTPS roadmap가 HTTP 200이 아님');
  const targets = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publication, route));
    const expected = byRoute.get(route);
    assert.equal(source.length, expected.bytes, `publication size 변경: ${route}`);
    assert.equal(sha256(source), expected.sha256, `publication hash 변경: ${route}`);
    const target = inspect(previewRoot, route);
    const targetSha256 = target.exists ? sha256(target.bytes) : null;
    const action = !target.exists ? 'create' : targetSha256 === expected.sha256 ? 'same' : 'replace';
    const pids = action === 'replace' ? openPids(path.join(previewRoot, route)) : [];
    assert.equal(pids.length, 0, `교체 route가 열려 있음 (${pids.join(',')}): ${route}`);
    targets.push({ route, action, sourceBytes: source.length, sourceSha256: expected.sha256, targetExists: target.exists, targetBytes: target.exists ? target.bytes.length : null, targetSha256, openHandleCount: pids.length });
  }
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const report = {
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree, trackedDiffSha256: manifest.trackedDiffSha256,
    tailscaleBackendRunning: true, previewServerPid: server.serverPid,
    localRoadmapBefore: { status: server.localStatus, bytes: server.localBytes, sha256: server.localSha256 },
    remoteRoadmapBefore: { status: remote.status, bytes: remote.bytes.length, sha256: sha256(remote.bytes) },
    selectedRouteCount: targets.length, selectedBytes: targets.reduce((sum, item) => sum + item.sourceBytes, 0), counts,
    changedRouteOpenHandleCount: targets.reduce((sum, item) => sum + item.openHandleCount, 0),
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-fit-content-2026-11-03/pass01',
    noDeletes: true, targets,
  };
  fs.writeFileSync(preflightPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(preflightPath), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(copyPath), '기존 copy evidence 보존');
  const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
  assert.equal(preflight.sourceHead, manifest.sourceHead);
  assert.equal(preflight.sourceTree, manifest.sourceTree);
  assert.equal(preflight.trackedDiffSha256, manifest.trackedDiffSha256);
  assert.equal(preflight.selectedRouteCount, sourcePaths.length);
  for (const item of preflight.targets) {
    const current = inspect(previewRoot, item.route);
    assert.equal(current.exists, item.targetExists, `preflight 뒤 target 존재 상태 변경: ${item.route}`);
    if (current.exists) assert.equal(sha256(current.bytes), item.targetSha256, `preflight 뒤 target hash 변경: ${item.route}`);
    assert.equal(sha256(fs.readFileSync(path.join(publication, item.route))), item.sourceSha256, `preflight 뒤 source 변경: ${item.route}`);
  }
  const replacements = preflight.targets.filter((item) => item.action === 'replace');
  if (replacements.length) {
    assert.ok(!fs.existsSync(backupRoot), 'backup 경로를 덮어쓰지 않음');
    safeEnsureDirectory(path.dirname(backupRoot));
    fs.mkdirSync(backupRoot, { recursive: false });
    assertDir(backupRoot);
    for (const item of replacements) {
      const backup = path.join(backupRoot, item.route);
      safeEnsureDirectory(path.dirname(backup));
      fs.copyFileSync(path.join(previewRoot, item.route), backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${item.route}`);
    }
  }
  const created = []; const replaced = []; const identical = [];
  for (const item of preflight.targets) {
    const sourcePath = path.join(publication, item.route);
    if (item.action === 'same') { identical.push({ route: item.route, sha256: item.sourceSha256 }); continue; }
    const source = fs.readFileSync(sourcePath);
    const destination = path.join(previewRoot, item.route);
    safeEnsureDirectory(path.dirname(destination));
    const current = inspect(previewRoot, item.route);
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
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree, trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length, createdCount: created.length, replacedCount: replaced.length, identicalCount: identical.length,
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-fit-content-2026-11-03/pass01',
    noDeletes: true, created, replaced, identical,
  };
  fs.writeFileSync(copyPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(copyPath), 'verify 전에 copy가 필요함');
  assert.ok(!fs.existsSync(verifyPath), '기존 served verification 보존');
  const copied = JSON.parse(fs.readFileSync(copyPath, 'utf8'));
  const localResults = [];
  const remoteResults = [];
  const criticalRoutes = new Set([
    'roadmap.html', 'STATUS.html', 'internal/0059-c11-grid-layout.html',
    'plan/c11-root-html-grid-fit-content.html',
    'internal/evidence/c11-root-html-grid-fit-content-runtime-2026-11-03/README.html',
    'internal/evidence/c11-root-html-grid-fit-content-runtime-2026-11-03/pr-diff-review-30.html',
    'internal/evidence/c11-root-html-grid-fit-content-runtime-2026-11-03/roadmap-doc-pr-review-30.html',
  ]);
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publication, route));
    assert.equal(sha256(source), byRoute.get(route).sha256, `publication hash 변경: ${route}`);
    const disk = inspect(previewRoot, route);
    assert.ok(disk.exists, `copied route 누락: ${route}`);
    assert.equal(sha256(disk.bytes), byRoute.get(route).sha256, `local disk hash 불일치: ${route}`);
    if (criticalRoutes.has(route)) {
      const local = await fetchBytes(`http://127.0.0.1:8768/spinon/${encodedRoute(route)}`);
      assert.equal(local.status, 200, `local HTTP status 오류: ${route}`);
      assert.equal(sha256(local.bytes), byRoute.get(route).sha256, `local served hash 불일치: ${route}`);
      localResults.push({ route, status: local.status, bytes: local.bytes.length, sha256: sha256(local.bytes) });
    }
  }
  const queue = [...sourcePaths];
  const workers = Array.from({ length: 8 }, async () => {
    while (queue.length) {
      const route = queue.shift();
      const result = await fetchBytes(`${remoteBase}${encodedRoute(route)}`);
      assert.equal(result.status, 200, `HTTPS status 오류: ${route}`);
      assert.equal(sha256(result.bytes), byRoute.get(route).sha256, `HTTPS served hash 불일치: ${route}`);
      if (criticalRoutes.has(route)) remoteResults.push({ route, status: result.status, bytes: result.bytes.length, sha256: sha256(result.bytes) });
    }
  });
  await Promise.all(workers);
  assert.equal(remoteResults.length, criticalRoutes.size, 'critical HTTPS route verification 누락');
  const report = {
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree, publicationRouteCount: sourcePaths.length,
    localCriticalRoutes: localResults.length, httpsAllRoutes: sourcePaths.length, criticalHttpsRoutes: remoteResults.length,
    officialCount: '72/213', c11Complete: false, noDeletes: true,
    copied: { createdCount: copied.createdCount, replacedCount: copied.replacedCount, identicalCount: copied.identicalCount },
    localResults, remoteResults,
  };
  fs.writeFileSync(verifyPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
