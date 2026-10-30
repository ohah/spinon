import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-4-prewrap-20261031/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publication = path.join(base, 'publication');
const publicationStaging = path.join(base, 'publication-staging');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const reportPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [publication, publicationStaging, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication을 덮어쓰지 않기 위해 중단: ${candidate}`);
}
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(publicationStaging, { recursive: false });

const selected = new Map();
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
  const target = path.join(publicationStaging, route);
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
      if (route !== 'index.html' && route !== '404.html') {
        addBytes(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
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
  addBytes(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}

const roadmap = fs.readFileSync(path.join(publicationStaging, 'roadmap.html'), 'utf8');
for (const marker of [
  'white-space:pre-wrap',
  '16/16',
  '1.77778 / 1',
  '1.7777778 / 1',
  '72/213',
  'PR diff 검토 30개',
]) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
assert.ok(roadmap.indexOf('white-space:pre-wrap') < roadmap.indexOf('white-space:pre-line'), 'PreWrap entry must appear before the merged PreLine entry');

const status = fs.readFileSync(path.join(publicationStaging, 'STATUS.html'), 'utf8');
for (const marker of ['72/213', 'anonymous-grid-text-pre-wrap', '1.77778 / 1', '1.7777778 / 1']) {
  assert.ok(status.includes(marker), `rendered STATUS marker 누락: ${marker}`);
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-anonymous-grid-text-pre-wrap.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/implementation-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/artifact-boundary/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/android-avd-c114-anonymous-grid-text-android-avd-20261030T182836149Z-c7a75e1a/anonymous-pre-wrap-soft-wrap.png',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T182908683Z-2a6cafcf/anonymous-pre-wrap-soft-wrap.png',
  'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/README.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

fs.renameSync(publicationStaging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  selectedRootFileCount: rootManifest.stagedFileCount,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  requiredRoutes,
  roadmapMarkers: ['white-space:pre-wrap', '16/16', '72/213'],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
