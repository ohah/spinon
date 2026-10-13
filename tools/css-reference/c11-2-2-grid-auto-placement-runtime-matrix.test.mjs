import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { buildC1122GridAutoPlacementRuntimeMatrix } from './build-c11-2-2-grid-auto-placement-runtime-matrix.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const matrixPath = 'tests/fixtures/css/c11/grid-auto-placement-runtime-matrix.json';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('C11.2.2 runtime matrix pins all candidate and expected-rejection scenes', async () => {
  const matrix = await buildC1122GridAutoPlacementRuntimeMatrix();
  const generator = await readFile(resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-auto-placement-runtime-matrix.mjs'));
  matrix.inputs.generator.sha256 = sha256(generator);

  assert.deepEqual(matrix.summary, {
    sceneCount: 16,
    candidateCount: 11,
    expectedRejectCount: 5,
    nodeCount: 69,
  });
  assert.equal(matrix.cases.filter(({ scope }) => scope === 'implementation-candidate').length, 11);
  assert.equal(matrix.cases.filter(({ scope }) => scope === 'chromium-oracle-only').length, 5);
  for (const scene of matrix.cases) {
    assert.equal(scene.nodeIds[0], scene.rootId, `${scene.id}: root ID`);
    assert.equal(scene.nodeIds.length, scene.parentIds.length, `${scene.id}: parent 수`);
    assert.equal(sha256(scene.source), scene.sourceSha256, `${scene.id}: source hash`);
    assert.ok(scene.source.includes(`sceneId:${JSON.stringify(scene.id)}`), `${scene.id}: 고정 source ID`);
    assert.ok(scene.source.includes('background-color:'), `${scene.id}: paint color`);
    if (scene.scope === 'chromium-oracle-only') {
      assert.equal(scene.expected.outcome, 'expected-rejection', `${scene.id}: 거부 기대값`);
      assert.ok(scene.expected.code);
      assert.ok(scene.expected.property);
      assert.ok(scene.expected.runtimeNodeId >= 2);
    } else {
      assert.deepEqual(scene.expected, { outcome: 'success' }, `${scene.id}: 성공 기대값`);
    }
  }

  const checkedIn = await readFile(resolve(repositoryRoot, matrixPath), 'utf8');
  assert.equal(checkedIn, `${JSON.stringify(matrix, null, 2)}\n`, `${matrixPath}가 생성기 결과와 다릅니다.`);
});
