import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { randomUUID, createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const publicationPath = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1378-c11-4-2026-11-14/pass02/publication');
const manifestPath = path.join(evidence, 'selected-publication-pass02.json');
const previewRoot = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1378-c11-4-2026-11-14/pass02');
const origin = 'https://macstudio.tailed42f2.ts.net';
const [mode] = process.argv.slice(2);
const reportPaths = {
  preflight: path.join(evidence, 'preflight-pass02.json'),
  copy: path.join(evidence, 'copy-and-backup-pass02.json'),
  verify: path.join(evidence, 'served-verification-pass02.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
assert.equal(manifest.officialCount, '78/214');
assert.equal(manifest.c11Complete, false);
assert.ok(fs.realpathSync(previewRoot) === previewRoot, 'Tailnet preview root은 설정된 실제 디렉터리여야 함');
assert.equal(fs.existsSync(publicationPath), true, '선택 publication이 있어야 함');

function assertDirectoryNoSymlink(directory) {
  const parsed = path.parse(directory);
  let current = parsed.root;
  for (const segment of directory.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, segment);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink() && stat.isDirectory(), `안전하지 않은 디렉터리: ${current}`);
  }
}

function inspectPath(root, relative) {
  assert.ok(!path.posix.isAbsolute(relative) && relative.split('/').every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `잘못된 route: ${relative}`);
  let current = root;
  const parts = relative.split('/');
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
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' });
  assert.ok(sockets.includes('127.0.0.1:8768') && sockets.includes('(LISTEN)'), '기존 preview listener가 확인되지 않음');
  return { backendState: tailscale.BackendState, serverPid };
}

function readOpenHandles(targets) {
  const existing = targets.filter((item) => item.targetExists).map((item) => item.targetPath);
  if (!existing.length) return [];
  const result = spawnSync('lsof', ['-nP', '--', ...existing], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split('\n').slice(1).filter(Boolean).map((row) => {
    const index = row.indexOf(previewRoot);
    assert.ok(index >= 0, `예상하지 못한 lsof 경로: ${row}`);
    const filePath = row.slice(index);
    const route = path.relative(previewRoot, filePath).split(path.sep).join('/');
    return { command: row.split(/\s+/)[0], route };
  });
}

function listTargets() {
  const routes = [...manifest.routes].sort((a, b) => a.path.localeCompare(b.path));
  assert.ok(routes.length > 0, 'publication route가 비어 있음');
  return routes.map((route) => {
    const sourcePath = path.join(publicationPath, route.path);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceBytes.length, route.bytes, `publication byte 수 변경: ${route.path}`);
    assert.equal(sha256(sourceBytes), route.sha256, `publication hash 변경: ${route.path}`);
    const target = inspectPath(previewRoot, route.path);
    const targetHash = target.exists ? sha256(target.bytes) : null;
    return {
      path: route.path,
      sourceBytes: sourceBytes.length,
      sourceSha256: route.sha256,
      targetExists: target.exists,
      previousBytes: target.exists ? target.bytes.length : null,
      previousSha256: targetHash,
      action: !target.exists ? 'create' : targetHash === route.sha256 ? 'same' : 'replace',
      targetPath: path.join(previewRoot, route.path),
    };
  });
}

function writeReport(file, value) {
  assert.ok(!fs.existsSync(file), `기존 report를 덮어쓰지 않음: ${file}`);
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
}

assertDirectoryNoSymlink(previewRoot);
if (mode === 'preflight') {
  assert.ok(!fs.existsSync(backupRoot), `기존 backup root를 덮어쓰지 않음: ${backupRoot}`);
  const server = inspectServer();
  const targets = listTargets();
  const openHandles = readOpenHandles(targets);
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, targets.filter((item) => item.action === action).length]));
  const local = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { cache: 'no-store' });
  assert.equal(local.status, 200, '현재 local preview roadmap가 HTTP 200이어야 함');
  const currentRoadmap = Buffer.from(await local.arrayBuffer());
  const oldRoadmap = targets.find((item) => item.path === 'roadmap.html');
  assert.ok(oldRoadmap?.targetExists, '기존 roadmap.html이 있어야 함');
  assert.equal(sha256(currentRoadmap), oldRoadmap.previousSha256, 'local server 응답과 기존 roadmap 파일이 일치해야 함');
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    selectedInputsSha256: manifest.selectedInputsSha256,
    tailscaleBackendState: server.backendState,
    previewServerVerified: true,
    previewPidUnchanged: server.serverPid,
    previewRoot: 'SpinonPreview',
    backupRoot: 'SpinonPreviewBackups/roadmap-tailnet-sync-pr1378-c11-4-2026-11-14/pass02',
    localRoadmapStatus: local.status,
    localRoadmapBytes: currentRoadmap.length,
    localRoadmapSha256: sha256(currentRoadmap),
    routeCount: targets.length,
    sourceBytes: targets.reduce((sum, item) => sum + item.sourceBytes, 0),
    counts,
    openHandleCount: openHandles.length,
    openHandles,
    noDeletes: true,
    targets: targets.map(({ targetPath, ...item }) => item),
  };
  writeReport(reportPaths.preflight, report);
  process.stdout.write(`${JSON.stringify({ ...report, targets: undefined }, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reportPaths.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reportPaths.copy), '기존 copy report를 덮어쓰지 않음');
  const preflight = JSON.parse(fs.readFileSync(reportPaths.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, manifest.sourceHead);
  assert.equal(preflight.sourceTree, manifest.sourceTree);
  assert.equal(preflight.selectedInputsSha256, manifest.selectedInputsSha256);
  assert.equal(preflight.routeCount, manifest.routes.length);
  assert.ok(!fs.existsSync(backupRoot), `기존 backup root를 덮어쓰지 않음: ${backupRoot}`);
  const currentTargets = listTargets();
  assert.equal(currentTargets.length, preflight.targets.length);
  for (let index = 0; index < currentTargets.length; index += 1) {
    const current = currentTargets[index];
    const expected = preflight.targets[index];
    assert.equal(current.path, expected.path);
    assert.equal(current.targetExists, expected.targetExists, `preflight 후 target 상태 변경: ${current.path}`);
    assert.equal(current.previousSha256, expected.previousSha256, `preflight 후 target hash 변경: ${current.path}`);
    assert.equal(current.sourceSha256, expected.sourceSha256, `preflight 후 source hash 변경: ${current.path}`);
  }
  fs.mkdirSync(path.dirname(backupRoot), { recursive: false });
  fs.mkdirSync(backupRoot, { recursive: false });
  const created = [];
  const replaced = [];
  const unchanged = [];
  const backups = [];
  for (const item of currentTargets) {
    if (item.action === 'same') {
      unchanged.push({ path: item.path, bytes: item.sourceBytes, sha256: item.sourceSha256 });
      continue;
    }
    const sourceBytes = fs.readFileSync(path.join(publicationPath, item.path));
    const targetPath = item.targetPath;
    const current = inspectPath(previewRoot, item.path);
    if (current.exists) {
      assert.equal(sha256(current.bytes), item.previousSha256, `copy 직전 target 변경: ${item.path}`);
      const backup = path.join(backupRoot, item.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(targetPath, backup, fs.constants.COPYFILE_EXCL);
      const backupBytes = fs.readFileSync(backup);
      assert.equal(sha256(backupBytes), item.previousSha256, `backup hash 불일치: ${item.path}`);
      backups.push({ path: item.path, bytes: backupBytes.length, sha256: sha256(backupBytes), verified: true });
    } else {
      fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    }
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    const modeBits = current.exists ? current.stat.mode & 0o777 : 0o644;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx', mode: modeBits });
    fs.chmodSync(temporary, modeBits);
    fs.renameSync(temporary, targetPath);
    const updated = fs.readFileSync(targetPath);
    assert.equal(updated.length, item.sourceBytes, `target byte 수 불일치: ${item.path}`);
    assert.equal(sha256(updated), item.sourceSha256, `target hash 불일치: ${item.path}`);
    const result = { path: item.path, bytes: updated.length, sha256: item.sourceSha256, atomicRename: true };
    if (current.exists) replaced.push({ ...result, previousBytes: current.bytes.length, previousSha256: item.previousSha256 });
    else created.push(result);
  }
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    selectedInputsSha256: manifest.selectedInputsSha256,
    routeCount: currentTargets.length,
    counts: { create: created.length, replace: replaced.length, same: unchanged.length },
    created,
    replaced,
    unchanged,
    backups,
    backupSha256Matches: backups.every((entry) => entry.verified),
    atomicReplacements: replaced.length,
    noDeletes: true,
  };
  writeReport(reportPaths.copy, report);
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, unchanged: undefined, backups: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reportPaths.copy), 'served 검증 전에 copy report가 필요함');
  assert.ok(!fs.existsSync(reportPaths.verify), '기존 served report를 덮어쓰지 않음');
  const server = inspectServer();
  const pending = [...manifest.routes];
  const results = [];
  await Promise.all(Array.from({ length: 6 }, async () => {
    while (pending.length > 0) {
      const route = pending.shift();
      const localBytes = fs.readFileSync(path.join(previewRoot, route.path));
      assert.equal(localBytes.length, route.bytes, `local preview byte 수 불일치: ${route.path}`);
      assert.equal(sha256(localBytes), route.sha256, `local preview hash 불일치: ${route.path}`);
      const relativeUrl = route.path.split('/').map(encodeURIComponent).join('/');
      const response = await fetch(new URL(`/spinon/${relativeUrl}`, origin), { cache: 'no-store' });
      const servedBytes = Buffer.from(await response.arrayBuffer());
      const servedSha256 = sha256(servedBytes);
      results.push({
        path: route.path,
        status: response.status,
        contentType: response.headers.get('content-type'),
        localBytes: localBytes.length,
        servedBytes: servedBytes.length,
        localSha256: route.sha256,
        servedSha256,
        matched: response.status === 200 && servedBytes.length === localBytes.length && servedSha256 === route.sha256,
      });
    }
  }));
  results.sort((a, b) => a.path.localeCompare(b.path));
  const mismatches = results.filter((item) => !item.matched);
  const backupReport = JSON.parse(fs.readFileSync(reportPaths.copy, 'utf8'));
  for (const item of backupReport.backups) {
    const backup = fs.readFileSync(path.join(backupRoot, item.path));
    assert.equal(backup.length, item.bytes, `backup bytes 불일치: ${item.path}`);
    assert.equal(sha256(backup), item.sha256, `backup hash 불일치: ${item.path}`);
  }
  const report = {
    sourceHead: manifest.sourceHead,
    sourceTree: manifest.sourceTree,
    selectedInputsSha256: manifest.selectedInputsSha256,
    tailscaleBackendState: server.backendState,
    previewServerVerified: true,
    previewPidUnchanged: server.serverPid,
    routeCount: results.length,
    matchedCount: results.length - mismatches.length,
    mismatchCount: mismatches.length,
    allVerified: mismatches.length === 0,
    backupsVerified: backupReport.backups.length,
    backupHashesMatch: true,
    noDeletes: true,
    routes: results,
  };
  writeReport(reportPaths.verify, report);
  process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
  assert.equal(mismatches.length, 0, `${mismatches.length} HTTPS served route mismatch`);
}
