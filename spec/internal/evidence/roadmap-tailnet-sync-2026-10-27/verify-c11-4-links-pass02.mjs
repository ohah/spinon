import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const reportPath = path.join(evidenceDirectory, 'selected-c11-4-links-pass02.json');
const buildReportPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const evidence = 'internal/evidence/c11-4-generic-direct-text-runtime-matrix-2026-10-27';
const run = `${evidence}/runs/c114-direct-text-matrix-20261027T031021Z-pass02`;
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite targeted link report');
const build = JSON.parse(fs.readFileSync(buildReportPath, 'utf8'));
const selectedRoot = path.join(repositoryRoot, build.selectedOutputRoot);
const roadmapBytes = execFileSync('git', ['show', `${build.sourceHead}:spec/public/roadmap.html`], { cwd: repositoryRoot });
const roadmap = roadmapBytes.toString('utf8');
const expected = [
  `/spinon/plan/c11-4-generic-direct-text-runtime-matrix.html`,
  `/spinon/${evidence}/README.html`,
  `/spinon/${evidence}/implementation-runtime-review-20.html`,
  `/spinon/${evidence}/pr-diff-review-20.html`,
  `/spinon/${run}/android-avd/screenshot.png`,
  `/spinon/${run}/ios-simulator/screenshot.png`,
  '/spinon/STATUS.html#css-구현-체크리스트',
  `/spinon/${evidence}/post-merge-status-roadmap-review-20.html`,
];
const recordStart = roadmap.indexOf('2026-10-27 C11.4 direct element-text intrinsic · 6-scene app matrix PR');
assert.notEqual(recordStart, -1, 'public roadmap direct-text record is missing');
const recordEnd = roadmap.indexOf('</p>', recordStart);
assert.notEqual(recordEnd, -1, 'public roadmap direct-text record is not a closed paragraph');
const record = roadmap.slice(recordStart, recordEnd);
const actual = [...record.matchAll(/href="([^"]+)"/g)].map((match) => match[1]).filter((href) => href.startsWith('/spinon/'));
for (const route of expected) assert.ok(actual.includes(route), `roadmap record omits ${route}`);

const requiredFiles = [
  'roadmap.html', 'STATUS.html',
  'plan/c11-4-generic-direct-text-runtime-matrix.html',
  `${evidence}/README.html`,
  `${evidence}/implementation-runtime-review-20.html`,
  `${evidence}/pr-diff-review-20.html`,
  `${evidence}/post-merge-status-roadmap-review-20.html`,
  'internal/evidence/roadmap-tailnet-sync-2026-10-27/status-roadmap-pr-diff-review-30.html',
  `${run}/android-avd/screenshot.png`, `${run}/ios-simulator/screenshot.png`,
  `${run}/android-avd/matrix.tsv`, `${run}/ios-simulator/matrix.tsv`,
];
const routeReport = [];
for (const route of requiredFiles) {
  const absolute = path.join(selectedRoot, route);
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `selected route is missing or unsafe: ${route}`);
  const bytes = fs.readFileSync(absolute);
  if (route.endsWith('.png')) assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', `invalid PNG: ${route}`);
  routeReport.push({ path: route, bytes: bytes.length, sha256: sha256(bytes) });
}
const report = {
  pass: 'pass02', sourceHead: build.sourceHead, sourceTree: build.sourceTree,
  scope: 'C11.4 direct-text roadmap record and its selected evidence pages/screens only',
  roadmapDirectTextLinkCount: actual.length,
  expectedDirectTextRoutes: expected,
  allExpectedRoutesPublished: true,
  checkedSelectedRouteCount: routeReport.length,
  selectedRoutes: routeReport,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
