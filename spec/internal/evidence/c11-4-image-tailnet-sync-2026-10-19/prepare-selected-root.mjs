import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (!/^pass3$|^pass4$|^pass5$/.test(run ?? '')) throw new Error('usage: node prepare-selected-root.mjs pass3|pass4|pass5');

const specRoot = path.join(repoRoot, 'spec');
const outputRoot = path.join(repoRoot, 'build/spinon', `c11-4-image-tailnet-root-${run}`);
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root for ${run}`);

const selected = [
  ['spec/0001-conformance.md', '0001-conformance.md'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/README.md', 'internal/README.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/public/roadmap.svg', 'public/roadmap.svg'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19', 'internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19'],
  ['spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19', 'internal/evidence/c11-4-image-tailnet-sync-2026-10-19'],
];

const staticEvidenceExtensions = new Set([
  '.bin', '.css', '.csv', '.html', '.json', '.log', '.mjs', '.pdf', '.png', '.sh', '.svg', '.tar', '.ts', '.txt', '.webp',
]);
function copySelected(sourceRelative, destinationRelative) {
  if (sourceRelative.endsWith(`/prepare-root-${run}.json`)) return;
  const source = path.join(repoRoot, sourceRelative);
  if (!fs.existsSync(source)) throw new Error(`missing selected source: ${sourceRelative}`);
  const stat = fs.lstatSync(source);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected source: ${sourceRelative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(source).sort()) {
      copySelected(`${sourceRelative}/${name}`, `${destinationRelative}/${name}`);
    }
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected source: ${sourceRelative}`);
  const isEvidenceAsset = destinationRelative.startsWith('internal/evidence/')
    && path.extname(source) !== '.md'
    && staticEvidenceExtensions.has(path.extname(source));
  const stagedRelative = isEvidenceAsset ? path.join('public', destinationRelative) : destinationRelative;
  const destination = path.join(outputRoot, stagedRelative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination);
  fs.chmodSync(destination, stat.mode & 0o777);
}

for (const [sourceRelative, destinationRelative] of selected) {
  copySelected(sourceRelative, destinationRelative);
}

const textExtensions = new Set(['.md', '.html', '.json', '.log', '.txt', '.mjs', '.ts', '.sh']);
const redacted = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      visit(fullPath);
      continue;
    }
    if (!entry.isFile() || !textExtensions.has(path.extname(entry.name))) continue;
    const before = fs.readFileSync(fullPath, 'utf8');
    const after = before
      .replace(/\/Users\/[^/\s"'<>:,)}\]]+(?:\/[^\s"'<>:,)}\]]*)*/g, '[host-path-redacted]')
      .replace(/\/private\/tmp(?:\/[^\s"'<>:,)}\]]*)*/g, '[worktree-path-redacted]');
    if (after !== before) {
      fs.writeFileSync(fullPath, after, 'utf8');
      redacted.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
    }
  }
}
visit(outputRoot);

const files = [];
function collect(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) collect(fullPath);
    else if (entry.isFile()) files.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
  }
}
collect(outputRoot);
files.sort();
process.stdout.write(`${JSON.stringify({
  run,
  source_branch: sourceBranch,
  source_commit: sourceCommit,
  output_root_relative: path.relative(repoRoot, outputRoot).split(path.sep).join('/'),
  selected_sources: selected.map(([source, destination]) => ({ source, destination })),
  file_count: files.length,
  redacted_file_count: redacted.length,
  redacted_files: redacted.sort(),
  files,
}, null, 2)}\n`);
