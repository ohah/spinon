import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceHead = '10ae04a09f9f0b1b97056693dad0438a2c022b62';
const branchHead = '1d04fec9b174ed16a65021129134145684b01bcc';
const originMain = '22eafb6cad66b6be8facaf264299e7bd6073d0f5';
const sourceRelative = 'build/spinon/c11-4-pr562-tailnet-site-pass70-selected';
const sourceRoot = path.join(repositoryRoot, sourceRelative);
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr562-tailnet-roadmap-sync-2026-10-23/pass70';
const previewOrigin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const mode = process.argv[2];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertSourceRevision() {
  const localHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
  const remoteHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
  assert.equal(localHead, branchHead, 'checkout is not the selected Tailnet sync PR revision');
  assert.equal(remoteHead, originMain, 'origin/main changed after site build');
}

function safeRelative(relative) {
  assert.ok(relative && !path.isAbsolute(relative), `absolute or empty path: ${relative}`);
  const normalized = path.normalize(relative);
  assert.ok(normalized !== '..' && !normalized.startsWith(`..${path.sep}`), `path escapes root: ${relative}`);
  return normalized;
}

function assertRegularFile(filePath, mayBeMissing = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
    return stat;
  } catch (error) {
    if (mayBeMissing && error.code === 'ENOENT') return null;
    throw error;
  }
}

function listFiles(root, relative = '') {
  const entries = fs.readdirSync(path.join(root, relative), { withFileTypes: true })
    .sort((a, b) => a.name.localeCompare(b.name));
  const files = [];
  for (const entry of entries) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(root, child);
    if (entry.isSymbolicLink()) throw new Error(`symlink in tree: ${child}`);
    if (entry.isDirectory()) files.push(...listFiles(root, child));
    else if (entry.isFile()) files.push({ path: child, bytes: fs.statSync(absolute).size });
    else throw new Error(`unsupported tree entry: ${child}`);
  }
  return files;
}

function readJson(name) {
  return JSON.parse(fs.readFileSync(path.join(evidenceDirectory, name), 'utf8'));
}

function writeJson(name, value) {
  fs.writeFileSync(path.join(evidenceDirectory, name), `${JSON.stringify(value, null, 2)}\n`);
}

if (!['preflight', 'copy', 'verify'].includes(mode)) {
  throw new Error('usage: node sync-tailnet-pass70.mjs preflight|copy|verify');
}

