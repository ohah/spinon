import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-two-axis-multitrack-tailnet-sync-root-pass42-selected');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-fit-content-tailnet-sync-root-pass41-selected');
const baseline = '7db827461f2b2722034b9a84d9a7710129bcb356';
const sourceHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(sourceTree, mainTree, 'selected docs must be based on fetched origin/main');
assert.ok(fs.statSync(previousRoot).isDirectory(), 'pass41 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace an existing pass42 selected root');

const localChanges = execFileSync('git', ['diff', '--name-only', 'origin/main'], {
  cwd: repositoryRoot,
  encoding: 'utf8',
}).trim().split('\n').filter(Boolean).sort();
assert.deepEqual(localChanges, [
  'plan/c11-4-cyclic-two-axis-multitrack.md',
  'spec/public/roadmap.html',
].sort(), 'unexpected tracked source edits are present');
fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true });

function copyFile(sourceRelative, targetRelative = sourceRelative) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  assert.ok(fs.statSync(source).isFile(), `source file is missing: ${sourceRelative}`);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
}

const changedPaths = execFileSync('git', ['diff', '--name-only', baseline, 'origin/main'], {
  cwd: repositoryRoot,
  encoding: 'utf8',
}).trim().split('\n').filter(Boolean);
const changedPlanFiles = changedPaths.filter((file) => file.startsWith('plan/') && file.endsWith('.md'));
for (const file of changedPlanFiles) copyFile(file);
copyFile('plan/c11-4-cyclic-two-axis-multitrack.md');

const documentCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
];
for (const [source, target] of documentCopies) copyFile(source, target);

const evidenceDirectories = [...new Set(changedPaths
  .filter((file) => file.startsWith('spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-'))
  .map((file) => file.split('/').slice(0, 4).join('/')))].sort();
assert.deepEqual(evidenceDirectories, [
  'spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-precomparison-2026-10-22',
  'spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22',
]);
const evidenceAliases = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence', 'public/spec/internal/evidence'];
const copiedEvidenceDirectories = [];
for (const sourceDirectory of evidenceDirectories) {
  const sourceFiles = execFileSync('git', ['ls-files', '--', sourceDirectory], {
    cwd: repositoryRoot,
    encoding: 'utf8',
  }).trim().split('\n').filter(Boolean);
  assert.ok(sourceFiles.length > 0, `no tracked files under ${sourceDirectory}`);
  const directoryName = sourceDirectory.slice('spec/internal/evidence/'.length);
  for (const sourceFile of sourceFiles) {
    const suffix = path.relative(sourceDirectory, sourceFile).split(path.sep).join('/');
    for (const alias of evidenceAliases) copyFile(sourceFile, `${alias}/${directoryName}/${suffix}`);
  }
  copiedEvidenceDirectories.push({ directory: sourceDirectory, trackedFiles: sourceFiles.length });
}

const fixtureFiles = changedPaths.filter((file) => file.startsWith('tests/fixtures/css/c11/') && file.endsWith('.html'));
for (const file of fixtureFiles) copyFile(file);

const prepared = {
  pass: 'pass42',
  baseline,
  sourceHead,
  sourceTree,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  previousRoot: path.relative(repositoryRoot, previousRoot),
  localChanges,
  changedPlanFiles,
  copiedDocumentFiles: documentCopies.length,
  evidenceAliases,
  copiedEvidenceDirectories,
  copiedTrackedEvidenceFiles: copiedEvidenceDirectories.reduce((sum, entry) => sum + entry.trackedFiles, 0),
  copiedFixtureFiles: fixtureFiles,
  ignoredUntrackedWorkspaceEvidence: true,
  noPreviewWrites: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass42.json'), `${JSON.stringify(prepared, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(prepared, null, 2)}\n`);
