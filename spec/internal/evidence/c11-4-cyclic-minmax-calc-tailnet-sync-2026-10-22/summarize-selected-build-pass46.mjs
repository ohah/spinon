import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass46.json'), 'utf8'));
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-tailnet-site-pass46');
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
const runtime = 'internal/evidence/c11-4-cyclic-minmax-calc-runtime-2026-10-22';
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-cyclic-minmax-calc.html',
  'plan/c11-4-cyclic-minmax-calc-plan-review-20.html',
  `${runtime}/README.html`,
  `${runtime}/implementation-review-20.html`,
  `${runtime}/pr-diff-review-30.html`,
  `${runtime}/roadmap-pr-diff-review-30.html`,
  `${runtime}/android-avd-c114-cyclic-minmax-calc-android-avd-20261022T003653863Z-c6952f2b/definite-minmax-calc-small-limit.png`,
  `${runtime}/ios-simulator-c114-cyclic-minmax-calc-ios-simulator-20261022T003845541Z-fdde4073/definite-minmax-calc-small-limit.png`,
  'internal/evidence/c11-4-cyclic-minmax-calc-precomparison-2026-10-22/README.html',
];
const routeSet = new Set(files.map((file) => file.path));
for (const route of requiredRoutes) assert.ok(routeSet.has(route), `missing route: ${route}`);

const readPage = (relative) => fs.readFileSync(path.join(siteRoot, relative), 'utf8');
const roadmap = readPage('roadmap.html');
const status = readPage('STATUS.html');
const specStatus = readPage('spec/STATUS.html');
assert.ok(roadmap.includes('PR #487') && roadmap.includes('2a04286dba1da4bef45fb6b1a1ee9a1cace54e56'), 'roadmap merge record is missing');
assert.match(roadmap, /72\/213/);
assert.ok(status.includes('2a04286dba1da4bef45fb6b1a1ee9a1cace54e56'), 'STATUS merge record is missing');
assert.ok(specStatus.includes('2a04286dba1da4bef45fb6b1a1ee9a1cace54e56'), 'spec STATUS merge record is missing');
assert.match(status, /72\/213/);
assert.match(specStatus, /72\/213/);
assert.match(fs.readFileSync(path.join(repositoryRoot, 'spec/STATUS.md'), 'utf8'), /^\s*- \[ \] \*\*C11\.4 intrinsic track sizing/m);

const treeHash = createHash('sha256');
for (const file of files) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  pass: 'pass46',
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
  pr487MergeShaRendered: true,
  buildLog: 'selected-build-pass46.log',
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass46.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
