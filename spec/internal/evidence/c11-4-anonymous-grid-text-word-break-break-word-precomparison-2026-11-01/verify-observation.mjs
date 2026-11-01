import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const root = new URL("./", import.meta.url);
const [expected, observation] = await Promise.all([
  readFile(new URL("expected-values.json", root), "utf8").then(JSON.parse),
  readFile(new URL("chrome156-observation.json", root), "utf8").then(JSON.parse),
]);

assert.equal(observation.chrome, "156.0.0.0");
assert.equal(observation.devicePixelRatio, 1);
assert.equal(observation.fontsLoaded.ahem, true);
assert.equal(observation.supportsWordBreakBreakWord, true);
const expectedSceneIds = expected.matrix.flatMap(pair =>
  expected.trackValues.map(track =>
    `word-break-${pair.wordBreak}-overflow-wrap-${pair.overflowWrap}-${track.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "")}`,
  ),
).sort();
const actualSceneIds = observation.observations.map(scene => scene.id).sort();
assert.equal(expectedSceneIds.length, 18);
assert.equal(observation.observations.length, expectedSceneIds.length);
assert.equal(new Set(actualSceneIds).size, actualSceneIds.length, "duplicate scene IDs");
assert.deepEqual(actualSceneIds, expectedSceneIds, "missing or unexpected scene IDs");

const expectedByPair = new Map(
  expected.matrix.map(value => [`${value.wordBreak}/${value.overflowWrap}`, value]),
);
const px = value => Number.parseFloat(value);
const sameNumbers = (actual, wanted) => {
  assert.deepEqual(actual, wanted);
};

for (const actual of observation.observations) {
  const pair = expectedByPair.get(`${actual.authoredWordBreak}/${actual.authoredOverflowWrap}`);
  assert.ok(pair, `${actual.id}: unplanned property pair`);
  assert.ok(expected.trackValues.includes(actual.track), `${actual.id}: unplanned track`);
  assert.equal(
    actual.id,
    `word-break-${pair.wordBreak}-overflow-wrap-${pair.overflowWrap}-${actual.track.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "")}`,
    `${actual.id}: scene identity does not match its properties and track`,
  );
  assert.equal(actual.computedWordBreak, pair.wordBreak, `${actual.id}: computed word-break`);
  assert.equal(actual.computedOverflowWrap, pair.overflowWrap, `${actual.id}: computed overflow-wrap`);
  assert.equal(actual.rootBox.width, expected.gridWidthCssPx, `${actual.id}: root width`);

  let wantedColumn;
  let wantedRow;
  let wantedLineWidths;
  if (actual.track === "min-content") {
    wantedColumn = pair.minContentColumnCssPx;
    const wrapped = wantedColumn === expected.font.glyphAdvanceCssPx;
    wantedRow = wrapped
      ? expected.allCases.wrappedMinContentRowCssPx
      : expected.font.lineHeightCssPx;
    wantedLineWidths = wrapped
      ? expected.allCases.wrappedMinContentLineWidthsCssPx
      : [expected.font.glyphAdvanceCssPx * expected.text.length];
  } else if (actual.track === "max-content") {
    wantedColumn = expected.allCases.maxContentColumnCssPx;
    wantedRow = expected.allCases.maxContentRowCssPx;
    wantedLineWidths = expected.allCases.maxContentLineWidthsCssPx;
  } else {
    assert.equal(actual.track, "10px", `${actual.id}: unexpected track`);
    wantedColumn = expected.font.glyphAdvanceCssPx;
    wantedRow = pair.fixed10Px.rowCssPx;
    wantedLineWidths = pair.fixed10Px.lineWidthsCssPx;
  }

  assert.equal(px(actual.computedColumns), wantedColumn, `${actual.id}: column`);
  assert.equal(px(actual.computedRows), wantedRow, `${actual.id}: row`);
  assert.equal(actual.rootBox.height, wantedRow, `${actual.id}: root height`);
  assert.equal(actual.lineCount, wantedLineWidths.length, `${actual.id}: line count`);
  sameNumbers(
    actual.textRects.map(rect => rect.width),
    wantedLineWidths,
  );
  sameNumbers(
    actual.textRects.map(rect => rect.height),
    wantedLineWidths.map(() => expected.font.lineHeightCssPx),
  );
}

process.stdout.write("Chrome 156 precomparison: 18/18 scenes match independent expectations\n");
