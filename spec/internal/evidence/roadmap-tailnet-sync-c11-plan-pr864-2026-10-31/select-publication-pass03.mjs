import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = 'cf217d8bab6ea4b5f27e43f4be48d62c8b8272ca';
const fullPublication = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass02/publication-pass02');
const pass03 = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass03');
const staging = path.join(pass03, 'publication-staging-pass03');
const publication = path.join(pass03, 'publication-pass03');
const reportPath = path.join(evidence, 'selected-publication-pass03.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 64 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
assert.ok(fs.statSync(fullPublication).isDirectory());
for (const candidate of [pass03, staging, publication, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `existing data preserved: ${candidate}`);
}

const pageRoutes = [
  'plan/c11-css-grid.html',
  'spec/plan/c11-css-grid.html',
  'plan/c11-4-grid-track-sizing.html',
  'spec/plan/c11-4-grid-track-sizing.html',
  'internal/evidence/c11-roadmap-sync-pr864-2026-10-31/pr-diff-review-30.html',
  'spec/internal/evidence/c11-roadmap-sync-pr864-2026-10-31/pr-diff-review-30.html',
];
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copyRoute(route, reason) {
  safeRoute(route);
  if (selected.has(route)) return;
  const source = path.join(fullPublication, route);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe publication source: ${route}`);
  const bytes = fs.readFileSync(source);
  selected.set(route, { reason, bytes: bytes.length, sha256: hash(bytes) });
}

for (const route of pageRoutes) copyRoute(route, 'PR #864 updated page');
for (const route of pageRoutes) {
  const html = fs.readFileSync(path.join(fullPublication, route), 'utf8');
  for (const match of html.matchAll(/(?:src|href)="\/spinon\/([^"?#]+)(?:[?#][^"]*)?"/g)) {
    const resource = decodeURIComponent(match[1]);
    safeRoute(resource);
    if (!resource.endsWith('.html') && fs.existsSync(path.join(fullPublication, resource))) {
      copyRoute(resource, `referenced by ${route}`);
    }
  }
}
copyRoute('static/search_index.ko.ed3e171c.json', 'RSPress navigation search index');

fs.mkdirSync(pass03, { recursive: true });
fs.mkdirSync(staging, { recursive: false });
for (const [route, item] of selected) {
  const destination = path.join(staging, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(path.join(fullPublication, route), destination, fs.constants.COPYFILE_EXCL);
  assert.equal(hash(fs.readFileSync(destination)), item.sha256, `selected copy hash mismatch: ${route}`);
}
fs.renameSync(staging, publication);

const routes = [...selected].sort(([left], [right]) => left.localeCompare(right))
  .map(([route, item]) => ({ path: route, ...item }));
const report = {
  sourceHead: head,
  fullBuildSource: 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass02/publication-pass02',
  publicationRoot: path.relative(repo, publication),
  changedPages: pageRoutes,
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  fullDocsLinkValidation: false,
  routes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
