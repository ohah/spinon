import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library', 'Application Support', 'SpinonPreview');
const backupRoot = path.join(os.homedir(), 'Library', 'Application Support', 'SpinonPreview-backups', 'roadmap-tailnet-sync-pr1021-2026-11-02');
const previewRootLabel = '~/Library/Application Support/SpinonPreview';
const backupRootLabel = '~/Library/Application Support/SpinonPreview-backups/roadmap-tailnet-sync-pr1021-2026-11-02';
const reportPaths = {
  manifest: path.join(evidence, 'route-publication-manifest.json'),
  preflight: path.join(evidence, 'tailnet-preflight.json'),
  copy: path.join(evidence, 'tailnet-copy.json'),
  verify: path.join(evidence, 'tailnet-served-verification.json'),
};
const routes = [
  { source: 'spec/public/roadmap.html', route: 'roadmap.html' },
  { source: 'spec/public/roadmap.html', route: 'public/roadmap.html' },
  { source: 'spec/public/roadmap.html', route: 'spec/public/roadmap.html' },
  { source: 'spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/android-api37-avd-percentage-overflow.png', route: 'internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/android-api37-avd-percentage-overflow.png' },
  { source: 'spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/ios-26_2-simulator-percentage-overflow.png', route: 'internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/ios-26_2-simulator-percentage-overflow.png' },
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
function writeJson(file, value) {
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
}
function validateRoute(route) {
  assert.ok(route && !route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid route segment: ${route}`);
}
function inspectOptional(root, route) {
  validateRoute(route);
  const parts = route.split('/');
  let current = root;
  for (const [index, part] of parts.entries()) {
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
  assert.ok(fs.existsSync(previewRoot), 'configured preview root is missing');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root resolves through an unexpected path');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `expected one preview listener: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `unexpected preview process: ${args}`);
  const dnsName = (tailscale.Self?.DNSName || '').replace(/\.$/, '');
  assert.ok(dnsName, 'Tailscale self DNS name is missing');
  return { backendState: tailscale.BackendState, selfDnsName: dnsName, serverPid, serverArgs: args };
}
function openHandles(routeList) {
  const open = [];
  for (const route of routeList) {
    const target = path.join(previewRoot, route);
    const result = spawnSync('lsof', ['-nP', '--', target], { encoding: 'utf8', maxBuffer: 1024 * 1024 });
    assert.ok(result.status === 0 || result.status === 1, `lsof failed for ${route}: ${result.stderr}`);
    const rows = result.stdout.trim().split('\n').slice(1).filter(Boolean);
    for (const row of rows) open.push({ route, row });
  }
  return open;
}
function loadManifest() {
  return JSON.parse(fs.readFileSync(reportPaths.manifest, 'utf8'));
}
function assertCurrentSource(manifest) {
  const head = git(['rev-parse', 'HEAD']);
  assert.equal(head, git(['rev-parse', 'origin/main']), 'selected source must be merged origin/main');
  assert.equal(head, manifest.sourceHead, 'source HEAD differs from the manifest');
  assert.equal(git(['status', '--porcelain', '--untracked-files=no']), '', 'tracked checkout must be clean');
}

const mode = process.argv[2];
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-selected-routes.mjs preflight|copy|verify');
const server = inspectServer();
const sourceHead = git(['rev-parse', 'HEAD']);
assert.equal(sourceHead, git(['rev-parse', 'origin/main']), 'source must be merged origin/main');
assert.equal(git(['status', '--porcelain', '--untracked-files=no']), '', 'tracked checkout must be clean');

if (mode === 'preflight') {
  for (const file of Object.values(reportPaths)) assert.ok(!fs.existsSync(file), `preserving existing report: ${file}`);
  assert.ok(!fs.existsSync(backupRoot), `preserving existing backup directory: ${backupRoot}`);
  const entries = routes.map(({ source, route }) => {
    validateRoute(route);
    const sourcePath = path.join(repo, source);
    const sourceStat = fs.lstatSync(sourcePath);
    assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source must be a regular file: ${source}`);
    const sourceBytes = fs.readFileSync(sourcePath);
    const target = inspectOptional(previewRoot, route);
    return {
      source,
      route,
      bytes: sourceBytes.length,
      sha256: sha256(sourceBytes),
      previousExists: target.exists,
      previousBytes: target.exists ? target.bytes.length : null,
      previousSha256: target.exists ? sha256(target.bytes) : null,
      previousMode: target.exists ? target.stat.mode & 0o777 : null,
    };
  });
  const changedExisting = entries.filter((entry) => entry.previousExists && entry.previousSha256 !== entry.sha256).map((entry) => entry.route);
  const handles = openHandles(changedExisting);
  assert.equal(handles.length, 0, `changed files are open: ${JSON.stringify(handles)}`);
  const manifest = {
    sourceHead,
    sourceTree: git(['rev-parse', 'HEAD^{tree}']),
    previewRoot: previewRootLabel,
    routeCount: entries.length,
    changedRouteCount: entries.filter((entry) => entry.previousSha256 !== entry.sha256).length,
    changedExistingRouteCount: changedExisting.length,
    newRouteCount: entries.filter((entry) => !entry.previousExists).length,
    noDeletes: true,
    routes: entries,
  };
  const preflight = {
    sourceHead,
    previewRoot: previewRootLabel,
    backupRoot: backupRootLabel,
    server,
    changedOpenHandles: handles,
    noDeletes: true,
    routeCount: entries.length,
    changedRouteCount: manifest.changedRouteCount,
    changedExistingRouteCount: manifest.changedExistingRouteCount,
    newRouteCount: manifest.newRouteCount,
    routes: entries.map(({ route, previousExists, previousBytes, previousSha256, sha256: nextSha256 }) => ({ route, previousExists, previousBytes, previousSha256, nextSha256 })),
  };
  writeJson(reportPaths.manifest, manifest);
  writeJson(reportPaths.preflight, preflight);
  process.stdout.write(`${JSON.stringify(preflight, null, 2)}\n`);
} else if (mode === 'copy') {
  const manifest = loadManifest();
  assertCurrentSource(manifest);
  assert.ok(fs.existsSync(reportPaths.preflight), 'preflight report required before copy');
  assert.ok(!fs.existsSync(reportPaths.copy) && !fs.existsSync(reportPaths.verify), 'preserving existing later-stage reports');
  const preflight = JSON.parse(fs.readFileSync(reportPaths.preflight, 'utf8'));
  assert.equal(preflight.server.serverPid, server.serverPid, 'preview server PID changed after preflight');
  const changed = manifest.routes.filter((entry) => entry.previousSha256 !== entry.sha256);
  const changedExisting = changed.filter((entry) => entry.previousExists).map((entry) => entry.route);
  const handles = openHandles(changedExisting);
  assert.equal(handles.length, 0, `changed files are open: ${JSON.stringify(handles)}`);
  for (const entry of manifest.routes) {
    const target = inspectOptional(previewRoot, entry.route);
    assert.equal(target.exists, entry.previousExists, `target existence changed after preflight: ${entry.route}`);
    assert.equal(target.exists ? sha256(target.bytes) : null, entry.previousSha256, `target content changed after preflight: ${entry.route}`);
    const sourceBytes = fs.readFileSync(path.join(repo, entry.source));
    assert.equal(sha256(sourceBytes), entry.sha256, `source changed after preflight: ${entry.source}`);
  }
  const changedExistingEntries = changed.filter((entry) => entry.previousExists);
  if (changedExistingEntries.length > 0) {
    assert.ok(!fs.existsSync(backupRoot), `preserving existing backup directory: ${backupRoot}`);
    fs.mkdirSync(path.dirname(backupRoot), { recursive: true });
    fs.mkdirSync(backupRoot, { recursive: false });
  }
  const backups = [];
  for (const entry of changedExistingEntries) {
    const sourcePath = path.join(previewRoot, entry.route);
    const backupPath = path.join(backupRoot, entry.route);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    fs.copyFileSync(sourcePath, backupPath, fs.constants.COPYFILE_EXCL);
    assert.equal(sha256(fs.readFileSync(backupPath)), entry.previousSha256, `backup hash mismatch: ${entry.route}`);
    backups.push({ route: entry.route, previousSha256: entry.previousSha256, bytes: entry.previousBytes });
  }
  const temporaries = [];
  for (const entry of changed) {
    const sourceBytes = fs.readFileSync(path.join(repo, entry.source));
    const targetPath = path.join(previewRoot, entry.route);
    fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    const modeBits = entry.previousExists ? entry.previousMode : 0o644;
    const fd = fs.openSync(temporary, 'wx', modeBits);
    try {
      fs.writeFileSync(fd, sourceBytes);
      fs.fsyncSync(fd);
    } finally {
      fs.closeSync(fd);
    }
    temporaries.push({ entry, targetPath, temporary });
  }
  const created = [];
  const replaced = [];
  try {
    for (const { entry, targetPath, temporary } of temporaries) {
      const current = inspectOptional(previewRoot, entry.route);
      assert.equal(current.exists, entry.previousExists, `target existence changed before rename: ${entry.route}`);
      assert.equal(current.exists ? sha256(current.bytes) : null, entry.previousSha256, `target changed before rename: ${entry.route}`);
      fs.renameSync(temporary, targetPath);
      assert.equal(sha256(fs.readFileSync(targetPath)), entry.sha256, `published hash mismatch: ${entry.route}`);
      if (entry.previousExists) replaced.push({ route: entry.route, previousSha256: entry.previousSha256, sourceSha256: entry.sha256 });
      else created.push({ route: entry.route, sourceSha256: entry.sha256 });
    }
  } finally {
    for (const { temporary } of temporaries) if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
  }
  const identical = manifest.routes.filter((entry) => entry.previousSha256 === entry.sha256).map((entry) => entry.route);
  const report = {
    sourceHead: manifest.sourceHead,
    previewRoot: previewRootLabel,
    backupRoot: changedExistingEntries.length ? backupRootLabel : null,
    routeCount: manifest.routeCount,
    selectedBytes: manifest.routes.reduce((sum, entry) => sum + entry.bytes, 0),
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    noDeletes: true,
    backups,
    created,
    replaced,
    identical,
    server,
  };
  writeJson(reportPaths.copy, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  const manifest = loadManifest();
  assertCurrentSource(manifest);
  assert.ok(fs.existsSync(reportPaths.copy), 'copy report required before served verification');
  const host = server.selfDnsName;
  const checks = [];
  for (const entry of manifest.routes) {
    const expected = fs.readFileSync(path.join(repo, entry.source));
    const expectedHash = sha256(expected);
    assert.equal(expectedHash, entry.sha256, `source hash changed: ${entry.source}`);
    for (const [kind, base] of [['local', 'http://127.0.0.1:8768'], ['tailnet', `https://${host}`]]) {
      const url = `${base}/spinon/${entry.route}?source_sha256=${entry.sha256}`;
      const response = await fetch(url, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(20000) });
      const bytes = Buffer.from(await response.arrayBuffer());
      assert.equal(response.status, 200, `${kind} route failed: ${entry.route}`);
      assert.equal(bytes.length, entry.bytes, `${kind} size mismatch: ${entry.route}`);
      assert.equal(sha256(bytes), expectedHash, `${kind} hash mismatch: ${entry.route}`);
      checks.push({ kind, route: entry.route, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
    }
  }
  const roadmap = checks.find((item) => item.kind === 'tailnet' && item.route === 'roadmap.html');
  assert.ok(roadmap, 'Tailnet roadmap route was not checked');
  const roadmapBytes = await (await fetch(`https://${host}/spinon/roadmap.html?card-check=${manifest.sourceHead}`, { headers: { 'cache-control': 'no-cache' } })).text();
  assert.ok(roadmapBytes.includes('PR #1019 Rebase and merge'), 'served roadmap lacks the C11.1 percentage card');
  assert.ok(roadmapBytes.includes('72/213'), 'served roadmap lacks the official completion count');
  const report = {
    sourceHead: manifest.sourceHead,
    tailnetHost: host,
    previewRoot: previewRootLabel,
    noDeletes: true,
    checkedRouteCount: checks.length,
    uniqueRouteCount: manifest.routeCount,
    requiredCardMarkers: ['PR #1019 Rebase and merge', '8e549ed55ac6f7e9611a4f7e1e571fca3a1e5684', '72/213'],
    checks,
  };
  writeJson(reportPaths.verify, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
