import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const workspace = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1375-2026-11-14');
const root = path.join(workspace, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const evidenceName = 'c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-runtime-2026-11-13';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const files = [];

assert.ok(!fs.existsSync(workspace), '기존 build 산출물 보존: ' + workspace);
assert.ok(!fs.existsSync(manifestPath), '기존 manifest 보존: ' + manifestPath);
for (const dir of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(dir);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), '안전하지 않은 build 경로: ' + dir);
}
fs.mkdirSync(workspace, { recursive: false });
fs.mkdirSync(root, { recursive: false });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), '잘못된 route: ' + route);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), '잘못된 route: ' + route);
}
function addFile(sourceRelative, route) {
  safeRoute(route);
  let current = repo;
  for (const part of sourceRelative.split('/')) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), 'source symlink 거부: ' + sourceRelative);
  }
  assert.ok(fs.lstatSync(current).isFile(), 'source 일반 파일이 아님: ' + sourceRelative);
  const bytes = fs.readFileSync(current);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  const digest = sha256(bytes);
  assert.equal(sha256(fs.readFileSync(target)), digest, 'selected-root hash 불일치: ' + route);
  files.push({ source: sourceRelative, route, bytes: bytes.length, sha256: digest });
}
function addMapped(source, routes) {
  for (const route of routes) addFile(source, route);
}
function addTrackedTree(prefix, routePrefixes) {
  const tracked = git(['ls-files', '-z', '--', prefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(tracked.length, 'tracked source tree 비어 있음: ' + prefix);
  const excludedExtensions = new Set(['.js', '.jsx', '.mjs', '.cjs', '.sh', '.ts', '.tsx']);
  for (const source of tracked) {
    assert.ok(source.startsWith(prefix + '/'), '예상 밖 tracked source: ' + source);
    if (excludedExtensions.has(path.extname(source).toLowerCase())) continue;
    const suffix = source.slice(prefix.length + 1);
    for (const routePrefix of routePrefixes) addFile(source, routePrefix + '/' + suffix);
  }
}

addMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped('spec/public/' + name, [name, 'public/' + name, 'spec/public/' + name]);
}
addMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
for (const name of [
  'c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum.md',
  'c11-4-grid-track-sizing.md',
]) addMapped('plan/' + name, ['plan/' + name, 'spec/plan/' + name]);
addTrackedTree('spec/internal/evidence/' + evidenceName, [
  'internal/evidence/' + evidenceName,
  'spec/internal/evidence/' + evidenceName,
]);
addTrackedTree('spec/internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14', [
  'internal/evidence/roadmap-tailnet-sync-pr1375-2026-11-14',
]);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
files.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('#1375 Rebase and merge'), 'roadmap #1375 card 누락');
for (const marker of ['22161976cab0bfd68a1d545eeb616441a0ab7a0e', '78/214', 'C11.4/C11 미완료']) {
  assert.ok(roadmap.includes(marker), 'roadmap marker 누락: ' + marker);
}
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('78/214'), '공식 상태 count 불일치');

const report = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  trackedDiffSha256: sha256(git(['diff', '--binary', 'HEAD'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: files.length,
  stagedBytes: files.reduce((sum, file) => sum + file.bytes, 0),
  sourceTreesFromGitIndexOnly: true,
  noSymlinks: true,
  officialCount: '78/214',
  c11Complete: false,
  files,
};
fs.writeFileSync(manifestPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, files: undefined }, null, 2) + '\n');
