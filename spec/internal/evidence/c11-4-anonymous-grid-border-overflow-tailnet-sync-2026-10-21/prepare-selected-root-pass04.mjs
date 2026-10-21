import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseCommit = '4adbdccb5af4a688f8e153c127c9892c5aaed299';
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-text-tailnet-sync-root-pass40-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-border-overflow-tailnet-root-pass04-selected');

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

function targetsFor(sourceRelative) {
  if (sourceRelative.startsWith('plan/')) return [`${sourceRelative}`];
  if (sourceRelative === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (sourceRelative.startsWith('spec/public/')) {
    return [`public/${sourceRelative.slice('spec/public/'.length)}`];
  }
  if (sourceRelative.startsWith('spec/internal/evidence/')) {
    const suffix = sourceRelative.slice('spec/internal/evidence/'.length);
    return [`internal/evidence/${suffix}`, `spec/internal/evidence/${suffix}`, `public/internal/evidence/${suffix}`];
  }
  if (sourceRelative.startsWith('spec/internal/')) {
    const suffix = sourceRelative.slice('spec/internal/'.length);
    return [`internal/${suffix}`, `spec/internal/${suffix}`];
  }
  if (sourceRelative.startsWith('spec/plan/')) {
    return [`spec/plan/${sourceRelative.slice('spec/plan/'.length)}`];
  }
  if (sourceRelative.startsWith('spec/')) return [sourceRelative.slice('spec/'.length)];
  throw new Error(`unhandled tracked source path: ${sourceRelative}`);
}

assert.ok(fs.statSync(baseRoot).isDirectory(), 'pass40 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace an existing pass04 selected root');
assertPlainTree(baseRoot);
assert.equal(execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), process.env.SPINON_TAILNET_SOURCE_SHA, 'source commit must match the recorded SHA');
assert.doesNotThrow(() => execFileSync('git', ['diff', '--quiet', 'HEAD', '--', 'plan', 'spec'], { cwd: repositoryRoot, stdio: 'ignore' }));
fs.cpSync(baseRoot, selectedRoot, { recursive: true, errorOnExist: true });

const changedPaths = execFileSync('git', [
  'diff', '--name-only', '--diff-filter=ACMRT', `${baseCommit}..HEAD`, '--', 'plan', 'spec',
], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean);
const copiedFiles = [];
for (const sourceRelative of changedPaths) {
  assert.ok(!sourceRelative.startsWith('/') && !sourceRelative.split('/').includes('..'), `unsafe source path: ${sourceRelative}`);
  const source = path.join(repositoryRoot, sourceRelative);
  assert.ok(fs.lstatSync(source).isFile(), `source file is missing: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const gitBytes = execFileSync('git', ['show', `HEAD:${sourceRelative}`], { cwd: repositoryRoot, maxBuffer: 16 * 1024 * 1024 });
  assert.deepEqual(bytes, gitBytes, `workspace source differs from merged main: ${sourceRelative}`);
  for (const targetRelative of targetsFor(sourceRelative)) {
    const target = path.join(selectedRoot, targetRelative);
    assert.ok(target.startsWith(`${selectedRoot}${path.sep}`), `unsafe selected path: ${targetRelative}`);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    const priorExists = fs.existsSync(target);
    if (priorExists) assert.ok(fs.lstatSync(target).isFile(), `unsafe selected target: ${targetRelative}`);
    const previousBytes = priorExists ? fs.readFileSync(target) : null;
    const temp = `${target}.spinon-pass04-${process.pid}`;
    const descriptor = fs.openSync(temp, 'wx', 0o644);
    try {
      fs.writeFileSync(descriptor, bytes);
      fs.fsyncSync(descriptor);
    } finally {
      fs.closeSync(descriptor);
    }
    fs.renameSync(temp, target);
    const copied = fs.readFileSync(target);
    assert.deepEqual(copied, bytes, `selected target differs from main source: ${targetRelative}`);
    copiedFiles.push({
      source: sourceRelative,
      target: targetRelative,
      status: !priorExists ? 'create' : sha256(previousBytes) === sha256(bytes) ? 'same' : 'replace',
      bytes: bytes.byteLength,
      sha256: sha256(bytes),
    });
  }
}

const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const tree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const output = {
  pass: 'pass04',
  sourceHead: head,
  sourceTree: tree,
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  baseContentCommit: baseCommit,
  changedSourceCount: changedPaths.length,
  copiedTargetCount: copiedFiles.length,
  copiedFiles,
  noPreviewWrites: true,
  noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass04.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...output, copiedFiles: undefined }, null, 2)}\n`);
