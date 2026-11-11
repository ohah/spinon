import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1363-c114-sixfit-plan-2026-11-12');
const root = path.join(buildBase, 'root');
const reportPath = path.join(evidenceDirectory, 'selected-root.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const staged = [];

for (const candidate of [buildBase, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `existing task output is preserved: ${candidate}`);
}
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe build path: ${directory}`);
}
fs.mkdirSync(root, { recursive: true });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}

function copyFile(sourceRelative, route) {
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source is not a regular file: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(hash(fs.readFileSync(target)), hash(bytes), `staging hash mismatch: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: hash(bytes) });
}

function copyTree(sourceRelative, routePrefix) {
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isDirectory() && !sourceStat.isSymbolicLink(), `source tree is unsafe: ${sourceRelative}`);
  function walk(relative = '') {
    for (const entry of fs.readdirSync(path.join(source, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = relative ? `${relative}/${entry.name}` : entry.name;
      const absolute = path.join(source, child);
      assert.ok(!entry.isSymbolicLink(), `source symlink is not accepted: ${sourceRelative}/${child}`);
      if (entry.isDirectory()) walk(child);
      else if (entry.isFile()) copyFile(`${sourceRelative}/${child}`, `${routePrefix}/${child}`);
      else throw new Error(`unsupported source entry: ${sourceRelative}/${child}`);
    }
  }
  walk();
}

for (const source of [
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.md',
]) {
  copyFile(source, source);
}

copyFile('spec/public/icon.svg', 'public/icon.svg');
copyFile('spec/public/logo.svg', 'public/logo.svg');

copyTree(
  'spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12',
);
copyFile(
  'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html',
  'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html',
);

const report = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  fullDocsDeadLinkCheck: false,
  fullSiteBuildClaim: false,
  noSymlinks: true,
  staged,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
