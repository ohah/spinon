import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c114-line-break-postmerge-20261101');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const publicationBase = path.join(buildBase, 'publication-pass01');
const staging = path.join(publicationBase, 'staging');
const publication = path.join(publicationBase, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const reportPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [publicationBase, staging, publication, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication 산출물을 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 selected build 디렉터리: ${directory}`);
}
fs.mkdirSync(staging, { recursive: true });
const selected = new Map();
const rootFiles = new Map(rootManifest.staged.map((item) => [item.route, item]));

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}

function addBytes(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const prior = selected.get(route);
  if (prior) {
    assert.equal(prior.sha256, digest, `route source 충돌: ${route}`);
    return;
  }
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `staging hash 불일치: ${route}`);
  selected.set(route, { path: route, source, kind, bytes: bytes.length, sha256: digest });
}

function addSiteFile(route) {
  const source = path.join(site, route);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `generated route가 regular file이 아님: ${route}`);
  addBytes(route, fs.readFileSync(source), `rspress:${route}`, 'generated-page-or-asset');
}

function addRootFile(route) {
  const entry = rootFiles.get(route);
  if (!entry) return false;
  const bytes = fs.readFileSync(path.join(root, route));
  assert.equal(bytes.length, entry.bytes, `selected root 크기 변경: ${route}`);
  assert.equal(sha256(bytes), entry.sha256, `selected root hash 변경: ${route}`);
  addBytes(route, bytes, `selected-root:${entry.source}`, 'source-page-or-asset');
  return true;
}

const generatedFiles = [];
function walkSite(relative = '') {
  for (const entry of fs.readdirSync(path.join(site, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(site, child);
    assert.ok(!entry.isSymbolicLink(), `generated site symlink 거부: ${child}`);
    if (entry.isDirectory()) walkSite(child);
    else if (entry.isFile()) generatedFiles.push(child.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 generated site 항목: ${child}`);
  }
}
walkSite();
for (const route of generatedFiles) {
  if (route === '404.html' || route === 'index.html') continue;
  if (route.endsWith('.html') || route.startsWith('static/') || route === 'icon.svg' || route === 'logo.svg') addSiteFile(route);
}

for (const route of ['roadmap.html', 'STATUS.md', 'spec/STATUS.md', 'icon.svg', 'public/icon.svg', 'spec/public/icon.svg', 'logo.svg', 'public/logo.svg', 'spec/public/logo.svg']) {
  addRootFile(route);
}
for (const [route, item] of rootFiles) {
  if (/\.(?:png|jpe?g|webp|svg|json|tsv|log|txt|md|html)$/i.test(route) &&
      (/^internal\/evidence\/c11-4-anonymous-grid-text-line-break-(?:precomparison|runtime)-2026-11-01\//.test(route) ||
       /^spec\/internal\/evidence\/c11-4-anonymous-grid-text-line-break-(?:precomparison|runtime)-2026-11-01\//.test(route))) {
    addRootFile(route);
  }
}

const roadmapBytes = fs.readFileSync(path.join(root, 'roadmap.html'));
const roadmap = roadmapBytes.toString('utf8');
const cardStart = roadmap.indexOf('PR #945 Rebase and merge');
assert.ok(cardStart >= 0, 'PR #945 public roadmap card가 있어야 함');
const cardStartTag = roadmap.lastIndexOf('<p ', cardStart);
const cardEnd = roadmap.indexOf('</p>', cardStart);
assert.ok(cardStartTag >= 0 && cardEnd > cardStart, '대상 card paragraph 경계를 확인해야 함');
const card = roadmap.slice(cardStartTag, cardEnd + 4);
for (const marker of ['9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d', '72/213', '/spinon/plan/c11-4-anonymous-grid-text-computed-line-break.html']) {
  assert.ok(card.includes(marker), `PR #945 card marker 누락: ${marker}`);
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-css-grid.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-anonymous-grid-text-computed-line-break.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/android-avd/representative-screen.png',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/ios-simulator/representative-screen.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

const localLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.equal(url.origin, 'https://preview.invalid', `외부 origin 링크를 local로 해석하지 않음: ${href}`);
  assert.ok(url.pathname.startsWith('/spinon/'), `예상하지 못한 local base: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
  assert.ok(selected.has(route), `PR #945 card의 local link route 누락: ${href} -> ${route}`);
  localLinks.push({ href, route, sha256: selected.get(route).sha256 });
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  selectedRoot: rootManifest.selectedRoot,
  generatedHtmlCount: generatedFiles.filter((route) => route.endsWith('.html')).length,
  fullDocsLinkValidation: false,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  requiredRoutes,
  roadmapMarkers: ['PR #945 Rebase and merge', '9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d', '72/213'],
  checkedCardLocalLinks: localLinks,
  routes,
};

fs.renameSync(staging, publication);
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
