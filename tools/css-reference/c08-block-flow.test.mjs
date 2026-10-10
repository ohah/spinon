import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const referencePath = join(repositoryRoot, 'tests/fixtures/css/references/c08-block-flow-v1.json');
const inventoryPath = join(repositoryRoot, 'tests/fixtures/css/c08/block-flow-inventory.json');
const htmlPath = join(repositoryRoot, 'tests/fixtures/css/c08/block-flow.html');
const runtimePath = join(repositoryRoot, 'tests/fixtures/css/c08/runtime-block-paint.js');
const capturePath = join(repositoryRoot, 'tools/css-reference/capture-c08-block-flow.mjs');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('C08 고정 기준과 런타임 fixture의 해시·입력 계약을 검증한다', async () => {
  const [referenceBytes, inventoryBytes, htmlBytes, runtimeBytes, captureBytes] = await Promise.all([
    readFile(referencePath),
    readFile(inventoryPath),
    readFile(htmlPath),
    readFile(runtimePath),
    readFile(capturePath),
  ]);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  assert.equal(reference.schema, 'spinon-css-c08-block-flow-reference/v1');
  assert.equal(reference.fixture.id, 'C08-block-flow-v1');
  assert.equal(reference.fixture.inventorySha256, hash(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, hash(htmlBytes));
  assert.equal(reference.fixture.runtimeSourceSha256, hash(runtimeBytes));
  assert.equal(reference.captureTool.sha256, hash(captureBytes));
  assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
  assert.equal(reference.observations.length, 2);

  const expectedIds = ['root', 'hero', 'hero-child', 'group', 'first', 'second', 'hidden', 'hidden-child', 'last'];
  const expectedFrames = [
    [0, 0, 280, 86],
    [0, 0, 280, 32],
    [0, 0, 280, 10],
    [0, 32, 280, 38],
    [0, 32, 280, 20],
    [0, 52, 120, 18],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 70, 280, 16],
  ];
  for (const [index, observation] of reference.observations.entries()) {
    assert.equal(observation.viewport.deviceScaleFactor, index + 1);
    assert.deepEqual(observation.nodes.map(({ id }) => id), expectedIds);
    assert.deepEqual(
      observation.nodes.map(({ rect }) => [rect.x, rect.y, rect.width, rect.height]),
      expectedFrames,
      `DPR ${index + 1}`,
    );
    assert.equal(observation.nodes[0].properties.display, 'block');
    assert.equal(observation.nodes[2].properties.color, observation.nodes[1].properties.color);
    assert.equal(observation.nodes[2].properties['font-size'], observation.nodes[1].properties['font-size']);
    assert.equal(observation.nodes[2].properties['font-family'], observation.nodes[1].properties['font-family']);
  }
  assert.deepEqual(reference.observations[0].nodes, reference.observations[1].nodes);
});
