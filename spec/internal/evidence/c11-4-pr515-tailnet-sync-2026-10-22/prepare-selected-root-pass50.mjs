import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-cross-axis-span-groups-tailnet-root-pass49-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr515-tailnet-root-pass50-selected');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const aliases = ['internal', 'spec/internal', 'public/internal', 'public/spec/internal'];

assert.ok(fs.lstatSync(previousRoot).isDirectory(), 'pass49 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace existing pass50 selected root');
fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true });

const overwrites = [];
function copy(sourceRelative, targetRelative, { allowExisting = false } = {}) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  assert.ok(fs.lstatSync(source).isFile(), `missing source file: ${sourceRelative}`);
  if (fs.existsSync(target)) {
    assert.ok(allowExisting, `refusing unexpected overwrite: ${targetRelative}`);
    overwrites.push({ path: targetRelative, previous_sha256: hash(fs.readFileSync(target)) });
  }
  fs.mkdirSync(path.dirname(target), { recursive: true });
  const bytes = fs.readFileSync(source);
  const textFile = path.extname(source).toLowerCase() !== '.png';
  const content = textFile
    ? Buffer.from(bytes.toString('utf8')
      .replaceAll('/Users/yoonhb/Documents/workspace/spinon', '[repository]')
      .replaceAll('/Users/yoonhb/Library/Android/sdk', '[android-sdk]')
      .replaceAll('/Users/yoonhb', '[user-home]'))
    : bytes;
  fs.writeFileSync(target, content);
  return { source: sourceRelative, target: targetRelative, bytes: content.byteLength, sha256: hash(content) };
}

const documentPairs = [
  ['spec/STATUS.md', 'STATUS.md', true],
  ['spec/STATUS.md', 'spec/STATUS.md', true],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md', true],
  ['spec/internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md', true],
  ['spec/internal/0059-c11-grid-layout.md', 'public/internal/0059-c11-grid-layout.md', true],
  ['spec/internal/0059-c11-grid-layout.md', 'public/spec/internal/0059-c11-grid-layout.md', true],
  ['spec/public/roadmap.html', 'roadmap.html', true],
  ['spec/public/roadmap.html', 'public/roadmap.html', true],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md', 'plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md', true],
];
const copiedDocuments = documentPairs.map(([source, target, allowExisting]) => copy(source, target, { allowExisting }));

const runtimeEvidence = 'spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22';
const runtimeFiles = ['README.md', 'implementation-runtime-review-20.md', 'pr-diff-review-30.md'];
const copiedRuntimeEvidence = [];
for (const alias of aliases) {
  for (const name of runtimeFiles) {
    copiedRuntimeEvidence.push(copy(`${runtimeEvidence}/${name}`, `${alias}/evidence/${path.basename(runtimeEvidence)}/${name}`, { allowExisting: true }));
  }
}

const regressionEvidence = 'spec/internal/evidence/c11-4-pr515-prereq-regression-2026-10-22';
const regressionFiles = [
  'README.md', 'matrix-summary.json', 'android-matrix-result.json', 'android-simulator.json',
  'android-intrinsic-forward.log', 'android-definite-forward.log',
  'android-anonymous-routing-failure-before-fix.log', 'android-anonymous-routing-after-fix.log',
  'android-swiftshader-watchdog-context.log', 'android-definite-forward.png',
  'ios-matrix-result.json', 'ios-simulator.json', 'ios-intrinsic-forward.log', 'ios-definite-forward.log',
  'ios-definite-forward.png', 'ios-simulator-build.log',
];
const copiedRegressionEvidence = [];
for (const alias of aliases) {
  for (const name of regressionFiles) {
    copiedRegressionEvidence.push(copy(`${regressionEvidence}/${name}`, `${alias}/evidence/${path.basename(regressionEvidence)}/${name}`));
  }
}

function listFiles(directory, prefix = '') {
  const entries = fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name));
  const files = [];
  for (const entry of entries) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected root: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(absolute, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`unsupported selected root entry: ${relative}`);
  }
  return files;
}
const selectedFiles = listFiles(selectedRoot);
const treeHash = createHash('sha256');
let bytesTotal = 0;
for (const relative of selectedFiles) {
  const bytes = fs.readFileSync(path.join(selectedRoot, relative));
  bytesTotal += bytes.byteLength;
  treeHash.update(relative).update('\0').update(hash(bytes)).update('\n');
}
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const manifest = {
  pass: 'pass50', sourceHead, previousRoot: path.relative(repositoryRoot, previousRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot), selectedFileCount: selectedFiles.length,
  selectedBytes: bytesTotal, selectedTreeSha256: treeHash.digest('hex'), copiedDocuments,
  copiedRuntimeEvidence, copiedRegressionEvidence, intentionalOverwrites: overwrites,
  ignoredUntrackedWorkspaceEvidence: true, noPreviewWrites: true, noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass50.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ pass: manifest.pass, sourceHead, selectedFileCount: manifest.selectedFileCount, selectedBytes: manifest.selectedBytes, selectedTreeSha256: manifest.selectedTreeSha256, documentCopies: copiedDocuments.length, runtimeEvidenceCopies: copiedRuntimeEvidence.length, regressionEvidenceCopies: copiedRegressionEvidence.length, intentionalOverwrites: overwrites.length, noPreviewWrites: true, noDeletes: true }, null, 2)}\n`);
