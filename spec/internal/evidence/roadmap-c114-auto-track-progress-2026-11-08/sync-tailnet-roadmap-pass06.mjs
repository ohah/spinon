import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-c114-auto-track-progress-2026-11-08/pass06';
const sourcePath = path.join(repo, 'spec/public/roadmap.html');
const targetPath = path.join(previewRoot, 'roadmap.html');
const preflightPath = path.join(evidence, 'preflight-pass06.json');
const copyPath = path.join(evidence, 'copy-and-backup-pass06.json');
const [mode] = process.argv.slice(2);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertNoSymlinkPath(target) {
  const absolute = path.resolve(target);
  const parsed = path.parse(absolute);
  let current = parsed.root;
  for (const part of absolute.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `symlink 경로 거부: ${current}`);
  }
}

function inspectServer() {
  const status = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(status.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assert.equal(status.Self?.DNSName, 'macstudio.tailed42f2.ts.net.', '기대한 Tailnet preview host인지 확인');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, '미리보기 root는 지정된 실제 디렉터리여야 함');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const pid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(pid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  const sockets = execFileSync('lsof', ['-nP', '-a', '-p', String(pid), '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
  assert.ok(sockets.some((row) => row.includes('127.0.0.1:8768') && row.includes('(LISTEN)')), 'preview는 localhost:8768에서 listen해야 함');
  return { status: status.BackendState, host: status.Self.DNSName, pid, args, sockets };
}

function targetOpenHandles() {
  const result = spawnSync('lsof', ['-nP', '--', targetPath], { encoding: 'utf8', maxBuffer: 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split('\n').slice(1).filter(Boolean);
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet-roadmap.mjs preflight|copy|verify');
assertNoSymlinkPath(path.dirname(sourcePath));
assertNoSymlinkPath(sourcePath);
assertNoSymlinkPath(previewRoot);
assertNoSymlinkPath(path.dirname(targetPath));
const sourceBytes = fs.readFileSync(sourcePath);
const sourceText = sourceBytes.toString('utf8');
for (const marker of ['현재 진행 · C11.4 Grid track sizing', '2026-11-08 C11.4 parsed Root HTML', 'PR #1272 Rebase and merge', '공식 <code>78/214</code>']) {
  assert.ok(sourceText.includes(marker), `source progress marker 누락: ${marker}`);
}
const targetStat = fs.lstatSync(targetPath);
assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), 'roadmap target가 일반 파일이어야 함');
const currentTargetBytes = fs.readFileSync(targetPath);
const currentTargetSha256 = sha256(currentTargetBytes);
const sourceSha256 = sha256(sourceBytes);
const server = inspectServer();
const openHandles = targetOpenHandles();

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(preflightPath), '기존 preflight evidence를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(copyPath), 'copy 전용 evidence 경로가 비어 있어야 함');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup 경로를 덮어쓰지 않음: ${backupRoot}`);
  assert.equal(openHandles.length, 0, `변경할 target open handle 거부: ${openHandles.join('\n')}`);
  const report = {
    sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceBranch: execFileSync('git', ['branch', '--show-current'], { cwd: repo, encoding: 'utf8' }).trim(),
    sourceBytes: sourceBytes.length,
    sourceSha256,
    targetPath,
    targetBytes: currentTargetBytes.length,
    targetSha256: currentTargetSha256,
    previewRoot,
    backupRoot,
    tailscaleBackend: server.status,
    tailnetHost: server.host,
    previewPid: server.pid,
    previewArgs: server.args,
    previewSockets: server.sockets,
    targetOpenHandles: openHandles,
    updateCount: 1,
    deletions: 0,
  };
  fs.writeFileSync(preflightPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(preflightPath), 'copy 전에 preflight를 실행해야 함');
  assert.ok(!fs.existsSync(copyPath), '기존 copy evidence를 덮어쓰지 않음');
  const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
  assert.equal(preflight.sourceSha256, sourceSha256, 'preflight 이후 source가 바뀜');
  assert.equal(preflight.targetSha256, currentTargetSha256, 'preflight 이후 target가 바뀜');
  assert.equal(openHandles.length, 0, `복사 직전 target open handle 거부: ${openHandles.join('\n')}`);
  assert.ok(!fs.existsSync(backupRoot), `기존 backup 경로를 덮어쓰지 않음: ${backupRoot}`);
  fs.mkdirSync(backupRoot, { recursive: true });
  assertNoSymlinkPath(backupRoot);
  const backupPath = path.join(backupRoot, 'roadmap.html');
  fs.writeFileSync(backupPath, currentTargetBytes, { flag: 'wx' });
  const backupBytes = fs.readFileSync(backupPath);
  assert.equal(sha256(backupBytes), currentTargetSha256, 'backup hash 불일치');
  const temporaryPath = `${targetPath}.sync-${randomUUID()}`;
  fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(temporaryPath)), sourceSha256, 'temporary copy hash 불일치');
  fs.renameSync(temporaryPath, targetPath);
  const publishedBytes = fs.readFileSync(targetPath);
  assert.equal(sha256(publishedBytes), sourceSha256, 'published roadmap hash 불일치');
  const report = {
    sourceHead: preflight.sourceHead,
    sourceBranch: preflight.sourceBranch,
    route: '/spinon/roadmap.html',
    sourceSha256,
    previousSha256: currentTargetSha256,
    publishedSha256: sha256(publishedBytes),
    backupPath,
    backupSha256: sha256(backupBytes),
    serverPidPreserved: server.pid === preflight.previewPid,
    serverPid: server.pid,
    updateCount: 1,
    deletions: 0,
    serverRestarted: false,
  };
  fs.writeFileSync(copyPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(copyPath), 'verify 전에 copy를 실행해야 함');
  const copy = JSON.parse(fs.readFileSync(copyPath, 'utf8'));
  assert.equal(copy.sourceSha256, sourceSha256, '현재 source와 copy 기록이 다름');
  assert.equal(copy.publishedSha256, sha256(fs.readFileSync(targetPath)), '현재 preview hash가 copy 기록과 다름');
  assert.ok(sourceText.includes('현재 진행 · C11.4'), '현재 progress source marker 누락');
  process.stdout.write(`${JSON.stringify({ route: '/spinon/roadmap.html', sourceSha256, previewSha256: copy.publishedSha256, backupSha256: copy.backupSha256, sourceAndPreviewMatch: true, serverPid: server.pid, deletions: 0 }, null, 2)}\n`);
}
