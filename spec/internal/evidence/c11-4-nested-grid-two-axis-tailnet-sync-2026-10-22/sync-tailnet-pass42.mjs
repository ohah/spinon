import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const [mode, preflightArgument] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-two-axis-multitrack-tailnet-sync-site-pass42');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-cyclic-two-axis-multitrack-tailnet-sync-2026-10-22/pass42';
const targetRootRelative = 'SpinonPreview';
const backupRootRelative = 'SpinonPreviewBackups/c11-4-cyclic-two-axis-multitrack-tailnet-sync-2026-10-22/pass42';

assert.equal(pass, 'pass42', 'set SPINON_TAILNET_RUN=pass42 to enable this synchronization');
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet-pass42.mjs preflight|copy|verify [PREFLIGHT_JSON]');

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

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
  const paths = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected build: ${relative}`);
    if (entry.isDirectory()) paths.push(...listFiles(fullPath, relative));
    else if (entry.isFile()) paths.push(relative);
    else throw new Error(`unsupported selected build entry: ${relative}`);
  }
  return paths;
}

function inspectPath(root, relative) {
  assertDirectory(root);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in target path: ${relative}`);
      if (index < parts.length - 1 && !stat.isDirectory()) throw new Error(`non-directory in target path: ${relative}`);
      if (index === parts.length - 1 && !stat.isFile()) throw new Error(`non-file target: ${relative}`);
      if (index === parts.length - 1) {
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
  const absolutePath = filePath ?? path.join(evidenceDirectory, 'preflight-pass42.json');
  const value = JSON.parse(fs.readFileSync(absolutePath, 'utf8'));
  assert.equal(value.pass, 'pass42');
  assert.equal(value.preview_root_relative, targetRootRelative);
  assert.equal(value.source_root_relative, path.relative(repositoryRoot, sourceRoot).split(path.sep).join('/'));
  assert.ok(value.files.length > 0);
  return { absolutePath, value };
}

if (mode === 'preflight') {
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  const entries = listFiles(sourceRoot).map((relative) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, relative));
    const previous = inspectPath(targetRoot, relative);
    const sourceHash = sha256(sourceBytes);
    const status = !previous.exists ? 'create' : previous.sha256 === sourceHash ? 'same' : 'replace';
    return {
      path: relative,
      status,
      source_bytes: sourceBytes.byteLength,
      source_sha256: sourceHash,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.byteLength : null,
      previous_sha256: previous.exists ? previous.sha256 : null,
      backup_path: status === 'replace' ? `${backupRootRelative}/${relative}` : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [
    status, entries.filter((entry) => entry.status === status).length,
  ]));
  const output = {
    pass,
    source_root_relative: path.relative(repositoryRoot, sourceRoot).split(path.sep).join('/'),
    preview_root_relative: targetRootRelative,
    backup_root_relative: backupRootRelative,
    target_count: entries.length,
    source_bytes_total: entries.reduce((sum, entry) => sum + entry.source_bytes, 0),
    backup_bytes_total: entries.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
    counts,
    no_deletes: true,
    files: entries,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'preflight-pass42.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ pass, target_count: output.target_count, source_bytes_total: output.source_bytes_total, backup_bytes_total: output.backup_bytes_total, counts, no_deletes: true }, null, 2)}\n`);
}

if (mode === 'copy') {
  const { value: preflight } = readPreflight(preflightArgument);
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup directory: ${backupRoot}`);
  fs.mkdirSync(backupRoot, { recursive: true });
  assertDirectory(backupRoot);

  const copied = [];
  for (const entry of preflight.files) {
    const sourcePath = path.join(sourceRoot, entry.path);
    const sourceBytes = fs.readFileSync(sourcePath);
    assert.equal(sourceBytes.byteLength, entry.source_bytes, `source byte count changed: ${entry.path}`);
    assert.equal(sha256(sourceBytes), entry.source_sha256, `source hash changed: ${entry.path}`);
    const existing = inspectPath(targetRoot, entry.path);
    assert.equal(existing.exists, entry.target_exists, `target existence changed since preflight: ${entry.path}`);
    if (existing.exists) {
      assert.equal(existing.bytes.byteLength, entry.previous_bytes, `target size changed since preflight: ${entry.path}`);
      assert.equal(existing.sha256, entry.previous_sha256, `target hash changed since preflight: ${entry.path}`);
    }
    if (entry.status === 'same') {
      copied.push({ path: entry.path, action: 'same', sha256: entry.source_sha256 });
      continue;
    }

    const destination = path.join(targetRoot, entry.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (entry.status === 'replace') {
      const backupPath = path.join(backupRoot, entry.path);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.writeFileSync(backupPath, existing.bytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backupPath)), entry.previous_sha256, `backup hash mismatch: ${entry.path}`);
    }
    const temporaryPath = `${destination}.spinon-pass42-${randomUUID()}.tmp`;
    fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporaryPath, destination);
    copied.push({ path: entry.path, action: entry.status, sha256: entry.source_sha256 });
  }

  const output = {
    pass,
    source_root_relative: path.relative(repositoryRoot, sourceRoot).split(path.sep).join('/'),
    preview_root_relative: targetRootRelative,
    backup_root_relative: backupRootRelative,
    counts: Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, copied.filter((entry) => entry.action === status).length])),
    backups_verified: copied.filter((entry) => entry.action === 'replace').length,
    copied,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'copy-and-backup-pass42.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ pass, counts: output.counts, backups_verified: output.backups_verified }, null, 2)}\n`);
}

if (mode === 'verify') {
  const { value: preflight } = readPreflight(preflightArgument);
  assertDirectory(sourceRoot);
  assertDirectory(targetRoot);
  assertDirectory(backupRoot);
  const failures = [];
  let matchingTargetCount = 0;
  let matchingBackupCount = 0;
  for (const entry of preflight.files) {
    const target = inspectPath(targetRoot, entry.path);
    const targetMatches = target.exists
      && target.bytes.byteLength === entry.source_bytes
      && target.sha256 === entry.source_sha256;
    if (targetMatches) matchingTargetCount += 1;
    else failures.push({ path: entry.path, kind: 'target', actual_sha256: target.exists ? target.sha256 : null, expected_sha256: entry.source_sha256 });

    if (entry.status === 'replace') {
      const backupPath = path.join(backupRoot, entry.path);
      const backupBytes = fs.readFileSync(backupPath);
      const backupMatches = backupBytes.byteLength === entry.previous_bytes && sha256(backupBytes) === entry.previous_sha256;
      if (backupMatches) matchingBackupCount += 1;
      else failures.push({ path: entry.path, kind: 'backup', actual_sha256: sha256(backupBytes), expected_sha256: entry.previous_sha256 });
    }
  }
  const output = {
    pass,
    target_count: preflight.files.length,
    matching_target_count: matchingTargetCount,
    backup_count: preflight.counts.replace,
    matching_backup_count: matchingBackupCount,
    failure_count: failures.length,
    failures,
    all_verified: failures.length === 0,
  };
  fs.writeFileSync(path.join(evidenceDirectory, 'final-verification-pass42.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ pass, target_count: output.target_count, matching_target_count: output.matching_target_count, backup_count: output.backup_count, matching_backup_count: output.matching_backup_count, failure_count: output.failure_count, all_verified: output.all_verified }, null, 2)}\n`);
  if (!output.all_verified) process.exitCode = 1;
}
