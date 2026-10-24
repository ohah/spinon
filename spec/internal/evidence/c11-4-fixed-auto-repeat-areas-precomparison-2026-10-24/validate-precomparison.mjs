import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const observation = JSON.parse(
  await readFile(resolve(evidenceDir, "chrome156-observation.json"), "utf8"),
);
const expected = JSON.parse(
  await readFile(resolve(evidenceDir, "expected-values.json"), "utf8"),
);
const epsilon = 1 / 64;
const observedById = new Map(observation.map((scene) => [scene.id, scene]));
const fixedBreadth =
  expected.basis.fixedPrefixTrackSizesPx.reduce((sum, size) => sum + size, 0) +
  expected.basis.fixedSuffixTrackSizesPx.reduce((sum, size) => sum + size, 0);
const fixedTrackCount =
  expected.basis.fixedPrefixTrackSizesPx.length +
  expected.basis.fixedSuffixTrackSizesPx.length;
const gapCountAtZeroRepeats = fixedTrackCount - 1;
const repeatTrackSize = expected.basis.autoRepeatTrackSizePx;
const gapSize = expected.basis.gapPx;
const minimumBreadth = (repeatCount) =>
  fixedBreadth +
  repeatTrackSize * repeatCount +
  gapSize * (fixedTrackCount + repeatCount - 1);
assert.equal(
  expected.basis.autoRepeatTrackCountFormula,
  `${fixedBreadth + gapSize * gapCountAtZeroRepeats}px + ${repeatTrackSize + gapSize}px * n <= availableSize`,
  "documented independent auto-repeat formula",
);
assert.deepEqual(
  [...observedById.keys()].sort(),
  Object.keys(expected.scenes).sort(),
  "Chrome scene inventory differs from the independent expectation",
);

let comparedBoxes = 0;
for (const [id, target] of Object.entries(expected.scenes)) {
  const actual = observedById.get(id);
  const availableSize = actual.root[target.axis === "columns" ? 2 : 3];
  assert.equal(availableSize, target.availableSizePx, `${id}: definite container size`);
  let calculatedAutoRepeatCount = 0;
  while (minimumBreadth(calculatedAutoRepeatCount + 1) <= target.availableSizePx) {
    calculatedAutoRepeatCount += 1;
  }
  assert.equal(calculatedAutoRepeatCount, target.autoRepeatCount, `${id}: independent fill count`);
  const areaRows = [...actual.areas.matchAll(/"([^"]*)"/g)].map(([, row]) =>
    row.trim().split(/\s+/),
  );
  const areaDimension = target.axis === "columns" ? areaRows[0].length : areaRows.length;
  assert.equal(areaDimension, target.areaDimensionTracks, `${id}: area dimension`);
  const expandedTemplateTracks = 6 + target.autoRepeatCount;
  assert.equal(expandedTemplateTracks, target.expandedTemplateTracks, `${id}: template track count`);
  assert.equal(
    Math.max(expandedTemplateTracks, areaDimension),
    target.usedTrackSizesPx.length,
    `${id}: explicit tracks after area expansion`,
  );
  const actualTrackText = target.axis === "columns" ? actual.columns : actual.rows;
  const actualTrackSizes = [...actualTrackText.matchAll(/(\d+(?:\.\d+)?)px/g)].map(
    ([, size]) => Number(size),
  );
  assert.deepEqual(actualTrackSizes, target.usedTrackSizesPx, `${id}: used tracks`);
  assert.equal(actualTrackSizes.length, target.usedTrackSizesPx.length, `${id}: track count`);
  const children = new Map(actual.children.map((child) => [child.className, child]));
  assert.deepEqual(
    [...children.keys()].sort(),
    Object.keys(target.children).sort(),
    `${id}: child inventory`,
  );
  for (const [className, expectedRect] of Object.entries(target.children)) {
    const rect = children.get(className).rect;
    const relative = [
      rect[0] - actual.root[0],
      rect[1] - actual.root[1],
      rect[2],
      rect[3],
    ];
    relative.forEach((value, index) => {
      assert.ok(
        Math.abs(value - expectedRect[index]) <= epsilon,
        `${id}/${className} rect[${index}]: ${value} != ${expectedRect[index]}`,
      );
    });
    comparedBoxes += 1;
  }
}

for (const axis of ["columns", "rows"]) {
  const scene = observedById.get(`${axis}-fill-collision`);
  const trackText = axis === "columns" ? scene.columns : scene.rows;
  const lineNamesByLine = [[]];
  let trackIndex = 0;
  for (const match of trackText.matchAll(/\[([^\]]*)\]|(\d+(?:\.\d+)?)px/g)) {
    if (match[1] !== undefined) {
      lineNamesByLine[trackIndex].push(...match[1].trim().split(/\s+/));
    } else {
      trackIndex += 1;
      lineNamesByLine[trackIndex] ??= [];
    }
  }
  const explicitOccurrences = Object.fromEntries(
    ["hero-start", "hero-end"].map((name) => [
      name,
      lineNamesByLine.flatMap((names, index) => (names.includes(name) ? [index + 1] : [])),
    ]),
  );
  assert.deepEqual(
    explicitOccurrences,
    expected.namedLineCollision.mergedDistinctPhysicalLineNumbers1Based,
    `${axis}: explicit repeated name occurrence ordering`,
  );

  const areaRows = [...scene.areas.matchAll(/"([^"]*)"/g)].map(([, row]) =>
    row.trim().split(/\s+/),
  );
  const areaColumns = areaRows[0].length;
  const areaRowCount = areaRows.length;
  const heroCells = [];
  for (let row = 0; row < areaRows.length; row += 1) {
    for (let column = 0; column < areaColumns; column += 1) {
      if (areaRows[row][column] === "hero") heroCells.push([row, column]);
    }
  }
  assert.ok(heroCells.length > 0, `${axis}: hero area exists`);
  const starts = [
    Math.min(...heroCells.map(([row, column]) => (axis === "columns" ? column : row))) + 1,
  ];
  const ends = [
    Math.max(...heroCells.map(([row, column]) => (axis === "columns" ? column : row))) + 2,
  ];
  assert.deepEqual(
    { "hero-start": starts, "hero-end": ends },
    expected.namedLineCollision.areaGeneratedLineNumbers1Based,
    `${axis}: area-generated line positions`,
  );
}

console.log(
  `PASS: ${Object.keys(expected.scenes).length} scenes, ${comparedBoxes} child boxes; tolerance ${epsilon} CSS px`,
);
