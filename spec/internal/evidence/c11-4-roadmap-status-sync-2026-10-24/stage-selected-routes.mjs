import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-root-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-site-selected');
const pass = process.argv[2] ?? 'pass01';
assert.ok(['pass01', 'pass02'].includes(pass), 'usage: stage-selected-routes.mjs [pass01|pass02]');
const reportPath = path.join(evidenceDirectory, pass === 'pass01' ? 'selected-build-verification.json' : `selected-build-verification-${pass}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected-build verification');

const staged = [];
for (const [sourceRelative, targetRelative] of [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/chrome156-zero-percent-auto-repeat-agent-browser-pass02.png', 'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/chrome156-zero-percent-auto-repeat-agent-browser-pass02.png'],
]) {
  const source = path.join(selectedRoot, sourceRelative);
  const target = path.join(outputRoot, targetRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe route source: ${sourceRelative}`);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  if (fs.existsSync(target)) {
    const targetStat = fs.lstatSync(target);
    assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `unsafe route target: ${targetRelative}`);
    fs.unlinkSync(target);
  }
  const bytes = fs.readFileSync(source);
  const temporary = `${target}.${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  fs.renameSync(temporary, target);
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staged route hash mismatch: ${targetRelative}`);
  staged.push({ source: sourceRelative, path: targetRelative, bytes: bytes.length, sha256: sha256(bytes) });
}

const expectedRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum-plan-review-20.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/plan-pr-diff-review-20.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/chrome156-zero-percent-auto-repeat-agent-browser-pass02.png',
  'static/image/chrome156-zero-percent-auto-repeat-agent-browser-pass02.d9add6adc9.png',
  'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/plan-pr-diff-review-20.html',
];
const routes = expectedRoutes.map((relative) => {
  const absolute = path.join(outputRoot, relative);
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing or unsafe built route: ${relative}`);
  const bytes = fs.readFileSync(absolute);
  return { path: relative, bytes: bytes.length, sha256: sha256(bytes) };
});

const roadmap = fs.readFileSync(path.join(outputRoot, 'roadmap.html'), 'utf8');
for (const marker of ['C11.4 auto-repeat 0% minimum · 계획 단계', 'C13 overflow·scroll 현재 판정', '현재 활성 작업은 아니며', '72/213', '13-scene']) {
  assert.ok(roadmap.includes(marker), `roadmap missing marker: ${marker}`);
}
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
for (const marker of ['기준', '2026-10-24', 'zero-percent auto-repeat minimum', '72/213', 'C11.4 checkbox']) {
  assert.ok(status.includes(marker), `status missing marker: ${marker}`);
}
const precomparison = fs.readFileSync(path.join(outputRoot, 'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html'), 'utf8');
assert.ok(precomparison.includes('/spinon/static/image/chrome156-zero-percent-auto-repeat-agent-browser-pass02.d9add6adc9.png'), 'precomparison screenshot asset was not bundled');

const report = {
  pass,
  rspressVersion: '2.0.22',
  selectedBuildCommand: './packages/docs/node_modules/.bin/rspress build --config spec/internal/evidence/c11-4-roadmap-status-sync-2026-10-24/rspress.config.ts',
  sourceRoadmapSha256: sha256(fs.readFileSync(path.join(repositoryRoot, 'spec/public/roadmap.html'))),
  outputRoadmapSha256: sha256(fs.readFileSync(path.join(outputRoot, 'roadmap.html'))),
  sourceStatusSha256: sha256(fs.readFileSync(path.join(repositoryRoot, 'spec/STATUS.md'))),
  outputStatusRouteSha256: sha256(fs.readFileSync(path.join(outputRoot, 'STATUS.html'))),
  expectedRoutes: routes,
  explicitlyStagedStaticRoutes: staged,
  markerChecksPassed: true,
  fullRepositoryDeadLinkChecks: 'not run in this isolated build; the full build previously failed on unrelated existing dead links',
};
assert.equal(report.sourceRoadmapSha256, report.outputRoadmapSha256, 'roadmap source/build hash mismatch');
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ routeCount: routes.length, markerChecksPassed: true, roadmapSha256: report.sourceRoadmapSha256 }, null, 2)}\n`);
