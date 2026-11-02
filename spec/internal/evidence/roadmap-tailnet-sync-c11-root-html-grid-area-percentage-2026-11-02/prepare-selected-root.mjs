import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-root-html-percentage-2026-11-02/pass02');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

assert.ok(!fs.existsSync(base), `기존 build 결과 보존: ${base}`);
assert.ok(!fs.existsSync(manifestPath), `기존 manifest 보존: ${manifestPath}`);
fs.mkdirSync(root, { recursive: true });
const staged = [];
const scriptExtensions = new Set(['.cjs', '.js', '.jsx', '.mjs', '.sh', '.ts', '.tsx']);

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function addFile(sourceRelative, route) {
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  assert.ok(!fs.lstatSync(source).isSymbolicLink() && fs.lstatSync(source).isFile(), `source 일반 파일 확인: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `selected-root hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function addMapped(source, routes) {
  for (const route of routes) addFile(source, route);
}

function addTrackedTree(sourcePrefix, routePrefixes) {
  const tracked = git(['ls-files', '-z', '--', sourcePrefix]).toString('utf8').split('\0').filter(Boolean);
  assert.ok(tracked.length, `tracked source tree 비어 있음: ${sourcePrefix}`);
  for (const source of tracked) {
    assert.ok(source.startsWith(`${sourcePrefix}/`), `예상 밖 tracked source: ${source}`);
    if (scriptExtensions.has(path.extname(source).toLowerCase())) continue;
    const suffix = source.slice(sourcePrefix.length + 1);
    for (const prefix of routePrefixes) addFile(source, `${prefix}/${suffix}`);
  }
}

for (const name of ['roadmap.html', 'icon.svg', 'logo.svg', 'roadmap.svg']) {
  addMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
addMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
addMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
for (const name of [
  'c11-css-grid.md',
  'c11-root-html-grid-area-percentage.md',
  'c11-root-html-grid-runtime.md',
  'c11-root-html-grid-percentage-tracks.md',
  'c11-root-html-grid-template-areas.md',
  'c11-root-html-grid-fr-named-areas.md',
]) {
  addMapped(`plan/${name}`, [`plan/${name}`, `spec/plan/${name}`]);
}
for (const name of [
  'c11-root-html-grid-area-percentage-precomparison-2026-11-02',
  'c11-root-html-grid-area-percentage-runtime-2026-11-02',
  'c11-root-html-grid-area-percentage-postmerge-roadmap-2026-11-02',
]) {
  const prefix = `spec/internal/evidence/${name}`;
  addTrackedTree(prefix, [`internal/evidence/${name}`, `spec/internal/evidence/${name}`]);
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
assert.equal(roadmap.split('PR #1035 Rebase and merge').length - 1, 1, 'roadmap #1035 card가 정확히 하나여야 함');
for (const marker of ['4b921f3e8ccd86e7e156562bb7e5b4a464f082f2', '72/213']) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
assert.ok(fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8').includes('72/213'));

const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']).toString().trim(),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']).toString().trim(),
  trackedDiffSha256: sha256(git(['diff', '--binary', 'HEAD'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  selectedRootFileCount: staged.length,
  selectedRootBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  sourceTreesFromGitIndexOnly: true,
  noSymlinks: true,
  officialCount: '72/213',
  c11Complete: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
