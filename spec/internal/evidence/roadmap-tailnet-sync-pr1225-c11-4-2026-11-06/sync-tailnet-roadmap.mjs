import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const sourceRelative = 'spec/public/roadmap.html';
const source = path.join(repo, sourceRelative);
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const target = path.join(previewRoot, 'roadmap.html');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1225-c11-4-2026-11-06/pass01');
const remoteUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function regularFile(file) {
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `일반 파일만 허용: ${file}`);
  return { bytes: fs.readFileSync(file), stat };
}

function regularDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `일반 디렉터리만 허용: ${directory}`);
}

function openPids(file) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', file], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}

function previewServer() {
  const output = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
  const pids = output.trim().split(/\s+/).filter(Boolean);
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const pid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(pid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview server: ${args}`);
  const cwd = execFileSync('lsof', ['-a', '-p', String(pid), '-d', 'cwd', '-Fn'], { encoding: 'utf8' })
    .split('\n').find((line) => line.startsWith('n'))?.slice(1);
  assert.equal(cwd, previewRoot, `preview root 불일치: ${cwd}`);
  return { pid, args, cwd };
}

async function fetchBytes(url) {
  const response = await fetch(url, { signal: AbortSignal.timeout(20000), cache: 'no-store' });
  return { status: response.status, bytes: Buffer.from(await response.arrayBuffer()) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), '사용법: node sync-tailnet-roadmap.mjs preflight|copy|verify');
regularDirectory(previewRoot);
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink 경로임');

if (mode === 'preflight') {
  for (const file of [...Object.values(reports), backupRoot]) assert.ok(!fs.existsSync(file), `기존 근거를 덮어쓰지 않음: ${file}`);
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  const server = previewServer();
  const current = regularFile(source);
  const currentHtml = current.bytes.toString('utf8');
  for (const marker of ['PR #1224', '4039ed60327b09128503559fcc45268acbb9637e', '78/214']) {
    assert.ok(currentHtml.includes(marker), `현재 roadmap source marker 누락: ${marker}`);
  }
  const previous = regularFile(target);
  const handles = openPids(target);
  assert.equal(handles.length, 0, '교체 대상 roadmap.html이 열린 상태임');
  const local = await fetchBytes('http://127.0.0.1:8768/spinon/roadmap.html');
  const remote = await fetchBytes(remoteUrl);
  assert.equal(local.status, 200, 'local preview가 HTTP 200이 아님');
  assert.equal(remote.status, 200, 'Tailnet preview가 HTTP 200이 아님');
  assert.equal(sha256(local.bytes), sha256(previous.bytes), 'local server target hash 불일치');
  assert.equal(sha256(remote.bytes), sha256(previous.bytes), 'Tailnet server target hash 불일치');
  const report = {
    sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(),
    source: { path: sourceRelative, bytes: current.bytes.length, sha256: sha256(current.bytes) },
    sourceMarkers: ['PR #1224', '4039ed60327b09128503559fcc45268acbb9637e', '78/214'],
    previewRoot,
    target: { path: target, bytes: previous.bytes.length, sha256: sha256(previous.bytes), mode: previous.stat.mode & 0o777 },
    backupRoot,
    tailscaleBackendState: tailscale.BackendState,
    previewServer: server,
    openHandlePids: handles,
    local: { status: local.status, bytes: local.bytes.length, sha256: sha256(local.bytes) },
    tailnet: { url: remoteUrl, status: remote.status, bytes: remote.bytes.length, sha256: sha256(remote.bytes) },
    filesToReplace: ['roadmap.html'],
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight 필요');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy 근거를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(backupRoot), '기존 backup을 덮어쓰지 않음');
  const previous = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  const sourceFile = regularFile(source);
  const targetFile = regularFile(target);
  assert.equal(sha256(sourceFile.bytes), previous.source.sha256, 'preflight 후 source 변경');
  assert.equal(sha256(targetFile.bytes), previous.target.sha256, 'preflight 후 target 변경');
  assert.equal(openPids(target).length, 0, 'copy 직전 열린 target 거부');

  const backupParent = path.dirname(backupRoot);
  fs.mkdirSync(backupParent, { recursive: true });
  regularDirectory(backupParent);
  fs.mkdirSync(backupRoot, { recursive: false });
  regularDirectory(backupRoot);
  const backup = path.join(backupRoot, 'roadmap.html');
  fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
  assert.equal(sha256(fs.readFileSync(backup)), previous.target.sha256, 'backup hash 불일치');

  const temporary = `${target}.roadmap-sync-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, sourceFile.bytes, { flag: 'wx', mode: targetFile.stat.mode & 0o777 });
  fs.chmodSync(temporary, targetFile.stat.mode & 0o777);
  fs.renameSync(temporary, target);
  const published = regularFile(target);
  assert.equal(sha256(published.bytes), previous.source.sha256, 'atomic replace 후 hash 불일치');
  const report = {
    source: previous.source,
    target: { path: target, bytes: published.bytes.length, sha256: sha256(published.bytes) },
    previousTarget: previous.target,
    backup: { path: backup, bytes: fs.statSync(backup).size, sha256: sha256(fs.readFileSync(backup)), byteIdenticalToPrevious: true },
    method: 'same-directory temporary file and atomic rename',
    openHandlePids: [],
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy 필요');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify 근거를 덮어쓰지 않음');
  const sourceFile = regularFile(source);
  const targetFile = regularFile(target);
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  assert.equal(sha256(sourceFile.bytes), copied.source.sha256, 'verify source hash 불일치');
  assert.equal(sha256(targetFile.bytes), copied.source.sha256, 'preview target hash 불일치');
  const local = await fetchBytes('http://127.0.0.1:8768/spinon/roadmap.html');
  const remote = await fetchBytes(remoteUrl);
  assert.equal(local.status, 200, 'local preview가 HTTP 200이 아님');
  assert.equal(remote.status, 200, 'Tailnet preview가 HTTP 200이 아님');
  assert.equal(sha256(local.bytes), copied.source.sha256, 'local HTTP hash 불일치');
  assert.equal(sha256(remote.bytes), copied.source.sha256, 'Tailnet HTTPS hash 불일치');
  const markers = ['PR #1224', '4039ed60327b09128503559fcc45268acbb9637e', '78/214'];
  for (const marker of markers) {
    assert.ok(local.bytes.toString('utf8').includes(marker), `local served marker 누락: ${marker}`);
    assert.ok(remote.bytes.toString('utf8').includes(marker), `Tailnet served marker 누락: ${marker}`);
  }
  const report = {
    source: { path: sourceRelative, bytes: sourceFile.bytes.length, sha256: sha256(sourceFile.bytes) },
    local: { url: 'http://127.0.0.1:8768/spinon/roadmap.html', status: local.status, bytes: local.bytes.length, sha256: sha256(local.bytes) },
    tailnet: { url: remoteUrl, status: remote.status, bytes: remote.bytes.length, sha256: sha256(remote.bytes) },
    markers,
    byteIdentical: true,
    backupHashVerified: sha256(fs.readFileSync(copied.backup.path)) === copied.backup.sha256,
    serverRestarted: false,
  };
  assert.ok(report.backupHashVerified, 'backup hash 재검증 실패');
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
