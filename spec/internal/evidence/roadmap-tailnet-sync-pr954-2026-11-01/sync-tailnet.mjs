import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr954-2026-11-01/pass01';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const publicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-tailnet-pass01.json'), 'utf8'));
const publicationRoot = path.join(repo, publicationManifest.publicationRoot);
const [mode] = process.argv.slice(2);
const reports = {
  preflight: path.join(evidence, 'tailnet-preflight-pass01.json'),
  copy: path.join(evidence, 'tailnet-copy-and-backup-pass01.json'),
  verify: path.join(evidence, 'tailnet-served-verification-pass01.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();

function assertDirectoryNoSymlink(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) paths.push(...listFiles(directory, relative));
    else if (entry.isFile()) paths.push(relative);
    else throw new Error(`지원하지 않는 source 항목: ${relative}`);
  }
  return paths;
}

function inspectPath(root, relative) {
  assertDirectoryNoSymlink(root);
  let current = root;
  const parts = relative.split('/');
  assert.ok(parts.length > 0 && parts.every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `잘못된 route: ${relative}`);
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink 거부: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent가 디렉터리가 아님: ${relative}`);
      else {
        assert.ok(stat.isFile(), `target가 파일이 아님: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function inspectServer() {
  const status = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(status.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, '미리보기 root는 설정된 실제 디렉터리여야 함');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  const listener = sockets.filter((row) => row.includes('127.0.0.1:8768') && row.includes('(LISTEN)'));
  assert.equal(listener.length, 1, `127.0.0.1:8768 listener 검증 실패: ${sockets.join('\n')}`);
  const local = fetch('http://127.0.0.1:8768/spinon/roadmap.html');
  return { status, serverPid, args, sockets, local };
}

function readOpenHandles(targets) {
  const existing = targets.filter((item) => item.targetExists).map((item) => item.target);
  if (!existing.length) return [];
  const result = spawnSync('lsof', ['-nP', '--', ...existing], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  const rows = result.stdout.trim().split('\n').slice(1).filter(Boolean);
  return rows.map((row) => {
    const index = row.indexOf(previewRoot);
    assert.ok(index >= 0, `예상하지 못한 lsof 경로: ${row}`);
    const target = row.slice(index);
    const relative = path.relative(previewRoot, target).split(path.sep).join('/');
    const item = targets.find((candidate) => candidate.route === relative);
    assert.ok(item, `선택되지 않은 열린 route: ${relative}`);
    return { command: row.split(/\s+/)[0], path: relative, currentSha256: item.targetSha256, nextSha256: item.sourceSha256 };
  });
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDirectoryNoSymlink(publicationRoot);
const sourcePaths = listFiles(publicationRoot);
const byRoute = new Map(publicationManifest.routes.map((route) => [route.path, route]));
assert.equal(sourcePaths.length, byRoute.size, 'publication route 수 변경');
for (const relative of sourcePaths) assert.ok(byRoute.has(relative), `manifest에 없는 route: ${relative}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight report를 덮어쓰지 않음');
  const server = inspectServer();
  const localResponse = await server.local;
  const localBytes = Buffer.from(await localResponse.arrayBuffer());
  assert.equal(localResponse.status, 200, '현재 local preview roadmap가 HTTP 200이어야 함');
  const targets = sourcePaths.map((route) => {
    const sourceBytes = fs.readFileSync(path.join(publicationRoot, route));
    const manifestRoute = byRoute.get(route);
    assert.equal(sourceBytes.length, manifestRoute.bytes, `publication 크기 변경: ${route}`);
    assert.equal(sha256(sourceBytes), manifestRoute.sha256, `publication hash 변경: ${route}`);
    const target = inspectPath(previewRoot, route);
    const targetSha256 = target.exists ? sha256(target.bytes) : null;
    return {
      route,
      sourceBytes: sourceBytes.length,
      sourceSha256: sha256(sourceBytes),
      target: target.exists ? 'exists' : 'missing',
      targetExists: target.exists,
      targetBytes: target.exists ? target.bytes.length : null,
      targetSha256,
      action: !target.exists ? 'create' : targetSha256 === sha256(sourceBytes) ? 'same' : 'replace',
      targetPath: path.join(previewRoot, route),
    };
  });
  const openHandles = readOpenHandles(targets);
  for (const item of openHandles) {
    assert.equal(item.currentSha256, item.nextSha256, `변경할 열린 preview route 거부: ${item.path}`);
  }
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    tailscaleBackendState: server.status.BackendState,
    selfHostName: server.status.Self?.HostName ?? null,
    selfTailscaleIPs: server.status.Self?.TailscaleIPs ?? [],
    previewPid: server.serverPid,
    previewArgs: server.args,
    previewListen: '127.0.0.1:8768',
    localRoadmapStatus: localResponse.status,
    localRoadmapBytes: localBytes.length,
    localRoadmapSha256: sha256(localBytes),
    previewRoot,
    backupRoot,
    routeCount: targets.length,
    targetBytes: targets.reduce((sum, item) => sum + item.sourceBytes, 0),
    counts,
    openHandleCount: openHandles.length,
    openHandles,
    noDeletes: true,
    targets,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight를 실행해야 함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy report를 덮어쓰지 않음');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, publicationManifest.sourceHead);
  assert.equal(preflight.sourceTree, publicationManifest.sourceTree);
  assert.equal(preflight.trackedDiffSha256, publicationManifest.trackedDiffSha256);
  assert.equal(preflight.routeCount, sourcePaths.length);
  assert.ok(preflight.openHandles.every((item) => item.currentSha256 === item.nextSha256), '변경할 열린 route가 없어야 함');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup 경로를 덮어쓰지 않음: ${backupRoot}`);

  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publicationRoot, route));
    const item = preflight.targets.find((entry) => entry.route === route);
    assert.ok(item, `preflight route 누락: ${route}`);
    assert.equal(sha256(source), item.sourceSha256, `preflight 이후 source가 변경됨: ${route}`);
    const current = inspectPath(previewRoot, route);
    assert.equal(current.exists, item.targetExists, `preflight 이후 target 존재 상태가 변경됨: ${route}`);
    if (current.exists) assert.equal(sha256(current.bytes), item.targetSha256, `preflight 이후 target이 변경됨: ${route}`);
  }

  const backupParent = path.dirname(path.dirname(backupRoot));
  assertDirectoryNoSymlink(backupParent);
  fs.mkdirSync(path.dirname(backupRoot), { recursive: false });
  fs.mkdirSync(backupRoot, { recursive: false });
  const created = [];
  const replaced = [];
  const identical = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publicationRoot, route));
    const item = preflight.targets.find((entry) => entry.route === route);
    const targetPath = path.join(previewRoot, route);
    const current = inspectPath(previewRoot, route);
    if (current.exists && sha256(current.bytes) === item.sourceSha256) {
      identical.push({ path: route, bytes: current.bytes.length, sha256: item.sourceSha256 });
      continue;
    }
    if (current.exists) {
      assert.ok(!preflight.openHandles.some((handle) => handle.path === route), `열린 route 덮어쓰기 거부: ${route}`);
      const backup = path.join(backupRoot, route);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(targetPath, backup, fs.constants.COPYFILE_EXCL);
      const backupBytes = fs.readFileSync(backup);
      assert.equal(sha256(backupBytes), item.targetSha256, `backup hash 불일치: ${route}`);
    } else {
      fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    }
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: current.exists ? current.stat.mode & 0o777 : 0o644 });
    if (current.exists) fs.chmodSync(temporary, current.stat.mode & 0o777);
    fs.renameSync(temporary, targetPath);
    const updated = fs.readFileSync(targetPath);
    assert.equal(sha256(updated), item.sourceSha256, `Tailnet local target hash 불일치: ${route}`);
    if (current.exists) {
      replaced.push({ path: route, bytes: updated.length, sha256: item.sourceSha256, backupPath: path.relative(backupRoot, path.join(backupRoot, route)), previousBytes: current.bytes.length, previousSha256: item.targetSha256 });
    } else created.push({ path: route, bytes: updated.length, sha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot,
    selectedRouteCount: sourcePaths.length,
    selectedBytes: preflight.targetBytes,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    openHandleCount: preflight.openHandleCount,
    noDeletes: true,
    created,
    replaced,
    identical,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy를 실행해야 함');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify report를 덮어쓰지 않음');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const localTargets = [];
  for (const route of sourcePaths) {
    const source = fs.readFileSync(path.join(publicationRoot, route));
    const target = inspectPath(previewRoot, route);
    assert.ok(target.exists, `미리보기 target 누락: ${route}`);
    assert.equal(target.bytes.length, source.length, `미리보기 target 크기 불일치: ${route}`);
    assert.equal(sha256(target.bytes), sha256(source), `미리보기 target hash 불일치: ${route}`);
    localTargets.push({ path: route, bytes: source.length, sha256: sha256(source), verified: true });
  }

  const backupChecks = [];
  for (const item of copied.replaced) {
    const backup = inspectPath(backupRoot, item.backupPath);
    assert.ok(backup.exists, `backup 누락: ${item.backupPath}`);
    assert.equal(backup.bytes.length, item.previousBytes, `backup 크기 불일치: ${item.path}`);
    assert.equal(sha256(backup.bytes), item.previousSha256, `backup hash 불일치: ${item.path}`);
    backupChecks.push({ path: item.path, bytes: backup.bytes.length, sha256: item.previousSha256, verified: true });
  }

  const remote = new Array(sourcePaths.length);
  let next = 0;
  await Promise.all(Array.from({ length: 16 }, async () => {
    while (true) {
      const index = next;
      next += 1;
      if (index >= sourcePaths.length) return;
      const route = sourcePaths[index];
      const response = await fetch(new URL(route.split('/').map(encodeURIComponent).join('/'), remoteBase));
      const bytes = Buffer.from(await response.arrayBuffer());
      const expected = byRoute.get(route);
      assert.equal(response.status, 200, `HTTPS 상태 ${response.status}: ${route}`);
      assert.equal(bytes.length, expected.bytes, `HTTPS 크기 불일치: ${route}`);
      assert.equal(sha256(bytes), expected.sha256, `HTTPS hash 불일치: ${route}`);
      remote[index] = { path: route, status: response.status, bytes: bytes.length, sha256: expected.sha256, verified: true };
    }
  }));

  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot,
    remoteBase,
    routeCount: sourcePaths.length,
    verifiedBytes: remote.reduce((sum, item) => sum + item.bytes, 0),
    allLocalTargetsVerified: localTargets.every((item) => item.verified),
    backupCount: backupChecks.length,
    allBackupsVerified: backupChecks.every((item) => item.verified),
    allHttp200: remote.every((item) => item.status === 200),
    allRemoteHashesMatch: remote.every((item) => item.verified),
    serverRestarted: false,
    noDeletes: true,
    localTargets,
    backups: backupChecks,
    remote,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, localTargets: undefined, backups: undefined, remote: undefined }, null, 2)}\n`);
}
