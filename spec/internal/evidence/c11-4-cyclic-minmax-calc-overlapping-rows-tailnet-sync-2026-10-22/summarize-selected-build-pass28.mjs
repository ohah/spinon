import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDir, '../../../..');
const prepPath = path.join(evidenceDir, 'selected-root-preparation-pass28.json');
const prep = JSON.parse(fs.readFileSync(prepPath, 'utf8'));
const siteRoot = path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-rows-tailnet-site-pass28-retry');
const outputPath = path.join(evidenceDir, 'selected-build-summary-pass28.json');
assert.ok(fs.statSync(siteRoot).isDirectory());
assert.ok(!fs.existsSync(outputPath), 'refusing to overwrite build summary');

const runtime = 'internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-runtime-2026-10-22';
const routes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-cyclic-minmax-calc-overlapping-rows.html',
  'plan/c11-4-cyclic-minmax-calc-overlapping-rows-plan-review-20.html',
  `${runtime}/README.html`,
  `${runtime}/implementation-runtime-review-20.html`,
  `${runtime}/pr-diff-review-20.html`,
];
const files = [];
const symlinks = [];
function walk(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) symlinks.push(relative);
    else if (entry.isDirectory()) walk(absolute, relative);
    else if (entry.isFile()) {
      const bytes = fs.readFileSync(absolute);
      files.push({ path: relative, bytes: bytes.byteLength, sha256: createHash('sha256').update(bytes).digest('hex') });
    } else throw new Error(`unsupported site entry: ${relative}`);
  }
}
walk(siteRoot);
const paths = new Set(files.map((file) => file.path));
for (const route of routes) assert.ok(paths.has(route), `missing route: ${route}`);
assert.deepEqual(symlinks, [], 'site contains symlinks');
const read = (route) => fs.readFileSync(path.join(siteRoot, route), 'utf8');
const mergeSha = '36acdd4f48f23499835530704fa8aef20590a6e5';
for (const [label, page] of [['roadmap', read('roadmap.html')], ['STATUS', read('STATUS.html')], ['spec/STATUS', read('spec/STATUS.html')], ['plan', read('plan/c11-4-cyclic-minmax-calc-overlapping-rows.html')]]) {
  assert.ok(page.includes(mergeSha), `${label} is missing PR #503 merge SHA`);
  assert.ok(page.includes('77/213'), `${label} changed the official completion count`);
}
assert.ok(read('roadmap.html').includes('PR #503 병합'));
assert.ok(read('STATUS.html').includes('required status checks'));
const evidence = read(`${runtime}/README.html`);
assert.ok(evidence.includes('pr-diff-review-20'));
const screenshotAssets = files.filter((file) => file.path.startsWith('static/image/')
  && file.path.includes('definite-minmax-calc-overlap-rows-small-forward'));
assert.equal(screenshotAssets.length, 2, 'both small-control screenshots must be emitted as optimized assets');
for (const asset of screenshotAssets) {
  assert.ok(evidence.includes(`/spinon/${asset.path}`), `evidence page does not reference ${asset.path}`);
  assert.ok(asset.bytes > 50_000, `optimized screenshot unexpectedly small: ${asset.path}`);
}
routes.push(...screenshotAssets.map((asset) => asset.path));
const siteTreeHash = createHash('sha256');
for (const file of files) { siteTreeHash.update(file.path); siteTreeHash.update('\0'); siteTreeHash.update(file.sha256); siteTreeHash.update('\n'); }
const logPath = path.join(evidenceDir, 'selected-build-pass28.log');
const buildLog = fs.readFileSync(logPath, 'utf8');
assert.ok(buildLog.includes('Rspress v2.0.22'));
assert.ok(buildLog.includes('built in'));
const result = {
  schemaVersion: 1,
  sourceCommit: prep.sourceCommit,
  sourceTree: prep.sourceTree,
  selectedRoot: prep.selectedRoot,
  selectedFiles: prep.selectedFiles,
  selectedBytes: prep.selectedBytes,
  siteRoot: path.relative(repo, siteRoot).split(path.sep).join('/'),
  rspressVersion: '2.0.22',
  siteFileCount: files.length,
  siteBytes: files.reduce((sum, file) => sum + file.bytes, 0),
  siteTreeSha256: siteTreeHash.digest('hex'),
  requiredRoutes: routes,
  noSymlinks: true,
  roadmapShowsPr503Merged: true,
  officialCompletionCountStill77of213: true,
  c114StillUnchecked: true,
  runtimeEvidenceRendered: true,
  buildLog: 'selected-build-pass28.log',
};
fs.writeFileSync(outputPath, `${JSON.stringify(result, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
