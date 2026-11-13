import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1375-final-2026-11-14');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-final.json'), 'utf8'));
const manifestPath = path.join(evidence, 'selected-publication-final.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const target of [staging, publication, manifestPath]) {
  assert.ok(!fs.existsSync(target), '기존 publication 산출물 보존: ' + target);
}
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), '안전하지 않은 디렉터리: ' + directory);
}
fs.mkdirSync(staging, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), '잘못된 publication route: ' + route);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), '잘못된 publication route: ' + route);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, '같은 route의 source 내용이 다름: ' + route);
    return;
  }
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, 'publication hash 불일치: ' + route);
  selected.set(route, { path: route, source, kind, bytes: bytes.length, sha256: digest });
}
function walkSite(relative) {
  const directory = relative ? path.join(site, relative) : site;
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? relative + '/' + entry.name : entry.name;
    const absolute = path.join(site, child);
    assert.ok(!entry.isSymbolicLink(), 'generated site symlink 거부: ' + child);
    if (entry.isDirectory()) walkSite(child);
    else if (entry.isFile()) {
      if (child === 'index.html' || child === '404.html') continue;
      add(child.split(path.sep).join('/'), fs.readFileSync(absolute), 'rspress:' + child, 'generated-page-or-asset');
    } else throw new Error('지원하지 않는 generated site 항목: ' + child);
  }
}
walkSite('');

for (const item of rootManifest.files) {
  if (item.source === 'generated:index.md') continue;
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, 'selected root 크기 변경: ' + item.route);
  assert.equal(sha256(bytes), item.sha256, 'selected root hash 변경: ' + item.route);
  add(item.route, bytes, 'selected-root:' + item.source, 'source-page-or-asset');
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const headingIndex = roadmap.indexOf('최근 병합 bounded slice:');
const start = roadmap.lastIndexOf('<span>', headingIndex);
const end = roadmap.indexOf('</span>', headingIndex);
assert.ok(headingIndex >= 0 && start >= 0 && end >= 0, 'roadmap current-card span 누락');
const card = roadmap.slice(start, end + 7);
assert.ok(card.includes('#1375 Rebase and merge'), 'roadmap PR #1375 current card 누락');
assert.ok(card.includes('22161976cab0bfd68a1d545eeb616441a0ab7a0e'), 'roadmap merge SHA 누락');
const checkedLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), '예상하지 않은 local href: ' + href);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  safeRoute(route);
  assert.ok(byRoute.has(route), 'roadmap local link route 누락: ' + route);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.ok(checkedLinks.length >= 5, 'roadmap local link가 예상보다 적음');
for (const required of [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum.html',
  'plan/c11-4-grid-track-sizing.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-runtime-2026-11-13/README.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-runtime-2026-11-13/merge-PR1375.html',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/post-merge-docs-pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/agent-browser-verification.json',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/simulator-cleanup.json',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/tailnet-roadmap-desktop-agent-browser.png',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/tailnet-roadmap-mobile-agent-browser.png',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/tailnet-roadmap-mobile-kanban-scroll-agent-browser.png',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/selected-publication.json',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/preflight.json',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/copy-and-backup.json',
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14/served-verification.json',
]) assert.ok(byRoute.has(required), '필수 route 누락: ' + required);
const renderedRoadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['#1375 Rebase and merge', '22161976cab0bfd68a1d545eeb616441a0ab7a0e', '78/214']) {
  assert.ok(renderedRoadmap.includes(marker), 'roadmap publication marker 누락: ' + marker);
}
const renderedStatus = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
assert.ok(renderedStatus.includes('78/214'), 'rendered STATUS official count 불일치');
assert.ok(!renderedStatus.includes('78/215'), 'rendered STATUS가 공식 count를 변경함');

fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '78/214',
  c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  excludedRoutes: [
    { path: 'index.html and 404.html', reason: 'existing Tailnet preview entry and fallback are outside this selected roadmap update' },
    { path: 'spec/plan/c11-css-grid.html', reason: 'unrelated to the #1375 current card; preserve existing route' },
  ],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, routes: undefined }, null, 2) + '\n');
