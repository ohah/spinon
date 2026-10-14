import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const referencePath = 'tests/fixtures/css/references/c14-replaced-auto-size-v1.json';
const inventoryPath = 'tests/fixtures/css/c14/replaced-auto-size-inventory.json';
const htmlPath = 'tests/fixtures/css/c14/replaced-auto-size.html';
const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [referenceBytes, inventoryBytes, htmlBytes] = await Promise.all([
  read(referencePath),
  read(inventoryPath),
  read(htmlPath),
]);
const reference = JSON.parse(referenceBytes.toString('utf8'));
const inventory = JSON.parse(inventoryBytes.toString('utf8'));

test('C14.1 Chrome reference는 HTML, inventory, screenshot 원본에 고정돼 있다', async () => {
  assert.equal(reference.schema, 'spinon-css-c14-replaced-auto-size-reference/v1');
  assert.equal(inventory.schema, 'spinon-css-c14-replaced-auto-size-inventory/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, inventoryPath);
  assert.equal(reference.fixture.htmlPath, htmlPath);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.oracle.product, 'Chrome/155.0.8059.40');
  assert.deepEqual(reference.environment.deviceScaleFactor, 1);
  assert.equal(reference.comparison.dprInvariant, false);

  for (const screenshot of reference.screenshots) {
    assert.equal(sha256(await read(screenshot.path)), screenshot.sha256, screenshot.path);
  }
});

test('C14.1 관찰값은 inventory의 세 replaced-image 경계를 보존한다', () => {
  assert.deepEqual(
    reference.observations.map(({ id }) => id),
    inventory.nodes.map(({ id }) => id),
  );
  const byId = new Map(reference.observations.map((observation) => [observation.id, observation]));
  const expectedRects = new Map([
    ['image', { x: 0, y: 0, width: 24, height: 16 }],
    ['sized-width', { x: 0, y: 16, width: 48, height: 32 }],
    ['sized-height', { x: 0, y: 48, width: 36, height: 24 }],
  ]);

  for (const node of inventory.nodes) {
    const observation = byId.get(node.id);
    assert.ok(observation, `reference에 ${node.id} 관찰값이 있어야 합니다`);
    assert.equal(observation.complete, true, `${node.id} image decode`);
    assert.equal(observation.naturalWidth, node.naturalWidth, `${node.id} naturalWidth`);
    assert.equal(observation.naturalHeight, node.naturalHeight, `${node.id} naturalHeight`);
    assert.deepEqual(observation.rect, expectedRects.get(node.id), `${node.id} frame`);
  }
});
