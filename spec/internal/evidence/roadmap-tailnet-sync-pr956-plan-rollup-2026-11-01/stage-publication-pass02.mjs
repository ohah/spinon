import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-plan-rollup-pr956-tailnet-20261101-pass02');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const publicationBase = path.join(buildBase, 'publication-pass02');
const staging = path.join(publicationBase, 'staging');
const publication = path.join(publicationBase, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass02.json'), 'utf8'));
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr956-plan-rollup-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication-tailnet-pass01.json'), 'utf8'));
const previousRoot = path.join(repo, previousManifest.publicationRoot);
const reportPath = path.join(evidence, 'selected-publication-tailnet-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const selected = new Map();
const changes = [];

for (const candidate of [publicationBase, staging, publication, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication 산출물을 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [root, site, previousRoot]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 publication source: ${directory}`);
}
fs.mkdirSync(staging, { recursive: true });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}

function put(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const prior = selected.get(route);
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  if (prior) {
    if (prior.sha256 === digest) return;
    fs.writeFileSync(target, bytes);
    changes.push({ path: route, previousSha256: prior.sha256, sha256: digest, previousSource: prior.source, source });
  } else {
    fs.writeFileSync(target, bytes, { flag: 'wx' });
  }
  selected.set(route, { path: route, bytes: bytes.length, sha256: digest, source, kind });
}

function walk(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) paths.push(...walk(directory, relative));
    else if (entry.isFile()) paths.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 source 항목: ${relative}`);
  }
  return paths;
}

const previousPaths = walk(previousRoot);
const previousByRoute = new Map(previousManifest.routes.map((route) => [route.path, route]));
assert.equal(previousPaths.length, previousByRoute.size, '이전 publication route 수가 manifest와 다름');
for (const route of previousPaths) {
  const bytes = fs.readFileSync(path.join(previousRoot, route));
  const expected = previousByRoute.get(route);
  assert.ok(expected, `이전 publication manifest에 없는 route: ${route}`);
  assert.equal(bytes.length, expected.bytes, `이전 publication 크기 불일치: ${route}`);
  assert.equal(sha256(bytes), expected.sha256, `이전 publication hash 불일치: ${route}`);
  put(route, bytes, `previous-publication:${route}`, 'preserved-route');
}

const generatedFiles = walk(site);
for (const route of generatedFiles) {
  if (route === 'index.html' || route === '404.html') continue;
  put(route, fs.readFileSync(path.join(site, route)), `rspress:${route}`, 'generated-page-or-asset');
}

for (const item of rootManifest.staged) {
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, `selected root 크기 불일치: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 불일치: ${item.route}`);
  put(item.route, bytes, `selected-root:${item.source}`, 'selected-source');
}

for (const name of ['roadmap.html', 'roadmap.svg', 'icon.svg', 'logo.svg']) {
  put(name, fs.readFileSync(path.join(root, 'public', name)), `selected-root:public/${name}`, 'public-alias');
}
put('tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html',
  fs.readFileSync(path.join(repo, 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html')),
  'repository:tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html', 'fixture-source');

const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
const marker = 'PR #952 Rebase and merge';
const markerIndex = roadmap.indexOf(marker);
assert.ok(markerIndex >= 0, 'PR #952 public roadmap card가 있어야 함');
const cardStart = roadmap.lastIndexOf('<p ', markerIndex);
const cardEnd = roadmap.indexOf('</p>', markerIndex);
assert.ok(cardStart >= 0 && cardEnd > markerIndex, 'PR #952 roadmap card 경계를 확인해야 함');
const card = roadmap.slice(cardStart, cardEnd + 4);
for (const expected of ['be62a4dd3c1b67ada2db094f19a76231e9f3df35', '72/213', 'text-transform']) {
  assert.ok(card.includes(expected), `PR #952 roadmap card marker 누락: ${expected}`);
}

const checkedCardLocalLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.equal(url.origin, 'https://preview.invalid', `외부 origin 링크를 local로 해석하지 않음: ${href}`);
  assert.ok(url.pathname.startsWith('/spinon/'), `예상하지 못한 local base: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
  assert.ok(selected.has(route), `PR #952 card local route 누락: ${href} -> ${route}`);
  checkedCardLocalLinks.push({ href, route, sha256: selected.get(route).sha256 });
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-css-grid.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-anonymous-grid-text-computed-text-transform.html',
  'plan/c11-4-anonymous-grid-text-computed-text-transform-plan-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/c114-computed-text-transform-android-avd-1793490026684-7e379b0a/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/c114-computed-text-transform-ios-simulator-1793490071946-86bc2cd5/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/post-merge-status-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/public-roadmap-pr-diff-review-30.html',
  'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

const sourceHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(rootManifest.sourceHead, sourceHead, 'selected root HEAD가 origin/main과 다름');
assert.equal(rootManifest.sourceTree, sourceTree, 'selected root tree가 origin/main과 다름');
assert.equal(execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(), sourceTree, 'checkout tree가 origin/main과 다름');
const trackedDiff = execFileSync('git', ['diff', '--binary', previousManifest.sourceHead, 'origin/main', '--'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead,
  sourceTree,
  trackedDiffSha256: sha256(trackedDiff),
  previousSourceHead: previousManifest.sourceHead,
  previousSourceTree: previousManifest.sourceTree,
  selectedRoot: rootManifest.selectedRoot,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  previousRouteCount: previousPaths.length,
  addedRoutes: routes.filter((route) => !previousByRoute.has(route.path)).length,
  replacedRoutes: changes.length,
  preservedRoutes: routes.filter((route) => previousByRoute.has(route.path) && route.sha256 === previousByRoute.get(route.path).sha256).length,
  noDeletes: true,
  fullDocsDeadLinkCheck: false,
  serverRestarted: false,
  requiredRoutes,
  roadmapMarkers: ['PR #952 Rebase and merge', 'be62a4dd3c1b67ada2db094f19a76231e9f3df35', '72/213'],
  checkedCardLocalLinks,
  changes,
  routes,
};
fs.renameSync(staging, publication);
fs.writeFileSync(reportPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined, changes: undefined, checkedCardLocalLinks: undefined }, null, 2)}\n`);
