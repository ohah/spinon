import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1153-2026-11-04');
const route = 'roadmap.html';
const targetPath = path.join(previewRoot, route);
const backupPath = path.join(backupRoot, route);
const preflightPath = path.join(evidenceDirectory, 'preflight.json');
const copyReportPath = path.join(evidenceDirectory, 'copy.json');
const verifyReportPath = path.join(evidenceDirectory, 'verify.json');
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const [mode, preflightArgument] = process.argv.slice(2);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-roadmap.mjs preflight|copy|verify [preflight.json]');

function readRegularFile(filePath, label) {
  const stat = fs.lstatSync(filePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `${label} must be a regular file: ${filePath}`);
  return fs.readFileSync(filePath);
}

function readOptionalTarget(filePath) {
  try {
    return { exists: true, bytes: readRegularFile(filePath, 'target'), stat: fs.lstatSync(filePath) };
  } catch (error) {
    if (error.code === 'ENOENT') return { exists: false, bytes: null, stat: null };
    throw error;
  }
}

function assertDirectory(directoryPath, label) {
  const stat = fs.lstatSync(directoryPath);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `${label} must be a real directory: ${directoryPath}`);
}

function writeNewJson(filePath, value) {
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
}

function openHandles(filename) {
  const result = spawnSync('lsof', ['-nP', '--', filename], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, 'lsof could not inspect target');
  const lines = result.stdout.trim().split('\n');
  return lines.slice(1).filter(Boolean).map((line) => {
    const fields = line.trim().split(/\s+/);
    assert.ok(fields.length >= 9, `failed to parse lsof row: ${line}`);
    return {
      command: fields[0],
      pid: Number(fields[1]),
      fd: fields[3],
      type: fields[4],
      inode: fields[7],
      name: fields.slice(8).join(' '),
    };
  });
}

function assertReadOnlyServerHandles(handles, serverPid, filename, stat) {
  for (const handle of handles) {
    assert.equal(handle.pid, serverPid, `unexpected process has target open: ${filename}`);
    assert.equal(handle.command, 'node', `unexpected target command: ${filename}`);
    assert.match(handle.fd, /^\d+r$/, `target handle is not read-only: ${filename}`);
    assert.equal(handle.type, 'REG', `target is not an ordinary file: ${filename}`);
    assert.equal(handle.inode, String(stat.ino), `handle inode does not match current target: ${filename}`);
  }
  return handles;
}

async function checkServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend must be Running');
  assert.ok(tailscale.Self?.DNSName?.startsWith('macstudio.tailed42f2.ts.net'), 'unexpected Tailscale host');
  assertDirectory(previewRoot, 'preview root');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root must not be a symlink or alias');
  const pidOutput = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim();
  const pids = [...new Set(pidOutput.split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, 'expected exactly one preview listener on port 8768');
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), 'listener is not the expected preview server');
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(serverPid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1);
  assert.equal(sockets.filter((line) => line.includes('127.0.0.1:8768') && line.includes('(LISTEN)')).length, 1, 'preview bind address or port mismatch');
  const localResponse = await fetch(`http://127.0.0.1:8768/spinon/${route}?sync=pr1153-preflight`, { signal: AbortSignal.timeout(10000) });
  assert.equal(localResponse.status, 200, 'local preview must return HTTP 200');
  const localBytes = Buffer.from(await localResponse.arrayBuffer());
  const origin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
  assert.ok(origin, 'SPINON_PREVIEW_ORIGIN must be set');
  const originUrl = new URL(origin);
  assert.equal(originUrl.protocol, 'https:', 'remote verification requires HTTPS');
  assert.equal(originUrl.hostname, 'macstudio.tailed42f2.ts.net', 'unexpected Tailnet host');
  const remoteResponse = await fetch(`${originUrl.origin}/spinon/${route}?sync=pr1153-preflight`, {
    headers: { 'cache-control': 'no-cache' },
    signal: AbortSignal.timeout(15000),
  });
  assert.equal(remoteResponse.status, 200, 'Tailnet preview must return HTTP 200');
  const remoteBytes = Buffer.from(await remoteResponse.arrayBuffer());
  return {
    tailscaleBackend: tailscale.BackendState,
    tailscaleHost: tailscale.Self.DNSName,
    serverPid,
    serverArgs: args,
    listenAddress: '127.0.0.1:8768',
    localStatus: localResponse.status,
    localBytes: localBytes.byteLength,
    localSha256: sha256(localBytes),
    remoteStatus: remoteResponse.status,
    remoteBytes: remoteBytes.byteLength,
    remoteSha256: sha256(remoteBytes),
  };
}

