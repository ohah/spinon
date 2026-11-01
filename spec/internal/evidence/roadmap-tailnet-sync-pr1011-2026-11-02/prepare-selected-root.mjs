import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1011-20261102/pass07');
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass07.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();

for (const target of [buildBase, root, manifestPath]) {
  assert.ok(!fs.existsSync(target), `preserving existing output: ${target}`);
}
fs.mkdirSync(root, { recursive: true });

const copies = [];
function copy(sourceRelative, route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid route: ${route}`);
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source must be a regular file: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `copy hash mismatch: ${route}`);
  copies.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}
function copyTree(sourceRelative, routePrefix, include = (route) => /\.(?:md|png|json|log|txt|sha256|svg|html)$/i.test(route)) {
  const source = path.join(repo, sourceRelative);
  function walk(directory, relative = '') {
    for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative ? `${relative}/${entry.name}` : entry.name;
      const full = path.join(directory, child);
      assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${sourceRelative}/${child}`);
      if (entry.isDirectory() && ['pre-build-input-backups', '.app', 'Pods'].includes(entry.name)) continue;
      if (entry.isDirectory()) walk(directory, child);
      else if (entry.isFile() && include(child)) copy(`${sourceRelative}/${child}`, `${routePrefix}/${child}`);
      else if (!entry.isFile()) throw new Error(`unsupported source entry: ${sourceRelative}/${child}`);
    }
  }
  walk(source);
}

for (const [source, routes] of [
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['plan/c11-css-grid.md', ['plan/c11-css-grid.md', 'spec/plan/c11-css-grid.md']],
  ['plan/c11-root-html-grid-anonymous-text.md', [
    'plan/c11-root-html-grid-anonymous-text.md',
    'spec/plan/c11-root-html-grid-anonymous-text.md',
  ]],
  ['plan/c11-root-html-grid-anonymous-text-plan-review-20.md', [
    'plan/c11-root-html-grid-anonymous-text-plan-review-20.md',
    'spec/plan/c11-root-html-grid-anonymous-text-plan-review-20.md',
  ]],
  ['spec/public/icon.svg', ['public/icon.svg']],
  ['spec/public/logo.svg', ['public/logo.svg']],
  ['spec/public/roadmap.svg', ['public/roadmap.svg']],
]) {
  for (const route of routes) copy(source, route);
}
for (const folder of [
  'c11-root-html-grid-anonymous-text-precomparison-2026-11-02',
  'c11-root-html-grid-anonymous-text-implementation-2026-11-02',
]) {
  const source = `spec/internal/evidence/${folder}`;
  copyTree(source, `internal/evidence/${folder}`);
  copyTree(source, source);
}
for (const prefix of ['internal/evidence', 'spec/internal/evidence']) {
  copy(
    'spec/internal/evidence/roadmap-tailnet-sync-pr1011-2026-11-02/roadmap-pr-diff-review-30.md',
    `${prefix}/roadmap-tailnet-sync-pr1011-2026-11-02/roadmap-pr-diff-review-30.md`,
  );
}

const report = {
  sourceHead: git(['rev-parse', 'HEAD']),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']),
  dirtyTrackedDiffSha256: sha256(execFileSync('git', ['diff', '--binary', 'HEAD'], { cwd: repo })),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  sourceFileCount: copies.length,
  sourceBytes: copies.reduce((sum, item) => sum + item.bytes, 0),
  noSymlinks: true,
  noDeletes: true,
  copies,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, copies: undefined }, null, 2)}\n`);
