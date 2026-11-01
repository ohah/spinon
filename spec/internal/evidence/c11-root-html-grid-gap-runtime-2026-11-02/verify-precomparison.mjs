import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const root = new URL('./', import.meta.url);
const expected = JSON.parse(readFileSync(new URL('expected.json', root), 'utf8'));
const browserCliOutput = JSON.parse(
  readFileSync(new URL('chrome-observation.json', root), 'utf8'),
);
const observed = JSON.parse(browserCliOutput);

assert.match(observed.browser, /HeadlessChrome\/156\.0\.0\.0/);
assert.deepEqual(observed.viewport, expected.viewportCssPx);
assert.deepEqual(observed.root.frame, expected.root.frameCssPx);
assert.deepEqual(
  observed.root.columns.split(' ').map((value) => Number.parseFloat(value)),
  expected.root.columnsCssPx,
);
assert.deepEqual(
  observed.root.rows.split(' ').map((value) => Number.parseFloat(value)),
  expected.root.rowsCssPx,
);
assert.equal(observed.root.columnGap, `${expected.root.columnGapCssPx}px`);
assert.equal(observed.root.rowGap, `${expected.root.rowGapCssPx}px`);
assert.deepEqual(
  observed.items.map(({ selector, frame }) => ({ selector, frameCssPx: frame })),
  expected.items,
);

process.stdout.write('C11 Root HTML Grid gap precomparison: 1/1 passed\n');
