import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-calc-tailnet-sync-root-pass41-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-fit-content-calc-tailnet-sync-root-pass42-selected');
const evidenceDirectories = [
  'c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22',
  'c11-4-cyclic-calc-track-sizing-2026-10-22',
  'c11-4-cyclic-calc-track-sizing-runtime-2026-10-22-2026-10-22',
  'c11-4-cyclic-calc-nested-depth-regression-2026-10-22',
  'c11-4-cyclic-fit-content-calc-precomparison-2026-10-22',
];
const evidenceDestinations = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence'];
const selectedSourceCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['plan/c11-4-cyclic-calc-track-sizing.md', 'plan/c11-4-cyclic-calc-track-sizing.md'],
  ['plan/c11-4-cyclic-calc-track-sizing-plan-review-20.md', 'plan/c11-4-cyclic-calc-track-sizing-plan-review-20.md'],
  ['plan/c11-4-cyclic-fit-content-calc.md', 'plan/c11-4-cyclic-fit-content-calc.md'],
  ['plan/c11-4-cyclic-fit-content-calc-plan-review-20.md', 'plan/c11-4-cyclic-fit-content-calc-plan-review-20.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html', 'tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html'],
  ['tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html', 'public/tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html'],
  ['tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html', 'tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html'],
  ['tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html', 'public/tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html'],
];

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertPlainTree(directory) {
  const directoryStat = fs.lstatSync(directory);
  assert.ok(directoryStat.isDirectory() && !directoryStat.isSymbolicLink(), `directory is not a plain directory: ${directory}`);
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

function assertSameTree(source, target, label) {
  assertPlainTree(source);
  assertPlainTree(target);
  const sourceFiles = treeFiles(source);
  const targetFiles = treeFiles(target);
  assert.deepEqual(targetFiles, sourceFiles, `${label}: file set differs`);
  for (const relative of sourceFiles) {
    assert.deepEqual(
      fs.readFileSync(path.join(target, relative)),
      fs.readFileSync(path.join(source, relative)),
      `${label}: content differs at ${relative}`,
    );
  }
}

assert.ok(fs.statSync(baseRoot).isDirectory(), `pass41 selected root is missing: ${baseRoot}`);
assertPlainTree(baseRoot);
if (fs.existsSync(selectedRoot)) {
  const selectedStat = fs.lstatSync(selectedRoot);
  assert.ok(selectedStat.isDirectory() && !selectedStat.isSymbolicLink(), `unsafe existing pass42 selected root: ${selectedRoot}`);
  assertPlainTree(selectedRoot);
  const baseFiles = new Set(treeFiles(baseRoot));
  const selectedFiles = treeFiles(selectedRoot);
  const selectedSourceTargets = new Set(selectedSourceCopies.map(([, target]) => target));
  const selectedEvidenceTargets = evidenceDestinations.flatMap((destination) => evidenceDirectories.map((directory) => `${destination}/${directory}`));
  const mayDiffer = (relative) => selectedSourceTargets.has(relative)
    || selectedEvidenceTargets.some((target) => relative === target || relative.startsWith(`${target}/`));
  for (const relative of baseFiles) {
    if (mayDiffer(relative)) continue;
    assert.deepEqual(fs.readFileSync(path.join(selectedRoot, relative)), fs.readFileSync(path.join(baseRoot, relative)), `unexpected pass42 root mutation: ${relative}`);
  }
  for (const relative of selectedFiles) {
    assert.ok(baseFiles.has(relative) || mayDiffer(relative), `unexpected file in partial pass42 root: ${relative}`);
  }
} else {
  fs.cpSync(baseRoot, selectedRoot, { recursive: true, errorOnExist: true });
}
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
    if (fs.existsSync(target)) {
      const targetStat = fs.lstatSync(target);
      assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), `unsafe selected evidence: ${destination}/${directory}`);
      assertSameTree(source, target, `existing selected evidence differs: ${destination}/${directory}`);
    } else {
      fs.mkdirSync(path.dirname(target), { recursive: true });
      fs.cpSync(source, target, { recursive: true, errorOnExist: true });
    }
    copiedEvidence.push({ destination: `${destination}/${directory}`, source: `spec/internal/evidence/${directory}` });
  }
}

const selectedFiles = treeFiles(selectedRoot);
const selectedBytes = selectedFiles.reduce((sum, relative) => sum + fs.statSync(path.join(selectedRoot, relative)).size, 0);
const output = {
  pass: 'pass42',
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
const outputPath = path.join(evidenceDirectory, 'selected-root-preparation-pass42.json');
const outputBytes = Buffer.from(`${JSON.stringify(output, null, 2)}\n`);
if (fs.existsSync(outputPath)) assert.deepEqual(fs.readFileSync(outputPath), outputBytes, 'existing selected-root record differs');
else fs.writeFileSync(outputPath, outputBytes, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
