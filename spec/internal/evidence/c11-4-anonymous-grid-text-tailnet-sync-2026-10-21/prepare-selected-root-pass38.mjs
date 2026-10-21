import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-text-tailnet-sync-root-pass37-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-text-tailnet-sync-root-pass38-selected');
const evidenceDirectories = [
  'c11-4-anonymous-grid-text-precomparison-2026-10-21',
  'c11-4-anonymous-grid-text-2026-10-21',
];
const evidenceDestinations = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence'];
const selectedSourceCopies = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-anonymous-grid-text-items.md', 'plan/c11-4-anonymous-grid-text-items.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'],
  ['tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-precomparison.html', 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-precomparison.html'],
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

assert.ok(fs.statSync(baseRoot).isDirectory(), 'previous selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace an existing selected root');
for (const directory of evidenceDirectories) {
  assertPlainTree(path.join(repositoryRoot, 'spec/internal/evidence', directory));
}
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
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.cpSync(source, target, { recursive: true });
    copiedEvidence.push({ destination: `${destination}/${directory}`, source: `spec/internal/evidence/${directory}` });
  }
}

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const output = {
  pass: 'pass38',
  baseRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  sourceHead,
  sourceTree,
  selectedSourceCopies: copiedFiles,
  evidenceDirectories,
  evidenceDestinations,
  copiedEvidence,
  noPreviewWrites: true,
  noDeletes: true,
};
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
