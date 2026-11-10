import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const taskBuildRoot = path.join(repo, 'build/spinon/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11');
const [pass = 'pass01'] = process.argv.slice(2);
assert.match(pass, /^pass[0-9]+$/u, `잘못된 pass 이름: ${pass}`);
const base = path.join(taskBuildRoot, pass);
const root = path.join(base, 'root');
const passSuffix = pass === 'pass01' ? '' : `-${pass}`;
const manifestPath = path.join(evidence, `selected-root${passSuffix}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const sourceRef = process.env.SPINON_DOC_SOURCE_REF ?? 'HEAD';
const materializedHead = git(['rev-parse', 'HEAD']).toString().trim();
const materializedTree = git(['rev-parse', 'HEAD^{tree}']).toString().trim();
const sourceHead = git(['rev-parse', sourceRef]).toString().trim();
const sourceTree = git(['rev-parse', `${sourceRef}^{tree}`]).toString().trim();
assert.equal(sourceTree, materializedTree, 'checkout tree가 선택한 source ref와 다름');
git(['diff', '--quiet', sourceRef, '--']);

function hasCommittedFile(relative) {
  const result = spawnSync('git', ['ls-tree', '-r', '--name-only', sourceRef, '--', relative], { cwd: repo, encoding: 'utf8' });
  assert.equal(result.status, 0, `git ls-tree 실패: ${relative}`);
  return result.stdout.trim().split('\n').includes(relative);
}

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
  const committed = git(['show', `${sourceRef}:${relative}`]);
  assert.equal(sha(fs.readFileSync(current)), sha(committed), `checkout source가 선택 ref와 다름: ${relative}`);
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
    if (!hasCommittedFile(sourceAsset)) continue;
    references.add(sourceAsset);
  }
  for (const asset of [...references].sort()) {
    const relativeAsset = path.posix.relative(path.posix.dirname(source), asset);
    for (const route of routes) {
      addFile(asset, path.posix.join(path.posix.dirname(route), relativeAsset));
    }
  }
}
function addDirectory(sourcePrefix, routePrefixes) {
  let rootSource = repo;
  for (const part of sourcePrefix.split('/')) {
    rootSource = path.join(rootSource, part);
    const stat = fs.lstatSync(rootSource);
    assert.ok(!stat.isSymbolicLink(), `evidence directory symlink 거부: ${sourcePrefix}`);
  }
  assert.ok(fs.lstatSync(rootSource).isDirectory(), `evidence 경로가 디렉터리가 아님: ${sourcePrefix}`);
  const tracked = git(['ls-tree', '-r', '--name-only', sourceRef, '--', sourcePrefix]).toString('utf8').trim().split('\n').filter(Boolean);
  for (const source of tracked) {
    assert.ok(source.startsWith(`${sourcePrefix}/`), `tracked evidence 경로 이탈: ${source}`);
    const stat = fs.lstatSync(path.join(repo, source));
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `tracked evidence 파일이 일반 파일이 아님: ${source}`);
    const child = path.posix.relative(sourcePrefix, source);
    for (const routePrefix of routePrefixes) addFile(source, path.posix.join(routePrefix, child));
  }
}

addMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
addMarkdown('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMarkdown('spec/0001-conformance.md', ['0001-conformance.md', 'spec/0001-conformance.md']);
addMarkdown('spec/internal/README.md', ['internal/README.md', 'spec/internal/README.md']);
addMarkdown('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
addMarkdown('plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md', [
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md',
  'spec/plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-plan-review-20.md', [
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-plan-review-20.md',
  'spec/plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-plan-review-20.md',
]);
addMapped('tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html', [
  'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum.md', [
  'plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum.md',
  'spec/plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum.md',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-plan-review-20.md', [
  'plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-plan-review-20.md',
  'spec/plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-plan-review-20.md',
]);
for (const name of [
  'c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10',
  'c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10',
  'c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-runtime-2026-11-10',
]) {
  const prefix = `spec/internal/evidence/${name}`;
  addDirectory(prefix, [`internal/evidence/${name}`, `spec/internal/evidence/${name}`]);
}

addMarkdown('spec/internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/README.md', [
  'internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/README.md',
  'spec/internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/README.md',
]);
addMarkdown('spec/internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/post-merge-docs-pr-diff-review-30.md', [
  'internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/post-merge-docs-pr-diff-review-30.md',
  'spec/internal/evidence/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11/post-merge-docs-pr-diff-review-30.md',
]);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('PR #1338 Rebase and merge'), 'roadmap #1338 implementation record missing');
assert.ok(roadmap.includes('ebcf720d65246340632273f1a40500b346cbf403'), 'roadmap #1338 merge SHA missing');
assert.ok(roadmap.includes('계획 PR #1334 Rebase and merge'), 'roadmap #1334 plan history missing');
assert.ok(roadmap.includes('50678c278dc235bfb09ad2bf24e4ef174a46358a'), 'roadmap #1334 plan merge SHA missing');
assert.ok(roadmap.includes("detailInProgress:[{ code:'C11.4 bounded'"), 'C11.4 implementation card is not in progress');
assert.ok(roadmap.includes('공식 상태 <code>78/214</code>'), 'roadmap official count missing');
assert.equal((roadmap.match(/id="kanban-heading"/gu) || []).length, 1, 'duplicate Kanban heading remains');
const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(status.includes('78/214'), 'STATUS official count mismatch');
assert.ok(status.includes('C11.4 parsed Root HTML repeated-text row-span three-fit'), 'STATUS PR #1338 entry missing');
assert.match(status, /- \[ \] \*\*C11\.4 intrinsic track sizing\*\*/u, 'C11.4 checkbox is not unchecked');
const report = {
  sourceHead,
  sourceTree,
  materializedHead,
  materializedTree,
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
