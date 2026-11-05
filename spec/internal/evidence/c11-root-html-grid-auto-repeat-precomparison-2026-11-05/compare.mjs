import { readFileSync } from 'node:fs';

const base = 'spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05';
const expected = JSON.parse(readFileSync(`${base}/normative-expected.json`, 'utf8'));
const observed = JSON.parse(readFileSync(`${base}/chromium-observation.json`, 'utf8'));
const close = (a, b) => Math.abs(a - b) <= 1e-9;
const failures = [];
const summaries = [];
const rootSizes = new Map([
  ['columns-fill-two-repetitions', [200, 20]],
  ['columns-fit-two-repetitions', [200, 20]],
  ['columns-fit-hidden-item', [200, 20]],
  ['columns-fill-boundary-below', [169, 20]],
  ['columns-fill-subpx-ua-count-floor', [2, 20]],
  ['columns-fit-empty', [200, 20]],
  ['columns-fill-boundary-exact', [170, 20]],
  ['columns-visible-overflow', [170, 20]],
  ['rows-fill-two-repetitions', [60, 200]],
  ['rows-fit-two-repetitions', [60, 200]],
  ['rows-fit-empty', [60, 200]],
  ['rows-fill-boundary-below', [60, 169]],
  ['rows-fill-boundary-exact', [60, 170]],
]);

const byId = new Map(observed.results.map((result) => [result.id, result]));
for (const testCase of expected.cases) {
  const result = byId.get(testCase.id);
  if (!result) {
    failures.push(`${testCase.id}: missing Chromium result`);
    continue;
  }
  const axisRows = testCase.id.startsWith('rows-');
  const trackText = axisRows ? result.computedRows : result.computedColumns;
  const tracks = trackText.split(/\s+/).map((value) => Number.parseFloat(value));
  const rootSize = rootSizes.get(testCase.id);
  if (tracks.length !== testCase.tracksPx.length || !tracks.every((value, index) => close(value, testCase.tracksPx[index]))) {
    failures.push(`${testCase.id}: tracks expected ${JSON.stringify(testCase.tracksPx)} got ${JSON.stringify(tracks)}`);
  }
  if (!close(result.frame[2], rootSize[0]) || !close(result.frame[3], rootSize[1])) {
    failures.push(`${testCase.id}: root size expected ${rootSize.join('x')} got ${result.frame[2]}x${result.frame[3]}`);
  }
  const wantedGap = testCase.gapPx ?? [10, 10];
  const actualGap = result.gap.map((value) => Number.parseFloat(value));
  if (!actualGap.every((value, index) => close(value, wantedGap[index]))) {
    failures.push(`${testCase.id}: gaps expected ${JSON.stringify(wantedGap)} got ${JSON.stringify(actualGap)}`);
  }
  for (let index = 0; index < testCase.itemsRelativeToGrid.length; index += 1) {
    const wanted = testCase.itemsRelativeToGrid[index];
    const got = result.items[index];
    if (wanted === null) {
      if (got.display !== 'none' || got.frame.some((value) => value !== 0)) {
        failures.push(`${testCase.id}: hidden child was not absent from layout`);
      }
      continue;
    }
    const local = [got.frame[0] - result.frame[0], got.frame[1] - result.frame[1], got.frame[2], got.frame[3]];
    if (!local.every((value, offset) => close(value, wanted[offset]))) {
      failures.push(`${testCase.id} item ${index}: expected ${JSON.stringify(wanted)} got ${JSON.stringify(local)}`);
    }
  }
  if (testCase.overflowPx !== undefined) {
    const overflow = result.items[0].frame[2] - result.frame[2];
    if (!close(overflow, testCase.overflowPx)) failures.push(`${testCase.id}: expected ${testCase.overflowPx}px overflow got ${overflow}px`);
  }
  summaries.push({ id: testCase.id, tracks, root: result.frame.slice(2), visibleItems: result.items.filter((item) => item.display !== 'none').length });
}
if (byId.size !== expected.cases.length) failures.push(`result count expected ${expected.cases.length} got ${byId.size}`);
console.log(JSON.stringify({
  status: failures.length === 0 ? 'pass' : 'fail',
  browser: observed.userAgent,
  viewport: observed.viewport,
  caseCount: summaries.length,
  checks: ['repeat boundary', 'computed explicit track sizes', 'auto-fit collapse', 'row and column axes', 'item frames', 'hidden item exclusion', 'visible overflow', 'gaps', 'UA repeat-count floor'],
  failures,
  summaries,
}, null, 2));
if (failures.length) process.exitCode = 1;
