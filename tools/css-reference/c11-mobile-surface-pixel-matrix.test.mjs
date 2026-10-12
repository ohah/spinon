import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

import { buildC11MobileSurfacePixelMatrix } from './build-c11-mobile-surface-pixel-matrix.mjs';

const fixturePath = new URL('../../tests/fixtures/css/c11/mobile-surface-pixel-matrix.json', import.meta.url);

test('pinned Chromium samples cover 10 C11 scenes and 36 visible pixels', async () => {
  const fixture = await buildC11MobileSurfacePixelMatrix();
  const stored = JSON.parse(await readFile(fixturePath, 'utf8'));
  assert.deepEqual(stored, fixture);
  assert.equal(fixture.summary.sceneCount, 10);
  assert.equal(fixture.summary.sampleCount, 36);
  assert.deepEqual(fixture.scenes.map(({ id }) => id), [
    'fixed-tracks-gap', 'percentage-tracks', 'fractional-tracks-gap',
    'mixed-fixed-percent-fr', 'row-auto-placement', 'empty-grid',
    'single-item-grid', 'grid-under-block', 'grid-under-flex', 'flex-under-grid',
  ]);
  assert.deepEqual(fixture.scenes.map(({ samples }) => samples.length), [5, 3, 3, 4, 6, 1, 2, 3, 5, 4]);
  for (const scene of fixture.scenes) {
    assert.equal(new Set(scene.samples.map(({ nodeId }) => nodeId)).size, scene.samples.length);
    for (const sample of scene.samples) {
      assert.ok(sample.xCssPx >= 0 && sample.xCssPx < 320);
      assert.ok(sample.yCssPx >= 0 && sample.yCssPx < 240);
      assert.equal(sample.rgba.length, 4);
    }
  }
});

test('generator rejects screenshot hash drift and node inventory drift', async () => {
  const fixture = await buildC11MobileSurfacePixelMatrix();
  assert.equal(fixture.oracle.dpr, 1);
  for (const scene of fixture.scenes) {
    assert.match(scene.screenshot.sha256, /^[a-f0-9]{64}$/);
    assert.ok(scene.samples.length > 0 && scene.samples.length <= 64);
    assert.equal(new Set(scene.samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
      scene.samples.length);
  }
  const reference = JSON.parse(await readFile(new URL(
    '../../tests/fixtures/css/references/c11-grid-precomparison-v2.json', import.meta.url), 'utf8'));
  const screenshotPath = reference.screenshots[0].path;
  const screenshot = Buffer.from(await readFile(new URL(`../../${screenshotPath}`, import.meta.url)));
  screenshot[screenshot.length - 8] ^= 0xff;
  await assert.rejects(() => buildC11MobileSurfacePixelMatrix({ [screenshotPath]: screenshot }));

  const inventoryPath = 'tests/fixtures/css/c11/grid-layout-inventory.json';
  const inventory = JSON.parse(await readFile(new URL(`../../${inventoryPath}`, import.meta.url), 'utf8'));
  inventory.cases[0].nodeIds.push('unreferenced-node');
  await assert.rejects(() => buildC11MobileSurfacePixelMatrix({
    [inventoryPath]: Buffer.from(`${JSON.stringify(inventory)}\n`),
  }));
});
