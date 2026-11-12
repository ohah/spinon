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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1372-history-correction-2026-11-12');
const publicationRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1372-2026-11-12/publication');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const routes = ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'];
const routeBytes = fs.readFileSync(path.join(repo, 'spec/public/roadmap.html'));
const routeHash = createHash('sha256').update(routeBytes).digest('hex');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const routeSet = new Set(routes);
const serverPid = Number(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim());
const serverArgs = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
const oldRootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const oldPublicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));

assert.ok(serverPid > 0 && serverArgs.includes('tailnet-preview-server.mjs'), '예상된 Tailnet preview server가 아님');
const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
assert.ok(!fs.existsSync(backupRoot), '기존 correction backup 보존');
fs.mkdirSync(backupRoot, { recursive: false });

const backups = [];
for (const route of routes) {
  const target = path.join(previewRoot, route);
  const publicationTarget = path.join(publicationRoot, route);
  for (const file of [target, publicationTarget]) {
    const stat = fs.lstatSync(file);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'route가 symlink 또는 일반 파일이 아님: ' + route);
  }
  const lsof = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
  assert.ok(lsof.status === 0 || lsof.status === 1, 'lsof 대상 검사가 실패함: ' + route);
  assert.equal(lsof.stdout.trim(), '', '열린 roadmap route는 교체하지 않음: ' + route);
  const previous = fs.readFileSync(target);
  const backup = path.join(backupRoot, route);
  fs.mkdirSync(path.dirname(backup), { recursive: true });
  fs.writeFileSync(backup, previous, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(backup)), sha256(previous), 'correction backup hash 불일치: ' + route);

  const temporary = target + '.roadmap-correction-' + randomUUID() + '.tmp';
  const mode = fs.statSync(target).mode & 0o777;
  fs.writeFileSync(temporary, routeBytes, { flag: 'wx', mode });
  fs.chmodSync(temporary, mode);
  fs.renameSync(temporary, target);
  assert.equal(sha256(fs.readFileSync(target)), routeHash, 'corrected roadmap hash 불일치: ' + route);

  const staged = path.join(publicationRoot, route);
  const stagedMode = fs.statSync(staged).mode & 0o777;
  const stagedTemp = staged + '.roadmap-correction-' + randomUUID() + '.tmp';
  fs.writeFileSync(stagedTemp, routeBytes, { flag: 'wx', mode: stagedMode });
  fs.chmodSync(stagedTemp, stagedMode);
  fs.renameSync(stagedTemp, staged);
  backups.push({ route, previousSha256: sha256(previous), sha256: routeHash, bytes: routeBytes.length });
}

for (const item of oldRootManifest.files.filter((file) => routeSet.has(file.route))) {
  item.bytes = routeBytes.length;
  item.sha256 = routeHash;
}
oldRootManifest.trackedDiffSha256 = sha256(execFileSync('git', ['diff', '--binary', 'HEAD'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 }));
oldRootManifest.files.forEach((file) => {
  if (routeSet.has(file.route)) {
    const staged = path.join(repo, oldRootManifest.selectedRoot, file.route);
    const stagedMode = fs.statSync(staged).mode & 0o777;
    const temp = staged + '.roadmap-correction-' + randomUUID() + '.tmp';
    fs.writeFileSync(temp, routeBytes, { flag: 'wx', mode: stagedMode });
    fs.chmodSync(temp, stagedMode);
    fs.renameSync(temp, staged);
  }
});
fs.writeFileSync(path.join(evidence, 'selected-root.json'), JSON.stringify(oldRootManifest, null, 2) + '\n');
for (const item of oldPublicationManifest.routes.filter((route) => routeSet.has(route.path))) {
  item.bytes = routeBytes.length;
  item.sha256 = routeHash;
}
oldPublicationManifest.totalBytes = oldPublicationManifest.routes.reduce((sum, item) => sum + item.bytes, 0);
oldPublicationManifest.trackedDiffSha256 = oldRootManifest.trackedDiffSha256;
fs.writeFileSync(path.join(evidence, 'selected-publication.json'), JSON.stringify(oldPublicationManifest, null, 2) + '\n');

const localResponse = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
const localBytes = Buffer.from(await localResponse.arrayBuffer());
assert.equal(localResponse.status, 200, 'corrected local roadmap HTTP status');
assert.equal(sha256(localBytes), routeHash, 'corrected local roadmap hash');
const remoteResponse = await fetch(new URL('roadmap.html', remoteBase), { signal: AbortSignal.timeout(20000) });
const remoteBytes = Buffer.from(await remoteResponse.arrayBuffer());
assert.equal(remoteResponse.status, 200, 'corrected HTTPS roadmap HTTP status');
assert.equal(sha256(remoteBytes), routeHash, 'corrected HTTPS roadmap hash');
const historicalRoutes = [];
for (const route of [
  'internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-runtime-2026-11-12/README.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-runtime-2026-11-12/merge-PR1369.html',
]) {
  const response = await fetch(new URL(route.split('/').map(encodeURIComponent).join('/'), remoteBase), { signal: AbortSignal.timeout(20000) });
  assert.equal(response.status, 200, '기존 #1369 history route 응답: ' + route);
  historicalRoutes.push({ route, httpStatus: response.status });
}
const report = {
  correction: '복원한 #1369 역사 항목과 기존 evidence 링크를 보존',
  sourceHead: oldRootManifest.sourceHead,
  trackedDiffSha256: oldRootManifest.trackedDiffSha256,
  serverPid,
  previewServerRestarted: false,
  routes: backups,
  backupLocation: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1372-history-correction-2026-11-12',
  localRoadmap: { httpStatus: localResponse.status, bytes: localBytes.length, sha256: routeHash },
  httpsRoadmap: { httpStatus: remoteResponse.status, bytes: remoteBytes.length, sha256: routeHash },
  historical1369Routes: historicalRoutes,
  noDeletes: true,
};
fs.writeFileSync(path.join(evidence, 'roadmap-correction.json'), JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, routes: undefined }, null, 2) + '\n');
