import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const taskBuildRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1320-c11-4-2026-11-10');
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
addMarkdown('plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md', [
  'plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-plan-review-20.md', [
  'plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-plan-review-20.md',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-plan-review-20.md',
]);
addMarkdown('plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-pr-review-30.md', [
  'plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-pr-review-30.md',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-pr-review-30.md',
]);
const autoMinEvidence = 'c11-4-root-html-repeated-text-autoplacement-two-fit-auto-min-precomparison-2026-11-10';
addMarkdown(`spec/internal/evidence/${autoMinEvidence}/README.md`, [
  `internal/evidence/${autoMinEvidence}/README.md`,
  `spec/internal/evidence/${autoMinEvidence}/README.md`,
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
const implementationEvidence = 'c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-simulators-2026-11-10';
for (const filename of ['README.md', 'implementation-runtime-review-20.md', 'pr-diff-review-30.md']) {
  const source = `spec/internal/evidence/${implementationEvidence}/${filename}`;
  addMarkdown(source, [`internal/evidence/${implementationEvidence}/${filename}`, `spec/internal/evidence/${implementationEvidence}/${filename}`]);
}

const nextPlan = 'c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum';
const nextPlanReview = `${nextPlan}-plan-review-20`;
addMarkdown(`plan/${nextPlan}.md`, [`plan/${nextPlan}.md`, `spec/plan/${nextPlan}.md`]);
addMarkdown(`plan/${nextPlanReview}.md`, [`plan/${nextPlanReview}.md`, `spec/plan/${nextPlanReview}.md`]);
const previousOneFitPlan = 'c11-4-root-html-repeated-text-autoplacement-fit-content-positions';
addMarkdown(`plan/${previousOneFitPlan}.md`, [`plan/${previousOneFitPlan}.md`, `spec/plan/${previousOneFitPlan}.md`]);
addMarkdown(`plan/${previousOneFitPlan}-plan-review-20.md`, [
  `plan/${previousOneFitPlan}-plan-review-20.md`,
  `spec/plan/${previousOneFitPlan}-plan-review-20.md`,
]);

const nextEvidence = 'c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10';
const nextEvidenceSource = `spec/internal/evidence/${nextEvidence}`;
const nextEvidenceRoutes = [`internal/evidence/${nextEvidence}`, `spec/internal/evidence/${nextEvidence}`];
addMarkdown(`${nextEvidenceSource}/README.md`, nextEvidenceRoutes.map((route) => `${route}/README.md`));
for (const filename of [
  'SHA256SUMS',
  'agent-browser-lifecycle.json',
  'capture-chrome.mjs',
  'chrome-matrix-observation.ndjson',
  'chrome157-one-fit-auto-min-sample.png',
  'matrix-verification.json',
  'oracle-fixture.html',
  'plan-pr-diff-review-20.md',
  'verify-chrome-matrix.py',
]) {
  for (const route of nextEvidenceRoutes) addFile(`${nextEvidenceSource}/${filename}`, `${route}/${filename}`);
}

const previousOneFitEvidence = 'c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09';
const previousOneFitEvidenceSource = `spec/internal/evidence/${previousOneFitEvidence}`;
const previousOneFitEvidenceRoutes = [`internal/evidence/${previousOneFitEvidence}`, `spec/internal/evidence/${previousOneFitEvidence}`];
addMarkdown(`${previousOneFitEvidenceSource}/README.md`, previousOneFitEvidenceRoutes.map((route) => `${route}/README.md`));
for (const filename of ['SHA256SUMS', 'chrome157-matrix-observation.ndjson', 'chrome157-sample.png', 'matrix-verification.json', 'verify-chrome-matrix.py']) {
  for (const route of previousOneFitEvidenceRoutes) addFile(`${previousOneFitEvidenceSource}/${filename}`, `${route}/${filename}`);
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.ok(roadmap.includes('#1318 Rebase and merge'), 'roadmap #1318 병합 카드 누락');
assert.ok(roadmap.includes(`/spinon/plan/${nextPlan}.html`), '다음 one-fit auto-minimum plan 링크 누락');
assert.ok(staged.some((item) => item.route === `internal/evidence/${nextEvidence}/README.md`), '다음 one-fit 사전 비교 source route 누락');
assert.ok(roadmap.includes('e9f7f4abf32619ccf2739d077c5c8db16ff35f13'), 'roadmap merge SHA 누락');
assert.ok(roadmap.includes('78/214'), 'roadmap 공식 완료 수 누락');
assert.ok(roadmap.includes('/spinon/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.html'), '구현 계획 route 누락');
assert.ok(roadmap.includes(`/spinon/internal/evidence/${implementationEvidence}/README.html`), '구현 evidence route 누락');
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('78/214'), 'STATUS 공식 완료 수 누락');
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('e9f7f4abf32619ccf2739d077c5c8db16ff35f13'), '구현 PR #1318 merge SHA 누락');
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('1,440/1,440 scene'), 'one-fit auto-minimum plan 상태 누락');
assert.ok(fs.readFileSync(path.join(root, 'internal/0059-c11-grid-layout.md'), 'utf8').includes('0.1.0'), '내부 계약 numeric version 누락');
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
