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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1031-c11-area-fr-2026-11-02/pass01');
const publicationPath = path.join(evidence, 'route-publication-manifest.json');
const publicationRoot = path.join(repo, JSON.parse(fs.readFileSync(publicationPath, 'utf8')).publicationRoot);
const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const localBase = 'http://127.0.0.1:8768/spinon/';
const reports = {
  preflight: path.join(evidence, 'tailnet-preflight.json'),
  copy: path.join(evidence, 'tailnet-copy.json'),
  verify: path.join(evidence, 'tailnet-served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const [mode] = process.argv.slice(2);

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
}
function inspectOptional(root, route) {
  assert.ok(route && !route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  let current = root;
  const parts = route.split('/');
  for (const [index, part] of parts.entries()) {
    assert.ok(part && part !== '.' && part !== '..', `invalid route segment: ${route}`);
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
  assertDirectory(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root changed');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `expected one preview listener: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `unexpected preview process: ${args}`);
  return { backendState: tailscale.BackendState, dnsName: tailscale.Self?.DNSName, serverPid };
}
function targetMatches(entry) {
  const current = inspectOptional(previewRoot, entry.path);
  assert.equal(current.exists, entry.previousSha256 !== null, `target existence changed: ${entry.path}`);
  if (current.exists) {
    assert.equal(current.bytes.length, entry.previousBytes, `target size changed since staging: ${entry.path}`);
    assert.equal(sha256(current.bytes), entry.previousSha256, `target hash changed since staging: ${entry.path}`);
  }
  return current;
}
function openHandles(route) {
  const target = path.join(previewRoot, route);
  const result = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof failed: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean);
}
async function fetchHash(base, route) {
  const url = new URL(route.split('/').map(encodeURIComponent).join('/'), base);
  const response = await fetch(url, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(30_000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  return { status: response.status, bytes: bytes.length, sha256: sha256(bytes) };
}
function writeReport(file, report) {
  fs.writeFileSync(file, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node tailnet-sync.mjs preflight|copy|verify');
assert.equal(git(['branch', '--show-current']), 'main');
assert.equal(git(['rev-parse', 'HEAD']), publication.sourceHead, 'main changed after publication staging');
assert.equal(git(['status', '--porcelain', '--untracked-files=no']), '', 'tracked working tree must be clean');
assertDirectory(publicationRoot);
const server = inspectServer();
assert.equal(server.serverPid, publication.server.serverPid, 'Tailnet preview server changed since staging');
const changed = publication.routes.filter((entry) => entry.previousSha256 !== entry.sha256);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), 'preserving existing preflight report');
  assert.ok(!fs.existsSync(reports.copy) && !fs.existsSync(reports.verify), 'unexpected later-stage report exists');
  for (const entry of publication.routes) targetMatches(entry);
  const handles = changed.filter((entry) => entry.previousSha256 !== null)
    .map((entry) => ({ route: entry.path, pids: openHandles(entry.path) }))
    .filter((entry) => entry.pids.length > 0);
  assert.equal(handles.length, 0, `changed targets have open handles: ${JSON.stringify(handles.slice(0, 5))}`);
  const before = [];
  for (const route of ['roadmap.html']) {
    for (const [kind, base] of [['local', localBase], ['tailnet', remoteBase]]) {
      const response = await fetchHash(base, route);
      assert.equal(response.status, 200, `${kind} roadmap is unavailable before sync`);
      before.push({ kind, route, ...response });
    }
  }
  assert.equal(before[0].sha256, before[1].sha256, 'local and Tailnet roadmap differ before sync');
  writeReport(reports.preflight, {
    sourceHead: publication.sourceHead,
    selectedRouteCount: publication.routeCount,
    changedRouteCount: changed.length,
    changedExistingRouteCount: changed.filter((entry) => entry.previousSha256 !== null).length,
    newRouteCount: publication.newRouteCount,
    openChangedTargets: handles,
    noDeletes: true,
    roadmapBefore: before,
    server,
  });
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'preflight report required before copy');
  assert.ok(!fs.existsSync(reports.copy) && !fs.existsSync(reports.verify), 'preserving later-stage reports');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, publication.sourceHead);
  for (const entry of publication.routes) targetMatches(entry);
  fs.mkdirSync(path.dirname(backupRoot), { recursive: true });
  fs.mkdirSync(backupRoot, { recursive: false });
  const created = [];
  const replaced = [];
  const unchanged = [];
  for (const entry of publication.routes) {
    if (entry.previousSha256 === entry.sha256) {
      unchanged.push(entry.path);
      continue;
    }
    const sourceBytes = fs.readFileSync(path.join(publicationRoot, entry.path));
    assert.equal(sourceBytes.length, entry.bytes, `publication source size changed: ${entry.path}`);
    assert.equal(sha256(sourceBytes), entry.sha256, `publication source hash changed: ${entry.path}`);
    const targetPath = path.join(previewRoot, entry.path);
    const previous = entry.previousSha256 === null ? null : targetMatches(entry);
    if (previous) {
      const backup = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.copyFileSync(targetPath, backup, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backup)), entry.previousSha256, `backup hash mismatch: ${entry.path}`);
      replaced.push({ route: entry.path, previousSha256: entry.previousSha256, sourceSha256: entry.sha256 });
    } else {
      assert.ok(!inspectOptional(previewRoot, entry.path).exists, `new route appeared after preflight: ${entry.path}`);
      fs.mkdirSync(path.dirname(targetPath), { recursive: true });
      created.push({ route: entry.path, sourceSha256: entry.sha256 });
    }
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    const modeBits = previous ? previous.stat.mode & 0o777 : 0o644;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx', mode: modeBits });
    fs.chmodSync(temporary, modeBits);
    fs.renameSync(temporary, targetPath);
    const copied = fs.readFileSync(targetPath);
    assert.equal(sha256(copied), entry.sha256, `published target hash mismatch: ${entry.path}`);
  }
  writeReport(reports.copy, {
    sourceHead: publication.sourceHead,
    selectedRouteCount: publication.routeCount,
    changedRouteCount: changed.length,
    createdCount: created.length,
    replacedCount: replaced.length,
    unchangedCount: unchanged.length,
    backupCount: replaced.length,
    noDeletes: true,
    created,
    replaced,
    unchanged,
    server: inspectServer(),
  });
} else {
  assert.ok(fs.existsSync(reports.copy), 'copy report required before verify');
  assert.ok(!fs.existsSync(reports.verify), 'preserving existing verify report');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  assert.equal(copied.sourceHead, publication.sourceHead);
  const checked = [];
  for (const entry of publication.routes) {
    const target = inspectOptional(previewRoot, entry.path);
    assert.ok(target.exists, `published route missing: ${entry.path}`);
    assert.equal(target.bytes.length, entry.bytes, `published route size mismatch: ${entry.path}`);
    assert.equal(sha256(target.bytes), entry.sha256, `published route hash mismatch: ${entry.path}`);
    checked.push(`${entry.path}\0${entry.bytes}\0${entry.sha256}\n`);
  }
  const backups = [];
  for (const entry of copied.replaced) {
    const backup = inspectOptional(backupRoot, entry.route);
    assert.ok(backup.exists, `backup missing: ${entry.route}`);
    assert.equal(sha256(backup.bytes), entry.previousSha256, `backup hash mismatch: ${entry.route}`);
    backups.push(entry.route);
  }
  const http = [];
  for (const route of publication.requiredRoutes) {
    for (const [kind, base] of [['local', localBase], ['tailnet', remoteBase]]) {
      const response = await fetchHash(base, route);
      const expected = publication.routes.find((entry) => entry.path === route);
      assert.ok(expected, `required route absent from publication manifest: ${route}`);
      assert.equal(response.status, 200, `${kind} HTTP status for ${route}`);
      assert.equal(response.bytes, expected.bytes, `${kind} HTTP size mismatch: ${route}`);
      assert.equal(response.sha256, expected.sha256, `${kind} HTTP hash mismatch: ${route}`);
      http.push({ kind, route, ...response });
    }
  }
  writeReport(reports.verify, {
    sourceHead: publication.sourceHead,
    selectedRouteCount: publication.routeCount,
    selectedBytes: publication.totalBytes,
    selectedRouteHash: sha256(Buffer.from(checked.join(''))),
    publishedRouteCount: publication.routes.length,
    backupCount: backups.length,
    backups,
    noDeletes: true,
    requiredHttpRouteCount: publication.requiredRoutes.length,
    http,
    server: inspectServer(),
  });
}
