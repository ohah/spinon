import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-after-pr1369-2026-11-12');
const root = path.join(buildBase, 'root');
const reportPath = path.join(evidenceDirectory, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
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
  safeRoute(sourceRelative);
  safeRoute(route);
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source is not a regular file: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staging hash mismatch: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

function copyTree(sourceRelative, routePrefix) {
  safeRoute(sourceRelative);
  safeRoute(routePrefix);
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
      else throw new Error(`unsupported source entry: ${absolute}`);
    }
  }
  walk();
}

copyFile('spec/STATUS.md', 'STATUS.md');
copyFile('spec/STATUS.md', 'spec/STATUS.md');
copyFile('plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md');
copyFile('plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md');
copyFile('plan/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md', 'plan/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md');
copyFile('plan/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-plan-review-20.md', 'plan/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-plan-review-20.md');
copyFile('spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md');
copyFile('spec/public/icon.svg', 'public/icon.svg');
copyFile('spec/public/logo.svg', 'public/logo.svg');
copyTree(
  'spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12',
);
copyTree(
  'spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-runtime-2026-11-12',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-runtime-2026-11-12',
);

const report = {
  schema: 'spinon.tailnet-preview-selected-root/v1',
  sourceCommit: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim(),
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
