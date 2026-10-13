import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { buildC1122GridAutoPlacementPixelMatrix } from './build-c11-2-2-grid-auto-placement-pixel-matrix.mjs';
import { loadC1122Inputs, loadC1122PixelMatrix } from './c11-2-2-grid-auto-placement-runtime-support.mjs';

const root = resolve(import.meta.dirname, '../..');

test('pinned Chrome 154 screenshot에서 11개 성공 장면의 item/gap 표본을 만든다', async () => {
  const matrix = await buildC1122GridAutoPlacementPixelMatrix();
  assert.equal(matrix.schema, 'spinon-css-c11-2-2-grid-auto-placement-mobile-surface-pixel-matrix/v1');
  assert.deepEqual(matrix.runtime.viewportCssPx, { width: 320, height: 240 });
  assert.equal(matrix.summary.sceneCount, 16);
  assert.equal(matrix.summary.pixelSceneCount, 11);
  assert.equal(matrix.summary.expectedRejectCount, 5);
  assert.equal(matrix.summary.sampleCount, 51);
  for (const fixtureCase of matrix.cases) {
    if (fixtureCase.outcome === 'expected-rejection') {
      assert.deepEqual(fixtureCase.samples ?? [], []);
      continue;
    }
    assert.ok(fixtureCase.samples.some(({ kind }) => kind.endsWith('gap')),
      `${fixtureCase.id}: gap 표본이 없습니다.`);
    assert.ok(fixtureCase.samples.every(({ xCssPx, yCssPx }) =>
      xCssPx >= 0 && xCssPx < 320 && yCssPx >= 0 && yCssPx < 240));
    assert.ok(fixtureCase.samples.length <= 64);
    assert.equal(new Set(fixtureCase.samples.map(({ id }) => id)).size, fixtureCase.samples.length);
    assert.equal(new Set(fixtureCase.samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
      fixtureCase.samples.length);
  }
});

test('실행 입력의 고정 hash와 11개 성공·5개 거부 장면이 표면 행렬에서 일치한다', async () => {
  const inputs = await loadC1122Inputs();
  const { matrix } = await loadC1122PixelMatrix(inputs);
  assert.deepEqual(matrix.cases.map(({ id }) => id), inputs.matrix.cases.map(({ id }) => id));
  assert.equal(matrix.cases.filter(({ samples }) => (samples ?? []).length > 0).length, 11);
  assert.equal(matrix.cases.filter(({ samples }) => (samples ?? []).length === 0).length, 5);
});

test('hidden item은 표면 표본으로 취급하지 않는다', async () => {
  const matrix = await buildC1122GridAutoPlacementPixelMatrix();
  const scene = matrix.cases.find(({ id }) => id === 'hidden-child-excluded');
  assert.ok(scene);
  assert.equal(scene.samples.some(({ nodeId }) => nodeId === 'c1122-hidden-item'), false);
  assert.ok(scene.samples.some(({ kind }) => kind === 'column-gap'));
});

test('source inventory hash가 어긋나면 Pixel matrix 생성이 실패한다', async () => {
  const path = 'tests/fixtures/css/c11/grid-auto-placement-inventory-v3.json';
  const bytes = await readFile(resolve(root, path));
  const changed = Buffer.from(bytes);
  changed[changed.length - 2] ^= 1;
  await assert.rejects(buildC1122GridAutoPlacementPixelMatrix({ [path]: changed }));
});

test('고정 HTML hash가 어긋나면 Pixel matrix 생성이 실패한다', async () => {
  const path = 'tests/fixtures/css/c11/grid-auto-placement-v3.html';
  const bytes = await readFile(resolve(root, path));
  await assert.rejects(buildC1122GridAutoPlacementPixelMatrix({
    [path]: Buffer.concat([bytes, Buffer.from('\n<!-- mutation -->')]),
  }));
});

test('Chrome oracle hash가 어긋나면 Pixel matrix 생성이 실패한다', async () => {
  const path = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json';
  const bytes = await readFile(resolve(root, path));
  await assert.rejects(buildC1122GridAutoPlacementPixelMatrix({
    [path]: Buffer.concat([bytes, Buffer.from(' ')]),
  }));
});

test('Chrome DPR 1 screenshot이 변경되면 source oracle 판정에서 거부한다', async () => {
  const referencePath = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json';
  const referenceBytes = await readFile(resolve(root, referencePath));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const screenshotPath = reference.screenshots[0].path;
  const screenshotBytes = await readFile(resolve(root, screenshotPath));
  const changed = Buffer.from(screenshotBytes);
  changed[changed.length - 1] ^= 1;
  await assert.rejects(buildC1122GridAutoPlacementPixelMatrix({ [screenshotPath]: changed }));
});

test('reference에서 scene 순서가 빠지면 fixture 전체를 생성하지 않는다', async () => {
  const path = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json';
  const bytes = await readFile(resolve(root, path));
  const reference = JSON.parse(bytes.toString('utf8'));
  reference.observations[0].cases.pop();
  await assert.rejects(buildC1122GridAutoPlacementPixelMatrix({
    [path]: Buffer.from(JSON.stringify(reference)),
  }));
});

test('생성된 sample RGBA는 고정 screenshot 실제 pixel과 일치한다', async () => {
  const matrix = await buildC1122GridAutoPlacementPixelMatrix();
  for (const fixtureCase of matrix.cases.filter(({ outcome }) => outcome === 'surface-pixel-compare')) {
    const path = fixtureCase.chromeScreenshot.path;
    const png = await readFile(resolve(root, path));
    const { decodeRgbaPng } = await import('./read-png-rgba.mjs');
    const image = decodeRgbaPng(png);
    for (const sample of fixtureCase.samples) {
      assert.deepEqual(image.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba,
        `${fixtureCase.id}/${sample.id}`);
    }
  }
});
