import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const taskBuildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-4-zoom-2026-10-29');
const siteRoot = path.join(taskBuildRoot, 'selected-site');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-4-zoom-2026-10-29';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const localBase = 'http://127.0.0.1:8768/spinon/';
const reportPaths = {
  handles: path.join(evidenceDirectory, 'target-open-handles.json'),
  preflight: path.join(evidenceDirectory, 'target-preflight.json'),
  copy: path.join(evidenceDirectory, 'target-copy-log.json'),
  verify: path.join(evidenceDirectory, 'served-hash-check.json'),
};
for (const report of Object.values(reportPaths)) assert.ok(!fs.existsSync(report), `refusing to replace report: ${report}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const trackedDiff = execFileSync('git', ['diff', '--binary', '--', 'spec/public/roadmap.html'], { cwd: repositoryRoot });
const trackedDiffSha256 = sha256(trackedDiff);
const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend must be running');
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root must be the configured real directory');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
const listenerOutput = execFileSync('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN', '-Fpcn'], { encoding: 'utf8' });
const listenerLines = listenerOutput.split('\n').filter(Boolean);
const previewPid = Number(listenerLines.find((line) => line.startsWith('p'))?.slice(1));
assert.ok(Number.isInteger(previewPid) && previewPid > 0, 'expected the existing preview server on port 8768');
const localBefore = await fetch(new URL('roadmap.html', localBase));
const localBeforeBytes = Buffer.from(await localBefore.arrayBuffer());
assert.equal(localBefore.status, 200, 'existing local roadmap must be served before sync');

const routeMap = new Map();
function addRoute(sourcePath, route) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const source = path.join(repositoryRoot, sourcePath);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${sourcePath}`);
  const bytes = fs.readFileSync(source);
  const entry = { route, source, sourceRelative: path.relative(repositoryRoot, source).split(path.sep).join('/'), bytes, sha256: sha256(bytes) };
  const prior = routeMap.get(route);
  if (prior) {
    assert.equal(prior.sha256, entry.sha256, `conflicting content for route: ${route}`);
    return;
  }
  routeMap.set(route, entry);
}
function addTree(sourceRelative, routePrefix = '') {
  const source = path.join(repositoryRoot, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe source directory: ${sourceRelative}`);
  for (const entry of fs.readdirSync(source, { withFileTypes: true })) {
    const childSource = path.join(sourceRelative, entry.name);
    const childRoute = routePrefix ? `${routePrefix}/${entry.name}` : entry.name;
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected source: ${childSource}`);
    if (entry.isDirectory()) addTree(childSource, childRoute);
    else if (entry.isFile()) addRoute(childSource, childRoute);
    else throw new Error(`unsupported selected source entry: ${childSource}`);
  }
}
addTree(path.relative(repositoryRoot, siteRoot).split(path.sep).join('/'));
const roadmapSource = path.join(repositoryRoot, 'spec/public/roadmap.html');
assert.equal(routeMap.get('roadmap.html')?.sha256, sha256(fs.readFileSync(roadmapSource)), 'built roadmap route must match public source bytes');
addRoute('spec/public/roadmap.html', 'public/roadmap.html');
addRoute('spec/public/roadmap.html', 'spec/public/roadmap.html');
addTree('spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29', 'internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29');
addTree('spec/internal/evidence/c11-4-fit-content-zoom-runtime-2026-10-29', 'internal/evidence/c11-4-fit-content-zoom-runtime-2026-10-29');
const routes = [...routeMap.values()];
const targets = routes.map((entry) => {
  const target = path.resolve(previewRoot, entry.route);
  assert.ok(target.startsWith(`${previewRoot}${path.sep}`), `target escaped preview root: ${entry.route}`);
  let targetStat = null;
  try { targetStat = fs.lstatSync(target); }
  catch (error) { if (error.code !== 'ENOENT') throw error; }
  if (targetStat) assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `unsafe preview target: ${entry.route}`);
  return { ...entry, target, targetStat };
});
const changedExisting = targets.filter((item) => item.targetStat && sha256(fs.readFileSync(item.target)) !== item.sha256);
const existingTargets = targets.filter((item) => item.targetStat).map((item) => item.target);
const handleResult = existingTargets.length
  ? spawnSync('lsof', ['-nP', '--', ...existingTargets], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 })
  : { status: 1, stdout: '', stderr: '' };
