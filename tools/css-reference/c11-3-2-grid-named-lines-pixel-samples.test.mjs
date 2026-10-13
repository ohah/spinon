import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c11/grid-named-lines-c1132-mobile-surface-pixel-matrix-v1.json';
const fixture = JSON.parse(await readFile(join(repositoryRoot, fixturePath), 'utf8'));
const screenshotBytes = await readFile(join(repositoryRoot, fixture.oracle.screenshotPath));
const referenceBytes = await readFile(join(repositoryRoot, fixture.oracle.referencePath));
const runtimeMatrixBytes = await readFile(join(repositoryRoot, fixture.inputs.runtimeMatrix.path));
const runtimeMatrix = JSON.parse(runtimeMatrixBytes.toString('utf8'));
const runID = 'c1132-pixel-matrix-test-20261014';
const rgba = (value) => value.join(',');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function androidLog() {
  const common = `run_id=${runID} scene=${fixture.sceneId}`;
  const sampleLines = fixture.samples.map((sample) =>
    `SPINON_C1132_GRID_PIXEL_MATRIX_SAMPLE ${common} pid=123 generation=4 sample=${sample.id} css_x=${sample.xCssPx} css_y=${sample.yCssPx} x=${sample.xCssPx} y=${sample.yCssPx} rgba=${rgba(sample.rgba)} expected=${rgba(sample.rgba)} exact=true`);
  return [
    `SPINON_C1132_GRID_PIXEL_MATRIX_BEGIN ${common} pid=123 generation=4 width=320 height=240 viewport_width=320 viewport_height=240 scale_x=1 scale_y=1 density=1 samples=${fixture.samples.length}`,
    `SPINON_C1132_GRID_PIXEL_MATRIX_CAPTURE status=ok ${common} generation=4 width=320 height=240 viewport_width=320 viewport_height=240 scale_x=1 scale_y=1 density=1 attempts=1 png=/data/user/0/dev.spinon.bootstrap/cache/c1132-grid-surface-pixel-matrix/${fixture.sceneId}-${runID}.png sha256=${fixture.oracle.screenshotSha256}`,
    ...sampleLines,
    `SPINON_C1132_GRID_PIXEL_MATRIX_SUMMARY status=matched ${common} generation=4 expected_samples=${fixture.samples.length} observed_samples=${fixture.samples.length} matched_samples=${fixture.samples.length}`,
  ].join('\n');
}

function iosLog() {
  const common = `run_id=${runID} pid=123 surface_generation=4 document_revision=1 render_tree_revision=1 environment_revision=0`;
  const samples = fixture.samples.map((sample) =>
    `SPINON_C1132_METAL_PIXEL_SAMPLE ${common} sample=${sample.id} x=${sample.xCssPx} y=${sample.yCssPx} rgba=${rgba(sample.rgba)} expected=${rgba(sample.rgba)} exact=true`);
  return [
    `SPINON_C1132_METAL_PIXEL_BEGIN ${common} scene=${fixture.sceneId} viewport=320x240 drawable=320x240`,
    ...samples,
    `SPINON_C1132_METAL_PIXEL_SUMMARY ${common} drawable=320x240 scale_x=1 scale_y=1 status=matched expected_samples=${fixture.samples.length} observed_samples=${fixture.samples.length} matched_samples=${fixture.samples.length}`,
  ].join('\n');
}

test('고정 Chrome 표면·reference 해시와 두 색상 표본을 확인한다', () => {
  assert.equal(sha256(runtimeMatrixBytes), fixture.inputs.runtimeMatrix.sha256);
  assert.equal(fixture.inputs.reference.path, fixture.oracle.referencePath);
  assert.equal(fixture.inputs.reference.sha256, fixture.oracle.referenceSha256);
  assert.equal(runtimeMatrix.schema, 'spinon-css-c11-3-2-grid-named-lines-runtime-matrix/v1');
  assert.deepEqual(runtimeMatrix.cases.map(({ id }) => id), [
    'positive-occurrence', 'missing-name-implicit-line',
  ]);
  assert.equal(runtimeMatrix.cases[0].expected.outcome, 'success');
  assert.equal(runtimeMatrix.cases[1].expected.outcome, 'expected-rejection');
  assert.ok(runtimeMatrix.cases[0].source.includes('grid-column:mid 2 / last'));
  const result = validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes });
  assert.equal(result.referenceId, fixture.oracle.referenceId);
  assert.deepEqual(result.screenshot, [320, 240]);
  assert.equal(result.sampleCount, 2);
  assert.deepEqual(fixture.samples.map(({ id }) => id), ['grid-background', 'item-blue']);
});

test('Android PixelCopy 표본 두 개를 Chrome 표면과 정확히 비교한다', () => {
  const result = compareC11AndroidPixelCopyScene(androidLog(), {
    scene: { id: fixture.sceneId, samples: fixture.samples },
    runID,
    pngBytes: screenshotBytes,
    viewportCssPx: fixture.runtime.viewportCssPx,
    markerPrefix: 'C1132',
    cacheDirectory: 'c1132-grid-surface-pixel-matrix',
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.matchedSamples, fixture.samples.length);
  assert.equal(result.maximumChannelError, 0);
});

test('Android PixelCopy의 한 채널 오차를 통과시키지 않는다', () => {
  const mismatch = androidLog().replace('rgba=40,102,246,255', 'rgba=41,102,246,255');
  assert.throws(() => compareC11AndroidPixelCopyScene(mismatch, {
    scene: { id: fixture.sceneId, samples: fixture.samples },
    runID,
    pngBytes: screenshotBytes,
    viewportCssPx: fixture.runtime.viewportCssPx,
    markerPrefix: 'C1132',
    cacheDirectory: 'c1132-grid-surface-pixel-matrix',
  }));
});

test('iOS Metal drawable 표본을 Chrome 표면과 정확히 비교한다', () => {
  const result = compareC11IOSMetalPixelLogs(iosLog(), { fixture, runID, markerPrefix: 'C1132' });
  assert.equal(result.status, 'matched');
  assert.equal(result.schema, 'spinon-c11-3-2-ios-metal-surface-pixels/v1');
  assert.equal(result.matchedSamples, fixture.samples.length);
  assert.equal(result.maximumChannelError, 0);
});

test('iOS Metal 표본의 색상 오차를 통과시키지 않는다', () => {
  const mismatch = iosLog().replace('rgba=237,241,247,255', 'rgba=237,240,247,255');
  assert.throws(() => compareC11IOSMetalPixelLogs(mismatch, {
    fixture, runID, markerPrefix: 'C1132',
  }));
});
