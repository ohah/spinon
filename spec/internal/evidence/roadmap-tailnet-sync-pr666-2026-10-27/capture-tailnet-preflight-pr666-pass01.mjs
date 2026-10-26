import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const roadmapUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const build = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-build-pr666-pass02.json'), 'utf8'));
const selectedRoot = path.join(repositoryRoot, build.selectedOutputRoot);
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pr666-pass01.json');
const handlesPath = path.join(evidenceDirectory, 'target-open-handles-pr666-pass01.json');
const responsePath = '/tmp/spinon-roadmap-tailnet-preflight-pr666-20261027.html';
for (const candidate of [preflightPath, handlesPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const rootStat = fs.lstatSync(previewRoot);
assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'preview root is missing or unsafe');
const routes = build.selectedRoutes.map(({ path: route }) => route);
const missing = [];
const existingTargets = [];
for (const route of routes) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const target = path.join(previewRoot, route);
  try {
    const stat = fs.lstatSync(target);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `target is not a regular file: ${target}`);
    existingTargets.push(target);
  } catch (error) {
    if (error.code === 'ENOENT') missing.push(route);
    else throw error;
  }
}

const lsof = spawnSync('lsof', ['-nP', '--', ...existingTargets], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
assert.ok(lsof.status === 0 || lsof.status === 1, `lsof failed: ${lsof.stderr}`);
const handleLines = lsof.stdout.trim() ? lsof.stdout.trim().split('\n') : [];
const ps = spawnSync('ps', ['-p', '28040', '-o', 'pid=,ppid=,command='], { encoding: 'utf8' });
assert.equal(ps.status, 0, 'expected Tailnet preview server PID 28040 is not running');
assert.match(ps.stdout, /tailnet-preview-server\.mjs/);
const listen = spawnSync('lsof', ['-nP', '-a', '-p', '28040', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
assert.equal(listen.status, 0, 'preview PID is not listening on 127.0.0.1:8768');
assert.match(listen.stdout, /127\.0\.0\.1:8768/);

const tailscaleRaw = spawnSync('tailscale', ['status', '--json'], { encoding: 'utf8' });
assert.equal(tailscaleRaw.status, 0, tailscaleRaw.stderr);
const tailscale = JSON.parse(tailscaleRaw.stdout);
assert.equal(tailscale.BackendState, 'Running');
assert.equal(tailscale.Self.Online, true);

const response = spawnSync('curl', ['-sS', '-L', '--max-time', '15', '-o', responsePath, '-w', '%{http_code}', roadmapUrl], { encoding: 'utf8' });
assert.equal(response.status, 0, response.stderr);
const body = fs.readFileSync(responsePath);
const httpCode = response.stdout.trim();
assert.equal(httpCode, '200');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

const preflight = {
  sourceHead: build.sourceHead,
  sourceTree: build.sourceTree,
  previewRoot,
  previewServerPid: 28040,
  previewServerCommand: ps.stdout.trim(),
  previewListenSocket: listen.stdout.trim().split('\n').slice(1),
  tailscaleBackendState: tailscale.BackendState,
  tailscaleSelfOnline: tailscale.Self.Online,
  tailscaleSelfHostName: tailscale.Self.HostName,
  selectedRouteCount: routes.length,
  existingTargetCount: existingTargets.length,
  missingTargetCount: missing.length,
  targetFilesWithOpenHandles: Math.max(0, handleLines.length - 1),
  existingRoadmapUrl: roadmapUrl,
  existingRoadmapHttpStatus: Number(httpCode),
  existingRoadmapBytes: body.length,
  existingRoadmapSha256: sha256(body),
  backupRootPlanned: '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr666-2026-10-27/pass01',
  noPreviewTargetWritesPerformed: true,
  noDeletesPlanned: true,
};
const handles = {
  selectedRouteCount: routes.length,
  existingTargetCount: existingTargets.length,
  missingTargetCount: missing.length,
  missingTargets: missing,
  lsofExitCode: lsof.status,
  lsofOutput: lsof.stdout.trim(),
  noPreviewTargetWritesPerformed: true,
};
fs.writeFileSync(preflightPath, `${JSON.stringify(preflight, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(handlesPath, `${JSON.stringify(handles, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(preflight, null, 2)}\n`);