const sourceBytes = readRegularFile(sourcePath, 'roadmap source');
const sourceHash = sha256(sourceBytes);
const sourceCommit = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mergedSourceBytes = execFileSync('git', ['show', `${sourceCommit}:spec/public/roadmap.html`], { cwd: repositoryRoot });
assert.deepEqual(sourceBytes, mergedSourceBytes, 'refusing to publish source that differs from origin/main');
const sourceText = sourceBytes.toString('utf8');
assert.equal(sourceText.split('PR #1152 Rebase and merge').length - 1, 1, 'expected exactly one PR #1152 roadmap card');
assert.ok(sourceText.includes('72/213'), 'official completion count missing from roadmap');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(preflightPath), 'refusing to overwrite preflight.json');
  assert.ok(!fs.existsSync(copyReportPath) && !fs.existsSync(verifyReportPath), 'sync reports already exist');
  assertDirectory(previewRoot, 'preview root');
  assertDirectory(path.dirname(backupRoot), 'backup parent');
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  const server = await checkServer();
  const previous = readOptionalTarget(targetPath);
  const handles = previous.exists
    ? assertReadOnlyServerHandles(openHandles(targetPath), server.serverPid, targetPath, previous.stat)
    : [];
  const report = {
    mode: 'preflight',
    source_commit: sourceCommit,
    route,
    source_bytes: sourceBytes.byteLength,
    source_sha256: sourceHash,
    preview_root: previewRoot,
    target_exists: previous.exists,
    target_bytes: previous.bytes?.byteLength ?? null,
    target_sha256: previous.bytes ? sha256(previous.bytes) : null,
    target_inode: previous.stat?.ino ?? null,
    target_open_handles: handles,
    backup_root: backupRoot,
    server,
    no_deletes: true,
  };
  writeNewJson(preflightPath, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(preflightArgument, `${mode} requires the preflight JSON path`);
const preflight = JSON.parse(readRegularFile(path.resolve(evidenceDirectory, preflightArgument), 'preflight report').toString('utf8'));
assert.equal(preflight.mode, 'preflight', 'preflight mode mismatch');
assert.equal(preflight.source_commit, sourceCommit, 'source commit changed after preflight');
assert.equal(preflight.source_bytes, sourceBytes.byteLength, 'source size changed after preflight');
assert.equal(preflight.source_sha256, sourceHash, 'source hash changed after preflight');

if (mode === 'copy') {
  assert.ok(!fs.existsSync(copyReportPath), 'refusing to overwrite copy.json');
  assert.ok(!fs.existsSync(verifyReportPath), 'verify.json already exists');
  const server = await checkServer();
  assertDirectory(previewRoot, 'preview root');
  assertDirectory(path.dirname(backupRoot), 'backup parent');
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  const previous = readOptionalTarget(targetPath);
  assert.equal(previous.exists, preflight.target_exists, 'preview target existence changed after preflight');
  assert.equal(previous.bytes?.byteLength ?? null, preflight.target_bytes, 'preview target size changed after preflight');
  assert.equal(previous.bytes ? sha256(previous.bytes) : null, preflight.target_sha256, 'preview target hash changed after preflight');
  if (previous.exists) {
    assertReadOnlyServerHandles(openHandles(targetPath), server.serverPid, targetPath, previous.stat);
    fs.mkdirSync(backupRoot, { recursive: false });
    assertDirectory(backupRoot, 'backup root');
    fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
    assert.equal(sha256(readRegularFile(backupPath, 'backup')), preflight.target_sha256, 'backup hash mismatch');
  }
  const temporaryPath = `${targetPath}.roadmap-sync-${randomUUID()}.tmp`;
  fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
  try {
    fs.renameSync(temporaryPath, targetPath);
  } catch (error) {
    if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
    throw error;
  }
  const copiedBytes = readRegularFile(targetPath, 'copied roadmap');
  assert.equal(copiedBytes.byteLength, sourceBytes.byteLength, 'copied roadmap size mismatch');
  assert.equal(sha256(copiedBytes), sourceHash, 'copied roadmap hash mismatch');
  const report = {
    mode: 'copy',
    source_commit: sourceCommit,
    route,
    source_bytes: sourceBytes.byteLength,
    source_sha256: sourceHash,
    target_bytes: copiedBytes.byteLength,
    target_sha256: sha256(copiedBytes),
    backup_created: previous.exists,
    backup_sha256: previous.exists ? sha256(readRegularFile(backupPath, 'backup')) : null,
    server,
    no_deletes: true,
  };
  writeNewJson(copyReportPath, report);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  process.exit(0);
}

assert.ok(!fs.existsSync(verifyReportPath), 'refusing to overwrite verify.json');
assert.ok(fs.existsSync(copyReportPath), 'copy.json must exist before remote verification');
const copyReport = JSON.parse(readRegularFile(copyReportPath, 'copy report').toString('utf8'));
assert.equal(copyReport.source_sha256, sourceHash, 'copy report source hash mismatch');
assert.equal(copyReport.target_sha256, sourceHash, 'copy report target hash mismatch');
assertDirectory(previewRoot, 'preview root');
assertDirectory(path.dirname(backupRoot), 'backup parent');
const localBytes = readRegularFile(targetPath, 'preview roadmap');
assert.equal(localBytes.byteLength, sourceBytes.byteLength, 'local preview size mismatch');
assert.equal(sha256(localBytes), sourceHash, 'local preview hash mismatch');
if (preflight.target_exists) {
  const backupBytes = readRegularFile(backupPath, 'backup');
  assert.equal(backupBytes.byteLength, preflight.target_bytes, 'backup size mismatch');
  assert.equal(sha256(backupBytes), preflight.target_sha256, 'backup hash mismatch');
}
const server = await checkServer();
const verifyOrigin = process.env.SPINON_PREVIEW_ORIGIN?.replace(/\/$/, '');
const verifyUrl = new URL(verifyOrigin);
const response = await fetch(`${verifyUrl.origin}/spinon/${route}?sync=pr1153-final-${sourceHash.slice(0, 12)}`, {
  headers: { 'cache-control': 'no-cache' },
  signal: AbortSignal.timeout(15000),
});
assert.equal(response.status, 200, `Tailnet HTTP status: ${response.status}`);
const remoteBytes = Buffer.from(await response.arrayBuffer());
assert.equal(remoteBytes.byteLength, sourceBytes.byteLength, 'Tailnet response size mismatch');
assert.equal(sha256(remoteBytes), sourceHash, 'Tailnet served hash mismatch');
const report = {
  mode: 'verify',
  source_commit: sourceCommit,
  route,
  status: response.status,
  source_bytes: sourceBytes.byteLength,
  source_sha256: sourceHash,
  local_bytes: localBytes.byteLength,
  local_sha256: sha256(localBytes),
  remote_bytes: remoteBytes.byteLength,
  remote_sha256: sha256(remoteBytes),
  local_hash_matches: true,
  remote_http_200_and_hash_matches: true,
  server,
  no_deletes: true,
};
writeNewJson(verifyReportPath, report);
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
