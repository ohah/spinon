import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-text-tailnet-sync-root-pass40-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-calc-tailnet-sync-root-pass41-selected');
const evidenceDirectories = [
  'c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22',
  'c11-4-cyclic-calc-track-sizing-2026-10-22',
  'c11-4-cyclic-calc-track-sizing-runtime-2026-10-22-2026-10-22',
  'c11-4-cyclic-calc-nested-depth-regression-2026-10-22',
];
const evidenceDestinations = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence'];
const selectedSourceCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['plan/c11-4-cyclic-calc-track-sizing.md', 'plan/c11-4-cyclic-calc-track-sizing.md'],
  ['plan/c11-4-cyclic-calc-track-sizing-plan-review-20.md', 'plan/c11-4-cyclic-calc-track-sizing-plan-review-20.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html', 'tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html'],
  ['tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html', 'public/tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html'],
];

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertPlainTree(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink is not allowed: ${fullPath}`);
    if (entry.isDirectory()) assertPlainTree(fullPath);
    else assert.ok(entry.isFile(), `unsupported filesystem entry: ${fullPath}`);
  }
}

function treeFiles(directory, prefix = '') {
  return fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))
    .flatMap((entry) => {
      const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
      return entry.isDirectory() ? treeFiles(path.join(directory, entry.name), relative) : [relative];
    });
}

assert.ok(fs.statSync(baseRoot).isDirectory(), `pass40 selected root is missing: ${baseRoot}`);
assert.ok(!fs.existsSync(selectedRoot), `refusing to replace existing pass41 selected root: ${selectedRoot}`);
assertPlainTree(baseRoot);
for (const directory of evidenceDirectories) {
  assertPlainTree(path.join(repositoryRoot, 'spec/internal/evidence', directory));
}
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(sourceTree, mainTree, 'workspace source tree does not match merged origin/main');
const trackedStatus = execFileSync('git', ['status', '--porcelain', '--untracked-files=no'], { cwd: repositoryRoot, encoding: 'utf8' });
assert.equal(trackedStatus.trim(), '', 'tracked workspace files must be clean before selecting source');
fs.cpSync(baseRoot, selectedRoot, { recursive: true, errorOnExist: true });

const copiedFiles = [];
for (const [sourceRelative, targetRelative] of selectedSourceCopies) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  assert.ok(fs.lstatSync(source).isFile(), `source file is missing: ${sourceRelative}`);
  assert.ok(!fs.existsSync(target) || fs.lstatSync(target).isFile(), `unsafe selected target: ${targetRelative}`);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  const bytes = fs.readFileSync(target);
  assert.deepEqual(bytes, fs.readFileSync(source));
  copiedFiles.push({ source: sourceRelative, target: targetRelative, bytes: bytes.byteLength, sha256: sha256(bytes) });
}

const copiedEvidence = [];
for (const destination of evidenceDestinations) {
  for (const directory of evidenceDirectories) {
    const source = path.join(repositoryRoot, 'spec/internal/evidence', directory);
    const target = path.join(selectedRoot, destination, directory);
    assert.ok(!fs.existsSync(target), `refusing to overwrite selected evidence: ${destination}/${directory}`);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.cpSync(source, target, { recursive: true, errorOnExist: true });
    copiedEvidence.push({ destination: `${destination}/${directory}`, source: `spec/internal/evidence/${directory}` });
  }
}

const selectedFiles = treeFiles(selectedRoot);
const selectedBytes = selectedFiles.reduce((sum, relative) => sum + fs.statSync(path.join(selectedRoot, relative)).size, 0);
const output = {
  pass: 'pass41',
  baseRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  sourceHead,
  sourceTree,
  originMainHead: mainHead,
  originMainTree: mainTree,
  selectedSourceCopies: copiedFiles,
  evidenceDirectories,
  evidenceDestinations,
  copiedEvidence,
  selectedFileCount: selectedFiles.length,
  selectedBytes,
  noPreviewWrites: true,
  noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass41.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
