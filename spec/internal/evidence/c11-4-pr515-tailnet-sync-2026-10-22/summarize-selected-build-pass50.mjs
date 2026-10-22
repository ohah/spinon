import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr515-tailnet-site-pass50');
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const routes = [
  'roadmap.html', 'STATUS.html', 'spec/STATUS.html', 'internal/0059-c11-grid-layout.html',
  'plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.html',
  'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/README.html',
  'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/pr-diff-review-30.html',
  'internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/README.html',
  'internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/android-definite-forward.png',
  'internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/ios-definite-forward.png',
];
const routeChecks = routes.map((route) => ({ route, exists: fs.statSync(path.join(outputRoot, route)).isFile() }));
assert.ok(routeChecks.every((entry) => entry.exists), 'one or more required routes are missing');

function listFiles(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in build output: ${relative}`);
    if (entry.isDirectory()) result.push(...listFiles(absolute, relative));
    else if (entry.isFile()) result.push(relative);
    else throw new Error(`unsupported build output entry: ${relative}`);
  }
  return result;
}
const files = listFiles(outputRoot);
const tree = createHash('sha256');
let bytes = 0;
for (const relative of files) {
  const value = fs.readFileSync(path.join(outputRoot, relative));
  bytes += value.byteLength;
  tree.update(relative).update('\0').update(digest(value)).update('\n');
}
const roadmap = fs.readFileSync(path.join(outputRoot, 'roadmap.html'), 'utf8');
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
const runtimeEvidence = fs.readFileSync(path.join(outputRoot, routes[5]), 'utf8');
assert.ok(roadmap.includes('mixed cross-axis span groups runtime fixed slice'));
assert.ok(roadmap.includes('263/263') && roadmap.includes('219/219'));
assert.ok(roadmap.includes('77/213'));
assert.ok(status.includes('77/213'));
assert.ok(runtimeEvidence.includes('PixelCopy') && runtimeEvidence.includes('219/219'));
assert.ok(runtimeEvidence.includes('host GPU'));

const summary = {
  pass: 'pass50',
  sourceRoot: 'build/spinon/c11-4-pr515-tailnet-root-pass50-selected',
  outputRoot: 'build/spinon/c11-4-pr515-tailnet-site-pass50',
  fileCount: files.length,
  totalBytes: bytes,
  treeSha256: tree.digest('hex'),
  requiredRoutes: routeChecks.map((entry) => entry.route),
  routeChecks,
  publicRoadmapShowsRuntimeResults: true,
  completionCountStill77of213: true,
  c11_4RemainsIncomplete: true,
  androidAvdAndIosPrecompositorLimitsExplicit: true,
  noSymlinks: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass50.json'), `${JSON.stringify(summary, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
