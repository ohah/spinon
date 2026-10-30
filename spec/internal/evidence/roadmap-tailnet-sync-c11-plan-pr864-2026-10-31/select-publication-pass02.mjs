import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = 'cf217d8bab6ea4b5f27e43f4be48d62c8b8272ca';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass02');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publicationStaging = path.join(base, 'publication-staging-pass02');
const finalPublication = path.join(base, 'publication-pass02');
const rootManifestPath = path.join(evidence, 'selected-root-pass02.json');
const reportPath = path.join(evidence, 'selected-publication-pass02.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
assert.equal(rootManifest.sourceHead, head);
assert.equal(rootManifest.sourceTree, git(['rev-parse', `${head}^{tree}`]).toString().trim());
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe publication input: ${directory}`);
}
for (const candidate of [publicationStaging, finalPublication, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 publication을 덮어쓰지 않기 위해 중단: ${candidate}`);
}
fs.mkdirSync(publicationStaging, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}
function copyRoute(route, sourcePath, label) {
  safeRoute(route);
  const bytes = fs.readFileSync(sourcePath);
  const digest = hash(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `publication route collision: ${route}`);
    return;
  }
  const destination = path.join(publicationStaging, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(hash(fs.readFileSync(destination)), digest, `publication copy hash mismatch: ${route}`);
  selected.set(route, { source: label, bytes: bytes.length, sha256: digest });
}
function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in generated output: ${child}`);
    if (entry.isDirectory()) walk(directory, child);
    else if (entry.isFile()) {
      if (child !== 'index.html' && child !== '404.html') copyRoute(child.split(path.sep).join('/'), absolute, `rspress:${child}`);
    } else throw new Error(`unsupported output entry: ${child}`);
  }
}
walk(site);
for (const entry of rootManifest.staged) {
  if (entry.source === 'generated:index.md') continue;
  const sourcePath = path.join(root, entry.route);
  assert.equal(hash(fs.readFileSync(sourcePath)), entry.sha256, `staged source changed: ${entry.route}`);
  copyRoute(entry.route, sourcePath, `staged:${entry.source}`);
}

const rawRoadmap = path.join(root, 'public/roadmap.html');
assert.deepEqual(fs.readFileSync(rawRoadmap), git(['show', `${head}:spec/public/roadmap.html`]), 'roadmap is not the merged main source');
for (const route of ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']) {
  copyRoute(route, rawRoadmap, 'source:spec/public/roadmap.html');
}
for (const [name, source] of [['icon.svg', 'spec/public/icon.svg'], ['logo.svg', 'spec/public/logo.svg'], ['roadmap.svg', 'spec/public/roadmap.svg']]) {
  const sourcePath = path.join(root, source);
  for (const route of [name, `public/${name}`, `spec/public/${name}`]) {
    copyRoute(route, sourcePath, `source:${source}`);
  }
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-css-grid.html',
  'spec/plan/c11-css-grid.html',
  'plan/c11-4-grid-track-sizing.html',
  'spec/plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-auto-repeat-pattern-ceiling.html',
  'plan/c11-4-direct-text-aspect-ratio-finite.html',
  'plan/c11-4-direct-text-aspect-ratio-auto.html',
  'plan/c11-4-direct-text-aspect-ratio-min-max.html',
  'plan/c11-4-anonymous-grid-text-nowrap.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-auto-repeat-pattern-ceiling-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-auto-repeat-pattern-ceiling-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-auto-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/README.html',
  'internal/evidence/c11-roadmap-sync-pr864-2026-10-31/pr-diff-review-30.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);

const roadmap = fs.readFileSync(path.join(publicationStaging, 'roadmap.html'), 'utf8');
for (const marker of ['#861 Rebase and merge', '9b7c3a357d94db0e47f4397b73d3376a35de4439', '72/213']) assert.ok(roadmap.includes(marker), 'roadmap marker missing: '+marker);
const status = fs.readFileSync(path.join(publicationStaging, 'STATUS.html'), 'utf8');
for (const marker of ['pull/861', '72/213', 'C11.4']) assert.ok(status.includes(marker), 'STATUS marker missing: '+marker);
const gridPlan = fs.readFileSync(path.join(publicationStaging, 'plan/c11-css-grid.html'), 'utf8');
for (const marker of ['PR #837', 'PR #850', 'PR #861', '72/213']) assert.ok(gridPlan.includes(marker), 'C11 plan marker missing: '+marker);
const trackPlan = fs.readFileSync(path.join(publicationStaging, 'plan/c11-4-grid-track-sizing.html'), 'utf8');
for (const marker of ['PR #850', 'PR #861', '20×10px', '50×10px', '72/213']) assert.ok(trackPlan.includes(marker), 'C11.4 plan marker missing: '+marker);
const routes = [...selected].sort(([left], [right]) => left.localeCompare(right))
  .map(([route, item]) => ({ path: route, ...item }));
const report = {
  sourceHead: head,
  sourceTree: rootManifest.sourceTree,
  publicationRoot: path.relative(repo, finalPublication),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  roadmapMarkers: ['#861 Rebase and merge','9b7c3a357d94db0e47f4397b73d3376a35de4439','72/213'],
  requiredRoutes,
  fullDocsLinkValidation: false,
  routes,
};
fs.renameSync(publicationStaging, finalPublication);
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
