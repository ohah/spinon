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
if (!['preflight', 'copy'].includes(mode) || !['pass16', 'pass17', 'pass18'].includes(pass) || !sourceArg || !targetArg || !backupArg) {
  throw new Error('usage: SPINON_TAILNET_RUN=pass16|pass17|pass18 node sync-preview.mjs preflight|copy SOURCE TARGET BACKUP [PREFLIGHT_JSON]');
}
const sourceRoot = path.resolve(repoRoot, sourceArg);
const targetRoot = path.resolve(targetArg);
const backupRoot = path.resolve(backupArg);
const sourceRootRelative = path.relative(repoRoot, sourceRoot).split(path.sep).join('/');
const expectedSourceRootRelative = `build/spinon/c11-4-pr391-tailnet-site-${pass}`;
const targetExpected = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRootRelative = `SpinonPreviewBackups/c11-4-pr391-tailnet-sync-2026-10-20/${pass}`;
const backupExpected = path.resolve(os.homedir(), `Library/Application Support/SpinonPreviewBackups/c11-4-pr391-tailnet-sync-2026-10-20/${pass}`);
if (sourceRootRelative !== expectedSourceRootRelative || targetRoot !== targetExpected || backupRoot !== backupExpected) throw new Error('source, preview, or backup path does not match the approved pass-specific locations');

const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceWorktreeDirty = execFileSync('git', ['status', '--porcelain'], { cwd: repoRoot, encoding: 'utf8' }).length > 0;
const previewRootRelative = 'SpinonPreview';
const backupRootShort = `SpinonPreviewBackups/c11-4-pr391-tailnet-sync-2026-10-20/${pass}`;
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');

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
    } catch (error) { if (error.code !== 'ENOENT') throw error; }
  }
}
assertSafeDirectoryPath(sourceRoot); assertSafeDirectoryPath(targetRoot); assertSafeDirectoryPath(backupRoot);

function inspectPath(root, relative) {
  const parts = relative.split('/');
  let current = root;
  try {
    const stat = fs.lstatSync(root);
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new Error(`unsafe root: ${root}`);
  } catch (error) { if (error.code !== 'ENOENT') throw error; }
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in destination path: ${relative}`);
      if (index < parts.length - 1 && !stat.isDirectory()) throw new Error(`non-directory in destination path: ${relative}`);
      if (index === parts.length - 1 && !stat.isFile()) throw new Error(`non-file destination: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), stat };
    } catch (error) { if (error.code === 'ENOENT') return { exists: false }; throw error; }
  }
  return { exists: false };
}
function buildPreflight() {
  const entries = listFiles(sourceRoot).map((relative) => {
    const source = fs.readFileSync(path.join(sourceRoot, relative));
    const prior = inspectPath(targetRoot, relative);
    const sourceHash = hash(source); const priorHash = prior.exists ? hash(prior.bytes) : null;
    const status = !prior.exists ? 'create' : priorHash === sourceHash ? 'same' : 'replace';
    return { path: relative, source_bytes: source.byteLength, source_sha256: sourceHash, target_exists: prior.exists,
      previous_bytes: prior.exists ? prior.bytes.byteLength : null, previous_sha256: priorHash, status,
      backup_path: status === 'replace' ? `${backupRootShort}/${relative}` : null };
  });
  return { pass, source_root_relative: sourceRootRelative, source_branch: sourceBranch, source_worktree_head: sourceCommit,
    source_worktree_dirty_at_preflight: sourceWorktreeDirty, preview_root_relative: previewRootRelative,
    backup_root_relative: backupRootShort, target_count: entries.length, total_bytes: entries.reduce((sum, e) => sum + e.source_bytes, 0),
    counts: Object.fromEntries(['create', 'replace', 'same'].map((s) => [s, entries.filter((e) => e.status === s).length])),
    no_deletes: true, files: entries };
}
if (mode === 'preflight') { process.stdout.write(`${JSON.stringify(buildPreflight(), null, 2)}\n`); process.exit(0); }
if (!preflightArg) throw new Error('copy mode requires the preflight JSON path');
const expected = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
if (expected.pass !== pass || expected.source_worktree_head !== sourceCommit || expected.preview_root_relative !== previewRootRelative || expected.backup_root_relative !== backupRootShort) throw new Error('preflight identity does not match this copy run');
const byPath = new Map(expected.files.map((entry) => [entry.path, entry]));
const actualPaths = listFiles(sourceRoot);
if (actualPaths.length !== byPath.size || actualPaths.some((relative) => !byPath.has(relative))) throw new Error('selected build file set changed after preflight');
const copies = []; const backups = [];
for (const relative of actualPaths) {
  const entry = byPath.get(relative); const sourcePath = path.join(sourceRoot, relative); const source = fs.readFileSync(sourcePath);
  if (source.byteLength !== entry.source_bytes || hash(source) !== entry.source_sha256) throw new Error(`selected build changed after preflight: ${relative}`);
  const previous = inspectPath(targetRoot, relative);
  if (previous.exists !== entry.target_exists || (previous.exists && (previous.bytes.byteLength !== entry.previous_bytes || hash(previous.bytes) !== entry.previous_sha256))) throw new Error(`preview target changed after preflight: ${relative}`);
  if (entry.status === 'replace') {
    const backupRelative = entry.backup_path.slice(backupRootShort.length + 1); const backupPath = path.join(backupRoot, backupRelative);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    if (fs.existsSync(backupPath)) {
      if (hash(fs.readFileSync(backupPath)) !== entry.previous_sha256) throw new Error(`existing backup has different bytes: ${backupRelative}`);
    } else fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx', mode: previous.stat.mode & 0o777 });
    const saved = fs.readFileSync(backupPath); backups.push({ path: relative, backup_path: `${backupRootShort}/${relative}`, bytes: saved.byteLength, sha256: hash(saved), expected_sha256: entry.previous_sha256, verified: saved.byteLength === entry.previous_bytes && hash(saved) === entry.previous_sha256 });
  }
  if (entry.status !== 'same') {
    const targetPath = path.join(targetRoot, relative); fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const temporaryPath = `${targetPath}.spinon-sync-${process.pid}`;
    fs.writeFileSync(temporaryPath, source, { flag: 'wx', mode: fs.statSync(sourcePath).mode & 0o777 });
    fs.renameSync(temporaryPath, targetPath);
  }
  const copied = inspectPath(targetRoot, relative); const targetHash = copied.exists ? hash(copied.bytes) : null;
  copies.push({ path: relative, status: entry.status, source_bytes: source.byteLength, target_bytes: copied.exists ? copied.bytes.byteLength : null,
    source_sha256: entry.source_sha256, target_sha256: targetHash, verified: copied.exists && copied.bytes.byteLength === source.byteLength && targetHash === entry.source_sha256 });
}
process.stdout.write(`${JSON.stringify({ copy_verification: { pass, preview_root_relative: previewRootRelative, target_count: copies.length,
  counts: Object.fromEntries(['create', 'replace', 'same'].map((s) => [s, copies.filter((e) => e.status === s).length])), no_deletes: true,
  atomic_replacements: copies.filter((e) => e.status !== 'same').length, all_verified: copies.every((e) => e.verified), files: copies },
  backup_verification: { pass, backup_root_relative: backupRootShort, target_count: backups.length, all_verified: backups.every((e) => e.verified), files: backups } }, null, 2)}\n`);
