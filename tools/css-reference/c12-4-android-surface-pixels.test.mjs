import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { deflateSync } from 'node:zlib';

import { compareC124AndroidSurfaceCase } from './compare-c12-4-android-surface-pixels.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = join(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryBytes = await readFile(join(repositoryRoot,
  'tests/fixtures/css/c12/stacking-context-inventory.json'));
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const inventorySha256 = createHash('sha256').update(inventoryBytes).digest('hex');
const reference = JSON.parse(await readFile(join(repositoryRoot,
  'tests/fixtures/css/references/c12-4-stacking-context-v1.json'), 'utf8'));
const chromePng = await readFile(join(repositoryRoot,
  'spec/internal/evidence/c12-4-stacking-context-2026-10-11/chrome-dpr-1.png'));
const chromeSamples = reference.screenshotFiles
  .find(({ deviceScaleFactor }) => deviceScaleFactor === 1).samples;
const sourceImage = decodeRgbaPng(chromePng);

function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) crc = (crc & 1) ? (0xedb88320 ^ (crc >>> 1)) : (crc >>> 1);
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const typeBytes = Buffer.from(type);
  const output = Buffer.alloc(12 + data.length);
  output.writeUInt32BE(data.length, 0);
  typeBytes.copy(output, 4);
  data.copy(output, 8);
  output.writeUInt32BE(crc32(output.subarray(4, 8 + data.length)), 8 + data.length);
  return output;
}

function createSurfacePng(mutate) {
  const width = 320;
  const height = 240;
  const rowBytes = width * 4;
  const raw = Buffer.alloc(height * (rowBytes + 1));
  for (let y = 0; y < height; y += 1) {
    const rowOffset = y * (rowBytes + 1);
    raw[rowOffset] = 0;
    for (let x = 0; x < width; x += 1) {
      const color = sourceImage.getPixel(x, y);
      const value = mutate?.(x, y, color) ?? color;
      Buffer.from(value).copy(raw, rowOffset + 1 + x * 4);
    }
  }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header.set([8, 6, 0, 0, 0], 8);
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk('IHDR', header),
    chunk('IDAT', deflateSync(raw)),
    chunk('IEND', Buffer.alloc(0)),
  ]);
}

const androidSurfacePng = createSurfacePng();

function validLog({ runId = 'run-1', rgba = [229, 72, 77, 255], sampleLines = true } = {}) {
  const imageSha256 = createHash('sha256').update(androidSurfacePng).digest('hex');
  const capture = `SPINON_C124_SURFACE_CAPTURE status=ok run_id=${runId}`
    + ' case=root-negative-flow width=320 height=240 density=1 attempts=1'
    + ` inventory_sha256=${inventorySha256} sha256=${imageSha256}`;
  const pixels = sampleLines ? [
    `SPINON_C124_SURFACE_PIXEL run_id=${runId} case=root-negative-flow`
      + ` sample=negative-under-flow x=85 y=55 rgba=${rgba.join(',')}`,
    `SPINON_C124_SURFACE_PIXEL run_id=${runId} case=root-negative-flow`
      + ' sample=root-canvas-background x=15 y=15 rgba=17,34,51,255',
  ] : [];
  return { imageSha256, log: [capture, ...pixels].join('\n') };
}

function compare(log, pngBytes = androidSurfacePng) {
  const parsed = validLog();
  if (log === undefined) log = parsed.log;
  if (pngBytes === androidSurfacePng) {
    log = log.replace(`sha256=${parsed.imageSha256}`,
      `sha256=${createHashForTest(pngBytes)}`);
  }
  const fixtureCase = inventory.cases.find(({ id }) => id === 'root-negative-flow');
  return compareC124AndroidSurfaceCase({
    log,
    pngBytes,
    caseId: fixtureCase.id,
    runId: 'run-1',
    fixtureCase: {
      id: fixtureCase.id,
      expected: { kind: 'supported' },
      samples: fixtureCase.samples,
    },
    referenceSamples: chromeSamples,
    stage: { x: inventory.layout.originCssPx.x, y: inventory.layout.originCssPx.y },
    chromeOrigin: { x: inventory.layout.originCssPx.x, y: inventory.layout.originCssPx.y },
    viewportCssPx: { width: 320, height: 240 },
    inventorySha256,
    tolerance: inventory.comparison.maximumPixelChannelError,
    expectedDensity: 1,
  });
}

function createHashForTest(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('Android pixel reader decodes fixed Chrome PNG RGBA samples', () => {
  const decoded = decodeRgbaPng(chromePng);
  assert.deepEqual({ width: decoded.width, height: decoded.height }, { width: 1000, height: 1400 });
  assert.deepEqual(decoded.getPixel(85, 55), [229, 72, 77, 255]);
  assert.deepEqual(decoded.getPixel(15, 15), [17, 34, 51, 255]);
});

test('Android surface comparator accepts the exact scene and all fixed samples', () => {
  const result = compare();
  assert.equal(result.caseId, 'root-negative-flow');
  assert.equal(result.samples.length, 2);
  assert.equal(result.maximumChannelError, 0);
  assert.deepEqual(decodeRgbaPng(androidSurfacePng).getPixel(85, 55), [229, 72, 77, 255]);
});

test('Android surface comparator rejects stale, incomplete, incorrect, and mismatched PNG evidence', () => {
  const baseline = validLog();
  assert.throws(() => compare(baseline.log.replaceAll('run-1', 'stale-run')),
    /이전 실행|실행의 sample/);
  assert.throws(() => compare(validLog({ sampleLines: false }).log), /sample 수/);
  const wrongColorPng = createSurfacePng((x, y, rgba) =>
    x === 85 && y === 55 ? [231, ...rgba.slice(1)] : rgba);
  const wrongColorLog = validLog({ rgba: [231, 72, 77, 255] }).log.replace(
    `sha256=${createHashForTest(androidSurfacePng)}`,
    `sha256=${createHashForTest(wrongColorPng)}`,
  );
  assert.throws(() => compare(wrongColorLog, wrongColorPng), /허용치를 넘었습니다/);
  assert.throws(() => compare(baseline.log, Buffer.from([1, 2, 3])), /길이가 올바르지 않습니다/);
  const wrongPixel = createSurfacePng((x, y, rgba) =>
    x === 85 && y === 55 ? [rgba[0] ^ 0xff, ...rgba.slice(1)] : rgba);
  const wrongLog = validLog().log.replace(
    `sha256=${createHashForTest(androidSurfacePng)}`,
    `sha256=${createHashForTest(wrongPixel)}`,
  );
  // reference 색에 맞춘 log라도 실제 PNG가 다르면 독립 pixel decode에서 실패해야 합니다.
  assert.throws(() => compare(wrongLog, wrongPixel), /PNG의 실제 픽셀이 다릅니다/);
});
