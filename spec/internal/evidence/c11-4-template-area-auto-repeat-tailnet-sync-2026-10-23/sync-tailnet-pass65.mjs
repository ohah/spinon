import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass65';
const [mode, preflightArgument] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(pass, 'pass65', 'set SPINON_TAILNET_RUN=pass65 to enable this synchronization');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass65.mjs preflight|copy|verify [PREFLIGHT_JSON]');

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
    assert.ok(!entry.isSymbolicLink(), `symlink in source or target tree: ${relative}`);
    if (entry.isDirectory()) result.push(...listFiles(absolute, relative));
    else if (entry.isFile()) result.push(relative);
    else throw new Error(`unsupported tree entry: ${relative}`);
  }
  return result;
}
function inspectTarget(relative) {
  let current = targetRoot;
  const parts = relative.split('/');
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in target path: ${relative}`);
      else {
        assert.ok(stat.isFile(), `non-file target: ${relative}`);
        const bytes = fs.readFileSync(current);
        return { exists: true, bytes: bytes.byteLength, sha256: sha256(bytes), absolute: current };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false, absolute: path.join(targetRoot, relative) };
      throw error;
    }
  }
  return { exists: false, absolute: path.join(targetRoot, relative) };
}
function readPreflight(filePath = path.join(evidenceDirectory, 'preflight-pass65.json')) {
  const value = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  assert.equal(value.pass, pass);
  assert.equal(value.previewRootRelative, 'SpinonPreview');
  assert.equal(value.sourceRootRelative, 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected');
  assert.ok(value.files.length > 0);
  return value;
}

if (mode === 'preflight') {
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  const files = listFiles(sourceRoot).map((relative) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, relative));
    const previous = inspectTarget(relative);
    const sourceSha256 = sha256(sourceBytes);
    const action = !previous.exists ? 'create' : previous.sha256 === sourceSha256 ? 'same' : 'replace';
    return {
      path: relative, action, sourceBytes: sourceBytes.byteLength, sourceSha256,
      ...(previous.exists ? { previousBytes: previous.bytes, previousSha256: previous.sha256 } : {}),
    };
  });
  const counts = files.reduce((result, item) => {
    result[item.action] += 1;
    result.bytes[item.action] += item.sourceBytes;
    return result;
  }, { create: 0, replace: 0, same: 0, bytes: { create: 0, replace: 0, same: 0 } });
  const output = {
    pass, sourceRootRelative: 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected',
    previewRootRelative: 'SpinonPreview',
    backupRootRelative: 'SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass65',
    fileCount: files.length, counts, files, deletes: 0,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'preflight-pass65.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ pass, fileCount: files.length, counts, deletes: 0 }, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readPreflight(preflightArgument);
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assert.ok(!fs.existsSync(path.join(evidenceDirectory, 'copy-and-backup-pass65.json')), 'copy report already exists');
  let resumedItems = 0;
  const counts = { create: 0, replace: 0, same: 0 };
  for (const item of preflight.files) {
    const sourcePath = path.join(sourceRoot, item.path);
    const destination = path.join(targetRoot, item.path);
    const source = fs.readFileSync(sourcePath);
    assert.equal(source.byteLength, item.sourceBytes, `source size changed: ${item.path}`);
    assert.equal(sha256(source), item.sourceSha256, `source hash changed: ${item.path}`);
    const previous = inspectTarget(item.path);
    if (item.action === 'same') {
      assert.ok(previous.exists && previous.sha256 === item.sourceSha256, `identical target changed: ${item.path}`);
      counts.same += 1;
      continue;
    }
    if (item.action === 'create') assert.ok(!previous.exists, `new target appeared after preflight: ${item.path}`);
    if (item.action === 'replace') {
      assert.ok(previous.exists, `replacement target disappeared after preflight: ${item.path}`);
      assert.equal(previous.bytes, item.previousBytes, `replacement size changed after preflight: ${item.path}`);
      assert.equal(previous.sha256, item.previousSha256, `replacement hash changed after preflight: ${item.path}`);
      const backupPath = path.join(backupRoot, item.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      if (fs.existsSync(backupPath)) resumedItems += 1;
      else fs.linkSync(destination, backupPath);
      const backup = fs.readFileSync(backupPath);
      assert.equal(backup.byteLength, item.previousBytes, `backup size mismatch: ${item.path}`);
      assert.equal(sha256(backup), item.previousSha256, `backup hash mismatch: ${item.path}`);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    const temporary = `${destination}.spinon-pass65-${randomUUID()}.tmp`;
    fs.linkSync(sourcePath, temporary);
    try {
      fs.renameSync(temporary, destination);
    } catch (error) {
      if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
      throw error;
    }
    counts[item.action] += 1;
  }
  const output = {
    pass, fileCount: preflight.fileCount, counts, backupsVerified: counts.replace,
    noDeletes: true, resumedItems, fallbackReplacements: 0,
    copyStorage: 'same-volume hard links to verified build files',
    backupStorage: 'same-volume hard links preserving replaced target bytes',
    backupRootRelative: preflight.backupRootRelative,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'copy-and-backup-pass65.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
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
    const target = inspectTarget(item.path);
    if (sourceBytes.byteLength === item.sourceBytes && sha256(sourceBytes) === item.sourceSha256
      && target.exists && target.bytes === item.sourceBytes && target.sha256 === item.sourceSha256) targetsVerified += 1;
    else failures.push({ path: item.path, kind: 'target' });
    if (item.action === 'replace') {
      const backupBytes = fs.readFileSync(path.join(backupRoot, item.path));
      if (backupBytes.byteLength === item.previousBytes && sha256(backupBytes) === item.previousSha256) backupsVerified += 1;
      else failures.push({ path: item.path, kind: 'backup' });
    }
  }
  const output = {
    pass, fileCount: preflight.fileCount, targetsVerified,
    backupsExpected: preflight.counts.replace, backupsVerified,
    failures, failureCount: failures.length, allVerified: failures.length === 0, noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'final-verification-pass65.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...output, failures: undefined }, null, 2)}\n`);
  if (!output.allVerified) process.exitCode = 1;
}
