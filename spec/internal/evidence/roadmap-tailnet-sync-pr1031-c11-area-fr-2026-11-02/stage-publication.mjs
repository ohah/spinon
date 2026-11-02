import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const selected = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const selectedRoot = path.resolve(repo, selected.selectedRoot);
const buildRoot = path.dirname(selectedRoot);
const site = path.join(buildRoot, 'site');
const publicationRoot = path.join(buildRoot, 'route-publication');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const manifestPath = path.join(evidence, 'route-publication-manifest.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
}
function walk(directory, prefix = '') {
  const routes = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const route = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${route}`);
    if (entry.isDirectory()) routes.push(...walk(directory, route));
    else if (entry.isFile()) routes.push(route.split(path.sep).join('/'));
    else throw new Error(`unsupported route entry: ${route}`);
  }
  return routes;
}
function inspectOptional(root, route) {
  assert.ok(route && !route.startsWith('/') && !route.includes('\\'), `invalid route: ${route}`);
  let current = root;
  const parts = route.split('/');
  for (const [index, part] of parts.entries()) {
    assert.ok(part && part !== '.' && part !== '..', `invalid route segment: ${route}`);
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink rejected: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent is not a directory: ${route}`);
      else {
        assert.ok(stat.isFile(), `target is not a file: ${route}`);
        return { exists: true, bytes: fs.readFileSync(current), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}
function inspectServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend must be Running');
  assertDirectory(previewRoot);
  assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root changed');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `expected one preview listener: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `unexpected preview process: ${args}`);
  return { backendState: tailscale.BackendState, dnsName: tailscale.Self?.DNSName, serverPid };
}
function put(route, bytes, source) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `invalid publication route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `invalid publication route: ${route}`);
  const digest = sha256(bytes);
  const old = staged.get(route);
  if (old) {
    assert.equal(old.sha256, digest, `publication route collision: ${route}`);
    return;
  }
  const target = path.join(publicationRoot, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `staging hash mismatch: ${route}`);
  const previous = inspectOptional(previewRoot, route);
  staged.set(route, {
    path: route,
    bytes: bytes.length,
    sha256: digest,
    source,
    previousSha256: previous.exists ? sha256(previous.bytes) : null,
    previousBytes: previous.exists ? previous.bytes.length : null,
  });
}

assert.equal(git(['branch', '--show-current']), 'main');
assert.equal(git(['rev-parse', 'HEAD']), git(['rev-parse', 'origin/main']), 'source must be merged origin/main');
assert.equal(git(['status', '--porcelain', '--untracked-files=no']), '', 'tracked working tree must be clean');
assert.equal(selected.sourceHead, git(['rev-parse', 'HEAD']));
assertDirectory(selectedRoot);
assertDirectory(site);
assertDirectory(previewRoot);
assert.ok(!fs.existsSync(publicationRoot) && !fs.existsSync(manifestPath), 'preserving existing publication output');
const server = inspectServer();
const staged = new Map();
fs.mkdirSync(publicationRoot, { recursive: false });

for (const route of walk(site)) {
  if (route === '404.html' || route === 'index.html') continue;
  put(route, fs.readFileSync(path.join(site, route)), `rspress:${route}`);
}
for (const item of selected.copies) {
  const bytes = fs.readFileSync(path.join(selectedRoot, item.route));
  assert.equal(bytes.length, item.bytes, `selected source size mismatch: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected source hash mismatch: ${item.route}`);
  put(item.route, bytes, `selected-root:${item.source}`);
}
const probe = 'spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/root-app-runtime-probe.js';
put(
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/root-app-runtime-probe.js',
  fs.readFileSync(path.join(repo, probe)),
  `raw-source:${probe}`,
);

const requiredRoutes = [
  'roadmap.html', 'STATUS.md', 'STATUS.html', 'spec/STATUS.md', 'spec/STATUS.html',
  'plan/c11-css-grid.html', 'plan/c11-root-html-grid-implicit-rows.html',
  'plan/c11-root-html-grid-fr-percent-implicit-rows.html',
  'plan/c11-root-html-grid-fr-named-areas.html',
  'spec/plan/c11-root-html-grid-implicit-rows.html',
  'spec/plan/c11-root-html-grid-fr-percent-implicit-rows.html',
  'spec/plan/c11-root-html-grid-fr-named-areas.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-root-html-grid-implicit-rows-precomparison-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/android-api37-avd.png',
  'internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/ios-26_2-simulator.png',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/roadmap-postmerge-docs-pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/android-api37.png',
  'internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/ios-26_2-simulator.png',
  'internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/chrome-agent-browser.png',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/roadmap-postmerge-docs-pr-diff-review-30.html',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/android-avd/c11-root-html-grid-fr-named-areas.png',
  'internal/evidence/c11-root-html-grid-fr-named-areas-runtime-2026-11-02/ios-simulator/c11-root-html-grid-fr-named-areas.png',
];
for (const route of requiredRoutes) assert.ok(staged.has(route), `required route not staged: ${route}`);

const roadmap = fs.readFileSync(path.join(publicationRoot, 'roadmap.html'), 'utf8');
const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">([\s\S]*?)<\/p>/g)]
  .map((match) => match[1]).find((html) => html.includes('PR #1030 Rebase and merge'));
assert.ok(card, 'merged PR #1030 roadmap card missing');
for (const marker of ['579435b1955e7ad157baa13d100d75f13a5dd283', '72/213', 'C11.1/C11']) {
  assert.ok(card.includes(marker), `roadmap marker missing: ${marker}`);
}
const checkedLinks = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.ok(url.pathname.startsWith('/spinon/'), `unexpected local base path: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
  assert.ok(staged.has(route) || inspectOptional(previewRoot, route).exists, `roadmap route absent from publication and preview: ${href}`);
  checkedLinks.push({ href, route, staged: staged.has(route) });
}
const routes = [...staged.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead: selected.sourceHead,
  sourceTree: selected.sourceTree,
  selectedRoot: selected.selectedRoot,
  publicationRoot: path.relative(repo, publicationRoot).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, entry) => sum + entry.bytes, 0),
  changedRouteCount: routes.filter((entry) => entry.previousSha256 !== entry.sha256).length,
  newRouteCount: routes.filter((entry) => entry.previousSha256 === null).length,
  unchangedRouteCount: routes.filter((entry) => entry.previousSha256 === entry.sha256).length,
  noDeletes: true,
  fullDocsDeadLinkCheck: false,
  server,
  requiredRoutes,
  checkedRoadmapLinks: checkedLinks,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined, checkedRoadmapLinks: undefined }, null, 2)}\n`);
