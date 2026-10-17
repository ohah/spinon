import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { decodeRgbaPng } from './read-png-rgba.mjs';
import {
  compareAndroidPixelCopyFiles,
  compareAndroidPixelCopyPng,
  compareMetalFullSurfaceArtifact,
  comparePhysicalSurfaceImages,
  loadFullSurfaceChromeObservation,
  outsideBorderPhysicalSamples,
} from './compare-c13-1-full-physical-pixels.mjs';

const root = new URL('../../', import.meta.url);
const evidence = new URL(
  'spec/internal/evidence/c13-1-full-physical-pixel-parity-2026-10-18/',
  root,
);
const chrome = loadFullSurfaceChromeObservation(
  fileURLToPath(new URL('chrome155-dpr-3-observation.json', evidence)),
);
const androidChrome = loadFullSurfaceChromeObservation(
  fileURLToPath(new URL('chrome155-dpr-3-observation.json', evidence)),
);
const reference = chrome.reference;
const { width, height } = reference;
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function referenceBytes() {
  const rgba = Buffer.alloc(width * height * 4);
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const offset = (y * width + x) * 4;
      for (const [channel, value] of reference.getPixel(x, y).entries()) {
        rgba[offset + channel] = value;
      }
    }
  }
  return rgba;
}

function artifact(bytes = referenceBytes()) {
  const runId = 'a'.repeat(32);
  const generation = 17;
  const processID = 4321;
  const manifest = {
    schema: 'spinon-c13-1-ios-metal-full-surface-pixels/v1',
    status: 'complete',
    runId,
    generation,
    viewportCssPx: [300, 100],
    drawablePixelSize: [width, height],
    surfaceFormat: 'Bgra8UnormSrgb',
    rgbaOrder: 'RGBA8',
    pixelCount: width * height,
    rgbaByteLength: bytes.length,
    batchSize: 64,
    batchCount: Math.ceil(width * height / 64),
    sampleFile: `c131-metal-full-surface-${runId}.rgba`,
    sampleSha256: sha256(bytes),
    processID,
    elapsedMilliseconds: 20_000,
  };
  return {
    manifest,
    rgbaBytes: bytes,
    expectedIdentity: { runId, generation, processID },
  };
}

test('compares all 270,000 iOS drawable texels and all RGBA channels', () => {
  const input = artifact();
  const result = compareMetalFullSurfaceArtifact({ ...input, reference });
  assert.equal(result.status, 'matched');
  assert.equal(result.dimensions[0] * result.dimensions[1], 270_000);
  assert.equal(result.comparedPixels, 270_000);
  assert.equal(result.comparedChannels, 1_080_000);
  assert.equal(result.mismatchedPixels, 0);
});

test('includes fixture paint beyond the container border in the physical surface', () => {
  const samples = outsideBorderPhysicalSamples({
    geometryObservation: chrome.geometryObservation,
    reference,
    scaleX: 3,
    scaleY: 3,
  });
  const escape = samples.find((sample) =>
    sample.caseId === 'positioned-escape' && sample.css[0] === 270 && sample.css[1] === 72);
  assert.deepEqual(escape.physicalPixelCenter, [811, 217]);
  assert.ok(escape.css[0] >= escape.frame.x + escape.frame.width);
});

