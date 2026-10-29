import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = '900404c95c57b60c3cc3e3306f2c488c2abc3586';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-3-3-pr792-2026-10-29');
const root = path.join(base, 'root-pass03');
const site = path.join(base, 'site-pass03');
const staging = path.join(base, 'publication-pass03-staging');
const publication = path.join(base, 'publication-pass03');
const rootReportPath = path.join(evidence, 'staged-root-pass03.json');
const reportPath = path.join(evidence, 'selected-publication-pass03.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 32 * 1024 * 1024 });
assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
assert.equal(git(['rev-parse', 'HEAD']).toString().trim(), head);
assert.equal(git(['rev-parse', 'HEAD^{tree}']).toString().trim(), git([`rev-parse`, `${head}^{tree}`]).toString().trim());
const rootManifest = JSON.parse(fs.readFileSync(rootReportPath, 'utf8'));
assert.equal(rootManifest.sourceHead, head);
assert.equal(rootManifest.sourceTree, git(['rev-parse', `${head}^{tree}`]).toString().trim());
for (const candidate of [staging, publication, reportPath]) assert.ok(!fs.existsSync(candidate), `refusing to overwrite ${candidate}`);
assert.ok(fs.lstatSync(root).isDirectory() && !fs.lstatSync(root).isSymbolicLink());
assert.ok(fs.lstatSync(site).isDirectory() && !fs.lstatSync(site).isSymbolicLink());
fs.mkdirSync(staging, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copyRoute(route, sourcePath, label) {
  safeRoute(route);
  const stat = fs.lstatSync(sourcePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe publication source: ${sourcePath}`);
  const bytes = fs.readFileSync(sourcePath);
  const destination = path.join(staging, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(hash(copied), hash(bytes), `publication copy hash mismatch: ${route}`);
  selected.set(route, { source: label, bytes: copied.length, sha256: hash(copied) });
}
function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in generated output: ${child}`);
    if (entry.isDirectory()) walk(directory, child);
    else if (entry.isFile()) {
      if (child !== 'index.html' && child !== '404.html') {
        if (!selected.has(child.split(path.sep).join('/'))) copyRoute(child.split(path.sep).join('/'), absolute, `rspress:${child}`);
      }
    } else throw new Error(`unsupported generated output entry: ${child}`);
  }
}
walk(site);
for (const entry of rootManifest.staged) {
  if (entry.source === 'generated:index.md') continue;
  const sourcePath = path.join(root, entry.route);
  assert.equal(hash(fs.readFileSync(sourcePath)), entry.sha256, `staged source changed: ${entry.route}`);
  if (!selected.has(entry.route)) copyRoute(entry.route, sourcePath, `staged:${entry.source}`);
}
const requiredRoutes = [
  'roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html',
  'STATUS.html', 'spec/STATUS.html', 'internal/0059-c11-grid-layout.html',
  'plan/c11-3-3-leading-named-implicit-lines.html',
  'plan/c11-css-grid.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-precomparison-2026-10-29/README.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/README.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/implementation-review-20.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/pr-diff-review-30.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/post-merge-status-review-30.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/roadmap-pr-diff-review-30.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29/plan-status-review-30.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-plan-review-20-2026-10-29.html',
  'internal/evidence/c11-3-3-leading-named-implicit-lines-plan-pr-diff-review-30-2026-10-29.html',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-29/android-avd-c1133-matrix-20261029T044453973Z-30ad6bc8/android-screen.png',
  'internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-29/ios-simulator-c1133-ios-matrix-20261029T044652725Z-9bc5bb6e/ios-leading-positive-missing-occurrences-screen.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);
const roadmapBytes = fs.readFileSync(path.join(staging, 'roadmap.html'));
assert.deepEqual(roadmapBytes, git(['show', `${head}:spec/public/roadmap.html`]));
const roadmap = roadmapBytes.toString('utf8');
for (const marker of ['PR #790', 'PR #791', 'd993eb4114b17a49b2d096f319053fd43c6fcf27', 'b532c96a718bdede496b9509da6f20bd142805b1', '72/213']) {
  assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
}
const status = fs.readFileSync(path.join(staging, 'STATUS.html'), 'utf8');
for (const marker of ['pull/790', 'd993eb4114b17a49b2d096f319053fd43c6fcf27', '72/213']) {
  assert.ok(status.includes(marker), `rendered STATUS marker missing: ${marker}`);
}
const plan = fs.readFileSync(path.join(staging, 'plan/c11-3-3-leading-named-implicit-lines.html'), 'utf8');
for (const marker of ['PR #790', 'd993eb4114b17a49b2d096f319053fd43c6fcf27', 'C11.3.3과 C11은 여전히 미완료']) {
  assert.ok(plan.includes(marker), `rendered plan marker missing: ${marker}`);
}
for (const route of requiredRoutes.filter((route) => route.endsWith('.png'))) {
  assert.deepEqual(fs.readFileSync(path.join(staging, route)).subarray(0, 8), Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), `invalid PNG: ${route}`);
}
const routes = [...selected].sort(([a], [b]) => a.localeCompare(b)).map(([route, item]) => ({ path: route, ...item }));
const report = {
  sourceHead: head,
  sourceTree: rootManifest.sourceTree,
  publicationRoot: path.relative(repo, publication),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  noDeletes: true,
  exactMergedRoadmapSource: true,
  requiredRoutes,
  roadmapMarkers: ['PR #790', 'PR #791', 'd993eb4114b17a49b2d096f319053fd43c6fcf27', 'b532c96a718bdede496b9509da6f20bd142805b1', '72/213'],
  fullDocsLinkValidation: false,
  routes,
};
fs.renameSync(staging, publication);
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
