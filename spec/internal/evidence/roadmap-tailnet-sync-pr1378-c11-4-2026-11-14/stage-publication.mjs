import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1378-c11-4-2026-11-14/pass02');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass02.json'), 'utf8'));
const manifestPath = path.join(evidence, 'selected-publication-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [staging, publication, manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication 산출물을 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
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
  const prior = selected.get(route);
  if (prior) {
    assert.equal(prior.sha256, digest, `route byte 충돌: ${route}`);
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
      if (route !== 'index.html' && route !== '404.html') addBytes(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
    } else throw new Error(`지원하지 않는 generated site entry: ${child}`);
  }
}

walkSite();
for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md') continue;
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, `selected root byte 수 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 변경: ${item.route}`);
  addBytes(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}

const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
const start = roadmap.indexOf('최근 병합 bounded slice:');
assert.ok(start >= 0, 'roadmap 최신 bounded slice 누락');
const end = roadmap.indexOf('</span>', start);
assert.ok(end > start, 'roadmap 최신 bounded slice 경계 누락');
const card = roadmap.slice(start, end);
for (const marker of ['#1378 Rebase and merge', 'cbf3b8579b7aa4915998378acd2d0b03c7916077', '78/214', '다음 bounded 후보 미선정']) {
  assert.ok(card.includes(marker), `roadmap 최신 slice marker 누락: ${marker}`);
}

const routes = new Map([...selected.values()].map((entry) => [entry.path, entry]));
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href) || href.startsWith('#')) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 roadmap route: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 local route: ${href}`);
  assert.ok(routes.has(route), `roadmap local link가 publication에 없음: ${route}`);
  checkedLinks.push({ href, route, sha256: routes.get(route).sha256, verified: true });
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/README.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/merge-PR1378.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/ios-mask16.png',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/android-mask16.png',
];
for (const route of requiredRoutes) assert.ok(routes.has(route), `필수 route 누락: ${route}`);

const rawRoadmap = fs.readFileSync(path.join(staging, 'roadmap.html'));
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  selectedInputsSha256: rootManifest.selectedInputsSha256,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  selectedRootFileCount: rootManifest.stagedFileCount,
  routeCount: selected.size,
  totalBytes: [...selected.values()].reduce((sum, route) => sum + route.bytes, 0),
  rawRoadmapSha256: sha256(rawRoadmap),
  rawRoadmapBytes: rawRoadmap.length,
  officialCount: '78/214',
  c11Complete: false,
  roadmapLocalLinkCount: checkedLinks.length,
  checkedLinks,
  requiredRoutes,
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes: [...selected.values()].sort((a, b) => a.path.localeCompare(b.path)),
};
fs.renameSync(staging, publication);
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined, checkedLinks: undefined }, null, 2)}\n`);
