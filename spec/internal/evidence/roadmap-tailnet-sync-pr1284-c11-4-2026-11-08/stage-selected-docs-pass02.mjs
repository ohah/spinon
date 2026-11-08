import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-root-manifest-pass03.json'), 'utf8'));
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1284-c11-4-next-2026-11-08-pass02');
const previousRoot = path.join(repo, previousManifest.selectedRoot);
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-manifest-pass02.json');
const reportPath = path.join(evidence, 'stage-result-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const target of [buildBase, root, manifestPath, reportPath]) {
  assert.ok(!fs.existsSync(target), `preserve existing stage output: ${target}`);
}

function walk(directory, relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${child}`);
    if (entry.isDirectory()) files.push(...walk(directory, child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`unsupported filesystem entry: ${child}`);
  }
  return files;
}

const previousFiles = walk(previousRoot);
assert.equal(previousFiles.length, previousManifest.stagedFileCount, 'previous selected root file count changed');
fs.mkdirSync(buildBase, { recursive: false });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false, preserveTimestamps: true });

const sourcesByRoute = new Map();
function stage(sourceRelative, routes = [sourceRelative]) {
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source is not a regular file: ${sourceRelative}`);
  for (const route of routes) {
    assert.ok(!route.startsWith('/') && !route.split('/').includes('..'), `unsafe route: ${route}`);
    const previous = sourcesByRoute.get(route);
    assert.ok(!previous || previous === sourceRelative, `duplicate route source collision: ${route}`);
    sourcesByRoute.set(route, sourceRelative);
    const target = path.join(root, route);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.copyFileSync(source, target);
  }
}

for (const route of [
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-root-html-repeated-intrinsic-auto-rows.md',
  'plan/c11-4-root-html-repeated-intrinsic-tail-permutations.md',
  'plan/c11-4-root-html-repeated-intrinsic-tail-permutations-plan-review-20.md',
]) stage(route);

stage('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
stage('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
stage('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);

const candidateEvidence = 'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08';
const candidateRoutes = [
  'README.md',
  'expected.json',
  'chrome-observation.json',
  'chrome-verification.json',
  'chrome-agent-browser-157.png',
];
for (const filename of candidateRoutes) {
  const source = `${candidateEvidence}/${filename}`;
  stage(source, [`internal/${source.slice('spec/internal/'.length)}`, source]);
}

const previousRuntimeEvidence = 'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-runtime-2026-11-08/README.md';
stage(previousRuntimeEvidence, [
  'internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-runtime-2026-11-08/README.md',
  'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-runtime-2026-11-08/README.md',
]);

const roadmapEvidence = 'spec/internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08';
for (const filename of [
  'README.md',
  'roadmap-status-review-20.md',
  'roadmap-pr-diff-review-30.md',
  'live-before-desktop.json',
  'live-before-desktop.png',
]) {
  const source = `${roadmapEvidence}/${filename}`;
  stage(source, [`internal/${source.slice('spec/internal/'.length)}`, source]);
}

stage('tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-tail-permutations-precomparison.html');

const updatedRoutes = new Set(sourcesByRoute.keys());
const staged = [];
for (const route of walk(root)) {
  const sourceRelative = updatedRoutes.has(route)
    ? sourcesByRoute.get(route)
    : previousManifest.staged.find((item) => item.route === route)?.source;
  assert.ok(sourceRelative, `no previous source entry for staged file: ${route}`);
  const sourceBytes = fs.readFileSync(path.join(repo, sourceRelative));
  const stagedBytes = fs.readFileSync(path.join(root, route));
  assert.equal(sha256(stagedBytes), sha256(sourceBytes), `staged source hash mismatch: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: stagedBytes.length, sha256: sha256(stagedBytes) });
}

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of [
  '다음 구현: C11.4 반복 intrinsic tail 30순서',
  'Grid 반복 intrinsic tail 30순서',
  'PR #1284',
  '78/214',
]) assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
for (const marker of ['Grid 24순서 구현', '진행 중: C11.4 Grid implicit 행 목록']) {
  assert.ok(!roadmap.includes(marker), `stale active roadmap marker remains: ${marker}`);
}

const report = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  copiedFromPreviousSelectedRoot: previousManifest.selectedRoot,
  previousStagedFileCount: previousFiles.length,
  updatedRouteCount: updatedRoutes.size,
  updatedRoutes: [...updatedRoutes].sort(),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  allFilesMatchCurrentSources: true,
  noSymlinks: true,
  fullDocsDeadLinkCheck: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(reportPath, `${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
