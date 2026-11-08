import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-4-pr1277-status-docs-2026-11-08');
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-manifest.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const staged = [];

for (const target of [buildBase, root, manifestPath]) {
  assert.ok(!fs.existsSync(target), `기존 산출물을 보존함: ${target}`);
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
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `일반 파일이 아님: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staging hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function copyTree(sourceRelative, routePrefix) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 source tree: ${sourceRelative}`);
  function walk(relative = '') {
    for (const entry of fs.readdirSync(path.join(source, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative ? `${relative}/${entry.name}` : entry.name;
      const absolute = path.join(source, child);
      assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${sourceRelative}/${child}`);
      if (entry.isDirectory()) walk(child);
      else if (entry.isFile()) {
        if (['.cjs', '.js', '.jsx', '.mjs', '.ts', '.tsx'].includes(path.extname(entry.name).toLowerCase())) continue;
        copyFile(`${sourceRelative}/${child}`, `${routePrefix}/${child}`);
      } else throw new Error(`지원하지 않는 source 항목: ${sourceRelative}/${child}`);
    }
  }
  walk();
}

for (const [source, routes] of [
  ['spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']],
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['spec/public/icon.svg', ['icon.svg', 'public/icon.svg']],
  ['spec/public/logo.svg', ['logo.svg', 'public/logo.svg']],
  ['spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md']],
  ['plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.md', ['plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.md']],
  ['plan/c11-4-root-html-fit-content-min-content-auto-remaining-order-plan-review-20.md', ['plan/c11-4-root-html-fit-content-min-content-auto-remaining-order-plan-review-20.md']],
  ['plan/c11-4-grid-track-sizing.md', ['plan/c11-4-grid-track-sizing.md']],
  ['plan/c11-4-root-html-fit-content-min-content-auto-track-list.md', ['plan/c11-4-root-html-fit-content-min-content-auto-track-list.md']],
  ['tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html', ['tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html']],
  ['tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html', ['tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html']],
]) {
  for (const route of routes) copyFile(source, route);
}

for (const name of [
  'c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08',
  'c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08',
  'roadmap-tailnet-sync-pr1277-c11-4-2026-11-08',
]) {
  const source = `spec/internal/evidence/${name}`;
  copyTree(source, `internal/evidence/${name}`);
}

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of [
  '진행 중: C11.4 Grid 트랙 크기',
  'Grid 트랙 크기',
  'C11.4 · 진행 중',
  '78/214',
  '로드맵 진행 표시와 링크 점검',
]) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);

const report = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((total, item) => total + item.bytes, 0),
  fullDocsDeadLinkCheck: false,
  noSymlinks: true,
  excludedToolingExtensions: ['.cjs', '.js', '.jsx', '.mjs', '.ts', '.tsx'],
  roadmapMarkers: ['진행 중: C11.4 Grid 트랙 크기', 'C11.4 · 진행 중', '78/214'],
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
