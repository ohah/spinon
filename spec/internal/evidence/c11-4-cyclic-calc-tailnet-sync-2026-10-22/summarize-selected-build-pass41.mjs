import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-calc-tailnet-sync-root-pass41-selected');
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-calc-tailnet-site-pass41');
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass41.json'), 'utf8'));
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-cyclic-calc-track-sizing.html',
  'plan/c11-4-cyclic-calc-track-sizing-plan-review-20.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-css-grid.html',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/README.html',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/README.html',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/implementation-review-20.html',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/pr-diff-review-30.html',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224422209Z-5ec7de6b/cyclic-calc-two-axis.png',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224442114Z-cd934a2e/cyclic-calc-two-axis.png',
  'internal/evidence/c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/chrome156-calc-tracks.png',
  'tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html',
];

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected build: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(fullPath, relative));
    else {
      assert.ok(entry.isFile(), `unsupported selected build entry: ${relative}`);
      files.push(relative);
    }
  }
  return files;
}

const files = listFiles(siteRoot);
const manifestParts = [];
let bytes = 0;
for (const relative of files) {
  const content = fs.readFileSync(path.join(siteRoot, relative));
  bytes += content.byteLength;
  manifestParts.push(`${relative}\0${content.byteLength}\0${sha256(content)}\n`);
}
const missingRoutes = requiredRoutes.filter((route) => !fs.existsSync(path.join(siteRoot, route)));
assert.deepEqual(missingRoutes, [], `required routes are missing: ${missingRoutes.join(', ')}`);
const roadmap = fs.readFileSync(path.join(siteRoot, 'roadmap.html'), 'utf8');
const status = fs.readFileSync(path.join(siteRoot, 'STATUS.html'), 'utf8');
assert.ok(roadmap.includes('PR #480'), 'PR #480 note missing from rendered public roadmap');
assert.ok(roadmap.includes('72/213'), 'official count missing from rendered roadmap');
assert.ok(status.includes('e8c7eb06c66fc7424002eaca76af081ecd72a24f'), 'merged SHA missing from rendered status');

const output = {
  pass: 'pass41',
  rspressVersion: '2.0.22',
  selectedRoot: preparation.selectedRoot,
  siteRoot: path.relative(repositoryRoot, siteRoot),
  sourceHead: preparation.sourceHead,
  sourceTree: preparation.sourceTree,
  originMainHead: preparation.originMainHead,
  originMainTree: preparation.originMainTree,
  fileCount: files.length,
  bytes,
  siteTreeSha256: sha256(Buffer.from(manifestParts.join(''))),
  newRoadmapNotePresent: true,
  statusEntryPresent: true,
  requiredPreviewRoutes: requiredRoutes,
  noSymlinks: true,
  buildLog: 'selected-build-pass41.log',
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass41.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
