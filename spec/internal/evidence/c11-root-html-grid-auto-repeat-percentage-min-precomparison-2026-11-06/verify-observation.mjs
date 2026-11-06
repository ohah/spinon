import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const evidenceRoot = new URL("./", import.meta.url);
const expected = JSON.parse(
  readFileSync(new URL("expected.json", evidenceRoot), "utf8"),
);
const observed = JSON.parse(
  readFileSync(new URL("chrome157-observation.json", evidenceRoot), "utf8"),
);
const tolerance = 0.02;
const close = (actual, value, context) => {
  assert.ok(
    Number.isFinite(actual) && Math.abs(actual - value) <= tolerance,
    `${context}: expected ${value} ± ${tolerance}, received ${actual}`,
  );
};
const parseTracks = (value) => value.trim().split(/\s+/).map(Number.parseFloat);

assert.match(observed.userAgent, /HeadlessChrome\/157\.0\.0\.0/);
assert.equal(observed.supportsPercentageMin, true);
assert.equal(observed.observations.length, expected.scenes.length);
for (const scene of expected.scenes) {
  const actual = observed.observations.find((candidate) => candidate.id === scene.id);
  assert.ok(actual, `missing scene ${scene.id}`);
  const trackText = scene.basis.axis === "width" ? actual.computedColumns : actual.computedRows;
  const usedTracks = parseTracks(trackText);
  assert.equal(usedTracks.length, scene.tracks.length, `${scene.id} track count`);
  assert.equal(
    usedTracks.length,
    scene.repeatCount * scene.basis.patternTracks,
    `${scene.id} expanded repeat count`,
  );
  scene.tracks.forEach((value, index) => close(usedTracks[index], value, `${scene.id} track ${index}`));
  assert.equal(actual.items.length, scene.items.length, `${scene.id} item count`);
  scene.items.forEach((expectedItem, index) => {
    const actualItem = actual.items[index];
    assert.equal(actualItem.index, expectedItem.index, `${scene.id} source order ${index}`);
    for (const property of ["x", "y", "width", "height"]) {
      close(actualItem[property], expectedItem[property], `${scene.id} item ${index} ${property}`);
    }
  });
  if (scene.overflow) {
    const overflow = Math.max(0, usedTracks.reduce((sum, value) => sum + value, 0) - scene.basis.size);
    close(overflow, scene.overflow.cssPixels, `${scene.id} visible ${scene.overflow.axis} overflow`);
  }
}
process.stdout.write(`Chrome ${observed.chrome}: ${expected.scenes.length}/${expected.scenes.length} scenes match independent expectations\n`);
