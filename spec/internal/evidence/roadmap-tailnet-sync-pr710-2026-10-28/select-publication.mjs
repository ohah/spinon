import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-root');
const siteRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-site');
const publicationRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-publication');
const rootManifestPath = path.join(evidenceDirectory, 'selected-root.json');
const reportPath = path.join(evidenceDirectory, 'selected-publication.json');
const evidencePrefix = 'spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28/';
const planBase = 'c11-4-generic-mixed-direct-text-image-intrinsic';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [publicationRoot, reportPath]) assert.ok(!fs.existsSync(candidate), `기존 경로 보존을 위해 중단: ${candidate}`);
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(rootManifest.sourceHead, sourceHead, 'staged root HEAD가 현재 HEAD와 다름');
assert.equal(rootManifest.sourceTree, sourceTree, 'staged root tree가 현재 tree와 다름');
assert.ok(fs.lstatSync(siteRoot).isDirectory() && !fs.lstatSync(siteRoot).isSymbolicLink(), 'Rspress output이 없음');
fs.mkdirSync(publicationRoot, { recursive: false });
const selected = new Map();

function copyRoute(route, sourcePath, sourceLabel) {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 route: ${route}`);
  const bytes = fs.readFileSync(sourcePath);
  const destination = path.join(publicationRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), `publication hash 불일치: ${route}`);
  selected.set(route, { source: sourceLabel, bytes: copied.length, sha256: sha256(copied) });
}

function walk(root, relative = '') {
  for (const entry of fs.readdirSync(path.join(root, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(root, child);
    assert.ok(!entry.isSymbolicLink(), `Rspress output에 symlink가 있음: ${child}`);
    if (entry.isDirectory()) walk(root, child);
    else if (entry.isFile()) {
      if (child !== 'index.html' && child !== '404.html') copyRoute(child, absolute, `rspress:${child}`);
    } else throw new Error(`지원하지 않는 output entry: ${child}`);
  }
}

walk(siteRoot);
const rawEntries = rootManifest.staged.filter((entry) => (
  entry.source === 'spec/public/roadmap.html'
  || entry.source === 'spec/public/icon.svg'
  || entry.source === 'spec/public/logo.svg'
  || entry.source === 'spec/STATUS.md'
  || entry.source.startsWith('plan/')
  || entry.source.startsWith(evidencePrefix)
));
for (const entry of rawEntries) {
  const sourcePath = path.join(selectedRoot, entry.path);
  assert.equal(sha256(fs.readFileSync(sourcePath)), entry.sha256, `staged source hash changed: ${entry.path}`);
  copyRoute(entry.path, sourcePath, `staged:${entry.source}`);
}
copyRoute('logo.svg', path.join(selectedRoot, 'public/logo.svg'), 'staged:spec/public/logo.svg');

const evidenceRouteBase = `internal/${evidencePrefix.slice('spec/internal/'.length)}`;
const generatedRoutes = [
  'roadmap.html',
  'STATUS.html', 'spec/STATUS.html',
  `plan/${planBase}.html`, `spec/plan/${planBase}.html`,
  `plan/${planBase}-plan-review-20.html`, `spec/plan/${planBase}-plan-review-20.html`,
  `plan/${planBase}-plan-pr-diff-review-20.html`, `spec/plan/${planBase}-plan-pr-diff-review-20.html`,
  `${evidenceRouteBase}README.html`, `spec/${evidenceRouteBase}README.html`,
  `${evidenceRouteBase}implementation-runtime-review-20.html`, `spec/${evidenceRouteBase}implementation-runtime-review-20.html`,
  `${evidenceRouteBase}pr-diff-review-30.html`, `spec/${evidenceRouteBase}pr-diff-review-30.html`,
];
for (const route of generatedRoutes) assert.ok(selected.has(route), `generated/selected page missing: ${route}`);

const requiredRoutes = [
  'roadmap.html', 'STATUS.html', `plan/${planBase}.html`, `${evidenceRouteBase}README.html`,
  `${evidenceRouteBase}implementation-runtime-review-20.html`, `${evidenceRouteBase}pr-diff-review-30.html`,
  `${evidenceRouteBase}runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/android-simulator.json`,
  `${evidenceRouteBase}runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/android-direct-min-content-generic-forward.png`,
  `${evidenceRouteBase}runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/ios-simulator.json`,
  `${evidenceRouteBase}runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/ios-direct-min-content-generic-forward.png`,
];
for (const route of requiredRoutes) assert.ok(selected.has(route), `required publication route missing: ${route}`);

const roadmap = fs.readFileSync(path.join(publicationRoot, 'roadmap.html'), 'utf8');
for (const marker of ['72/213', 'PR #710', '58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94', 'generic mixed direct text/image intrinsic']) {
  assert.ok(roadmap.includes(marker), `roadmap marker missing: ${marker}`);
}
const statusHtml = fs.readFileSync(path.join(publicationRoot, 'STATUS.html'), 'utf8');
for (const marker of ['72/213', 'C11.4', '58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94']) {
  assert.ok(statusHtml.includes(marker), `STATUS marker missing: ${marker}`);
}
const planHtml = fs.readFileSync(path.join(publicationRoot, `plan/${planBase}.html`), 'utf8');
for (const marker of ['PR #710', '58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94', '72/213']) {
  assert.ok(planHtml.includes(marker), `plan marker missing: ${marker}`);
}
for (const route of requiredRoutes.filter((entry) => entry.endsWith('.png'))) {
  assert.deepEqual(fs.readFileSync(path.join(publicationRoot, route)).subarray(0, 8), Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), `invalid PNG signature: ${route}`);
}

const routes = [...selected].sort(([a], [b]) => a.localeCompare(b)).map(([route, entry]) => ({ path: route, ...entry }));
const report = {
  sourceHead, sourceTree,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  siteRoot: path.relative(repositoryRoot, siteRoot),
  publicationRoot: path.relative(repositoryRoot, publicationRoot),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, entry) => sum + entry.bytes, 0),
  roadmapMarkers: ['72/213', 'PR #710', '58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94'],
  requiredRoutes,
  fullDocsLinkValidation: false,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
