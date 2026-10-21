import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseCommit = '44c18e61d2328753726cd120eeca3cff465a29a4';
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-spanning-avd-tailnet-root-pass05-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-nested-grid-roadmap-root-pass01-selected');
const expectedHead = process.env.SPINON_TAILNET_SOURCE_SHA;

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}
function walk(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink is not allowed: ${relative}`);
    if (entry.isDirectory()) result.push(...walk(fullPath, relative));
    else {
      assert.ok(entry.isFile(), `unsupported filesystem entry: ${relative}`);
      const bytes = fs.readFileSync(fullPath);
      result.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    }
  }
  return result;
}
function targetsFor(sourceRelative) {
  if (sourceRelative.startsWith('plan/')) return [sourceRelative];
  if (sourceRelative === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (sourceRelative.startsWith('spec/public/')) return [`public/${sourceRelative.slice('spec/public/'.length)}`];
  if (sourceRelative.startsWith('spec/internal/evidence/')) {
    const suffix = sourceRelative.slice('spec/internal/evidence/'.length);
    return [`internal/evidence/${suffix}`, `spec/internal/evidence/${suffix}`, `public/internal/evidence/${suffix}`];
  }
  if (sourceRelative.startsWith('spec/internal/')) {
    const suffix = sourceRelative.slice('spec/internal/'.length);
    return [`internal/${suffix}`, `spec/internal/${suffix}`];
  }
  if (sourceRelative.startsWith('spec/plan/')) return [`spec/plan/${sourceRelative.slice('spec/plan/'.length)}`];
  if (sourceRelative.startsWith('spec/')) return [sourceRelative.slice('spec/'.length)];
  throw new Error(`unhandled tracked source path: ${sourceRelative}`);
}

assert.ok(expectedHead, 'SPINON_TAILNET_SOURCE_SHA is required');
assert.equal(execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), expectedHead, 'source commit changed');
assert.doesNotThrow(() => execFileSync('git', ['diff', '--quiet', 'HEAD', '--', 'plan', 'spec'], { cwd: repositoryRoot, stdio: 'ignore' }), 'tracked plan/spec changes must be committed');
assert.ok(fs.statSync(baseRoot).isDirectory(), 'pass05 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace an existing pass01 selected root');
const baseFiles = walk(baseRoot);
fs.cpSync(baseRoot, selectedRoot, { recursive: true, errorOnExist: true });

const changed = execFileSync('git', ['diff', '--name-status', '44c18e61d2328753726cd120eeca3cff465a29a4..HEAD', '--', 'plan', 'spec'], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean);
const copiedFiles = [];
for (const row of changed) {
  const [status, sourceRelative] = row.split('\t');
  assert.ok(!status.includes('D'), `deletions require explicit selected-site review: ${row}`);
  assert.ok(['A', 'M', 'T', 'R100', 'C100'].includes(status), `unexpected change status: ${row}`);
  assert.ok(sourceRelative && !sourceRelative.startsWith('/') && !sourceRelative.split('/').includes('..'), `unsafe source path: ${sourceRelative}`);
  const source = path.join(repositoryRoot, sourceRelative);
  assert.ok(fs.lstatSync(source).isFile(), `source is not a regular file: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  assert.deepEqual(bytes, execFileSync('git', ['show', `HEAD:${sourceRelative}`], { cwd: repositoryRoot, maxBuffer: 32 * 1024 * 1024 }), `workspace source differs from HEAD: ${sourceRelative}`);
  for (const targetRelative of targetsFor(sourceRelative)) {
    const target = path.join(selectedRoot, targetRelative);
    assert.ok(target.startsWith(`${selectedRoot}${path.sep}`), `unsafe selected path: ${targetRelative}`);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    const previous = fs.existsSync(target) ? fs.readFileSync(target) : null;
    const temp = `${target}.spinon-pass01-${process.pid}`;
    const descriptor = fs.openSync(temp, 'wx', 0o644);
    try {
      fs.writeFileSync(descriptor, bytes);
      fs.fsyncSync(descriptor);
    } finally {
      fs.closeSync(descriptor);
    }
    fs.renameSync(temp, target);
    const copied = fs.readFileSync(target);
    assert.deepEqual(copied, bytes, `selected file differs from HEAD: ${targetRelative}`);
    copiedFiles.push({ source: sourceRelative, target: targetRelative, action: previous === null ? 'create' : sha256(previous) === sha256(bytes) ? 'same' : 'replace', bytes: bytes.byteLength, sha256: sha256(bytes) });
  }
}

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const selectedFiles = walk(selectedRoot);
const treeHash = createHash('sha256');
for (const file of selectedFiles) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  pass: 'pass01', sourceHead, sourceTree, baseCommit, baseRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot), baseFileCount: baseFiles.length,
  changedSourceCount: changed.length, copiedTargetCount: copiedFiles.length,
  selectedFileCount: selectedFiles.length, selectedTreeSha256: treeHash.digest('hex'),
  copiedFiles, noPreviewWrites: true, noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass01.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...output, copiedFiles: undefined }, null, 2)}\n`);
