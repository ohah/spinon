import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const selectedRoot = path.join(repoRoot, 'build/spinon/c11-4-direct-text-minmax-roadmap-root-20261030');
const outputRoot = path.join(repoRoot, 'build/spinon/c11-4-direct-text-minmax-roadmap-site-20261030');
const reportPath = path.join(evidenceDir, 'selected-root.json');
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.equal(sourceHead, execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repoRoot, encoding: 'utf8' }).trim(), 'selected source must be merged origin/main');
assert.ok(!execFileSync('git', ['status', '--porcelain', '--untracked-files=no'], { cwd: repoRoot, encoding: 'utf8' }).trim(), 'tracked source changes must be committed');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace selected build output');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected-root report');
fs.mkdirSync(selectedRoot, { recursive: false });

const entries = [];
function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html'];
  if (relative === 'spec/public/roadmap.svg') return ['roadmap.svg', 'public/roadmap.svg'];
  if (relative === 'spec/public/icon.svg') return ['public/icon.svg'];
  if (relative === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (relative.startsWith('plan/')) return [relative, 'spec/' + relative];
  if (relative.startsWith('tests/')) return [relative];
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    return [withoutSpec, 'public/' + withoutSpec, 'public/' + relative];
  }
  throw new Error('source path has no route aliases: ' + relative);
}

function copyFile(relative) {
  const sourcePath = path.join(repoRoot, relative);
  const stat = fs.lstatSync(sourcePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe source file: ' + relative);
  const source = fs.readFileSync(sourcePath);
  for (const route of aliases(relative)) {
    const destination = path.join(selectedRoot, route);
    assert.ok(destination.startsWith(selectedRoot + path.sep), 'route escaped selected root: ' + route);
    assert.ok(!fs.existsSync(destination), 'duplicate selected route: ' + route);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, source, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(source), 'selected copy hash mismatch: ' + route);
    entries.push({ source: relative, path: route, bytes: copied.byteLength, sha256: sha256(copied) });
  }
}

function copyTree(relative) {
  const sourcePath = path.join(repoRoot, relative);
  const stat = fs.lstatSync(sourcePath);
  assert.ok(!stat.isSymbolicLink(), 'symlink in selected source: ' + relative);
  if (stat.isFile()) return copyFile(relative);
  assert.ok(stat.isDirectory(), 'unsupported selected source: ' + relative);
  for (const name of fs.readdirSync(sourcePath).sort((a, b) => a.localeCompare(b))) {
    copyTree(path.posix.join(relative, name));
  }
}

const sourcePaths = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/public/roadmap.svg',
  'spec/public/icon.svg',
  'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-direct-text-aspect-ratio-min-max.md',
  'tests/fixtures/css/c11/grid-direct-text-aspect-ratio-min-max-precomparison.html',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.md',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/minmax-implementation-runtime-review-20.md',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/artifact-boundary/minmax-archive-boundary.json',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvut1ldd-e32f3c4c/result.json',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvut1ldd-e32f3c4c/grid-text-aspect-ratio-minmax-min-width-45.png',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/ios-simulator/c114-text-ratio-mvut4y8z-74dc1d74/result.json',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/ios-simulator/c114-text-ratio-mvut4y8z-74dc1d74/grid-text-aspect-ratio-minmax-max-height-17-75-min-controls.png',
];
for (const relative of sourcePaths) copyTree(relative);

const report = {
  sourceHead,
  sourceTree,
  selectedRoot: path.relative(repoRoot, selectedRoot),
  sourcePaths,
  routeCount: entries.length,
  totalBytes: entries.reduce((sum, entry) => sum + entry.bytes, 0),
  noDeletes: true,
  entries,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, entries: undefined }, null, 2) + '\n');
