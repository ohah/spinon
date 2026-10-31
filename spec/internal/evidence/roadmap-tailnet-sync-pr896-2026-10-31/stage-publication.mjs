import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr896-2026-10-31/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publication = path.join(base, 'publication');
const staging = path.join(base, 'publication-staging-pass01');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const reportPath = path.join(evidence, 'selected-publication-pass01.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const candidate of [publication, staging, reportPath]) assert.ok(!fs.existsSync(candidate), `existing output preserved: ${candidate}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe build directory: ${directory}`);
}
fs.mkdirSync(staging, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = hash(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `publication route collision: ${route}`);
    return;
  }
  const destination = path.join(staging, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(hash(fs.readFileSync(destination)), digest, `publication hash mismatch: ${route}`);
  selected.set(route, { path: route, source, kind, bytes: bytes.length, sha256: digest });
}
function walk(directory, relative = '', generated = false) {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in publication input: ${child}`);
    if (entry.isDirectory()) walk(directory, child, generated);
    else if (entry.isFile()) {
      const route = child.split(path.sep).join('/');
      if (generated && (route === 'index.html' || route === '404.html')) continue;
      if (!generated && route === 'index.md') continue;
      add(route, fs.readFileSync(absolute), `${generated ? 'rspress' : 'selected-root'}:${route}`, generated ? 'generated-page-or-asset' : 'source-page-or-asset');
    } else throw new Error(`unsupported publication input: ${child}`);
  }
}
walk(site, '', true);
for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md') continue;
  const source = path.join(root, item.route);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `selected root size changed: ${item.route}`);
  assert.equal(hash(bytes), item.sha256, `selected root hash changed: ${item.route}`);
  add(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}
assert.deepEqual(
  fs.readFileSync(path.join(staging, 'roadmap.html')),
  fs.readFileSync(path.join(root, 'spec/public/roadmap.html')),
  'raw roadmap is not the merged source',
);
const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
for (const marker of ['PR #893 Rebase and merge', 'ae292fff135a925ef30a3e2437082e96c2386e9c', '140/140', 'PR #885 Rebase and merge', '9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3', 'PR #880 Rebase and merge', 'c592212bbf858cda0f310bb5885e864687960d06', '55/55', '177/177', '72/213', 'C11.4/C11은 미완료']) {
  assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
}
const status = fs.readFileSync(path.join(staging, 'STATUS.html'), 'utf8');
for (const marker of ['72/213', 'C11.4 intrinsic track sizing', '9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3', 'ae292fff135a925ef30a3e2437082e96c2386e9c', 'c11-4-anonymous-pre-tabs-2026-10-31']) {
  assert.ok(status.includes(marker), `STATUS marker missing: ${marker}`);
}
const plan = fs.readFileSync(path.join(staging, 'plan/c11-4-direct-text-aspect-ratio-definite-axis.html'), 'utf8');
for (const marker of ['width:60px', 'height:30px', '2 / 1', '72/213']) assert.ok(plan.includes(marker), `candidate plan marker missing: ${marker}`);
const prePlan = fs.readFileSync(path.join(staging, 'plan/c11-4-anonymous-grid-text-pre.html'), 'utf8');
for (const marker of ['white-space:pre', '31/31', '72/213']) assert.ok(prePlan.includes(marker), `pre plan marker missing: ${marker}`);
const requiredRoutes = [
  'plan/c11-4-anonymous-grid-text-pre-tab-size-values.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/README.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/pr-diff-review-30.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/post-merge-status-review-30.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/roadmap-entry-pr-diff-review-30.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/rspress-image-links-review-30.html',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/c114-tab-values-20261031024357017-869b04d0/android-avd/representative.png',
  'internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31/c114-tab-values-20261031024357017-869b04d0/ios-simulator/representative.png',
  'plan/c11-4-anonymous-grid-text-pre-tabs.html',
  'internal/evidence/c11-4-anonymous-pre-tabs-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-pre-tabs-2026-10-31/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-pre-tabs-2026-10-31/android-avd-c114-anonymous-grid-text-android-avd-20261030T234628547Z-d752add6/anonymous-pre-tab-max-content.png',
  'internal/evidence/c11-4-anonymous-pre-tabs-2026-10-31/ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T234748178Z-8979ca74/anonymous-pre-tab-max-content.png',
  'plan/c11-4-anonymous-grid-text-pre.html',
  'internal/evidence/c11-4-anonymous-text-pre-precomparison-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-text-pre-precomparison-2026-10-31/chrome-156.png',
  'internal/evidence/c11-4-anonymous-grid-text-pre-runtime-2026-10-31-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-runtime-2026-10-31-2026-10-31/pr-diff-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-runtime-2026-10-31-2026-10-31/post-merge-record-review-20.html',
  'roadmap.html', 'STATUS.html', 'spec/STATUS.html',
  'plan/c11-css-grid.html', 'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-direct-text-aspect-ratio-definite-axis.html',
  'internal/0059-c11-grid-layout.html',
  'internal/0070-c15-1-inline-text-layout.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/README.html',
  'tests/fixtures/css/c11/anonymous-grid-text-pre-precomparison.html',
  'internal/evidence/c11-4-anonymous-text-pre-precomparison-2026-10-31/oracle-image-reference-review-20.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/pr-diff-review-30.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/android-avd/c114-text-ratio-mvvhtfz4-348d5435/grid-text-aspect-ratio-definite-width-ratio.png',
  'internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/ios-simulator/c114-text-ratio-mvvhvvfz-4b9929a2/grid-text-aspect-ratio-definite-width-ratio.png',
  'internal/evidence/c11-4-direct-text-aspect-ratio-axis-transfer-precomparison-2026-10-31/README.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);
const tabPlan = fs.readFileSync(path.join(staging, 'plan/c11-4-anonymous-grid-text-pre-tabs.html'), 'utf8');
const tabValuesPlan = fs.readFileSync(path.join(staging, 'plan/c11-4-anonymous-grid-text-pre-tab-size-values.html'), 'utf8');
for (const marker of ['14 computed values', '140/140', '72/213', 'PR #893']) assert.ok(tabValuesPlan.includes(marker), `tab-size plan marker missing: ${marker}`);
for (const marker of ['tab-size:8', '40/40', '72/213', 'PR #885']) assert.ok(tabPlan.includes(marker), `tab plan marker missing: ${marker}`);
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  selectedRootFileCount: rootManifest.stagedFileCount,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  requiredRoutes,
  roadmapMarkers: ['PR #893 Rebase and merge', 'ae292fff135a925ef30a3e2437082e96c2386e9c', '140/140', 'PR #885 Rebase and merge', '9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3', 'PR #880 Rebase and merge', 'c592212bbf858cda0f310bb5885e864687960d06', '72/213'],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
