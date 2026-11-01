import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const source = path.join(repo, 'spec/public/roadmap.html');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const target = path.join(previewRoot, 'roadmap.html');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1002-2026-11-02/pass01');
const reportPaths = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const remoteUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
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

function pidsWithOpenFile(file) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', file], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof 검사 실패: ${file}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}

function localListener() {
  const result = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
  const pids = result.trim().split(/\s+/).filter(Boolean);
  assert.equal(pids.length, 1, '8768 listener는 하나여야 함');
  const pid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(pid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '기존 8768 listener가 예상된 Tailnet preview server가 아님');
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(pid), '-i'], { encoding: 'utf8' });
  assert.match(sockets, /127\.0\.0\.1:8768.*LISTEN/);
  return { pid, args };
}

async function fetchLocal() {
  const response = await fetch('http://127.0.0.1:8768/spinon/roadmap.html', { signal: AbortSignal.timeout(10000) });
  return { status: response.status, bytes: Buffer.from(await response.arrayBuffer()) };
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), '사용법: node sync-tailnet-roadmap.mjs preflight|copy|verify');
regularDirectory(previewRoot);
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root가 symlink 경로임');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reportPaths.preflight), '기존 preflight 증거를 보존함');
  assert.ok(!fs.existsSync(backupRoot), 'backup 경로가 이미 있으면 덮어쓰지 않음');
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  const listener = localListener();
  const sourceFile = regularFile(source);
  const targetFile = regularFile(target);
  const openPids = pidsWithOpenFile(target);
  assert.equal(openPids.length, 0, '교체할 roadmap.html이 현재 열린 상태라 중단함');
  const local = await fetchLocal();
  assert.equal(local.status, 200, 'local Tailnet preview가 HTTP 200이 아님');
  assert.equal(sha256(local.bytes), sha256(targetFile.bytes), '서버가 읽는 roadmap과 preflight target hash가 다름');
  const report = {
    sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(),
    trackedDiffSha256: sha256(execFileSync('git', ['diff', '--binary', 'HEAD'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 })),
    tailscaleBackend: tailscale.BackendState,
    previewServerPid: listener.pid,
    localStatus: local.status,
    localBytes: local.bytes.length,
    sourceBytes: sourceFile.bytes.length,
    previousBytes: targetFile.bytes.length,
    sourceSha256: sha256(sourceFile.bytes),
    previousSha256: sha256(targetFile.bytes),
    openHandlePids: openPids,
    backupRoot: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1002-2026-11-02/pass01',
    filesToReplace: ['roadmap.html'],
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reportPaths.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reportPaths.preflight), 'copy 전 preflight 필요');
  assert.ok(!fs.existsSync(reportPaths.copy), '기존 copy 증거를 보존함');
  assert.ok(!fs.existsSync(backupRoot), 'backup 경로를 덮어쓰지 않음');
  const prior = JSON.parse(fs.readFileSync(reportPaths.preflight, 'utf8'));
  const sourceFile = regularFile(source);
  const targetFile = regularFile(target);
  assert.equal(sha256(sourceFile.bytes), prior.sourceSha256, 'preflight 이후 source가 변경됨');
  assert.equal(sha256(targetFile.bytes), prior.previousSha256, 'preflight 이후 target이 변경됨');
  assert.equal(pidsWithOpenFile(target).length, 0, 'copy 직전 열린 target 거부');

  const backupParent = path.dirname(backupRoot);
  regularDirectory(path.dirname(backupParent));
  if (!fs.existsSync(backupParent)) fs.mkdirSync(backupParent, { recursive: false });
  regularDirectory(backupParent);
  fs.mkdirSync(backupRoot, { recursive: false });
  regularDirectory(backupRoot);
  const backupFile = path.join(backupRoot, 'roadmap.html');
  fs.copyFileSync(target, backupFile, fs.constants.COPYFILE_EXCL);
  assert.equal(sha256(fs.readFileSync(backupFile)), prior.previousSha256, 'backup hash 불일치');

  const temporary = `${target}.roadmap-sync-${process.pid}.tmp`;
  assert.ok(!fs.existsSync(temporary), '임시 파일이 이미 존재함');
  const modeBits = targetFile.stat.mode & 0o777;
  fs.writeFileSync(temporary, sourceFile.bytes, { flag: 'wx', mode: modeBits });
  fs.chmodSync(temporary, modeBits);
  fs.renameSync(temporary, target);
  assert.equal(sha256(fs.readFileSync(target)), prior.sourceSha256, 'atomic copy 후 hash 불일치');
  const report = {
    sourceSha256: prior.sourceSha256,
    previousSha256: prior.previousSha256,
    targetSha256: sha256(fs.readFileSync(target)),
    targetBytes: fs.statSync(target).size,
    backupPath: '~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1002-2026-11-02/pass01/roadmap.html',
    backupSha256: sha256(fs.readFileSync(backupFile)),
    atomicRename: true,
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reportPaths.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reportPaths.copy), 'verify 전 copy 필요');
  assert.ok(!fs.existsSync(reportPaths.verify), '기존 verify 증거를 보존함');
  const sourceFile = regularFile(source);
  const targetFile = regularFile(target);
  const copied = JSON.parse(fs.readFileSync(reportPaths.copy, 'utf8'));
  assert.equal(sha256(sourceFile.bytes), copied.sourceSha256, 'verify 시점 source hash 불일치');
  assert.equal(sha256(targetFile.bytes), copied.sourceSha256, 'preview target hash 불일치');
  const local = await fetchLocal();
  assert.equal(local.status, 200, 'local Tailnet preview가 HTTP 200이 아님');
  assert.equal(sha256(local.bytes), copied.sourceSha256, 'local HTTP served hash 불일치');
  const remote = await fetch(remoteUrl, { signal: AbortSignal.timeout(20000), cache: 'no-store' });
  const remoteBytes = Buffer.from(await remote.arrayBuffer());
  assert.equal(remote.status, 200, 'remote Tailnet preview가 HTTP 200이 아님');
  assert.equal(sha256(remoteBytes), copied.sourceSha256, 'remote HTTPS served hash 불일치');
  const report = {
    localUrl: 'http://127.0.0.1:8768/spinon/roadmap.html',
    localStatus: local.status,
    localBytes: local.bytes.length,
    localSha256: sha256(local.bytes),
    remoteUrl,
    remoteStatus: remote.status,
    remoteBytes: remoteBytes.length,
    remoteSha256: sha256(remoteBytes),
    sourceSha256: sha256(sourceFile.bytes),
    allHashesMatch: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reportPaths.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