assertSourceRevision();
assert.ok(fs.statSync(sourceRoot).isDirectory(), 'selected RSPress output is missing');
assert.ok(fs.statSync(targetRoot).isDirectory(), 'Tailnet preview root is missing');

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, 'preflight-pass70.json')), 'preflight already exists');
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const sourceFiles = listFiles(sourceRoot);
  const targetFiles = listFiles(targetRoot);
  const targetByPath = new Map(targetFiles.map((file) => [file.path, file]));
  const delta = [];
  let unchangedFiles = 0;
  for (const source of sourceFiles) {
    const relative = safeRelative(source.path);
    const sourcePath = path.join(sourceRoot, relative);
    const sourceBytes = fs.readFileSync(sourcePath);
    const sourceHash = sha256(sourceBytes);
    const previous = targetByPath.get(relative);
    if (!previous) {
      delta.push({ path: relative, operation: 'add', bytes: sourceBytes.byteLength, sha256: sourceHash });
      continue;
    }
    const previousBytes = fs.readFileSync(path.join(targetRoot, relative));
    const previousHash = sha256(previousBytes);
    if (sourceBytes.byteLength === previousBytes.byteLength && sourceHash === previousHash) {
      unchangedFiles += 1;
      continue;
    }
    delta.push({
      path: relative,
      operation: 'replace',
      bytes: sourceBytes.byteLength,
      sha256: sourceHash,
      previousBytes: previousBytes.byteLength,
      previousSha256: previousHash,
    });
  }
  const preflight = {
    pass: 'pass70',
    createdAt: new Date().toISOString(),
    sourceHead,
    branchHead: localHead,
    originMain,
    sourceRoot: sourceRelative,
    targetRootRelative: 'SpinonPreview',
    backupRootRelative: 'SpinonPreviewBackups/c11-4-pr562-tailnet-roadmap-sync-2026-10-23/pass70',
    sourceFiles: sourceFiles.length,
    sourceBytes: sourceFiles.reduce((sum, file) => sum + file.bytes, 0),
    previousTargetFiles: targetFiles.length,
    previousTargetBytes: targetFiles.reduce((sum, file) => sum + file.bytes, 0),
    unchangedFiles,
    addedFiles: delta.filter((file) => file.operation === 'add').length,
    replacedFiles: delta.filter((file) => file.operation === 'replace').length,
    deltaFiles: delta.length,
    deltaBytes: delta.reduce((sum, file) => sum + file.bytes, 0),
    backupBytes: delta.reduce((sum, file) => sum + (file.previousBytes ?? 0), 0),
    noDeletes: true,
  };
  writeJson('preflight-pass70.json', preflight);
  writeJson('delta-manifest-pass70.json', delta);
  process.stdout.write(`${JSON.stringify(preflight, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readJson('preflight-pass70.json');
  const delta = readJson('delta-manifest-pass70.json');
  assert.equal(preflight.sourceHead, sourceHead);
  assert.equal(preflight.branchHead, branchHead);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
  const backupDevice = fs.statSync(targetRoot).dev;
  fs.mkdirSync(backupRoot, { recursive: true });
  let addedFiles = 0;
  let replacedFiles = 0;
  let alreadyAppliedFiles = 0;
  for (const entry of delta) {
    const relative = safeRelative(entry.path);
    const sourcePath = path.join(sourceRoot, relative);
    const targetPath = path.join(targetRoot, relative);
    const sourceStat = assertRegularFile(sourcePath);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceStat.size, entry.bytes, `source size changed: ${relative}`);
    assert.equal(sha256(sourceBytes), entry.sha256, `source hash changed: ${relative}`);
    fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const previousStat = assertRegularFile(targetPath, true);
    if (previousStat) {
      const previousBytes = fs.readFileSync(targetPath);
      const previousHash = sha256(previousBytes);
      if (previousHash === entry.sha256 && previousBytes.byteLength === entry.bytes) {
        alreadyAppliedFiles += 1;
        continue;
      }
      assert.equal(entry.operation, 'replace', `new target appeared unexpectedly: ${relative}`);
      assert.equal(previousBytes.byteLength, entry.previousBytes, `target size changed after preflight: ${relative}`);
      assert.equal(previousHash, entry.previousSha256, `target content changed after preflight: ${relative}`);
      const backupPath = path.join(backupRoot, relative);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      assert.ok(!fs.existsSync(backupPath), `backup already exists: ${relative}`);
      if (previousStat.dev === backupDevice) fs.linkSync(targetPath, backupPath);
      else fs.copyFileSync(targetPath, backupPath, fs.constants.COPYFILE_EXCL);
      assert.equal(sha256(fs.readFileSync(backupPath)), entry.previousSha256, `backup hash mismatch: ${relative}`);
      replacedFiles += 1;
    } else {
      assert.equal(entry.operation, 'add', `preflight target disappeared: ${relative}`);
      addedFiles += 1;
    }
    const temporaryPath = `${targetPath}.spinon-pass70-${randomUUID()}.tmp`;
    try {
      fs.copyFileSync(sourcePath, temporaryPath, fs.constants.COPYFILE_EXCL);
      fs.chmodSync(temporaryPath, sourceStat.mode & 0o777);
      fs.renameSync(temporaryPath, targetPath);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    assert.equal(sha256(fs.readFileSync(targetPath)), entry.sha256, `replacement hash mismatch: ${relative}`);
  }
  const result = {
    pass: 'pass70',
    addedFiles,
    replacedFiles,
    alreadyAppliedFiles,
    backupRootRelative: preflight.backupRootRelative,
    backupMethod: 'same-volume hard link where possible; otherwise exclusive copy',
    replacementMethod: 'copy to unique sibling temporary file then atomic rename',
    noDeletes: true,
  };
  writeJson('copy-and-backup-pass70.json', result);
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}

if (mode === 'verify') {
  const preflight = readJson('preflight-pass70.json');
  const delta = readJson('delta-manifest-pass70.json');
  const localMismatches = [];
  for (const entry of delta) {
    const relative = safeRelative(entry.path);
    const targetPath = path.join(targetRoot, relative);
    const targetStat = assertRegularFile(targetPath);
    const targetBytes = fs.readFileSync(targetPath);
    assert.equal(targetStat.size, entry.bytes, `local size mismatch: ${relative}`);
    assert.equal(sha256(targetBytes), entry.sha256, `local hash mismatch: ${relative}`);
    if (entry.operation === 'replace') {
      const backupPath = path.join(backupRoot, relative);
      assertRegularFile(backupPath);
      assert.equal(sha256(fs.readFileSync(backupPath)), entry.previousSha256, `backup hash mismatch: ${relative}`);
    }
  }
  const targetAfter = listFiles(targetRoot);
  assert.ok(targetAfter.length >= preflight.previousTargetFiles, 'target file count decreased');
  const routes = [
    'roadmap.html',
    'STATUS.html',
    '0001-conformance.html',
    'internal/README.html',
    'internal/0059-c11-grid-layout.html',
    'plan/c11-4-auto-repeat-positive-percent-minimum.html',
    'plan/c11-4-auto-repeat-positive-percent-minimum-revision-review-20.html',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/README.html',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/README.html',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/implementation-runtime-review-20.html',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/pr-diff-review-20.html',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/android-avd-c114-positive-percent-minimum-commit-android-avd-20261023T104649350Z-da0f371b/percentage-overflow-columns-calc.png',
    'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/ios-simulator-c114-positive-percent-minimum-commit-ios-simulator-20261023T104719112Z-2d91e097/percentage-overflow-columns-calc.png',
    'internal/evidence/c11-4-positive-percent-status-update-2026-10-23/pr-diff-review-20.html',
  ];
  const remoteChecks = [];
  const remoteRouteBodies = new Map();
  for (const relative of routes) {
    const sourcePath = path.join(sourceRoot, safeRelative(relative));
    const sourceBytes = fs.readFileSync(sourcePath);
    const expected = { bytes: sourceBytes.byteLength, sha256: sha256(sourceBytes) };
    assert.equal(sha256(fs.readFileSync(path.join(targetRoot, relative))), expected.sha256, `local route differs from source: ${relative}`);
    const response = await fetch(new URL(encodeURI(relative), previewOrigin), {
      cache: 'no-store',
      signal: AbortSignal.timeout(30000),
    });
    const bytes = Buffer.from(await response.arrayBuffer());
    const actualHash = sha256(bytes);
    assert.equal(response.status, 200, `HTTP ${response.status} for ${relative}`);
    assert.equal(bytes.byteLength, expected.bytes, `remote size mismatch: ${relative}`);
    assert.equal(actualHash, expected.sha256, `remote hash mismatch: ${relative}`);
    remoteChecks.push({ path: relative, status: response.status, bytes: bytes.byteLength, sha256: actualHash, matchesSource: true });
    remoteRouteBodies.set(relative, bytes.toString('utf8'));
  }
  const assetPaths = new Set();
  for (const html of remoteRouteBodies.values()) {
    for (const match of html.matchAll(/(?:src|href)=["'](?:https?:\/\/[^"']+)?\/?spinon\/(static\/[^"'#?]+)["']/g)) {
      assetPaths.add(match[1]);
    }
  }
  for (const asset of [...assetPaths].sort()) {
    const sourcePath = path.join(sourceRoot, safeRelative(asset));
    const sourceBytes = fs.readFileSync(sourcePath);
    const expected = { bytes: sourceBytes.byteLength, sha256: sha256(sourceBytes) };
    assert.equal(sha256(fs.readFileSync(path.join(targetRoot, asset))), expected.sha256, `local asset differs from source: ${asset}`);
    const response = await fetch(new URL(encodeURI(asset), previewOrigin), {
      cache: 'no-store',
      signal: AbortSignal.timeout(30000),
    });
    const bytes = Buffer.from(await response.arrayBuffer());
    const actualHash = sha256(bytes);
    assert.equal(response.status, 200, `HTTP ${response.status} for ${asset}`);
    assert.equal(bytes.byteLength, expected.bytes, `remote asset size mismatch: ${asset}`);
    assert.equal(actualHash, expected.sha256, `remote asset hash mismatch: ${asset}`);
    remoteChecks.push({ path: asset, status: response.status, bytes: bytes.byteLength, sha256: actualHash, matchesSource: true });
  }
  const result = {
    pass: 'pass70',
    sourceHead,
    branchHead,
    localDeltaFilesVerified: delta.length,
    localMismatches,
    targetFilesBefore: preflight.previousTargetFiles,
    targetFilesAfter: targetAfter.length,
    noDeletes: targetAfter.length >= preflight.previousTargetFiles,
    remoteChecks,
    allVerified: true,
  };
  writeJson('remote-verification-pass70.json', result);
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}
