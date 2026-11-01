import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repo, 'build/spinon/c11-4-loose-and-text-indent-flags-roadmap-pr964-pass01');
const root = path.join(buildRoot, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr956-plan-rollup-2026-11-01');
const previousRootManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-root-pass02.json'), 'utf8'));
const previousRoot = path.join(repo, previousRootManifest.selectedRoot);
const previousPublicationManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication-tailnet-pass02.json'), 'utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const tracked = execFileSync('git', ['ls-files', '-z'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 }).toString('utf8').split('\0').filter(Boolean);
const selectedSources = tracked.filter((source) =>
  source === 'plan/c11-css-grid.md' ||
  source === 'plan/c11-4-grid-track-sizing.md' ||
  source.startsWith('plan/c11-4-anonymous-grid-text-indent-flags') ||
  source.startsWith('plan/c11-4-anonymous-grid-text-line-break-loose') ||
  source === 'spec/STATUS.md' ||
  source === 'spec/internal/0059-c11-grid-layout.md' ||
  source.startsWith('spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-') ||
  source.startsWith('spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-'),
).sort();

assert.ok(!fs.existsSync(root) && !fs.existsSync(manifestPath), '기존 selected root 또는 manifest를 덮어쓰지 않음');
for (const directory of [previousRoot, path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build directory: ${directory}`);
}
assert.equal(git(['rev-parse', 'HEAD']), git(['rev-parse', 'origin/main']), 'checkout HEAD가 origin/main과 같아야 한다');
const sourceTree = git(['rev-parse', 'HEAD^{tree}']);
const roadmapDiff = execFileSync('git', ['diff', '--binary', 'HEAD', '--', 'spec/public/roadmap.html'], { cwd: repo });
const mergedSourceDiff = execFileSync('git', ['diff', '--binary', previousRootManifest.sourceHead, 'origin/main', '--'], { cwd: repo, maxBuffer: 64 * 1024 * 1024 });
const trackedDiffSha256 = sha256(Buffer.concat([mergedSourceDiff, Buffer.from('\0working-tree-roadmap\0'), roadmapDiff]));
const trackedChanges = execFileSync('git', ['diff', '--name-only', 'HEAD', '--'], { cwd: repo, encoding: 'utf8' }).trim().split('\n').filter(Boolean);
assert.deepEqual(trackedChanges, ['spec/public/roadmap.html'], 'selected root 밖 tracked 변경이 없어야 한다');

function listFiles(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `source symlink 거부: ${relative}`);
    if (entry.isDirectory()) result.push(...listFiles(directory, relative));
    else if (entry.isFile()) result.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 source entry: ${relative}`);
  }
  return result;
}

const priorFiles = listFiles(previousRoot);
assert.equal(priorFiles.length, previousRootManifest.stagedFileCount, '이전 selected-root manifest와 파일 수가 달라졌다');
fs.mkdirSync(buildRoot, { recursive: false });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false });
const overlay = new Map();

function copyRoute(sourceRelative, route) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 route: ${route}`);
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source regular file 확인 실패: ${sourceRelative}`);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  const bytes = fs.readFileSync(target);
  assert.equal(sha256(bytes), sha256(fs.readFileSync(source)), `copy hash 불일치: ${route}`);
  overlay.set(route, { source: sourceRelative, bytes: bytes.length, sha256: sha256(bytes) });
}

for (const source of selectedSources) {
  if (source.startsWith('plan/')) {
    copyRoute(source, source);
  } else if (source === 'spec/STATUS.md') {
    copyRoute(source, 'STATUS.md');
    copyRoute(source, 'spec/STATUS.md');
  } else if (source.startsWith('spec/internal/')) {
    copyRoute(source, source);
    copyRoute(source, source.slice('spec/'.length));
  }
}
copyRoute('spec/public/roadmap.html', 'roadmap.html');
copyRoute('spec/public/roadmap.html', 'public/roadmap.html');
copyRoute('spec/public/roadmap.html', 'spec/public/roadmap.html');

const staged = listFiles(root).map((route) => {
  const bytes = fs.readFileSync(path.join(root, route));
  const prior = priorFiles.includes(route) ? fs.readFileSync(path.join(previousRoot, route)) : null;
  return {
    source: overlay.get(route)?.source ?? `previous-selected-root:${route}`,
    route,
    bytes: bytes.length,
    sha256: sha256(bytes),
    changedFromPrevious: prior ? sha256(prior) !== sha256(bytes) : true,
  };
});
const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']),
  sourceTree,
  previousSourceHead: previousRootManifest.sourceHead,
  previousSelectedRoot: previousRootManifest.selectedRoot,
  previousPublicationRoot: previousPublicationManifest.publicationRoot,
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  trackedDiffSha256,
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  previousStagedFileCount: priorFiles.length,
  overlayFileCount: overlay.size,
  workingTreeRoadmapDiffSha256: sha256(roadmapDiff),
  workingTreeRoadmapSha256: sha256(fs.readFileSync(path.join(repo, 'spec/public/roadmap.html'))),
  selectedSources,
  noSymlinks: true,
  noSourceDeletes: true,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
