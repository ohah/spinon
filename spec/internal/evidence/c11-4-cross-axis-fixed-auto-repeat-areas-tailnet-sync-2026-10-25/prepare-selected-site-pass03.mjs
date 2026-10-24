import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceBuild = path.join(repositoryRoot, 'build/spinon/c114-cross-axis-merge-roadmap-20261025-selected-site-pass03');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c114-cross-axis-merge-roadmap-20261025-selected-root-pass03');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-cross-axis-merge-roadmap-20261025-selected-pass03');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass03.json');
const prefix = 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25';
const routes = [
  'roadmap.html',
  'icon.svg',
  'logo.svg',
  'STATUS.html',
  'plan/c11-4-cross-axis-fixed-auto-repeat-areas.html',
  'plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-review-20.html',
  'plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-pr-diff-review-20.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/README.html',
  'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/chrome-agent-browser.png',
  `${prefix}/README.html`,
  `${prefix}/implementation-runtime-review-20.html`,
  `${prefix}/pr-diff-review-30.html`,
  `${prefix}/post-merge-roadmap-pr-diff-review-30.html`,
  `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`,
  `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`,
  `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`,
  `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`,
];
const directSources = new Map([
  ['roadmap.html', 'roadmap.html'],
  ['icon.svg', 'public/icon.svg'],
  ['logo.svg', 'public/logo.svg'],
  ['internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/chrome-agent-browser.png', 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/chrome-agent-browser.png'],
  [`internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`, `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`],
  [`internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`, `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`],
  [`internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`, `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`],
  [`internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`, `internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`],
]);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(sourceBuild).isDirectory(), 'selected Rspress build is required');
if (fs.existsSync(selectedRoot)) {
  const existingRoot = fs.lstatSync(selectedRoot);
  assert.ok(existingRoot.isDirectory() && !existingRoot.isSymbolicLink(), 'unsafe selected output root');
} else {
  fs.mkdirSync(selectedRoot, { recursive: true });
}
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected build report');
const selectedRoutes = [];
for (const route of routes) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const directSource = directSources.get(route);
  const source = path.join(directSource ? sourceRoot : sourceBuild, directSource ?? route);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe source: ${route}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  let copied;
  if (fs.existsSync(destination)) {
    const existing = fs.lstatSync(destination);
    assert.ok(existing.isFile() && !existing.isSymbolicLink(), `unsafe selected output: ${route}`);
    copied = fs.readFileSync(destination);
  } else {
    const temporary = `${destination}.spinon-selected-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, bytes, { flag: 'wx' });
    fs.renameSync(temporary, destination);
    copied = fs.readFileSync(destination);
  }
  assert.equal(sha256(copied), sha256(bytes), `selected route hash mismatch: ${route}`);
  selectedRoutes.push({ path: route, bytes: copied.byteLength, sha256: sha256(copied) });
}

const roadmap = fs.readFileSync(path.join(selectedRoot, 'roadmap.html'), 'utf8');
for (const marker of ['#607', '2f00c2c95061db635facece5233327f895c99319', '72/213', '11/11', '35/35']) {
  assert.ok(roadmap.includes(marker), `roadmap missing merged-state marker: ${marker}`);
}
const status = fs.readFileSync(path.join(selectedRoot, 'STATUS.html'), 'utf8');
for (const marker of ['#607', '2f00c2c95061db635facece5233327f895c99319', '72/213']) {
  assert.ok(status.includes(marker), `status page missing merged-state marker: ${marker}`);
}
const report = {
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  sourceBuild: path.relative(repositoryRoot, sourceBuild),
  selectedRouteCount: selectedRoutes.length,
  selectedRoutes,
  roadmap_merge_sha: '2f00c2c95061db635facece5233327f895c99319',
  official_completion_count: '72/213',
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
