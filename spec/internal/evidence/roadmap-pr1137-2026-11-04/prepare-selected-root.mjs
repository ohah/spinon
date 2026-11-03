import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const pass = 'pass03';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1137-2026-11-04', pass);
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, `selected-root-${pass}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
function ensureDirectory(directory) {
  if (fs.existsSync(directory)) {
    assertDirectory(directory);
    return;
  }
  const parent = path.dirname(directory);
  if (parent !== directory) ensureDirectory(parent);
  fs.mkdirSync(directory, { recursive: false });
  assertDirectory(directory);
}
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function regularSource(relative) {
  let current = repo;
  for (const part of relative.split('/')) {
    current = path.join(current, part);
    assert.ok(!fs.lstatSync(current).isSymbolicLink(), `source symlink 거부: ${relative}`);
  }
  assert.ok(fs.lstatSync(current).isFile(), `source 일반 파일이 아님: ${relative}`);
  return current;
}

assert.ok(!fs.existsSync(base), `기존 build 산출물을 보존함: ${base}`);
assert.ok(!fs.existsSync(manifestPath), `기존 manifest를 보존함: ${manifestPath}`);
ensureDirectory(base);
ensureDirectory(root);

const staged = [];
function addFile(sourceRelative, route) {
  safeRoute(route);
  const source = regularSource(sourceRelative);
  const target = path.join(root, route);
  ensureDirectory(path.dirname(target));
  const bytes = fs.readFileSync(source);
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  const digest = sha256(bytes);
  assert.equal(sha256(fs.readFileSync(target)), digest, `selected-root hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: digest });
}
function addMapped(source, routes) {
  for (const route of routes) addFile(source, route);
}
function addTree(sourcePrefix, routePrefixes) {
  const files = git(['ls-files', '-z', '--', sourcePrefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(files.length, `tracked source tree 비어 있음: ${sourcePrefix}`);
  for (const source of files) {
    if (!source.startsWith(`${sourcePrefix}/`)) continue;
    const suffix = source.slice(sourcePrefix.length + 1);
    for (const prefix of routePrefixes) addFile(source, `${prefix}/${suffix}`);
  }
}

addMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
addMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMapped('plan/c11-root-html-grid-explicit-placement.md', [
  'plan/c11-root-html-grid-explicit-placement.md',
  'spec/plan/c11-root-html-grid-explicit-placement.md',
]);
for (const name of ['c11-css-grid.md', 'c11-2-1-explicit-grid-placement.md']) {
  addMapped(`plan/${name}`, [`plan/${name}`, `spec/plan/${name}`]);
}
addMapped('spec/internal/0059-c11-grid-layout.md', [
  'internal/0059-c11-grid-layout.md',
  'spec/internal/0059-c11-grid-layout.md',
]);
addTree('spec/internal/evidence/c11-root-html-grid-explicit-placement-precomparison-2026-11-04', [
  'internal/evidence/c11-root-html-grid-explicit-placement-precomparison-2026-11-04',
  'spec/internal/evidence/c11-root-html-grid-explicit-placement-precomparison-2026-11-04',
]);
addTree('spec/internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04', [
  'internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04',
  'spec/internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04',
]);
addFile('spec/internal/evidence/roadmap-pr1137-2026-11-04/roadmap-pr-diff-review-30.md',
  'internal/evidence/roadmap-pr1137-2026-11-04/roadmap-pr-diff-review-30.md');
addFile('spec/internal/evidence/roadmap-pr1137-2026-11-04/roadmap-pr-diff-review-30.md',
  'spec/internal/evidence/roadmap-pr1137-2026-11-04/roadmap-pr-diff-review-30.md');
addFile('spec/internal/evidence/roadmap-pr1137-2026-11-04/README.md',
  'internal/evidence/roadmap-pr1137-2026-11-04/README.md');
addFile('spec/internal/evidence/roadmap-pr1137-2026-11-04/README.md',
  'spec/internal/evidence/roadmap-pr1137-2026-11-04/README.md');

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['PR #1137 Rebase and merge', '6af14f69512c0c73d890efc728751e0ca342f9c0', '72/213']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('72/213'));

const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  trackedDiffSha256: sha256(git(['diff', '--binary', 'HEAD'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  fileCount: staged.length,
  totalBytes: staged.reduce((sum, file) => sum + file.bytes, 0),
  onlyListedSourcesCopied: true,
  noSymlinks: true,
  officialCount: '72/213',
  c11Complete: false,
  files: staged,
};
const manifestText = `${JSON.stringify(manifest, null, 2)}\n`;
fs.writeFileSync(manifestPath, manifestText, { flag: 'wx' });
const manifestRoute = path.join(root, 'internal/evidence/roadmap-pr1137-2026-11-04/selected-root-pass03.json');
ensureDirectory(path.dirname(manifestRoute));
fs.writeFileSync(manifestRoute, manifestText, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, files: undefined }, null, 2)}\n`);
