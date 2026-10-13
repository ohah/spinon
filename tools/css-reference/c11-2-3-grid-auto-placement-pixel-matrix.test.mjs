import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { buildC1123GridAutoPlacementPixelMatrix } from './build-c11-2-3-grid-auto-placement-pixel-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = resolve(import.meta.dirname, '../..');
const matrixPath = 'tests/fixtures/css/c11/grid-auto-placement-c1123-mobile-surface-pixel-matrix.json';

test('Chrome 154 고정 screenshot에서 C11.2.3 성공 19 scene의 표본 행렬을 결정적으로 만든다', async () => {
  const generated = await buildC1123GridAutoPlacementPixelMatrix();
  const stored = JSON.parse((await readFile(resolve(root, matrixPath))).toString('utf8'));
  assert.deepEqual(generated, stored);
  assert.equal(stored.schema, 'spinon-css-c11-2-3-grid-auto-placement-mobile-surface-pixel-matrix/v1');
  assert.deepEqual(stored.runtime.viewportCssPx, { width: 320, height: 240 });
  assert.deepEqual(stored.summary, {
    sceneCount: 25,
    pixelSceneCount: 19,
    expectedRejectCount: 6,
    sampleCount: 99,
    hiddenNodesAreNotSampled: true,
    screenshotsArePixelParity: false,
  });
  for (const fixtureCase of stored.cases) {
    if (fixtureCase.outcome === 'expected-rejection') {
      assert.deepEqual(fixtureCase.samples ?? [], []);
      continue;
    }
    assert.ok(fixtureCase.samples.some(({ kind }) => kind === 'item'),
      `${fixtureCase.id}: visible item 표본이 없습니다.`);
    assert.ok(fixtureCase.samples.length >= 2 && fixtureCase.samples.length <= 64);
    assert.equal(new Set(fixtureCase.samples.map(({ id }) => id)).size, fixtureCase.samples.length);
    assert.equal(new Set(fixtureCase.samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
      fixtureCase.samples.length);
    assert.ok(fixtureCase.samples.every(({ xCssPx, yCssPx }) =>
      xCssPx >= 0 && xCssPx < 320 && yCssPx >= 0 && yCssPx < 240));
  }
});

test('거부 6 scene은 pixel sample을 만들지 않고 hidden child도 제외한다', async () => {
  const matrix = await buildC1123GridAutoPlacementPixelMatrix();
  assert.equal(matrix.cases.filter(({ outcome }) => outcome === 'expected-rejection').length, 6);
  for (const fixtureCase of matrix.cases.filter(({ outcome }) => outcome === 'expected-rejection')) {
    assert.deepEqual(fixtureCase.samples ?? [], [], fixtureCase.id);
  }
  const hidden = matrix.cases.find(({ id }) => id === 'hidden-item-excluded');
  assert.ok(hidden);
  assert.equal(hidden.samples.some(({ nodeId }) => nodeId.endsWith('-hidden')), false);
});

test('track limit 장면의 1px 높이 item은 명시된 단일 pixel 예외로 기록한다', async () => {
  const matrix = await buildC1123GridAutoPlacementPixelMatrix();
  const scene = matrix.cases.find(({ id }) => id === 'auto-span-at-track-limit');
  const thin = scene.samples.find(({ sampleFootprint }) => sampleFootprint === 'single-pixel-thin-box');
  assert.deepEqual([thin.xCssPx, thin.yCssPx], [319, 20]);
  assert.deepEqual(thin.rgba, [40, 102, 246, 255]);
});

test('표본 RGBA가 모든 고정 Chrome PNG 픽셀과 일치한다', async () => {
  const matrix = await buildC1123GridAutoPlacementPixelMatrix();
  for (const fixtureCase of matrix.cases.filter(({ outcome }) => outcome === 'surface-pixel-compare')) {
    const png = await readFile(resolve(root, fixtureCase.chromeScreenshot.path));
    const image = decodeRgbaPng(png);
    for (const sample of fixtureCase.samples) {
      assert.deepEqual(image.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba,
        `${fixtureCase.id}/${sample.id}`);
    }
  }
});

test('inventory, HTML, Chrome reference, screenshot hash가 바뀌면 matrix 생성이 실패한다', async () => {
  const mutations = [
    ['tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json', null],
    ['tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html', null],
    ['tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json', null],
  ];
  for (const [path] of mutations) {
    const bytes = await readFile(resolve(root, path));
    await assert.rejects(buildC1123GridAutoPlacementPixelMatrix({
      [path]: Buffer.concat([bytes, Buffer.from(' ')]),
    }), path);
  }
  const referencePath = mutations[2][0];
  const reference = JSON.parse((await readFile(resolve(root, referencePath))).toString('utf8'));
  const screenshotPath = reference.screenshots.find(({ caseId }) => caseId === 'row-sparse-baseline').path;
  const screenshot = Buffer.from(await readFile(resolve(root, screenshotPath)));
  screenshot[screenshot.length - 1] ^= 1;
  await assert.rejects(buildC1123GridAutoPlacementPixelMatrix({ [screenshotPath]: screenshot }));
});

test('Chrome oracle에서 scene이 빠지거나 순서가 바뀌면 전체 matrix를 거부한다', async () => {
  const path = 'tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json';
  const reference = JSON.parse((await readFile(resolve(root, path))).toString('utf8'));
  reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases.pop();
  await assert.rejects(buildC1123GridAutoPlacementPixelMatrix({
    [path]: Buffer.from(JSON.stringify(reference)),
  }));
});
