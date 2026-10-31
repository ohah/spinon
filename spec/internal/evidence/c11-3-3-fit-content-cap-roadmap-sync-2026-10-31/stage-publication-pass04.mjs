import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-fit-content-cap-20261031');
const sourceBase = path.join(buildBase, 'pass05');
const base = path.join(buildBase, 'pass06');
const root = path.join(sourceBase, 'root');
const site = path.join(sourceBase, 'site');
const publication = path.join(base, 'publication');
const staging = path.join(base, 'publication-staging');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass03.json'), 'utf8'));
const reportPath = path.join(evidence, 'selected-publication-pass04.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [base, publication, staging, reportPath]) assert.ok(!fs.existsSync(candidate), `기존 pass03 산출물을 덮어쓰지 않음: ${candidate}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(path.dirname(publication), { recursive: false });
fs.mkdirSync(staging, { recursive: false });

const selected = new Map();
const rootFiles = new Map(rootManifest.staged.map((entry) => [entry.route, entry]));
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}
function addBytes(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route source 충돌: ${route}`);
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
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `generated route가 일반 파일이 아님: ${route}`);
  addBytes(route, fs.readFileSync(source), `rspress:${route}`, 'generated-page-or-asset');
}
function addRootFile(route) {
  const entry = rootFiles.get(route);
  if (!entry) return false;
  const source = path.join(root, route);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.byteLength, entry.bytes, `selected root 크기 변경: ${route}`);
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
  if (route === 'index.html' || route === '404.html') continue;
  if (route.endsWith('.html') || route.startsWith('static/') || route === 'icon.svg' || route === 'logo.svg' || route === 'roadmap.svg') addSiteFile(route);
}

assert.ok(addRootFile('roadmap.html'), 'raw roadmap route is required');
for (const route of ['STATUS.md', 'spec/STATUS.md', 'public/icon.svg', 'spec/public/icon.svg', 'logo.svg', 'public/logo.svg', 'spec/public/logo.svg']) addRootFile(route);

const importantPages = [
  'plan/c11-3-3-template-area-fit-content-cap.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/README.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/plan-review-20.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/plan-pr-diff-review-30.html',
];
for (const route of importantPages) assert.ok(selected.has(route), `required rendered page 누락: ${route}`);

function localRouteFromUrl(raw, pageRoute) {
  try {
    const url = new URL(raw, `https://preview.invalid/spinon/${pageRoute}`);
    if (url.origin !== 'https://preview.invalid') return null;
    const pathname = decodeURIComponent(url.pathname);
    if (pathname === '/spinon' || pathname === '/spinon/') return null;
    if (!pathname.startsWith('/spinon/')) return null;
    return pathname.slice('/spinon/'.length);
  } catch {
    return null;
  }
}
const linkedSourceRoutes = new Set();
function collectLinkedSources(html, pageRoute) {
  for (const match of html.matchAll(/\b(?:href|src)=["']([^"']+)["']/g)) {
    const route = localRouteFromUrl(match[1], pageRoute);
    if (route && rootFiles.has(route)) linkedSourceRoutes.add(route);
  }
}
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const cardStart = roadmap.indexOf('C11.3.3 area-backed typed <code>grid-auto-*</code>');
assert.ok(cardStart >= 0, 'C11.3.3 roadmap card exists');
const cardStartTag = roadmap.lastIndexOf('<p ', cardStart);
const cardEnd = roadmap.indexOf('</p>', cardStart);
assert.ok(cardStartTag >= 0 && cardEnd > cardStart, 'target card paragraph boundary exists');
const cardHtml = roadmap.slice(cardStartTag, cardEnd + 4);
for (const marker of [
  '/spinon/plan/c11-3-3-template-area-fit-content-cap.html',
  '/spinon/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html',
  '72/213',
  'fit-content cap 후속 계획',
]) assert.ok(cardHtml.includes(marker), `C11.3.3 card marker 누락: ${marker}`);
collectLinkedSources(cardHtml, 'roadmap.html');
for (const route of importantPages) collectLinkedSources(fs.readFileSync(path.join(site, route), 'utf8'), route);
for (const route of linkedSourceRoutes) addRootFile(route);

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-3-3-template-area-fit-content-cap.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/README.html',
  'internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/chrome-156-agent-browser.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

fs.renameSync(staging, publication);
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  generatedHtmlCount: routes.filter((item) => item.path.endsWith('.html')).length,
  linkedSourceRouteCount: linkedSourceRoutes.size,
  requiredRoutes,
  roadmapMarkers: ['/spinon/plan/c11-3-3-template-area-fit-content-cap.html', '/spinon/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html', '72/213'],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined, linkedSourceRoutes: [...linkedSourceRoutes].sort() }, null, 2)}\n`);
