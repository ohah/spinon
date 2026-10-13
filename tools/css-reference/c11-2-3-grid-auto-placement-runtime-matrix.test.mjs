import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { resolve } from 'node:path';
import { buildC1123GridAutoPlacementRuntimeMatrix } from './build-c11-2-3-grid-auto-placement-runtime-matrix.mjs';
import { loadC1123Inputs } from './c11-2-3-grid-auto-placement-runtime-support.mjs';

const matrixPath = resolve('tests/fixtures/css/c11/grid-auto-placement-c1123-runtime-matrix-v1.json');

test('runtime matrix 재생성이 고정 v6 inventory와 Chrome 154 v7 근거에 일치한다', async () => {
  const generated = await buildC1123GridAutoPlacementRuntimeMatrix();
  const saved = await readFile(matrixPath, 'utf8');
  assert.equal(saved, `${JSON.stringify(generated, null, 2)}\n`);
});

test('capture 입력은 runtime matrix, HTML, inventory, Chromium reference hash를 모두 묶는다', async () => {
  const inputs = await loadC1123Inputs();
  assert.equal(inputs.hashes.html, inputs.matrix.inputs.html.sha256);
  assert.equal(inputs.reference.fixture.htmlSha256, inputs.hashes.html);
  assert.equal(inputs.reference.fixture.htmlPath, inputs.matrix.inputs.html.path);
});

test('고정 행렬은 25개 scene의 19 성공과 6 기대 거부 계약을 보존한다', async () => {
  const matrix = await buildC1123GridAutoPlacementRuntimeMatrix();
  assert.equal(matrix.profileId, 'RuntimeBlockFlexGridAutoPlacementDenseV1');
  assert.deepEqual(matrix.summary, {
    sceneCount: 25,
    implementationCandidateCount: 21,
    oracleOnlyCount: 4,
    successCount: 19,
    expectedRejectCount: 6,
    nodeCount: 107,
  });
  assert.deepEqual(
    matrix.cases.filter(({ expected }) => expected.outcome === 'expected-rejection').map(({ id }) => id),
    [
      'row-dense-fixed-row',
      'reject-repeated-auto-track',
      'reject-negative-implicit-line',
      'reject-nonzero-order',
      'reject-indefinite-axis',
      'auto-span-over-track-limit',
    ],
  );
});

test('runtime JavaScript는 고정된 scene node와 parent graph만 구성한다', async () => {
  const matrix = await buildC1123GridAutoPlacementRuntimeMatrix();
  for (const scene of matrix.cases) {
    assert.equal(scene.index >= 0, true);
    assert.equal(scene.source.length <= 64 * 1024, true);
    assert.equal(scene.sourceSha256.length, 64);
    assert.equal(scene.source.includes(`sceneId:${JSON.stringify(scene.id)}`), true);
    assert.deepEqual(scene.nodeIds.length, scene.parentIds.length);
    assert.equal(new Set(scene.nodeIds).size, scene.nodeIds.length);
    for (const [index, parentId] of scene.parentIds.entries()) {
      if (parentId !== null) assert.equal(scene.nodeIds.slice(0, index).includes(parentId), true);
    }
  }
});

test('runtime paint 색상은 Chrome reference fixture HTML의 node별 palette를 그대로 따른다', async () => {
  const matrix = await buildC1123GridAutoPlacementRuntimeMatrix();
  assert.equal(matrix.inputs.html.path, 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html');
  assert.equal(matrix.inputs.html.sha256.length, 64);
  const palette = new Map([
    ['blue', '#2866f6'], ['red', '#e5484d'], ['green', '#19a974'],
    ['yellow', '#ffc857'], ['purple', '#8758ff'], ['cyan', '#0ea5e9'],
    ['orange', '#f97316'], ['pink', '#ec4899'], ['surface', '#edf1f7'],
  ]);
  const html = await readFile(resolve('tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html'), 'utf8');
  for (const scene of matrix.cases) {
    const sourceLines = scene.source.split('\n');
    for (const node of scene.nodeIds) {
      const match = html.match(new RegExp(`<[^>]*\\bid="${node}"[^>]*\\bclass="([^"]+)"`));
      assert.ok(match, `${scene.id}/${node}: HTML fixture node 없음`);
      const colorClass = match[1].split(/\s+/).find((name) => palette.has(name));
      assert.ok(colorClass, `${scene.id}/${node}: HTML fixture color class 없음`);
      const idLine = sourceLines.findIndex((line) =>
        line === `node_${scene.nodeIds.indexOf(node)}.setAttribute("id", ${JSON.stringify(node)});`);
      assert.ok(idLine >= 0, `${scene.id}/${node}: runtime source ID 설정 없음`);
      const styleMatch = sourceLines[idLine + 1].match(/\.setAttribute\("style", (".*")\);$/);
      assert.ok(styleMatch, `${scene.id}/${node}: runtime source style 설정 없음`);
      assert.match(JSON.parse(styleMatch[1]),
        new RegExp(`(?:^|;)background-color:${palette.get(colorClass)}(?:;|$)`),
        `${scene.id}/${node}: HTML palette 색상이 runtime source에 없습니다.`);
    }
  }
  assert.match(matrix.cases[0].source, /background-color:#2866f6/);
  assert.doesNotMatch(matrix.cases[0].source, /#3678f6|#f08a35/);
});
