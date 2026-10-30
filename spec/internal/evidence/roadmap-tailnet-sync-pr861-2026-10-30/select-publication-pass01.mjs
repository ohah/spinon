import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = '9450dca8754f7bf63ae271c86e40d5f815c35d2c';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr861-2026-10-30/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publicationStaging = path.join(base, 'publication-staging-pass01');
const finalPublication = path.join(base, 'publication-pass01');
const rootManifestPath = path.join(evidence, 'selected-root-pass01.json');
const reportPath = path.join(evidence, 'selected-publication-pass01.json');
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
  'public/roadmap.html',
  'spec/public/roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-anonymous-grid-text-nowrap.html',
  'spec/plan/c11-4-anonymous-grid-text-nowrap.html',
  'plan/c11-4-anonymous-grid-text-nowrap-plan-review-20.html',
  'spec/plan/c11-4-anonymous-grid-text-nowrap-plan-review-20.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/plan-pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/chrome156-anonymous-text-nowrap.png',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/README.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/pr-diff-review-30.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/artifact-boundary/README.html',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/anonymous-nowrap-min-content.png',
  'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/anonymous-nowrap-min-content.png',
  'internal/evidence/roadmap-tailnet-sync-pr861-2026-10-30/roadmap-pr-diff-review-30.html',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);

const roadmap = fs.readFileSync(path.join(publicationStaging, 'roadmap.html'), 'utf8');
for (const marker of ['#861 Rebase and merge', '9b7c3a357d94db0e47f4397b73d3376a35de4439', '72/213', 'C11.4/C11']) {
  assert.ok(roadmap.includes(marker), 'roadmap marker missing: ' + marker);
}
const status = fs.readFileSync(path.join(publicationStaging, 'STATUS.html'), 'utf8');
for (const marker of ['pull/861', '72/213', 'C11.4']) {
  assert.ok(status.includes(marker), 'rendered STATUS marker missing: ' + marker);
}
const runtime = fs.readFileSync(path.join(publicationStaging, 'internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/README.html'), 'utf8');
for (const marker of ['20×10px', '50×10px', 'draw/present', 'Android', 'iOS']) {
  assert.ok(runtime.includes(marker), 'runtime evidence marker missing: ' + marker);
}
const routes = [...selected].sort(([left], [right]) => left.localeCompare(right))
  .map(([route, item]) => ({ path: route, ...item }));
const report = {
  sourceHead: head,
  sourceTree: rootManifest.sourceTree,
  publicationRoot: path.relative(repo, finalPublication),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  roadmapMarkers: ['#861 Rebase and merge', '9b7c3a357d94db0e47f4397b73d3376a35de4439', '72/213'],
  requiredRoutes,
  fullDocsLinkValidation: false,
  routes,
};
fs.renameSync(publicationStaging, finalPublication);
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
