import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceHead = '986d596fdea4f9235671acc4f9bdd6c13dc3a06e';
const sourceRelative = 'spec/public/roadmap.html';
const sourcePath = path.join(repositoryRoot, sourceRelative);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const targetPath = path.join(targetRoot, 'roadmap.html');
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass66';
const backupPath = path.join(backupRoot, 'roadmap.html');
const priorManifestPath = path.join(evidenceDirectory, 'preflight-pass65.json');
const mode = process.argv[2];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const localHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const remoteHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceBlob = execFileSync('git', ['rev-parse', `origin/main:${sourceRelative}`], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const localBlob = execFileSync('git', ['rev-parse', `HEAD:${sourceRelative}`], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(localHead, sourceHead, 'unexpected local HEAD');
assert.equal(remoteHead, sourceHead, 'unexpected origin/main');
assert.equal(localBlob, sourceBlob, 'local source does not match origin/main');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-roadmap-pass66.mjs preflight|copy|verify');

function assertSafeFile(filePath) {
  const stat = fs.lstatSync(filePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
}
function sourceBytes() {
  const bytes = fs.readFileSync(sourcePath);
  const text = bytes.toString('utf8');
  assert.ok(text.includes('/spinon/STATUS.html#css-구현-체크리스트'), 'corrected status anchor missing');
  const start = text.indexOf('2026-10-23 C11.4 auto-repeat + <code>grid-template-areas</code> 단일 축 제한 runtime slice');
  const end = text.indexOf('</p>', start);
  assert.ok(start >= 0 && end > start && !text.slice(start, end).includes('#c11-css-grid'), 'stale fragment remains in PR #539 paragraph');
  return bytes;
}
function readPreflight() {
  const data = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'preflight-pass66.json'), 'utf8'));
  assert.equal(data.pass, 'pass66');
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
  const pass65 = JSON.parse(fs.readFileSync(priorManifestPath, 'utf8'));
  const previousPass65 = pass65.files.find((item) => item.path === 'roadmap.html');
  assert.ok(previousPass65, 'pass65 roadmap manifest entry missing');
  assert.equal(sha256(previous), previousPass65.sourceSha256, 'Tailnet roadmap changed after pass65');
  assert.notEqual(sha256(source), sha256(previous), 'pass66 roadmap already matches source');
  const output = {
    pass: 'pass66', sourceHead, sourcePath: sourceRelative,
    sourceBytes: source.byteLength, sourceSha256: sha256(source),
    targetRootRelative: 'SpinonPreview', path: 'roadmap.html',
    previousBytes: previous.byteLength, previousSha256: sha256(previous),
    previousMatchesPass65: true,
    backupRootRelative: 'SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass66',
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'preflight-pass66.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
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
  const temporary = `${targetPath}.spinon-pass66-${randomUUID()}.tmp`;
  fs.linkSync(sourcePath, temporary);
  try {
    fs.renameSync(temporary, targetPath);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  assert.equal(sha256(fs.readFileSync(targetPath)), preflight.sourceSha256, 'replacement hash mismatch');
  const output = {
    pass: 'pass66', path: 'roadmap.html', previousBackedUp: true,
    targetMatchesSource: true, backupMatchesPrevious: true,
    copyMethod: 'same-volume hard link and atomic rename', noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'copy-and-backup-pass66.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
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
  const output = { pass: 'pass66', local, remote, noDeletes: true, allVerified: true };
  fs.writeFileSync(path.join(evidenceDirectory, 'remote-verification-pass66.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}
