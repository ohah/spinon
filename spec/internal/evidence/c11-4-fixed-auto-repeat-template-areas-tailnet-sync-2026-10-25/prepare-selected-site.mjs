import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceBuild = path.join(repositoryRoot, 'build/spinon/c114-merge-roadmap-20261025-selected-site');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c114-merge-roadmap-20261025-selected-root');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-merge-roadmap-20261025-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification.json');
const routes = [
  'roadmap.html',
  'icon.svg',
  'logo.svg',
  'STATUS.html',
  'plan/c11-4-fixed-auto-repeat-template-areas.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-24/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-24/pr-diff-review-30.html',
  'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-avd-final-2026-10-25/fixed-auto-area-columns-area-larger.png',
  'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/ios-simulator-final-2026-10-25/fixed-auto-area-columns-area-larger.png',
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(sourceBuild).isDirectory(), 'selected Rspress build is required');
if (fs.existsSync(selectedRoot)) {
  const existingRoot = fs.lstatSync(selectedRoot);
  assert.ok(existingRoot.isDirectory() && !existingRoot.isSymbolicLink(), 'unsafe selected output root');
} else {
  fs.mkdirSync(selectedRoot, { recursive: true });
}
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected build report');
fs.mkdirSync(selectedRoot, { recursive: true });

const selectedRoutes = [];
for (const route of routes) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
  const staticSources = new Map([
    ['roadmap.html', 'public/roadmap.html'],
    ['icon.svg', 'public/icon.svg'],
    ['logo.svg', 'public/logo.svg'],
    ['internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-avd-final-2026-10-25/fixed-auto-area-columns-area-larger.png', 'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-avd-final-2026-10-25/fixed-auto-area-columns-area-larger.png'],
    ['internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/ios-simulator-final-2026-10-25/fixed-auto-area-columns-area-larger.png', 'internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/ios-simulator-final-2026-10-25/fixed-auto-area-columns-area-larger.png'],
  ]);
  const staticSource = staticSources.get(route);
  const source = path.join(staticSource ? sourceRoot : sourceBuild, staticSource ?? route);
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
for (const marker of ['#604', '912c10bee4e2d88374a2a32689ac37c824b1207d', '72/213', '9/9', '8/8']) {
  assert.ok(roadmap.includes(marker), `roadmap missing merged-state marker: ${marker}`);
}
const status = fs.readFileSync(path.join(selectedRoot, 'STATUS.html'), 'utf8');
for (const marker of ['#604', '912c10bee4e2d88374a2a32689ac37c824b1207d', '72/213']) {
  assert.ok(status.includes(marker), `status page missing merged-state marker: ${marker}`);
}

const report = {
  selectedOutputRoot: path.relative(repositoryRoot, selectedRoot),
  sourceBuild: path.relative(repositoryRoot, sourceBuild),
  selectedRouteCount: selectedRoutes.length,
  selectedRoutes,
  roadmap_merge_sha: '912c10bee4e2d88374a2a32689ac37c824b1207d',
  official_completion_count: '72/213',
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
