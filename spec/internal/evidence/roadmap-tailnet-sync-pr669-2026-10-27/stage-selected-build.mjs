import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceManifest = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-source.json'), 'utf8'));
const siteRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr669-20261027-site');
const stageRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr669-20261027-stage');
const reportPath = path.join(evidenceDirectory, 'selected-build.json');
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(head, sourceManifest.sourceHead, 'source changed after selected-root preparation');
assert.ok(!fs.existsSync(stageRoot), `refusing to replace stage root: ${stageRoot}`);
assert.ok(!fs.existsSync(reportPath), `refusing to replace ${reportPath}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const records = new Map();
const staticPattern = /\.(?:css|js|svg|png|jpe?g|webp|gif|woff2?|ttf|ico)$/i;

function add(route, source, kind) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing or unsafe source: ${source}`);
  const bytes = fs.readFileSync(source);
  const hash = sha256(bytes);
  const existing = records.get(route);
  if (existing) {
    assert.equal(existing.sha256, hash, `conflicting route sources: ${route}`);
    return;
  }
  records.set(route, { path: route, source: path.relative(repositoryRoot, source).split(path.sep).join('/'), kind, bytes: bytes.length, sha256: hash });
}

const generated = [
  'roadmap.html',
  'plan/c11-4-align-content-normal-runtime.html',
  'plan/c11-4-grid-track-sizing.html',
  'STATUS.html',
  'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'spec/internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/README.html',
  'spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/README.html',
  'internal/evidence/c11-4-align-content-normal-runtime-2026-10-27/README.html',
  'internal/evidence/c11-4-align-content-normal-runtime-2026-10-27/implementation-review-20.html',
  'internal/evidence/c11-4-align-content-normal-runtime-2026-10-27/pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/pr-diff-review-30.html',
];
for (const route of generated) add(route, path.join(siteRoot, route), 'generated-page');

function addRawTree(sourceRelative, routePrefix) {
  const sourceDirectory = path.join(repositoryRoot, sourceRelative);
  for (const entry of fs.readdirSync(sourceDirectory, { withFileTypes: true })) {
    const sourceRel = path.join(sourceRelative, entry.name).split(path.sep).join('/');
    const route = path.posix.join(routePrefix, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`refusing symlink: ${sourceRel}`);
    if (entry.isDirectory()) addRawTree(sourceRel, route);
    else if (!entry.name.endsWith('.md')) add(route, path.join(repositoryRoot, sourceRel), 'source-asset');
  }
}

for (const folder of [
  'c11-4-align-content-normal-runtime-precomparison-2026-10-27',
  'c11-4-align-content-normal-runtime-2026-10-27',
]) {
  addRawTree(`spec/internal/evidence/${folder}`, `internal/evidence/${folder}`);
  addRawTree(`spec/internal/evidence/${folder}`, `spec/internal/evidence/${folder}`);
}
add('tests/fixtures/css/c11/grid-track-sizing-c114-align-content-normal-runtime.html', path.join(repositoryRoot, 'tests/fixtures/css/c11/grid-track-sizing-c114-align-content-normal-runtime.html'), 'source-asset');

for (const item of [...records.values()]) {
  if (item.kind !== 'generated-page') continue;
  const html = fs.readFileSync(path.join(siteRoot, item.path), 'utf8');
  for (const match of html.matchAll(/(?:href|src)=["']\/spinon\/([^"'#?]+)["']/g)) {
    const route = match[1];
    if (!staticPattern.test(route) || records.has(route)) continue;
    const asset = path.join(siteRoot, route);
    try { add(route, asset, 'generated-asset'); }
    catch (error) {
      if (error.code !== 'ENOENT') throw error;
    }
  }
}

const selected = [...records.values()].sort((a, b) => a.path.localeCompare(b.path));
for (const item of selected) {
  const source = path.join(repositoryRoot, item.source);
  const target = path.join(stageRoot, item.path);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target, fs.constants.COPYFILE_EXCL);
  assert.equal(sha256(fs.readFileSync(target)), item.sha256, `staged hash mismatch: ${item.path}`);
}
const report = {
  sourceHead: sourceManifest.sourceHead,
  sourceTree: sourceManifest.sourceTree,
  siteRoot: path.relative(repositoryRoot, siteRoot).split(path.sep).join('/'),
  stageRoot: path.relative(repositoryRoot, stageRoot).split(path.sep).join('/'),
  generatedPageCount: generated.length,
  routeCount: selected.length,
  totalBytes: selected.reduce((sum, item) => sum + item.bytes, 0),
  noDeletes: true,
  routes: selected,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
