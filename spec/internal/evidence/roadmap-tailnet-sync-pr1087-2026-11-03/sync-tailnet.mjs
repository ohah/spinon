import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const publicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-routes.json'), 'utf8'));
const publicationRoot = path.join(repo, publicationManifest.patchRoot);
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1087-2026-11-03/pass01';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const [mode] = process.argv.slice(2);
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function inspectPath(root, route) {
  assertDirectory(root);
  const parts = route.split('/');
  assert.ok(parts.length && parts.every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `잘못된 route: ${route}`);
  let current = root;
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink 거부: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent가 디렉터리가 아님: ${route}`);
      else {
        assert.ok(stat.isFile(), `target가 파일이 아님: ${route}`);
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
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 예상 경로와 다름');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  return { status, serverPid, args, local: fetch('http://127.0.0.1:8768/spinon/roadmap.html') };
}

function readOpenHandles(targets) {
  const open = [];
  for (const item of targets.filter((target) => target.targetExists && target.targetSha256 !== target.sourceSha256)) {
    const target = path.join(previewRoot, item.route);
    const result = spawnSync('lsof', ['-nP', '--', target], { encoding: 'utf8', maxBuffer: 1024 * 1024 });
    assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
    const rows = result.stdout.trim().split('\n').slice(1).filter(Boolean);
    for (const row of rows) open.push({ route: item.route, command: row.split(/\s+/)[0], row });
  }
  return open;
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDirectory(publicationRoot);
assertDirectory(previewRoot);
const routeManifest = new Map(publicationManifest.routes.map((route) => [route.route, route]));
assert.equal(routeManifest.size, publicationManifest.routeCount, 'patch route 수가 manifest와 다름');
for (const route of publicationManifest.routes) {
  const source = fs.readFileSync(path.join(publicationRoot, route.route));
  assert.equal(source.length, route.bytes, `patch 크기 불일치: ${route.route}`);
  assert.equal(sha256(source), route.sha256, `patch hash 불일치: ${route.route}`);
}

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight report를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup을 덮어쓰지 않음: ${backupRoot}`);
  const server = inspectServer();
  const response = await server.local;
  const localBytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, '현재 local roadmap가 HTTP 200이어야 함');
  const targets = publicationManifest.routes.map((route) => {
    const target = inspectPath(previewRoot, route.route);
    return {
      route: route.route,
      sourceBytes: route.bytes,
      sourceSha256: route.sha256,
      targetExists: target.exists,
      targetBytes: target.exists ? target.bytes.length : null,
      targetSha256: target.exists ? sha256(target.bytes) : null,
    };
  });
  const openHandles = readOpenHandles(targets);
  assert.equal(openHandles.length, 0, `변경 route가 열려 있음: ${JSON.stringify(openHandles)}`);
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    patchRouteCount: targets.length,
    patchBytes: publicationManifest.totalBytes,
    previewRoot,
    backupRoot,
    remoteBase,
    tailscaleBackendState: server.status.BackendState,
    selfHostName: server.status.Self?.HostName ?? null,
    previewPid: server.serverPid,
    previewArgs: server.args,
    localRoadmapBefore: { status: response.status, bytes: localBytes.length, sha256: sha256(localBytes) },
    changedRouteCount: targets.filter((item) => !item.targetExists || item.targetSha256 !== item.sourceSha256).length,
    openHandleCount: openHandles.length,
    openHandles,
    noDeletes: true,
    previewServerRestarted: false,
    targets,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight를 실행해야 함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy report를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup을 덮어쓰지 않음: ${backupRoot}`);
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, publicationManifest.sourceHead);
  assert.equal(preflight.sourceTree, publicationManifest.sourceTree);
  assert.equal(preflight.trackedDiffSha256, publicationManifest.trackedDiffSha256);
  assert.equal(preflight.patchRouteCount, publicationManifest.routeCount);
  assert.equal(preflight.openHandleCount, 0);
  const currentTargets = publicationManifest.routes.map((route) => {
    const item = preflight.targets.find((target) => target.route === route.route);
    assert.ok(item, `preflight route 누락: ${route.route}`);
    const source = fs.readFileSync(path.join(publicationRoot, route.route));
    assert.equal(sha256(source), item.sourceSha256, `preflight 이후 source 변경: ${route.route}`);
    const target = inspectPath(previewRoot, route.route);
    assert.equal(target.exists, item.targetExists, `preflight 이후 target 존재 상태 변경: ${route.route}`);
    if (target.exists) assert.equal(sha256(target.bytes), item.targetSha256, `preflight 이후 target 변경: ${route.route}`);
    return { route: route.route, item, source, target };
  });
  const openHandles = readOpenHandles(currentTargets.map(({ route, item, target }) => ({
    route,
    targetExists: target.exists,
    targetSha256: target.exists ? sha256(target.bytes) : null,
    sourceSha256: item.sourceSha256,
  })));
  assert.equal(openHandles.length, 0, `copy 직전 변경 route가 열려 있음: ${JSON.stringify(openHandles)}`);
  fs.mkdirSync(backupRoot, { recursive: true });
  const created = [];
  const replaced = [];
  const identical = [];
  for (const { route, item, source, target: current } of currentTargets) {
    const targetPath = path.join(previewRoot, route);
    if (current.exists && sha256(current.bytes) === item.sourceSha256) {
      identical.push(route);
      continue;
    }
    if (current.exists) {
      const backup = path.join(backupRoot, route);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(targetPath, backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${route}`);
    } else {
      fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    }
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: current.exists ? current.stat.mode & 0o777 : 0o644 });
    if (current.exists) fs.chmodSync(temporary, current.stat.mode & 0o777);
    fs.renameSync(temporary, targetPath);
    assert.equal(sha256(fs.readFileSync(targetPath)), item.sourceSha256, `atomic copy hash 불일치: ${route}`);
    if (current.exists) replaced.push({ route, previousSha256: item.targetSha256, sourceSha256: item.sourceSha256 });
    else created.push({ route, sourceSha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot,
    patchRouteCount: publicationManifest.routeCount,
    patchBytes: publicationManifest.totalBytes,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    openHandleCount: openHandles.length,
    noDeletes: true,
    previewServerRestarted: false,
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
  const verifiedRoutes = [];
  for (const route of publicationManifest.routes) {
    const source = fs.readFileSync(path.join(publicationRoot, route.route));
    const target = inspectPath(previewRoot, route.route);
    assert.ok(target.exists, `preview route 누락: ${route.route}`);
    assert.equal(target.bytes.length, source.length, `preview 크기 불일치: ${route.route}`);
    assert.equal(sha256(target.bytes), sha256(source), `preview hash 불일치: ${route.route}`);
    verifiedRoutes.push({ route: route.route, bytes: source.length, sha256: sha256(source) });
  }
  const backupChecks = [];
  for (const item of copied.replaced) {
    const backup = inspectPath(backupRoot, item.route);
    assert.ok(backup.exists, `backup 누락: ${item.route}`);
    assert.equal(sha256(backup.bytes), item.previousSha256, `backup hash 불일치: ${item.route}`);
    backupChecks.push(item.route);
  }
  const critical = await Promise.all(publicationManifest.requiredRoutes.map(async (route) => {
    const expected = fs.readFileSync(path.join(publicationRoot, route));
    const localResponse = await fetch(`http://127.0.0.1:8768/spinon/${route}`);
    const localBytes = Buffer.from(await localResponse.arrayBuffer());
    assert.equal(localResponse.status, 200, `local HTTP status: ${route}`);
    assert.equal(sha256(localBytes), sha256(expected), `local HTTP hash 불일치: ${route}`);
    const remoteResponse = await fetch(new URL(route, remoteBase));
    const remoteBytes = Buffer.from(await remoteResponse.arrayBuffer());
    assert.equal(remoteResponse.status, 200, `Tailnet HTTP status: ${route}`);
    assert.equal(sha256(remoteBytes), sha256(expected), `Tailnet HTTP hash 불일치: ${route}`);
    return { route, bytes: expected.length, sha256: sha256(expected), localStatus: localResponse.status, remoteStatus: remoteResponse.status, verified: true };
  }));
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    patchRouteCount: publicationManifest.routeCount,
    verifiedRouteCount: verifiedRoutes.length,
    verifiedPatchSha256: sha256(Buffer.from(verifiedRoutes.map((item) => `${item.route}\0${item.bytes}\0${item.sha256}\n`).join(''))),
    backupCheckCount: backupChecks.length,
    backupChecks,
    localAndTailnetHttp: critical,
    noDeletes: true,
    previewServerRestarted: false,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, localAndTailnetHttp: undefined }, null, 2)}\n`);
}
