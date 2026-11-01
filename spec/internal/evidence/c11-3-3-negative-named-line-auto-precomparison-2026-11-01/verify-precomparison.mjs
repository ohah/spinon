import assert from 'node:assert/strict';
import fs from 'node:fs';

const base = new URL('.', import.meta.url);
const input = JSON.parse(fs.readFileSync(new URL('input.json', base), 'utf8'));
const expected = JSON.parse(fs.readFileSync(new URL('expected-values.json', base), 'utf8'));
const observed = JSON.parse(fs.readFileSync(new URL('chrome-observation.json', base), 'utf8'));

assert.match(observed.userAgent, /HeadlessChrome\/156\./);
assert.deepEqual(observed.viewport, input.viewport, 'browser viewport');
assert.equal(input.cases.length, expected.expectations.length);
assert.equal(input.cases.length, observed.cases.length);

const pxValues = (value) => [...value.matchAll(/(?:^|\s)(\d+(?:\.\d+)?)px(?=\s|$)/g)].map((match) => Number(match[1]));
for (let index = 0; index < input.cases.length; index += 1) {
  const inputCase = input.cases[index];
  const expectedCase = expected.expectations[index];
  const observedCase = observed.cases[index];
  assert.equal(expectedCase.id, inputCase.id, `expected case order ${index}`);
  assert.equal(observedCase.id, inputCase.id, `observed case order ${index}`);
  assert.deepEqual(
    { width: observedCase.root.width, height: observedCase.root.height },
    { width: input.root.width, height: input.root.height },
    `${inputCase.id} root size`,
  );
  const observedTracks = pxValues(inputCase.axis === 'columns' ? observedCase.columns : observedCase.rows);
  assert.deepEqual(observedTracks, expectedCase.trackSizes, `${inputCase.id} track sizes`);
  assert.deepEqual(observedCase.item, expectedCase.itemFrameRelativeToRoot, `${inputCase.id} frame`);
}

console.log(`PASS: ${observed.viewport.width}x${observed.viewport.height} DPR ${observed.viewport.deviceScaleFactor}; ${input.cases.length}/${input.cases.length} Chrome observations match independent tracks and frames`);
