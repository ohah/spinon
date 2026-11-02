import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1073-2026-11-03/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const manifestPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const target of [staging, publication, manifestPath]) assert.ok(!fs.existsSync(target), `기존 publication 보존: ${target}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
fs.mkdirSync(staging, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) { assert.equal(previous.sha256, digest, `route 내용 충돌: ${route}`); return; }
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `publication hash 불일치: ${route}`);
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
      if (!['index.html', '404.html', 'spec/plan/c11-css-grid.html'].includes(route)) add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
    } else throw new Error(`지원하지 않는 generated site 항목: ${child}`);
  }
}
walkSite();
for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md' || item.route === 'spec/plan/c11-css-grid.md') continue;
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, `selected root 크기 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 변경: ${item.route}`);
  add(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const start = roadmap.indexOf('PR #1072 Rebase and merge');
assert.ok(start >= 0, 'PR #1072 roadmap card 누락');
const cardStart = roadmap.lastIndexOf('<p ', start);
const cardEnd = roadmap.indexOf('</p>', start);
assert.ok(cardStart >= 0 && cardEnd >= 0, 'PR #1072 roadmap paragraph 경계 누락');
const card = roadmap.slice(cardStart, cardEnd + 4);
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
  assert.ok(byRoute.has(route), `PR #1072 카드 링크 route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedLinks.length, 8, `PR #1072 카드 링크 수 불일치: ${checkedLinks.length}`);
for (const required of [
  'roadmap.html', 'STATUS.html', 'plan/c11-css-grid.html', 'plan/c11-root-html-grid-percentage-gap.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/README.html',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/roadmap-pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/android-api37-avd.png',
  'internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/ios-26_2-simulator.png',
]) assert.ok(byRoute.has(required), `필수 route 누락: ${required}`);
for (const marker of ['6508a1a91d04c7e83c00b10f230519d52e8ed86b', '72/213']) assert.ok(card.includes(marker), `roadmap marker 누락: ${marker}`);
const status = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
assert.ok(status.includes('C11 CSS Grid') && status.includes('72/213'), 'published status C11/count marker 누락');
fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213', c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  excludedRoutes: [{ path: 'spec/plan/c11-css-grid.html', reason: 'the pre-existing preview server still holds open readers; the live /plan/c11-css-grid.html route is included' }, { path: 'spec/plan/c11-css-grid.md', reason: 'paired raw-source alias was excluded with the open spec/plan route' }],
  openHandlePolicy: 'Existing readers are preserved on their old inode; every changed destination is backed up and replaced by atomic rename. No route is edited in place and no files are deleted.',
  fullDocsLinkValidation: false, noDeletes: true, routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
