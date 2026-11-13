import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const pass = 'pass02';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1378-c11-4-2026-11-14', pass);
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

for (const candidate of [base, root, manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물을 덮어쓰지 않음: ${candidate}`);
}
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 build parent: ${directory}`);
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
  let current = repo;
  for (const part of sourceRelative.split('/')) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `source symlink 거부: ${sourceRelative}`);
  }
  assert.ok(fs.lstatSync(source).isFile(), `source 일반 파일이 아님: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staging hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function copyMapped(source, routes) {
  for (const route of routes) copyFile(source, route);
}

function copyTrackedTree(sourcePrefix, routePrefixes) {
  const tracked = git(['ls-files', '-z', '--', sourcePrefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(tracked.length > 0, `tracked source tree가 비어 있음: ${sourcePrefix}`);
  for (const sourceRelative of tracked) {
    assert.ok(sourceRelative.startsWith(`${sourcePrefix}/`), `unexpected tracked source: ${sourceRelative}`);
    const suffix = sourceRelative.slice(sourcePrefix.length + 1);
    for (const routePrefix of routePrefixes) copyFile(sourceRelative, `${routePrefix}/${suffix}`);
  }
}

copyMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  copyMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
copyMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
copyMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
copyMapped('plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.md', [
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.md',
  'spec/plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.md',
]);
copyMapped('plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-implementation-review-20.md', [
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-implementation-review-20.md',
]);
copyMapped('plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-plan-review-20.md', [
  'plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-plan-review-20.md',
]);
copyMapped('tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison.html', [
  'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison.html',
]);

for (const name of [
  'c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14',
  'c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14',
]) {
  copyTrackedTree(`spec/internal/evidence/${name}`, [
    `internal/evidence/${name}`,
    `spec/internal/evidence/${name}`,
  ]);
}
copyFile(
  'spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/merge-PR1378.md',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/merge-PR1378.md',
);

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['PR #1378 Rebase and merge', 'cbf3b8579b7aa4915998378acd2d0b03c7916077', '78/214', '다음 bounded 후보 미선정']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
const status = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.match(status, /^- \[ \] \*\*C11\.4 intrinsic track sizing/m);
assert.ok(status.includes('공식 `78/214`'));

const report = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  selectedInputsSha256: sha256(Buffer.from(staged.map((item) => `${item.source}\0${item.route}\0${item.bytes}\0${item.sha256}`).join('\n'))),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  sourceTreesFromGitIndexOnly: true,
  fullDocsLinkValidation: false,
  officialCount: '78/214',
  c11Complete: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
