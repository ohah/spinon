import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const fixture = JSON.parse(readFileSync(
  'tests/fixtures/css/c11/grid-template-repeat-c1131-pixel-samples-v1.json', 'utf8'));
const referenceBytes = readFileSync(fixture.oracle.referencePath);
const screenshotBytes = readFileSync(fixture.oracle.screenshotPath);
const runID = 'c1131-pixel-matrix-test-20261013';
const rgba = (value) => value.join(',');

function androidLog() {
  const common = `run_id=${runID} scene=${fixture.sceneId}`;
  const sampleLines = fixture.samples.map((sample) =>
    `SPINON_C1131_GRID_PIXEL_MATRIX_SAMPLE ${common} pid=123 generation=4 sample=${sample.id} css_x=${sample.xCssPx} css_y=${sample.yCssPx} x=${sample.xCssPx} y=${sample.yCssPx} rgba=${rgba(sample.rgba)} expected=${rgba(sample.rgba)} exact=true`);
  return [
    `SPINON_C1131_GRID_PIXEL_MATRIX_BEGIN ${common} pid=123 generation=4 width=320 height=240 viewport_width=320 viewport_height=240 scale_x=1 scale_y=1 density=1 samples=${fixture.samples.length}`,
    `SPINON_C1131_GRID_PIXEL_MATRIX_CAPTURE status=ok ${common} generation=4 width=320 height=240 viewport_width=320 viewport_height=240 scale_x=1 scale_y=1 density=1 attempts=1 png=/data/user/0/dev.spinon.bootstrap/cache/c1131-grid-surface-pixel-matrix/${fixture.sceneId}-${runID}.png sha256=${fixture.oracle.screenshotSha256}`,
    ...sampleLines,
    `SPINON_C1131_GRID_PIXEL_MATRIX_SUMMARY status=matched ${common} generation=4 expected_samples=${fixture.samples.length} observed_samples=${fixture.samples.length} matched_samples=${fixture.samples.length}`,
  ].join('\n');
}

function iosLog() {
  const common = `run_id=${runID} pid=123 surface_generation=4 document_revision=1 render_tree_revision=1 environment_revision=0`;
  const samples = fixture.samples.map((sample) => {
    const x = Math.floor(sample.xCssPx + 0.5);
    const y = Math.floor(sample.yCssPx + 0.5);
    return `SPINON_C1131_METAL_PIXEL_SAMPLE ${common} sample=${sample.id} x=${x} y=${y} rgba=${rgba(sample.rgba)} expected=${rgba(sample.rgba)} exact=true`;
  });
  return [
    `SPINON_C1131_METAL_PIXEL_BEGIN ${common} scene=${fixture.sceneId} viewport=320x240 drawable=320x240`,
    ...samples,
    `SPINON_C1131_METAL_PIXEL_SUMMARY ${common} drawable=320x240 scale_x=1 scale_y=1 status=matched expected_samples=${fixture.samples.length} observed_samples=${fixture.samples.length} matched_samples=${fixture.samples.length}`,
  ].join('\n');
}

test('pins C11.3.1 sample colors to the Chrome 154 DPR 1 screenshot', () => {
  const result = validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes });
  assert.equal(result.referenceId, fixture.oracle.referenceId);
  assert.deepEqual(result.screenshot, [320, 240]);
  assert.equal(result.sampleCount, 5);
});

test('compares Android PixelCopy samples against the fixed Chrome pixels', () => {
  const result = compareC11AndroidPixelCopyScene(androidLog(), {
    scene: { id: fixture.sceneId, samples: fixture.samples },
    runID,
    pngBytes: screenshotBytes,
    viewportCssPx: fixture.runtime.viewportCssPx,
    markerPrefix: 'C1131',
    cacheDirectory: 'c1131-grid-surface-pixel-matrix',
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.matchedSamples, fixture.samples.length);
});

test('rejects Android pixel mismatch even when the capture is complete', () => {
  const mismatch = androidLog().replace('rgba=40,102,246,255', 'rgba=41,102,246,255');
  assert.throws(() => compareC11AndroidPixelCopyScene(mismatch, {
    scene: { id: fixture.sceneId, samples: fixture.samples },
    runID,
    pngBytes: screenshotBytes,
    viewportCssPx: fixture.runtime.viewportCssPx,
    markerPrefix: 'C1131',
    cacheDirectory: 'c1131-grid-surface-pixel-matrix',
  }));
});

test('compares iOS Metal drawable pixels with the same Chrome oracle', () => {
  const result = compareC11IOSMetalPixelLogs(iosLog(), {
    fixture,
    runID,
    markerPrefix: 'C1131',
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.schema, 'spinon-c11-3-1-ios-metal-surface-pixels/v1');
  assert.equal(result.matchedSamples, fixture.samples.length);
});

test('rejects a C11.3.1 Metal log with one changed sample', () => {
  const mismatch = iosLog().replace('rgba=229,72,77,255', 'rgba=229,73,77,255');
  assert.throws(() => compareC11IOSMetalPixelLogs(mismatch, {
    fixture,
    runID,
    markerPrefix: 'C1131',
  }));
});