test('compares all Android PixelCopy texels and rejects an outside-border channel mutation', () => {
  const { reference } = androidChrome;
  const screenshotPath = fileURLToPath(new URL('chrome155-dpr-3.png', evidence));
  const observationPath = fileURLToPath(
    new URL('chrome155-dpr-3-observation.json', evidence),
  );
  const screenshotBytes = readFileSync(screenshotPath);
  const runIdentity = {
    runId: '1a356eaabdc3',
    generation: 4,
    processID: 5678,
    pngSha256: sha256(screenshotBytes),
  };
  const exact = compareAndroidPixelCopyPng({
    reference, actual: reference, runIdentity, width: 900, height: 300,
  });
  assert.equal(exact.status, 'matched');
  assert.equal(exact.comparedPixels, 270_000);
  assert.equal(exact.comparedChannels, 1_080_000);
  const fileComparison = compareAndroidPixelCopyFiles({
    actualPath: screenshotPath,
    observationPath,
    runIdentity,
  });
  assert.equal(fileComparison.status, 'matched');
  assert.equal(fileComparison.comparedPixels, 270_000);
  assert.equal(fileComparison.comparedChannels, 1_080_000);
  assert.ok(fileComparison.outsideBorderSamples.some((sample) =>
    sample.caseId === 'positioned-escape'));
  for (const invalidRunId of ['a'.repeat(17), 'A'.repeat(12), '']) {
    assert.throws(() => compareAndroidPixelCopyPng({
      reference, actual: reference, width: 900, height: 300,
      runIdentity: { ...runIdentity, runId: invalidRunId },
    }));
  }

  const outside = outsideBorderPhysicalSamples({
    geometryObservation: androidChrome.geometryObservation,
    reference,
    scaleX: 3,
    scaleY: 3,
  }).find((sample) => sample.caseId === 'positioned-escape' && sample.css[0] === 270);
  const [outsideX, outsideY] = outside.physicalPixelCenter;
  const original = reference.getPixel(outsideX, outsideY);
  const changed = {
    width: reference.width,
    height: reference.height,
    getPixel(x, y) {
      if (x !== outsideX || y !== outsideY) return reference.getPixel(x, y);
      const rgba = [...original];
      rgba[3] = rgba[3] === 255 ? 254 : rgba[3] + 1;
      return rgba;
    },
  };
  const mismatch = compareAndroidPixelCopyPng({
    reference, actual: changed, runIdentity, width: 900, height: 300,
  });
  assert.equal(mismatch.status, 'mismatch');
  assert.equal(mismatch.mismatchedPixels, 1);
  assert.deepEqual(mismatch.mismatches[0].pixel, [outsideX, outsideY]);
});

test('rejects one-byte red, green, blue, and alpha changes at the outside-border texel', () => {
  const baseline = referenceBytes();
  const offset = (217 * width + 811) * 4;
  const expected = reference.getPixel(811, 217);
  for (let channel = 0; channel < 4; channel += 1) {
    const changed = Buffer.from(baseline);
    changed[offset + channel] = expected[channel] === 255
      ? expected[channel] - 1 : expected[channel] + 1;
    const input = artifact(changed);
    const result = compareMetalFullSurfaceArtifact({ ...input, reference });
    assert.equal(result.status, 'mismatch', `channel ${channel}`);
    assert.equal(result.mismatchedPixels, 1);
    assert.deepEqual(result.mismatches[0].pixel, [811, 217]);
    assert.equal(result.maximumChannelError, 1);
  }
});

test('rejects missing bytes, padded readback rows, duplicate texels, and reordered texels', () => {
  const baseline = referenceBytes();
  const manifest = artifact(baseline).manifest;
  for (const changed of [
    baseline.subarray(4),
    Buffer.concat([baseline, Buffer.alloc(252)]),
  ]) {
    assert.throws(() => compareMetalFullSurfaceArtifact({
      manifest: { ...manifest, sampleSha256: sha256(changed) },
      rgbaBytes: changed,
      reference,
      expectedIdentity: artifact().expectedIdentity,
    }), /byte 길이/);
  }

  const changed = Buffer.from(baseline);
  const firstOffset = 0;
  const secondOffset = (217 * width + 811) * 4;
  const first = [...changed.subarray(firstOffset, firstOffset + 4)];
  const second = [...changed.subarray(secondOffset, secondOffset + 4)];
  changed.set(first, secondOffset);
  changed.set(second, firstOffset);
  const result = compareMetalFullSurfaceArtifact({
    ...artifact(changed),
    reference,
  });
  assert.equal(result.status, 'mismatch');
  assert.ok(result.mismatchedPixels > 0);
});

test('rejects stale process identity, generation, altered hash, dimensions, and format', () => {
  const input = artifact();
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    expectedIdentity: { ...input.expectedIdentity, runId: 'b'.repeat(32) },
  }), /run ID/);
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    expectedIdentity: { ...input.expectedIdentity, generation: 18 },
  }), /generation/);
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    expectedIdentity: { ...input.expectedIdentity, processID: 4322 },
  }), /process ID/);
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    manifest: { ...input.manifest, sampleSha256: '0'.repeat(64) },
  }), /SHA-256/);
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    manifest: { ...input.manifest, drawablePixelSize: [300, 900] },
  }));
  assert.throws(() => compareMetalFullSurfaceArtifact({
    ...input,
    reference,
    manifest: { ...input.manifest, surfaceFormat: 'Rgba16Float' },
  }), /format/);
});

test('physical comparator refuses scaling or dimension substitution', () => {
  const actual = decodeRgbaPng(readFileSync(new URL('chrome155-dpr-3.png', evidence)));
  assert.throws(() => comparePhysicalSurfaceImages({
    reference,
    actual: { ...actual, width: 300 },
    width: 900,
    height: 300,
  }), /크기/);
});
