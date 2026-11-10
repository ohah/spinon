import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const [pass = 'pass01'] = process.argv.slice(2);
assert.match(pass, /^pass[0-9]+$/u, `잘못된 pass 이름: ${pass}`);
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1326-c11-4-2026-11-10', pass);
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
const cardStart = roadmap.indexOf('최근 병합 bounded slice:');
assert.ok(cardStart >= 0 && roadmap.includes('#1326 Rebase and merge', cardStart), 'roadmap #1326 병합 카드 누락');
const cardTagStart = roadmap.lastIndexOf('<span>', cardStart);
const cardTagEnd = roadmap.indexOf('</span>', cardStart);
assert.ok(cardTagStart >= 0 && cardTagEnd >= 0, 'roadmap heading span 경계 누락');
const card = roadmap.slice(cardTagStart, cardTagEnd + '</span>'.length);
assert.ok(card.includes('b5925aa274718a16c4e8f0bd9ef4bed4423aec6c'), 'roadmap merge SHA 누락');
assert.ok(roadmap.includes('78/214'), 'roadmap 공식 완료 수 누락');

const planMarker = 'PR #1324 Rebase and merge';
const planMarkerIndex = roadmap.indexOf(planMarker);
assert.ok(planMarkerIndex >= 0, 'roadmap #1324 plan card 누락');
const planParagraphStart = roadmap.lastIndexOf('<p class="kanban-note hero-progress">', planMarkerIndex);
const planParagraphEnd = roadmap.indexOf('</p>', planMarkerIndex);
assert.ok(planParagraphStart >= 0 && planParagraphEnd >= 0, 'roadmap #1324 paragraph 경계 누락');
const planParagraph = roadmap.slice(planParagraphStart, planParagraphEnd + 4);
assert.ok(planParagraph.includes('f14b9bc640ed770d132b09f065e967a0bee79596'), '#1324 merge SHA 누락');
assert.ok(planParagraph.includes('78/214') && planParagraph.includes('0.1.0'), '#1324 상태·계약 경계 누락');
assert.ok(planParagraph.includes('Chrome 사전 비교 자료다'), 'Chrome 사전 비교 맥락 누락');
const checkedPlanLinks = [];
for (const [, href] of planParagraph.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), '예상하지 않은 #1324 local href: ' + href);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), '안전하지 않은 #1324 href: ' + href);
  assert.ok(byRoute.has(route), '#1324 roadmap 링크 route 누락: ' + route);
  checkedPlanLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedPlanLinks.length, 3, '#1324 card local-link count 변경: ' + checkedPlanLinks.length);
assert.ok(checkedPlanLinks.some((item) => item.route === 'plan/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum.html'), '#1324 계획 route 미검증');
assert.ok(checkedPlanLinks.some((item) => item.route === 'internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/README.html'), '#1324 비교 evidence route 미검증');
assert.ok(checkedPlanLinks.some((item) => item.route === 'internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-2026-11-10/README.html'), '#1324 implementation evidence route 미검증');
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
  assert.ok(byRoute.has(route), `#1326 roadmap 링크 route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.equal(checkedLinks.length, 3, `#1326 card local-link count 변경: ${checkedLinks.length}`);
const planRoute = 'plan/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum.html';
assert.ok(roadmap.includes(`/spinon/${planRoute}`), '공개 roadmap의 one-fit 계획 href 누락');
assert.ok(byRoute.has(planRoute), 'one-fit 계획의 selected publication route 누락');
const implementationEvidenceRoute = 'internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum-simulators-2026-11-10/README.html';
assert.ok(roadmap.includes(`/spinon/${implementationEvidenceRoute}`), '공개 roadmap의 implementation evidence href 누락');
assert.ok(byRoute.has(implementationEvidenceRoute), 'implementation evidence의 selected publication route 누락');
const rowSpanEvidenceRoute = 'internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-2026-11-10/README.html';
assert.ok(roadmap.includes(`/spinon/${rowSpanEvidenceRoute}`), 'roadmap #1326 implementation evidence 링크 누락');
assert.ok(byRoute.has(rowSpanEvidenceRoute), 'row-span implementation evidence route 누락');
const precomparisonRoute = 'internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/README.html';
assert.ok(byRoute.has(precomparisonRoute), 'one-fit 사전 비교의 selected publication route 누락');

const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(status.includes('78/214'), 'STATUS 집계 불일치');
assert.ok(status.includes('PR [#1322]') && status.includes('727229aea05e33537962fe9953ee206a37befecb'), 'STATUS one-fit merge 상태 누락');
assert.match(status, /- \[ \] <span id="c11-css-grid"/u, 'C11 checkbox가 미완료가 아님');
assert.match(status, /- \[ \] \*\*C11\.4 intrinsic track sizing\*\*/u, 'C11.4 checkbox가 미완료가 아님');
const contract = fs.readFileSync(path.join(root, 'internal/0059-c11-grid-layout.md'), 'utf8');
assert.ok(contract.includes('PR [#1322]') && contract.includes('727229aea05e33537962fe9953ee206a37befecb'), '내부 계약 병합 metadata 불일치');
assert.ok(status.includes('PR [#1326]') && status.includes('b5925aa274718a16c4e8f0bd9ef4bed4423aec6c'), 'STATUS #1326 merge metadata 불일치');
assert.ok(contract.includes('PR [#1326]') && contract.includes('b5925aa274718a16c4e8f0bd9ef4bed4423aec6c'), '내부 계약 #1326 merge metadata 불일치');

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
  checkedPlanLinks,
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
