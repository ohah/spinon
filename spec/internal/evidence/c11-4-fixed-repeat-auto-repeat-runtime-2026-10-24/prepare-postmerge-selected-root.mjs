import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-fixed-repeat-auto-repeat-root-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-fixed-repeat-auto-repeat-root-postmerge-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-fixed-repeat-auto-repeat-site-postmerge-selected');
const manifestPath = path.join(evidenceDirectory, 'postmerge-selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'pass01 selected source tree is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace post-merge selected source tree');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace post-merge RSPress output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite post-merge root manifest');

const sourceFiles = [
  'spec/public/roadmap.html',
  'spec/STATUS.md',
  'plan/c11-4-auto-repeat-fixed-repeat.md',
  'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/README.md',
  'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/roadmap-pr-diff-review-30.md',
];

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected base: ${sourcePath}`);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error(`unsupported selected base entry: ${sourcePath}`);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html'];
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative.startsWith('spec/internal/')) targets.add(relative.slice('spec/'.length));
  return [...targets];
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of sourceFiles) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${relative}`);
  const bytes = fs.readFileSync(source);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const existing = fs.lstatSync(destination);
      assert.ok(existing.isFile() && !existing.isSymbolicLink(), `unsafe overlay target: ${alias}`);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `overlay hash mismatch: ${alias}`);
    overlays.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const manifest = {
  baseRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  sourceFiles,
  overlays,
  storageMethod: 'hardlink prior selected tree; unlink overlay destinations before copying current source bytes',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ sourceFileCount: sourceFiles.length, overlayCount: overlays.length, noDeletes: true }, null, 2)}\n`);
