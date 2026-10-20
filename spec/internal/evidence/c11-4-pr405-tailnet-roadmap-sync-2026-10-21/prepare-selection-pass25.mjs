import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const baseRoot = path.join(repoRoot, 'build/spinon/c11-4-text-intrinsic-physical-tailnet-root-pass24');
const selectedRoot = path.join(repoRoot, 'build/spinon/c11-4-pr405-tailnet-root-pass25');
const evidenceName = 'c11-4-auto-fit-min-size-axis-physical-android-2026-10-21';
const physicalEvidence = path.join(repoRoot, 'spec/internal/evidence', evidenceName);

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
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

if (fs.existsSync(selectedRoot)) throw new Error(`selection output already exists: ${selectedRoot}`);
if (!fs.statSync(baseRoot).isDirectory()) throw new Error(`missing pass24 input: ${baseRoot}`);
const copied = copyTree(baseRoot, selectedRoot);
const revision = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();

const fileMappings = [
  { source: 'spec/STATUS.md', destinations: ['STATUS.md', 'spec/STATUS.md'] },
  { source: 'spec/public/roadmap.html', destinations: ['public/roadmap.html'] },
  {
    source: 'plan/c11-4-auto-fit-axis-physical-android.md',
    destinations: ['plan/c11-4-auto-fit-axis-physical-android.md', 'spec/plan/c11-4-auto-fit-axis-physical-android.md'],
  },
];

for (const mapping of fileMappings) {
  for (const destination of mapping.destinations) {
    copyTree(path.join(repoRoot, mapping.source), path.join(selectedRoot, destination), copied);
  }
}

for (const destination of [`internal/evidence/${evidenceName}`, `spec/internal/evidence/${evidenceName}`]) {
  copyTree(physicalEvidence, path.join(selectedRoot, destination), copied);
}

const manifest = {
  source_revision: revision,
  base_root_relative: path.relative(repoRoot, baseRoot).split(path.sep).join('/'),
  selected_root_relative: path.relative(repoRoot, selectedRoot).split(path.sep).join('/'),
  overlay_sources: [
    ...fileMappings.map(({ source }) => source),
    `spec/internal/evidence/${evidenceName}/**`,
  ],
  overlay_file_count: copied.length - countFiles(baseRoot),
  selected_file_count: listFiles(selectedRoot).length,
  overlays: copied.slice(countFiles(baseRoot)),
};
fs.writeFileSync(path.join(evidenceDir, 'selection-manifest-pass25.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  source_revision: revision,
  selected_root: manifest.selected_root_relative,
  base_file_count: countFiles(baseRoot),
  selected_file_count: manifest.selected_file_count,
  overlay_file_count: manifest.overlay_file_count,
  manifest_path: 'selection-manifest-pass25.json',
}, null, 2)}\n`);

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

function countFiles(directory) {
  return listFiles(directory).length;
}
