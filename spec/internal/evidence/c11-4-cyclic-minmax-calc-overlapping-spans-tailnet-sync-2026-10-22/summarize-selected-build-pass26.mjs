import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
assert.ok(['pass26-initial', 'pass26-final'].includes(pass ?? ''), 'set pass26-initial or pass26-final');
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-docs-root-' + pass + '.json'), 'utf8'));
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-site-' + pass + '-retry');
const files = [];

function walk(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in selected site: ' + relative);
    if (entry.isDirectory()) walk(fullPath, relative);
    else {
      assert.ok(entry.isFile(), 'unsupported selected site entry: ' + relative);
      const bytes = fs.readFileSync(fullPath);
      files.push({ path: relative, bytes: bytes.byteLength, sha256: createHash('sha256').update(bytes).digest('hex') });
    }
  }
}

walk(siteRoot);
const runtime = 'internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-runtime-2026-10-22';
const tailnet = 'internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-sync-2026-10-22';
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-cyclic-minmax-calc-overlapping-spans.html',
  'internal/README.html',
  runtime + '/README.html',
  runtime + '/implementation-review-20.html',
  runtime + '/pr-diff-review-20.html',
  runtime + '/android-avd-c114-overlapping-span-groups-android-avd-20261022T045550962Z-9ada2377/definite-minmax-calc-overlap-small-forward.png',
  runtime + '/ios-simulator-c114-overlapping-span-groups-ios-simulator-20261022T045608568Z-282431e3/definite-minmax-calc-overlap-small-forward.png',
  tailnet + '/README.html',
  tailnet + '/pr-diff-review-30.html',
];
const fileSet = new Set(files.map((file) => file.path));
for (const route of requiredRoutes) assert.ok(fileSet.has(route), 'missing route: ' + route);

const readPage = (relative) => fs.readFileSync(path.join(siteRoot, relative), 'utf8');
const roadmap = readPage('roadmap.html');
const status = readPage('STATUS.html');
const specStatus = readPage('spec/STATUS.html');
const plan = readPage('plan/c11-4-cyclic-minmax-calc-overlapping-spans.html');
const runtimePage = readPage(runtime + '/README.html');
const reviewPage = readPage(tailnet + '/pr-diff-review-30.html');
const sourceStatus = fs.readFileSync(path.join(repositoryRoot, 'spec/STATUS.md'), 'utf8');
assert.ok(roadmap.includes('PR #500') && roadmap.includes('2f3f45c6aa27f427b36e906fa3a72fd8d864e402'), 'roadmap merge record missing');
assert.match(roadmap, /77\/213/);
assert.ok(status.includes('2f3f45c6aa27f427b36e906fa3a72fd8d864e402'), 'STATUS merge record missing');
assert.ok(specStatus.includes('2f3f45c6aa27f427b36e906fa3a72fd8d864e402'), 'spec STATUS merge record missing');
assert.match(status, /77\/213/);
assert.match(specStatus, /77\/213/);
assert.match(sourceStatus, /^\s*- \[ \] \*\*C11\.4 intrinsic track sizing/m);
assert.ok(plan.includes('PR #500') && plan.includes('2f3f45c6aa27f427b36e906fa3a72fd8d864e402'), 'plan merge record missing');
assert.match(runtimePage, /android-avd-c114-overlapping-span-groups-android-avd/);
assert.match(runtimePage, /ios-simulator-c114-overlapping-span-groups-ios-simulator/);
assert.equal((reviewPage.match(/<tr>/g) || []).length, 31, 'review page must contain header plus 30 review rows');

const treeHash = createHash('sha256');
for (const file of files) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  schemaVersion: 1,
  pass,
  rspressVersion: '2.0.22',
  sourceHead: preparation.sourceCommit,
  sourceTree: preparation.sourceTree,
  selectedRoot: preparation.selectedRoot,
  siteRoot: path.relative(repositoryRoot, siteRoot).split(path.sep).join('/'),
  fileCount: files.length,
  bytes: files.reduce((sum, file) => sum + file.bytes, 0),
  siteTreeSha256: treeHash.digest('hex'),
  requiredRoutes,
  noSymlinks: true,
  latestMainRoadmapRendered: true,
  c114Unchecked: true,
  officialCompletionCountStill77of213: true,
  pr500MergeShaRendered: true,
  documentPRReviewRows: 30,
  buildLog: 'selected-build-' + pass + '-retry.log',
};
const outputPath = path.join(evidenceDirectory, 'selected-build-summary-' + pass + '.json');
fs.writeFileSync(outputPath, JSON.stringify(output, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify(output, null, 2) + '\n');
