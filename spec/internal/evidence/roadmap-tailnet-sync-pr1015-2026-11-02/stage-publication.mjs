import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const build = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1015-20261102');
const selectedRoot = path.join(build, 'root');
const site = path.join(build, 'site');
const previewRoot = path.join(process.env.HOME, 'Library/Application Support/SpinonPreview');
const manifestPath = path.join(evidence, 'route-publication-manifest.json');
const selected = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();

function walk(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const route = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${route}`);
    if (entry.isDirectory()) result.push(...walk(directory, route));
    else if (entry.isFile()) result.push(route.split(path.sep).join('/'));
    else throw new Error(`unsupported entry: ${route}`);
  }
  return result;
}
function inspectOptional(root, route) {
  assert.ok(route && !route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  let current = root;
  const parts = route.split('/');
  for (const [index, part] of parts.entries()) {
    assert.ok(part && part !== '.' && part !== '..', `invalid route part: ${route}`);
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink rejected: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `parent is not a directory: ${route}`);
      else {
        assert.ok(stat.isFile(), `not a regular file: ${route}`);
        return { exists: true, bytes: fs.readFileSync(current) };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function put(staged, route, bytes, source) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid route part: ${route}`);
  const digest = sha256(bytes);
  const previous = staged.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route source collision: ${route}`);
    return;
  }
  const target = path.join(build, 'route-publication', route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `staging hash mismatch: ${route}`);
  staged.set(route, { path: route, bytes: bytes.length, sha256: digest, source });
}

assert.equal(git(['rev-parse', 'HEAD']), git(['rev-parse', 'origin/main']), 'publication source must be merged origin/main');
assert.equal(selected.sourceHead, git(['rev-parse', 'HEAD']), 'selected root has a different source HEAD');
assert.ok(!fs.existsSync(path.join(build, 'route-publication')), 'preserving existing route publication');
fs.mkdirSync(path.join(build, 'route-publication'));
const staged = new Map();
for (const route of walk(site)) {
  if (route === '404.html' || route === 'index.html') continue;
  if (route.endsWith('.html') || route.startsWith('static/') || ['icon.svg', 'logo.svg'].includes(route)) {
    put(staged, route, fs.readFileSync(path.join(site, route)), `rspress:${route}`);
  }
}
for (const item of selected.copies) {
  const bytes = fs.readFileSync(path.join(selectedRoot, item.route));
  assert.equal(bytes.length, item.bytes, `selected source size mismatch: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected source hash mismatch: ${item.route}`);
  put(staged, item.route, bytes, `selected-root:${item.source}`);
}

const requiredRoutes = [
  'roadmap.html', 'STATUS.md', 'STATUS.html', 'spec/STATUS.md', 'spec/STATUS.html',
  'plan/c11-css-grid.html', 'plan/c11-root-html-grid-fr-track.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-root-html-grid-fr-track-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-fr-track-runtime-2026-11-02/android/simulator.png',
  'internal/evidence/c11-root-html-grid-fr-track-runtime-2026-11-02/ios/simulator.png',
];
for (const route of requiredRoutes) assert.ok(staged.has(route), `required route missing from publication: ${route}`);
const roadmap = fs.readFileSync(path.join(build, 'route-publication', 'roadmap.html'), 'utf8');
const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">([\s\S]*?)<\/p>/g)]
  .map((match) => match[1]).find((html) => html.includes('PR #1015 Rebase and merge'));
assert.ok(card, 'PR #1015 roadmap progress card missing');
for (const marker of ['2026-11-02T00:39:29Z', '015ff0e9ac959a269b60728a1ac3a6ab962791ea', '72/213']) {
  assert.ok(card.includes(marker), `roadmap marker missing: ${marker}`);
}
const checkedRoadmapLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.ok(url.pathname.startsWith('/spinon/'), `unexpected local route base: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
  if (!staged.has(route)) assert.ok(inspectOptional(previewRoot, route).exists, `roadmap link target absent: ${href}`);
  checkedRoadmapLinks.push({ href, route, staged: staged.has(route) });
}
assert.ok(checkedRoadmapLinks.length >= 5, 'expected roadmap links to be checked');

const routes = [...staged.values()].sort((a, b) => a.path.localeCompare(b.path)).map((entry) => {
  const current = inspectOptional(previewRoot, entry.path);
  return {
    ...entry,
    previousSha256: current.exists ? sha256(current.bytes) : null,
    previousBytes: current.exists ? current.bytes.length : null,
  };
});
const publication = {
  sourceHead: selected.sourceHead,
  sourceTree: selected.sourceTree,
  selectedRoot: selected.selectedRoot,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, entry) => sum + entry.bytes, 0),
  changedRouteCount: routes.filter((entry) => entry.sha256 !== entry.previousSha256).length,
  changedExistingRouteCount: routes.filter((entry) => entry.previousSha256 && entry.sha256 !== entry.previousSha256).length,
  newRouteCount: routes.filter((entry) => entry.previousSha256 === null).length,
  unchangedRouteCount: routes.filter((entry) => entry.sha256 === entry.previousSha256).length,
  noDeletes: true,
  requiredRoutes,
  checkedRoadmapLinks,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(publication, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...publication, routes: undefined, checkedRoadmapLinks: undefined }, null, 2)}\n`);
