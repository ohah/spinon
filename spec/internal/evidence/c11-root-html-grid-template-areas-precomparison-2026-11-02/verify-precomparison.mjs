import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const readJson = (name) => JSON.parse(readFileSync(resolve(here, name), 'utf8'));
const expected = readJson('expected.json');
const observed = readJson('chrome-observation.json');
const fixture = readFileSync(
  resolve(here, '../../../../tests/fixtures/css/c11/root-html-grid-template-areas-precomparison.html'),
  'utf8',
);

assert.match(observed.userAgent, /HeadlessChrome\/156\./);
assert.deepEqual(observed.viewportCssPx, expected.browserOracle.viewportCssPx);
assert.equal(observed.deviceScaleFactor, expected.browserOracle.deviceScaleFactor);
assert.deepEqual(observed.root.rectCssPx, expected.root.rectCssPx);
assert.equal(observed.root.gridTemplateColumns, expected.root.gridTemplateColumns.join(' '));
assert.equal(observed.root.gridTemplateRows, expected.root.gridTemplateRows.join(' '));
assert.equal(observed.root.gridTemplateAreas, expected.root.gridTemplateAreas);
assert.deepEqual(observed.originalOrder, expected.sourceOrderControl.before);
assert.deepEqual(observed.reversedOrder, expected.sourceOrderControl.after);
assert.equal(observed.sourceOrderControl.semanticItemFramesEqual, true);

const observedByArea = new Map(observed.items.map((item) => [item.area, item]));
const reversedByArea = new Map(observed.reversedItems.map((item) => [item.area, item]));
for (const item of expected.items) {
  const actual = observedByArea.get(item.area);
  const reversed = reversedByArea.get(item.area);
  assert.ok(actual, `missing Chrome item ${item.area}`);
  assert.ok(reversed, `missing reversed Chrome item ${item.area}`);
  assert.equal(actual.computedGridArea, item.gridArea, `${item.area} grid-area`);
  assert.deepEqual(actual.rectCssPx, item.rectCssPx, `${item.area} frame`);
  assert.deepEqual(reversed.rectCssPx, actual.rectCssPx, `${item.area} reordered frame`);
  assert.equal(actual.backgroundColor, item.backgroundColor, `${item.area} background`);
  assert.deepEqual(actual.computedPlacementLonghands, Array(4).fill(item.gridArea));
  assert.deepEqual(reversed.computedPlacementLonghands, actual.computedPlacementLonghands);
}
assert.equal(observed.items.length, expected.items.length);
assert.equal(observed.reversedItems.length, expected.items.length);

for (const declaration of [
  'grid-template-columns: 40px 80px',
  'grid-template-rows: 20px 50px',
  'grid-template-areas: "header header" "side content"',
  'grid-area: header',
  'grid-area: side',
  'grid-area: content',
]) {
  assert.ok(fixture.includes(declaration), `fixture missing ${declaration}`);
}

process.stdout.write(`${JSON.stringify({
  result: 'pass',
  checkGroups: ['browser metadata', 'root geometry', 'three item placements', 'DOM-order invariance', 'fixture declarations'],
  browser: 'HeadlessChrome 156',
  semanticItemFrames: observed.items.length,
  sourceOrderControl: 'pass',
  fixtureDeclarations: 6,
})}\n`);
