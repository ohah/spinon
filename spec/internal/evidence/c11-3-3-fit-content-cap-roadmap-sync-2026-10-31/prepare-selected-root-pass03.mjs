import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-fit-content-cap-20261031/pass05');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass03.json');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const sourceHead = git(['rev-parse', 'origin/main']);
const sourceTree = git(['rev-parse', 'origin/main^{tree}']);
const worktreeHead = git(['rev-parse', 'HEAD']);
const trackedDiff = execFileSync('git', ['diff', '--binary', 'origin/main'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const skippedPageSources = [];

assert.equal(worktreeHead, sourceHead, '선택 root는 현재 origin/main과 같은 HEAD에서 작성한다');
for (const candidate of [base, root, manifestPath]) assert.ok(!fs.existsSync(candidate), `기존 산출물을 덮어쓰지 않음: ${candidate}`);
fs.mkdirSync(root, { recursive: true });
const staged = [];

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function copyFile(sourceRelative, route) {
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `일반 파일이 아닌 source: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staging hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function copyMappedFile(source, routes) {
  for (const route of routes) copyFile(source, route);
}

function copyTree(sourceRelative, routePrefix) {
  const listed = execFileSync('git', ['ls-tree', '-r', '--name-only', '-z', 'origin/main', '--', sourceRelative], { cwd: repo }).toString().split('\0').filter(Boolean);
  assert.ok(listed.length > 0, `tracked source directory is empty: ${sourceRelative}`);
  for (const source of listed) {
    if (source.endsWith('.js')) { skippedPageSources.push(source); continue; }
    assert.ok(source.startsWith(sourceRelative + '/'), `source escaped selected directory: ${source}`);
    const relative = source.slice(sourceRelative.length + 1);
    copyFile(source, path.posix.join(routePrefix, relative));
  }
}

copyMappedFile('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg']) copyMappedFile(`spec/public/${name}`, [`public/${name}`, `spec/public/${name}`]);
copyMappedFile('spec/public/logo.svg', ['logo.svg']);
copyMappedFile('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
copyMappedFile('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);

copyMappedFile('plan/c11-3-3-template-area-fit-content-cap.md', [
  'plan/c11-3-3-template-area-fit-content-cap.md',
  'spec/plan/c11-3-3-template-area-fit-content-cap.md',
]);

for (const name of ['c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31']) {
  const source = `spec/internal/evidence/${name}`;
  copyTree(source, `internal/evidence/${name}`);
  copyTree(source, `spec/internal/evidence/${name}`);
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['PR #911', '72/213', 'c11-3-3-template-area-fit-content-cap.html', 'c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
const report = {
  sourceHead,
  sourceTree,
  worktreeHead,
  trackedDiffSha256: sha256(trackedDiff),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  fullDocsLinkValidation: false,
  noSymlinks: true,
  skippedPageSources,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
