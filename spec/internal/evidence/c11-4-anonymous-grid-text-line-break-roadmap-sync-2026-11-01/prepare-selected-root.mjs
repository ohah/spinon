import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const evidence = path.dirname(fileURLToPath(import.meta.url));
const buildBase = path.join(repo, 'build/spinon/roadmap-c114-line-break-postmerge-20261101');
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const staged = [];

for (const candidate of [buildBase, root, manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물은 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build 경로: ${directory}`);
}
fs.mkdirSync(root, { recursive: true });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function copyFile(sourceRelative, route) {
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source가 regular file이 아님: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const prior = staged.find((entry) => entry.route === route);
  if (prior) {
    assert.equal(prior.sha256, digest, `route source 충돌: ${route}`);
    return;
  }
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `staging hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: digest });
}

function copyTree(sourceRelative, routePrefix) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `source tree가 안전하지 않음: ${sourceRelative}`);
  function walk(relative = '') {
    for (const entry of fs.readdirSync(path.join(source, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative ? `${relative}/${entry.name}` : entry.name;
      const absolute = path.join(source, child);
      assert.ok(!entry.isSymbolicLink(), `source symlink 거부: ${sourceRelative}/${child}`);
      if (entry.isDirectory()) walk(child);
      else if (entry.isFile()) {
        if (/\.(?:cjs|js|jsx|mjs|ts|tsx)$/i.test(entry.name)) continue;
        copyFile(`${sourceRelative}/${child}`, `${routePrefix}/${child}`);
      } else throw new Error(`지원하지 않는 source 항목: ${sourceRelative}/${child}`);
    }
  }
  walk();
}

for (const [source, routes] of [
  ['spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']],
  ['spec/public/icon.svg', ['icon.svg', 'public/icon.svg', 'spec/public/icon.svg']],
  ['spec/public/logo.svg', ['logo.svg', 'public/logo.svg', 'spec/public/logo.svg']],
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']],
  ['plan/c11-css-grid.md', ['plan/c11-css-grid.md']],
  ['plan/c11-4-grid-track-sizing.md', ['plan/c11-4-grid-track-sizing.md']],
  ['plan/c11-4-anonymous-grid-text-computed-line-break.md', ['plan/c11-4-anonymous-grid-text-computed-line-break.md']],
  ['plan/c11-4-anonymous-grid-text-computed-line-break-plan-review-20.md', ['plan/c11-4-anonymous-grid-text-computed-line-break-plan-review-20.md']],
  ['plan/c11-4-anonymous-grid-text-computed-line-break-plan-pr-diff-review-20.md', ['plan/c11-4-anonymous-grid-text-computed-line-break-plan-pr-diff-review-20.md']],
]) {
  for (const route of routes) copyFile(source, route);
}

for (const name of [
  'c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01',
  'c11-4-anonymous-grid-text-line-break-runtime-2026-11-01',
]) {
  const source = `spec/internal/evidence/${name}`;
  copyTree(source, `internal/evidence/${name}`);
  copyTree(source, `spec/internal/evidence/${name}`);
}

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['PR #945 Rebase and merge', '9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d', '72/213']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
const manifest = {
  sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
  sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(),
  trackedDiffSha256: sha256(execFileSync('git', ['diff', '--binary'], { cwd: repo })),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  fullDocsDeadLinkCheck: false,
  noSymlinks: true,
  roadmapMarkers: ['PR #945 Rebase and merge', '9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d', '72/213'],
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, entry) => sum + entry.bytes, 0),
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
