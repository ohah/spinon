import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const expectedHead = '82f7514dc51bedd4160f27180145a13bc3df8750';
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const builtPath = path.join(repositoryRoot, 'build/spinon/c11-4-pr550-tailnet-site-pass69-selected/roadmap.html');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const targetPath = path.join(targetRoot, 'roadmap.html');
const backupPath = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr550-tailnet-roadmap-sync-2026-10-23/pass69/roadmap.html';
const url = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const mode = process.argv[2];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const localHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const remoteHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(localHead, expectedHead);
assert.equal(remoteHead, expectedHead);
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-static-roadmap-pass69.mjs preflight|copy|verify');

function safeRegularFile(filePath, mayBeMissing = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
    return stat;
  } catch (error) {
    if (mayBeMissing && error.code === 'ENOENT') return null;
    throw error;
  }
}

function save(name, value) {
  fs.writeFileSync(path.join(evidenceDirectory, name), `${JSON.stringify(value, null, 2)}\n`);
}

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, 'roadmap-preflight-pass69.json')));
  assert.ok(!fs.existsSync(backupPath), 'refusing to reuse roadmap backup');
  safeRegularFile(sourcePath);
  safeRegularFile(targetPath);
  const source = fs.readFileSync(sourcePath);
  const previous = fs.readFileSync(targetPath);
  assert.equal(previous.byteLength, 267555, 'Tailnet roadmap size changed since inventory');
  assert.equal(sha256(previous), 'd01d12f91617c559772ec61f0fd934994dba22e80f7534d825e483b6ee296f09', 'Tailnet roadmap changed since inventory');
  const record = {
    pass: 'pass69',
    sourceHead: expectedHead,
    source: { path: 'spec/public/roadmap.html', bytes: source.byteLength, sha256: sha256(source) },
    builtRoute: 'build/spinon/c11-4-pr550-tailnet-site-pass69-selected/roadmap.html',
    target: { path: 'roadmap.html', bytes: previous.byteLength, sha256: sha256(previous) },
    backup: { path: 'SpinonPreviewBackups/c11-4-pr550-tailnet-roadmap-sync-2026-10-23/pass69/roadmap.html' },
    alreadyMatchesSource: previous.byteLength === source.byteLength && sha256(previous) === sha256(source),
    copyNeeded: previous.byteLength !== source.byteLength || sha256(previous) !== sha256(source),
    noDeletes: true,
  };
  save('roadmap-preflight-pass69.json', record);
  process.stdout.write(`${JSON.stringify(record, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'roadmap-preflight-pass69.json'), 'utf8'));
  const source = fs.readFileSync(sourcePath);
  const previousStat = safeRegularFile(targetPath);
  assert.equal(sha256(source), preflight.source.sha256);
  assert.equal(previousStat.size, preflight.target.bytes);
  assert.equal(sha256(fs.readFileSync(targetPath)), preflight.target.sha256);
  fs.writeFileSync(builtPath, source, { flag: 'wx' });
  const targetAlreadyMatches = preflight.target.sha256 === preflight.source.sha256
    && preflight.target.bytes === preflight.source.bytes;
  if (!targetAlreadyMatches) {
    fs.linkSync(targetPath, backupPath);
    assert.equal(sha256(fs.readFileSync(backupPath)), preflight.target.sha256);
    const temporaryPath = `${targetPath}.spinon-pass69-${randomUUID()}.tmp`;
    try {
      fs.copyFileSync(sourcePath, temporaryPath, fs.constants.COPYFILE_EXCL);
      fs.renameSync(temporaryPath, targetPath);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
  }
  const result = {
    pass: 'pass69',
    sourceBytes: source.byteLength,
    sourceSha256: sha256(source),
    previousBackedUp: !targetAlreadyMatches,
    backupSha256: targetAlreadyMatches ? null : sha256(fs.readFileSync(backupPath)),
    targetMatchesSource: sha256(fs.readFileSync(targetPath)) === sha256(source),
    copyMethod: targetAlreadyMatches ? 'no Tailnet write; source route staged for cross-route verification' : 'same-volume hard-link backup, atomic temporary-file replacement',
    targetAlreadyMatched: targetAlreadyMatches,
    noDeletes: true,
  };
  assert.equal(result.targetMatchesSource, true);
  save('roadmap-copy-and-backup-pass69.json', result);
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}

if (mode === 'verify') {
  const preflight = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'roadmap-preflight-pass69.json'), 'utf8'));
  const source = fs.readFileSync(sourcePath);
  const built = fs.readFileSync(builtPath);
  const target = fs.readFileSync(targetPath);
  assert.equal(sha256(source), preflight.source.sha256);
  assert.equal(sha256(built), preflight.source.sha256);
  assert.equal(sha256(target), preflight.source.sha256);
  const targetAlreadyMatched = preflight.target.sha256 === preflight.source.sha256
    && preflight.target.bytes === preflight.source.bytes;
  let backupSha256 = null;
  if (targetAlreadyMatched) assert.ok(!fs.existsSync(backupPath), 'unexpected backup for unchanged roadmap');
  else {
    const backup = fs.readFileSync(backupPath);
    backupSha256 = sha256(backup);
    assert.equal(backupSha256, preflight.target.sha256);
  }
  const response = await fetch(url, { cache: 'no-store', signal: AbortSignal.timeout(30000) });
  const served = Buffer.from(await response.arrayBuffer());
  const remote = {
    url,
    status: response.status,
    bytes: served.byteLength,
    sha256: sha256(served),
    matchesSource: response.status === 200 && served.byteLength === source.byteLength && sha256(served) === preflight.source.sha256,
  };
  assert.equal(remote.matchesSource, true);
  const result = {
    pass: 'pass69',
    local: { sourceSha256: sha256(source), builtSha256: sha256(built), targetSha256: sha256(target), backupSha256 },
    remote,
    targetAlreadyMatched,
    noDeletes: true,
    allVerified: true,
  };
  save('roadmap-remote-verification-pass69.json', result);
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}
