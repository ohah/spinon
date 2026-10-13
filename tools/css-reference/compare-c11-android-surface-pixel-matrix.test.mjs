import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { deflateSync } from 'node:zlib';
import test from 'node:test';

import {
  compareC11AndroidPixelCopyScene,
  cssPixelCenterToSurfacePixel,
} from './compare-c11-android-surface-pixel-matrix.mjs';

const repositoryRoot = resolve(new URL('../..', import.meta.url).pathname);
const matrix = JSON.parse(readFileSync(resolve(repositoryRoot,
  'tests/fixtures/css/c11/mobile-surface-pixel-matrix.json'), 'utf8'));
const scene = matrix.scenes[0];
const viewportCssPx = matrix.viewportCssPx;
const surface = { width: 900, height: 675 };
const pixels = scene.samples.map((sample) => ({
  ...sample,
  x: cssPixelCenterToSurfacePixel(sample.xCssPx, surface.width, viewportCssPx.width),
  y: cssPixelCenterToSurfacePixel(sample.yCssPx, surface.height, viewportCssPx.height),
}));
const pngBytes = testPng(surface.width, surface.height, pixels);
const runID = 'c11-android-matrix-run-001';
const pngPath = `/data/user/0/dev.spinon.bootstrap/cache/c11-grid-surface-pixel-matrix/${scene.id}-${runID}.png`;

function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit++) {
      crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const typeBytes = Buffer.from(type, 'ascii');
  const chunk = Buffer.alloc(12 + data.length);
  chunk.writeUInt32BE(data.length, 0);
  typeBytes.copy(chunk, 4);
  data.copy(chunk, 8);
  chunk.writeUInt32BE(crc32(Buffer.concat([typeBytes, data])), 8 + data.length);
  return chunk;
}

function testPng(width, height, points) {
  const rowBytes = width * 4 + 1;
  const raw = Buffer.alloc(rowBytes * height);
  for (const point of points) {
    const offset = point.y * rowBytes + 1 + point.x * 4;
    point.rgba.forEach((channel, index) => { raw[offset + index] = channel; });
  }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header[8] = 8;
  header[9] = 6;
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    pngChunk('IHDR', header),
    pngChunk('IDAT', deflateSync(raw)),
    pngChunk('IEND', Buffer.alloc(0)),
  ]);
}

function pixel(point) {
  const mapped = pixels.find(({ id }) => id === point.id);
  return `SPINON_C111_GRID_PIXEL_MATRIX_SAMPLE run_id=${runID} pid=50123 scene=${scene.id} generation=4 sample=${point.id} css_x=${point.xCssPx} css_y=${point.yCssPx} x=${mapped.x} y=${mapped.y} rgba=${point.rgba.join(',')}`;
}

function validLog() {
  const captureHash = createHash('sha256').update(pngBytes).digest('hex');
  return [
    `SPINON_C111_GRID_PIXEL_MATRIX_BEGIN run_id=${runID} pid=50123 scene=${scene.id} generation=4 width=${surface.width} height=${surface.height} viewport_width=${viewportCssPx.width} viewport_height=${viewportCssPx.height} scale_x=2.8125 scale_y=2.8125 density=1 samples=${scene.samples.length}`,
    `SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${runID} scene=${scene.id} generation=4 width=${surface.width} height=${surface.height} viewport_width=${viewportCssPx.width} viewport_height=${viewportCssPx.height} scale_x=2.8125 scale_y=2.8125 density=1.0 attempts=1 png=${pngPath} sha256=${captureHash}`,
    ...scene.samples.map(pixel),
    `SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY status=captured run_id=${runID} scene=${scene.id} generation=4 expected_samples=${scene.samples.length} observed_samples=${scene.samples.length}`,
  ].join('\n');
}

function compare(log = validLog(), options = {}) {
  return compareC11AndroidPixelCopyScene(log, {
    scene, runID, pngBytes, viewportCssPx, ...options,
  });
}

function mustReject(log, overrides = {}) {
  assert.throws(() => compare(log, overrides));
}

test('accepts one PixelCopy PNG and all exact pinned Chrome scene pixels', () => {
  const result = compare();
  assert.equal(result.status, 'matched');
  assert.equal(result.expectedSamples, scene.samples.length);
  assert.equal(result.matchedSamples, scene.samples.length);
  assert.equal(result.maximumChannelError, 0);
});

test('maps CSS centers by actual PixelCopy dimensions even when density is deliberately wrong', () => {
  assert.equal(cssPixelCenterToSurfacePixel(20, 900, 320), 57);
  assert.equal(cssPixelCenterToSurfacePixel(10, 675, 240), 29);
  assert.deepEqual([
    cssPixelCenterToSurfacePixel(1, 5, 4),
    cssPixelCenterToSurfacePixel(1, 7, 4),
  ], [1, 2], 'x/y 축은 각 실제 surface 크기로 독립 계산한다.');
  assert.equal(compare().samples[0].pixel[0], 57);
});

test('rejects coordinates produced from density instead of actual surface scale', () => {
  const first = pixels[0];
  const oldDensityX = Math.floor((first.xCssPx + 0.5) * 1);
  const log = validLog().replace(`x=${first.x} y=${first.y}`, `x=${oldDensityX} y=${first.y}`);
  mustReject(log);
});

