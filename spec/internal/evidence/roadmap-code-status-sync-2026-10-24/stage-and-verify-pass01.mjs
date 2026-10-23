import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass01-selected');
const reportPath = path.join(evidenceDirectory, 'selected-build-verification-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const roadmapSource = fs.readFileSync(path.join(repositoryRoot, 'spec/public/roadmap.html'));
const roadmapPath = path.join(outputRoot, 'roadmap.html');
assert.ok(fs.lstatSync(roadmapPath).isFile(), 'RSPress build did not produce roadmap route');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected build report');

const temporaryPath = `${roadmapPath}.spinon-pass01-${randomUUID()}.tmp`;
try {
  fs.writeFileSync(temporaryPath, roadmapSource, { flag: 'wx' });
  fs.renameSync(temporaryPath, roadmapPath);
} catch (error) {
  if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
  throw error;
}
assert.equal(sha256(fs.readFileSync(roadmapPath)), sha256(roadmapSource), 'staged roadmap differs from tracked source');

const planNames = [
  'c11-4-auto-fit-named-line-implicit-fallback',
  'c11-4-auto-repeat-named-line-implicit-fallback',
  'c11-4-auto-repeat-named-lines',
  'c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups',
  'c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups',
  'c11-4-cyclic-minmax-calc-cross-axis-span-groups',
  'c11-4-cyclic-minmax-calc-minimum-span',
  'c11-4-cyclic-minmax-calc-overlapping-rows',
  'c11-4-cyclic-minmax-calc-overlapping-spans',
  'c11-4-cyclic-minmax-calc-two-axis-spanning-text',
  'c11-4-cyclic-minmax-max-content-mixed-text-image-span',
  'c11-4-cyclic-minmax-mixed-text-image-span',
  'c11-4-cyclic-minmax-replaced-image-span',
  'c11-4-two-axis-template-areas',
];
const routes = [
  'roadmap.html',
  'STATUS.html',
  ...planNames.map((name) => `plan/${name}.html`),
  'internal/evidence/roadmap-code-status-sync-2026-10-24/README.html',
  'internal/evidence/roadmap-code-status-sync-2026-10-24/pr-diff-review-20.html',
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-24/README.html',
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-24/pr-diff-review-20.html',
];

const roadmapText = fs.readFileSync(roadmapPath, 'utf8');
for (const marker of ['2026-10-24 C13 overflow·scroll 현재 판정', 'C13.4 scrollbar/gutter', 'C11.4', '72/213']) {
  assert.ok(roadmapText.includes(marker), `roadmap missing marker: ${marker}`);
}
const statusPath = path.join(outputRoot, 'STATUS.html');
const statusText = fs.readFileSync(statusPath, 'utf8');
for (const marker of ['2026-10-24 코드·로드맵 재대조', 'C13.4 scrollbar/gutter', 'C11.4', '72/213']) {
  assert.ok(statusText.includes(marker), `status page missing marker: ${marker}`);
}
for (const name of planNames) {
  const page = fs.readFileSync(path.join(outputRoot, 'plan', `${name}.html`), 'utf8');
  assert.ok(page.includes('72/213'), `${name} page missing current count`);
  assert.ok(!page.includes('77/213'), `${name} page retains stale unqualified count`);
}

const hashes = routes.map((route) => {
  const file = path.join(outputRoot, route);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing or unsafe built route: ${route}`);
  const bytes = fs.readFileSync(file);
  return { path: route, bytes: bytes.length, sha256: sha256(bytes) };
});
const report = {
  pass: 'pass01',
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  roadmapSourceSha256: sha256(roadmapSource),
  roadmapBuiltSha256: sha256(fs.readFileSync(roadmapPath)),
  officialCountAndC13StateMarkersVerified: true,
  livePlanPagesWithCurrentCount: planNames.length,
  allRequiredRoutesPresent: true,
  routeHashes: hashes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
