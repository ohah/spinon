import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass04.json'), 'utf8'));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-border-overflow-tailnet-site-pass04');
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
  'plan/c11-4-anonymous-grid-border-overflow.html',
  'plan/c11-4-anonymous-grid-two-run-three-image-border-overflow.html',
  'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/README.html',
  'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T083242895Z-3df1ac86/two-anonymous-runs-three-images-forward.png',
  'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T083242895Z-3df1ac86/three-images-two-anonymous-runs-reversed.png',
  'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T083256735Z-5aeaa47a/two-anonymous-runs-three-images-forward.png',
  'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T083256735Z-5aeaa47a/three-images-two-anonymous-runs-reversed.png',
  'plan/c11-4-anonymous-grid-border-overflow-implementation-review-20.html',
  'plan/c11-4-anonymous-grid-border-overflow-pr-diff-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/README.html',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/anonymous-runs-two-images-forward.png',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T070603043Z-8ea2e515/anonymous-runs-two-images-forward.png',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-tailnet-sync-2026-10-21/README.html',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-tailnet-sync-2026-10-21/pr433-agent-browser-full-pass01.png',
];
for (const route of requiredRoutes) assert.ok(files.some((file) => file.path === route), `missing route: ${route}`);
const roadmap = fs.readFileSync(path.join(siteRoot, 'roadmap.html'), 'utf8');
const status = fs.readFileSync(path.join(siteRoot, 'STATUS.html'), 'utf8');
const plan = fs.readFileSync(path.join(siteRoot, 'plan/c11-4-anonymous-grid-border-overflow.html'), 'utf8');
const newPlan = fs.readFileSync(path.join(siteRoot, 'plan/c11-4-anonymous-grid-two-run-three-image-border-overflow.html'), 'utf8');
assert.ok(roadmap.includes('PR #431') && roadmap.includes('PR #436') && roadmap.includes('C11.4 anonymous Grid 두 run + 세 image'));
assert.ok(roadmap.includes('c11-4-anonymous-grid-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow'));
assert.ok(roadmap.includes('c11-4-anonymous-grid-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow'));
assert.ok(status.includes('pull/431') && status.includes('e0ac9aaf1681aeb56850bf749ec099cf1737d44f') && status.includes('72/213'));
assert.ok(plan.includes('420be2c40cb0a9f68023121f66ff122158494ffa') && plan.includes('Tailnet preview pass01'));
assert.ok(newPlan.includes('e0ac9aaf1681aeb56850bf749ec099cf1737d44f') && newPlan.includes('PR #436'));
const simulatorReadme = fs.readFileSync(path.join(siteRoot, 'internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/README.html'), 'utf8');
for (const alt of ['Android AVD forward scene', 'Android AVD reversed scene', 'iOS Simulator forward scene', 'iOS Simulator reversed scene']) assert.ok(simulatorReadme.includes(alt), `missing rendered simulator image alt: ${alt}`);
const syncReadme = fs.readFileSync(path.join(siteRoot, 'internal/evidence/c11-4-anonymous-grid-border-overflow-tailnet-sync-2026-10-21/README.html'), 'utf8');
assert.ok(syncReadme.includes('2,992/2,992') && syncReadme.includes('C11.4 checkbox'));
for (const alt of ['Tailnet roadmap의 PR #431 병합 항목', 'Tailnet에서 열린 Android AVD forward 캡처', 'Tailnet에서 열린 iOS Simulator forward 캡처']) assert.ok(syncReadme.includes(alt), `missing rendered README image alt: ${alt}`);
const treeHash = createHash('sha256');
for (const file of files) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  pass: 'pass04',
  rspressVersion: '2.0.22',
  sourceHead: preparation.sourceHead,
  sourceTree: preparation.sourceTree,
  selectedRoot: 'build/spinon/c11-4-anonymous-grid-border-overflow-tailnet-root-pass04-selected',
  siteRoot: path.relative(repositoryRoot, siteRoot),
  fileCount: files.length,
  bytes: files.reduce((sum, file) => sum + file.bytes, 0),
  siteTreeSha256: treeHash.digest('hex'),
  requiredRoutes,
  noSymlinks: true,
  roadmapNotePresent: true,
  statusMergeRecordPresent: true,
  newImplementationMergeRecordPresent: true,
  simulatorScreenshotsPresent: true,
  officialCompletionCountStill72of213: true,
  buildLog: 'selected-build-pass04.log',
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass04.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