test('supports the separately namespaced C11.2.2 PixelCopy marker and cache path', () => {
  const c1122RunID = 'c1122-a-test-grid-run-001';
  const c1122Path = `/data/user/0/dev.spinon.bootstrap/cache/c1122-grid-surface-pixel-matrix/${scene.id}-${c1122RunID}.png`;
  const log = validLog().replaceAll(runID, c1122RunID)
    .replaceAll('SPINON_C111_GRID_PIXEL_MATRIX', 'SPINON_C1122_GRID_PIXEL_MATRIX')
    .replace('c11-grid-surface-pixel-matrix', 'c1122-grid-surface-pixel-matrix')
    .replaceAll(pngPath, c1122Path);
  const result = compare(log, {
    runID: c1122RunID,
    markerPrefix: 'C1122',
    cacheDirectory: 'c1122-grid-surface-pixel-matrix',
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.markerPrefix, 'C1122');
});

test('supports the separately namespaced C11.2.3 PixelCopy marker and cache path', () => {
  const c1123RunID = 'c1123-a-test-grid-run-001';
  const c1123Path = `/data/user/0/dev.spinon.bootstrap/cache/c1123-grid-surface-pixel-matrix/${scene.id}-${c1123RunID}.png`;
  const log = validLog().replaceAll(runID, c1123RunID)
    .replaceAll('SPINON_C111_GRID_PIXEL_MATRIX', 'SPINON_C1123_GRID_PIXEL_MATRIX')
    .replace('c11-grid-surface-pixel-matrix', 'c1123-grid-surface-pixel-matrix')
    .replaceAll(pngPath, c1123Path);
  const result = compare(log, {
    runID: c1123RunID,
    markerPrefix: 'C1123',
    cacheDirectory: 'c1123-grid-surface-pixel-matrix',
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.markerPrefix, 'C1123');
});

test('isolates the requested run from unrelated app processes', () => {
  const otherRunID = 'c11-android-matrix-run-002';
  assert.equal(compare(`${validLog()}\n${validLog().replaceAll(runID, otherRunID)}`).status, 'matched');
});

test('rejects stale run ID', () => mustReject(validLog().replaceAll(`run_id=${runID}`, 'run_id=stale-run')));
test('rejects missing begin marker', () => mustReject(validLog().replace(/SPINON_C111_GRID_PIXEL_MATRIX_BEGIN .*\n/, '')));
test('rejects duplicate begin marker', () => {
  const log = validLog();
  mustReject(log.replace(/SPINON_C111_GRID_PIXEL_MATRIX_BEGIN .*\n/, (line) => `${line}${line}`));
});
test('rejects missing successful capture marker', () => mustReject(validLog().replace(/SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE .*\n/, '')));
test('rejects capture error status', () => mustReject(validLog().replace('CAPTURE status=ok', 'CAPTURE status=error')));
test('rejects missing summary marker', () => mustReject(validLog().replace(/SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY .*$/, '')));
test('rejects incomplete expected sample count', () => mustReject(validLog()
  .replace(`expected_samples=${scene.samples.length}`, `expected_samples=${scene.samples.length - 1}`)));
test('rejects duplicate sample ID', () => {
  const lines = validLog().split('\n');
  mustReject([...lines, lines[2]].join('\n'));
});
test('rejects missing sample', () => mustReject(validLog().replace(/SPINON_C111_GRID_PIXEL_MATRIX_SAMPLE .*\n/, '')));
test('rejects unknown sample ID', () => mustReject(validLog().replace(`sample=${scene.samples[0].id}`, 'sample=unknown')));
test('rejects mismatched scene', () => mustReject(validLog().replaceAll(`scene=${scene.id}`, 'scene=percentage-tracks')));
test('rejects process ID changes between begin and sample', () => mustReject(validLog()
  .replace('pid=50123 scene=', 'pid=50124 scene=')));
test('rejects surface generation changes between begin and sample', () => mustReject(validLog()
  .replace('generation=4 sample=', 'generation=5 sample=')));
test('rejects begin/capture dimensions mismatch', () => mustReject(validLog()
  .replace('width=900 height=675 viewport_width=320 viewport_height=240 scale_x=2.8125 scale_y=2.8125 density=1.0',
    'width=640 height=480 viewport_width=320 viewport_height=240 scale_x=2.8125 scale_y=2.8125 density=1.0')));
test('rejects invalid density', () => mustReject(validLog().replace('density=1.0', 'density=0')));
test('rejects viewport metadata inconsistent with the fixed C11 runtime', () => mustReject(
  validLog().replace('viewport_width=320 viewport_height=240', 'viewport_width=360 viewport_height=240'),
));
test('rejects scales that are not derived from the captured bitmap dimensions', () => mustReject(
  validLog().replace('scale_x=2.8125', 'scale_x=1'),
));
test('rejects off-by-one CSS coordinate', () => mustReject(validLog().replace(`css_x=${scene.samples[0].xCssPx}`, `css_x=${scene.samples[0].xCssPx + 1}`)));
test('rejects off-by-one device pixel coordinate', () => mustReject(validLog()
  .replace(` x=${pixels[0].x} y=`, ` x=${pixels[0].x + 1} y=`)));
test('rejects PixelCopy RGBA mismatch', () => mustReject(validLog().replace(`rgba=${scene.samples[0].rgba.join(',')}`, 'rgba=41,102,246,255')));
test('rejects PNG hash mismatch', () => mustReject(validLog().replace(/sha256=[a-f0-9]{64}/, `sha256=${'0'.repeat(64)}`)));
test('rejects altered PixelCopy PNG bytes', () => mustReject(validLog(), { pngBytes: Buffer.from('not a PNG') }));
