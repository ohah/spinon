import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceHead = '849f04fbffc6f9efb85f7269791c7b6c51768724';
const sourceRelative = 'spec/public/roadmap.html';
const sourcePath = path.join(repositoryRoot, sourceRelative);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const targetPath = path.join(targetRoot, 'roadmap.html');
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr546-tailnet-sync-2026-10-23/pass67';
const backupPath = path.join(backupRoot, 'roadmap.html');
const priorManifestPath = path.resolve(evidenceDirectory, '../c11-4-template-area-auto-repeat-tailnet-sync-2026-10-23/remote-verification-pass66.json');
const mode = process.argv[2];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const localHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const remoteHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceBlob = execFileSync('git', ['rev-parse', `origin/main:${sourceRelative}`], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const localBlob = execFileSync('git', ['rev-parse', `HEAD:${sourceRelative}`], { cwd: repositoryRoot, encoding: 'utf8' }).trim();

assert.equal(localHead, sourceHead, 'unexpected local HEAD');
assert.equal(remoteHead, sourceHead, 'unexpected origin/main');
assert.equal(localBlob, sourceBlob, 'local source does not match origin/main');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-roadmap-pass67.mjs preflight|copy|verify');

function assertSafeFile(filePath) {
  const stat = fs.lstatSync(filePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
}

function sourceBytes() {
  const bytes = fs.readFileSync(sourcePath);
  const text = bytes.toString('utf8');
  assert.ok(text.includes('PR #545는 Rebase and merge로'), 'PR #545 merge record missing');
  assert.ok(text.includes('d7a6b35966f7ba9ad53fc5c450bfd1e6ec9ead34'), 'PR #545 merge SHA missing');
  assert.ok(text.includes('C11.4 양축 auto-repeat + grid-template-areas runtime 제한 slice'), 'C11.4 roadmap entry missing');
  assert.ok(text.includes('77/213'), 'official completion count missing');
  assert.ok(text.includes('/spinon/STATUS.html#css-구현-체크리스트'), 'correct status anchor missing');
  return bytes;
}

function readPriorManifest() {
  const data = JSON.parse(fs.readFileSync(priorManifestPath, 'utf8'));
  assert.equal(data.pass, 'pass66');
  assert.equal(data.remote.status, 200);
  assert.equal(data.remote.matchesSource, true);
  return data;
}

function readPreflight() {
  const data = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'preflight-pass67.json'), 'utf8'));
  assert.equal(data.pass, 'pass67');
  assert.equal(data.sourceHead, sourceHead);
  assert.equal(data.path, 'roadmap.html');
  return data;
}

if (mode === 'preflight') {
  assertSafeFile(sourcePath);
  assertSafeFile(targetPath);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const source = sourceBytes();
  const previous = fs.readFileSync(targetPath);
  const pass66 = readPriorManifest();
  assert.equal(sha256(previous), pass66.local.targetSha256, 'Tailnet roadmap changed after pass66');
  assert.equal(sha256(previous), pass66.remote.sha256, 'pass66 local/remote hashes differ');
  assert.notEqual(sha256(source), sha256(previous), 'pass67 roadmap already matches source');
  const output = {
    pass: 'pass67', sourceHead, sourcePath: sourceRelative,
    sourceBytes: source.byteLength, sourceSha256: sha256(source),
    targetRootRelative: 'SpinonPreview', path: 'roadmap.html',
    previousBytes: previous.byteLength, previousSha256: sha256(previous),
    previousMatchesPass66: true,
    backupRootRelative: 'SpinonPreviewBackups/c11-4-pr546-tailnet-sync-2026-10-23/pass67',
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'preflight-pass67.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readPreflight();
  assertSafeFile(sourcePath);
  assertSafeFile(targetPath);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const source = sourceBytes();
  const previous = fs.readFileSync(targetPath);
  assert.equal(source.byteLength, preflight.sourceBytes);
  assert.equal(sha256(source), preflight.sourceSha256);
  assert.equal(previous.byteLength, preflight.previousBytes);
  assert.equal(sha256(previous), preflight.previousSha256);
  fs.mkdirSync(backupRoot, { recursive: true });
  fs.linkSync(targetPath, backupPath);
  assert.equal(sha256(fs.readFileSync(backupPath)), preflight.previousSha256, 'backup hash mismatch');
  const temporary = `${targetPath}.spinon-pass67-${randomUUID()}.tmp`;
  fs.linkSync(sourcePath, temporary);
  try {
    fs.renameSync(temporary, targetPath);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  assert.equal(sha256(fs.readFileSync(targetPath)), preflight.sourceSha256, 'replacement hash mismatch');
  const output = {
    pass: 'pass67', path: 'roadmap.html', previousBackedUp: true,
    targetMatchesSource: true, backupMatchesPrevious: true,
    copyMethod: 'same-volume hard link and atomic rename', noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'copy-and-backup-pass67.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}

if (mode === 'verify') {
  const preflight = readPreflight();
  assertSafeFile(sourcePath);
  assertSafeFile(targetPath);
  assertSafeFile(backupPath);
  const source = fs.readFileSync(sourcePath);
  const target = fs.readFileSync(targetPath);
  const backup = fs.readFileSync(backupPath);
  const local = {
    sourceBytes: source.byteLength, sourceSha256: sha256(source),
    targetBytes: target.byteLength, targetSha256: sha256(target),
    backupBytes: backup.byteLength, backupSha256: sha256(backup),
  };
  assert.equal(local.sourceSha256, preflight.sourceSha256);
  assert.equal(local.targetSha256, preflight.sourceSha256);
  assert.equal(local.backupSha256, preflight.previousSha256);
  const url = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
  const response = await fetch(url, { signal: AbortSignal.timeout(30000), cache: 'no-store' });
  const served = Buffer.from(await response.arrayBuffer());
  const remote = {
    url, status: response.status, bytes: served.byteLength,
    sha256: sha256(served), matchesSource: response.status === 200
      && served.byteLength === preflight.sourceBytes && sha256(served) === preflight.sourceSha256,
  };
  assert.equal(remote.matchesSource, true, 'remote response differs from main source');
  const output = { pass: 'pass67', local, remote, noDeletes: true, allVerified: true };
  fs.writeFileSync(path.join(evidenceDirectory, 'remote-verification-pass67.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}
