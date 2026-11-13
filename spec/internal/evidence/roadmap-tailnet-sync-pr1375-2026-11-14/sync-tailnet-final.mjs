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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1375-final-2026-11-14');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-final.json'), 'utf8'));
const publication = path.join(repo, manifest.publicationRoot);
const reports = {
  preflight: path.join(evidence, 'preflight-final.json'),
  copy: path.join(evidence, 'copy-and-backup-final.json'),
  verify: path.join(evidence, 'served-verification-final.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function assertDir(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), '안전하지 않은 디렉터리: ' + path.basename(directory));
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), '잘못된 route: ' + route);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), '잘못된 route: ' + route);
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
      assert.ok(!stat.isSymbolicLink(), 'symlink 대상 거부: ' + route);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), 'route parent가 디렉터리가 아님: ' + route);
      else {
        assert.ok(stat.isFile(), 'route가 일반 파일이 아님: ' + route);
        return { exists: true, bytes: fs.readFileSync(current), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function listFiles(directory, relative) {
  const entries = fs.readdirSync(path.join(directory, relative), { withFileTypes: true });
  const result = [];
  for (const entry of entries.sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? relative + '/' + entry.name : entry.name;
    assert.ok(!entry.isSymbolicLink(), 'publication symlink 거부: ' + child);
    if (entry.isDirectory()) result.push(...listFiles(directory, child));
    else if (entry.isFile()) result.push(child.split(path.sep).join('/'));
    else throw new Error('지원하지 않는 publication 항목: ' + child);
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
  assert.ok(result.status === 0 || result.status === 1, 'lsof가 대상 파일을 검사하지 못함: ' + filename);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}
async function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  const serve = execFileSync('tailscale', ['serve', 'status'], { encoding: 'utf8' });
  assert.ok(serve.includes('8768'), 'Tailnet serve가 preview server port 8768을 가리키지 않음');
  assertDir(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink 또는 alias 경로임');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, '8768 preview listener가 하나여야 함');
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '8768 listener가 예상된 preview server가 아님');
  const local = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
  const localBytes = Buffer.from(await local.arrayBuffer());
  assert.equal(local.status, 200, 'local preview roadmap가 HTTP 200이 아님');
  return { serverPid, localStatus: local.status, localBytes: localBytes.length, localSha256: sha256(localBytes) };
}

assert.ok(['preflight', 'copy', 'verify', 'verify-final'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify|verify-final');
assertDir(publication);
const sourcePaths = listFiles(publication, '');
const routeManifest = new Map(manifest.routes.map((item) => [item.path, item]));
assert.equal(sourcePaths.length, routeManifest.size, 'publication과 manifest route 수 불일치');
for (const route of sourcePaths) assert.ok(routeManifest.has(route), 'manifest에 없는 publication route: ' + route);
for (const [route, item] of routeManifest) {
  const source = inspect(publication, route);
  assert.ok(source.exists, 'publication source 누락: ' + route);
  assert.equal(source.bytes.length, item.bytes, 'publication 크기 불일치: ' + route);
  assert.equal(sha256(source.bytes), item.sha256, 'publication hash 불일치: ' + route);
}

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight 증거 보존');
  const server = await checkServer();
  const targets = [];
  for (const route of sourcePaths) {
    const current = inspect(previewRoot, route);
    const destination = path.join(previewRoot, route);
    const sourceBytes = fs.readFileSync(path.join(publication, route));
    const sourceSha256 = sha256(sourceBytes);
    const action = !current.exists ? 'create' : sha256(current.bytes) === sourceSha256 ? 'same' : 'replace';
    const open = action === 'replace' ? openPids(destination) : [];
    assert.equal(open.length, 0, '열린 대상 파일은 교체하지 않음: ' + route + ' pids=' + open.join(','));
    targets.push({
      route,
      action,
      sourceSha256,
      bytes: sourceBytes.length,
      targetSha256: current.exists ? sha256(current.bytes) : null,
      openPids: open,
    });
  }
  const replaceCount = targets.filter((item) => item.action === 'replace').length;
  if (replaceCount) assert.ok(!fs.existsSync(backupRoot), '기존 backup 경로를 덮어쓰지 않음');
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length,
    replaceCount,
    createCount: targets.filter((item) => item.action === 'create').length,
    identicalCount: targets.filter((item) => item.action === 'same').length,
    localServer: server,
    previewRootVerified: true,
    targetSymlinks: 0,
    openReplacementTargets: 0,
    noDeletes: true,
    targets,
  };
  fs.writeFileSync(reports.preflight, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, targets: undefined }, null, 2) + '\n');
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight 필요');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy 증거 보존');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  const server = await checkServer();
  assert.equal(server.serverPid, preflight.localServer.serverPid, 'preflight 뒤 preview server가 바뀜');
  for (const item of preflight.targets) {
    const current = inspect(previewRoot, item.route);
    const currentSha256 = current.exists ? sha256(current.bytes) : null;
    assert.equal(currentSha256, item.targetSha256, 'preflight 이후 target 변경: ' + item.route);
    assert.equal(sha256(fs.readFileSync(path.join(publication, item.route))), item.sourceSha256, 'preflight 이후 source 변경: ' + item.route);
  }
  const replacements = preflight.targets.filter((item) => item.action === 'replace');
  if (replacements.length) {
    ensureDirectory(path.dirname(backupRoot));
    fs.mkdirSync(backupRoot, { recursive: false });
    assertDir(backupRoot);
    for (const item of replacements) {
      const backup = path.join(backupRoot, item.route);
      ensureDirectory(path.dirname(backup));
      fs.copyFileSync(path.join(previewRoot, item.route), backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, 'backup hash 불일치: ' + item.route);
    }
  }
  const created = [];
  const replaced = [];
  const identical = [];
  for (const item of preflight.targets) {
    if (item.action === 'same') {
      identical.push({ route: item.route, sha256: item.sourceSha256 });
      continue;
    }
    const destination = path.join(previewRoot, item.route);
    ensureDirectory(path.dirname(destination));
    const current = inspect(previewRoot, item.route);
    if (item.action === 'replace') {
      assert.equal(sha256(current.bytes), item.targetSha256, 'copy 직전 target 변경: ' + item.route);
      assert.equal(openPids(destination).length, 0, 'copy 직전 열린 target 거부: ' + item.route);
    }
    const source = fs.readFileSync(path.join(publication, item.route));
    const temporary = destination + '.roadmap-sync-' + randomUUID() + '.tmp';
    const modeBits = item.action === 'replace' ? current.stat.mode & 0o777 : 0o644;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: modeBits });
    fs.chmodSync(temporary, modeBits);
    fs.renameSync(temporary, destination);
    assert.equal(sha256(fs.readFileSync(destination)), item.sourceSha256, '복사 후 hash 불일치: ' + item.route);
    if (item.action === 'replace') replaced.push({ route: item.route, previousSha256: item.targetSha256, sha256: item.sourceSha256 });
    else created.push({ route: item.route, sha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length,
    createCount: created.length,
    replaceCount: replaced.length,
    identicalCount: identical.length,
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1375-final-2026-11-14',
    noDeletes: true,
    created,
    replaced,
    identical,
  };
  fs.writeFileSync(reports.copy, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2) + '\n');
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy가 필요함');
  const verifyReport = mode === 'verify-final' ? path.join(evidence, 'served-verification-final.json') : reports.verify;
  assert.ok(!fs.existsSync(verifyReport), '기존 served verification 증거 보존');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const server = await checkServer();
  const localResults = [];
  for (const route of sourcePaths) {
    const expected = routeManifest.get(route);
    const response = await fetch('http://127.0.0.1:8768/spinon/' + route.split('/').map(encodeURIComponent).join('/'), { signal: AbortSignal.timeout(20000) });
    assert.equal(response.status, 200, 'local preview HTTP status: ' + route);
    const bytes = Buffer.from(await response.arrayBuffer());
    assert.equal(bytes.length, expected.bytes, 'local preview size: ' + route);
    assert.equal(sha256(bytes), expected.sha256, 'local preview hash: ' + route);
    localResults.push({ route, bytes: bytes.length, sha256: expected.sha256, httpStatus: response.status, verified: true });
  }
  const backupChecks = [];
  for (const item of copied.replaced) {
    const backup = inspect(backupRoot, item.route);
    assert.ok(backup.exists, '이전 파일 backup 누락: ' + item.route);
    assert.equal(sha256(backup.bytes), item.previousSha256, 'backup hash 불일치: ' + item.route);
    backupChecks.push({ route: item.route, sha256: item.previousSha256, verified: true });
  }
  const remoteResults = new Array(sourcePaths.length);
  let next = 0;
  async function worker() {
    while (true) {
      const index = next++;
      if (index >= sourcePaths.length) return;
      const route = sourcePaths[index];
      const expected = routeManifest.get(route);
      const url = new URL(route.split('/').map(encodeURIComponent).join('/'), remoteBase);
      const response = await fetch(url, { signal: AbortSignal.timeout(25000) });
      assert.equal(response.status, 200, 'HTTPS preview HTTP status: ' + route);
      const bytes = Buffer.from(await response.arrayBuffer());
      assert.equal(bytes.length, expected.bytes, 'HTTPS preview size: ' + route);
      assert.equal(sha256(bytes), expected.sha256, 'HTTPS preview hash: ' + route);
      remoteResults[index] = { route, bytes: bytes.length, sha256: expected.sha256, httpStatus: response.status, verified: true };
    }
  }
  await Promise.all(Array.from({ length: 6 }, () => worker()));
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    trackedDiffSha256: manifest.trackedDiffSha256,
    selectedRouteCount: sourcePaths.length,
    localHttp200AndHashVerified: localResults.length,
    httpsHttp200AndHashVerified: remoteResults.length,
    backupsVerified: backupChecks.length,
    correctionApplied: false,
    previewServerPid: server.serverPid,
    previewServerRestarted: false,
    officialCount: '78/214',
    c11Complete: false,
    noDeletes: true,
    localResults,
    httpsResults: remoteResults,
    backups: backupChecks,
  };
  fs.writeFileSync(verifyReport, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ ...report, localResults: undefined, httpsResults: undefined, backups: undefined }, null, 2) + '\n');
}
