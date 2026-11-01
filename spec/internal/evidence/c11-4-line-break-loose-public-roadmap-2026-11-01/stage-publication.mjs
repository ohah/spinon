import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repo, 'build/spinon/c11-4-loose-and-text-indent-flags-roadmap-pr964-pass01');
const root = path.join(buildRoot, 'root');
const site = path.join(buildRoot, 'site');
const publicationBase = path.join(buildRoot, 'publication-pass01');
const staging = path.join(publicationBase, 'staging');
const publication = path.join(publicationBase, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr956-plan-rollup-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication-tailnet-pass02.json'), 'utf8'));
const previousRoot = path.join(repo, previousManifest.publicationRoot);
const reportPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const selected = new Map();
const changes = [];

for (const target of [publicationBase, staging, publication, reportPath]) assert.ok(!fs.existsSync(target), `기존 publication artifact를 덮어쓰지 않음: ${target}`);
for (const directory of [root, site, previousRoot]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 publication source: ${directory}`);
}
fs.mkdirSync(staging, { recursive: true });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function put(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const prior = selected.get(route);
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  if (prior) {
    if (prior.sha256 === digest) return;
    fs.writeFileSync(target, bytes);
    changes.push({ path: route, previousSha256: prior.sha256, sha256: digest, previousSource: prior.source, source });
  } else {
    fs.writeFileSync(target, bytes, { flag: 'wx' });
  }
  selected.set(route, { path: route, bytes: bytes.length, sha256: digest, source, kind });
}

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `publication symlink 거부: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(directory, relative));
    else if (entry.isFile()) files.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication source: ${relative}`);
  }
  return files;
}

const oldFiles = listFiles(previousRoot);
const oldByRoute = new Map(previousManifest.routes.map((route) => [route.path, route]));
assert.equal(oldFiles.length, oldByRoute.size, '이전 publication 파일 수가 manifest와 다름');
for (const route of oldFiles) {
  const bytes = fs.readFileSync(path.join(previousRoot, route));
  const expected = oldByRoute.get(route);
  assert.ok(expected, `이전 manifest에서 route 누락: ${route}`);
  assert.equal(bytes.length, expected.bytes, `이전 route 크기 불일치: ${route}`);
  assert.equal(sha256(bytes), expected.sha256, `이전 route hash 불일치: ${route}`);
  put(route, bytes, `previous-publication:${route}`, 'preserved-route');
}

const generated = listFiles(site);
for (const route of generated) {
  if (route === 'index.html' || route === '404.html') continue;
  put(route, fs.readFileSync(path.join(site, route)), `rspress:${route}`, 'generated-page-or-asset');
}
for (const item of rootManifest.staged) {
  const bytes = fs.readFileSync(path.join(root, item.route));
  assert.equal(bytes.length, item.bytes, `selected root size mismatch: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash mismatch: ${item.route}`);
  put(item.route, bytes, `selected-root:${item.source}`, 'selected-source');
}

for (const name of ['roadmap.html', 'roadmap.svg', 'icon.svg', 'logo.svg']) {
  const bytes = fs.readFileSync(path.join(root, 'public', name));
  put(name, bytes, `selected-root:public/${name}`, 'public-alias');
}
const fixture = 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html';
put(fixture, fs.readFileSync(path.join(repo, fixture)), `repository:${fixture}`, 'fixture-source');

const roadmap = fs.readFileSync(path.join(staging, 'roadmap.html'), 'utf8');
for (const marker of [
  'PR #962 Rebase and merge', '5831014125e26079748f0b99cb99d3c21613358c',
  'PR #959 Rebase and merge', '590f5946e97e0252113abf34b23ae31b13819acf', '72/213',
]) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);

const checkedCards = [];
for (const pr of ['962', '959']) {
  const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">[\s\S]*?<\/p>/g)]
    .map((match) => match[0])
    .find((value) => value.includes(`PR #${pr} Rebase and merge`));
  assert.ok(card, `PR #${pr} public roadmap card가 있어야 한다`);
  const links = [];
  for (const match of card.matchAll(/href="([^"]+)"/g)) {
    const href = match[1];
    if (/^(?:https?:|mailto:)/i.test(href)) continue;
    const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
    assert.equal(url.origin, 'https://preview.invalid', `외부 origin을 local route로 처리하지 않음: ${href}`);
    assert.ok(url.pathname.startsWith('/spinon/'), `site base 이탈: ${href}`);
    const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
    assert.ok(selected.has(route), `PR #${pr} card route가 publication에 없음: ${href} -> ${route}`);
    links.push({ href, route, sha256: selected.get(route).sha256 });
  }
  checkedCards.push({ pr: Number(pr), links });
}

const requiredRoutes = [
  'roadmap.html', 'STATUS.html', 'plan/c11-css-grid.html', 'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-anonymous-grid-text-indent-flags.html', 'plan/c11-4-anonymous-grid-text-line-break-loose.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/post-merge-status-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/README.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/post-merge-status-roadmap-review-30.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `필수 publication route 누락: ${route}`);

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(rootManifest.sourceHead, sourceHead, 'selected root HEAD가 checkout HEAD와 다름');
assert.equal(rootManifest.sourceTree, sourceTree, 'selected root tree가 checkout tree와 다름');
assert.equal(sourceHead, execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim(), '원격 main에서 벗어난 상태로 게시하지 않음');
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead,
  sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  workingTreeRoadmapSha256: rootManifest.workingTreeRoadmapSha256,
  previousSourceHead: previousManifest.sourceHead,
  previousSourceTree: previousManifest.sourceTree,
  selectedRoot: rootManifest.selectedRoot,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  previousRouteCount: oldFiles.length,
  addedRoutes: routes.filter((route) => !oldByRoute.has(route.path)).length,
  replacedRoutes: changes.length,
  preservedRoutes: routes.filter((route) => oldByRoute.has(route.path) && route.sha256 === oldByRoute.get(route.path).sha256).length,
  noDeletes: true,
  fullDocsDeadLinkCheck: false,
  selectedRspressBuild: '2.0.22 success; checkDeadLinks and checkDeadImages disabled for selected-root build',
  serverRestarted: false,
  requiredRoutes,
  checkedCards,
  changes,
  routes,
};
fs.renameSync(staging, publication);
fs.writeFileSync(reportPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined, changes: undefined, checkedCards: undefined }, null, 2)}\n`);
