import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr718-root-selected');
const reportPath = path.join(evidenceDirectory, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args, options = {}) => execFileSync('git', args, { cwd: repositoryRoot, ...options });

const sourceHead = git(['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
const sourceTree = git(['rev-parse', 'HEAD^{tree}'], { encoding: 'utf8' }).trim();
assert.equal(git(['rev-parse', 'origin/main'], { encoding: 'utf8' }).trim(), sourceHead, 'checkout must match origin/main');
assert.ok(!fs.existsSync(selectedRoot), `refusing to overwrite selected root: ${selectedRoot}`);
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);

const tracked = git(['ls-tree', '-r', '--name-only', '-z', 'HEAD'], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 })
  .split('\0').filter(Boolean);
const sources = new Set([
  'spec/public/roadmap.html',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/STATUS.md',
  'plan/c11-4-grid-track-sizing.md',
  'crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs',
]);
for (const file of tracked) {
  if (file.startsWith('plan/c11-4-flex-stage-oracle')) sources.add(file);
  if (file.startsWith('spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/')) sources.add(file);
  if (file.startsWith('spec/internal/evidence/c11-4-flex-stage-oracle-implementation-2026-10-28/')) sources.add(file);
}
const sourcePaths = [...sources].sort();
for (const source of sourcePaths) assert.ok(tracked.includes(source), `source is not tracked at HEAD: ${source}`);

function routesFor(source) {
  if (source === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html', source];
  if (source === 'spec/public/icon.svg' || source === 'spec/public/logo.svg') return [source.slice('spec/'.length), source];
  if (source === 'spec/STATUS.md') return ['STATUS.md', source];
  if (source.startsWith('plan/')) return [source, `spec/${source}`];
  if (source.startsWith('spec/internal/')) return [source, source.slice('spec/'.length)];
  return [source];
}

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}

fs.mkdirSync(selectedRoot, { recursive: false });
const staged = [];
for (const source of sourcePaths) {
  const bytes = git(['show', `HEAD:${source}`], { maxBuffer: 64 * 1024 * 1024 });
  assert.deepEqual(fs.readFileSync(path.join(repositoryRoot, source)), bytes, `working copy differs from HEAD: ${source}`);
  for (const route of routesFor(source)) {
    safeRoute(route);
    const destination = path.join(selectedRoot, route);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.deepEqual(copied, bytes, `staged bytes differ: ${route}`);
    staged.push({ source, path: route, bytes: copied.length, sha256: sha256(copied) });
  }
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html)\n');
fs.writeFileSync(path.join(selectedRoot, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', path: 'index.md', bytes: index.length, sha256: sha256(index) });

const report = {
  sourceHead,
  sourceTree,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  sourceCount: sourcePaths.length,
  routeCount: staged.length,
  totalBytes: staged.reduce((sum, entry) => sum + entry.bytes, 0),
  sources: sourcePaths,
  staged,
  fullDocsLinkValidation: false,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
