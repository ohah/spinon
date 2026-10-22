import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
assert.ok(['pass26-initial', 'pass26-final'].includes(pass ?? ''), 'set pass26-initial or pass26-final');
const archiveRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass);
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass + '-selected');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-tailnet-root-pass46-selected');
const baseline = '48eaefe89d1eaad81cd355e961dadd56e0078c00';
const archiveMetadataPath = path.join(evidenceDirectory, process.argv[2] || ('selected-root-preparation-' + pass + '.json'));
const outputPath = path.join(evidenceDirectory, process.argv[3] || ('selected-docs-root-' + pass + '.json'));
const archiveMetadata = JSON.parse(fs.readFileSync(archiveMetadataPath, 'utf8'));
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', sourceCommit + '^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(archiveMetadata.sourceCommit, sourceCommit, 'archive must match committed branch HEAD');
assert.equal(archiveMetadata.sourceTree, sourceTree, 'archive tree must match committed branch tree');
assert.ok(fs.statSync(archiveRoot).isDirectory(), 'committed git archive is missing');
const previousRootInfo = JSON.parse(fs.readFileSync(path.join(
  repositoryRoot,
  'spec/internal/evidence/c11-4-cyclic-minmax-calc-tailnet-sync-2026-10-22/selected-root-preparation-pass46.json',
), 'utf8'));
assert.equal(previousRootInfo.sourceHead, baseline, 'pass46 selected root has a different source baseline');
assert.ok(fs.statSync(previousRoot).isDirectory(), 'pass46 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace existing selected docs root');

function scanTree(directory) {
  let files = 0;
  let bytes = 0;
  const symlinks = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) symlinks.push(path.relative(directory, fullPath));
    else if (entry.isDirectory()) {
      const child = scanTree(fullPath);
      files += child.files;
      bytes += child.bytes;
      symlinks.push(...child.symlinks.map((item) => path.join(entry.name, item)));
    } else if (entry.isFile()) {
      files += 1;
      bytes += fs.statSync(fullPath).size;
    } else {
      throw new Error('unsupported archive entry: ' + fullPath);
    }
  }
  return { files, bytes, symlinks };
}
const archiveInventory = scanTree(archiveRoot);
assert.deepEqual(archiveInventory.symlinks, [], 'committed archive contains symlinks');
const previousInventory = scanTree(previousRoot);
assert.deepEqual(previousInventory.symlinks, [], 'previous selected root contains symlinks');
fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true });

function copyFile(sourceRelative, targetRelative = sourceRelative) {
  const source = path.join(archiveRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'source file is not regular: ' + sourceRelative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
}

const changedPaths = execFileSync('git', ['diff', '--name-only', baseline, sourceCommit], {
  cwd: repositoryRoot,
  encoding: 'utf8',
}).trim().split(String.fromCharCode(10)).filter(Boolean);
const changedPlanFiles = changedPaths.filter((file) =>
  (file.startsWith('plan/c11-') || file === 'plan/c11-css-grid.md') && file.endsWith('.md'));
for (const file of changedPlanFiles) copyFile(file);

const documentCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/internal/README.md', 'internal/README.md'],
  ['spec/internal/README.md', 'spec/internal/README.md'],
  ['spec/internal/README.md', 'public/internal/README.md'],
  ['spec/internal/README.md', 'public/spec/internal/README.md'],
];
for (const [source, target] of documentCopies) copyFile(source, target);

const changedInternalDocs = changedPaths.filter((file) =>
  file.startsWith('spec/internal/') && !file.startsWith('spec/internal/evidence/')
  && file.endsWith('.md') && file !== 'spec/internal/README.md');
const internalAliases = ['internal', 'spec/internal', 'public/internal', 'public/spec/internal'];
for (const sourceFile of changedInternalDocs) {
  const suffix = sourceFile.slice('spec/internal/'.length);
  for (const alias of internalAliases) copyFile(sourceFile, alias + '/' + suffix);
}

const evidenceDirectories = [...new Set(changedPaths
  .filter((file) => file.startsWith('spec/internal/evidence/c11-4-'))
  .map((file) => file.split('/').slice(0, 4).join('/')))].sort();
const evidenceAliases = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence', 'public/spec/internal/evidence'];
const copiedEvidenceDirectories = [];
for (const sourceDirectory of evidenceDirectories) {
  const sourceFiles = execFileSync('git', ['ls-tree', '-r', '--name-only', sourceCommit, '--', sourceDirectory], {
    cwd: repositoryRoot,
    encoding: 'utf8',
  }).trim().split(String.fromCharCode(10)).filter(Boolean);
  assert.ok(sourceFiles.length > 0, 'no tracked files under ' + sourceDirectory);
  const directoryName = sourceDirectory.slice('spec/internal/evidence/'.length);
  for (const sourceFile of sourceFiles) {
    const suffix = path.relative(sourceDirectory, sourceFile).split(path.sep).join('/');
    for (const alias of evidenceAliases) copyFile(sourceFile, alias + '/' + directoryName + '/' + suffix);
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
        reason: '선택 문서 root의 Rspress route로 해석되는 운영용 build config 제외',
      });
      fs.rmSync(fullPath);
    }
  }
  return omitted;
}
const omittedOperationalConfigs = omitOperationalRspressConfigs(selectedRoot);
const selectedInventory = scanTree(selectedRoot);
assert.deepEqual(selectedInventory.symlinks, [], 'selected docs root contains symlinks');
const output = {
  schemaVersion: 1,
  pass,
  baseline,
  sourceCommit,
  sourceTree,
  sourceBranch: archiveMetadata.sourceBranch,
  archiveRoot: path.relative(repositoryRoot, archiveRoot).split(path.sep).join('/'),
  archiveFiles: archiveInventory.files,
  archiveBytes: archiveInventory.bytes,
  selectedRoot: path.relative(repositoryRoot, selectedRoot).split(path.sep).join('/'),
  selectedFiles: selectedInventory.files,
  selectedBytes: selectedInventory.bytes,
  changedPlanFiles,
  changedInternalDocs,
  copiedDocumentFiles: documentCopies.length,
  evidenceAliases,
  copiedEvidenceDirectories,
  copiedTrackedEvidenceFiles: copiedEvidenceDirectories.reduce((sum, entry) => sum + entry.trackedFiles, 0),
  copiedFixtureFiles: fixtureFiles,
  omittedOperationalConfigs,
  ignoredUntrackedWorkspaceEvidence: true,
  selection: 'previous committed docs root plus changed tracked C11.4 plans, STATUS, roadmap, internal index, evidence directories, and fixture pages since pass46',
  noPreviewWrites: true,
};
fs.writeFileSync(outputPath, JSON.stringify(output, null, 2) + String.fromCharCode(10), { flag: 'wx' });
process.stdout.write(JSON.stringify(output, null, 2) + String.fromCharCode(10));
