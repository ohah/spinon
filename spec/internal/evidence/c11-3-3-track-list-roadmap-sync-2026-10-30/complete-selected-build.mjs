import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-3-3-track-list-roadmap-root-20261030');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-3-3-track-list-roadmap-site-20261030');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected documentation root missing');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'RSPress output missing');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected-build report');

function copyIfMissing(sourcePath, relativeOutput) {
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), 'unsafe selected source: ' + sourcePath);
  const sourceBytes = fs.readFileSync(sourcePath);
  const destination = path.join(outputRoot, relativeOutput);
  assert.ok(destination.startsWith(outputRoot + path.sep), 'output escaped selected site: ' + relativeOutput);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const previous = fs.readFileSync(destination);
    assert.equal(sha256(previous), sha256(sourceBytes), 'existing build asset differs: ' + relativeOutput);
    return;
  }
  fs.writeFileSync(destination, sourceBytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(destination)), sha256(sourceBytes), 'copied build asset hash mismatch');
}

copyIfMissing(path.join(selectedRoot, 'roadmap.svg'), 'roadmap.svg');
copyIfMissing(path.join(selectedRoot, 'public/icon.svg'), 'icon.svg');

const staticFiles = [
  'internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/expected.json',
  'internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/observation.json',
  'internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/fixture.png',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/result.json',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/android-track-list-cycle-screen.png',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/result.json',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/ios-track-list-cycle-screen.png',
];
for (const relative of staticFiles) copyIfMissing(path.join(selectedRoot, relative), relative);

const expectedRoutes = [
  'roadmap.html',
  'roadmap.svg',
  'icon.svg',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-3-3-template-area-implicit-tracks.html',
  'plan/c11-3-3-template-area-track-list-cycling.html',
  'internal/evidence/c11-3-3-template-area-track-list-cycling-implementation-2026-10-30/README.html',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/README.html',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/README.html',
  'internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/fixture.png',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/android-track-list-cycle-screen.png',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/ios-track-list-cycle-screen.png',
];
for (const route of expectedRoutes) assert.ok(fs.lstatSync(path.join(outputRoot, route)).isFile(), 'required route missing: ' + route);

function listFiles(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    if (relative === '404.html') continue;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in output: ' + relative);
    if (entry.isDirectory()) result.push(...listFiles(fullPath, relative));
    else if (entry.isFile()) {
      const bytes = fs.readFileSync(fullPath);
      result.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    } else throw new Error('unsupported output entry: ' + relative);
  }
  return result;
}

const files = listFiles(outputRoot);
const report = {
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  fileCount: files.length,
  totalBytes: files.reduce((sum, entry) => sum + entry.bytes, 0),
  expectedRoutes,
  excludedExistingPreviewRoute: '404.html',
  noDeletes: true,
  files,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, files: undefined }, null, 2) + '\n');
