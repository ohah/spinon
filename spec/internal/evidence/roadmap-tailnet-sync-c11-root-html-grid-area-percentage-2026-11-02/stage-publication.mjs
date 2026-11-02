import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-root-html-percentage-2026-11-02/pass02');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifestPath = path.join(evidence, 'selected-root.json');
const manifestPath = path.join(evidence, 'selected-publication.json');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const target of [staging, publication, manifestPath]) assert.ok(!fs.existsSync(target), `기존 publication 보존: ${target}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${path.basename(directory)}`);
}
fs.mkdirSync(staging, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 publication route: ${route}`);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route 내용 충돌: ${route}`);
    return;
  }
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
      if (route !== 'index.html' && route !== '404.html' && route !== 'spec/plan/c11-css-grid.html') {
        add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
      }
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
const marker = roadmap.indexOf('PR #1035 Rebase and merge');
assert.ok(marker >= 0, 'roadmap #1035 card 누락');
const paragraphStart = roadmap.lastIndexOf('<p ', marker);
const paragraphEnd = roadmap.indexOf('</p>', marker);
assert.ok(paragraphStart >= 0 && paragraphEnd >= 0, 'roadmap #1035 paragraph 경계 누락');
const card = roadmap.slice(paragraphStart, paragraphEnd + 4);
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
  assert.ok(byRoute.has(route), `roadmap card route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(card.split('PR #1035 Rebase and merge').length - 1, 1);
assert.ok(card.includes('4b921f3e8ccd86e7e156562bb7e5b4a464f082f2'));
assert.ok(card.includes('72/213'));
assert.ok(checkedLinks.length >= 7, `card local link count 예상 이하: ${checkedLinks.length}`);
for (const required of [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-css-grid.html',
  'plan/c11-root-html-grid-area-percentage.html',
  'internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/android-avd/overflowing-columns.png',
  'internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/ios-simulator/overflowing-columns.png',
]) assert.ok(byRoute.has(required), `필수 route 누락: ${required}`);
const status = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
assert.ok(status.includes('72/213') && status.includes('C11 CSS Grid'));

fs.renameSync(staging, publication);
const manifest = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  selectedRootFileCount: rootManifest.selectedRootFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213',
  c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  excludedRoutes: [
    { path: 'spec/plan/c11-css-grid.html', reason: '새 카드에서 사용하지 않는 계획 alias라 범위 밖으로 두고 미갱신' },
    { path: 'spec/plan/c11-css-grid.md', reason: '짝이 되는 raw-source alias는 새 카드/계획 링크에서 사용하지 않아 미게시' },
  ],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined }, null, 2)}\n`);
