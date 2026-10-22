import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-tailnet-root-pass46-selected');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-fit-content-calc-tailnet-root-pass45-selected');
const baseline = '2f6876c62985a5e8cccd62d34c9fb7ad854a9bde';
const sourceHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const workTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(sourceTree, workTree, 'selected docs must use the fetched origin/main tree');
assert.ok(fs.statSync(previousRoot).isDirectory(), 'pass45 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace an existing pass46 selected root');

const localChanges = execFileSync('git', ['diff', '--name-only', 'origin/main'], {
  cwd: repositoryRoot,
  encoding: 'utf8',
}).trim().split('\n').filter(Boolean).sort();
assert.deepEqual(localChanges, [], 'tracked workspace edits differ from origin/main');
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
const changedPlanFiles = changedPaths.filter((file) =>
  (file.startsWith('plan/c11-') || file === 'plan/c11-css-grid.md') && file.endsWith('.md'));
for (const file of changedPlanFiles) copyFile(file);

const documentCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
];
for (const [source, target] of documentCopies) copyFile(source, target);

const evidenceDirectories = [...new Set(changedPaths
  .filter((file) => file.startsWith('spec/internal/evidence/c11-4-'))
  .map((file) => file.split('/').slice(0, 4).join('/')))].sort();
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

function omitOperationalRspressConfigs(directory, prefix = '') {
  const omitted = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      omitted.push(...omitOperationalRspressConfigs(fullPath, relative));
    } else if (entry.isFile() && entry.name.startsWith('rspress') && entry.name.endsWith('.config.ts')) {
      const bytes = fs.readFileSync(fullPath);
      omitted.push({
        path: relative,
        bytes: bytes.byteLength,
        sha256: createHash('sha256').update(bytes).digest('hex'),
        reason: '선택 문서 root에서 Rspress route로 해석되는 운영용 build config 제외',
      });
      fs.rmSync(fullPath);
    }
  }
  return omitted;
}
const omittedOperationalConfigs = omitOperationalRspressConfigs(selectedRoot);
const prepared = {
  pass: 'pass46',
  baseline,
  sourceHead,
  sourceTree,
  workTree,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  previousRoot: path.relative(repositoryRoot, previousRoot),
  localChanges,
  changedPlanFiles,
  copiedDocumentFiles: documentCopies.length,
  evidenceAliases,
  copiedEvidenceDirectories,
  copiedTrackedEvidenceFiles: copiedEvidenceDirectories.reduce((sum, entry) => sum + entry.trackedFiles, 0),
  copiedFixtureFiles: fixtureFiles,
  omittedOperationalConfigs,
  ignoredUntrackedWorkspaceEvidence: true,
  noPreviewWrites: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass46.json'), `${JSON.stringify(prepared, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(prepared, null, 2)}\n`);
