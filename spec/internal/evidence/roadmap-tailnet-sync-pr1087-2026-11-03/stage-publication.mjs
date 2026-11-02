import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1087-2026-11-03/pass01');
const selectedRoot = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const publication = path.join(buildBase, 'publication');
const reportPath = path.join(evidence, 'selected-routes.json');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview/spinon';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const selected = new Map();

for (const target of [publication, reportPath]) {
  assert.ok(!fs.existsSync(target), '기존 publication 산출물을 덮어쓰지 않음: ' + target);
}
for (const target of [selectedRoot, site]) {
  const stat = fs.lstatSync(target);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), '안전하지 않은 build 경로: ' + target);
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    assert.ok(!entry.isSymbolicLink(), 'symlink 거부: ' + relative);
    if (entry.isDirectory()) paths.push(...listFiles(directory, relative));
    else if (entry.isFile()) paths.push(relative.split(path.sep).join('/'));
    else throw new Error('지원하지 않는 파일 형식: ' + relative);
  }
  return paths;
}

function put(route, bytes, source) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..') && !route.includes('\\'), '잘못된 route: ' + route);
  const hash = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, hash, '중복 route의 내용이 다름: ' + route);
    return;
  }
  const target = path.join(publication, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  selected.set(route, { route, bytes: bytes.length, sha256: hash, source });
}

function addRootFile(route) {
  const file = path.join(selectedRoot, route);
  const bytes = fs.readFileSync(file);
  const expected = rootManifest.staged.find((item) => item.route === route);
  assert.ok(expected, 'selected root manifest에 route 누락: ' + route);
  assert.equal(bytes.length, expected.bytes, 'selected root size 불일치: ' + route);
  assert.equal(sha256(bytes), expected.sha256, 'selected root hash 불일치: ' + route);
  put(route, bytes, 'selected-root:' + route);
}

function addSiteFile(route) {
  const file = path.join(site, route);
  assert.ok(fs.existsSync(file), 'Rspress output route 누락: ' + route);
  put(route, fs.readFileSync(file), 'rspress:' + route);
}

for (const overlay of rootManifest.overlays) addRootFile(overlay.route);
for (const overlay of rootManifest.overlays) {
  if (overlay.route.endsWith('.md')) addSiteFile(overlay.route.replace(/\.md$/i, '.html'));
}
for (const route of listFiles(site)) addSiteFile(route);

for (const name of ['roadmap.html', 'roadmap.svg', 'icon.svg', 'logo.svg']) {
  const source = path.join(selectedRoot, 'public', name);
  if (fs.existsSync(source)) put(name, fs.readFileSync(source), 'selected-root:public/' + name);
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'STATUS.md',
  'public/roadmap.html',
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-root-html-grid-minmax.html',
  'plan/c11-root-html-grid-minmax-plan-review-20.html',
  'plan/c11-root-html-grid-minmax-implementation-review-20.html',
  'plan/c11-root-html-grid-minmax-implementation-pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/README.html',
  'internal/evidence/c11-root-html-grid-minmax-runtime-2026-11-03/README.html',
  'spec/internal/evidence/c11-root-html-grid-minmax-runtime-2026-11-03/README.html',
  'plan/roadmap-status-counter-identity.html',
  'plan/roadmap-status-counter-identity-plan-review-20.html',
  'internal/evidence/roadmap-status-counter-identity-2026-11-03/README.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), '필수 publication route 누락: ' + route);

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
for (const marker of ['PR #1086 Rebase and merge', '2bb59a35c89358eee77918f8b7e9f7b9c9fae7c1']) {
  assert.ok(roadmap.includes(marker), '로드맵 marker 누락: ' + marker);
}
assert.ok(selected.has('spec/STATUS.md'), '공식 상태 원본 route 누락');
const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">([\s\S]*?)<\/p>/g)]
  .map((match) => match[1])
  .find((html) => html.includes('PR #1086 Rebase and merge'));
assert.ok(card, 'PR #1086 roadmap card not found');
const checkedLocalLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.equal(url.origin, 'https://preview.invalid', 'local link가 다른 origin으로 해석됨: ' + href);
  assert.ok(url.pathname.startsWith('/spinon/'), '기준 경로가 다른 local link: ' + href);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length)) || 'index.html';
  assert.ok(selected.has(route) || fs.existsSync(path.join(previewRoot, route)), 'PR #1086 card route 누락: ' + href + ' -> ' + route);
  checkedLocalLinks.push({ href, route, staged: selected.has(route) });
}
assert.ok(checkedLocalLinks.some((item) => item.route === 'plan/c11-root-html-grid-minmax.html'));
assert.ok(checkedLocalLinks.some((item) => item.route === 'spec/internal/evidence/c11-root-html-grid-minmax-runtime-2026-11-03/README.html'));

const routes = [...selected.values()].sort((a, b) => a.route.localeCompare(b.route));
const manifest = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  previousSourceHead: rootManifest.previousSourceHead,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  patchRoot: path.relative(repo, publication).split(path.sep).join('/'),
  priorTailnetSnapshot: 'PR #1084 / C11.4 full-size-kana, verified before this sync with Agent Browser',
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  requiredRoutes,
  checkedLocalLinks,
  noDeletes: true,
  previewServerRestarted: false,
  routes,
};
fs.writeFileSync(reportPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...manifest, routes: undefined }, null, 2) + '\n');
