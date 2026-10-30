import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = '8c344ddf81a65ffa74803597ac431a18d1954a04';
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr831-2026-10-30/pass01');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const publication = path.join(buildBase, 'publication-staging');
const finalPublication = path.join(buildBase, 'publication');
const reportPath = path.join(evidence, 'selected-root-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 32 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
const sourceTree = git(['rev-parse', head + '^{tree}']).toString().trim();
assert.ok(!fs.existsSync(buildBase), 'refusing to reuse an existing build directory');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite an existing manifest');

const sources = new Set([
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-grid-text-last-baseline-multiline.md',
]);
for (const prefix of [
  'spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30',
  'spec/internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30',
]) {
  const files = git(['ls-tree', '-r', '--name-only', head, prefix]).toString().trim().split('\n');
  for (const file of files) if (file) sources.add(file);
}

fs.mkdirSync(buildBase, { recursive: true });
fs.mkdirSync(root, { recursive: false });
const staged = [];
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copySource(source, route) {
  safeRoute(route);
  const bytes = git(['show', head + ':' + source]);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes));
  staged.push({ source, route, bytes: bytes.length, sha256: sha256(bytes) });
}
for (const source of [...sources].sort()) {
  if (source.startsWith('plan/')) {
    copySource(source, source);
    copySource(source, 'spec/' + source);
  } else if (source.startsWith('spec/public/')) {
    const basename = path.basename(source);
    copySource(source, 'public/' + basename);
    copySource(source, source);
  } else if (source.startsWith('spec/')) {
    copySource(source, source.slice('spec/'.length));
    copySource(source, source);
  } else throw new Error('unexpected source: ' + source);
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: sha256(index) });
const report = {
  sourceHead: head,
  sourceTree,
  selectedRoot: path.relative(repo, root),
  siteRoot: path.relative(repo, site),
  sourceCount: sources.size,
  stagedRouteCount: staged.length,
  fullDocsLinkValidation: false,
  staged,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, staged: undefined }, null, 2) + '\n');
