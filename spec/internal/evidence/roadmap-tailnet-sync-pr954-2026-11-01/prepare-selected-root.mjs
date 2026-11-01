import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-4-text-transform-pr954-tailnet-postmerge-20261101');
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const staged = new Map();

for (const candidate of [buildBase, root, manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물을 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(root, { recursive: true });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function add(sourceRelative, route) {
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source가 regular file이 아님: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = staged.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `선택 root route 충돌: ${route}`);
    return;
  }
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `선택 root hash 불일치: ${route}`);
  staged.set(route, { source: sourceRelative, route, bytes: bytes.length, sha256: digest });
}

for (const name of ['roadmap.html', 'roadmap.svg', 'icon.svg', 'logo.svg']) {
  add(`spec/public/${name}`, `public/${name}`);
  add(`spec/public/${name}`, `spec/public/${name}`);
}
for (const [source, routes] of [
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']],
  ['plan/c11-css-grid.md', ['plan/c11-css-grid.md']],
  ['plan/c11-4-grid-track-sizing.md', ['plan/c11-4-grid-track-sizing.md']],
  ['plan/c11-4-anonymous-grid-text-computed-text-transform.md', ['plan/c11-4-anonymous-grid-text-computed-text-transform.md']],
  ['plan/c11-4-anonymous-grid-text-computed-text-transform-plan-review-20.md', ['plan/c11-4-anonymous-grid-text-computed-text-transform-plan-review-20.md']],
]) {
  for (const route of routes) add(source, route);
}

const evidenceDirs = [
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01',
];
for (const directory of evidenceDirs) {
  const tracked = execFileSync('git', ['ls-files', '-z', '--', directory], { cwd: repo });
  const entries = tracked.toString('utf8').split('\0').filter(Boolean);
  assert.ok(entries.length > 0, `tracked evidence가 없음: ${directory}`);
  for (const source of entries) {
    assert.ok(source.startsWith(`${directory}/`), `예상하지 못한 tracked 경로: ${source}`);
    const relative = source.slice('spec/'.length);
    add(source, relative);
    add(source, `spec/${relative}`);
  }
}

add('tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html', 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform.html');

const roadmap = fs.readFileSync(path.join(root, 'public/roadmap.html'), 'utf8');
for (const marker of ['PR #952 Rebase and merge', 'be62a4dd3c1b67ada2db094f19a76231e9f3df35', '72/213']) {
  assert.ok(roadmap.includes(marker), `현재 공개 roadmap marker 누락: ${marker}`);
}

const sourceHead = git(['rev-parse', 'origin/main']);
const sourceTree = git(['rev-parse', 'origin/main^{tree}']);
assert.equal(git(['rev-parse', 'HEAD^{tree}']), sourceTree, 'checkout tree가 origin/main과 다름');
const manifest = {
  sourceHead,
  sourceTree,
  trackedDiffSha256: sha256(execFileSync('git', ['diff', '--binary'], { cwd: repo })),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  fullDocsDeadLinkCheck: false,
  noSymlinks: true,
  roadmapMarkers: ['PR #952 Rebase and merge', 'be62a4dd3c1b67ada2db094f19a76231e9f3df35', '72/213'],
  stagedFileCount: staged.size,
  stagedBytes: [...staged.values()].reduce((sum, item) => sum + item.bytes, 0),
  staged: [...staged.values()].sort((a, b) => a.route.localeCompare(b.route)),
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
