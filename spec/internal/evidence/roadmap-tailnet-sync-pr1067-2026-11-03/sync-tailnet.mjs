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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1067-2026-11-03/pass01');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));
const publication = path.join(repo, manifest.publicationRoot);
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const exec = (file, args) => execFileSync(file, args, { encoding: 'utf8' });
const [mode] = process.argv.slice(2);

function assertDir(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function inspect(root, route) {
  assertDir(root); safeRoute(route);
  let current = root;
  const parts = route.split('/');
  for (let i = 0; i < parts.length; i += 1) {
    current = path.join(current, parts[i]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink route 거부: ${route}`);
      if (i < parts.length - 1) assert.ok(stat.isDirectory(), `route parent가 디렉터리가 아님: ${route}`);
      else { assert.ok(stat.isFile(), `route가 일반 파일이 아님: ${route}`); return { exists: true, bytes: fs.readFileSync(current), stat }; }
    } catch (error) { if (error.code === 'ENOENT') return { exists: false }; throw error; }
  }
  return { exists: false };
}
function listFiles(directory, relative = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a,b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `publication symlink 거부: ${child}`);
    if (entry.isDirectory()) result.push(...listFiles(directory, child));
    else if (entry.isFile()) result.push(child.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication 항목: ${child}`);
  }
  return result;
}
function ensureDirectory(directory) {
  if (fs.existsSync(directory)) { assertDir(directory); return; }
  const parent = path.dirname(directory);
  if (parent !== directory) ensureDirectory(parent);
  fs.mkdirSync(directory, { recursive: false });
  assertDir(directory);
}
function openPids(filename) {
  const result = spawnSync('lsof', ['-nP'], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.equal(result.status, 0, '전체 lsof 결과를 읽지 못함');
  const pids = [];
  for (const line of result.stdout.split('\n')) {
    if (!line.endsWith(filename)) continue;
    const columns = line.trim().split(/\s+/);
    const pid = Number(columns[1]);
    if (Number.isInteger(pid)) pids.push(pid);
  }
  return [...new Set(pids)];
}
async function serverState() {
  const tailnet = JSON.parse(exec('tailscale', ['status', '--json']));
  assert.equal(tailnet.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assertDir(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink 또는 alias 경로임');
  const pids = [...new Set(exec('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN']).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, '8768 preview listener가 하나여야 함');
  const pid = Number(pids[0]);
  const args = exec('ps', ['-p', String(pid), '-o', 'args=']).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '8768 listener가 예상된 preview server가 아님');
  const listeners = exec('lsof', ['-nP', '-a', '-p', String(pid), '-iTCP', '-sTCP:LISTEN']);
  assert.ok(listeners.includes('127.0.0.1:8768'), 'preview listener 주소·port 불일치');
  const response = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, 'local roadmap가 HTTP 200이 아님');
  return { backendState: tailnet.BackendState, serverPid: pid, localStatus: response.status, localBytes: bytes.length, localSha256: sha256(bytes) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDir(publication);
const routes = listFiles(publication);
const expected = new Map(manifest.routes.map((item) => [item.path, item]));
assert.equal(routes.length, expected.size, 'publication과 manifest route 수 불일치');
for (const route of routes) assert.ok(expected.has(route), `manifest에 없는 publication route: ${route}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight 증거 보존');
  assert.ok(!fs.existsSync(backupRoot), `고유 backup 경로가 이미 존재함: ${backupRoot}`);
  const server = await serverState();
  const targets = [];
  for (const route of routes) {
    const source = fs.readFileSync(path.join(publication, route));
    const item = expected.get(route);
    assert.equal(source.length, item.bytes, `publication size 변경: ${route}`);
    assert.equal(sha256(source), item.sha256, `publication hash 변경: ${route}`);
    const target = inspect(previewRoot, route);
    const oldHash = target.exists ? sha256(target.bytes) : null;
    const action = !target.exists ? 'create' : oldHash === item.sha256 ? 'same' : 'replace';
    const file = path.join(previewRoot, route);
    const openReaderPids = action === 'replace' ? openPids(file) : [];
    targets.push({ route, action, sourceBytes: item.bytes, sourceSha256: item.sha256, targetExists: target.exists, targetBytes: target.exists ? target.bytes.length : null, targetSha256: oldHash, openReaderPids });
  }
  const counts = Object.fromEntries(['create','replace','same'].map((action) => [action, targets.filter((x) => x.action === action).length]));
  const report = {
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree, backendState: server.backendState,
    previewServerPid: server.serverPid, localRoadmapStatus: server.localStatus, localRoadmapBytes: server.localBytes,
    localRoadmapSha256: server.localSha256, selectedRouteCount: routes.length,
    selectedBytes: routes.reduce((sum, route) => sum + expected.get(route).bytes, 0), counts,
    routesWithPreExistingReaders: targets.filter((x) => x.openReaderPids.length).map((x) => ({ route: x.route, pids: x.openReaderPids })),
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1067-2026-11-03/pass01',
    replacementPolicy: 'Back up prior bytes and atomically rename a same-directory temporary file. Open readers retain their old inode; no existing route is truncated in place and no route is removed.',
    noDeletes: true, targets,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy evidence 보존');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, manifest.sourceHead);
  assert.equal(preflight.sourceTree, manifest.sourceTree);
  assert.equal(preflight.selectedRouteCount, routes.length);
  for (const target of preflight.targets) {
    const current = inspect(previewRoot, target.route);
    assert.equal(current.exists, target.targetExists, `preflight 후 target 존재 상태 변경: ${target.route}`);
    if (current.exists) assert.equal(sha256(current.bytes), target.targetSha256, `preflight 후 target 변경: ${target.route}`);
    assert.equal(sha256(fs.readFileSync(path.join(publication, target.route))), target.sourceSha256, `preflight 후 source 변경: ${target.route}`);
  }
  const replacements = preflight.targets.filter((x) => x.action === 'replace');
  if (replacements.length) { ensureDirectory(path.dirname(backupRoot)); fs.mkdirSync(backupRoot, { recursive: false }); assertDir(backupRoot); }
  for (const item of replacements) {
    const backup = path.join(backupRoot, item.route);
    ensureDirectory(path.dirname(backup));
    fs.copyFileSync(path.join(previewRoot, item.route), backup, fs.constants.COPYFILE_EXCL);
    assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${item.route}`);
  }
  const created = [], replaced = [], identical = [];
  for (const item of preflight.targets) {
    const sourcePath = path.join(publication, item.route);
    const source = fs.readFileSync(sourcePath);
    if (item.action === 'same') { identical.push({ route: item.route, sha256: item.sourceSha256 }); continue; }
    const destination = path.join(previewRoot, item.route);
    ensureDirectory(path.dirname(destination));
    const current = inspect(previewRoot, item.route);
    if (item.action === 'replace') assert.equal(sha256(current.bytes), item.targetSha256, `복사 직전 target 변경: ${item.route}`);
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
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree, selectedRouteCount: routes.length,
    createdCount: created.length, replacedCount: replaced.length, identicalCount: identical.length,
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1067-2026-11-03/pass01',
    replacementPolicy: preflight.replacementPolicy, noDeletes: true, created, replaced, identical,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy가 필요함');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify evidence 보존');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const checkRoutes = [...new Set([
    'roadmap.html', 'STATUS.html', 'plan/c11-css-grid.html',
    ...manifest.checkedRoadmapLinks.map((item) => item.route),
    'internal/0059-c11-grid-layout.html',
  ])];
  const checks = [];
  for (const route of checkRoutes) {
    const item = expected.get(route); assert.ok(item, `verify route가 manifest에 없음: ${route}`);
    const sourceBytes = fs.readFileSync(path.join(publication, route));
    assert.equal(sha256(sourceBytes), item.sha256, `publication hash 변경: ${route}`);
    const results = [];
    for (const [origin, url] of [['local', `http://127.0.0.1:8768/spinon/${route}`], ['tailnet', `${remoteBase}${route}`]]) {
      const response = await fetch(url, { signal: AbortSignal.timeout(15000) });
      const bytes = Buffer.from(await response.arrayBuffer());
      assert.equal(response.status, 200, `${origin} HTTP status ${route}`);
      assert.equal(sha256(bytes), item.sha256, `${origin} hash 불일치: ${route}`);
      results.push({ origin, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
    }
    checks.push({ route, expectedSha256: item.sha256, results });
  }
  const roadmap = checks.find((item) => item.route === 'roadmap.html');
  const roadmapText = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
  const start = roadmapText.indexOf('PR #1066 Rebase and merge');
  const card = roadmapText.slice(roadmapText.lastIndexOf('<p ', start), roadmapText.indexOf('</p>', start) + 4);
  assert.ok(card.includes('72/213') && card.includes('2dde385c5f240c8b8837b9e55d9c5431f5f9294b'), 'published C11 roadmap markers 누락');
  const report = {
    sourceHead: manifest.sourceHead, sourceTree: manifest.sourceTree,
    selectedRouteCount: manifest.routeCount, backupAndCopy: { created: copied.createdCount, replaced: copied.replacedCount, identical: copied.identicalCount },
    officialCount: '72/213', c11Complete: false,
    checkedRouteCount: checks.length, checks, roadmapContainsMergeAndCount: true,
    noDeletes: true,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