assert.ok(handleResult.status === 0 || handleResult.status === 1, `lsof failed: ${handleResult.stderr}`);
const openTargets = new Map();
for (const line of handleResult.stdout.trim().split('\n').slice(1).filter(Boolean)) {
  const index = line.indexOf(previewRoot);
  if (index < 0) continue;
  const absolutePath = line.slice(index);
  const relative = path.relative(previewRoot, absolutePath).split(path.sep).join('/');
  const item = targets.find((candidate) => candidate.route === relative);
  assert.ok(item, `unexpected open preview target: ${absolutePath}`);
  openTargets.set(relative, { command: line.split(/\s+/)[0], path: relative, sha256: sha256(fs.readFileSync(item.target)) });
}
for (const item of changedExisting) {
  assert.ok(!openTargets.has(item.route), `refusing to replace changed open target: ${item.route}`);
}
const openReport = {
  previewRoot,
  listenerPid: previewPid,
  existingTargetCount: existingTargets.length,
  changedExistingTargetCount: changedExisting.length,
  openTargetCount: openTargets.size,
  openTargets: [...openTargets.values()],
  changedOpenTargets: [],
  result: 'no changed open target',
};
fs.writeFileSync(reportPaths.handles, `${JSON.stringify(openReport, null, 2)}\n`, { flag: 'wx' });
const preflight = {
  sourceHead,
  sourceTree,
  trackedRoadmapDiffSha256: trackedDiffSha256,
  tailscaleBackendState: tailscale.BackendState,
  selfHostName: tailscale.Self?.HostName ?? null,
  selfTailscaleIPs: tailscale.Self?.TailscaleIPs ?? [],
  previewPid,
  localPreviewUrl: `${localBase}roadmap.html`,
  localRoadmapBeforeStatus: localBefore.status,
  localRoadmapBeforeBytes: localBeforeBytes.length,
  localRoadmapBeforeSha256: sha256(localBeforeBytes),
  previewRoot,
  backupRoot,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes.length, 0),
  createdCount: targets.filter((item) => !item.targetStat).length,
  replacedCount: changedExisting.length,
  identicalCount: targets.length - changedExisting.length - targets.filter((item) => !item.targetStat).length,
  changedOpenTargetCount: 0,
  noDeletes: true,
  routes: targets.map((item) => ({
    path: item.route,
    source: item.sourceRelative,
    bytes: item.bytes.length,
    sourceSha256: item.sha256,
    targetExists: Boolean(item.targetStat),
    targetSha256: item.targetStat ? sha256(fs.readFileSync(item.target)) : null,
  })),
};
fs.writeFileSync(reportPaths.preflight, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });

