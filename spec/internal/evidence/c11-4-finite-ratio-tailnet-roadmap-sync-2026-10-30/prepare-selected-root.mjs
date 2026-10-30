import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const expectedHead = '9a7d04bf21d832672c4c204fb9cceafc29cdb654';
const buildRoot = path.join(repositoryRoot, 'build/spinon/c11-4-finite-ratio-tailnet-sync-2026-10-30/pass02');
const root = path.join(buildRoot, 'root');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repositoryRoot, maxBuffer: 64 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'HEAD']).toString().trim(), expectedHead);
assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), expectedHead);
assert.ok(!fs.existsSync(buildRoot), 'refusing to overwrite an existing build root');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected-root.json');

const sources = new Set([
  'plan/c11-4-direct-text-aspect-ratio-finite.md',
  'plan/c11-4-grid-track-sizing.md',
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'tests/fixtures/css/c11/grid-direct-text-aspect-ratio-range.html',
]);
for (const prefix of [
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-range-precomparison-2026-10-30',
  'spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30',
  'spec/internal/evidence/c11-4-finite-ratio-post-merge-docs-2026-10-30',
]) {
  for (const file of git(['ls-tree', '-r', '--name-only', expectedHead, '--', prefix]).toString().trim().split('\n')) {
    if (file) sources.add(file);
  }
}

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}

fs.mkdirSync(buildRoot, { recursive: true });
fs.mkdirSync(root, { recursive: false });
const staged = [];
function copySource(source, route) {
  safeRoute(route);
  const bytes = git(['show', `${expectedHead}:${source}`]);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(hash(copied), hash(bytes), `staged hash mismatch: ${route}`);
  staged.push({ source, route, bytes: bytes.length, sha256: hash(bytes) });
}

for (const source of [...sources].sort()) {
  if (source.startsWith('plan/')) {
    copySource(source, source);
    copySource(source, `spec/${source}`);
  } else if (source.startsWith('spec/public/')) {
    copySource(source, source);
    copySource(source, `public/${path.basename(source)}`);
  } else if (source.startsWith('spec/')) {
    copySource(source, source);
    copySource(source, source.slice('spec/'.length));
  } else {
    copySource(source, source);
  }
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: hash(index) });
const sourceTree = git(['rev-parse', `${expectedHead}^{tree}`]).toString().trim();
const manifest = {
  sourceHead: expectedHead,
  sourceTree,
  rootRelative: path.relative(repositoryRoot, root).split(path.sep).join('/'),
  fileCount: staged.length,
  byteCount: staged.reduce((sum, entry) => sum + entry.bytes, 0),
  staged,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ sourceHead: expectedHead, sourceTree, root: manifest.rootRelative, fileCount: manifest.fileCount, byteCount: manifest.byteCount }, null, 2) + '\n');
