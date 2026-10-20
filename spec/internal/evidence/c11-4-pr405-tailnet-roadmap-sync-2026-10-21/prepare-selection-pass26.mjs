import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const baseRevision = '24267943981a9c7dee693155efb8607bcf8e405c';
const baseRoot = path.join(repoRoot, 'build/spinon/c11-4-text-intrinsic-physical-tailnet-root-pass24');
const selectedRoot = path.join(repoRoot, 'build/spinon/c11-4-pr405-tailnet-root-pass26');

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected tree: ${relative}`);
    if (entry.isDirectory()) paths.push(...listFiles(fullPath, relative));
    else if (entry.isFile()) paths.push(relative);
    else throw new Error(`unsupported selected tree entry: ${relative}`);
  }
  return paths;
}

function copyTree(source, destination, copied = []) {
  const stat = fs.lstatSync(source);
  if (stat.isSymbolicLink()) throw new Error(`refusing symlink: ${source}`);
  if (stat.isDirectory()) {
    fs.mkdirSync(destination, { recursive: true, mode: stat.mode & 0o777 });
    for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      copyTree(path.join(source, entry.name), path.join(destination, entry.name), copied);
    }
    return copied;
  }
  if (!stat.isFile()) throw new Error(`unsupported input: ${source}`);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination);
  fs.chmodSync(destination, stat.mode & 0o777);
  const bytes = fs.readFileSync(destination);
  copied.push({ path: path.relative(selectedRoot, destination).split(path.sep).join('/'), bytes: bytes.byteLength, sha256: sha256(bytes) });
  return copied;
}

function destinationsFor(sourcePath) {
  if (sourcePath === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (sourcePath === 'spec/public/roadmap.html') return ['public/roadmap.html'];
  if (sourcePath.startsWith('spec/internal/')) {
    return [`internal/${sourcePath.slice('spec/internal/'.length)}`, sourcePath];
  }
  if (sourcePath.startsWith('spec/plan/')) {
    return [`plan/${sourcePath.slice('spec/plan/'.length)}`, sourcePath];
  }
  if (sourcePath.startsWith('plan/')) return [sourcePath, `spec/${sourcePath}`];
  if (sourcePath.startsWith('spec/')) {
    const relative = sourcePath.slice('spec/'.length);
    return [relative, sourcePath];
  }
  return [];
}

if (fs.existsSync(selectedRoot)) throw new Error(`selection output already exists: ${selectedRoot}`);
if (!fs.statSync(baseRoot).isDirectory()) throw new Error(`missing pass24 input: ${baseRoot}`);
const currentRevision = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const changedPaths = execFileSync('git', ['diff', '--name-only', baseRevision, currentRevision, '--', 'spec', 'plan'], {
  cwd: repoRoot,
  encoding: 'utf8',
}).trim().split('\n').filter((sourcePath) => sourcePath
  && !sourcePath.startsWith('spec/internal/evidence/c11-4-pr405-tailnet-roadmap-sync-2026-10-21/'));

const copied = copyTree(baseRoot, selectedRoot);
const baseFileCount = copied.length;
const overlays = [];
const deletedSourcePaths = [];
for (const sourcePath of changedPaths) {
  const source = path.join(repoRoot, sourcePath);
  if (!fs.existsSync(source)) {
    deletedSourcePaths.push(sourcePath);
    continue;
  }
  const destinations = destinationsFor(sourcePath);
  if (destinations.length === 0) throw new Error(`unmapped changed documentation path: ${sourcePath}`);
  for (const destination of destinations) {
    const before = copied.length;
    copyTree(source, path.join(selectedRoot, destination), copied);
    for (const file of copied.slice(before)) overlays.push({ source: sourcePath, ...file });
  }
}

const manifest = {
  base_revision: baseRevision,
  source_revision: currentRevision,
  base_root_relative: path.relative(repoRoot, baseRoot).split(path.sep).join('/'),
  selected_root_relative: path.relative(repoRoot, selectedRoot).split(path.sep).join('/'),
  changed_source_path_count: changedPaths.length,
  deleted_source_paths_not_removed_from_preview: deletedSourcePaths,
  overlay_destination_file_count: overlays.length,
  base_file_count: baseFileCount,
  selected_file_count: listFiles(selectedRoot).length,
  overlays,
};
fs.writeFileSync(path.join(evidenceDir, 'selection-manifest-pass26.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  base_revision: baseRevision,
  source_revision: currentRevision,
  changed_source_path_count: changedPaths.length,
  deleted_source_paths_not_removed_from_preview: deletedSourcePaths,
  overlay_destination_file_count: overlays.length,
  base_file_count: baseFileCount,
  selected_file_count: manifest.selected_file_count,
  manifest_path: 'selection-manifest-pass26.json',
}, null, 2)}\n`);
