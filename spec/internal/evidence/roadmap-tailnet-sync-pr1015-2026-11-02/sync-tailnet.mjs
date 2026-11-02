import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const build = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1015-20261102');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1015-2026-11-02/pass01');
const publication = JSON.parse(fs.readFileSync(path.join(evidence, 'route-publication-manifest.json'), 'utf8'));
const source = path.join(build, 'route-publication');
const privateRoot = path.join(build, 'private');
const routeListPath = path.join(privateRoot, 'preview-route-list.json');
const statePath = path.join(privateRoot, 'preflight-state.json');
const reports = {
  preflight: path.join(evidence, 'tailnet-preflight.json'),
  copy: path.join(evidence, 'tailnet-copy.json'),
  verify: path.join(evidence, 'tailnet-served-verification.json'),
};
const localBase = 'http://127.0.0.1:8768/spinon/';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const [mode] = process.argv.slice(2);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
}
function walk(directory, prefix = '') {
  const routes = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const route = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${route}`);
    if (entry.isDirectory()) routes.push(...walk(directory, route));
    else if (entry.isFile()) routes.push(route.split(path.sep).join('/'));
    else throw new Error(`unsupported route entry: ${route}`);
  }
  return routes;
}
function inspectOptional(root, route) {
  assert.ok(route && !route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  const parts = route.split('/');
  let current = root;
  for (const [index, part] of parts.entries()) {
    assert.ok(part && part !== '.' && part !== '..', `invalid route part: ${route}`);
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink rejected: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `route parent is not a directory: ${route}`);
      else {
        assert.ok(stat.isFile(), `route is not a regular file: ${route}`);
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
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend must be Running');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root changed');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `expected one preview listener, got ${pids.length}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), 'unexpected preview server process');
  return { backendState: tailscale.BackendState, dnsName: tailscale.Self?.DNSName, serverPid };
}
function routeHandles(routes) {
  const handles = [];
  for (const route of routes) {
    const target = path.join(previewRoot, route);
    const result = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
    assert.ok(result.status === 0 || result.status === 1, `lsof failed for ${route}: ${result.stderr}`);
    for (const pid of result.stdout.trim().split(/\s+/).filter(Boolean)) handles.push({ route, pid: Number(pid) });
  }
  return handles;
}
function assertSource() {
  assert.equal(publication.sourceHead, git(['rev-parse', 'HEAD']), 'source HEAD differs from staged publication');
  assert.equal(publication.sourceHead, git(['rev-parse', 'origin/main']), 'source must be merged origin/main');
  assertDirectory(source);
  const sourceRoutes = walk(source);
  assert.equal(sourceRoutes.length, publication.routes.length, 'source route count differs from manifest');
  const listed = new Map(publication.routes.map((entry) => [entry.path, entry]));
  for (const route of sourceRoutes) {
    const entry = listed.get(route);
    assert.ok(entry, `unlisted publication route: ${route}`);
    const bytes = fs.readFileSync(path.join(source, route));
    assert.equal(bytes.length, entry.bytes, `source size mismatch: ${route}`);
    assert.equal(sha256(bytes), entry.sha256, `source hash mismatch: ${route}`);
  }
}
function reportWrite(target, value) {
  fs.writeFileSync(target, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(value, null, 2)}\n`);
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
assertSource();
assertDirectory(previewRoot);
const server = inspectServer();
const byRoute = new Map(publication.routes.map((entry) => [entry.path, entry]));

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight) && !fs.existsSync(reports.copy) && !fs.existsSync(reports.verify), 'preserving existing reports');
  assert.ok(!fs.existsSync(privateRoot), 'preserving existing private baseline');
  fs.mkdirSync(privateRoot, { recursive: false });
  const routes = walk(previewRoot);
  fs.writeFileSync(routeListPath, `${JSON.stringify(routes, null, 2)}\n`, { flag: 'wx' });
  const changed = publication.routes.filter((entry) => entry.sha256 !== entry.previousSha256);
  for (const entry of changed) {
    const current = inspectOptional(previewRoot, entry.path);
    assert.equal(current.exists, entry.previousSha256 !== null, `route existence changed since staging: ${entry.path}`);
    if (current.exists) {
      assert.equal(current.bytes.length, entry.previousBytes, `route size changed since staging: ${entry.path}`);
      assert.equal(sha256(current.bytes), entry.previousSha256, `route hash changed since staging: ${entry.path}`);
    }
  }
  const handles = routeHandles(changed.filter((entry) => entry.previousSha256 !== null).map((entry) => entry.path));
  assert.equal(handles.length, 0, `changed routes are open: ${JSON.stringify(handles.slice(0, 5))}`);
  const local = await fetch(`${localBase}roadmap.html`, { headers: { 'cache-control': 'no-cache' } });
  const localBytes = Buffer.from(await local.arrayBuffer());
  const remote = await fetch(`${remoteBase}roadmap.html`, { headers: { 'cache-control': 'no-cache' } });
  const remoteBytes = Buffer.from(await remote.arrayBuffer());
  assert.equal(local.status, 200, 'local roadmap unavailable before sync');
  assert.equal(remote.status, 200, 'Tailnet roadmap unavailable before sync');
  assert.equal(sha256(localBytes), sha256(remoteBytes), 'local and Tailnet roadmap differ before sync');
  const state = { sourceHead: publication.sourceHead, routeCount: routes.length, routes, serverPid: server.serverPid };
  fs.writeFileSync(statePath, `${JSON.stringify(state)}\n`, { flag: 'wx' });
  reportWrite(reports.preflight, {
    sourceHead: publication.sourceHead,
    publicationRouteCount: publication.routeCount,
    changedRouteCount: changed.length,
    changedExistingRouteCount: changed.filter((entry) => entry.previousSha256 !== null).length,
    newRouteCount: changed.filter((entry) => entry.previousSha256 === null).length,
    unchangedRouteCount: publication.unchangedRouteCount,
    currentPreviewRouteCount: routes.length,
    changedOpenHandles: handles,
    existingRouteDeletes: 0,
    noDeletes: true,
    localRoadmapBefore: { status: local.status, bytes: localBytes.length, sha256: sha256(localBytes) },
    tailnetRoadmapBefore: { status: remote.status, bytes: remoteBytes.length, sha256: sha256(remoteBytes) },
    server,
  });
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight) && fs.existsSync(routeListPath) && fs.existsSync(statePath), 'preflight required');
  assert.ok(!fs.existsSync(reports.copy) && !fs.existsSync(reports.verify), 'preserving existing later reports');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  const state = JSON.parse(fs.readFileSync(statePath, 'utf8'));
  assert.equal(preflight.sourceHead, publication.sourceHead);
  assert.deepEqual(walk(previewRoot), state.routes, 'preview route set changed after preflight');
  assert.equal(inspectServer().serverPid, state.serverPid, 'preview server process changed after preflight');
  const changed = publication.routes.filter((entry) => entry.sha256 !== entry.previousSha256);
  const handles = routeHandles(changed.filter((entry) => entry.previousSha256 !== null).map((entry) => entry.path));
  assert.equal(handles.length, 0, `changed routes are open: ${JSON.stringify(handles.slice(0, 5))}`);
  const backupParent = path.dirname(backupRoot);
  fs.mkdirSync(backupParent, { recursive: true });
  assert.equal(fs.realpathSync(backupParent), backupParent, 'backup parent resolves through symlink');
  fs.mkdirSync(backupRoot, { recursive: false });
  const created = [];
  const replaced = [];
  for (const entry of changed) {
    const sourceBytes = fs.readFileSync(path.join(source, entry.path));
    assert.equal(sha256(sourceBytes), entry.sha256, `source changed during copy: ${entry.path}`);
    const target = path.join(previewRoot, entry.path);
    let modeBits = 0o644;
    if (entry.previousSha256 !== null) {
      const current = inspectOptional(previewRoot, entry.path);
      assert.ok(current.exists, `existing route disappeared: ${entry.path}`);
      assert.equal(current.bytes.length, entry.previousBytes, `route size changed before backup: ${entry.path}`);
      assert.equal(sha256(current.bytes), entry.previousSha256, `route hash changed before backup: ${entry.path}`);
      modeBits = current.stat.mode & 0o777;
      const backup = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), entry.previousSha256, `backup hash mismatch: ${entry.path}`);
      replaced.push({ path: entry.path, previousSha256: entry.previousSha256, sourceSha256: entry.sha256 });
    } else {
      assert.ok(!inspectOptional(previewRoot, entry.path).exists, `new route appeared after preflight: ${entry.path}`);
      fs.mkdirSync(path.dirname(target), { recursive: true });
      created.push({ path: entry.path, sourceSha256: entry.sha256 });
    }
    const temporary = `${target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx', mode: modeBits });
    fs.chmodSync(temporary, modeBits);
    fs.renameSync(temporary, target);
    assert.equal(sha256(fs.readFileSync(target)), entry.sha256, `published route hash mismatch: ${entry.path}`);
  }
  reportWrite(reports.copy, {
    sourceHead: publication.sourceHead,
    publicationRouteCount: publication.routeCount,
    createdCount: created.length,
    replacedCount: replaced.length,
    unchangedCount: publication.unchangedRouteCount,
    backupCount: replaced.length,
    noDeletes: true,
    created,
    replaced,
    server: inspectServer(),
  });
} else {
  assert.ok(fs.existsSync(reports.copy) && fs.existsSync(routeListPath) && fs.existsSync(statePath), 'copy and baseline required before verify');
  assert.ok(!fs.existsSync(reports.verify), 'preserving existing verify report');
  const state = JSON.parse(fs.readFileSync(statePath, 'utf8'));
  const currentRoutes = walk(previewRoot);
  const currentSet = new Set(currentRoutes);
  const deleted = state.routes.filter((route) => !currentSet.has(route));
  assert.equal(deleted.length, 0, `previous preview routes deleted: ${deleted.slice(0, 5).join(', ')}`);
  for (const entry of publication.routes) {
    const current = inspectOptional(previewRoot, entry.path);
    assert.ok(current.exists, `published route missing: ${entry.path}`);
    assert.equal(current.bytes.length, entry.bytes, `published route size mismatch: ${entry.path}`);
    assert.equal(sha256(current.bytes), entry.sha256, `published route hash mismatch: ${entry.path}`);
  }
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  const backupChecks = [];
  for (const entry of copied.replaced) {
    const backup = inspectOptional(backupRoot, entry.path);
    assert.ok(backup.exists, `backup missing: ${entry.path}`);
    assert.equal(sha256(backup.bytes), entry.previousSha256, `backup hash mismatch: ${entry.path}`);
    backupChecks.push(entry.path);
  }
  const http = [];
  for (const route of publication.requiredRoutes) {
    for (const [kind, base] of [['local', localBase], ['tailnet', remoteBase]]) {
      const url = new URL(route.split('/').map(encodeURIComponent).join('/'), base);
      const response = await fetch(url, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(30_000) });
      const bytes = Buffer.from(await response.arrayBuffer());
      assert.equal(response.status, 200, `${kind} HTTP status mismatch: ${route}`);
      const sourceBytes = fs.readFileSync(path.join(source, route));
      assert.equal(bytes.length, sourceBytes.length, `${kind} HTTP size mismatch: ${route}`);
      assert.equal(sha256(bytes), sha256(sourceBytes), `${kind} HTTP hash mismatch: ${route}`);
      http.push({ kind, route, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
    }
  }
  reportWrite(reports.verify, {
    sourceHead: publication.sourceHead,
    publicationRouteCount: publication.routeCount,
    currentPreviewRouteCount: currentRoutes.length,
    baselinePreviewRouteCount: state.routes.length,
    deletedBaselineRouteCount: deleted.length,
    noDeletes: true,
    selectedRouteHash: sha256(Buffer.from(publication.routes.map((entry) => `${entry.path}\0${entry.bytes}\0${entry.sha256}\n`).join(''))),
    backupCount: backupChecks.length,
    requiredHttpRouteCount: publication.requiredRoutes.length,
    http,
    server: inspectServer(),
  });
}
