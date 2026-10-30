import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const root = new URL('.', import.meta.url);
const readJson = async (path) => JSON.parse(await readFile(new URL(path, root), 'utf8'));
const [expected, observed] = await Promise.all([
  readJson('./expected-values.json'),
  readJson('./chrome/chrome156-observation.json'),
]);

assert.equal(observed.browser.font10, true);
assert.equal(observed.browser.font20, true);
assert.match(observed.browser.userAgent, /HeadlessChrome\/156\./);

const observationById = {
  'grid-last-baseline-align-items-multiline': observed.observations.last,
  'grid-last-baseline-align-self-multiline': observed.observations.self,
  'grid-last-baseline-source-order-reversed': observed.observations.reversed,
  'grid-first-baseline-control': observed.observations.first,
  'grid-start-control': observed.observations.start,
};

for (const scene of expected.cases) {
  const actual = observationById[scene.id];
  assert.ok(actual, `missing browser scene ${scene.id}`);
  assert.equal(actual.root.width, scene.root.width, `${scene.id} root width`);
  assert.equal(actual.root.height, scene.root.height, `${scene.id} root height`);
  assert.equal(actual.root.gridTemplateColumns, scene.root.columns.map((n) => `${n}px`).join(' '));
  assert.equal(actual.root.gridTemplateRows, `${scene.root.rows[0]}px`);

  for (const item of scene.items) {
    const browserItem = actual.items.find((candidate) => candidate.id.endsWith(item.id));
    assert.ok(browserItem, `${scene.id} missing ${item.id}`);
    for (const key of ['x', 'y', 'width', 'height']) {
      assert.equal(browserItem[key], item[key], `${scene.id} ${item.id} ${key}`);
    }
    const expectedLineCount = item.id === 'small' && scene.id !== 'grid-first-baseline-control' ? 2 : 1;
    assert.equal(browserItem.lineRects.length, expectedLineCount);

    const fontSize = Number.parseFloat(browserItem.font);
    const baselineOffset = fontSize * 0.8;
    const firstLineBaseline = browserItem.lineRects[0].y + baselineOffset;
    const lastLine = browserItem.lineRects.at(-1);
    const lastLineBaseline = lastLine.y + baselineOffset;
    if (item.firstBaselineFromRowStart !== undefined) {
      assert.equal(firstLineBaseline, item.firstBaselineFromRowStart, `${scene.id} ${item.id} first baseline`);
    }
    if (item.lastBaselineFromRowStart !== undefined) {
      assert.equal(lastLineBaseline, item.lastBaselineFromRowStart, `${scene.id} ${item.id} last baseline`);
    }
  }
}

console.log(`PASS ${expected.cases.length} scenes: independent geometry, wrapping, and baseline expectations match Chrome observation`);
