import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const taskBuildRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1314-c11-4-2026-11-10');
const [pass = 'pass01'] = process.argv.slice(2);
assert.match(pass, /^pass[0-9]+$/u, `잘못된 pass 이름: ${pass}`);
const base = path.join(taskBuildRoot, pass);
const root = path.join(base, 'root');
const passSuffix = pass === 'pass01' ? '' : `-${pass}`;
const manifestPath = path.join(evidence, `selected-root${passSuffix}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

for (const target of [base, root, manifestPath]) assert.ok(!fs.existsSync(target), `기존 산출물 보존: ${target}`);
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
if (fs.existsSync(taskBuildRoot)) {
  const stat = fs.lstatSync(taskBuildRoot);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 task build 경로: ${taskBuildRoot}`);
} else fs.mkdirSync(taskBuildRoot, { recursive: false });
fs.mkdirSync(base, { recursive: false });
fs.mkdirSync(root, { recursive: false });

const staged = [];
const sha = (bytes) => sha256(bytes);
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
  const source = regularSource(sourceRelative);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  if (fs.existsSync(target)) {
    assert.equal(sha(fs.readFileSync(target)), sha(bytes), `selected-root route 충돌: ${route}`);
    return;
  }
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha(fs.readFileSync(target)), sha(bytes), `selected-root hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha(bytes) });
}
function addMapped(source, routes) {
  for (const route of routes) addFile(source, route);
}
function addMarkdown(source, routes) {
  addMapped(source, routes);
  const markdown = fs.readFileSync(regularSource(source), 'utf8');
  const references = new Set();
  for (const match of markdown.matchAll(/!\[[^\]]*\]\(([^)]+)\)/g)) {
    let reference = match[1].trim().split(/\s+/, 1)[0].replace(/^<|>$/g, '');
    if (/^(?:https?:|mailto:|data:|#)/i.test(reference) || reference.startsWith('/')) continue;
    reference = decodeURIComponent(reference.split(/[?#]/, 1)[0]);
    const sourceAsset = path.posix.normalize(path.posix.join(path.posix.dirname(source), reference));
    const absolute = path.resolve(repo, sourceAsset);
    assert.ok(absolute === repo || absolute.startsWith(`${repo}${path.sep}`), `Markdown asset 경로 이탈: ${reference}`);
    if (!fs.existsSync(absolute) || !fs.lstatSync(absolute).isFile()) continue;
    if (path.posix.extname(sourceAsset).toLowerCase() === '.md') continue;
    references.add(sourceAsset);
  }
  for (const asset of [...references].sort()) {
    const relativeAsset = path.posix.relative(path.posix.dirname(source), asset);
    for (const route of routes) {
      addFile(asset, path.posix.join(path.posix.dirname(route), relativeAsset));
    }
  }
}

addMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
addMarkdown('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMarkdown('spec/internal/README.md', ['internal/README.md', 'spec/internal/README.md']);
addMarkdown('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
addMarkdown('plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content.md', [
  'plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content.md',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content.md',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content-plan-review-20.md', [
  'plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content-plan-review-20.md',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-content-plan-review-20.md',
]);
for (const [name, files] of [
  ['c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10', ['README.md', 'plan-pr-diff-review-30.md']],
  ['c11-4-root-html-repeated-text-autoplacement-two-fit-content-implementation-2026-11-10', ['README.md', 'implementation-runtime-review-20.md', 'pr-diff-review-30.md']],
]) {
  const prefix = `spec/internal/evidence/${name}`;
  for (const filename of files) {
    const source = `${prefix}/${filename}`;
    addMarkdown(source, [`internal/evidence/${name}/${filename}`, `spec/internal/evidence/${name}/${filename}`]);
  }
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('PR #1314 Rebase and merge'), 'roadmap #1314 병합 카드 누락');
assert.ok(roadmap.includes('352e6c0b41a9fd4442eeaa5205f18dc1c94accc4'), 'roadmap merge SHA 누락');
assert.ok(roadmap.includes('78/214'), 'roadmap 공식 완료 수 누락');
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('78/214'), 'STATUS 공식 완료 수 누락');
const sourceHead = git(['rev-parse', 'HEAD']).toString().trim();
const report = {
  sourceHead,
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  sourceSymlinks: 0,
  officialCount: '78/214',
  c11Complete: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
