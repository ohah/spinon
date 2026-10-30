import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-grid-text-first-baseline-roadmap-root-pass03-20261030');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-grid-text-first-baseline-roadmap-site-pass03-20261030');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass03.json');
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

copyIfMissing(path.join(selectedRoot, 'roadmap.html'), 'roadmap.html');
copyIfMissing(path.join(selectedRoot, 'roadmap.svg'), 'roadmap.svg');
copyIfMissing(path.join(selectedRoot, 'public/icon.svg'), 'icon.svg');

const staticFiles = [
  "internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/expected-values.json",
  "internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/chrome-observation.json",
  "internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/chrome-156-grid-text-first-baseline.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/result.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/simulator.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-align-items.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-start-control.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-align-self.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-reversed.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/result.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/simulator.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-align-items.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-start-control.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-align-self.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-reversed.png"
];
for (const relative of staticFiles) copyIfMissing(path.join(selectedRoot, relative), relative);

const expectedRoutes = [
  "roadmap.html",
  "roadmap.svg",
  "icon.svg",
  "STATUS.html",
  "internal/0059-c11-grid-layout.html",
  "plan/c11-4-grid-text-first-baseline.html",
  "plan/c11-4-grid-text-first-baseline-plan-review-20.html",
  "internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/README.html",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/README.html",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/artifact-boundary/README.html",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/implementation-runtime-review-20.html",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/pr-diff-review-20.html",
  "internal/evidence/c11-4-grid-text-first-baseline-roadmap-sync-2026-10-30/post-merge-doc-sync-review-20.html",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/result.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/simulator.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-align-items.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-start-control.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-align-self.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/android-avd/c114-first-baseline-mvu6l1qt-17df8ba6/grid-text-first-baseline-reversed.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/result.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/simulator.json",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-align-items.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-start-control.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-align-self.png",
  "internal/evidence/c11-4-grid-text-first-baseline-runtime-2026-10-30/ios-simulator/c114-first-baseline-mvu6m6wk-c608999f/grid-text-first-baseline-reversed.png"
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
