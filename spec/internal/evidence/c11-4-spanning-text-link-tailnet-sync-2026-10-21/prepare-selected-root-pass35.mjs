import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-spanning-text-tailnet-sync-root-pass35-selected');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-spanning-text-tailnet-sync-root-pass34-selected');
const mergedTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const currentTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(currentTree, mergedTree, 'selected docs source tree must exactly match origin/main');
assert.ok(fs.statSync(previousRoot).isDirectory(), 'pass34 selected root is missing');
const resumedExistingRoot = fs.existsSync(selectedRoot);
if (resumedExistingRoot) {
  assert.ok(fs.statSync(selectedRoot).isDirectory(), 'pass35 selected root is not a directory');
  assertSameTree(previousRoot, selectedRoot);
} else {
  execFileSync('cp', ['-cR', previousRoot, selectedRoot], { cwd: repositoryRoot, stdio: 'inherit' });
  assertSameTree(previousRoot, selectedRoot);
}

function treeHashes(directory, prefix = '') {
  const files = new Map();
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in evidence tree: ${relative}`);
    if (entry.isDirectory()) {
      for (const [childPath, hash] of treeHashes(fullPath, relative)) files.set(childPath, hash);
    } else if (entry.isFile()) {
      files.set(relative, createHash('sha256').update(fs.readFileSync(fullPath)).digest('hex'));
    } else {
      throw new Error(`unsupported evidence entry: ${relative}`);
    }
  }
  return files;
}

function assertSameTree(source, target) {
  const sourceFiles = treeHashes(source);
  const targetFiles = treeHashes(target);
  assert.deepEqual([...targetFiles], [...sourceFiles], `selected evidence differs: ${target}`);
}

function copyFile(sourceRelative, targetRelative) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
}

for (const [source, target] of [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-image-multispan-contributions.md', 'plan/c11-4-image-multispan-contributions.md'],
]) copyFile(source, target);

const existingEvidenceName = 'c11-4-full-matrix-physical-android-2026-10-21';
const evidenceName = 'c11-4-spanning-auto-min-flex-text-2026-10-21';
const destinations = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence'];
const existingEvidenceSource = path.join(repositoryRoot, 'spec/internal/evidence', existingEvidenceName);
for (const destination of destinations) {
  assertSameTree(existingEvidenceSource, path.join(selectedRoot, destination, existingEvidenceName));
}
const source = path.join(repositoryRoot, 'spec/internal/evidence', evidenceName);
assert.ok(fs.statSync(source).isDirectory(), `merged evidence is missing: ${evidenceName}`);
const previousEvidenceMirrors = destinations.map((destination) =>
  path.join(selectedRoot, destination, evidenceName));
for (const target of previousEvidenceMirrors.slice(1)) {
  assertSameTree(previousEvidenceMirrors[0], target);
}
for (const target of previousEvidenceMirrors) {
  fs.cpSync(source, target, { recursive: true, force: true });
  assertSameTree(source, target);
}

const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
process.stdout.write(`${JSON.stringify({
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  previousRoot: path.relative(repositoryRoot, previousRoot),
  sourceHead: mainHead,
  sourceTree: currentTree,
  statusCopies: ['STATUS.md', 'spec/STATUS.md'],
  roadmap: 'public/roadmap.html',
  plans: ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-image-multispan-contributions.md'],
  evidenceName,
  evidenceCopies: destinations.length,
  existingEvidenceVerified: existingEvidenceName,
  resumedExistingRoot,
  cloneMethod: resumedExistingRoot ? 'validated existing root against pass34' : 'cp -cR APFS clone plus full SHA-256 comparison',
  noPreviewWrites: true,
}, null, 2)}\n`);
