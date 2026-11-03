import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const workspace = path.join(repo, 'build/spinon/roadmap-c11-calc-auto-columns-tailnet-sync-pass04-2026-11-03');
const base = path.join(workspace, 'pass01');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass04.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const scriptExtensions = new Set(['.js', '.jsx', '.mjs', '.cjs', '.ts', '.tsx', '.sh']);
const staged = [];

for (const target of [workspace, base, root, manifestPath]) assert.ok(!fs.existsSync(target), `기존 산출물 보존: ${target}`);
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(workspace, { recursive: false });
fs.mkdirSync(base, { recursive: false });
fs.mkdirSync(root, { recursive: false });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function regularSource(relative) {
  let current = repo;
  for (const part of relative.split('/')) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `source symlink 거부: ${relative}`);
  }
  assert.ok(fs.lstatSync(current).isFile(), `source 일반 파일이 아님: ${relative}`);
  return current;
}
function addFile(sourceRelative, route) {
  safeRoute(route);
  const bytes = fs.readFileSync(regularSource(sourceRelative));
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  const digest = sha256(bytes);
  assert.equal(sha256(fs.readFileSync(target)), digest, `selected-root hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: digest });
}
function addMapped(source, routes) { for (const route of routes) addFile(source, route); }
function addTrackedTree(sourcePrefix, routePrefixes) {
  const tracked = git(['ls-files', '-z', '--', sourcePrefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(tracked.length, `tracked source tree 비어 있음: ${sourcePrefix}`);
  for (const source of tracked) {
    assert.ok(source.startsWith(`${sourcePrefix}/`), `예상 밖 tracked source: ${source}`);
    if (scriptExtensions.has(path.extname(source).toLowerCase())) continue;
    const suffix = source.slice(sourcePrefix.length + 1);
    for (const prefix of routePrefixes) addFile(source, `${prefix}/${suffix}`);
  }
}

addMapped('spec/public/roadmap.html', ['roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
addMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
for (const name of [
  'c11-css-grid.md',
  'c11-root-html-grid-column-flow-calc-auto-columns.md',
  'c11-root-html-grid-column-flow-calc-auto-columns-plan-review-20.md',
]) addMapped(`plan/${name}`, [`plan/${name}`, `spec/plan/${name}`]);
for (const name of [
  'c11-root-html-grid-column-flow-calc-auto-columns-runtime-2026-11-03',
  'c11-root-html-grid-column-flow-calc-auto-columns-precomparison-2026-11-03',
  'roadmap-c11-calc-auto-columns-sync-2026-11-03',
]) addTrackedTree(`spec/internal/evidence/${name}`, [`internal/evidence/${name}`, `spec/internal/evidence/${name}`]);
addMapped('tests/fixtures/css/c11/root-html-grid-column-flow-calc-auto-columns-precomparison.html', [
  'tests/fixtures/css/c11/root-html-grid-column-flow-calc-auto-columns-precomparison.html',
]);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('PR #1097 Rebase and merge'), 'roadmap #1097 card 누락');
assert.ok(roadmap.includes('21d7ed0eb0c5a124133db3b75cb4504c229cf14f'), 'roadmap merge SHA 누락');
assert.ok(roadmap.includes('72/213'), 'roadmap official count 누락');
const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(status.includes('- [ ] **C11 CSS Grid**'), 'C11 parent는 미완료여야 함');
assert.ok(status.includes('72/213'), 'STATUS official count 누락');

const sourceHead = git(['rev-parse', 'origin/main']).toString().trim();
const sourceTree = git(['rev-parse', 'origin/main^{tree}']).toString().trim();
assert.equal(git(['rev-parse', 'HEAD^{tree}']).toString().trim(), sourceTree, 'checkout tree가 origin/main과 다름');
assert.equal(git(['diff', '--binary', 'origin/main']).length, 0, 'origin/main 대비 tracked diff가 있음');
const report = {
  sourceHead, sourceTree, trackedDiffSha256: sha256(git(['diff', '--binary', 'origin/main'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'), stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0), selectedFromGitIndexOnly: true,
  noSymlinks: true, officialCount: '72/213', c11Complete: false, staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
