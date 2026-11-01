import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-plan-rollup-pr956-tailnet-20261101-pass02');
const root = path.join(buildBase, 'root');
const previousRoot = path.join(repo, 'build/spinon/roadmap-c11-plan-rollup-pr956-tailnet-20261101/root');
const previousManifestPath = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr956-plan-rollup-2026-11-01/selected-root.json');
const manifestPath = path.join(evidence, 'selected-root-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const sourceByRoute = new Map();

assert.ok(fs.existsSync(buildBase) && fs.lstatSync(buildBase).isDirectory() && !fs.lstatSync(buildBase).isSymbolicLink());
for (const target of [root, manifestPath]) assert.ok(!fs.existsSync(target), `기존 산출물을 덮어쓰지 않음: ${target}`);
for (const directory of [previousRoot, path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 source directory: ${directory}`);
}

function walk(directory, prefix = '') {
  const results = [];
  const current = path.join(directory, prefix);
  for (const entry of fs.readdirSync(current, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) results.push(...walk(directory, relative));
    else if (entry.isFile()) results.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 선택 root 항목: ${relative}`);
  }
  return results;
}

const previousFiles = walk(previousRoot);
const previousManifest = JSON.parse(fs.readFileSync(previousManifestPath, 'utf8'));
assert.equal(previousFiles.length, previousManifest.stagedFileCount);
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false });

function replaceFromRepo(sourceRelative, route) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source가 regular file이 아님: ${sourceRelative}`);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target, fs.constants.COPYFILE_FICLONE);
  const bytes = fs.readFileSync(target);
  assert.equal(sha256(bytes), sha256(fs.readFileSync(source)), `source 복사 hash 불일치: ${route}`);
  sourceByRoute.set(route, sourceRelative);
}

for (const file of ['plan/c11-css-grid.md', 'plan/c11-4-grid-track-sizing.md']) {
  replaceFromRepo(file, file);
}
const rollupEvidence = 'spec/internal/evidence/c11-4-plan-rollup-sync-pr952-2026-11-01';
for (const file of ['README.md', 'pr-diff-review-30.md']) {
  const source = `${rollupEvidence}/${file}`;
  replaceFromRepo(source, source);
  replaceFromRepo(source, source.slice('spec/'.length));
}

const overflowPlanFiles = [
  'plan/c11-4-anonymous-grid-text-computed-overflow-wrap.md',
  'plan/c11-4-anonymous-grid-text-computed-overflow-wrap-plan-review-20.md',
  'plan/c11-4-anonymous-grid-text-computed-overflow-wrap-plan-pr-diff-review-30.md',
] ;
for (const source of overflowPlanFiles) replaceFromRepo(source, source);
for (const directory of [
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01',
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01',
]) {
  const tracked = execFileSync('git', ['ls-files', '-z', '--', directory], { cwd: repo });
  const entries = tracked.toString('utf8').split('\0').filter(Boolean);
  assert.ok(entries.length > 0, 'tracked evidence is missing: ' + directory);
  for (const source of entries) {
    assert.ok(source.startsWith(directory + '/'), 'unexpected tracked path: ' + source);
    replaceFromRepo(source, source);
    replaceFromRepo(source, source.slice('spec/'.length));
  }
}
const sourceHead = git(['rev-parse', 'origin/main']);
const sourceTree = git(['rev-parse', 'origin/main^{tree}']);
assert.equal(git(['rev-parse', 'HEAD^{tree}']), sourceTree, 'checkout tree가 origin/main과 다름');
execFileSync('git', ['diff', '--quiet', 'origin/main', '--'], { cwd: repo });
const trackedDiff = execFileSync('git', ['diff', '--binary', previousManifest.sourceHead, 'origin/main', '--'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const staged = walk(root).map((route) => {
  const bytes = fs.readFileSync(path.join(root, route));
  return { source: sourceByRoute.get(route) ?? `previous-selected-root:${route}`, route, bytes: bytes.length, sha256: sha256(bytes) };
});
for (const route of ['plan/c11-css-grid.md', 'plan/c11-4-grid-track-sizing.md', 'spec/internal/evidence/c11-4-plan-rollup-sync-pr952-2026-11-01/README.md']) {
  assert.ok(staged.some((item) => item.route === route), `선택 root route 누락: ${route}`);
}
const manifest = {
  sourceHead,
  sourceTree,
  trackedDiffSha256: sha256(trackedDiff),
  previousSelectedRoot: previousManifest.selectedRoot,
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  previousStagedFileCount: previousFiles.length,
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  overlayFiles: [...sourceByRoute.entries()].map(([route, source]) => ({ route, source })),
  noSymlinks: true,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
