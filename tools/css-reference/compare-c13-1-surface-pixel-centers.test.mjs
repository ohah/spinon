import assert from 'node:assert/strict';
import test from 'node:test';

import { compareCssPixelCenters } from './compare-c13-1-surface-pixel-centers.mjs';

function image(width, height, pixelAt) {
  return {
    width,
    height,
    getPixel(x, y) { return pixelAt(x, y); },
  };
}

function scaledImage(reference, width, height, mutate = () => null) {
  const scaleX = width / reference.width;
  const scaleY = height / reference.height;
  return image(width, height, (x, y) => {
    const cssX = Math.floor(x / scaleX);
    const cssY = Math.floor(y / scaleY);
    const changed = mutate(cssX, cssY);
    return changed ?? reference.getPixel(cssX, cssY);
  });
}

test('compares every CSS pixel center using independent physical x and y scales', () => {
  const reference = image(3, 2, (x, y) => [x * 30, y * 40, x + y, 255]);
  const actual = scaledImage(reference, 8, 5);
  const result = compareCssPixelCenters(reference, actual);
  assert.equal(result.status, 'matched');
  assert.deepEqual(result.scale, [8 / 3, 2.5]);
  assert.equal(result.comparedPixels, 6);
  assert.equal(result.comparedChannels, 24);
  assert.equal(result.mismatchedPixels, 0);
});

test('detects one changed RGBA channel at its CSS and PixelCopy coordinates', () => {
  const reference = image(3, 2, (x, y) => [x * 30, y * 40, 10, 255]);
  const actual = scaledImage(reference, 6, 4, (x, y) =>
    x === 1 && y === 1 ? [30, 40, 10, 254] : null);
  const result = compareCssPixelCenters(reference, actual);
  assert.equal(result.status, 'mismatch');
  assert.equal(result.mismatchedPixels, 1);
  assert.equal(result.maximumChannelError, 1);
  assert.deepEqual(result.mismatchBounds, { minX: 1, minY: 1, maxX: 1, maxY: 1 });
  assert.deepEqual(result.mismatches[0], {
    css: [1, 1],
    pixelCopy: [3, 3],
    expectedRgba: [30, 40, 10, 255],
    actualRgba: [30, 40, 10, 254],
  });
});

test('treats one-byte red, green, blue, and alpha changes as mismatches', () => {
  const reference = image(2, 2, () => [100, 110, 120, 255]);
  for (let channel = 0; channel < 4; channel += 1) {
    const actual = scaledImage(reference, 4, 4, (x, y) => {
      if (x !== 0 || y !== 0) return null;
      const pixel = [100, 110, 120, 255];
      pixel[channel] -= 1;
      return pixel;
    });
    const result = compareCssPixelCenters(reference, actual);
    assert.equal(result.mismatchedPixels, 1, `channel ${channel} must be exact`);
    assert.equal(result.maximumChannelError, 1, `channel ${channel} must report the one-byte delta`);
  }
});

test('rejects a PixelCopy image too small for unique logical pixel centers', () => {
  const reference = image(3, 2, () => [0, 0, 0, 255]);
  const undersized = image(2, 2, () => [0, 0, 0, 255]);
  assert.throws(() => compareCssPixelCenters(reference, undersized), /보다 작아/);
});

test('rejects undersized physical dimensions before attempting duplicate pixel mappings', () => {
  const reference = image(3, 2, () => [0, 0, 0, 255]);
  const undersizedHeight = image(4, 1, () => [0, 0, 0, 255]);
  assert.throws(() => compareCssPixelCenters(reference, undersizedHeight), /보다 작아/);
});
