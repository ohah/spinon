import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const publicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));
const publicationRoot = path.join(repo, publicationManifest.publicationRoot);
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-anonymous-grid-text-line-break-roadmap-sync-2026-11-01';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function assertDirectoryNoSymlink(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function inspectPath(root, route) {
  safeRoute(route);
  let current = root;
  for (const part of route.split('/').slice(0, -1)) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `route 상위 경로가 안전하지 않음: ${current}`);
    } catch (error) {
      if (error.code === 'ENOENT') continue;
      throw error;
    }
  }
  const target = path.join(root, route);
  try {
    const stat = fs.lstatSync(target);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `대상이 regular file이 아님: ${target}`);
    const bytes = fs.readFileSync(target);
    return { exists: true, bytes, sha256: sha256(bytes) };
  } catch (error) {
    if (error.code === 'ENOENT') return { exists: false };
    throw error;
  }
}

function inspectServer() {
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener는 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  assert.ok(sockets.some((row) => row.includes('127.0.0.1:8768') && row.includes('(LISTEN)')), '127.0.0.1:8768 listener 확인');
  return { serverPid, args, sockets };
}

function readOpenHandles(targets) {
  const paths = targets.filter((item) => item.targetExists).map((item) => item.targetPath);
  if (!paths.length) return [];
  const result = spawnSync('lsof', ['-nP', '--', ...paths], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split('\n').slice(1).filter(Boolean).map((row) => {
    const index = row.indexOf(previewRoot);
    assert.ok(index >= 0, `예상하지 못한 열린 파일: ${row}`);
    const openedPath = row.slice(index);
    const route = path.relative(previewRoot, openedPath).split(path.sep).join('/');
    const item = targets.find((candidate) => candidate.route === route);
    assert.ok(item, `선택되지 않은 열린 route: ${route}`);
    return { command: row.split(/\s+/)[0], path: route, currentSha256: item.targetSha256, nextSha256: item.sourceSha256 };
  });
}

async function fetchRecord(url) {
  try {
    const response = await fetch(url, { signal: AbortSignal.timeout(15000), cache: 'no-store' });
    const bytes = Buffer.from(await response.arrayBuffer());
    return { status: response.status, bytes: bytes.length, sha256: sha256(bytes) };
  } catch (error) {
    return { error: String(error) };
  }
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertDirectoryNoSymlink(publicationRoot);
assertDirectoryNoSymlink(previewRoot);
const routes = [...publicationManifest.routes].sort((a, b) => a.path.localeCompare(b.path));
const manifestByRoute = new Map(routes.map((item) => [item.path, item]));

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight report를 덮어쓰지 않음');
  const server = inspectServer();
  const localRoadmap = await fetchRecord('http://127.0.0.1:8768/spinon/roadmap.html');
  assert.equal(localRoadmap.status, 200, 'local preview roadmap는 HTTP 200이어야 함');
  const currentRemote = await fetchRecord(`${remoteBase}roadmap.html`);
  const targets = routes.map((item) => {
    const sourceBytes = fs.readFileSync(path.join(publicationRoot, item.path));
    assert.equal(sourceBytes.length, item.bytes, `publication 크기 변경: ${item.path}`);
    assert.equal(sha256(sourceBytes), item.sha256, `publication hash 변경: ${item.path}`);
    const target = inspectPath(previewRoot, item.path);
    return {
      route: item.path,
      sourceBytes: item.bytes,
      sourceSha256: item.sha256,
      targetExists: target.exists,
      targetBytes: target.exists ? target.bytes.length : null,
      targetSha256: target.exists ? target.sha256 : null,
      targetPath: path.join(previewRoot, item.path),
      action: !target.exists ? 'create' : target.sha256 === item.sha256 ? 'same' : 'replace',
    };
  });
  const openHandles = readOpenHandles(targets);
  for (const item of openHandles) assert.equal(item.currentSha256, item.nextSha256, `열린 파일 변경을 거부: ${item.path}`);
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot,
    server,
    localRoadmap,
    currentRemoteRoadmap: currentRemote,
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
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy report를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup을 덮어쓰지 않음: ${backupRoot}`);
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, publicationManifest.sourceHead, 'source HEAD가 preflight와 같아야 함');
  const targets = preflight.targets;
  const openHandles = readOpenHandles(targets);
  for (const item of openHandles) assert.equal(item.currentSha256, item.nextSha256, `열린 파일 변경을 거부: ${item.path}`);
  const replaceItems = targets.filter((item) => item.action === 'replace');
  if (replaceItems.length) {
    const backupParent = path.dirname(backupRoot);
    assertDirectoryNoSymlink(backupParent);
    fs.mkdirSync(backupRoot, { recursive: false });
  }
  const created = [];
  const replaced = [];
  const same = [];
  for (const item of targets) {
    const current = inspectPath(previewRoot, item.route);
    assert.equal(current.exists, item.targetExists, `대상 존재 상태가 preflight 뒤 변경됨: ${item.route}`);
    if (current.exists) assert.equal(current.sha256, item.targetSha256, `대상 hash가 preflight 뒤 변경됨: ${item.route}`);
    const bytes = fs.readFileSync(path.join(publicationRoot, item.route));
    assert.equal(sha256(bytes), item.sourceSha256, `source hash 변경: ${item.route}`);
    if (item.action === 'same') {
      same.push({ path: item.route, bytes: bytes.length, sha256: item.sourceSha256 });
      continue;
    }
    const targetPath = path.join(previewRoot, item.route);
    const parent = path.dirname(targetPath);
    const relativeParent = path.relative(previewRoot, parent);
    let currentParent = previewRoot;
    for (const part of relativeParent.split(path.sep).filter(Boolean)) {
      currentParent = path.join(currentParent, part);
      if (!fs.existsSync(currentParent)) fs.mkdirSync(currentParent, { recursive: false });
      assertDirectoryNoSymlink(currentParent);
    }
    if (item.action === 'replace') {
      const backup = path.join(backupRoot, item.route);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(targetPath, backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), item.targetSha256, `backup hash 불일치: ${item.route}`);
    }
    const temporary = path.join(parent, `.spinon-preview-${randomUUID()}.tmp`);
    fs.writeFileSync(temporary, bytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(temporary)), item.sourceSha256, `temporary hash 불일치: ${item.route}`);
    fs.renameSync(temporary, targetPath);
    if (item.action === 'replace') replaced.push({ path: item.route, bytes: bytes.length, sha256: item.sourceSha256, backup: path.relative(backupRoot, path.join(backupRoot, item.route)), previousBytes: current.bytes.length, previousSha256: item.targetSha256 });
    else created.push({ path: item.route, bytes: bytes.length, sha256: item.sourceSha256 });
  }
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    backupRoot: replaceItems.length ? backupRoot : null,
    routeCount: targets.length,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: same.length,
    noDeletes: true,
    created,
    replaced,
    identical: same,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.preflight) && fs.existsSync(reports.copy), 'verify 전에 preflight와 copy가 필요함');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify report를 덮어쓰지 않음');
  const copy = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const diskChecks = [];
  for (const item of routes) {
    const target = inspectPath(previewRoot, item.path);
    assert.ok(target.exists, `preview route 누락: ${item.path}`);
    assert.equal(target.bytes.length, item.bytes, `preview route 크기 불일치: ${item.path}`);
    assert.equal(target.sha256, item.sha256, `preview route hash 불일치: ${item.path}`);
    diskChecks.push({ path: item.path, bytes: item.bytes, sha256: item.sha256 });
  }
  const server = inspectServer();
  const routesToServe = [...new Set([
    'roadmap.html',
    ...publicationManifest.requiredRoutes.filter((route) => route !== 'roadmap.html'),
  ])];
  const served = [];
  for (const route of routesToServe) {
    const local = await fetchRecord(`http://127.0.0.1:8768/spinon/${route}`);
    assert.equal(local.status, 200, `local HTTP status 오류: ${route}`);
    assert.equal(local.sha256, manifestByRoute.get(route).sha256, `local served hash 불일치: ${route}`);
    const remote = await fetchRecord(`${remoteBase}${route}`);
    assert.equal(remote.status, 200, `Tailnet HTTP status 오류: ${route}: ${remote.error ?? ''}`);
    assert.equal(remote.sha256, manifestByRoute.get(route).sha256, `Tailnet served hash 불일치: ${route}`);
    served.push({ path: route, local, remote, expectedSha256: manifestByRoute.get(route).sha256 });
  }
  const report = {
    sourceHead: publicationManifest.sourceHead,
    sourceTree: publicationManifest.sourceTree,
    trackedDiffSha256: publicationManifest.trackedDiffSha256,
    previewRoot,
    server,
    routeCount: diskChecks.length,
    diskChecks,
    served,
    backupRoot: copy.backupRoot,
    noDeletes: true,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, diskChecks: undefined }, null, 2)}\n`);
}
