import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const fullRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass02-full');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass02-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(fullRoot).isDirectory(), 'full Rspress build is required');
for (const candidate of [selectedRoot, reportPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const generatedRoutes = [
  'STATUS.html', 'spec/STATUS.html',
  'plan/c11-3-3-unknown-named-area-implicit-lines.html', 'spec/plan/c11-3-3-unknown-named-area-implicit-lines.html',
  'plan/c11-4-fixed-repeat-minmax-flex.html', 'spec/plan/c11-4-fixed-repeat-minmax-flex.html',
  'plan/c11-4-grid-track-sizing.html', 'spec/plan/c11-4-grid-track-sizing.html',
  'internal/0059-c11-grid-layout.html', 'spec/internal/0059-c11-grid-layout.html',
  'internal/evidence/roadmap-code-status-sync-2026-10-25/pr-diff-review-30.html',
  'internal/evidence/c11-4-pr615-merge-status-2026-10-25/pr-diff-review-30.html',
  'internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/README.html',
  'internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/pr-diff-review-30.html',
  'internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.html',
  'internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/README.html',
  'internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/implementation-review-20.html',
  'internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/pr-diff-review-30.html',
  'internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/artifact-boundary/README.html',
];
const rawFiles = [
  { source: 'spec/public/roadmap.html', routes: ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'] },
  { source: 'spec/STATUS.md', routes: ['STATUS.md', 'spec/STATUS.md'] },
  { source: 'plan/c11-3-3-unknown-named-area-implicit-lines.md', routes: ['plan/c11-3-3-unknown-named-area-implicit-lines.md', 'spec/plan/c11-3-3-unknown-named-area-implicit-lines.md'] },
  { source: 'plan/c11-4-fixed-repeat-minmax-flex.md', routes: ['plan/c11-4-fixed-repeat-minmax-flex.md', 'spec/plan/c11-4-fixed-repeat-minmax-flex.md'] },
  { source: 'plan/c11-4-grid-track-sizing.md', routes: ['plan/c11-4-grid-track-sizing.md', 'spec/plan/c11-4-grid-track-sizing.md'] },
  { source: 'spec/internal/0059-c11-grid-layout.md', routes: ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'] },
];
const rawDirectories = [
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-25',
  'spec/internal/evidence/c11-4-pr615-merge-status-2026-10-25',
  'spec/internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25',
];

function walk(relativeDirectory) {
  const absolute = path.join(repositoryRoot, relativeDirectory);
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe source directory: ${relativeDirectory}`);
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

fs.mkdirSync(selectedRoot, { recursive: true });
const routeSources = new Map();
for (const route of generatedRoutes) routeSources.set(route, path.join(fullRoot, route));
for (const file of rawFiles) for (const route of file.routes) routeSources.set(route, path.join(repositoryRoot, file.source));
for (const directory of rawDirectories) {
  for (const source of walk(directory)) {
    const suffix = source.slice('spec/'.length);
    for (const route of [suffix, source]) routeSources.set(route, path.join(repositoryRoot, source));
  }
}
assert.equal(routeSources.size, generatedRoutes.length + rawFiles.reduce((sum, entry) => sum + entry.routes.length, 0)
  + rawDirectories.reduce((sum, directory) => sum + walk(directory).length * 2, 0), 'selected routes must not overlap');

const selectedRoutes = [];
for (const [route, source] of [...routeSources].sort(([a], [b]) => a.localeCompare(b))) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `missing/unsafe selected route: ${route}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const temporary = `${destination}.spinon-sync-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  fs.renameSync(temporary, destination);
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), `route hash mismatch: ${route}`);
  selectedRoutes.push({ path: route, bytes: copied.length, sha256: sha256(copied) });
}

const markers = [
  ['roadmap.html', ['72/213', '12dc2669814c4d5e7d0e2aa59d5196fdab98bb68', 'T235142067Z-4887fd16', 'T235205801Z-de685760', '0ccec669e5506b6caef53314702c7fb60090e45c']],
  ['plan/c11-3-3-unknown-named-area-implicit-lines.html', ['PR #393', '12dc2669814c4d5e7d0e2aa59d5196fdab98bb68']],
  ['internal/0059-c11-grid-layout.html', ['PR #393', '12dc2669814c4d5e7d0e2aa59d5196fdab98bb68', '0ccec669e5506b6caef53314702c7fb60090e45c']],
  ['plan/c11-4-fixed-repeat-minmax-flex.html', ['PR #615', '0ccec669e5506b6caef53314702c7fb60090e45c']],
  ['internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/README.html', ['Chrome 156', 'Android API 37', 'iPhone 17 Pro']],
  ['internal/evidence/roadmap-code-status-sync-2026-10-25/pr-diff-review-30.html', ['30', '차단 finding']],
];
for (const [route, routeMarkers] of markers) {
  const html = fs.readFileSync(path.join(selectedRoot, route), 'utf8');
  for (const marker of routeMarkers) assert.ok(html.includes(marker), `${route} missing marker: ${marker}`);
}

const sourceHead = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-pass02.json'), 'utf8')).mergedHead;
const report = {
  sourceHead,
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  generatedPages: generatedRoutes.length,
  selectedRouteCount: selectedRoutes.length,
  totalBytes: selectedRoutes.reduce((sum, entry) => sum + entry.bytes, 0),
  selectedRoutes,
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined }, null, 2)}\n`);
