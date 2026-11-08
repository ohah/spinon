import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const evidenceDir = new URL("./", import.meta.url);
const expected = JSON.parse(readFileSync(new URL("expected.json", evidenceDir), "utf8"));
const observed = JSON.parse(readFileSync(new URL("chrome-observation.json", evidenceDir), "utf8"));

assert.equal(observed.length, expected.scenes.length, "scene count");

for (const sceneExpected of expected.scenes) {
  const scene = observed.find(({ id }) => id === sceneExpected.id);
  assert.ok(scene, `missing scene ${sceneExpected.id}`);
  assert.equal(scene.cap, sceneExpected.cap, `${scene.id}: cap label`);
  assert.equal(scene.alignContent, sceneExpected.alignContent, `${scene.id}: align-content`);
  assert.equal(scene.grid.width, expected.contract.gridWidth, `${scene.id}: grid width`);
  assert.equal(scene.grid.height, expected.contract.gridHeight, `${scene.id}: grid height`);

  const expectedTrackList = sceneExpected.cap === "20px"
    ? expected.contract.trackListPx
    : expected.contract.trackListPercent;
  assert.equal(scene.gridAutoRows, expectedTrackList, `${scene.id}: computed grid-auto-rows`);

  const rows = scene.gridTemplateRows.split(" ").map((value) => Number.parseFloat(value));
  assert.deepEqual(rows, sceneExpected.rows, `${scene.id}: used track sizes`);
  assert.equal(scene.items.length, sceneExpected.rows.length, `${scene.id}: item count`);
  assert.equal(rows.reduce((sum, value) => sum + value, 0) + sceneExpected.freeSpace, 100,
    `${scene.id}: free-space accounting`);

  for (let index = 0; index < scene.items.length; index += 1) {
    const item = scene.items[index];
    const itemExpectedOffset = sceneExpected.itemOffsets[index];
    const itemExpectedSize = sceneExpected.itemSizes[index];
    assert.equal(item.index, index, `${scene.id}: item order ${index}`);
    assert.equal(item.rect.x - scene.grid.x, 0, `${scene.id}: item ${index} x`);
    assert.equal(item.rect.y - scene.grid.y, itemExpectedOffset, `${scene.id}: item ${index} y`);
    assert.equal(item.rect.width, expected.contract.gridWidth, `${scene.id}: item ${index} width`);
    assert.equal(item.rect.height, itemExpectedSize, `${scene.id}: item ${index} height`);

    if (index === 0) {
      assert.equal(item.payloadRect, null, `${scene.id}: explicit-row item payload`);
      continue;
    }

    assert.ok(item.payloadRect, `${scene.id}: missing payload ${index}`);
    assert.equal(item.payloadRect.x - scene.grid.x, 0, `${scene.id}: payload ${index} x`);
    assert.equal(item.payloadRect.y - scene.grid.y, itemExpectedOffset, `${scene.id}: payload ${index} y`);
    assert.equal(item.payloadRect.width, expected.contract.gridWidth, `${scene.id}: payload ${index} width`);
    assert.equal(item.payloadRect.height, expected.contract.payloadHeights[index - 1],
      `${scene.id}: payload ${index} height`);
  }

  const overflowItem = scene.items[7];
  assert.equal(overflowItem.rect.height, 20, `${scene.id}: repeated fit-content row cap`);
  assert.equal(overflowItem.payloadRect.bottom - scene.grid.bottom,
    sceneExpected.overflowPastGridBottom, `${scene.id}: overflow past grid border`);
}

console.log(`Chrome 157 repeated intrinsic implicit-row matrix: ${observed.length}/${expected.scenes.length} exact`);
