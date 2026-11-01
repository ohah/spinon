import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr987-2026-11-01/pass01');
const selectedRoot = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-c11-4-hyphens-auto-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication-pass02.json'), 'utf8'));
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const previousRoot = path.join(repo, previousManifest.publicationRoot);
const publication = path.join(buildBase, 'publication-pass02');
const reportPath = path.join(evidence, 'selected-publication-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const selected = new Map();

for (const target of [publication, reportPath]) assert.ok(!fs.existsSync(target), `기존 산출물을 덮어쓰지 않음: ${target}`);

function walk(directory, prefix = '') {
  const routes = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) routes.push(...walk(directory, relative));
    else if (entry.isFile()) routes.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 파일 형식: ${relative}`);
  }
  return routes;
}

function put(route, bytes, source) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..') && !route.includes('\\'), `잘못된 route: ${route}`);
  const target = path.join(publication, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes);
  selected.set(route, { path: route, bytes: bytes.length, sha256: sha256(bytes), source });
}

const oldRoutes = walk(previousRoot);
const oldManifestByPath = new Map(previousManifest.routes.map((route) => [route.path, route]));
assert.equal(oldRoutes.length, oldManifestByPath.size, '이전 publication route 수가 manifest와 다름');
for (const route of oldRoutes) {
  const bytes = fs.readFileSync(path.join(previousRoot, route));
  const expected = oldManifestByPath.get(route);
  assert.ok(expected, `이전 manifest route 누락: ${route}`);
  assert.equal(bytes.length, expected.bytes, `이전 publication 크기 불일치: ${route}`);
  assert.equal(sha256(bytes), expected.sha256, `이전 publication hash 불일치: ${route}`);
}
fs.cpSync(previousRoot, publication, { recursive: true, errorOnExist: true, force: false });
for (const route of oldRoutes) {
  const bytes = fs.readFileSync(path.join(publication, route));
  selected.set(route, { path: route, bytes: bytes.length, sha256: sha256(bytes), source: `previous-publication:${route}` });
}

for (const route of walk(site)) {
  if (route === 'index.html' || route === '404.html') continue;
  if (route.endsWith('/ios-simulator/build.log')) continue;
  put(route, fs.readFileSync(path.join(site, route)), `rspress:${route}`);
}
for (const item of rootManifest.staged) {
  if (item.route.endsWith('/ios-simulator/build.log')) continue;
  const bytes = fs.readFileSync(path.join(selectedRoot, item.route));
  assert.equal(bytes.length, item.bytes, `selected root 크기 불일치: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 불일치: ${item.route}`);
  put(item.route, bytes, `selected-root:${item.route}`);
}
for (const name of ['roadmap.html', 'roadmap.svg', 'icon.svg', 'logo.svg']) {
  const source = path.join(selectedRoot, 'public', name);
  if (fs.existsSync(source)) put(name, fs.readFileSync(source), `selected-root:public/${name}`);
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'public/roadmap.html',
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-css-grid.html',
  'plan/c11-4-anonymous-grid-text-transform-capitalize.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-post-merge-docs-review-30-2026-11-01.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/chrome156-text-transform-capitalize.png',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/android-avd-pass02/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/ios-simulator-pass04/representative-screen.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
for (const marker of ['#986 Rebase and merge', 'd5951d09390c746d29389e821f24088fbf51245b', '72/213', 'status=-12']) {
  assert.ok(roadmap.includes(marker), `로드맵 marker 누락: ${marker}`);
}
const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">([\s\S]*?)<\/p>/g)]
  .map((match) => match[1])
  .find((html) => html.includes('#986 Rebase and merge'));
assert.ok(card, 'PR #986 roadmap card not found');
const checkedLocalLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.equal(url.origin, 'https://preview.invalid', `외부 origin을 local 링크로 해석하지 않음: ${href}`);
  assert.ok(url.pathname.startsWith('/spinon/'), `기준 경로가 다른 local 링크: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length)) || 'index.html';
  assert.ok(selected.has(route), `PR #986 카드 local route 누락: ${href} -> ${route}`);
  checkedLocalLinks.push({ href, route, sha256: selected.get(route).sha256 });
}
assert.ok(checkedLocalLinks.some((item) => item.route === requiredRoutes[14]), 'PR diff review route를 검사하지 않음');

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const changedRoutes = routes.filter((route) => oldManifestByPath.get(route.path)?.sha256 !== route.sha256);
const manifest = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  previousPublication: previousManifest.publicationRoot,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  previousRouteCount: oldRoutes.length,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  addedRoutes: routes.filter((route) => !oldManifestByPath.has(route.path)).length,
  changedRouteCount: changedRoutes.length,
  fullDocsDeadLinkCheck: false,
  noDeletes: true,
  previewServerRestarted: false,
  requiredRoutes,
  checkedLocalLinks,
  changedRoutes,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined, changedRoutes: undefined, checkedLocalLinks: undefined }, null, 2)}\n`);
