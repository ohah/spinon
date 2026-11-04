import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const route = 'roadmap.html';
const source = path.join(repo, 'spec/public/roadmap.html');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const target = path.join(previewRoot, route);
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/c11-root-html-aspect-ratio-roadmap-tailnet-sync-2026-11-05');
const reportPaths = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
}
function readRegularFile(file) {
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${file}`);
  return { bytes: fs.readFileSync(file), stat };
}
function openPids(file) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', file], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, 'lsof failed to inspect target');
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}
function writeReport(name, value) {
  fs.writeFileSync(reportPaths[name], `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
}
async function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend must be Running');
  const pids = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' })
    .trim().split(/\s+/).filter(Boolean);
  assert.equal(pids.length, 1, 'expected one listener on preview port 8768');
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), 'preview listener is not the expected server');
  const response = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
  const body = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, 'local preview route is not HTTP 200');
  return { serverPid, localStatus: response.status, localBytes: body.length, localSha256: sha256(body) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-roadmap.mjs preflight|copy|verify');
assertDirectory(previewRoot);
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root must not be an alias');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reportPaths.preflight), 'preserve existing preflight report');
  assert.ok(!fs.existsSync(backupRoot), 'refuse to reuse backup path');
  const sourceBytes = fs.readFileSync(source);
  const targetFile = readRegularFile(target);
  const pids = openPids(target);
  assert.equal(pids.length, 0, 'refuse to replace a file with an open handle');
  const server = await checkServer();
  assert.equal(server.localSha256, sha256(targetFile.bytes), 'local route differs from target file');
  const report = {
    sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceBranch: execFileSync('git', ['branch', '--show-current'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceRoute: 'spec/public/roadmap.html',
    targetRoute: route,
    sourceBytes: sourceBytes.length,
    sourceSha256: sha256(sourceBytes),
    targetBytes: targetFile.bytes.length,
    targetSha256: sha256(targetFile.bytes),
    changedTargetOpenHandleCount: pids.length,
    tailscaleBackendRunning: true,
    previewServerPid: server.serverPid,
    localStatus: server.localStatus,
    localSha256: server.localSha256,
    backupLocation: '~/Library/Application Support/SpinonPreviewBackups/c11-root-html-aspect-ratio-roadmap-tailnet-sync-2026-11-05',
    noDeletes: true,
  };
  writeReport('preflight', report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reportPaths.preflight), 'preflight must run before copy');
  assert.ok(!fs.existsSync(reportPaths.copy), 'preserve existing copy report');
  const preflight = JSON.parse(fs.readFileSync(reportPaths.preflight, 'utf8'));
  const sourceBytes = fs.readFileSync(source);
  const targetFile = readRegularFile(target);
  assert.equal(sha256(sourceBytes), preflight.sourceSha256, 'source changed after preflight');
  assert.equal(sha256(targetFile.bytes), preflight.targetSha256, 'target changed after preflight');
  assert.equal(openPids(target).length, 0, 'refuse to replace a file with an open handle');
  fs.mkdirSync(backupRoot, { recursive: false });
  const backup = path.join(backupRoot, route);
  fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
  assert.equal(sha256(fs.readFileSync(backup)), preflight.targetSha256, 'backup hash mismatch');
  const temporary = `${target}.sync-${randomUUID()}.tmp`;
  const modeBits = targetFile.stat.mode & 0o777;
  fs.writeFileSync(temporary, sourceBytes, { flag: 'wx', mode: modeBits });
  fs.chmodSync(temporary, modeBits);
  fs.renameSync(temporary, target);
  const installed = readRegularFile(target);
  assert.equal(sha256(installed.bytes), preflight.sourceSha256, 'installed route hash mismatch');
  const report = {
    sourceHead: preflight.sourceHead,
    sourceSha256: preflight.sourceSha256,
    targetRoute: route,
    previousSha256: preflight.targetSha256,
    installedSha256: sha256(installed.bytes),
    backupPath: backup,
    backupSha256: sha256(fs.readFileSync(backup)),
    previewServerPid: preflight.previewServerPid,
    atomicRename: true,
    noDeletes: true,
  };
  writeReport('copy', report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reportPaths.copy), 'copy must run before verify');
  assert.ok(!fs.existsSync(reportPaths.verify), 'preserve existing verify report');
  const copied = JSON.parse(fs.readFileSync(reportPaths.copy, 'utf8'));
  const installed = readRegularFile(target);
  assert.equal(sha256(installed.bytes), copied.sourceSha256, 'target file hash mismatch');
  const server = await checkServer();
  assert.equal(server.localSha256, copied.sourceSha256, 'served local route hash mismatch');
  const report = {
    sourceHead: copied.sourceHead,
    targetRoute: route,
    targetBytes: installed.bytes.length,
    targetSha256: sha256(installed.bytes),
    localStatus: server.localStatus,
    localSha256: server.localSha256,
    previewServerPid: server.serverPid,
    verified: true,
  };
  writeReport('verify', report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
