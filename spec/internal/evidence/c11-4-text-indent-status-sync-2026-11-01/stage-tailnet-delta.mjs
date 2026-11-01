import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousEvidence = path.join(repo, 'spec/internal/evidence/c11-4-line-break-loose-public-roadmap-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication.json'), 'utf8'));
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root.json'), 'utf8'));
const site = path.join(repo, 'build/spinon/c11-4-text-indent-status-sync-2026-11-01-pass02/site');
const root = path.join(repo, rootManifest.selectedRoot);
const report = path.join(evidence, 'status-route-delta-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `source symlink 거부: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(directory, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`지원하지 않는 source entry: ${relative}`);
  }
  return files;
}

assert.ok(!fs.existsSync(report), '기존 delta report를 덮어쓰지 않음');
assert.equal(rootManifest.previousSelectedRoot, 'build/spinon/c11-4-loose-and-text-indent-flags-roadmap-pr964-pass01/root');
const old = new Map(previousManifest.routes.map((route) => [route.path, route]));
const sources = listFiles(site).filter((route) => route !== 'index.html' && route !== '404.html').map((route) => ({ route, source: path.join(site, route) }));
for (const route of ['STATUS.md', 'spec/STATUS.md']) sources.push({ route, source: path.join(root, route) });
const seen = new Set();
const routes = sources.map(({ route, source }) => {
  assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `잘못된 route: ${route}`);
  assert.ok(!seen.has(route), `중복 route: ${route}`);
  seen.add(route);
  const bytes = fs.readFileSync(source);
  const previous = old.get(route);
  return {
    route,
    source: path.relative(repo, source).split(path.sep).join('/'),
    bytes: bytes.length,
    sha256: sha256(bytes),
    previousSha256: previous?.sha256 ?? null,
    previousBytes: previous?.bytes ?? null,
    expectedAction: !previous ? 'create' : previous.sha256 === sha256(bytes) ? 'same' : 'replace',
  };
});
const changed = routes.filter((route) => route.expectedAction !== 'same');
assert.ok(routes.some((route) => route.route === 'STATUS.html'), 'STATUS.html page missing');
assert.ok(routes.some((route) => route.route === 'spec/STATUS.html'), 'spec/STATUS.html page missing');
const statusSource = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
for (const marker of ['[#959]', '590f5946e97e0252113abf34b23ae31b13819acf', '72/213']) assert.ok(statusSource.includes(marker), `selected STATUS source missing marker: ${marker}`);
const delta = {
  sourceHead: rootManifest.sourceHead,
  statusSha256: rootManifest.sourceStatusWorkingTreeSha256,
  previousPublicationHead: previousManifest.sourceHead,
  previousRouteCount: previousManifest.routeCount,
  generatedRouteCount: sources.length - 2,
  updateRouteCount: routes.length,
  createCount: routes.filter((route) => route.expectedAction === 'create').length,
  replaceCount: routes.filter((route) => route.expectedAction === 'replace').length,
  sameCount: routes.filter((route) => route.expectedAction === 'same').length,
  noDeletes: true,
  changedRoutes: changed,
  routes,
};
fs.writeFileSync(report, `${JSON.stringify(delta, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...delta, routes: undefined }, null, 2)}\n`);
