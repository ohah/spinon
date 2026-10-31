import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { buildC1122GridItemOrderRuntimeMatrix } from './build-c11-2-2-grid-item-order-runtime-matrix.mjs';
import { buildC1122GridItemOrderPixelMatrix } from './build-c11-2-2-grid-item-order-pixel-matrix.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const matrixPath = 'tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('C11.2.2 order runtime matrix fixes signed order and stable ties independently', async () => {
  const matrix = await buildC1122GridItemOrderRuntimeMatrix();
  const generator = await readFile(resolve(repositoryRoot,
    'tools/css-reference/build-c11-2-2-grid-item-order-runtime-matrix.mjs'));
  matrix.inputs.generator.sha256 = sha256(generator);

  assert.equal(matrix.schema, 'spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1');
  assert.equal(matrix.profileId, 'RuntimeBlockFlexGridAutoPlacementV1');
  assert.deepEqual(matrix.summary, { sceneCount: 2, nodeCount: 9, itemCount: 7 });
  assert.deepEqual(matrix.cases.map(({ id }) => id), ['order-mixed', 'order-ties']);
  for (const fixtureCase of matrix.cases) {
    assert.equal(fixtureCase.nodeIds[0], fixtureCase.rootId);
    assert.equal(fixtureCase.nodeIds.length, fixtureCase.parentIds.length);
    assert.equal(new Set(fixtureCase.nodeIds).size, fixtureCase.nodeIds.length);
    assert.equal(sha256(fixtureCase.source), fixtureCase.sourceSha256);
    assert.ok(fixtureCase.source.includes(`sceneId:${JSON.stringify(fixtureCase.id)}`));
    assert.deepEqual(fixtureCase.expected, { outcome: 'success' });
    assert.equal(fixtureCase.expectedRects.length, fixtureCase.nodeIds.length);
    assert.equal(fixtureCase.domItemOrder.length, fixtureCase.nodeIds.length - 1);
  }
  assert.deepEqual(matrix.cases[0].expectedRects.slice(1).map(({ rect }) => [rect.x, rect.y]),
    [[20, 0], [0, 0], [10, 0]]);
  assert.deepEqual(matrix.cases[1].expectedRects.slice(1).map(({ rect }) => [rect.x, rect.y]),
    [[20, 20], [0, 20], [10, 20], [0, 30]]);
  assert.deepEqual(matrix.cases[1].domItemOrder, [
    'c1122-order-ties-a', 'c1122-order-ties-b', 'c1122-order-ties-c', 'c1122-order-ties-d',
  ]);

  const checkedIn = await readFile(resolve(repositoryRoot, matrixPath), 'utf8');
  assert.equal(checkedIn, `${JSON.stringify(matrix, null, 2)}\n`, `${matrixPath}가 생성기 결과와 다릅니다.`);
});

test('C11.2.2 order surface pixel matrix samples each non-overlapping item interior', async () => {
  const matrix = await buildC1122GridItemOrderPixelMatrix();
  const generator = await readFile(resolve(repositoryRoot,
    'tools/css-reference/build-c11-2-2-grid-item-order-pixel-matrix.mjs'));
  matrix.inputs.generator.sha256 = sha256(generator);

  assert.equal(matrix.schema, 'spinon-css-c11-2-2-grid-item-order-mobile-surface-pixel-matrix/v1');
  assert.deepEqual(matrix.summary, { sceneCount: 2, sampleCount: 7 });
  for (const fixtureCase of matrix.cases) {
    assert.equal(fixtureCase.samples.length, fixtureCase.nodeIds.length - 1);
    assert.equal(new Set(fixtureCase.samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
      fixtureCase.samples.length);
    for (const sample of fixtureCase.samples) {
      assert.equal(sample.rgba.length, 4);
      assert.ok(sample.rgba.every((channel) => Number.isInteger(channel) && channel >= 0 && channel <= 255));
      assert.ok(sample.xCssPx < matrix.runtime.viewportCssPx.width);
      assert.ok(sample.yCssPx < matrix.runtime.viewportCssPx.height);
    }
  }
  const outputPath = 'tests/fixtures/css/c11/grid-item-order-mobile-surface-pixel-matrix-v1.json';
  const checkedIn = await readFile(resolve(repositoryRoot, outputPath), 'utf8');
  assert.equal(checkedIn, `${JSON.stringify(matrix, null, 2)}\n`, `${outputPath}가 생성기 결과와 다릅니다.`);
});
