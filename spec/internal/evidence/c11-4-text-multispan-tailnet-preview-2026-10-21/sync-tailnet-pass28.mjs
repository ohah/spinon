import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const [mode, preflightPath] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
const sourceRoot = path.join(repoRoot, 'build/spinon/c11-4-text-multispan-merge-tailnet-site-pass28');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-text-multispan-tailnet-preview-2026-10-21/pass28';
const targetRootRelative = 'SpinonPreview';
const backupRootRelative = 'SpinonPreviewBackups/c11-4-text-multispan-tailnet-preview-2026-10-21/pass28';
const previewServerPid = '1191';

if (!['preflight', 'copy', 'verify'].includes(mode) || pass !== 'pass28') {
  throw new Error('usage: SPINON_TAILNET_RUN=pass28 node sync-tailnet-pass28.mjs preflight|copy|verify [PREFLIGHT_JSON]');
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
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
  let current = root;
  const rootStat = fs.lstatSync(root);
  if (rootStat.isSymbolicLink() || !rootStat.isDirectory()) throw new Error(`unsafe root: ${root}`);
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      if (stat.isSymbolicLink()) throw new Error(`symlink in target path: ${relative}`);
      if (index < parts.length - 1 && !stat.isDirectory()) throw new Error(`non-directory in target path: ${relative}`);
      if (index === parts.length - 1 && !stat.isFile()) throw new Error(`non-file target: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), stat };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function summarize(entries) {
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [
    status, entries.filter((entry) => entry.status === status).length,
  ]));
  return {
    pass,
    source_root_relative: path.relative(repoRoot, sourceRoot).split(path.sep).join('/'),
    preview_root_relative: targetRootRelative,
    backup_root_relative: backupRootRelative,
    target_count: entries.length,
    source_bytes_total: entries.reduce((sum, entry) => sum + entry.source_bytes, 0),
    backup_bytes_total: entries.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
    counts,
    no_deletes: true,
    files: entries,
  };
}

if (mode === 'preflight') {
  const entries = listFiles(sourceRoot).map((relative) => {
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, relative));
    const previous = inspectPath(targetRoot, relative);
    const sourceHash = sha256(sourceBytes);
    const previousHash = previous.exists ? sha256(previous.bytes) : null;
    const status = !previous.exists ? 'create' : previousHash === sourceHash ? 'same' : 'replace';
    return {
      path: relative,
      status,
      source_bytes: sourceBytes.byteLength,
      source_sha256: sourceHash,
      target_exists: previous.exists,
      previous_bytes: previous.exists ? previous.bytes.byteLength : null,
      previous_sha256: previousHash,
      backup_path: status === 'replace' ? `${backupRootRelative}/${relative}` : null,
    };
  });
  process.stdout.write(`${JSON.stringify(summarize(entries), null, 2)}\n`);
  process.exit(0);
}

if (!preflightPath) throw new Error('copy mode requires a preflight JSON path');
const expected = JSON.parse(fs.readFileSync(path.resolve(evidenceDir, preflightPath), 'utf8'));
if (expected.pass !== pass || expected.preview_root_relative !== targetRootRelative
    || expected.backup_root_relative !== backupRootRelative) {
  throw new Error('preflight identity mismatch');
}

const byPath = new Map(expected.files.map((entry) => [entry.path, entry]));
const sourcePaths = listFiles(sourceRoot);
if (sourcePaths.length !== byPath.size || sourcePaths.some((relative) => !byPath.has(relative))) {
  throw new Error('selected file set changed after preflight');
}

if (mode === 'verify') {
  const targetResults = [];
  const backupVerification = [];
  for (const relative of sourcePaths) {
    const entry = byPath.get(relative);
    const sourceBytes = fs.readFileSync(path.join(sourceRoot, relative));
    const target = inspectPath(targetRoot, relative);
    const verified = target.exists && target.bytes.byteLength === entry.source_bytes
      && sha256(target.bytes) === entry.source_sha256;
    targetResults.push({ path: relative, bytes: target.bytes?.byteLength ?? null, sha256: target.exists ? sha256(target.bytes) : null, verified });
    if (entry.status === 'replace') {
      const backup = inspectPath(backupRoot, relative);
      const backupVerified = backup.exists && backup.bytes.byteLength === entry.previous_bytes
        && sha256(backup.bytes) === entry.previous_sha256;
      backupVerification.push({
        path: relative,
        backup_path: `${backupRootRelative}/${relative}`,
        bytes: backup.bytes?.byteLength ?? null,
        sha256: backup.exists ? sha256(backup.bytes) : null,
        expected_sha256: entry.previous_sha256,
        verified: backupVerified,
      });
    }
    if (sourceBytes.byteLength !== entry.source_bytes || sha256(sourceBytes) !== entry.source_sha256) {
      throw new Error(`selected build changed since preflight: ${relative}`);
    }
  }
  process.stdout.write(`${JSON.stringify({
    pass,
    target_count: targetResults.length,
    source_counts: expected.counts,
    all_targets_verified: targetResults.every((entry) => entry.verified),
    backup_count: backupVerification.length,
    all_backups_verified: backupVerification.every((entry) => entry.verified),
    no_deletes: true,
    targets: targetResults,
    backups: backupVerification,
  }, null, 2)}\n`);
  if (targetResults.some((entry) => !entry.verified) || backupVerification.some((entry) => !entry.verified)) process.exitCode = 1;
  process.exit();
}

const copyResults = [];
const backupResults = [];
const openHandleResults = [];
for (const relative of sourcePaths) {
  const expectedEntry = byPath.get(relative);
  const sourcePath = path.join(sourceRoot, relative);
  const sourceBytes = fs.readFileSync(sourcePath);
  if (sourceBytes.byteLength !== expectedEntry.source_bytes || sha256(sourceBytes) !== expectedEntry.source_sha256) {
    throw new Error(`selected build changed after preflight: ${relative}`);
  }

  const previous = inspectPath(targetRoot, relative);
  if (previous.exists !== expectedEntry.target_exists
      || (previous.exists && (previous.bytes.byteLength !== expectedEntry.previous_bytes
          || sha256(previous.bytes) !== expectedEntry.previous_sha256))) {
    throw new Error(`preview target changed after preflight: ${relative}`);
  }

  if (expectedEntry.status === 'replace') {
    const backupPath = path.join(backupRoot, relative);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    const backupExists = fs.existsSync(backupPath);
    if (backupExists) {
      const oldBackup = fs.readFileSync(backupPath);
      if (oldBackup.byteLength !== expectedEntry.previous_bytes || sha256(oldBackup) !== expectedEntry.previous_sha256) {
        throw new Error(`existing backup does not match preflight: ${relative}`);
      }
    } else {
      fs.writeFileSync(backupPath, previous.bytes, { flag: 'wx', mode: previous.stat.mode & 0o777 });
    }
    const backupBytes = fs.readFileSync(backupPath);
    const verified = backupBytes.byteLength === expectedEntry.previous_bytes
      && sha256(backupBytes) === expectedEntry.previous_sha256;
    if (!verified) throw new Error(`backup verification failed: ${relative}`);
    backupResults.push({
      path: relative,
      backup_path: `${backupRootRelative}/${relative}`,
      bytes: backupBytes.byteLength,
      sha256: sha256(backupBytes),
      verified,
      reused_after_interrupted_copy: backupExists,
    });
  }

  if (expectedEntry.status !== 'same') {
    const targetPath = path.join(targetRoot, relative);
    fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const temporaryPath = `${targetPath}.spinon-sync-${process.pid}`;
    if (fs.existsSync(temporaryPath)) throw new Error(`temporary path already exists: ${temporaryPath}`);

    if (expectedEntry.target_exists) {
      let openHandlePids = '';
      try {
        openHandlePids = execFileSync('lsof', ['-nP', '-t', targetPath], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
      } catch (error) {
        if (error.status !== 1) throw error;
        openHandlePids = String(error.stdout ?? '').trim();
      }
      if (openHandlePids) {
        const pids = [...new Set(openHandlePids.split(/\s+/).filter(Boolean))];
        if (pids.length !== 1 || pids[0] !== previewServerPid) {
          const detail = execFileSync('lsof', ['-nP', targetPath], { encoding: 'utf8' });
          throw new Error(`non-preview process holds target before rename: ${relative}\n${detail}`);
        }
        const args = execFileSync('ps', ['-p', previewServerPid, '-o', 'args='], { encoding: 'utf8' }).trim();
        if (!args.includes('tailnet-preview-server.mjs')) throw new Error(`unexpected preview PID command: ${args}`);
        const sockets = execFileSync('lsof', ['-nP', '-a', '-p', previewServerPid, '-i'], { encoding: 'utf8' }).trim().split('\n').slice(1).filter(Boolean);
        if (sockets.length !== 1 || !sockets[0].includes('127.0.0.1:8768') || !sockets[0].includes('(LISTEN)')) {
          throw new Error(`preview server has active network handles; refusing rename: ${sockets.join('\n')}`);
        }
        openHandleResults.push({
          path: relative,
          pid: previewServerPid,
          command: args,
          network_handles: sockets,
          disposition: 'atomic rename is safe: only the idle listener process holds the old inode; backup is verified and no client socket is active',
        });
      }
    }

    const descriptor = fs.openSync(temporaryPath, 'wx', fs.statSync(sourcePath).mode & 0o777);
    try {
      fs.writeFileSync(descriptor, sourceBytes);
      fs.fsyncSync(descriptor);
    } finally {
      fs.closeSync(descriptor);
    }
    fs.renameSync(temporaryPath, targetPath);
  }

  const copied = inspectPath(targetRoot, relative);
  const verified = copied.exists && copied.bytes.byteLength === sourceBytes.byteLength
    && sha256(copied.bytes) === expectedEntry.source_sha256;
  if (!verified) throw new Error(`target verification failed: ${relative}`);
  copyResults.push({ path: relative, status: expectedEntry.status, bytes: copied.bytes.byteLength, sha256: sha256(copied.bytes), verified });
}

const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [
  status, copyResults.filter((entry) => entry.status === status).length,
]));
process.stdout.write(`${JSON.stringify({
  pass,
  preview_root_relative: targetRootRelative,
  target_count: copyResults.length,
  counts,
  no_deletes: true,
  all_verified: copyResults.every((entry) => entry.verified),
  backup_count: backupResults.length,
  all_backups_verified: backupResults.every((entry) => entry.verified),
  open_handle_observations: openHandleResults,
  files: copyResults,
  backups: backupResults,
}, null, 2)}\n`);
