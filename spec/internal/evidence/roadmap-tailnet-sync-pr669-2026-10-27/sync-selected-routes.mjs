import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr669-2026-10-27/pass01';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const build = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-build.json'), 'utf8'));
const targetsReport = path.join(evidenceDirectory, 'target-open-handles.json');
const preflightReport = path.join(evidenceDirectory, 'preflight.json');
const copyReportPath = path.join(evidenceDirectory, 'tailnet-copy.json');
const verifyReportPath = path.join(evidenceDirectory, 'tailnet-verify.json');
for (const candidate of [targetsReport, preflightReport, copyReportPath, verifyReportPath]) {
  assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);
}
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const status = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(status.BackendState, 'Running', 'Tailscale backend must be running');
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root must be the configured real directory');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);

const listenerOutput = execFileSync('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN', '-Fpcn'], { encoding: 'utf8' });
const listener = listenerOutput.split('\n').filter(Boolean);
const previewPid = Number(listener.find((line) => line.startsWith('p'))?.slice(1));
assert.ok(Number.isInteger(previewPid) && previewPid > 0, 'preview server must listen on port 8768');
const preflightResponse = await fetch('http://127.0.0.1:8768/spinon/roadmap.html');
const preflightBytes = Buffer.from(await preflightResponse.arrayBuffer());
assert.equal(preflightResponse.status, 200, 'existing local roadmap must be served before sync');
const preflight = {
  tailscaleBackendState: status.BackendState,
  selfHostName: status.Self?.HostName ?? null,
  selfTailscaleIPs: status.Self?.TailscaleIPs ?? [],
  previewPid,
  previewListen: '127.0.0.1:8768',
  previewRoot,
  localRoadmapUrl: 'http://127.0.0.1:8768/spinon/roadmap.html',
  localRoadmapStatus: preflightResponse.status,
  localRoadmapBytes: preflightBytes.length,
  localRoadmapSha256: sha256(preflightBytes),
  serverRestarted: false,
};
fs.writeFileSync(preflightReport, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });

const routes = build.routes;
const targets = routes.map((route) => {
  assert.ok(!route.path.startsWith('/') && route.path.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route.path}`);
  const source = path.join(repositoryRoot, build.stageRoot, route.path);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe staged source: ${route.path}`);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, route.bytes, `staged source size changed: ${route.path}`);
  assert.equal(sha256(bytes), route.sha256, `staged source hash changed: ${route.path}`);
  const target = path.join(previewRoot, route.path);
  let targetStat = null;
  try { targetStat = fs.lstatSync(target); }
  catch (error) { if (error.code !== 'ENOENT') throw error; }
  if (targetStat) assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `unsafe preview target: ${route.path}`);
  return { route, source, target, bytes, targetStat };
});

const existing = targets.filter((item) => item.targetStat).map((item) => item.target);
const handleResult = existing.length
  ? spawnSync('lsof', ['-nP', '--', ...existing], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 })
  : { status: 1, stdout: '', stderr: '' };
assert.ok(handleResult.status === 0 || handleResult.status === 1, `lsof failed: ${handleResult.stderr}`);
const openRoutes = new Map();
for (const line of handleResult.stdout.trim().split('\n').slice(1).filter(Boolean)) {
  const filePath = line.slice(line.indexOf(previewRoot));
  const relative = path.relative(previewRoot, filePath).split(path.sep).join('/');
  const item = targets.find((candidate) => candidate.route.path === relative);
  assert.ok(item, `unexpected open preview handle: ${filePath}`);
  openRoutes.set(relative, { command: line.split(/\s+/)[0], path: relative, currentSha256: sha256(fs.readFileSync(item.target)), nextSha256: item.route.sha256 });
}
for (const [relative, record] of openRoutes) {
  assert.equal(record.currentSha256, record.nextSha256, `refusing to overwrite open route with changed bytes: ${relative}`);
}
fs.writeFileSync(targetsReport, `${JSON.stringify({ existingRouteCount: existing.length, openHandleCount: openRoutes.size, openRoutes: [...openRoutes.values()], result: 'no changed open target' }, null, 2)}\n`, { flag: 'wx' });

