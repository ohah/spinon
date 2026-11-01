import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr987-2026-11-01/pass01');
const root = path.join(buildBase, 'root');
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-c11-4-hyphens-auto-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-root-pass02.json'), 'utf8'));
const previousRoot = path.join(repo, previousManifest.selectedRoot);
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const overlays = [];

for (const target of [root, manifestPath]) {
  assert.ok(!fs.existsSync(target), `기존 산출물을 덮어쓰지 않음: ${target}`);
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) paths.push(...listFiles(directory, relative));
    else if (entry.isFile()) paths.push(relative.split(path.sep).join('/'));
    else throw new Error(`지원하지 않는 파일 형식: ${relative}`);
  }
  return paths;
}

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function replaceFile(sourceRelative, route) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source가 일반 파일이 아님: ${sourceRelative}`);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  const bytes = fs.readFileSync(target);
  assert.equal(sha256(bytes), sha256(fs.readFileSync(source)), `overlay hash 불일치: ${route}`);
  overlays.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function replaceTree(sourceRelative, routePrefix) {
  const source = path.join(repo, sourceRelative);
  assertDirectory(source);
  for (const relative of listFiles(source)) replaceFile(`${sourceRelative}/${relative}`, `${routePrefix}/${relative}`);
}

assertDirectory(previousRoot);
assertDirectory(path.join(repo, 'build'));
assertDirectory(path.join(repo, 'build/spinon'));
listFiles(previousRoot);
fs.mkdirSync(buildBase, { recursive: true });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false });

for (const [source, routes] of [
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['spec/public/roadmap.html', ['public/roadmap.html', 'spec/public/roadmap.html']],
  ['spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']],
  ['plan/c11-css-grid.md', ['plan/c11-css-grid.md']],
  ['plan/c11-4-anonymous-grid-text-transform-capitalize.md', ['plan/c11-4-anonymous-grid-text-transform-capitalize.md']],
  ['spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.md', [
    'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.md',
    'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.md',
  ]],
  ['spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-post-merge-docs-review-30-2026-11-01.md', [
    'internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-post-merge-docs-review-30-2026-11-01.md',
    'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-post-merge-docs-review-30-2026-11-01.md',
  ]],
]) {
  for (const route of routes) replaceFile(source, route);
}

for (const folder of [
  'c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01',
  'c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01',
]) {
  const source = `spec/internal/evidence/${folder}`;
  replaceTree(source, `internal/evidence/${folder}`);
  replaceTree(source, source);
}

const staged = listFiles(root).map((route) => {
  const bytes = fs.readFileSync(path.join(root, route));
  return { route, bytes: bytes.length, sha256: sha256(bytes) };
});
const trackedDiff = execFileSync('git', [
  'diff', '--binary', previousManifest.sourceHead, 'HEAD', '--',
  'spec/STATUS.md', 'spec/public/roadmap.html', 'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-css-grid.md', 'plan/c11-4-anonymous-grid-text-transform-capitalize.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-post-merge-docs-review-30-2026-11-01.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01',
], { cwd: repo, maxBuffer: 16 * 1024 * 1024 });
const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']),
  previousSourceHead: previousManifest.sourceHead,
  trackedDiffSha256: sha256(trackedDiff),
  previousRoot: path.relative(repo, previousRoot).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  baseFileCount: listFiles(previousRoot).length,
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, file) => sum + file.bytes, 0),
  overlayCount: overlays.length,
  noDeletes: true,
  overlays,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined, overlays: undefined }, null, 2)}\n`);
