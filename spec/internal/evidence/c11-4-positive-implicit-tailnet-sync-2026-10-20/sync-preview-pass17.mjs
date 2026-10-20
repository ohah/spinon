import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const [mode, sourceArg, targetArg, backupArg, preflightArg] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
if (!['preflight', 'copy'].includes(mode) || !['pass17'].includes(pass)
    || !sourceArg || !targetArg || !backupArg) {
  throw new Error('usage: SPINON_TAILNET_RUN=pass17 node sync-preview.mjs preflight|copy SOURCE TARGET BACKUP [PREFLIGHT_JSON]');
}

const sourceRoot = path.resolve(repoRoot, sourceArg);
const targetRoot = path.resolve(targetArg);
const backupRoot = path.resolve(backupArg);
const sourceRootRelative = path.relative(repoRoot, sourceRoot).split(path.sep).join('/');
const expectedSourceRootRelative = `build/spinon/c11-4-positive-implicit-tailnet-site-${pass}`;
if (sourceRootRelative !== expectedSourceRootRelative) {
  throw new Error(`source root must be the sanitized Rspress output: ${expectedSourceRootRelative}`);
}
const previewRootRelative = 'SpinonPreview';
const backupRootRelative = `SpinonPreviewBackups/c11-4-positive-implicit-tailnet-sync-2026-10-20/${pass}`;
const expectedTargetRoot = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview');
const expectedBackupRoot = path.resolve(os.homedir(), `Library/Application Support/SpinonPreviewBackups/c11-4-positive-implicit-tailnet-sync-2026-10-20/${pass}`);
if (targetRoot !== expectedTargetRoot || backupRoot !== expectedBackupRoot) {
  throw new Error('target and backup paths must match the approved SpinonPreview locations');
}
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceWorktreeDirty = execFileSync('git', ['status', '--porcelain'], { cwd: repoRoot, encoding: 'utf8' }).length > 0;

function hash(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected build: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(fullPath, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`unsupported selected build entry: ${relative}`);
  }
  return files;
}

