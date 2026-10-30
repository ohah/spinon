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
const rootManifestPath = path.join(evidence, 'selected-root-pass01.json');
const reportPath = path.join(evidence, 'selected-publication-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 32 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
assert.equal(rootManifest.sourceHead, head);
assert.equal(rootManifest.sourceTree, git(['rev-parse', head + '^{tree}']).toString().trim());
assert.ok(fs.lstatSync(root).isDirectory() && !fs.lstatSync(root).isSymbolicLink());
assert.ok(fs.lstatSync(site).isDirectory() && !fs.lstatSync(site).isSymbolicLink());
for (const candidate of [publication, finalPublication, reportPath]) {
  assert.ok(!fs.existsSync(candidate), 'refusing to reuse publication output: ' + candidate);
}

fs.mkdirSync(publication, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copyRoute(route, sourcePath, label) {
  safeRoute(route);
  const stat = fs.lstatSync(sourcePath);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe publication source: ' + sourcePath);
  assert.ok(!selected.has(route), 'duplicate route: ' + route);
  const bytes = fs.readFileSync(sourcePath);
  const destination = path.join(publication, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), 'publication copy hash mismatch: ' + route);
  selected.set(route, { source: label, bytes: copied.length, sha256: sha256(copied) });
}
function walk(directory, relative = '') {
  const entries = fs.readdirSync(path.join(directory, relative), { withFileTypes: true })
    .sort((a, b) => a.name.localeCompare(b.name));
  for (const entry of entries) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), 'symlink in generated output: ' + child);
    if (entry.isDirectory()) walk(directory, child);
    else if (entry.isFile()) {
      if (child !== 'index.html' && child !== '404.html') {
        copyRoute(child.split(path.sep).join('/'), absolute, 'rspress:' + child);
      }
    } else throw new Error('unsupported output entry: ' + child);
  }
}
walk(site);
for (const entry of rootManifest.staged) {
  if (entry.source === 'generated:index.md') continue;
  const sourcePath = path.join(root, entry.route);
  assert.equal(sha256(fs.readFileSync(sourcePath)), entry.sha256, 'staged source changed: ' + entry.route);
  if (selected.has(entry.route)) {
    assert.equal(sha256(fs.readFileSync(path.join(publication, entry.route))), entry.sha256,
      'generated route differs from selected source: ' + entry.route);
  } else copyRoute(entry.route, sourcePath, 'staged:' + entry.source);
}

const rawRoadmap = path.join(repo, 'spec/public/roadmap.html');
assert.deepEqual(fs.readFileSync(rawRoadmap), git(['show', head + ':spec/public/roadmap.html']),
  'roadmap source is not merged main');
for (const route of ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']) {
  if (selected.has(route)) {
    assert.deepEqual(fs.readFileSync(path.join(publication, route)), fs.readFileSync(rawRoadmap),
      'raw roadmap route differs: ' + route);
  } else copyRoute(route, rawRoadmap, 'source:spec/public/roadmap.html');
}
for (const [name, source] of [
  ['icon.svg', 'spec/public/icon.svg'],
  ['logo.svg', 'spec/public/logo.svg'],
  ['roadmap.svg', 'spec/public/roadmap.svg'],
]) {
  for (const route of [name, 'public/' + name, 'spec/public/' + name]) {
    if (!selected.has(route)) copyRoute(route, path.join(repo, source), 'source:' + source);
  }
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-grid-text-last-baseline-multiline.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/pr-diff-review-30.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/post-merge-status-roadmap-review-30.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/artifact-boundary/README.html',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/android-avd/c114-text-baseline-mvu9nh3u-ddfba41b/grid-text-last-baseline-align-items-multiline.png',
  'internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30/ios-simulator/c114-text-baseline-mvu9pccg-f329c361/grid-text-last-baseline-align-items-multiline.png',
  'internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/chrome/chrome156-grid-last-baseline.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), 'required route missing: ' + route);

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
for (const marker of ['PR #830', '926bc4839aa72c8859ec0c5dfddf74675b7dfabb', '72/213']) {
  assert.ok(roadmap.includes(marker), 'roadmap marker missing: ' + marker);
}
const statusHtml = fs.readFileSync(path.join(publication, 'STATUS.html'), 'utf8');
for (const marker of ['pull/830', '926bc4839aa72c8859ec0c5dfddf74675b7dfabb', '72/213']) {
  assert.ok(statusHtml.includes(marker), 'rendered STATUS marker missing: ' + marker);
}
const statusSource = git(['show', head + ':spec/STATUS.md']).toString();
assert.match(statusSource, /- \[ \] \*\*C11 CSS Grid/);
assert.match(statusSource, /- \[ \] \*\*C11\.4 intrinsic track sizing/);

const screenshotRoutes = requiredRoutes.filter((route) => route.endsWith('.png'));
for (const route of screenshotRoutes) {
  assert.deepEqual(fs.readFileSync(path.join(publication, route)).subarray(0, 8),
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), 'invalid PNG: ' + route);
}
const routes = [...selected].sort(([a], [b]) => a.localeCompare(b))
  .map(([route, item]) => ({ path: route, ...item }));
const report = {
  sourceHead: head,
  sourceTree: rootManifest.sourceTree,
  publicationRoot: path.relative(repo, finalPublication),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  roadmapMarkers: ['PR #830', '926bc4839aa72c8859ec0c5dfddf74675b7dfabb', '72/213'],
  statusMarkers: ['pull/830', '926bc4839aa72c8859ec0c5dfddf74675b7dfabb', '72/213'],
  screenshotRoutes,
  fullDocsLinkValidation: false,
  routes,
};
fs.renameSync(publication, finalPublication);
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, routes: undefined }, null, 2) + '\n');
