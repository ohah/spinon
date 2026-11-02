import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const build = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1015-20261102');
const root = path.join(build, 'root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const sourceHead = git(['rev-parse', 'HEAD']);
assert.equal(sourceHead, git(['rev-parse', 'origin/main']), 'source must be merged origin/main');
assert.equal(git(['status', '--porcelain', '--untracked-files=no']), '', 'tracked working tree must be clean');
for (const target of [build, manifestPath]) assert.ok(!fs.existsSync(target), `preserving existing output: ${target}`);
fs.mkdirSync(root, { recursive: true });

const copies = [];
function copy(sourceRelative, route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid route: ${route}`);
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source must be regular: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `copy hash mismatch: ${route}`);
  copies.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}
function copyEvidence(sourceRelative, prefix) {
  const source = path.join(repo, sourceRelative);
  function walk(directory, relative = '') {
    for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative ? `${relative}/${entry.name}` : entry.name;
      assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${sourceRelative}/${child}`);
      if (entry.isDirectory()) walk(directory, child);
      else if (entry.isFile() && /\.(?:md|png|json|log|txt|sha256|svg|html)$/i.test(child)) copy(`${sourceRelative}/${child}`, `${prefix}/${child}`);
      else if (!entry.isFile()) throw new Error(`unsupported source entry: ${sourceRelative}/${child}`);
    }
  }
  walk(source);
}

for (const [source, routes] of [
  ['spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']],
  ['spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']],
  ['spec/public/icon.svg', ['icon.svg', 'public/icon.svg']],
  ['spec/public/logo.svg', ['public/logo.svg']],
  ['spec/public/roadmap.svg', ['public/roadmap.svg']],
  ['plan/c11-css-grid.md', ['plan/c11-css-grid.md', 'spec/plan/c11-css-grid.md']],
  ['plan/c11-root-html-grid-fr-track.md', ['plan/c11-root-html-grid-fr-track.md', 'spec/plan/c11-root-html-grid-fr-track.md']],
  ['spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md']],
]) for (const route of routes) copy(source, route);

const evidenceFolder = 'c11-root-html-grid-fr-track-runtime-2026-11-02';
for (const prefix of ['internal/evidence', 'spec/internal/evidence']) {
  copyEvidence(`spec/internal/evidence/${evidenceFolder}`, `${prefix}/${evidenceFolder}`);
}

const manifest = {
  sourceHead,
  sourceTree: git(['rev-parse', 'HEAD^{tree}']),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  sourceFileCount: copies.length,
  sourceBytes: copies.reduce((sum, item) => sum + item.bytes, 0),
  noSymlinks: true,
  noDeletes: true,
  copies,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, copies: undefined }, null, 2)}\n`);
