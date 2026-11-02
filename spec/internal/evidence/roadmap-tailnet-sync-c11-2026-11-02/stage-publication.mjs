import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const pass = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-2026-11-02/pass05');
const root = path.join(pass, 'root');
const site = path.join(pass, 'site');
const staging = path.join(pass, 'publication-staging');
const publication = path.join(pass, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass05.json'), 'utf8'));
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
function walk(directory, relative, onFile) {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${child}`);
    if (entry.isDirectory()) walk(directory, child, onFile);
    else if (entry.isFile()) onFile(child, absolute);
    else throw new Error(`지원하지 않는 항목: ${child}`);
  }
}

walk(site, '', (route, absolute) => {
  if (route === 'index.html' || route === '404.html') return;
  add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
});

function addRootFile(route, aliases = []) {
  const bytes = fs.readFileSync(path.join(root, route));
  add(route, bytes, `selected-root:${route}`, 'source-page-or-asset');
  for (const alias of aliases) add(alias, bytes, `selected-root:${route}`, 'source-page-alias');
}
addRootFile('roadmap.html', ['public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  const route = `public/${name}`;
  const aliases = [name, `spec/public/${name}`];
  addRootFile(route, aliases);
}
addRootFile('STATUS.md', ['spec/STATUS.md']);
addRootFile('internal/0059-c11-grid-layout.md', ['spec/internal/0059-c11-grid-layout.md']);
for (const name of [
  'c11-css-grid.md',
  'c11-root-html-grid-runtime.md',
  'c11-3-template-repeat-named-lines-areas.md',
  'c11-root-html-grid-integer-repeat.md',
  'c11-root-html-grid-integer-repeat-plan-review-20.md',
]) addRootFile(`plan/${name}`, [`spec/plan/${name}`]);
for (const name of [
  'c11-root-html-grid-integer-repeat-precomparison-2026-11-02',
  'c11-root-html-grid-integer-repeat-runtime-2026-11-02',
]) {
  walk(root, `internal/evidence/${name}`, (route, absolute) => {
    const bytes = fs.readFileSync(absolute);
    add(route, bytes, `selected-root:${route}`, 'source-page-or-asset');
    add(`spec/${route}`, bytes, `selected-root:${route}`, 'source-page-alias');
  });
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
const marker = roadmap.indexOf('#1038');
assert.ok(marker >= 0, 'PR #1038 roadmap entry 누락');
const cardStart = roadmap.lastIndexOf('<p ', marker);
const cardEnd = roadmap.indexOf('</p>', marker);
assert.ok(cardStart >= 0 && cardEnd >= 0, 'PR #1038 roadmap paragraph 경계 누락');
const card = roadmap.slice(cardStart, cardEnd + 4);
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
  assert.ok(byRoute.has(route), `PR #1038 roadmap route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: byRoute.get(route).sha256 });
}
assert.ok(checkedLinks.length >= 8, `PR #1038 roadmap local link 수가 예상보다 적음: ${checkedLinks.length}`);
for (const required of [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-css-grid.html',
  'plan/c11-root-html-grid-integer-repeat.html',
  'internal/evidence/c11-root-html-grid-integer-repeat-precomparison-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/pr-diff-review-30.md',
  'internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/android-avd/simulator.png',
  'internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/ios-simulator/simulator.png',
]) assert.ok(byRoute.has(required), `필수 route 누락: ${required}`);
for (const markerText of ['b3ad473d4bfef4bb2f54a61603d498e620c03b0a', '72/213']) assert.ok(card.includes(markerText), `roadmap marker 누락: ${markerText}`);
const status = fs.readFileSync(path.join(staging, 'STATUS.html'), 'utf8');
assert.ok(status.includes('72/213') && status.includes('C11.3') && status.includes('C11.4'));

fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213',
  c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
