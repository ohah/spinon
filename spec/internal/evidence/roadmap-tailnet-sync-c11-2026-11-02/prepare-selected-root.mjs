import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const pass = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-2026-11-02/pass05');
const root = path.join(pass, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass05.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

for (const target of [pass, root, manifestPath]) assert.ok(!fs.existsSync(target), `기존 산출물 보존: ${target}`);
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(pass, { recursive: false });
fs.mkdirSync(root, { recursive: false });

const staged = [];
const scriptExtensions = new Set(['.cjs', '.js', '.jsx', '.mjs', '.sh', '.ts', '.tsx']);
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
function addFile(sourceRelative, rootRoute = sourceRelative) {
  safeRoute(rootRoute);
  const source = regularSource(sourceRelative);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, rootRoute);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  const digest = sha256(bytes);
  assert.equal(sha256(fs.readFileSync(target)), digest, `selected-root hash 불일치: ${rootRoute}`);
  staged.push({ source: sourceRelative, route: rootRoute, bytes: bytes.length, sha256: digest });
}
function addTrackedTree(sourcePrefix, rootPrefix) {
  const tracked = git(['ls-files', '-z', '--', sourcePrefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(tracked.length, `tracked source tree 비어 있음: ${sourcePrefix}`);
  for (const source of tracked) {
    assert.ok(source.startsWith(`${sourcePrefix}/`), `예상 밖 tracked source: ${source}`);
    if (scriptExtensions.has(path.extname(source).toLowerCase())) continue;
    addFile(source, `${rootPrefix}/${source.slice(sourcePrefix.length + 1)}`);
  }
}

addFile('spec/public/roadmap.html', 'roadmap.html');
addFile('spec/STATUS.md', 'STATUS.md');
addFile('spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md');
for (const name of [
  'c11-css-grid.md',
  'c11-root-html-grid-runtime.md',
  'c11-3-template-repeat-named-lines-areas.md',
  'c11-root-html-grid-integer-repeat.md',
  'c11-root-html-grid-integer-repeat-plan-review-20.md',
]) addFile(`plan/${name}`, `plan/${name}`);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) addFile(`spec/public/${name}`, `public/${name}`);

for (const name of [
  'c11-root-html-grid-integer-repeat-precomparison-2026-11-02',
  'c11-root-html-grid-integer-repeat-runtime-2026-11-02',
]) addTrackedTree(`spec/internal/evidence/${name}`, `internal/evidence/${name}`);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
for (const marker of ['#1038', 'b3ad473d4bfef4bb2f54a61603d498e620c03b0a', '72/213']) assert.ok(roadmap.includes(marker) || status.includes(marker), `C11 source marker 누락: ${marker}`);
assert.ok(status.includes('**C11 CSS Grid**') && status.includes('C11.3') && status.includes('C11.4'));

const report = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  sourceTreesFromGitIndexOnly: true,
  noSymlinks: true,
  officialCount: '72/213',
  c11Complete: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
