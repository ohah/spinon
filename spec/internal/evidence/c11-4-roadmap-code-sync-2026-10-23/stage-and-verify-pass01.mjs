import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-site-pass01-selected');
const roadmapSource = path.join(repositoryRoot, 'spec/public/roadmap.html');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceBytes = fs.readFileSync(roadmapSource);
const stagedPath = path.join(outputRoot, 'roadmap.html');
assert.ok(fs.lstatSync(stagedPath).isFile(), 'RSPress build did not produce the roadmap route');
const temporaryPath = `${stagedPath}.spinon-pass01-${randomUUID()}.tmp`;
try {
  fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
  fs.renameSync(temporaryPath, stagedPath);
} catch (error) {
  if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
  throw error;
}
assert.equal(sha256(fs.readFileSync(stagedPath)), sha256(sourceBytes), 'staged roadmap differs from source');

const routes = [
  'roadmap.html', 'STATUS.html', 'internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html', 'plan/c11-4-auto-repeat-template-areas.html',
  'plan/c11-4-auto-repeat-calc-minimum.html', 'plan/c11-4-auto-repeat-positive-percent-minimum.html',
  'internal/evidence/c11-4-auto-repeat-calc-minimum-runtime-2026-10-23/README.html',
  'internal/evidence/c11-4-template-area-auto-repeat-runtime-2026-10-23/README.html',
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
];
const routeHashes = [];
for (const route of routes) {
  const absolute = path.join(outputRoot, route);
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing built route: ${route}`);
  routeHashes.push({ path: route, bytes: stat.size, sha256: sha256(fs.readFileSync(absolute)) });
}

const roadmap = fs.readFileSync(stagedPath, 'utf8');
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
for (const value of ['72/213', '74/207', 'PR #553', 'C13 overflow·scroll 상태 대조', 'PR #568']) {
  assert.ok(roadmap.includes(value), `roadmap missing expected current/historical marker: ${value}`);
}
for (const value of ['72/213', 'ff4233d0df00a632d71c28398c9a8274fb4abc16', 'C11.4 checkbox는 미완료']) {
  assert.ok(status.includes(value), `status missing expected marker: ${value}`);
}
const mergePlan = fs.readFileSync(path.join(outputRoot, 'plan/c11-4-auto-repeat-template-areas.html'), 'utf8');
assert.ok(mergePlan.includes('0f46437c299df6f0c069b694a3f2184ac464ad3d'), 'plan has wrong #539 merge SHA');
assert.ok(!mergePlan.includes('0f46437c299df6f0c069b6943a2184ac464ad3d'), 'stale malformed #539 merge SHA remains');
const audit = fs.readFileSync(path.join(outputRoot, 'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html'), 'utf8');
const review = fs.readFileSync(path.join(outputRoot, 'internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html'), 'utf8');
for (const value of ['226', '87', '72/213', '4,096,773', 'active=false', 'pid=null']) {
  assert.ok(audit.includes(value), `audit evidence page missing final verification: ${value}`);
}
assert.ok(review.includes('226/87') && review.includes('통과'), 'PR diff review is incomplete');
assert.ok(!review.includes('검증 예정'), 'PR diff review has pending items');

const report = {
  pass: 'pass01',
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  roadmapSource: 'spec/public/roadmap.html',
  roadmapSourceSha256: sha256(sourceBytes),
  roadmapBuiltSha256: sha256(fs.readFileSync(stagedPath)),
  roadmapAndStatusMarkersVerified: true,
  routes: routeHashes,
  allRequiredRoutesPresent: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-verification-pass01.json'), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
