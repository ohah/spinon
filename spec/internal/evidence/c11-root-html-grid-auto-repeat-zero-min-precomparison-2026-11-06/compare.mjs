import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";

const directory = new URL("./", import.meta.url);
const expected = JSON.parse(
  readFileSync(new URL("independent-expected.json", directory), "utf8"),
);
const observed = JSON.parse(
  readFileSync(new URL("chrome157-observation.json", directory), "utf8"),
);

assert.match(observed.userAgent, /HeadlessChrome\/157\./);
assert.equal(observed.supportsZeroMinAutoRepeat, true);
assert.equal(observed.cases.length, expected.cases.length);

const observedById = new Map(observed.cases.map((item) => [item.id, item]));
const comparisons = expected.cases.map((testCase) => {
  const actual = observedById.get(testCase.id);
  assert.ok(actual, `Chrome observation is missing ${testCase.id}`);

  const flooredPatternSize = testCase.patternMinTrackSizesPx.reduce(
    (sum, size) => sum + Math.max(size, expected.uaTrackFloorCssPx),
    0,
  );
  const denominator =
    flooredPatternSize +
    testCase.patternMinTrackSizesPx.length * testCase.gapPx;
  const derivedCount = Math.max(
    1,
    Math.floor((testCase.availableSpacePx + testCase.gapPx) / denominator),
  );
  assert.equal(derivedCount, testCase.expectedRepetitionCount);

  const actualTracks = testCase.axis === "columns" ? actual.columns : actual.rows;
  assert.deepEqual(actualTracks, testCase.expectedTrackSizesPx);
  assert.equal(actualTracks.length, derivedCount * testCase.patternMinTrackSizesPx.length);

  if (testCase.expectedFirstItemSizePx === null) {
    assert.equal(actual.item, null);
  } else {
    assert.ok(actual.item, `${testCase.id} expected an in-flow item`);
    assert.equal(actual.item.width, testCase.expectedFirstItemSizePx[0]);
    assert.equal(actual.item.height, testCase.expectedFirstItemSizePx[1]);
  }

  return {
    id: testCase.id,
    repetitions: derivedCount,
    tracks: actualTracks.length,
    trackSizesCssPx: actualTracks,
    itemSizeCssPx: actual.item
      ? [actual.item.width, actual.item.height]
      : null,
    result: "PASS",
  };
});

const result = {
  browser: observed.userAgent,
  viewportCssPx: observed.viewportCssPx,
  independentFloorCssPx: expected.uaTrackFloorCssPx,
  cases: comparisons,
  passed: comparisons.length,
  failed: 0,
};

writeFileSync(
  new URL("comparison.json", directory),
  `${JSON.stringify(result, null, 2)}\n`,
);
process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