const backupTaskRoot = path.dirname(backupRoot);
const backupParent = path.dirname(backupTaskRoot);
const backupParentStat = fs.lstatSync(backupParent);
assert.ok(backupParentStat.isDirectory() && !backupParentStat.isSymbolicLink(), `unsafe backup parent: ${backupParent}`);
try {
  const taskStat = fs.lstatSync(backupTaskRoot);
  assert.ok(taskStat.isDirectory() && !taskStat.isSymbolicLink(), `unsafe backup task directory: ${backupTaskRoot}`);
} catch (error) {
  if (error.code !== 'ENOENT') throw error;
  fs.mkdirSync(backupTaskRoot, { recursive: false });
}
fs.mkdirSync(backupRoot, { recursive: false });
const backups = [];
for (const item of changedExisting) {
  const backup = path.join(backupRoot, item.route);
  fs.mkdirSync(path.dirname(backup), { recursive: true });
  fs.copyFileSync(item.target, backup, fs.constants.COPYFILE_EXCL);
  const bytes = fs.readFileSync(backup);
  const targetBytes = fs.readFileSync(item.target);
  assert.equal(sha256(bytes), sha256(targetBytes), `backup hash mismatch: ${item.route}`);
  backups.push({ path: item.route, bytes: bytes.length, sha256: sha256(bytes) });
}
const created = [];
const replaced = [];
const identical = [];
for (const item of targets) {
  if (item.targetStat) {
    const currentBytes = fs.readFileSync(item.target);
    const currentHash = sha256(currentBytes);
    if (currentHash === item.sha256) {
      identical.push({ path: item.route, bytes: currentBytes.length, sha256: currentHash });
      continue;
    }
    const mode = item.targetStat.mode & 0o777;
    const temp = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temp, item.bytes, { flag: 'wx', mode });
    fs.chmodSync(temp, mode);
    fs.renameSync(temp, item.target);
    assert.equal(sha256(fs.readFileSync(item.target)), item.sha256, `replacement hash mismatch: ${item.route}`);
    replaced.push({ path: item.route, bytes: item.bytes.length, sha256: item.sha256 });
  } else {
    fs.mkdirSync(path.dirname(item.target), { recursive: true });
    const temp = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temp, item.bytes, { flag: 'wx' });
    fs.renameSync(temp, item.target);
    assert.equal(sha256(fs.readFileSync(item.target)), item.sha256, `creation hash mismatch: ${item.route}`);
    created.push({ path: item.route, bytes: item.bytes.length, sha256: item.sha256 });
  }
}
const copyReport = {
  sourceHead,
  sourceTree,
  trackedRoadmapDiffSha256: trackedDiffSha256,
  previewRoot,
  backupRoot,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes.length, 0),
  createdCount: created.length,
  replacedCount: replaced.length,
  identicalCount: identical.length,
  backupCount: backups.length,
  noDeletes: true,
  backups,
  created,
  replaced,
  identical,
};
fs.writeFileSync(reportPaths.copy, `${JSON.stringify(copyReport, null, 2)}\n`, { flag: 'wx' });

const verified = [];
for (const item of routes) {
  for (const base of [localBase, remoteBase]) {
    try {
      const response = await fetch(new URL(item.route, base));
      const bytes = Buffer.from(await response.arrayBuffer());
      const result = {
        base,
        path: item.route,
        status: response.status,
        bytes: bytes.length,
        sha256: sha256(bytes),
        contentType: response.headers.get('content-type'),
        hashMatches: sha256(bytes) === item.sha256,
      };
      verified.push(result);
    } catch (error) {
      verified.push({ base, path: item.route, status: null, bytes: null, sha256: null, hashMatches: false, error: String(error) });
    }
  }
}
const verifyReport = {
  sourceHead,
  sourceTree,
  previewRoot,
  localBase,
  remoteBase,
  verifiedResponseCount: verified.length,
  routeCount: routes.length,
  allHttp200: verified.every((item) => item.status === 200),
  allHashesMatch: verified.every((item) => item.hashMatches),
  serverRestarted: false,
  noDeletes: true,
  responses: verified,
};
fs.writeFileSync(reportPaths.verify, `${JSON.stringify(verifyReport, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  previewPid,
  routeCount: routes.length,
  totalBytes: copyReport.totalBytes,
  created: created.length,
  replaced: replaced.length,
  identical: identical.length,
  backedUp: backups.length,
  responses: verified.length,
  allHttp200: verifyReport.allHttp200,
  allHashesMatch: verifyReport.allHashesMatch,
  noDeletes: true,
}, null, 2)}\n`);
assert.ok(verifyReport.allHttp200 && verifyReport.allHashesMatch, 'Tailnet sync verification failed; reports and backups are preserved');
