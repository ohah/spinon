import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr718-root-selected');
const siteRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr718-site-selected');
const publicationRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr718-publication-selected');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root.json');
const reportPath = path.join(evidenceDirectory, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
assert.ok(fs.lstatSync(selectedRoot).isDirectory() && !fs.lstatSync(selectedRoot).isSymbolicLink());
assert.ok(fs.lstatSync(siteRoot).isDirectory() && !fs.lstatSync(siteRoot).isSymbolicLink(), 'Rspress output is missing');
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);
if (fs.existsSync(publicationRoot)) {
  const stat = fs.lstatSync(publicationRoot);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe existing publication: ${publicationRoot}`);
} else fs.mkdirSync(publicationRoot, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}
function writeRoute(route, bytes, source, mode = 0o644) {
  safeRoute(route);
  const destination = path.join(publicationRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const stat = fs.lstatSync(destination);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe overlay target: ${route}`);
    fs.unlinkSync(destination);
  }
  fs.writeFileSync(destination, bytes, { flag: 'wx', mode });
  const copied = fs.readFileSync(destination);
  assert.deepEqual(copied, bytes, `publication bytes differ: ${route}`);
  selected.set(route, { path: route, source, bytes: copied.length, sha256: sha256(copied), mode });
}
function walk(root, relative = '') {
  for (const entry of fs.readdirSync(path.join(root, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(root, child);
    assert.ok(!entry.isSymbolicLink(), `symlink in build output: ${child}`);
    if (entry.isDirectory()) walk(root, child);
    else if (entry.isFile()) {
      if (child !== 'index.html' && child !== '404.html') writeRoute(child, fs.readFileSync(absolute), `rspress:${child}`, fs.statSync(absolute).mode & 0o777);
    } else throw new Error(`unsupported build entry: ${child}`);
  }
}

walk(siteRoot);
for (const staged of rootManifest.staged) {
  const bytes = fs.readFileSync(path.join(selectedRoot, staged.path));
  assert.equal(bytes.length, staged.bytes, `staged size changed: ${staged.path}`);
  assert.equal(sha256(bytes), staged.sha256, `staged hash changed: ${staged.path}`);
  writeRoute(staged.path, bytes, `main:${staged.source}`);
}

const requiredRoutes = [
  'roadmap.html', 'STATUS.md', 'STATUS.html', 'spec/STATUS.md', 'spec/STATUS.html',
  'plan/c11-4-flex-stage-oracle.html', 'plan/c11-4-grid-track-sizing.html',
  'internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/README.html',
  'internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/chrome-flex-stage.png',
  'internal/evidence/c11-4-flex-stage-oracle-implementation-2026-10-28/README.html',
  'internal/evidence/c11-4-flex-stage-oracle-implementation-2026-10-28/implementation-review-20.html',
  'internal/evidence/c11-4-flex-stage-oracle-implementation-2026-10-28/pr-diff-review-30.html',
  'internal/evidence/c11-4-flex-stage-oracle-implementation-2026-10-28/post-merge-status-roadmap-review-30.html',
  'crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs',
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required route missing: ${route}`);

const roadmap = fs.readFileSync(path.join(publicationRoot, 'roadmap.html'), 'utf8');
for (const marker of ['PR #717 Rebase and merge', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213', 'C11.4']) {
  assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
}
const latestIndex = roadmap.indexOf('PR #717 Rebase and merge');
assert.ok(latestIndex >= 0, 'PR #717 roadmap note is missing');
const latestStart = roadmap.lastIndexOf('<p class="kanban-note', latestIndex);
const latestEnd = roadmap.indexOf('</p>', latestIndex);
assert.ok(latestStart >= 0 && latestEnd >= 0, 'could not isolate the PR #717 roadmap paragraph');
const latestRoadmapRow = roadmap.slice(latestStart, latestEnd + 4);
const roadmapLinks = [...latestRoadmapRow.matchAll(/href="\/spinon\/([^"]+)"/g)].map((match) => match[1]);
const missingRoadmapLinks = roadmapLinks.filter((route) => !selected.has(route));
assert.deepEqual(missingRoadmapLinks, [], `new roadmap links missing from publication: ${missingRoadmapLinks.join(', ')}`);

const status = fs.readFileSync(path.join(publicationRoot, 'STATUS.html'), 'utf8');
for (const marker of ['github.com/ohah/spinon/pull/717', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213']) assert.ok(status.includes(marker), `STATUS marker missing: ${marker}`);
const focusedPlan = fs.readFileSync(path.join(publicationRoot, 'plan/c11-4-flex-stage-oracle.html'), 'utf8');
for (const marker of ['github.com/ohah/spinon/pull/717', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213']) assert.ok(focusedPlan.includes(marker), `focused plan marker missing: ${marker}`);

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  requiredRoutes,
  roadmapMarkers: ['PR #717 Rebase and merge', 'c29482aa8fc3ac39cdc342eae138c3ad10a59683', '72/213', 'C11.4'],
  roadmapLocalLinksChecked: roadmapLinks,
  fullDocsLinkValidation: false,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
