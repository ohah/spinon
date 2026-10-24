import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass02-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'pass02 Rspress build is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite pass02 verification report');

const rawAssets = [
  { route: 'roadmap.html', source: 'spec/public/roadmap.html' },
  { route: 'tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html', source: 'tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html' },
  { route: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json' },
  { route: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png' },
  { route: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py' },
  { route: 'internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json' },
  { route: 'internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png' },
  { route: 'internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py', source: 'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py' },
];

const stagedAssets = [];
for (const asset of rawAssets) {
  const source = path.join(repositoryRoot, asset.source);
  const destination = path.join(outputRoot, asset.route);
  const bytes = fs.readFileSync(source);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const temporary = `${destination}.spinon-pass02-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  try {
    fs.renameSync(temporary, destination);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `raw asset differs after staging: ${asset.route}`);
  stagedAssets.push({ path: asset.route, source: asset.source, bytes: bytes.length, sha256: sha256(bytes) });
}

const routes = [...new Set([
  'roadmap.html',
  'STATUS.html',
  'plan/c11-4-auto-repeat-calc-neighbors.html',
  'plan/c11-4-auto-repeat-negative-percentage-neighbors.html',
  'plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.html',
  'spec/plan/c11-4-auto-repeat-negative-percentage-neighbors.html',
  'spec/plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.html',
  'internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.html',
  'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.html',
  ...rawAssets.map((asset) => asset.route),
])];
assert.equal(routes.length, new Set(routes).size, 'selected routes must be unique');
const markers = new Map([
  ['roadmap.html', ['72/213', 'C11.4']],
  ['STATUS.html', ['2026-10-24 C11.4 음수 percentage auto-repeat calc neighbor 계획', '72/213', '4faabe77c6b3a9a87fa1e50503649b8fcdc803d7']],
  ['plan/c11-4-auto-repeat-calc-neighbors.html', ['Rebase and merge', '4faabe77c6b3a9a87fa1e50503649b8fcdc803d7']],
  ['plan/c11-4-auto-repeat-negative-percentage-neighbors.html', ['signed percentage', 'calc(20px - 5%)', '72/213']],
  ['plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.html', ['20개', 'layout-representability']],
  ['spec/plan/c11-4-auto-repeat-negative-percentage-neighbors.html', ['signed percentage', 'calc(20px - 5%)', '72/213']],
  ['spec/plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.html', ['20개', 'layout-representability']],
  ['internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.html', ['Chrome track CSSOM', '0px', 'col-fit-prefix-item']],
  ['spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.html', ['Chrome track CSSOM', '0px', 'col-fit-prefix-item']],
]);

const hashes = [];
for (const route of routes) {
  const file = path.join(outputRoot, route);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing/unsafe selected route: ${route}`);
  const bytes = fs.readFileSync(file);
  const text = bytes.toString('utf8');
  for (const marker of markers.get(route) ?? []) assert.ok(text.includes(marker), `${route} missing marker: ${marker}`);
  hashes.push({ path: route, bytes: bytes.length, sha256: sha256(bytes) });
}

const report = {
  pass: 'pass02',
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  selectedRouteCount: routes.length,
  selectedRoutes: hashes,
  rawAssetsStagedWithoutDeletion: stagedAssets,
  officialCountAndMergeStateMarkersVerified: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedRoutes: undefined }, null, 2)}\n`);
