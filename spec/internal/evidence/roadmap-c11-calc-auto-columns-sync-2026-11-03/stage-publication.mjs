import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-c11-calc-auto-columns-tailnet-sync-pass04-2026-11-03/pass01');
const root = path.join(base, 'root');
const site = '/tmp/spinon-c11-calc-auto-columns-site-selected-pass04-20261103';
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass04.json'), 'utf8'));
const manifestPath = path.join(evidence, 'selected-publication-pass04.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

for (const target of [staging, publication, manifestPath]) assert.ok(!fs.existsSync(target), `기존 publication 보존: ${target}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 directory: ${directory}`);
}
fs.mkdirSync(staging, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 route: ${route}`);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) { assert.equal(previous.sha256, digest, `route 충돌: ${route}`); return; }
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
      if (route !== 'index.html' && route !== '404.html') add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
    } else throw new Error(`지원하지 않는 generated site 항목: ${child}`);
  }
}
walkSite();
for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md') continue;
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, `selected-root size 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected-root hash 변경: ${item.route}`);
  add(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const marker = roadmap.indexOf('PR #1097 Rebase and merge');
assert.ok(marker >= 0, 'roadmap C11 card 누락');
const cardStart = roadmap.lastIndexOf('<p ', marker);
const cardEnd = roadmap.indexOf('</p>', marker);
assert.ok(cardStart >= 0 && cardEnd >= 0, 'roadmap card paragraph 경계 누락');
const card = roadmap.slice(cardStart, cardEnd + 4);
const checkedRoadmapLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상 밖 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(byRoute.has(route), `roadmap link route 누락: ${route}`);
  checkedRoadmapLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedRoadmapLinks.length, 5, 'roadmap 내부 링크 수가 예상과 다름');
const status = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
assert.ok(status.includes('72/213'), 'generated STATUS counter 누락');
assert.ok(status.includes('C11 CSS Grid'), 'generated STATUS C11 누락');
for (const required of [
  'roadmap.html', 'STATUS.html', 'internal/0059-c11-grid-layout.html',
  'plan/c11-css-grid.html', 'plan/c11-root-html-grid-column-flow-calc-auto-columns.html',
  'internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-runtime-2026-11-03/README.html',
  'internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-runtime-2026-11-03/implementation-pr-diff-review-30.html',
  'internal/evidence/roadmap-c11-calc-auto-columns-sync-2026-11-03/roadmap-pr-diff-review-30.html',
]) assert.ok(byRoute.has(required), `필수 route 누락: ${required}`);
fs.renameSync(staging, publication);
const sourceHead = git(['rev-parse', 'origin/main']).toString().trim();
const sourceTree = git(['rev-parse', 'origin/main^{tree}']).toString().trim();
assert.equal(git(['rev-parse', 'HEAD^{tree}']).toString().trim(), sourceTree, 'checkout tree가 origin/main과 다름');
assert.equal(git(['diff', '--binary', 'origin/main']).length, 0, 'origin/main 대비 tracked diff가 있음');
const report = {
  sourceHead, sourceTree, trackedDiffSha256: sha256(git(['diff', '--binary', 'origin/main'])),
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length, totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213', c11Complete: false,
  selectedBuild: 'RSPress 2.0.22; curated tracked source root; dead-link/image checking disabled for selected publication only',
  checkedRoadmapLinks, requiredRoutes: ['roadmap.html', 'STATUS.html', 'plan/c11-css-grid.html', 'plan/c11-root-html-grid-column-flow-calc-auto-columns.html'],
  excludedRoutes: ['index.html', '404.html'], noDeletes: true, routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