function assertSafeDirectoryPath(directory) {
  const parsed = path.parse(directory);
  let current = parsed.root;
  for (const segment of directory.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, segment);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in directory path: ${current}`);
      if (!stat.isDirectory()) throw new Error(`non-directory in directory path: ${current}`);
    } catch (error) {
      if (error.code === 'ENOENT') break;
      throw error;
    }
  }
}

assertSafeDirectoryPath(sourceRoot);
assertSafeDirectoryPath(targetRoot);
assertSafeDirectoryPath(backupRoot);

function inspectPath(root, relative) {
  const parts = relative.split('/');
  let current = root;
  try {
    const rootStat = fs.lstatSync(root);
    if (rootStat.isSymbolicLink() || !rootStat.isDirectory()) throw new Error(`unsafe root: ${root}`);
  } catch (error) {
    if (error.code !== 'ENOENT') throw error;
  }
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in destination path: ${relative}`);
      if (index < parts.length - 1 && !stat.isDirectory()) throw new Error(`non-directory in destination path: ${relative}`);
      if (index === parts.length - 1 && !stat.isFile()) throw new Error(`non-file destination: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), stat };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function buildPreflight() {
  const entries = listFiles(sourceRoot).map((relative) => {
    const source = fs.readFileSync(path.join(sourceRoot, relative));
    const prior = inspectPath(targetRoot, relative);
    const sourceHash = hash(source);
    const priorHash = prior.exists ? hash(prior.bytes) : null;
    const status = !prior.exists ? 'create' : priorHash === sourceHash ? 'same' : 'replace';
    return {
      path: relative,
      source_bytes: source.byteLength,
      source_sha256: sourceHash,
      target_exists: prior.exists,
      previous_bytes: prior.exists ? prior.bytes.byteLength : null,
      previous_sha256: priorHash,
      status,
      backup_path: status === 'replace' ? `${backupRootRelative}/${relative}` : null,
    };
  });
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [
    status, entries.filter((entry) => entry.status === status).length,
  ]));
  return {
    pass,
    source_root_relative: sourceRootRelative,
    source_branch: sourceBranch,
    source_worktree_head: sourceCommit,
    source_worktree_dirty_at_preflight: sourceWorktreeDirty,
    preview_root_relative: previewRootRelative,
    backup_root_relative: backupRootRelative,
    target_count: entries.length,
    total_bytes: entries.reduce((sum, entry) => sum + entry.source_bytes, 0),
    counts,
    no_deletes: true,
    files: entries,
  };
}

if (mode === 'preflight') {
  process.stdout.write(`${JSON.stringify(buildPreflight(), null, 2)}\n`);
  process.exit(0);
}

if (!preflightArg) throw new Error('copy mode requires the preflight JSON path');
const expected = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
if (expected.pass !== pass || expected.source_worktree_head !== sourceCommit
    || expected.preview_root_relative !== previewRootRelative
    || expected.backup_root_relative !== backupRootRelative) {
  throw new Error('preflight identity does not match this copy run');
}

const preflightByPath = new Map(expected.files.map((entry) => [entry.path, entry]));
const actualPaths = listFiles(sourceRoot);
if (actualPaths.length !== preflightByPath.size || actualPaths.some((relative) => !preflightByPath.has(relative))) {
  throw new Error('selected build file set changed after preflight');
}

const copyEntries = [];
const backupEntries = [];
for (const relative of actualPaths) {
  const entry = preflightByPath.get(relative);
  const sourcePath = path.join(sourceRoot, relative);
  const sourceBytes = fs.readFileSync(sourcePath);
  if (sourceBytes.byteLength !== entry.source_bytes || hash(sourceBytes) !== entry.source_sha256) {
    throw new Error(`selected build changed after preflight: ${relative}`);
  }
  const previous = inspectPath(targetRoot, relative);
  if (previous.exists !== entry.target_exists
      || (previous.exists && (previous.bytes.byteLength !== entry.previous_bytes
          || hash(previous.bytes) !== entry.previous_sha256))) {
    throw new Error(`preview target changed after preflight: ${relative}`);
  }

  if (entry.status === 'replace') {
    const backupRelative = entry.backup_path.slice(backupRootRelative.length + 1);
    const backupPath = path.join(backupRoot, backupRelative);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    if (fs.existsSync(backupPath)) {
      if (hash(fs.readFileSync(backupPath)) !== entry.previous_sha256) {
        throw new Error(`existing backup has different bytes: ${backupRelative}`);
      }
    } else {
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx', mode: previous.stat.mode & 0o777 });
    }
    const backupBytes = fs.readFileSync(backupPath);
    backupEntries.push({
      path: relative,
      backup_path: `${backupRootRelative}/${relative}`,
      bytes: backupBytes.byteLength,
      sha256: hash(backupBytes),
      expected_sha256: entry.previous_sha256,
      verified: backupBytes.byteLength === entry.previous_bytes && hash(backupBytes) === entry.previous_sha256,
    });
  }

  if (entry.status !== 'same') {
    const targetPath = path.join(targetRoot, relative);
    fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const temporaryPath = `${targetPath}.spinon-sync-${process.pid}`;
    fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx', mode: fs.statSync(sourcePath).mode & 0o777 });
    fs.renameSync(temporaryPath, targetPath);
  }
  const copied = inspectPath(targetRoot, relative);
  const copiedHash = copied.exists ? hash(copied.bytes) : null;
  copyEntries.push({
    path: relative,
    status: entry.status,
    source_bytes: sourceBytes.byteLength,
    target_bytes: copied.exists ? copied.bytes.byteLength : null,
    source_sha256: entry.source_sha256,
    target_sha256: copiedHash,
    verified: copied.exists && copied.bytes.byteLength === sourceBytes.byteLength && copiedHash === entry.source_sha256,
  });
}

const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [
  status, copyEntries.filter((entry) => entry.status === status).length,
]));
const copyVerification = {
  pass,
  preview_root_relative: previewRootRelative,
  target_count: copyEntries.length,
  counts,
  no_deletes: true,
  atomic_replacements: copyEntries.filter((entry) => entry.status !== 'same').length,
  all_verified: copyEntries.every((entry) => entry.verified),
  files: copyEntries,
};
const backupVerification = {
  pass,
  backup_root_relative: backupRootRelative,
  target_count: backupEntries.length,
  all_verified: backupEntries.every((entry) => entry.verified),
  files: backupEntries,
};
process.stdout.write(`${JSON.stringify({ copy_verification: copyVerification, backup_verification: backupVerification }, null, 2)}\n`);
