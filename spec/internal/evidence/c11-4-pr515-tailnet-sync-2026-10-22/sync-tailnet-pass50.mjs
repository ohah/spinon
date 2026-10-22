import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr515-tailnet-site-pass50');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr515-tailnet-sync-2026-10-22/pass50';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.equal(pass, 'pass50', 'set SPINON_TAILNET_RUN=pass50 to enable this synchronization');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass50.mjs preflight|copy|verify [PREFLIGHT_JSON]');

function assertDirectory(root, { allowMissing = false } = {}) {
  try {
    const stat = fs.lstatSync(root);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${root}`);
    return true;
  } catch (error) {
    if (allowMissing && error.code === 'ENOENT') return false;
    throw error;
  }
}

function listFiles(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected build: ${relative}`);
    if (entry.isDirectory()) result.push(...listFiles(absolute, relative));
    else if (entry.isFile()) result.push(relative);
    else throw new Error(`unsupported selected build entry: ${relative}`);
  }
  return result;
}

function inspect(root, relative) {
  assertDirectory(root);
  let current = root;
  for (const [index, segment] of relative.split('/').entries()) {
    current = path.join(current, segment);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in target path: ${relative}`);
      if (index < relative.split('/').length - 1 && !stat.isDirectory()) throw new Error(`non-directory in target path: ${relative}`);
      if (index === relative.split('/').length - 1 && !stat.isFile()) throw new Error(`non-file target: ${relative}`);
      if (index === relative.split('/').length - 1) {
        const bytes = fs.readFileSync(current);
        return { exists: true, bytes, sha256: sha256(bytes) };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function readPreflight(filePath) {
  const absolute = filePath ?? path.join(evidenceDirectory, 'preflight-pass50.json');
  const value = JSON.parse(fs.readFileSync(absolute, 'utf8'));
  assert.equal(value.pass, pass);
  assert.equal(value.previewRootRelative, 'SpinonPreview');
  assert.equal(value.sourceRootRelative, 'build/spinon/c11-4-pr515-tailnet-site-pass50');
  assert.ok(value.files.length > 0);
  return value;
}

if (mode === 'preflight') {
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  const files = listFiles(sourceRoot).map((relative) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, relative));
    const previous = inspect(targetRoot, relative);
    const sourceHash = sha256(sourceBytes);
    const action = !previous.exists ? 'create' : previous.sha256 === sourceHash ? 'same' : 'replace';
    return {
      path: relative, action, sourceBytes: sourceBytes.byteLength, sourceSha256: sourceHash,
      previousExists: previous.exists, previousBytes: previous.exists ? previous.bytes.byteLength : null,
      previousSha256: previous.exists ? previous.sha256 : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, files.filter((item) => item.action === action).length]));
  const value = {
    pass, sourceRootRelative: path.relative(repositoryRoot, sourceRoot).split(path.sep).join('/'),
    previewRootRelative: 'SpinonPreview', backupRootRelative: 'SpinonPreviewBackups/c11-4-pr515-tailnet-sync-2026-10-22/pass50',
    fileCount: files.length, sourceBytes: files.reduce((sum, item) => sum + item.sourceBytes, 0),
    replacementBackupBytes: files.filter((item) => item.action === 'replace').reduce((sum, item) => sum + item.previousBytes, 0),
    counts, noDeletes: true, files,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'preflight-pass50.json'), `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ pass, fileCount: value.fileCount, sourceBytes: value.sourceBytes, replacementBackupBytes: value.replacementBackupBytes, counts, noDeletes: true }, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readPreflight(preflightArgument);
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  fs.mkdirSync(backupRoot, { recursive: true });
  assertDirectory(backupRoot);
  const counts = { create: 0, replace: 0, same: 0 };
  for (const item of preflight.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, item.path));
    assert.equal(sourceBytes.byteLength, item.sourceBytes, `source size changed: ${item.path}`);
    assert.equal(sha256(sourceBytes), item.sourceSha256, `source hash changed: ${item.path}`);
    const previous = inspect(targetRoot, item.path);
    assert.equal(previous.exists, item.previousExists, `target existence changed: ${item.path}`);
    if (previous.exists) {
      assert.equal(previous.bytes.byteLength, item.previousBytes, `target size changed: ${item.path}`);
      assert.equal(previous.sha256, item.previousSha256, `target changed since preflight: ${item.path}`);
    }
    if (item.action === 'same') {
      counts.same += 1;
      continue;
    }
    const destination = path.join(targetRoot, item.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (item.action === 'replace') {
      const backupPath = path.join(backupRoot, item.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backupPath)), item.previousSha256, `backup hash mismatch: ${item.path}`);
    }
    const temporary = `${destination}.spinon-pass50-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporary, destination);
    counts[item.action] += 1;
  }
  const output = {
    pass, fileCount: preflight.fileCount, counts,
    backupsVerified: counts.replace, noDeletes: true,
    backupRootRelative: preflight.backupRootRelative,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'copy-and-backup-pass50.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
}

if (mode === 'verify') {
  const preflight = readPreflight(preflightArgument);
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assertDirectory(backupRoot);
  let targetsVerified = 0;
  let backupsVerified = 0;
  const failures = [];
  for (const item of preflight.files) {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, item.path));
    const target = inspect(targetRoot, item.path);
    if (sourceBytes.byteLength === item.sourceBytes && sha256(sourceBytes) === item.sourceSha256
      && target.exists && target.bytes.byteLength === item.sourceBytes && target.sha256 === item.sourceSha256) targetsVerified += 1;
    else failures.push({ path: item.path, kind: 'target' });
    if (item.action === 'replace') {
      const backup = fs.readFileSync(path.join(backupRoot, item.path));
      if (backup.byteLength === item.previousBytes && sha256(backup) === item.previousSha256) backupsVerified += 1;
      else failures.push({ path: item.path, kind: 'backup' });
    }
  }
  const output = {
    pass, fileCount: preflight.fileCount, targetsVerified,
    backupsExpected: preflight.counts.replace, backupsVerified,
    failures, failureCount: failures.length, allVerified: failures.length === 0,
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'final-verification-pass50.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...output, failures: undefined }, null, 2)}\n`);
  if (!output.allVerified) process.exitCode = 1;
}
