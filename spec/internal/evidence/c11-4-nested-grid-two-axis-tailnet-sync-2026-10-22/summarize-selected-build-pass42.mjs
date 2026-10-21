import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass42.json'), 'utf8'));
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-two-axis-multitrack-tailnet-sync-site-pass42');
const files = [];

function walk(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected site: ${relative}`);
    if (entry.isDirectory()) walk(fullPath, relative);
    else {
      assert.ok(entry.isFile(), `unsupported selected site entry: ${relative}`);
      const bytes = fs.readFileSync(fullPath);
      files.push({ path: relative, bytes: bytes.byteLength, sha256: createHash('sha256').update(bytes).digest('hex') });
    }
  }
}

walk(siteRoot);
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-cyclic-two-axis-multitrack.html',
  'plan/c11-4-cyclic-two-axis-multitrack-plan-review-20.html',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-precomparison-2026-10-22/README.html',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-precomparison-2026-10-22/chrome156-two-axis-multitrack.png',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/README.html',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/pr-diff-review-20.html',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/android-avd-c114-cyclic-percent-tracks-android-avd-20261021T220632656Z-acc26dfe/cyclic-percent-two-axis-multitrack.png',
  'internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T220642593Z-54e24c0a/cyclic-percent-two-axis-multitrack.png',
];
const routeSet = new Set(files.map((file) => file.path));
for (const route of requiredRoutes) assert.ok(routeSet.has(route), `missing route: ${route}`);

const readPage = (relative) => fs.readFileSync(path.join(siteRoot, relative), 'utf8');
const roadmap = readPage('roadmap.html');
const status = readPage('STATUS.html');
const specStatus = readPage('spec/STATUS.html');
const plan = readPage('plan/c11-4-cyclic-two-axis-multitrack.html');
assert.ok(roadmap.includes('PR #478') && roadmap.includes('907390f75e0b25b399a47b6c863a19b7e5a326e7'), 'roadmap merge record is missing');
assert.match(status, /72\/213/);
assert.match(specStatus, /72\/213/);
assert.ok(status.includes('cyclic-two-axis-multitrack') && specStatus.includes('cyclic-two-axis-multitrack'), 'C11.4 status evidence links are missing');
assert.ok(plan.includes('Rebase and merge') && plan.includes('907390f75e0b25b399a47b6c863a19b7e5a326e7'), 'merged plan record is missing');
const sourceStatus = fs.readFileSync(path.join(repositoryRoot, 'spec/STATUS.md'), 'utf8');
assert.match(sourceStatus, /^\s*- \[ \] \*\*C11\.4 intrinsic track sizing/m);

const treeHash = createHash('sha256');
for (const file of files) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  pass: 'pass42',
  rspressVersion: '2.0.22',
  sourceHead: preparation.sourceHead,
  sourceTree: preparation.sourceTree,
  selectedRoot: preparation.selectedRoot,
  siteRoot: path.relative(repositoryRoot, siteRoot),
  fileCount: files.length,
  bytes: files.reduce((sum, file) => sum + file.bytes, 0),
  siteTreeSha256: treeHash.digest('hex'),
  requiredRoutes,
  noSymlinks: true,
  latestMainRoadmapRendered: true,
  c114Unchecked: true,
  officialCompletionCountStill72of213: true,
  cyclicMultitrackPlanAndSimulatorEvidenceRendered: true,
  buildLog: 'selected-build-pass42.log',
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass42.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
