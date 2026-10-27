import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const publicationPath = path.join(evidenceDirectory, 'selected-publication.json');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const copyPath = path.join(evidenceDirectory, 'tailnet-copy.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-verify.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr718-2026-10-28/pass01';
const origin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
const copy = JSON.parse(fs.readFileSync(copyPath, 'utf8'));
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);
assert.equal(copy.sourceHead, publication.sourceHead);
assert.equal(copy.routeCount, publication.routeCount);
assert.equal(copy.noRoutesDeleted, true);

const requiredRoutes = publication.requiredRoutes;
for (const route of requiredRoutes) assert.ok(publication.routes.some((entry) => entry.path === route), `required route not in publication: ${route}`);

const responses = [];
for (let index = 0; index < publication.routes.length; index += 1) {
  const entry = publication.routes[index];
  const local = fs.readFileSync(path.join(targetRoot, entry.path));
  assert.equal(local.length, entry.bytes, `local size mismatch: ${entry.path}`);
  assert.equal(sha256(local), entry.sha256, `local hash mismatch: ${entry.path}`);
  const route = entry.path.split('/').map(encodeURIComponent).join('/');
  const response = await fetch(`${origin}${route}?verify=${Date.now()}-${index}`, { cache: 'no-store' });
  assert.equal(response.status, 200, `HTTPS status for ${entry.path}: ${response.status}`);
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(bytes.length, entry.bytes, `HTTPS size mismatch: ${entry.path}`);
  assert.equal(sha256(bytes), entry.sha256, `HTTPS hash mismatch: ${entry.path}`);
  responses.push({ path: entry.path, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
}

async function readRoute(route) {
  const response = await fetch(`${origin}${route.split('/').map(encodeURIComponent).join('/')}?marker=${Date.now()}`, { cache: 'no-store' });
  assert.equal(response.status, 200, `marker route status: ${route}`);
  return response.text();
}
const roadmap = await readRoute('roadmap.html');
for (const marker of ['PR #717 Rebase and merge', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213', 'C11.4']) assert.ok(roadmap.includes(marker), `served roadmap marker missing: ${marker}`);
const status = await readRoute('STATUS.html');
for (const marker of ['github.com/ohah/spinon/pull/717', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213']) assert.ok(status.includes(marker), `served STATUS marker missing: ${marker}`);
const plan = await readRoute('plan/c11-4-flex-stage-oracle.html');
for (const marker of ['github.com/ohah/spinon/pull/717', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213']) assert.ok(plan.includes(marker), `served plan marker missing: ${marker}`);

const backupManifestPath = path.join(backupRoot, 'backup-manifest.json');
const backupManifest = JSON.parse(fs.readFileSync(backupManifestPath, 'utf8'));
assert.equal(backupManifest.fileCount, copy.replaced);
let verifiedBackupBytes = 0;
for (const entry of backupManifest.files) {
  const bytes = fs.readFileSync(path.join(backupRoot, entry.path));
  assert.equal(bytes.length, entry.bytes, `backup size mismatch: ${entry.path}`);
  assert.equal(sha256(bytes), entry.sha256, `backup hash mismatch: ${entry.path}`);
  verifiedBackupBytes += bytes.length;
}
assert.equal(verifiedBackupBytes, backupManifest.totalBytes);

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
assert.equal(tailscale.BackendState, 'Running');
assert.equal(tailscale.Self?.Online, true);
const listen = spawnSync('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
assert.equal(listen.status, 0, 'preview server stopped listening');
const beforePid = preflight.serverListen.match(/\nnode\s+(\d+)/)?.[1];
const afterPid = listen.stdout.match(/\nnode\s+(\d+)/)?.[1];
assert.ok(beforePid && afterPid && beforePid === afterPid, `preview server PID changed: ${beforePid} -> ${afterPid}`);

const report = {
  sourceHead: publication.sourceHead,
  sourceTree: publication.sourceTree,
  targetRoot,
  backupRoot,
  routeCount: responses.length,
  totalBytes: responses.reduce((sum, entry) => sum + entry.bytes, 0),
  allHttpsResponses200: responses.every((entry) => entry.status === 200),
  allLocalAndHttpsHashesMatch: true,
  roadmapMarkers: ['PR #717 Rebase and merge', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213', 'C11.4'],
  requiredRoutes,
  backupFileCount: backupManifest.fileCount,
  backupBytes: verifiedBackupBytes,
  backupsByteExact: true,
  tailscaleBackend: tailscale.BackendState,
  selfOnline: tailscale.Self.Online,
  previewServerPidBefore: beforePid,
  previewServerPidAfter: afterPid,
  noServerRestart: true,
  noRoutesDeleted: true,
  fullDocsLinkValidation: false,
  appRuntimeVerified: false,
  responses,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, responses: undefined }, null, 2)}\n`);
