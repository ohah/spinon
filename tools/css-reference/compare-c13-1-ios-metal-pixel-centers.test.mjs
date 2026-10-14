import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { validateObservation } from './capture-c13-1-compact-surface-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';
import { compareMetalPixelArtifacts } from './compare-c13-1-ios-metal-pixel-centers.mjs';

const root = new URL('../../', import.meta.url);
const evidence = new URL('spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/', root);
const observation = JSON.parse(readFileSync(new URL('chrome155-observation.json', evidence), 'utf8'));
validateObservation(observation);
const reference = decodeRgbaPng(readFileSync(new URL('chrome155-surface.png', evidence)));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function artifact({ width = 900, height = 300 } = {}) {
  const scaleX = width / 300;
  const scaleY = height / 100;
  const runId = 'a'.repeat(32);
  const generation = 7;
  const rows = [];
  for (let index = 0; index < 30_000; index += 1) {
    const cssX = index % 300;
    const cssY = Math.floor(index / 300);
    rows.push({
      index,
      cssX,
      cssY,
      drawableX: Math.floor((cssX + 0.5) * scaleX),
      drawableY: Math.floor((cssY + 0.5) * scaleY),
      rgba: reference.getPixel(cssX, cssY),
      runId,
      generation,
    });
  }
  const sampleBytes = Buffer.from(`${rows.map((row) => JSON.stringify(row)).join('\n')}\n`);
  const manifest = {
    schema: 'spinon-c13-1-ios-metal-pixel-centers/v1',
    status: 'complete',
    runId,
    generation,
    viewportCssPx: [300, 100],
    drawablePixelSize: [width, height],
    surfaceFormat: 'Bgra8UnormSrgb',
    rgbaOrder: 'RGBA8',
    scaleX,
    scaleY,
    sampleCount: 30_000,
    batchSize: 64,
    batchCount: 469,
    sampleFile: `c131-metal-pixel-centers-${runId}.jsonl`,
    sampleSha256: sha256(sampleBytes),
    processID: 1234,
    elapsedMilliseconds: 12_345,
  };
  return { manifest, sampleBytes, rows };
}

function compare(value) {
  return compareMetalPixelArtifacts({ ...value, reference, observation });
}

function bytesWithRows(rows) {
  return Buffer.from(`${rows.map((row) => JSON.stringify(row)).join('\n')}\n`);
}

test('matches all 30,000 iOS drawable sample centers and the overflow outside its container border', () => {
  const input = artifact();
  const result = compare(input);
  assert.equal(result.status, 'matched');
  assert.equal(result.sampleCount, 30_000);
  assert.equal(result.comparedChannels, 120_000);
  assert.equal(result.mismatchedPixels, 0);
  assert.ok(result.outsideBorderSamples.length >= 1);
  const escape = result.outsideBorderSamples.find((sample) =>
    sample.caseId === 'positioned-escape' && sample.css[0] === 270 && sample.css[1] === 72);
  assert.deepEqual(escape, {
    caseId: 'positioned-escape',
    css: [270, 72],
    expectedRgba: [227, 84, 78, 255],
  });
  const escapeFrame = observation.cases.find((item) => item.id === 'positioned-escape').frame.box;
  assert.ok(escape.css[0] >= escapeFrame.x + escapeFrame.width,
    'negative control coordinate must be beyond the target container border');
});

test('rejects one-byte changes to every RGBA channel outside the container border', () => {
  const baseline = artifact();
  const index = 72 * 300 + 270;
  const expected = reference.getPixel(270, 72);
  for (let channel = 0; channel < 4; channel += 1) {
    const rows = baseline.rows.map((row) => ({ ...row, rgba: [...row.rgba] }));
    rows[index].rgba[channel] = expected[channel] === 0 ? 1 : expected[channel] - 1;
    const sampleBytes = bytesWithRows(rows);
    const result = compare({
      ...baseline,
      sampleBytes,
      manifest: { ...baseline.manifest, sampleSha256: sha256(sampleBytes) },
    });
    assert.equal(result.status, 'mismatch', `channel ${channel}`);
    assert.deepEqual(result.mismatches[0].css, [270, 72]);
    assert.equal(result.mismatches[0].actualRgba[channel], expected[channel] - 1);
  }
});

test('rejects missing, duplicated, and reordered rows before color comparison', () => {
  const baseline = artifact();
  const lines = baseline.sampleBytes.toString('utf8').trimEnd().split('\n');
  const mutations = [
    lines.slice(1),
    [lines[0], lines[0], ...lines.slice(2)],
    [lines[1], lines[0], ...lines.slice(2)],
  ];
  for (const changedLines of mutations) {
    const sampleBytes = Buffer.from(`${changedLines.join('\n')}\n`);
    assert.throws(() => compare({
      ...baseline,
      sampleBytes,
      manifest: { ...baseline.manifest, sampleSha256: sha256(sampleBytes) },
    }));
  }
});

test('rejects stale run IDs, mixed generations, physical coordinate drift, bad formats, and altered hashes', () => {
  const baseline = artifact();
  const rows = baseline.rows.map((row) => ({ ...row }));

  rows[4].runId = 'b'.repeat(32);
  let sampleBytes = bytesWithRows(rows);
  assert.throws(() => compare({
    ...baseline,
    sampleBytes,
    manifest: { ...baseline.manifest, sampleSha256: sha256(sampleBytes) },
  }), /run ID/);

  rows[4].runId = baseline.manifest.runId;
  rows[4].generation += 1;
  sampleBytes = bytesWithRows(rows);
  assert.throws(() => compare({
    ...baseline,
    sampleBytes,
    manifest: { ...baseline.manifest, sampleSha256: sha256(sampleBytes) },
  }), /generation/);

  const changedSize = {
    ...baseline.manifest,
    drawablePixelSize: [901, 300],
    scaleX: 901 / 300,
  };
  assert.throws(() => compare({ ...baseline, manifest: changedSize }), /매핑/);
  assert.throws(() => compare({
    ...baseline,
    manifest: { ...baseline.manifest, surfaceFormat: 'Rgba16Float' },
  }), /format/);
  assert.throws(() => compare({
    ...baseline,
    manifest: { ...baseline.manifest, sampleSha256: '0'.repeat(64) },
  }), /SHA-256/);
});
