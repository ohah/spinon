import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/c11-4-word-spacing-post-merge-20261031/pass06');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publication = path.join(base, 'publication-pass05');
const staging = path.join(base, 'publication-staging-pass05');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass05.json'), 'utf8'));
const reportPath = path.join(evidence, 'selected-publication-pass05.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [publication, staging, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication을 덮어쓰지 않기 위해 중단: ${candidate}`);
}
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(staging, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}
function addBytes(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route source 충돌: ${route}`);
    return;
  }
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `staging hash 불일치: ${route}`);
  selected.set(route, { path: route, source, kind, bytes: bytes.length, sha256: digest });
}
function walkSite(relative = '') {
  for (const entry of fs.readdirSync(path.join(site, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(site, child);
    assert.ok(!entry.isSymbolicLink(), `generated site symlink 거부: ${child}`);
    if (entry.isDirectory()) walkSite(child);
    else if (entry.isFile()) {
      const route = child.split(path.sep).join('/');
      if (route !== 'index.html' && route !== '404.html') addBytes(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
    } else throw new Error(`지원하지 않는 generated site 항목: ${child}`);
  }
}

walkSite();
for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md') continue;
  const source = path.join(root, item.route);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `selected root 크기 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 변경: ${item.route}`);
  addBytes(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}

const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
for (const marker of ['PR #924', '93aba4eff3998c54c595b661578b339aeda89d87', 'computed <code>word-spacing</code>', '24/24', '72/213', 'glyph paint']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
const status = fs.readFileSync(path.join(staging, 'STATUS.html'), 'utf8');
for (const marker of ['72/213', 'word-spacing', '93aba4eff3998c54c595b661578b339aeda89d87', 'C11.4']) assert.ok(status.includes(marker), `rendered STATUS marker 누락: ${marker}`);

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-css-grid.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-anonymous-grid-text-computed-word-spacing.html',
  'plan/c11-4-anonymous-grid-text-computed-word-spacing-plan-review-20.html',
  'plan/c11-4-anonymous-grid-text-computed-word-spacing-plan-pr-diff-review-20.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/chrome156-agent-browser.png',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/spec-derived-expectations.json',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/android-avd/matrix.tsv',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/android-avd/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/ios-simulator/matrix.tsv',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/ios-simulator/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/artifact-boundary/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/pr-diff-review-30.html',
  'internal/evidence/c11-4-word-spacing-post-merge-2026-10-31/README.html',
  'internal/evidence/c11-4-word-spacing-post-merge-2026-10-31/pr-diff-review-30.html',
  'internal/evidence/c11-4-word-spacing-post-merge-2026-10-31/tailnet-roadmap-agent-browser.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

fs.renameSync(staging, publication);
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  selectedRootFileCount: rootManifest.stagedFileCount,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  requiredRoutes,
  roadmapMarkers: ['PR #924', '93aba4eff3998c54c595b661578b339aeda89d87', '24/24', '72/213'],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
