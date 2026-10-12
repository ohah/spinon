import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import test from 'node:test';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';

const repositoryRoot = resolve(new URL('../..', import.meta.url).pathname);
const matrix = JSON.parse(readFileSync(resolve(repositoryRoot,
  'tests/fixtures/css/c11/mobile-surface-pixel-matrix.json'), 'utf8'));
const scene = matrix.scenes[0];
const pngBytes = readFileSync(resolve(repositoryRoot, scene.screenshot.path));
const runID = 'c11-android-matrix-run-001';
const pngPath = `/data/user/0/dev.spinon.bootstrap/cache/c11-grid-surface-pixel-matrix/${scene.id}-${runID}.png`;

function pixel(point) {
  return `SPINON_C111_GRID_PIXEL_MATRIX_SAMPLE run_id=${runID} pid=50123 scene=${scene.id} generation=4 sample=${point.id} css_x=${point.xCssPx} css_y=${point.yCssPx} x=${point.xCssPx} y=${point.yCssPx} rgba=${point.rgba.join(',')}`;
}

function validLog() {
  const captureHash = createHash('sha256').update(pngBytes).digest('hex');
  return [
    `SPINON_C111_GRID_PIXEL_MATRIX_BEGIN run_id=${runID} pid=50123 scene=${scene.id} generation=4 width=360 height=800 density=1 samples=${scene.samples.length}`,
    `SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${runID} scene=${scene.id} generation=4 width=360 height=800 density=1.0 attempts=1 png=${pngPath} sha256=${captureHash}`,
    ...scene.samples.map(pixel),
    `SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY status=captured run_id=${runID} scene=${scene.id} generation=4 expected_samples=${scene.samples.length} observed_samples=${scene.samples.length}`,
  ].join('\n');
}

function compare(log = validLog(), options = {}) {
  return compareC11AndroidPixelCopyScene(log, { scene, runID, pngBytes, ...options });
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
  .replace('width=360 height=800 density=1.0', 'width=640 height=480 density=1.0')));
test('rejects invalid density', () => mustReject(validLog().replace('density=1.0', 'density=0')));
test('rejects off-by-one CSS coordinate', () => mustReject(validLog().replace(`css_x=${scene.samples[0].xCssPx}`, `css_x=${scene.samples[0].xCssPx + 1}`)));
test('rejects off-by-one device pixel coordinate', () => mustReject(validLog().replace(` x=${scene.samples[0].xCssPx} y=`, ` x=${scene.samples[0].xCssPx + 1} y=`)));
test('rejects PixelCopy RGBA mismatch', () => mustReject(validLog().replace(`rgba=${scene.samples[0].rgba.join(',')}`, 'rgba=41,102,246,255')));
test('rejects PNG hash mismatch', () => mustReject(validLog().replace(/sha256=[a-f0-9]{64}/, `sha256=${'0'.repeat(64)}`)));
test('rejects altered PixelCopy PNG bytes', () => mustReject(validLog(), { pngBytes: Buffer.from('not a PNG') }));
