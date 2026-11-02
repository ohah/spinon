import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";

const fixturePath = "tests/fixtures/css/c11/root-html-grid-fr-precomparison.html";
const evidencePath = "spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02";
const [fixture, expectedText, observationText] = await Promise.all([
  readFile(fixturePath),
  readFile(`${evidencePath}/expected.json`, "utf8"),
  readFile(`${evidencePath}/chrome156-observation.json`, "utf8"),
]);
const expected = JSON.parse(expectedText);
const envelope = JSON.parse(observationText);
const observation = envelope?.data?.result ?? envelope;
const tolerance = expected.frameToleranceCssPx;

assert.equal(createHash("sha256").update(fixture).digest("hex"), expected.fixtureSha256);
if (Object.hasOwn(envelope, "success")) assert.equal(envelope.success, true);
assert.match(observation?.userAgent ?? "", /HeadlessChrome\/156\./);
assert.equal(observation?.url, `file://${process.cwd()}/${fixturePath}`);

function number(value) {
  if (typeof value === "number") return value;
  const [numerator, denominator] = value.split("/").map(Number);
  assert.ok(Number.isFinite(numerator) && Number.isFinite(denominator) && denominator !== 0);
  return numerator / denominator;
}

function near(actual, wanted, label) {
  assert.ok(
    Number.isFinite(actual) && Math.abs(actual - number(wanted)) <= tolerance,
    `${label}: got ${actual}, expected ${wanted} ± ${tolerance}`,
  );
}

const scenes = new Map(observation.scenes.map((scene) => [scene.id, scene]));
assert.equal(scenes.size, 12);
for (const expectedScene of expected.positive) {
  const scene = scenes.get(expectedScene.scene);
  assert.ok(scene, `missing ${expectedScene.scene}`);
  assert.deepEqual(scene.rootSize, expectedScene.rootSize, `${expectedScene.scene} root`);
  assert.equal(scene.computedGap, expectedScene.computedGap, `${expectedScene.scene} gap`);
  for (const [axis, serialized] of [
    ["columns", scene.computedColumns],
    ["rows", scene.computedRows],
  ]) {
    const actual = serialized.split(/\s+/).map((part) => Number.parseFloat(part));
    const wanted = expectedScene.tracks[axis];
    assert.equal(actual.length, wanted.length, `${expectedScene.scene} ${axis} count`);
    wanted.forEach((value, index) => near(actual[index], value, `${expectedScene.scene} ${axis}[${index}]`));
  }
  assert.equal(scene.items.length, expectedScene.items.length, `${expectedScene.scene} item count`);
  expectedScene.items.forEach((frame, itemIndex) => {
    frame.forEach((value, fieldIndex) => {
      near(scene.items[itemIndex].frame[fieldIndex], value, `${expectedScene.scene} item[${itemIndex}][${fieldIndex}]`);
    });
  });
}
for (const divergence of expected.browserDivergences) {
  const scene = scenes.get(divergence.scene);
  assert.ok(scene, `missing browser divergence ${divergence.scene}`);
  const chromeColumns = scene.computedColumns.split(/\s+/).map(Number.parseFloat);
  const chromeRows = scene.computedRows.split(/\s+/).map(Number.parseFloat);
  assert.deepEqual(chromeColumns, divergence.chromeTracks.columns, `${divergence.scene} Chrome columns`);
  assert.deepEqual(chromeRows, divergence.chromeTracks.rows, `${divergence.scene} Chrome rows`);
  assert.equal(scene.items.length, divergence.chromeItems.length, `${divergence.scene} item count`);
  divergence.chromeItems.forEach((frame, itemIndex) => {
    frame.forEach((value, fieldIndex) => {
      near(scene.items[itemIndex].frame[fieldIndex], value, `${divergence.scene} Chrome item[${itemIndex}][${fieldIndex}]`);
    });
  });
  const flexFactors = Array(chromeColumns.length).fill(Math.fround(1e38));
  let f32FactorSum = Math.fround(0);
  for (const factor of flexFactors) f32FactorSum = Math.fround(f32FactorSum + factor);
  assert.equal(f32FactorSum, Infinity, `${divergence.scene} must overflow f32 accumulation`);
  const specFactorSum = flexFactors.reduce((sum, factor) => sum + factor, 0);
  const specTracks = flexFactors.map((factor) => (scene.rootSize[0] / specFactorSum) * factor);
  divergence.specTracks.columns.forEach((value, index) => near(specTracks[index], value, `${divergence.scene} spec column[${index}]`));
  assert.notDeepEqual(chromeColumns, divergence.specTracks.columns, `${divergence.scene} must retain the recorded spec/browser divergence`);
}
for (const expectedScene of expected.runtimeNegativeControls) {
  const scene = scenes.get(expectedScene.scene);
  assert.ok(scene, `missing negative control ${expectedScene.scene}`);
  assert.deepEqual(
    scene.computedColumns.split(/\s+/).map(Number.parseFloat),
    expectedScene.chromeTracks.columns,
    `${expectedScene.scene} Chrome columns`,
  );
  assert.deepEqual(
    scene.computedRows.split(/\s+/).map(Number.parseFloat),
    expectedScene.chromeTracks.rows,
    `${expectedScene.scene} Chrome rows`,
  );
}
console.log(JSON.stringify({
  status: "passed",
  positiveScenes: expected.positive.length,
  browserDivergences: expected.browserDivergences.length,
  negativeControls: expected.runtimeNegativeControls.length,
  frameToleranceCssPx: tolerance,
  browser: observation.userAgent,
}));
