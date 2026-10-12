import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { buildC1121GridPlacementPixelMatrix } from './build-c11-2-1-grid-placement-pixel-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c11/grid-placement-mobile-surface-pixel-matrix.json';
const fixtureBytes = await readFile(resolve(repositoryRoot, fixturePath));
const fixture = JSON.parse(fixtureBytes.toString('utf8'));

test('C11.2.1 pixel matrix generator reproduces the fixed fixture byte-for-byte', async () => {
  const generated = `${JSON.stringify(await buildC1121GridPlacementPixelMatrix(), null, 2)}\n`;
  assert.equal(generated, fixtureBytes.toString('utf8'));
});

test('all ten fixed cases separate six pixel scenes from four intentional rejections', () => {
  assert.equal(fixture.summary.sceneCount, 10);
  assert.equal(fixture.summary.pixelSceneCount, 6);
  assert.equal(fixture.summary.expectedRejectCount, 4);
  assert.deepEqual(fixture.cases.filter(({ outcome }) => outcome === 'surface-pixel-compare').map(({ id }) => id), [
    'numeric-lines', 'positive-spans', 'reversed-lines', 'shorthand-cascade',
    'overlap-source-order', 'nested-placement-modes',
  ]);
  assert.deepEqual(fixture.cases.filter(({ outcome }) => outcome === 'expected-rejection').map(({ id }) => id), [
    'deferred-implicit-track', 'deferred-named-line',
    'deferred-mixed-auto-explicit', 'deferred-equal-lines',
  ]);
});

test('every expected RGBA sample is in the mobile viewport and on a 3x3 uniform Chrome region', async () => {
  const byPath = new Map();
  for (const scene of fixture.cases.filter(({ outcome }) => outcome === 'surface-pixel-compare')) {
    const imageBytes = byPath.get(scene.chromeScreenshot.path)
      ?? await readFile(resolve(repositoryRoot, scene.chromeScreenshot.path));
    byPath.set(scene.chromeScreenshot.path, imageBytes);
    const image = decodeRgbaPng(imageBytes);
    const coordinates = new Set();
    const ids = new Set();
    for (const sample of scene.samples) {
      assert.ok(sample.xCssPx >= 1 && sample.xCssPx < 319);
      assert.ok(sample.yCssPx >= 1 && sample.yCssPx < 239);
      assert.ok(sample.xCssPx < fixture.runtime.viewportCssPx.width);
      assert.ok(sample.yCssPx < fixture.runtime.viewportCssPx.height);
      assert.equal(ids.has(sample.id), false);
      assert.equal(coordinates.has(`${sample.xCssPx},${sample.yCssPx}`), false);
      ids.add(sample.id);
      coordinates.add(`${sample.xCssPx},${sample.yCssPx}`);
      for (let dy = -1; dy <= 1; dy += 1) {
        for (let dx = -1; dx <= 1; dx += 1) {
          assert.deepEqual(image.getPixel(sample.xCssPx + dx, sample.yCssPx + dy), sample.rgba,
            `${scene.id}/${sample.id} 표본이 color 경계에 있습니다.`);
        }
      }
    }
  }
});

test('겹침 scene은 실제 위쪽 paint만 표본화하고 완전히 가려진 item을 기록한다', () => {
  const scene = fixture.cases.find(({ id }) => id === 'overlap-source-order');
  assert.deepEqual(scene.occludedNodeIds, ['c1121-overlap-first']);
  assert.equal(scene.samples.some(({ nodeId }) => nodeId === 'c1121-overlap-first'), false);
  assert.ok(scene.samples.some(({ nodeId, rgba }) => nodeId === 'c1121-overlap-second'
    && rgba.join(',') === '229,72,77,255'));
});

test('fixture records immutable Chrome and runtime input hashes', async () => {
  const reference = JSON.parse(await readFile(resolve(repositoryRoot, fixture.oracle.referencePath), 'utf8'));
  const inventory = await readFile(resolve(repositoryRoot, fixture.inputs.inventory.path));
  const html = await readFile(resolve(repositoryRoot, fixture.inputs.html.path));
  const runtimeMatrix = await readFile(resolve(repositoryRoot, fixture.inputs.runtimeMatrix.path));
  assert.equal(reference.referenceId, fixture.oracle.referenceId);
  assert.equal(fixture.inputs.inventory.sha256, createHash('sha256').update(inventory).digest('hex'));
  assert.equal(fixture.inputs.html.sha256, createHash('sha256').update(html).digest('hex'));
  assert.equal(fixture.inputs.runtimeMatrix.sha256, createHash('sha256').update(runtimeMatrix).digest('hex'));
});

test('inventory hash mismatch stops sample generation', async () => {
  const inventory = await readFile(resolve(repositoryRoot, fixture.inputs.inventory.path));
  const corrupted = Buffer.from(inventory);
  corrupted[corrupted.length - 2] ^= 1;
  await assert.rejects(buildC1121GridPlacementPixelMatrix({ [fixture.inputs.inventory.path]: corrupted }));
});

test('changed Chrome PNG fails before a pixel fixture can be generated', async () => {
  const scene = fixture.cases.find(({ outcome }) => outcome === 'surface-pixel-compare');
  const original = await readFile(resolve(repositoryRoot, scene.chromeScreenshot.path));
  const corrupted = Buffer.from(original);
  corrupted[corrupted.length - 1] ^= 1;
  await assert.rejects(buildC1121GridPlacementPixelMatrix({ [scene.chromeScreenshot.path]: corrupted }));
});

test('unexpected runtime outcome cannot enter the mobile pixel matrix', async () => {
  const path = fixture.inputs.runtimeMatrix.path;
  const matrix = JSON.parse(await readFile(resolve(repositoryRoot, path), 'utf8'));
  matrix.cases[0].expected.outcome = 'success-ish';
  await assert.rejects(buildC1121GridPlacementPixelMatrix({ [path]: Buffer.from(JSON.stringify(matrix)) }));
});

test('HTML paint palette drift is rejected instead of silently changing expected RGBA', async () => {
  const html = await readFile(resolve(repositoryRoot, fixture.inputs.html.path), 'utf8');
  const changed = html.replace('.blue { background-color: #2866f6; }', '.blue { background-color: #000000; }');
  assert.notEqual(changed, html);
  await assert.rejects(buildC1121GridPlacementPixelMatrix({ [fixture.inputs.html.path]: Buffer.from(changed) }));
});
