import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { decodeRgbaPng } from './read-png-rgba.mjs';
import { hasRunSummary } from './c11-grid-placement-pixel-capture.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c11/grid-placement-pixel-samples.json';
const fixture = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('C11.2.1 표본은 고정 Chrome reference와 screenshot에서 추출된다', async () => {
  assert.equal(fixture.schema, 'spinon-css-c11-2-1-grid-placement-pixel-samples/v1');
  assert.equal(fixture.fixtureId, 'c11-2-1-numeric-lines');
  assert.equal(fixture.sceneId, 'numeric-lines');
  const referenceBytes = await readFile(resolve(repositoryRoot, fixture.oracle.referencePath));
  const screenshotBytes = await readFile(resolve(repositoryRoot, fixture.oracle.screenshotPath));
  assert.equal(sha256(referenceBytes), fixture.oracle.referenceSha256);
  assert.equal(sha256(screenshotBytes), fixture.oracle.screenshotSha256);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(reference.referenceId, fixture.oracle.referenceId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  const observation = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  const scene = observation.cases.find(({ id }) => id === fixture.sceneId);
  assert.equal(scene.nodes.length, 3);
  assert.ok(reference.screenshots.some(({ caseId, path, sha256: screenshotHash }) =>
    caseId === fixture.sceneId && path === fixture.oracle.screenshotPath
      && screenshotHash === fixture.oracle.screenshotSha256));
  const screenshot = decodeRgbaPng(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], [360, 800]);
  assert.equal(fixture.samples.length, 5);
  assert.equal(new Set(fixture.samples.map(({ id }) => id)).size, fixture.samples.length);
  assert.equal(new Set(fixture.samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
    fixture.samples.length);
  for (const sample of fixture.samples) {
    assert.ok(sample.xCssPx < fixture.runtime.viewportCssPx.width);
    assert.ok(sample.yCssPx < fixture.runtime.viewportCssPx.height);
    assert.deepEqual(screenshot.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba, sample.id);
  }
});

test('표본 좌표는 파랑·빨강 item과 gap/background를 구별한다', () => {
  const sampleByID = new Map(fixture.samples.map((sample) => [sample.id, sample]));
  assert.deepEqual(sampleByID.get('positive-lines').rgba, [40, 102, 246, 255]);
  assert.deepEqual(sampleByID.get('negative-lines').rgba, [229, 72, 77, 255]);
  for (const id of ['grid-background', 'column-gap', 'row-gap']) {
    assert.deepEqual(sampleByID.get(id).rgba, [237, 241, 247, 255]);
  }
});

test('Android PixelCopy 완료 marker는 필드 순서가 달라도 같은 run만 인정한다', () => {
  const marker = 'SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY';
  assert.equal(hasRunSummary(`${marker} status=captured run_id=run-1 scene=numeric-lines`, marker, 'run-1'), true);
  assert.equal(hasRunSummary(`${marker} run_id=run-1 status=captured`, marker, 'run-1'), true);
  assert.equal(hasRunSummary(`${marker} status=captured run_id=run-2`, marker, 'run-1'), false);
  assert.equal(hasRunSummary(`${marker} status=error run_id=run-1`, marker, 'run-1'), false);
  assert.equal(hasRunSummary('', marker, 'run-1'), false);
});

test('iOS Metal pixel summary는 matched 상태가 run id 앞에 있어도 인식한다', () => {
  const marker = 'SPINON_C111_METAL_PIXEL_SUMMARY';
  assert.equal(
    hasRunSummary(`${marker} status=matched run_id=run-1 samples=5`, marker, 'run-1', 'matched'),
    true,
  );
  assert.equal(
    hasRunSummary(`${marker} status=mismatch run_id=run-1 samples=4`, marker, 'run-1', 'matched'),
    false,
  );
});
