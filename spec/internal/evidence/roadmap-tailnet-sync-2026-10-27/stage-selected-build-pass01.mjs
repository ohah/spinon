import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const fullRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261027-site-pass01-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261027-publication-pass01-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const evidence = 'spec/internal/evidence/c11-4-generic-direct-text-runtime-matrix-2026-10-27';
const run = `${evidence}/runs/c114-direct-text-matrix-20261027T031021Z-pass02`;
const generatedRoutes = [
  'STATUS.html', 'spec/STATUS.html',
  'plan/c11-4-generic-direct-text-runtime-matrix.html', 'spec/plan/c11-4-generic-direct-text-runtime-matrix.html',
  'plan/c11-4-generic-direct-text-runtime-matrix-plan-review-20.html', 'spec/plan/c11-4-generic-direct-text-runtime-matrix-plan-review-20.html',
  'plan/c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.html', 'spec/plan/c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.html',
  `${evidence.slice('spec/'.length)}/README.html`, `${evidence}/README.html`,
  `${evidence.slice('spec/'.length)}/implementation-runtime-review-20.html`, `${evidence}/implementation-runtime-review-20.html`,
  `${evidence.slice('spec/'.length)}/pr-diff-review-20.html`, `${evidence}/pr-diff-review-20.html`,
  `${evidence.slice('spec/'.length)}/post-merge-status-roadmap-review-20.html`, `${evidence}/post-merge-status-roadmap-review-20.html`,
  'internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.html',
  'spec/internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.html',
];
const rawFiles = [
  { source: 'spec/public/roadmap.html', routes: ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'] },
  { source: 'spec/STATUS.md', routes: ['STATUS.md', 'spec/STATUS.md'] },
  { source: 'plan/c11-4-generic-direct-text-runtime-matrix.md', routes: ['plan/c11-4-generic-direct-text-runtime-matrix.md', 'spec/plan/c11-4-generic-direct-text-runtime-matrix.md'] },
  { source: 'plan/c11-4-generic-direct-text-runtime-matrix-plan-review-20.md', routes: ['plan/c11-4-generic-direct-text-runtime-matrix-plan-review-20.md', 'spec/plan/c11-4-generic-direct-text-runtime-matrix-plan-review-20.md'] },
  { source: 'plan/c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.md', routes: ['plan/c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.md', 'spec/plan/c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.md'] },
  { source: `${evidence}/README.md`, routes: [`${evidence}/README.md`, `${evidence.slice('spec/'.length)}/README.md`, `public/${evidence.slice('spec/'.length)}/README.md`, `public/${evidence}/README.md`] },
  { source: `${evidence}/implementation-runtime-review-20.md`, routes: [`${evidence}/implementation-runtime-review-20.md`, `${evidence.slice('spec/'.length)}/implementation-runtime-review-20.md`, `public/${evidence.slice('spec/'.length)}/implementation-runtime-review-20.md`, `public/${evidence}/implementation-runtime-review-20.md`] },
  { source: `${evidence}/pr-diff-review-20.md`, routes: [`${evidence}/pr-diff-review-20.md`, `${evidence.slice('spec/'.length)}/pr-diff-review-20.md`, `public/${evidence.slice('spec/'.length)}/pr-diff-review-20.md`, `public/${evidence}/pr-diff-review-20.md`] },
  { source: `${evidence}/post-merge-status-roadmap-review-20.md`, routes: [`${evidence}/post-merge-status-roadmap-review-20.md`, `${evidence.slice('spec/'.length)}/post-merge-status-roadmap-review-20.md`, `public/${evidence.slice('spec/'.length)}/post-merge-status-roadmap-review-20.md`, `public/${evidence}/post-merge-status-roadmap-review-20.md`] },
  { source: 'spec/internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.md', routes: ['spec/internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.md', 'internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.md', 'public/internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.md', 'public/spec/internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.md'] },
  { source: `${run}/android-avd/screenshot.png`, routes: [`${run.slice('spec/'.length)}/android-avd/screenshot.png`, `${run}/android-avd/screenshot.png`] },
  { source: `${run}/android-avd/matrix.tsv`, routes: [`${run.slice('spec/'.length)}/android-avd/matrix.tsv`, `${run}/android-avd/matrix.tsv`] },
  { source: `${run}/ios-simulator/screenshot.png`, routes: [`${run.slice('spec/'.length)}/ios-simulator/screenshot.png`, `${run}/ios-simulator/screenshot.png`] },
  { source: `${run}/ios-simulator/matrix.tsv`, routes: [`${run.slice('spec/'.length)}/ios-simulator/matrix.tsv`, `${run}/ios-simulator/matrix.tsv`] },
  { source: `${run}/validation/archive-boundary/result.txt`, routes: [`${run.slice('spec/'.length)}/validation/archive-boundary/result.txt`, `${run}/validation/archive-boundary/result.txt`] },
];

assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected build report');
assert.ok(fs.lstatSync(rootManifestPath).isFile(), 'selected root manifest is missing');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sourceHead = rootManifest.sourceHead;
const sourceTree = rootManifest.sourceTree;
assert.equal(execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceHead);
assert.equal(execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceTree);
assert.ok(fs.lstatSync(fullRoot).isDirectory(), 'selected Rspress output is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace selected publication root');

const sources = new Map();
for (const route of generatedRoutes) {
  const generated = path.join(fullRoot, route);
  assert.ok(fs.lstatSync(generated).isFile(), `generated page is missing: ${route}`);
  sources.set(route, generated);
}
const tracked = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', sourceHead, '--', ...rawFiles.map((entry) => entry.source)], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));
for (const { source, routes } of rawFiles) {
  assert.ok(tracked.has(source), `raw source is not tracked at HEAD: ${source}`);
  const absolute = path.join(repositoryRoot, source);
  const sourceBytes = fs.readFileSync(absolute);
  const headBytes = execFileSync('git', ['show', `${sourceHead}:${source}`], { cwd: repositoryRoot });
  assert.deepEqual(sourceBytes, headBytes, `raw source differs from HEAD: ${source}`);
  for (const route of routes) sources.set(route, absolute);
}

for (const route of generatedRoutes) {
  const html = fs.readFileSync(path.join(fullRoot, route), 'utf8');
  for (const match of html.matchAll(/(?:href|src)=["']\/spinon\/([^"'#?]+)["']/g)) {
    const assetRoute = match[1];
    if (!/\.(?:css|js|svg|png|jpe?g|webp|gif|woff2?|ttf|ico|tsv|log|txt|md)$/i.test(assetRoute) || sources.has(assetRoute)) continue;
    const builtAsset = path.join(fullRoot, assetRoute);
    if (fs.existsSync(builtAsset)) sources.set(assetRoute, builtAsset);
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
  ['roadmap.html', ['72/213', '#679', 'C11.4', 'direct element-text intrinsic']],
  ['STATUS.html', ['72/213', '093a2adfa846f0ad0e7218074d34821f1825d771', 'C11.4']],
  ['plan/c11-4-generic-direct-text-runtime-matrix.html', ['#679', '72/213', 'glyph paint']],
  [`${evidence.slice('spec/'.length)}/README.html`, ['6/6', 'runtime_text_count=0', 'glyph_batch_draw_count=0']],
  [`${evidence.slice('spec/'.length)}/post-merge-status-roadmap-review-20.html`, ['20', '72/213', 'Tailnet']],
  ['internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.html', ['30', '72/213', 'C11.4']],
];
for (const [route, required] of markers) {
  const content = fs.readFileSync(path.join(selectedRoot, route), 'utf8').toLowerCase();
  for (const marker of required) assert.ok(content.includes(marker.toLowerCase()), `${route} is missing marker ${marker}`);
}
assert.ok(fs.readFileSync(path.join(selectedRoot, 'STATUS.md'), 'utf8').includes('- [ ] **C11.4'), 'C11.4 must remain unchecked');
for (const route of [`${run.slice('spec/'.length)}/android-avd/screenshot.png`, `${run.slice('spec/'.length)}/ios-simulator/screenshot.png`]) {
  const bytes = fs.readFileSync(path.join(selectedRoot, route));
  assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', `invalid PNG signature: ${route}`);
}

const report = {
  pass: 'pass01', sourceHead, sourceTree,
  fullBuildRoot: path.relative(repositoryRoot, fullRoot),
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  generatedRouteCount: generatedRoutes.length,
  selectedRouteCount: selectedRoutes.length,
  totalBytes: selectedRoutes.reduce((sum, entry) => sum + entry.bytes, 0),
  markers, noDeletes: true, selectedRoutes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined }, null, 2)}\n`);
