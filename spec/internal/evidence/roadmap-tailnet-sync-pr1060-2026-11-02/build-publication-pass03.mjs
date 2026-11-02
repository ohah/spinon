import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const build = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1060-2026-11-02/pass03');
const site = path.join(build, 'site');
const publication = path.join(build, 'publication');
const manifestPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const originHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(sourceHead, originHead, 'publication source must be latest origin/main');
assert.equal(execFileSync('git', ['status', '--porcelain=v1', '--untracked-files=no'], { cwd: repo, encoding: 'utf8' }).trim(), '', 'tracked source changes are not allowed');
assert.ok(fs.lstatSync(site).isDirectory() && !fs.lstatSync(site).isSymbolicLink(), 'selected Rspress site must be a real directory');
assert.ok(!fs.existsSync(publication), `existing publication preserved: ${publication}`);
assert.ok(!fs.existsSync(manifestPath), `existing manifest preserved: ${manifestPath}`);
fs.mkdirSync(publication, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid route: ${route}`);
}
function assertNoSymlink(relative) {
  let current = repo;
  for (const part of relative.split('/')) {
    current = path.join(current, part);
    assert.ok(!fs.lstatSync(current).isSymbolicLink(), `source symlink rejected: ${relative}`);
  }
}
function add(route, source, kind) {
  safeRoute(route);
  assertNoSymlink(path.relative(repo, source).split(path.sep).join('/'));
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route content collision: ${route}`);
    return;
  }
  const target = path.join(publication, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `publication hash mismatch: ${route}`);
  selected.set(route, { path: route, source: path.relative(repo, source).split(path.sep).join('/'), kind, bytes: bytes.length, sha256: digest });
}
function walk(directory, relative = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `generated site symlink rejected: ${child}`);
    if (entry.isDirectory()) result.push(...walk(directory, child));
    else if (entry.isFile()) result.push(child.split(path.sep).join('/'));
    else throw new Error(`unsupported generated entry: ${child}`);
  }
  return result;
}
function trackedFiles(prefix) {
  return execFileSync('git', ['ls-files', '-z', '--', prefix], { cwd: repo }).toString('utf8').split('\0').filter(Boolean);
}

for (const route of walk(site)) {
  if (route === 'index.html' || route === '404.html') continue;
  add(route, path.join(site, route), 'selected-rspress-output');
}
for (const [source, route] of [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/public/roadmap.html', 'spec/public/roadmap.html'],
  ['spec/public/logo.svg', 'logo.svg'],
  ['spec/public/roadmap.svg', 'roadmap.svg'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['plan/c11-root-html-grid-column-flow.md', 'plan/c11-root-html-grid-column-flow.md'],
  ['plan/c11-root-html-grid-column-flow-implicit-columns.md', 'plan/c11-root-html-grid-column-flow-implicit-columns.md'],
  ['plan/c11-root-html-grid-column-flow-plan-review-20.md', 'plan/c11-root-html-grid-column-flow-plan-review-20.md'],
  ['plan/c11-root-html-grid-column-flow-plan-pr-diff-review-30.md', 'plan/c11-root-html-grid-column-flow-plan-pr-diff-review-30.md'],
  ['plan/c11-root-html-grid-column-flow-implicit-columns-review-20.md', 'plan/c11-root-html-grid-column-flow-implicit-columns-review-20.md'],
  ['plan/c11-root-html-grid-column-flow-implicit-columns-plan-pr-diff-review-30.md', 'plan/c11-root-html-grid-column-flow-implicit-columns-plan-pr-diff-review-30.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
]) add(route, path.join(repo, source), 'tracked-source');

const prefixes = [
  'spec/internal/evidence/c11-root-html-grid-column-flow-precomparison-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-runtime-2026-11-02',
  'spec/internal/evidence/roadmap-c11-column-flow-plan-sync-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-precomparison-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-runtime-2026-11-02',
];
for (const prefix of prefixes) {
  const files = trackedFiles(prefix);
  assert.ok(files.length > 0, `tracked evidence is empty: ${prefix}`);
  for (const source of files) add(source.slice('spec/'.length), path.join(repo, source), 'tracked-evidence');
}
for (const source of trackedFiles('tests/fixtures/css/c11')) {
  if (source.includes('root-html-grid-column-flow')) add(source, path.join(repo, source), 'tracked-fixture');
}

const siteStatus = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
const statusSource = fs.readFileSync(path.join(repo, 'spec/STATUS.md'), 'utf8');
assert.ok(siteStatus.includes('72/213'), 'generated STATUS.html official count missing');
assert.match(statusSource, /^- \[ \] \*\*C11 CSS Grid\*\*/m, 'C11 checklist must remain unchecked');
assert.ok(siteStatus.includes('16c62916b4a72de6a44a9cbd3bec021d0454541e'), 'generated STATUS.html lacks PR #1059 merge record');

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('PR #1056 Rebase and merge') && roadmap.includes('PR #1059 Rebase and merge'), 'roadmap missing merged C11.1 cards');
assert.ok(roadmap.includes('이후 제한 구현 PR #1056') && roadmap.includes('PR #1059에서 fixed-px implicit columns를 추가했다'), 'roadmap #1054 history note is stale');
const checkedRoadmapLinks = [];
for (const marker of ['PR #1054 Rebase and merge', 'PR #1056 Rebase and merge', 'PR #1059 Rebase and merge']) {
  const index = roadmap.indexOf(marker);
  const start = roadmap.lastIndexOf('<p ', index);
  const end = roadmap.indexOf('</p>', index);
  assert.ok(index >= 0 && start >= 0 && end >= 0, `roadmap card bounds missing: ${marker}`);
  const card = roadmap.slice(start, end + 4);
  for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
    if (/^(?:https?:|mailto:)/i.test(href)) continue;
    assert.ok(href.startsWith('/spinon/'), `unexpected local href: ${href}`);
    const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
    assert.ok(selected.has(route), `roadmap card route missing from publication: ${route}`);
    checkedRoadmapLinks.push({ marker, href, route, sha256: selected.get(route).sha256 });
  }
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead,
  sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(),
  trackedDiffSha256: sha256(Buffer.alloc(0)),
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '72/213',
  c11Complete: false,
  checkedRoadmapLinks,
  fullDocsLinkValidation: false,
  untrackedSourcePublished: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined, checkedRoadmapLinks: undefined }, null, 2)}\n`);
