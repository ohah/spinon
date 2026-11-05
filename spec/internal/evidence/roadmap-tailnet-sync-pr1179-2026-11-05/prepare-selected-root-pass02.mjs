import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const taskRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1179-2026-11-05/pass02');
const runRoot = path.join(taskRoot, 'pass01');
const base = path.join(runRoot, 'pass01');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const scriptExtensions = new Set(['.cjs', '.js', '.jsx', '.mjs', '.sh', '.ts', '.tsx']);

for (const target of [taskRoot, runRoot, base, root, manifestPath]) assert.ok(!fs.existsSync(target), `기존 산출물 보존: ${target}`);
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(taskRoot, { recursive: false });
fs.mkdirSync(runRoot, { recursive: false });
fs.mkdirSync(base, { recursive: false });
fs.mkdirSync(root, { recursive: false });

const staged = [];
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
function addMapped(source, routes) {
  for (const route of routes) addFile(source, route);
}
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

addMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
addMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
addMapped('plan/c11-root-html-grid-auto-repeat.md', ['plan/c11-root-html-grid-auto-repeat.md']);
for (const name of [
  'c11-root-html-grid-auto-repeat-precomparison-2026-11-05',
  'c11-root-html-grid-auto-repeat-runtime-2026-11-05',
]) addTrackedTree(`spec/internal/evidence/${name}`, [`internal/evidence/${name}`, `spec/internal/evidence/${name}`]);
addMapped('tests/fixtures/css/c11/root-html-grid-auto-repeat-precomparison.html', [
  'tests/fixtures/css/c11/root-html-grid-auto-repeat-precomparison.html',
]);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const [pr, mergeSha] of [
  ['1176', 'da67a18032f449d8477cdd5d7ea0aba02e691b0b'],
  ['1179', '7c042f4ac5080609927fb26760112c5a88a56d3c'],
]) {
  assert.ok(roadmap.includes(`PR #${pr} Rebase and merge`), `roadmap #${pr} 카드 누락`);
  assert.ok(roadmap.includes(mergeSha), `roadmap #${pr} merge SHA 누락`);
}
assert.ok(roadmap.includes('78/214'), 'roadmap 공식 완료 수 누락');
const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(status.includes('78/214'), 'STATUS 공식 완료 수 불일치');
assert.match(status, /- \[ \] <span id="c11-css-grid"/u, 'C11 checkbox가 미완료가 아님');

const selectedDiffPaths = [
  'plan/c11-root-html-grid-auto-repeat.md',
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/public/roadmap.html',
];
const sourceHead = git(['rev-parse', 'HEAD']).toString().trim();
const report = {
  sourceHead,
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  trackedDiffSha256: sha256(git(['diff', '--binary', sourceHead, '--', ...selectedDiffPaths])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  noSymlinks: true,
  officialCount: '78/214',
  c11Complete: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
