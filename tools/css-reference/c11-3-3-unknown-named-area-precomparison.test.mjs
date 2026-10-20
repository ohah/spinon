import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

const inventoryPath = 'tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133-inventory.json';
const htmlPath = 'tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html';
const referencePath = 'tests/fixtures/css/references/c11-3-3-unknown-named-area-implicit-lines-precomparison-v1.json';
const [inventoryBytes, htmlBytes, referenceBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(referencePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('C11.3.3 unknown named-area Chrome baseline keeps its source and geometry', async () => {
  assert.equal(reference.schema, 'spinon-css-c11-3-3-unknown-named-area-implicit-lines-precomparison/v1');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.92');
  assert.equal(reference.oracle.revision, '@334b65d254ccc35df4fca82706d1753227b01039');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.wpt.cases[0].sha256,
    '7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4');
  assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  assert.equal(reference.observations[0].cases.length, inventory.cases.length);
  for (let index = 0; index < inventory.cases.length; index += 1) {
    const fixtureCase = inventory.cases[index];
    const observation = reference.observations[0].cases[index];
    assert.equal(observation.id, fixtureCase.id);
    assert.deepEqual(observation.nodes.map(({ id }) => id), fixtureCase.nodeIds);
    assert.deepEqual(observation.resolvedGrid.rows, fixtureCase.expectedRows);
    assert.deepEqual(observation.resolvedGrid.columns, fixtureCase.expectedColumns);
    assert.deepEqual(observation.nodes.at(-1).rect, fixtureCase.expectedRect);
  }
  assert.equal(reference.screenshots.length, inventory.cases.length * 2);
  for (const screenshot of reference.screenshots) {
    const bytes = await readFile(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, `${screenshot.path}: PNG SHA-256`);
    assert.deepEqual(bytes.subarray(0, 8), Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]));
    assert.equal(bytes.readUInt32BE(16), inventory.viewport.width * screenshot.deviceScaleFactor);
    assert.equal(bytes.readUInt32BE(20), inventory.viewport.height * screenshot.deviceScaleFactor);
  }
});
