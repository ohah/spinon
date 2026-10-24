import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-fixed-repeat-auto-repeat-site-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'selected RSPress build is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected build verification');

const rawAssets = [
  { route: 'roadmap.html', source: 'spec/public/roadmap.html' },
  { route: 'STATUS.md', source: 'spec/STATUS.md' },
  { route: 'tests/fixtures/css/c11/grid-auto-repeat-fixed-repeat-c114-precomparison.html', source: 'tests/fixtures/css/c11/grid-auto-repeat-fixed-repeat-c114-precomparison.html' },
  { route: 'internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-observation.json', source: 'spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-observation.json' },
  { route: 'internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/stylo-computed-ast-observation.json', source: 'spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/stylo-computed-ast-observation.json' },
  { route: 'internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/SHA256SUMS', source: 'spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/SHA256SUMS' },
  { route: 'internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-fixed-repeat-auto-repeat.png', source: 'spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-fixed-repeat-auto-repeat.png' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/artifact-boundary.json', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/artifact-boundary.json' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/result.json', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/result.json' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/simulator.json', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/simulator.json' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/result.json', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/result.json' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/simulator.json', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/simulator.json' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/fixed-repeat-fit-prefix-occupied.png', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/fixed-repeat-fit-prefix-occupied.png' },
  { route: 'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/fixed-repeat-fit-prefix-occupied.png', source: 'spec/internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/fixed-repeat-fit-prefix-occupied.png' },
];

for (const asset of rawAssets) {
  const source = path.join(repositoryRoot, asset.source);
  const destination = path.join(outputRoot, asset.route);
  const bytes = fs.readFileSync(source);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const temporary = `${destination}.spinon-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  try {
    fs.renameSync(temporary, destination);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `raw asset mismatch: ${asset.route}`);
}

const routes = [
  'STATUS.html',
  'plan/c11-4-auto-repeat-fixed-repeat.html',
  'plan/c11-4-auto-repeat-fixed-repeat-plan-review-20.html',
  'plan/c11-4-auto-repeat-fixed-repeat-revision-review-20.html',
  'plan/c11-4-auto-repeat-fixed-repeat-revision-pr-diff-review-30.html',
  'plan/c11-4-auto-repeat-fixed-repeat-pr-diff-review-30.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/README.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/selected-docs-build-scope.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/pr-diff-review-30.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/artifact-boundary.html',
  ...rawAssets.map((asset) => asset.route),
];
assert.equal(routes.length, new Set(routes).size, 'selected routes must be unique');

const markers = new Map([
  ['roadmap.html', ['2026-10-24 C11.4 fixed-repeat + auto-repeat', '72/213']],
  ['STATUS.html', ['C11.4 intrinsic track sizing', '72/213']],
  ['STATUS.md', ['C11.4 intrinsic track sizing', '72/213']],
  ['plan/c11-4-auto-repeat-fixed-repeat.html', ['computed', '72/213']],
  ['internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/README.html', ['11/11', 'iPhone 17 Pro']],
]);
const selectedRoutes = [];
for (const route of routes) {
  const file = path.join(outputRoot, route);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing/unsafe route: ${route}`);
  const bytes = fs.readFileSync(file);
  const content = bytes.toString('utf8');
  for (const marker of markers.get(route) ?? []) assert.ok(content.includes(marker), `${route} lacks marker ${marker}`);
  selectedRoutes.push({ path: route, bytes: bytes.length, sha256: sha256(bytes) });
}

const report = {
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  selectedRouteCount: selectedRoutes.length,
  selectedRoutes,
  rawAssets,
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selectedOutputRoot: report.selectedOutputRoot, selectedRouteCount: selectedRoutes.length, noDeletes: true }, null, 2)}\n`);
