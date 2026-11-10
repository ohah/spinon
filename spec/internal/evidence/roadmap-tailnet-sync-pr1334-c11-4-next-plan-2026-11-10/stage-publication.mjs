import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const [pass = 'pass01'] = process.argv.slice(2);
assert.match(pass, /^pass[0-9]+$/u, `잘못된 pass 이름: ${pass}`);
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1334-c11-4-next-plan-2026-11-10', pass);
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const passSuffix = pass === 'pass01' ? '' : `-${pass}`;
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, `selected-root${passSuffix}.json`), 'utf8'));
const manifestPath = path.join(evidence, `selected-publication${passSuffix}.json`);
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
      if (route !== 'index.html' && route !== '404.html') {
        add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
      }
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
  add(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const cardStart = roadmap.indexOf('<strong>2026-11-10 C11.4 parsed Root HTML six-row single-fit repeated-text row-span positions 4–6');
assert.ok(cardStart >= 0, 'roadmap #1331 진행 요약 누락');
const paragraphStart = roadmap.lastIndexOf('<p class="kanban-note hero-progress">', cardStart);
const paragraphEnd = roadmap.indexOf('</p>', cardStart);
assert.ok(paragraphStart >= 0 && paragraphEnd >= 0, '#1331 roadmap paragraph 경계 누락');
const card = roadmap.slice(paragraphStart, paragraphEnd + 4);
assert.ok(card.includes('4b4f48ea67ab4383954cc89cc21e1a71bdb29732'), 'roadmap merge SHA 누락');
assert.ok(roadmap.includes('78/214'), 'roadmap 공식 완료 수 누락');

const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
  assert.ok(byRoute.has(route), `#1331 roadmap 링크 route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedLinks.length, 2, `#1331 progress paragraph local-link count 변경: ${checkedLinks.length}`);

const nextPlanStart = roadmap.indexOf('<strong>2026-11-10 C11.4 다음 bounded 계획');
assert.ok(nextPlanStart >= 0, '#1334 다음 계획 roadmap paragraph 누락');
const nextParagraphStart = roadmap.lastIndexOf('<p class="kanban-note hero-progress">', nextPlanStart);
const nextParagraphEnd = roadmap.indexOf('</p>', nextPlanStart);
assert.ok(nextParagraphStart >= 0 && nextParagraphEnd >= 0, '#1334 roadmap paragraph 경계 누락');
const nextParagraph = roadmap.slice(nextParagraphStart, nextParagraphEnd + 4);
assert.ok(nextParagraph.includes('50678c278dc235bfb09ad2bf24e4ef174a46358a'), '#1334 plan PR merge SHA 누락');
const checkedNextLinks = [];
for (const [, href] of nextParagraph.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 next-plan local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 next-plan href: ${href}`);
  assert.ok(byRoute.has(route), `next-plan roadmap 링크 route 누락: ${route}`);
  checkedNextLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedNextLinks.length, 3, `next-plan progress paragraph local-link count 변경: ${checkedNextLinks.length}`);

const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(status.includes('78/214'), 'STATUS 집계 불일치');
assert.match(status, /- \[ \] <span id="c11-css-grid"/u, 'C11 checkbox가 미완료가 아님');
assert.match(status, /- \[ \] \*\*C11\.4 intrinsic track sizing\*\*/u, 'C11.4 checkbox가 미완료가 아님');
const contract = fs.readFileSync(path.join(root, 'internal/0059-c11-grid-layout.md'), 'utf8');
assert.ok(contract.includes('[#1331]') && contract.includes('4b4f48ea67ab4383954cc89cc21e1a71bdb29732'), '내부 계약 병합 metadata 불일치');

fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '78/214',
  c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  checkedNextPlanLinks: checkedNextLinks,
  excludedRoutes: [
    { path: 'index.html', reason: '기존 Tailnet preview 시작 페이지를 보존함' },
    { path: '404.html', reason: '기존 Tailnet 404 route를 보존함' },
  ],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
