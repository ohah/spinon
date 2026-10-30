import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/c11-4-finite-ratio-tailnet-sync-2026-10-30/pass02');
const root = path.join(buildRoot, 'root');
const site = path.join(buildRoot, 'site');
const publication = path.join(buildRoot, 'publication');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const publicationManifestPath = path.join(evidenceDirectory, 'selected-publication-pass02.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');

const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
assert.equal(rootManifest.sourceHead, '9a7d04bf21d832672c4c204fb9cceafc29cdb654');
assert.equal(rootManifest.fileCount, rootManifest.staged.length);
for (const directory of [root, site]) assert.ok(fs.lstatSync(directory).isDirectory() && !fs.lstatSync(directory).isSymbolicLink());
for (const candidate of [publication, publicationManifestPath]) assert.ok(!fs.existsSync(candidate), `refusing to overwrite ${candidate}`);
fs.mkdirSync(publication, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(typeof route === 'string' && route.length > 0);
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function add(route, bytes, source) {
  safeRoute(route);
  const digest = hash(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `publication route collision: ${route}`);
    return;
  }
  const destination = path.join(publication, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(hash(fs.readFileSync(destination)), digest, `publication copy mismatch: ${route}`);
  selected.set(route, { source, bytes: bytes.length, sha256: digest });
}
function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in generated output: ${child}`);
    if (entry.isDirectory()) walk(directory, child);
    else if (entry.isFile()) {
      const route = child.split(path.sep).join('/');
      if (route !== 'index.html' && route !== '404.html') add(route, fs.readFileSync(absolute), `rspress:${route}`);
    } else throw new Error(`unsupported output entry: ${child}`);
  }
}
walk(site);

for (const entry of rootManifest.staged) {
  if (entry.source === 'generated:index.md') continue;
  const bytes = fs.readFileSync(path.join(root, entry.route));
  assert.equal(bytes.length, entry.bytes, `staged source size changed: ${entry.route}`);
  assert.equal(hash(bytes), entry.sha256, `staged source hash changed: ${entry.route}`);
  add(entry.route, bytes, `source:${entry.source}`);
}

const rawRoadmap = fs.readFileSync(path.join(root, 'public/roadmap.html'));
for (const route of ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']) add(route, rawRoadmap, 'source:spec/public/roadmap.html');
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) {
  const bytes = fs.readFileSync(path.join(root, 'public', name));
  for (const route of [name, `public/${name}`, `spec/public/${name}`]) add(route, bytes, `source:spec/public/${name}`);
}

const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-direct-text-aspect-ratio-finite.html',
  'plan/c11-4-grid-track-sizing.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/finite-ratio-implementation-runtime-review-20.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/finite-ratio-pr-diff-review-30.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-range-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-finite-ratio-post-merge-docs-2026-10-30/post-merge-doc-diff-review-30.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvuhobpm-b96aae77/grid-text-aspect-ratio-range-column-1-2.png',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/ios-simulator/c114-text-ratio-mvuhr3w3-23d87e87/grid-text-aspect-ratio-range-column-1-2.png',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);

const roadmap = fs.readFileSync(path.join(publication, 'roadmap.html'), 'utf8');
for (const marker of ['#842', '78fd0b84e4132e1f91bd293735e97b33cbf0d9a0', '72/213', '49/49 exact', 'C11.4 direct-text finite-positive']) {
  assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
}
assert.ok(!roadmap.includes('아직 구현·runtime 실행은 없다.'), 'stale plan claim remains in roadmap');
const status = fs.readFileSync(path.join(publication, 'STATUS.html'), 'utf8');
for (const marker of ['pull/842', '78fd0b84e4132e1f91bd293735e97b33cbf0d9a0', '72/213']) assert.ok(status.includes(marker), `STATUS marker missing: ${marker}`);
const plan = fs.readFileSync(path.join(publication, 'plan/c11-4-direct-text-aspect-ratio-finite.html'), 'utf8');
for (const marker of ['pull/842', '78fd0b84e4132e1f91bd293735e97b33cbf0d9a0', 'C11.4']) assert.ok(plan.includes(marker), `plan marker missing: ${marker}`);

const files = [...selected].sort(([left], [right]) => left.localeCompare(right)).map(([route, value]) => ({ path: route, ...value }));
const manifest = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  routeCount: files.length,
  byteCount: files.reduce((sum, entry) => sum + entry.bytes, 0),
  requiredRoutes,
  files,
};
fs.writeFileSync(publicationManifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ sourceHead: manifest.sourceHead, routeCount: manifest.routeCount, byteCount: manifest.byteCount, requiredRouteCount: requiredRoutes.length }, null, 2) + '\n');
