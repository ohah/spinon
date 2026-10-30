import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-4-prewrap-20261031/pass02');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

for (const candidate of [base, root, manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물을 덮어쓰지 않기 위해 중단: ${candidate}`);
}
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

function copyTree(sourceRelative, routePrefix) {
  const sourceDirectory = path.join(repo, sourceRelative);
  function walk(directory, relative = '') {
    for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = path.join(relative, entry.name);
      const absolute = path.join(directory, child);
      assert.ok(!entry.isSymbolicLink(), `source symlink 거부: ${path.relative(repo, absolute)}`);
      if (entry.isDirectory()) walk(directory, child);
      else if (entry.isFile()) {
        const childRelative = child.split(path.sep).join('/');
        copyFile(path.posix.join(sourceRelative, childRelative), path.posix.join(routePrefix, childRelative));
      } else throw new Error(`지원하지 않는 source 항목: ${absolute}`);
    }
  }
  const stat = fs.lstatSync(sourceDirectory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `일반 디렉터리가 아닌 source: ${sourceRelative}`);
  walk(sourceDirectory);
}

function copyMappedFile(source, routes) {
  for (const route of routes) copyFile(source, route);
}

copyMappedFile('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  copyMappedFile(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
copyMappedFile('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
copyMappedFile('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);

for (const name of [
  'c11-4-anonymous-grid-text-pre-wrap.md',
  'c11-4-anonymous-grid-text-pre-wrap-plan-review-20.md',
  'c11-4-anonymous-grid-text-pre-wrap-plan-pr-diff-review-30.md',
]) {
  copyMappedFile(`plan/${name}`, [`plan/${name}`, `spec/plan/${name}`]);
}

const comparison = 'spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31';
copyTree(comparison, 'internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31');
copyTree(comparison, 'spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31');

const runtime = 'spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31';
for (const file of [
  'README.md',
  'implementation-review-20.md',
  'android-build.log',
  'android-capture.log',
  'ios-build.log',
  'ios-capture.log',
]) {
  copyMappedFile(`${runtime}/${file}`, [`internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/${file}`, `spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/${file}`]);
}
for (const directory of [
  'artifact-boundary',
  'android-avd-c114-anonymous-grid-text-android-avd-20261030T182836149Z-c7a75e1a',
  'ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T182908683Z-2a6cafcf',
]) {
  copyTree(`${runtime}/${directory}`, `internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/${directory}`);
  copyTree(`${runtime}/${directory}`, `spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/${directory}`);
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const report = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  trackedDiffSha256: sha256(git(['diff', '--binary'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  fullDocsLinkValidation: false,
  noSymlinks: true,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
