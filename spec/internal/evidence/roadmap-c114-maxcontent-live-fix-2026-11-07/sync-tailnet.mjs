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
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-c114-maxcontent-live-fix-2026-11-07');
const backup = path.join(backupRoot, 'roadmap.html');
const remote = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const reports = {
  preflight: path.join(evidence, 'preflight.json'),
  copy: path.join(evidence, 'copy-and-backup.json'),
  verify: path.join(evidence, 'served-verification.json'),
};
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [mode] = process.argv.slice(2);

function ensureDirectory(directory) {
  if (fs.existsSync(directory)) {
    const stat = fs.lstatSync(directory);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
    return;
  }
  ensureDirectory(path.dirname(directory));
  fs.mkdirSync(directory);
}

function serverState() {
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, 'Tailnet preview listener가 정확히 하나여야 함');
  const pid = pids[0];
  const args = execFileSync('ps', ['-p', pid, '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), '예상한 Tailnet preview server가 아님');
  return { pid: Number(pid), args };
}

function fetchBytes(url) {
  const result = spawnSync('curl', ['-k', '--fail', '--silent', '--show-error', '--max-time', '20', url], { encoding: null, maxBuffer: 20 * 1024 * 1024 });
  assert.equal(result.status, 0, `curl 실패: ${result.stderr?.toString() ?? ''}`);
  return result.stdout;
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: node sync-tailnet.mjs preflight|copy|verify');
const sourceBytes = fs.readFileSync(source);
const server = serverState();

if (mode === 'preflight') {
  for (const report of Object.values(reports)) assert.ok(!fs.existsSync(report), `기존 근거 보존: ${path.basename(report)}`);
  assert.ok(!fs.existsSync(backupRoot), '기존 backup 경로 보존');
  const existing = fs.readFileSync(target);
  const localServed = fetchBytes('http://127.0.0.1:8768/spinon/roadmap.html');
  const remoteServed = fetchBytes(remote);
  assert.equal(hash(existing), hash(localServed), 'preview 파일과 local HTTP 제공본 불일치');
  assert.equal(hash(localServed), hash(remoteServed), 'local과 Tailnet 제공본 불일치');
  const report = {
    previewServerPid: server.pid,
    target: '~/Library/Application Support/SpinonPreview/roadmap.html',
    remote,
    sourceBytes: sourceBytes.length,
    sourceSha256: hash(sourceBytes),
    priorBytes: existing.length,
    priorSha256: hash(existing),
    localServedStatus: 200,
    localServedSha256: hash(localServed),
    remoteServedStatus: 200,
    remoteServedSha256: hash(remoteServed),
    backup: '~/Library/Application Support/SpinonPreviewBackups/roadmap-c114-maxcontent-live-fix-2026-11-07/roadmap.html',
    changedRoutes: ['roadmap.html'],
    deletedRoutes: [],
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight가 필요함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy 근거 보존');
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  const current = fs.readFileSync(target);
  assert.equal(hash(current), preflight.priorSha256, 'preflight 이후 preview target 변경됨');
  assert.equal(hash(fs.readFileSync(source)), preflight.sourceSha256, 'preflight 이후 source 변경됨');
  ensureDirectory(backupRoot);
  fs.copyFileSync(target, backup, fs.constants.COPYFILE_EXCL);
  assert.equal(hash(fs.readFileSync(backup)), preflight.priorSha256, 'backup hash 불일치');
  const temporary = `${target}.roadmap-sync-${process.pid}.tmp`;
  fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
  fs.renameSync(temporary, target);
  assert.equal(hash(fs.readFileSync(target)), preflight.sourceSha256, 'copied preview hash 불일치');
  const report = {
    previewServerPid: server.pid,
    changedRoutes: ['roadmap.html'],
    createdRoutes: [],
    replacedRoutes: ['roadmap.html'],
    identicalRoutes: [],
    deletedRoutes: [],
    backupSha256: hash(fs.readFileSync(backup)),
    installedSha256: hash(fs.readFileSync(target)),
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy가 필요함');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify 근거 보존');
  const local = fetchBytes('http://127.0.0.1:8768/spinon/roadmap.html');
  const remoteBytes = fetchBytes(remote);
  const expected = hash(sourceBytes);
  assert.equal(hash(fs.readFileSync(target)), expected, 'preview 파일과 source 불일치');
  assert.equal(hash(local), expected, 'local HTTP 제공본과 source 불일치');
  assert.equal(hash(remoteBytes), expected, 'Tailnet HTTP 제공본과 source 불일치');
  const report = {
    previewServerPid: server.pid,
    localStatus: 200,
    localBytes: local.length,
    localSha256: hash(local),
    remoteStatus: 200,
    remoteBytes: remoteBytes.length,
    remoteSha256: hash(remoteBytes),
    sourceBytes: sourceBytes.length,
    sourceSha256: expected,
    changedRoutes: ['roadmap.html'],
    deletedRoutes: [],
    previewServerRestarted: false,
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}
