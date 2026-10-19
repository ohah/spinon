import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const evidenceRoot = path.relative(repoRoot, scriptDir).split(path.sep).join('/');
const targetRoot = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/c11-4-image-tailnet-preview-update-2026-10-20/pass8/wrong-staging-root');
const wrongCopyPath = path.join(scriptDir, 'copy-and-backup-pass8-wrong-staging-root.json');
const correctPreflightPath = path.join(scriptDir, 'preflight-pass8.json');

function hash(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertDirectoryPath(directory) {
  const parsed = path.parse(directory);
  let current = parsed.root;
  for (const segment of directory.slice(parsed.root.length).split(path.sep).filter(Boolean)) {
    current = path.join(current, segment);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink() || !stat.isDirectory()) throw new Error(`unsafe directory path: ${current}`);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      break;
    }
  }
}

function safeTargetPath(relative) {
  if (path.posix.isAbsolute(relative) || relative.split('/').some((part) => !part || part === '.' || part === '..')) {
    throw new Error(`unsafe preview path: ${relative}`);
  }
  let current = targetRoot;
  for (const [index, segment] of relative.split('/').entries()) {
    current = path.join(current, segment);
    const stat = fs.lstatSync(current);
    if (stat.isSymbolicLink()) throw new Error(`symlink in preview path: ${relative}`);
    if (index < relative.split('/').length - 1 && !stat.isDirectory()) {
      throw new Error(`non-directory component in preview path: ${relative}`);
    }
    if (index === relative.split('/').length - 1 && !stat.isFile()) {
      throw new Error(`non-file preview path: ${relative}`);
    }
  }
  return current;
}

assertDirectoryPath(targetRoot);
assertDirectoryPath(backupRoot);
const wrongCopy = JSON.parse(fs.readFileSync(wrongCopyPath, 'utf8')).copy_verification;
const correctPreflight = JSON.parse(fs.readFileSync(correctPreflightPath, 'utf8'));
if (wrongCopy.pass !== 'pass8' || correctPreflight.pass !== 'pass8'
    || correctPreflight.source_root_relative !== 'build/spinon/c11-4-image-tailnet-site-pass8') {
  throw new Error('pass8 source records do not match the expected correction');
}

const correctPaths = new Set(correctPreflight.files.map((entry) => entry.path));
const candidates = wrongCopy.files
  .filter((entry) => entry.status === 'create' && !correctPaths.has(entry.path))
  .sort((left, right) => left.path.localeCompare(right.path));
const targetFiles = candidates.map((entry) => {
  const target = safeTargetPath(entry.path);
  const bytes = fs.readFileSync(target);
  if (bytes.byteLength !== entry.target_bytes || hash(bytes) !== entry.source_sha256) {
    throw new Error(`preview file no longer matches the wrong-source copy record: ${entry.path}`);
  }
  const backup = path.join(backupRoot, entry.path);
  if (fs.existsSync(backup)) throw new Error(`refusing to replace an existing recovery backup: ${entry.path}`);
  return { entry, target, backup, bytes };
});

function inspectOpenFiles(files) {
  if (files.length === 0) return [];
  let output = '';
  let exitStatus = 0;
  try {
    output = execFileSync('lsof', ['-Fpcfn', '--', ...files.map((item) => item.target)], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    });
  } catch (error) {
    output = error.stdout ?? '';
    exitStatus = error.status ?? 1;
    if (exitStatus !== 1) throw error;
  }
  const expected = new Map(files.map((item) => [path.resolve(item.target), item.entry.path]));
  const descriptors = [];
  let processInfo = {};
  for (const line of output.split('\n')) {
    if (!line) continue;
    const field = line[0];
    const value = line.slice(1);
    if (field === 'p') processInfo = { pid: value };
    else if (field === 'c') processInfo.command = value;
    else if (field === 'f') processInfo.fd = value;
    else if (field === 'n') {
      const relative = expected.get(path.resolve(value));
      if (relative) descriptors.push({ ...processInfo, path: relative });
    }
  }
  return descriptors;
}

const initialOpenDescriptors = inspectOpenFiles(targetFiles);
const initialOpenPaths = new Set(initialOpenDescriptors.map((entry) => entry.path));
const closedCandidates = targetFiles.filter((item) => !initialOpenPaths.has(item.entry.path));
const backedUp = [];
for (const item of closedCandidates) {
  fs.mkdirSync(path.dirname(item.backup), { recursive: true });
  fs.writeFileSync(item.backup, item.bytes, { flag: 'wx', mode: 0o600 });
  const backupBytes = fs.readFileSync(item.backup);
  if (backupBytes.byteLength !== item.entry.target_bytes || hash(backupBytes) !== item.entry.source_sha256) {
    throw new Error(`recovery backup verification failed: ${item.entry.path}`);
  }
  backedUp.push(item);
}

const removed = [];
const freshOpenDescriptors = inspectOpenFiles(closedCandidates);
const freshOpenPaths = new Set(freshOpenDescriptors.map((entry) => entry.path));
const deleteCandidates = closedCandidates.filter((item) => !freshOpenPaths.has(item.entry.path));
try {
  for (const item of deleteCandidates) {
    const current = fs.readFileSync(item.target);
    if (current.byteLength !== item.entry.target_bytes || hash(current) !== item.entry.source_sha256) {
      throw new Error(`preview file changed before cleanup: ${item.entry.path}`);
    }
    fs.unlinkSync(item.target);
    if (fs.existsSync(item.target)) throw new Error(`preview file remains after cleanup: ${item.entry.path}`);
    removed.push(item.entry.path);
  }
} catch (error) {
  for (const item of deleteCandidates) {
    if (!fs.existsSync(item.target) && fs.existsSync(item.backup)) {
      fs.mkdirSync(path.dirname(item.target), { recursive: true });
      fs.copyFileSync(item.backup, item.target, fs.constants.COPYFILE_EXCL);
    }
  }
  throw error;
}

const result = {
  source: `${evidenceRoot}/copy-and-backup-pass8-wrong-staging-root.json`,
  correct_source_root: correctPreflight.source_root_relative,
  target_root_relative: 'SpinonPreview',
  recovery_backup_root_relative: 'SpinonPreviewBackups/c11-4-image-tailnet-preview-update-2026-10-20/pass8/wrong-staging-root',
  candidate_count: targetFiles.length,
  initially_open_path_count: initialOpenPaths.size,
  opened_before_delete_count: freshOpenPaths.size,
  total_bytes: targetFiles.reduce((sum, item) => sum + item.entry.target_bytes, 0),
  initially_open_descriptors: initialOpenDescriptors,
  opened_before_delete_descriptors: freshOpenDescriptors,
  recovery_backups_verified: backedUp.length,
  removed_count: removed.length,
  no_unbacked_removals: true,
  files: targetFiles.map((item) => ({
    path: item.entry.path,
    bytes: item.entry.target_bytes,
    sha256: item.entry.source_sha256,
    recovery_backup: backedUp.includes(item)
      ? path.relative(backupRoot, item.backup).split(path.sep).join('/')
      : null,
    skipped_open: initialOpenPaths.has(item.entry.path) || freshOpenPaths.has(item.entry.path),
    removed: removed.includes(item.entry.path),
  })),
};
process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
