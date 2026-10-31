import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = '2e8ab8cf5211635920430d69ff4ce34dd09f5ca4';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr896-2026-10-31/pass01');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 128 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
assert.equal(git(['rev-parse', `${head}^{tree}`]).toString().trim(), git(['rev-parse', 'HEAD^{tree}']).toString().trim());
assert.equal(git(['diff', '--quiet', 'HEAD']).toString(), '');
for (const candidate of [root, path.join(base, 'site'), path.join(base, 'publication'), path.join(base, 'publication-staging'), manifestPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물을 보존하기 위해 중단: ${candidate}`);
}
fs.mkdirSync(root, { recursive: false });

const staged = [];
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}
function copyFile(source, route) {
  safeRoute(route);
  const bytes = git(['show', `${head}:${source}`]);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const digest = hash(fs.readFileSync(destination));
  assert.equal(digest, hash(bytes), `staged hash mismatch: ${route}`);
  staged.push({ source, route, bytes: bytes.length, sha256: digest });
}
function copyMapped(source, routes) {
  for (const route of routes) copyFile(source, route);
}
function trackedFiles(prefix) {
  const records = git(['ls-tree', '-r', '-z', head, prefix]).toString().split('\0').filter(Boolean);
  return records.map((record) => {
    const tab = record.indexOf('\t');
    const meta = record.slice(0, tab).split(' ');
    const source = record.slice(tab + 1);
    assert.equal(meta[1], 'blob', `non-blob tracked source: ${source}`);
    assert.ok(meta[0] === '100644' || meta[0] === '100755', `symlink or special source: ${source}`);
    return source;
  });
}
function copyTree(prefix, routePrefix) {
  const files = trackedFiles(prefix);
  assert.ok(files.length > 0, `no tracked files: ${prefix}`);
  for (const source of files) {
    const relative = path.posix.relative(prefix, source);
    copyFile(source, path.posix.join(routePrefix, relative));
  }
}

copyMapped('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  copyMapped(`spec/public/${name}`, [name, `public/${name}`, `spec/public/${name}`]);
}
copyMapped('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
copyMapped('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']);
copyMapped('spec/internal/0070-c15-1-inline-text-layout.md', ['internal/0070-c15-1-inline-text-layout.md', 'spec/internal/0070-c15-1-inline-text-layout.md']);
copyMapped('tests/fixtures/css/c11/anonymous-grid-text-pre-precomparison.html', ['tests/fixtures/css/c11/anonymous-grid-text-pre-precomparison.html']);
for (const name of ['c11-css-grid.md', 'c11-4-grid-track-sizing.md', 'c11-4-direct-text-aspect-ratio-definite-axis.md', 'c11-4-anonymous-grid-text-pre.md', 'c11-4-anonymous-grid-text-pre-tabs.md', 'c11-4-anonymous-grid-text-pre-tab-size-values.md']) {
  copyMapped(`plan/${name}`, [`plan/${name}`, `spec/plan/${name}`]);
}
for (const prefix of [
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-axis-transfer-precomparison-2026-10-31',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31',
  'spec/internal/evidence/c11-4-anonymous-text-pre-precomparison-2026-10-31',
  'spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31',
  'spec/internal/evidence/c11-4-anonymous-pre-tabs-2026-10-31',
  'spec/internal/evidence/c11-4-anonymous-grid-text-pre-runtime-2026-10-31-2026-10-31',
  'spec/internal/evidence/roadmap-c11-4-definite-axis-pr-diff-review-30-2026-10-31.md',
  'spec/internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31',
]) {
  const isFile = prefix.endsWith('.md');
  const route = prefix.startsWith('spec/internal/') ? prefix.slice('spec/'.length) : prefix;
  if (isFile) {
    copyMapped(prefix, [route, prefix]);
  } else {
    copyTree(prefix, route);
    copyTree(prefix, prefix);
  }
}
const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: hash(index) });
const report = {
  sourceHead: head,
  sourceTree: git(['rev-parse', `${head}^{tree}`]).toString().trim(),
  trackedDiffSha256: hash(git(['diff', '--binary', 'HEAD'])),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  noSymlinks: true,
  fullDocsLinkValidation: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
