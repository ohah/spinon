import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const fullRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-site-pass01-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-publication-pass01-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root-pass01.json');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sourceHead = rootManifest.sourceHead;
const sourceTree = rootManifest.sourceTree;
assert.equal(execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceHead);
assert.equal(execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceTree);
for (const root of [fullRoot, previewRoot]) assert.ok(fs.lstatSync(root).isDirectory(), `required root is missing: ${root}`);
for (const candidate of [selectedRoot, reportPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const generatedRoutes = [
  'STATUS.html', 'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html', 'spec/internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html', 'spec/plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-stretch-auto-track-stage-oracle.html', 'spec/plan/c11-4-stretch-auto-track-stage-oracle.html',
  'plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.html', 'spec/plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.html',
  'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.html',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.html',
  'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.html',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.html',
  'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.html',
];
const rawFiles = [
  { source: 'spec/public/roadmap.html', routes: ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'] },
  { source: 'spec/STATUS.md', routes: ['STATUS.md', 'spec/STATUS.md'] },
  { source: 'spec/internal/0059-c11-grid-layout.md', routes: ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md', 'public/internal/0059-c11-grid-layout.md', 'public/spec/internal/0059-c11-grid-layout.md'] },
  { source: 'plan/c11-4-grid-track-sizing.md', routes: ['plan/c11-4-grid-track-sizing.md', 'spec/plan/c11-4-grid-track-sizing.md'] },
  { source: 'plan/c11-4-stretch-auto-track-stage-oracle.md', routes: ['plan/c11-4-stretch-auto-track-stage-oracle.md', 'spec/plan/c11-4-stretch-auto-track-stage-oracle.md'] },
  { source: 'plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md', routes: ['plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md', 'spec/plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md'] },
  { source: 'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md', routes: ['spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md', 'internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md', 'public/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md', 'public/spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md'] },
];

const sources = new Map();
for (const route of generatedRoutes) {
  const generated = path.join(fullRoot, route);
  assert.ok(fs.lstatSync(generated).isFile(), `generated page missing: ${route}`);
  sources.set(route, generated);
}
const trackedMainSources = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', sourceHead, '--', ...rawFiles.map((entry) => entry.source)], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));
assert.equal(trackedMainSources.size, rawFiles.length, 'a selected raw source is absent from origin/main');
for (const { source, routes } of rawFiles) {
  const absolute = path.join(repositoryRoot, source);
  const sourceBytes = fs.readFileSync(absolute);
  const mainBytes = execFileSync('git', ['show', `${sourceHead}:${source}`], { cwd: repositoryRoot });
  assert.deepEqual(sourceBytes, mainBytes, `raw source differs from origin/main: ${source}`);
  for (const route of routes) sources.set(route, absolute);
}

const assetPattern = /(?:href|src)=["']\/spinon\/([^"'#?]+)["']/g;
for (const route of generatedRoutes) {
  const html = fs.readFileSync(path.join(fullRoot, route), 'utf8');
  for (const match of html.matchAll(assetPattern)) {
    const assetRoute = match[1];
    if (!/\.(?:css|js|svg|png|jpe?g|webp|gif|woff2?|ttf|ico)$/i.test(assetRoute) || sources.has(assetRoute)) continue;
    const builtAsset = path.join(fullRoot, assetRoute);
    if (fs.existsSync(builtAsset)) sources.set(assetRoute, builtAsset);
    else assert.ok(fs.existsSync(path.join(previewRoot, assetRoute)), `asset missing from build and preview: ${assetRoute}`);
  }
}

fs.mkdirSync(selectedRoot, { recursive: true });
const selectedRoutes = [];
for (const [route, source] of [...sources].sort(([a], [b]) => a.localeCompare(b))) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing or unsafe selected source: ${route}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), `selected route hash mismatch: ${route}`);
  selectedRoutes.push({ path: route, bytes: copied.length, sha256: sha256(copied) });
}

const markers = [
  ['roadmap.html', ['72/213', 'C11.4']],
  ['STATUS.md', ['C11.4', '72/213']],
  ['plan/c11-4-grid-track-sizing.html', ['PASS 30/30', '72/213']],
  ['plan/c11-4-stretch-auto-track-stage-oracle.html', ['definite', '45px', 'PASS 30/30']],
  ['internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.html', ['1/1', 'PASS 30/30', '72/213']],
  ['internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.html', ['PASS', '9cdf9657']],
];
for (const [route, required] of markers) {
  const content = fs.readFileSync(path.join(selectedRoot, route), 'utf8').toLowerCase();
  for (const marker of required) assert.ok(content.includes(marker.toLowerCase()), `${route} missing marker: ${marker}`);
}
assert.ok(fs.readFileSync(path.join(selectedRoot, 'STATUS.md'), 'utf8').includes('- [ ] **C11.4'), 'C11.4 must remain unchecked');

const report = {
  pass: 'pass01',
  sourceHead,
  sourceTree,
  fullBuildRoot: path.relative(repositoryRoot, fullRoot),
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  generatedRouteCount: generatedRoutes.length,
  selectedRouteCount: selectedRoutes.length,
  totalBytes: selectedRoutes.reduce((sum, entry) => sum + entry.bytes, 0),
  markers,
  noDeletes: true,
  selectedRoutes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined }, null, 2)}\n`);
