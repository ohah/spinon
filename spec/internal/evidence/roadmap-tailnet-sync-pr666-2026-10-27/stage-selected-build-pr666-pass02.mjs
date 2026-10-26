import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root-pr666-pass01.json');
const reportPath = path.join(evidenceDirectory, 'selected-build-pr666-pass02.json');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sourceHead = rootManifest.sourceHead;
const sourceTree = rootManifest.sourceTree;
const fullRoot = path.join(repositoryRoot, rootManifest.outputRoot);
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr666-20261027-publication-pass02-selected');
assert.equal(execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceHead);
assert.equal(execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceTree);
for (const root of [fullRoot, path.join(repositoryRoot, rootManifest.selectedRoot), previewRoot]) {
  const stat = fs.lstatSync(root);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `missing or unsafe root: ${root}`);
}
for (const candidate of [selectedRoot, reportPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const sources = new Map();
const generatedRoutes = new Set([
  'STATUS.html', 'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html', 'spec/internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html', 'spec/plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-fit-content-auto-neighbor-runtime.html', 'spec/plan/c11-4-fit-content-auto-neighbor-runtime.html',
  'plan/c11-4-fit-content-auto-neighbor-wpt.html', 'spec/plan/c11-4-fit-content-auto-neighbor-wpt.html',
  'plan/c11-4-fit-content-auto-neighbor-runtime-plan-review-20.html', 'spec/plan/c11-4-fit-content-auto-neighbor-runtime-plan-review-20.html',
  'plan/c11-4-fit-content-auto-neighbor-runtime-plan-pr-diff-review-30.html', 'spec/plan/c11-4-fit-content-auto-neighbor-runtime-plan-pr-diff-review-30.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/README.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/README.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/artifact-boundary/README.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/artifact-boundary/README.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/implementation-review-20.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/implementation-review-20.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/pr-diff-review-30.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/README.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/README.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-observations.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-observations.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/plan-review-20.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/plan-review-20.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-review-20.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-review-20.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/pr-diff-review-30.html',
  'internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/implementation-pr-diff-review-30.html',
  'internal/evidence/c11-4-pr664-public-roadmap-update-2026-10-27/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-pr664-public-roadmap-update-2026-10-27/pr-diff-review-30.html',
  'internal/evidence/c11-4-rspress-relative-image-path-fix-2026-10-27/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-rspress-relative-image-path-fix-2026-10-27/pr-diff-review-30.html',
]);
const selectedRootPath = path.join(repositoryRoot, rootManifest.selectedRoot);
for (const route of generatedRoutes) assert.ok(fs.lstatSync(path.join(fullRoot, route)).isFile(), `generated page missing: ${route}`);
for (const overlay of rootManifest.overlays) {
  const sourceBytes = fs.readFileSync(path.join(repositoryRoot, overlay.source));
  const mainBytes = execFileSync('git', ['show', `${sourceHead}:${overlay.source}`], { cwd: repositoryRoot });
  assert.deepEqual(sourceBytes, mainBytes, `selected source differs from origin/main: ${overlay.source}`);
  const overlayPath = path.join(selectedRootPath, overlay.path);
  assert.equal(sha256(fs.readFileSync(overlayPath)), sha256(mainBytes), `selected-root overlay hash mismatch: ${overlay.path}`);
  const keepPublicRoadmap = ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'].includes(overlay.path);
  const keepStatus = ['STATUS.md', 'spec/STATUS.md'].includes(overlay.path);
  const keepContract = ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'].includes(overlay.path);
  const keepPlans = overlay.path.startsWith('plan/') || overlay.path.startsWith('spec/plan/');
  const evidenceRoots = [
    'c11-4-fit-content-auto-neighbor-runtime-2026-10-27/',
    'c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/',
    'c11-4-fit-content-auto-neighbor-wpt-2026-10-26/',
    'c11-4-pr664-public-roadmap-update-2026-10-27/',
    'c11-4-rspress-relative-image-path-fix-2026-10-27/',
  ];
  const keepEvidence = ['internal/evidence/', 'spec/internal/evidence/'].some((prefix) => {
    if (!overlay.path.startsWith(prefix)) return false;
    const remainder = overlay.path.slice(prefix.length);
    return evidenceRoots.some((root) => remainder.startsWith(root));
  });
  if (keepPublicRoadmap || keepStatus || keepContract || keepPlans || keepEvidence) sources.set(overlay.path, overlayPath);
}

for (const route of generatedRoutes) sources.set(route, path.join(fullRoot, route));
const externalExisting = new Map();
function sourceOrExistingAsset(route) {
  const builtAsset = path.join(fullRoot, route);
  try {
    const stat = fs.lstatSync(builtAsset);
    if (stat.isFile() && !stat.isSymbolicLink()) return builtAsset;
  } catch (error) { if (error.code !== 'ENOENT') throw error; }
  const previewAsset = path.join(previewRoot, route);
  try {
    const stat = fs.lstatSync(previewAsset);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe existing preview asset: ${route}`);
    externalExisting.set(route, { path: route, bytes: stat.size, sha256: sha256(fs.readFileSync(previewAsset)) });
    return null;
  } catch (error) {
    if (error.code === 'ENOENT') throw new Error(`asset missing from build and preview: ${route}`);
    throw error;
  }
}

const assetPattern = /(?:href|src)=["']\/spinon\/([^"'#?]+)["']/g;
const staticAssetPattern = /\.(?:css|js|svg|png|jpe?g|webp|gif|woff2?|ttf|ico)$/i;
const pendingCss = [];
for (const route of generatedRoutes) {
  const html = fs.readFileSync(path.join(fullRoot, route), 'utf8');
  for (const match of html.matchAll(assetPattern)) {
    const assetRoute = match[1];
    if (!staticAssetPattern.test(assetRoute) || sources.has(assetRoute)) continue;
    const assetSource = sourceOrExistingAsset(assetRoute);
    if (assetSource) {
      sources.set(assetRoute, assetSource);
      if (assetRoute.endsWith('.css')) pendingCss.push(assetRoute);
    }
  }
}
while (pendingCss.length) {
  const cssRoute = pendingCss.pop();
  const css = fs.readFileSync(sources.get(cssRoute), 'utf8');
  for (const match of css.matchAll(/url\(["']?(\/spinon\/[^\s)"']+)["']?\)/g)) {
    const assetRoute = match[1].split(/[?#]/)[0].replace(/^\/spinon\//, '');
    if (!staticAssetPattern.test(assetRoute) || sources.has(assetRoute)) continue;
    const assetSource = sourceOrExistingAsset(assetRoute);
    if (assetSource) {
      sources.set(assetRoute, assetSource);
      if (assetRoute.endsWith('.css')) pendingCss.push(assetRoute);
    }
  }
}
if (!sources.has('icon.svg')) {
  const icon = sourceOrExistingAsset('icon.svg');
  if (icon) sources.set('icon.svg', icon);
}

fs.mkdirSync(selectedRoot, { recursive: true });
const selectedRoutes = [];
for (const [route, source] of [...sources].sort(([a], [b]) => a.localeCompare(b))) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe selected route: ${route}`);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing or unsafe selected source: ${route}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const outputBytes = fs.readFileSync(destination);
  assert.equal(sha256(outputBytes), sha256(bytes), `selected route hash mismatch: ${route}`);
  selectedRoutes.push({ path: route, bytes: outputBytes.length, sha256: sha256(outputBytes) });
}

const requiredMarkers = [
  ['roadmap.html', ['#664', '72/213', 'fit-content(40px)']],
  ['STATUS.md', ['C11.4', '72/213']],
  ['plan/c11-4-fit-content-auto-neighbor-runtime.html', ['[10,290]', 'fit-content-auto-neighbor-wpt']],
  ['plan/c11-4-fit-content-auto-neighbor-wpt.html', ['[10,290]']],
  ['internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/README.html', ['PixelCopy 5/5', 'Metal drawable 5/5']],
  ['internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/implementation-review-20.html', ['19 PASS', '1 PARTIAL']],
  ['internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/pr-diff-review-30.html', ['PASS 29', 'PARTIAL 1']],
  ['internal/evidence/c11-4-rspress-relative-image-path-fix-2026-10-27/pr-diff-review-30.html', ['PASS 30', 'FAIL 0']],
];
for (const [route, markers] of requiredMarkers) {
  const selected = path.join(selectedRoot, route);
  assert.ok(fs.lstatSync(selected).isFile(), `selected route is missing: ${route}`);
  const content = fs.readFileSync(selected, 'utf8').toLowerCase();
  for (const marker of markers) assert.ok(content.includes(marker.toLowerCase()), `${route} is missing marker: ${marker}`);
}
assert.ok(fs.readFileSync(path.join(selectedRoot, 'STATUS.md'), 'utf8').includes('- [ ] **C11.4'), 'C11.4 must remain unchecked');

const report = {
  pass: 'pass02',
  sourceHead,
  sourceTree,
  selectedRoot: rootManifest.selectedRoot,
  fullBuildRoot: rootManifest.outputRoot,
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  generatedRouteCount: generatedRoutes.size,
  selectedRouteCount: selectedRoutes.length,
  selectedRouteBytes: selectedRoutes.reduce((sum, entry) => sum + entry.bytes, 0),
  sourceRootOverlayCount: rootManifest.overlays.length,
  existingPreviewAssetsVerified: externalExisting.size,
  externalExistingAssets: [...externalExisting.values()],
  requiredMarkers,
  noDeletes: true,
  selectedRoutes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined, externalExistingAssets: undefined }, null, 2)}\n`);
