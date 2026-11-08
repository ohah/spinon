import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const localBase = 'http://127.0.0.1:8768/spinon/';
const reportPath = path.join(evidence, 'evidence-assets-publication-pass03.json');
const servedPath = path.join(evidence, 'evidence-assets-served-pass03.json');
const files = [
  'publication-preflight-pass03.json',
  'selected-publication-manifest-pass03.json',
  'selected-publication-result-pass03.json',
  'served-link-verification-pass03.json',
  'selected-route-audit-pass03.json',
  'selected-root-manifest-pass03.json',
  'stage-pass03.json',
  'agent-browser-lifecycle-final.json',
  'agent-browser-final-roadmap-desktop.json',
  'agent-browser-final-roadmap-mobile.json',
  'agent-browser-final-plan.json',
  'agent-browser-final-precomparison.json',
  'agent-browser-final-runtime.json',
  'agent-browser-final-c114-pr-review.json',
  'agent-browser-final-evidence.json',
  'agent-browser-final-status-review.json',
  'agent-browser-final-diff-review.json',
  'final-pass03-roadmap-desktop.png',
  'final-pass03-roadmap-mobile.png',
].map((name) => `${path.basename(evidence)}/${name}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const target of [reportPath, servedPath]) assert.ok(!fs.existsSync(target), `기존 evidence 보존: ${target}`);
const rootStat = fs.lstatSync(previewRoot);
assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'Preview root 안전성 확인 실패');
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'Preview root에 symlink가 없음');
const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale가 Running이어야 함');
const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
assert.equal(pids.length, 1, `Preview listener 개수 불일치: ${pids.join(',')}`);
const serverPid = Number(pids[0]);
const serverArgs = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
assert.ok(serverArgs.includes('tailnet-preview-server.mjs'), '예상한 Preview server가 아님');

function ensureDirectories(directory) {
  const relative = path.relative(previewRoot, directory);
  assert.ok(relative && !relative.startsWith('..') && !path.isAbsolute(relative), 'Preview 밖의 경로 거부');
  let current = previewRoot;
  for (const part of relative.split(path.sep)) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current);
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe route directory: ${current}`);
  }
}

const copied = [];
for (const route of files) {
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `bad route: ${route}`);
  const source = path.join(evidence, path.basename(route));
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source must be a regular file: ${source}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(previewRoot, route);
  assert.ok(!fs.existsSync(target), `target already exists; refusing overwrite: ${route}`);
  ensureDirectories(path.dirname(target));
  const temporary = path.join(path.dirname(target), `.${path.basename(target)}.evidence-${randomUUID()}.tmp`);
  const fd = fs.openSync(temporary, 'wx', 0o644);
  try { fs.writeFileSync(fd, bytes); fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
  fs.linkSync(temporary, target);
  fs.unlinkSync(temporary);
  const dirFd = fs.openSync(path.dirname(target), 'r');
  try { fs.fsyncSync(dirFd); } finally { fs.closeSync(dirFd); }
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `copied evidence hash mismatch: ${route}`);
  copied.push({ route, bytes: bytes.length, sha256: sha256(bytes), method: 'exclusive hard link after temp-file fsync' });
}

const publication = {
  files: copied.length,
  totalBytes: copied.reduce((sum, item) => sum + item.bytes, 0),
  routes: copied,
  tailscaleBackendState: tailscale.BackendState,
  previewServerPid: serverPid,
  serverRestarted: false,
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(publication, null, 2)}\n`, { flag: 'wx' });

const checks = [];
for (const base of [localBase, remoteBase]) {
  for (const item of copied) {
    const url = `${base}${encodeURI(item.route)}`;
    const response = await fetch(url, { cache: 'no-store', signal: AbortSignal.timeout(30000) });
    const bytes = Buffer.from(await response.arrayBuffer());
    assert.equal(response.status, 200, `${url} HTTP ${response.status}`);
    assert.equal(sha256(bytes), item.sha256, `${url} served hash mismatch`);
    checks.push({ url, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
  }
}
const served = {
  requests: checks.length,
  sourceAssetsPerBase: copied.length,
  allHttp200: true,
  allHashesMatch: true,
  checks,
};
fs.writeFileSync(servedPath, `${JSON.stringify(served, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...publication, routes: undefined, requests: served.requests, allHttp200: true, allHashesMatch: true }, null, 2)}\n`);