fs.mkdirSync(backupRoot, { recursive: false });
const created = [];
const replaced = [];
const identical = [];
for (const item of targets) {
  const expectedHash = item.route.sha256;
  if (item.targetStat) {
    const currentBytes = fs.readFileSync(item.target);
    const currentHash = sha256(currentBytes);
    if (currentHash === expectedHash) {
      identical.push({ path: item.route.path, bytes: currentBytes.length, sha256: currentHash });
      continue;
    }
    assert.ok(!openRoutes.has(item.route.path), `refusing to overwrite open route: ${item.route.path}`);
    const backupPath = path.join(backupRoot, item.route.path);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    fs.copyFileSync(item.target, backupPath, fs.constants.COPYFILE_EXCL);
    const backupBytes = fs.readFileSync(backupPath);
    assert.equal(sha256(backupBytes), currentHash, `backup hash mismatch: ${item.route.path}`);
    const temp = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temp, item.bytes, { flag: 'wx', mode: item.targetStat.mode & 0o777 });
    fs.chmodSync(temp, item.targetStat.mode & 0o777);
    fs.renameSync(temp, item.target);
    assert.equal(sha256(fs.readFileSync(item.target)), expectedHash, `replacement hash mismatch: ${item.route.path}`);
    replaced.push({ path: item.route.path, bytes: item.bytes.length, sha256: expectedHash, backupPath: path.relative(backupRoot, backupPath), backupBytes: backupBytes.length, backupSha256: currentHash });
  } else {
    fs.mkdirSync(path.dirname(item.target), { recursive: true });
    const temp = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temp, item.bytes, { flag: 'wx' });
    fs.renameSync(temp, item.target);
    assert.equal(sha256(fs.readFileSync(item.target)), expectedHash, `created route hash mismatch: ${item.route.path}`);
    created.push({ path: item.route.path, bytes: item.bytes.length, sha256: expectedHash });
  }
}

const copyReport = {
  sourceHead: build.sourceHead,
  sourceTree: build.sourceTree,
  previewRoot,
  backupRoot,
  selectedRouteCount: routes.length,
  selectedBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  createdCount: created.length,
  replacedCount: replaced.length,
  identicalCount: identical.length,
  openHandleCount: openRoutes.size,
  noDeletes: true,
  created,
  replaced,
  identical,
};
fs.writeFileSync(copyReportPath, `${JSON.stringify(copyReport, null, 2)}\n`, { flag: 'wx' });

const verified = [];
for (const route of routes) {
  const response = await fetch(new URL(route.path, remoteBase));
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, `Tailnet response not 200: ${route.path}`);
  assert.equal(bytes.length, route.bytes, `Tailnet byte count mismatch: ${route.path}`);
  assert.equal(sha256(bytes), route.sha256, `Tailnet SHA-256 mismatch: ${route.path}`);
  verified.push({ path: route.path, status: response.status, bytes: bytes.length, sha256: route.sha256 });
}
const verifyReport = {
  sourceHead: build.sourceHead,
  previewRoot,
  remoteBase,
  routeCount: verified.length,
  verifiedBytes: verified.reduce((sum, item) => sum + item.bytes, 0),
  allHttp200: verified.every((item) => item.status === 200),
  allHashesMatch: true,
  serverRestarted: false,
  noDeletes: true,
  routes: verified,
};
fs.writeFileSync(verifyReportPath, `${JSON.stringify(verifyReport, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ preflight, copy: { selectedRouteCount: routes.length, created: created.length, replaced: replaced.length, identical: identical.length, openHandles: openRoutes.size, noDeletes: true }, verify: { routeCount: verified.length, allHttp200: verifyReport.allHttp200, allHashesMatch: verifyReport.allHashesMatch } }, null, 2)}\n`);
