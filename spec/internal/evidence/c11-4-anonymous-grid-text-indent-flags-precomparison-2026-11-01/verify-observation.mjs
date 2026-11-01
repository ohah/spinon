import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const root = new URL('.', import.meta.url);
const observation = JSON.parse(readFileSync(new URL('chrome156-observation.json', root), 'utf8'));
const expected = JSON.parse(readFileSync(new URL('expected-values.json', root), 'utf8'));
const byId = new Map(observation.cases.map((row) => [row.id, row]));
const epsilon = 1 / 64;
const close = (actual, wanted, label) => {
  assert.ok(Math.abs(actual - wanted) <= epsilon, `${label}: ${actual} != ${wanted}`);
};

assert.equal(observation.fontLoaded, true, 'Ahem must be loaded before measurement');
assert.match(observation.userAgent, /HeadlessChrome\/156\./);
assert.equal(observation.viewport.width, 1280);
assert.equal(observation.viewport.height, 633);
assert.equal(observation.viewport.dpr, 1);
assert.equal(observation.cases.length, 20);
assert.equal(expected.cases.length, 20);
assert.equal(byId.size, 20, 'scene IDs must be unique');

for (const wanted of expected.cases) {
  const actual = byId.get(wanted.id);
  assert.ok(actual, `missing scene ${wanted.id}`);
  assert.equal(actual.computedIndent, wanted.computedIndent, `${wanted.id} computed text-indent`);
  assert.equal(actual.whiteSpace, wanted.whiteSpace, `${wanted.id} white-space`);
  close(actual.rootWidth, wanted.usedColumnsPx, `${wanted.id} root width`);
  close(actual.rootHeight, wanted.rootHeightPx, `${wanted.id} root height`);
  const track = /^(-?(?:\d+\.?\d*|\.\d+))px$/.exec(actual.usedColumns);
  assert.ok(track, `${wanted.id} must resolve to one CSS-pixel track`);
  close(Number(track[1]), wanted.usedColumnsPx, `${wanted.id} used column`);

  const textLines = actual.textRects.filter((rect) => rect.width > 0);
  assert.equal(textLines.length, wanted.lineOffsetsPx.length, `${wanted.id} line count`);
  assert.equal(textLines.length, wanted.lineOriginsYPx.length, `${wanted.id} block line count`);
  for (let index = 0; index < textLines.length; index += 1) {
    close(textLines[index].x, wanted.lineOffsetsPx[index], `${wanted.id} line ${index} origin`);
    close(textLines[index].y, wanted.lineOriginsYPx[index], `${wanted.id} line ${index} block offset`);
    close(textLines[index].width, wanted.lineAdvancesPx[index], `${wanted.id} line ${index} advance`);
    close(textLines[index].height, 10, `${wanted.id} line ${index} height`);
  }
}

console.log(JSON.stringify({ sceneCount: expected.cases.length, mismatchCount: 0, toleranceCssPx: epsilon }));
