import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const fullRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass08-corrected-full');
const baseRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass07-selected-retry02');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass08-corrected-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass08-corrected.json');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root-pass08-corrected.json');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sourceHead = rootManifest.sourceHead;
const sourceTree = rootManifest.sourceTree;
assert.equal(execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceHead);
assert.equal(execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), sourceTree);
const trackedMainSources = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', sourceHead, '--', 'spec/STATUS.md', 'spec/public/roadmap.html', 'plan/c11-4-grid-track-sizing.md', 'plan/c11-4-cyclic-percentage-auto-repeat.md', 'plan/c11-4-orthogonal-row-intrinsic-contribution.md', 'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25', 'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25', 'spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25', 'spec/internal/evidence/c11-4-parallel-next-slices-plan-pr-diff-review-20.md'], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));

for (const root of [fullRoot, baseRoot]) assert.ok(fs.lstatSync(root).isDirectory(), `required root missing: ${root}`);
for (const candidate of [selectedRoot, reportPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const generatedRoutes = [
  'STATUS.html', 'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html', 'spec/plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-cyclic-percentage-auto-repeat.html', 'spec/plan/c11-4-cyclic-percentage-auto-repeat.html',
  'plan/c11-4-orthogonal-row-intrinsic-contribution.html', 'spec/plan/c11-4-orthogonal-row-intrinsic-contribution.html',
  'internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.html',
  'internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.html',
  'internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/merge-roadmap-pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.html',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/implementation-runtime-review-20.html',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/merge-roadmap-pr-diff-review-30.html',
  'internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/plan-review-20.html',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.html',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/plan-review-20.html',
  'internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.html',
  'internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/plan-review-20.html',
  'spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.html',
  'spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/plan-review-20.html',
  'internal/evidence/c11-4-parallel-next-slices-plan-pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-parallel-next-slices-plan-pr-diff-review-20.html',
];
const rawFiles = [
  { source: 'spec/public/roadmap.html', routes: ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'] },
  { source: 'spec/STATUS.md', routes: ['STATUS.md', 'spec/STATUS.md'] },
  { source: 'plan/c11-4-grid-track-sizing.md', routes: ['plan/c11-4-grid-track-sizing.md', 'spec/plan/c11-4-grid-track-sizing.md'] },
  { source: 'plan/c11-4-cyclic-percentage-auto-repeat.md', routes: ['plan/c11-4-cyclic-percentage-auto-repeat.md', 'spec/plan/c11-4-cyclic-percentage-auto-repeat.md'] },
  { source: 'plan/c11-4-orthogonal-row-intrinsic-contribution.md', routes: ['plan/c11-4-orthogonal-row-intrinsic-contribution.md', 'spec/plan/c11-4-orthogonal-row-intrinsic-contribution.md'] },
];
const rawDirectories = [
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25',
  'spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25',
  'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25',
];

function walk(relativeDirectory) {
  const absolute = path.join(repositoryRoot, relativeDirectory);
  assert.ok(fs.lstatSync(absolute).isDirectory(), `unsafe source directory: ${relativeDirectory}`);
  const files = [];
  for (const entry of fs.readdirSync(absolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.posix.join(relativeDirectory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in source directory: ${child}`);
    if (entry.isDirectory()) files.push(...walk(child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`unsupported source entry: ${child}`);
  }
  return files;
}

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const from = path.join(source, entry.name);
    const to = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base selected output: ${from}`);
    if (entry.isDirectory()) hardlinkTree(from, to);
    else if (entry.isFile()) fs.linkSync(from, to);
    else throw new Error(`unsupported base output entry: ${from}`);
  }
}

const sources = new Map();
for (const route of generatedRoutes) sources.set(route, path.join(fullRoot, route));
for (const { source, routes } of rawFiles) {
  assert.ok(trackedMainSources.has(source), 'raw source is absent from origin/main: ' + source);
  const absolute = path.join(repositoryRoot, source);
  const mainBytes = execFileSync('git', ['show', sourceHead + ':' + source], { cwd: repositoryRoot });
  assert.deepEqual(fs.readFileSync(absolute), mainBytes, 'raw source differs from origin/main: ' + source);
  for (const route of routes) sources.set(route, absolute);
}
for (const file of ['spec/internal/evidence/c11-4-parallel-next-slices-plan-pr-diff-review-20.md']) {
  const withoutSpec = file.slice('spec/'.length);
  for (const route of [file, withoutSpec, `public/${withoutSpec}`, `public/${file}`]) {
    sources.set(route, path.join(repositoryRoot, file));
  }
}
for (const directory of rawDirectories) {
  for (const source of walk(directory)) {
    if (!trackedMainSources.has(source)) continue;
    const sourceBytes = fs.readFileSync(path.join(repositoryRoot, source));
    const mainBytes = execFileSync('git', ['show', sourceHead + ':' + source], { cwd: repositoryRoot });
    assert.deepEqual(sourceBytes, mainBytes, 'evidence source differs from origin/main: ' + source);
    const withoutSpec = source.slice('spec/'.length);
    for (const route of [source, withoutSpec, `public/${withoutSpec}`, `public/${source}`]) {
      sources.set(route, path.join(repositoryRoot, source));
    }
  }
}

const assetPattern = /(?:href|src)=["']\/spinon\/([^"'#?]+)["']/g;
for (const route of generatedRoutes) {
  const htmlPath = path.join(fullRoot, route);
  assert.ok(fs.lstatSync(htmlPath).isFile(), `generated page missing: ${route}`);
  const html = fs.readFileSync(htmlPath, 'utf8');
  for (const match of html.matchAll(assetPattern)) {
    const assetRoute = match[1];
    if (!/\.(?:css|js|svg|png|jpe?g|webp|gif|woff2?|ttf|ico)$/i.test(assetRoute)) continue;
    if (sources.has(assetRoute)) continue;
    const builtAsset = path.join(fullRoot, assetRoute);
    if (fs.existsSync(builtAsset)) sources.set(assetRoute, builtAsset);
    else assert.ok(fs.existsSync(path.join(previewRoot, assetRoute)), `asset missing from build and preview: ${assetRoute}`);
  }
}

for (const route of sources.keys()) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}
hardlinkTree(baseRoot, selectedRoot);

const selectedRoutes = [];
for (const [route, source] of [...sources].sort(([a], [b]) => a.localeCompare(b))) {
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `missing/unsafe selected source: ${route}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const previous = fs.lstatSync(destination);
    assert.ok(previous.isFile() && !previous.isSymbolicLink(), `unsafe selected destination: ${route}`);
    fs.unlinkSync(destination);
  }
  const temporary = `${destination}.spinon-pass08-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  fs.renameSync(temporary, destination);
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), `route hash mismatch: ${route}`);
  selectedRoutes.push({ path: route, bytes: copied.length, sha256: sha256(copied) });
}

const markers = [
  ['roadmap.html', ['#623', '72/213', 'orthogonal']],
  ['STATUS.md', ['C11.4', '72/213', 'cyclic percentage']],
  ['plan/c11-4-cyclic-percentage-auto-repeat.html', ['intrinsic max-content', 'percentage basis', '72/213']],
  ['plan/c11-4-orthogonal-row-intrinsic-contribution.html', ['vertical-rl', 'feasibility gate', '72/213']],
  ['internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.html', ['cyclic percentage', '72/213']],
  ['internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/merge-roadmap-pr-diff-review-30.html', ['30 PASS', 'PR #624']],
  ['internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.html', ['[12px,12px]', 'intrinsic pass', 'Chrome 156']],
  ['internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.html', ['[50]', '[80]', 'HeadlessChrome/156.0.0.0']],
];
for (const [route, required] of markers) {
  const content = fs.readFileSync(path.join(selectedRoot, route), 'utf8').toLowerCase();
  for (const marker of required) assert.ok(content.includes(marker.toLowerCase()), `${route} missing marker: ${marker}`);
}
assert.ok(fs.readFileSync(path.join(selectedRoot, 'STATUS.md'), 'utf8').includes('- [ ] **C11.4'), 'C11.4 must remain unchecked');

const report = {
  pass: 'pass08-corrected',
  sourceHead,
  checkoutHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(),
  originMainHead: sourceHead,
  sourceTree,
  excludedUntrackedFiles: rootManifest.excludedUntrackedFiles,
  baseSelectedOutputRoot: path.relative(repositoryRoot, baseRoot),
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  generatedPageCount: generatedRoutes.length,
  selectedRouteCount: selectedRoutes.length,
  totalBytes: selectedRoutes.reduce((sum, entry) => sum + entry.bytes, 0),
  markers,
  noDeletes: true,
  selectedRoutes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined }, null, 2)}\n`);
