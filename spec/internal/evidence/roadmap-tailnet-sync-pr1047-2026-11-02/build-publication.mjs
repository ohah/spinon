import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const site = path.join(repo, 'build/spinon/c11-roadmap-pr1047-pass01/site');
const publication = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1047-2026-11-02/publication');
const manifestPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const target of [publication, manifestPath]) assert.ok(!fs.existsSync(target), `기존 publication 보존: ${target}`);
for (const directory of [site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build directory: ${directory}`);
}
fs.mkdirSync(publication, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function assertNoSymlink(source) {
  let current = repo;
  const parts = path.relative(repo, source).split(path.sep);
  for (const part of parts) {
    current = path.join(current, part);
    assert.ok(!fs.lstatSync(current).isSymbolicLink(), `source symlink 거부: ${path.relative(repo, source)}`);
  }
}
function add(route, source, kind) {
  safeRoute(route);
  assertNoSymlink(source);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route 내용 충돌: ${route}`);
    return;
  }
  const target = path.join(publication, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `publication hash 불일치: ${route}`);
  selected.set(route, { path: route, source: path.relative(repo, source).split(path.sep).join('/'), kind, bytes: bytes.length, sha256: digest });
}
function walk(directory, relative = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `generated site symlink 거부: ${child}`);
    if (entry.isDirectory()) result.push(...walk(directory, child));
    else if (entry.isFile()) result.push(child.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 publication 항목: ${child}`);
  }
  return result;
}
function trackedFiles(prefix) {
  return execFileSync('git', ['ls-files', '-z', '--', prefix], { cwd: repo }).toString('utf8').split('\0').filter(Boolean);
}

for (const route of walk(site)) {
  if (route === 'index.html' || route === '404.html') continue;
  add(route, path.join(site, route), 'rspress-generated');
}
for (const [source, route] of [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/public/roadmap.html', 'spec/public/roadmap.html'],
  ['spec/public/logo.svg', 'logo.svg'],
  ['spec/public/roadmap.svg', 'roadmap.svg'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['plan/c11-root-html-grid-auto-row-list.md', 'plan/c11-root-html-grid-auto-row-list.md'],
  ['plan/c11-root-html-grid-auto-row-percentage-list.md', 'plan/c11-root-html-grid-auto-row-percentage-list.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['plan/c11-root-html-grid-runtime.md', 'plan/c11-root-html-grid-runtime.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['tests/fixtures/css/c11/root-html-grid-auto-row-list-precomparison.html', 'tests/fixtures/css/c11/root-html-grid-auto-row-list-precomparison.html'],
  ['tests/fixtures/css/c11/root-html-grid-auto-row-percentage-list-precomparison.html', 'tests/fixtures/css/c11/root-html-grid-auto-row-percentage-list-precomparison.html'],
]) add(route, path.join(repo, source), 'tracked-source');

for (const prefix of [
  'spec/internal/evidence/c11-root-html-grid-auto-row-list-runtime-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-auto-row-list-precomparison-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02',
  'spec/internal/evidence/roadmap-tailnet-sync-pr1042-2026-11-02',
]) {
  for (const source of trackedFiles(prefix)) {
    const route = source.startsWith('spec/') ? source.slice('spec/'.length) : source;
    add(route, path.join(repo, source), 'tracked-source');
  }
}

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
const start = roadmap.indexOf('PR #1047 Rebase and merge');
assert.ok(start >= 0, 'roadmap #1042 card 누락');
const cardStart = roadmap.lastIndexOf('<p ', start);
const cardEnd = roadmap.indexOf('</p>', start);
assert.ok(cardStart >= 0 && cardEnd >= 0, '#1042 roadmap paragraph 경계 누락');
const card = roadmap.slice(cardStart, cardEnd + 4);
const checkedLinks = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(selected.has(route), `#1042 roadmap linked route 누락: ${route}`);
  checkedLinks.push({ href, route, sha256: selected.get(route).sha256 });
}
assert.equal(checkedLinks.length, 7, `#1047 roadmap local link 수 불일치: ${checkedLinks.length}`);
const status = fs.readFileSync(path.join(publication, 'STATUS.html'), 'utf8');
assert.ok(status.includes('72/213') && status.includes('C11 CSS Grid') && status.includes('id="css-구현-체크리스트"'));

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(execFileSync('git', ['status', '--porcelain=v1', '--untracked-files=no'], { cwd: repo, encoding: 'utf8' }).trim(), '', 'tracked source 변경이 없어야 함');
const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead,
  sourceTree,
  trackedDiffSha256: sha256(Buffer.alloc(0)),
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213',
  c11Complete: false,
  checkedRoadmapLinks: checkedLinks,
  fullDocsLinkValidation: false,
  untrackedEvidencePublished: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined }, null, 2)}\n`);
