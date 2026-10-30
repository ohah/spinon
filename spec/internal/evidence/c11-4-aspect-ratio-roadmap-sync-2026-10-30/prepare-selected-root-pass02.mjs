import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-aspect-ratio-roadmap-sync-root-pass02-20261030');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-aspect-ratio-roadmap-sync-site-pass02-20261030');
const reportPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace selected build output');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected-root report');
fs.mkdirSync(selectedRoot, { recursive: true });

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html'];
  if (relative === 'spec/public/icon.svg') return ['public/icon.svg'];
  if (relative === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (relative.startsWith('plan/')) return [relative, 'spec/' + relative];
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    return [withoutSpec, 'public/' + withoutSpec, 'public/' + relative];
  }
  throw new Error('source file has no selected route alias: ' + relative);
}

function copyFile(relative) {
  const sourcePath = path.join(repositoryRoot, relative);
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), 'unsafe source: ' + relative);
  const sourceBytes = fs.readFileSync(sourcePath);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    assert.ok(destination.startsWith(selectedRoot + path.sep), 'route escaped selected root: ' + alias);
    assert.ok(!fs.existsSync(destination), 'duplicate selected route: ' + alias);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, sourceBytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(sourceBytes), 'selected copy hash mismatch: ' + alias);
    entries.push({ source: relative, path: alias, bytes: copied.byteLength, sha256: sha256(copied) });
  }
}

function copyTree(relative) {
  const sourcePath = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(sourcePath);
  assert.ok(!stat.isSymbolicLink(), 'symlink source directory: ' + relative);
  if (stat.isFile()) return copyFile(relative);
  assert.ok(stat.isDirectory(), 'unsupported source entry: ' + relative);
  const names = fs.readdirSync(sourcePath).sort((a, b) => a.localeCompare(b));
  for (const name of names) copyTree(path.posix.join(relative, name));
}

const entries = [];
const sourcePaths = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/public/icon.svg',
  'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-4-direct-text-aspect-ratio.md',
  'plan/c11-4-direct-text-aspect-ratio-plan-review-20.md',
  'plan/c11-4-direct-text-aspect-ratio-plan-pr-diff-review-30.md',
  'plan/c11-4-direct-text-aspect-ratio-plan-follow-up-review-20.md',
  'plan/c11-4-grid-track-sizing.md',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-precomparison-2026-10-30',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30',
];;
for (const relative of sourcePaths) copyTree(relative);

const report = {
  sourceCommit,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  sourcePaths,
  fileCount: entries.length,
  totalBytes: entries.reduce((sum, entry) => sum + entry.bytes, 0),
  noDeletes: true,
  entries,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, entries: undefined }, null, 2) + '\n');
