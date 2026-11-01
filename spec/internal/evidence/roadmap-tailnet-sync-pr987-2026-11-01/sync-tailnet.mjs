import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const publicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-pass02.json'), 'utf8'));
const publicationRoot = path.join(repo, publicationManifest.publicationRoot);
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr987-2026-11-01/pass02';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const [mode] = process.argv.slice(2);
const reports = {
  preflight: path.join(evidence, 'preflight-pass02.json'),
  copy: path.join(evidence, 'copy-and-backup-pass02.json'),
  verify: path.join(evidence, 'served-verification-pass02.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function walk(directory, prefix = '') {
  const routes = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const route = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${route}`);
    if (entry.isDirectory()) routes.push(...walk(directory, route));
    else if (entry.isFile()) routes.push(route.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication 항목: ${route}`);
  }
  return routes;
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
  const local = fetch('http://127.0.0.1:8768/spinon/roadmap.html');
  return { status, serverPid, args, local };
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
const sourceRoutes = walk(publicationRoot);
const manifestByPath = new Map(publicationManifest.routes.map((route) => [route.path, route]));
assert.equal(sourceRoutes.length, manifestByPath.size, 'publication route 수가 manifest와 다름');
for (const route of sourceRoutes) assert.ok(manifestByPath.has(route), `manifest route 누락: ${route}`);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight report를 덮어쓰지 않음');
  const server = inspectServer();
  const response = await server.local;
  const body = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, '현재 local roadmap가 HTTP 200이어야 함');
  const localBefore = { status: response.status, bytes: body.length, sha256: sha256(body) };
  const targets = sourceRoutes.map((route) => {
    const source = fs.readFileSync(path.join(publicationRoot, route));
    const expected = manifestByPath.get(route);
    assert.equal(source.length, expected.bytes, `publication 크기 불일치: ${route}`);
    assert.equal(sha256(source), expected.sha256, `publication hash 불일치: ${route}`);
    const target = inspectPath(previewRoot, route);
    return {
      route,
      sourceBytes: source.length,
      sourceSha256: expected.sha256,
      targetExists: target.exists,
      targetBytes: target.exists ? target.bytes.length : null,
      targetSha256: target.exists ? sha256(target.bytes) : null,
    };
  });
  const openHandles = readOpenHandles(targets);
  assert.equal(openHandles.length, 0, `변경할 route가 열려 있음: ${JSON.stringify(openHandles)}`);
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    publicationRoot: publicationManifest.publicationRoot,
    previewRoot,
    backupRoot,
    tailscaleBackendState: server.status.BackendState,
    selfHostName: server.status.Self?.HostName ?? null,
    selfTailscaleIPs: server.status.Self?.TailscaleIPs ?? [],
    previewPid: server.serverPid,
    previewArgs: server.args,
    localRoadmapBefore: localBefore,
    routeCount: targets.length,
    changedRouteCount: targets.filter((item) => !item.targetExists || item.targetSha256 !== item.sourceSha256).length,
    changedBytes: targets.filter((item) => !item.targetExists || item.targetSha256 !== item.sourceSha256).reduce((sum, item) => sum + item.sourceBytes, 0),
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
  assert.ok(!fs.existsSync(backupRoot), `기존 backup을 덮어쓰지 않음: ${backupRoot}`);
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, publicationManifest.sourceHead);
  assert.equal(preflight.sourceTree, publicationManifest.sourceTree);
  assert.equal(preflight.trackedDiffSha256, publicationManifest.trackedDiffSha256);
  assert.equal(preflight.routeCount, sourceRoutes.length);
  assert.equal(preflight.openHandleCount, 0);
  const currentTargets = sourceRoutes.map((route) => {
    const item = preflight.targets.find((target) => target.route === route);
    assert.ok(item, `preflight route 누락: ${route}`);
    const source = fs.readFileSync(path.join(publicationRoot, route));
    assert.equal(sha256(source), item.sourceSha256, `preflight 이후 source 변경: ${route}`);
    const target = inspectPath(previewRoot, route);
    assert.equal(target.exists, item.targetExists, `preflight 이후 target 존재 상태 변경: ${route}`);
    if (target.exists) assert.equal(sha256(target.bytes), item.targetSha256, `preflight 이후 target 변경: ${route}`);
    return { route, item, source, target };
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
      const saved = fs.readFileSync(backup);
      assert.equal(sha256(saved), item.targetSha256, `backup hash 불일치: ${route}`);
    } else {
      fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    }
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: current.exists ? current.stat.mode & 0o777 : 0o644 });
    if (current.exists) fs.chmodSync(temporary, current.stat.mode & 0o777);
    fs.renameSync(temporary, targetPath);
    const updated = fs.readFileSync(targetPath);
    assert.equal(sha256(updated), item.sourceSha256, `atomic copy hash 불일치: ${route}`);
    if (current.exists) replaced.push({ route, previousSha256: item.targetSha256, sourceSha256: item.sourceSha256 });
    else created.push({ route, sourceSha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot,
    selectedRouteCount: sourceRoutes.length,
    selectedBytes: publicationManifest.totalBytes,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    openHandleCount: openHandles.length,
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
  const verifiedBytes = [];
  for (const route of sourceRoutes) {
    const source = fs.readFileSync(path.join(publicationRoot, route));
    const target = inspectPath(previewRoot, route);
    assert.ok(target.exists, `preview route 누락: ${route}`);
    assert.equal(target.bytes.length, source.length, `preview 크기 불일치: ${route}`);
    assert.equal(sha256(target.bytes), sha256(source), `preview hash 불일치: ${route}`);
    verifiedBytes.push(`${route}\0${source.length}\0${sha256(source)}\n`);
  }
  const routeHash = sha256(Buffer.from(verifiedBytes.join('')));
  const backupChecks = [];
  for (const item of copied.replaced) {
    const backup = inspectPath(backupRoot, item.route);
    assert.ok(backup.exists, `backup 누락: ${item.route}`);
    assert.equal(sha256(backup.bytes), item.previousSha256, `backup hash 불일치: ${item.route}`);
    backupChecks.push(item.route);
  }
  const criticalRoutes = publicationManifest.requiredRoutes;
  const critical = await Promise.all(criticalRoutes.map(async (route) => {
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
    previewRoot,
    backupRoot,
    selectedRouteCount: sourceRoutes.length,
    verifiedRouteCount: verifiedBytes.length,
    verifiedRouteSetSha256: routeHash,
    replacedRouteCount: copied.replacedCount,
    createdRouteCount: copied.createdCount,
    verifiedBackupCount: backupChecks.length,
    verifiedBackupRoutes: backupChecks,
    criticalHttpRoutes: critical,
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
