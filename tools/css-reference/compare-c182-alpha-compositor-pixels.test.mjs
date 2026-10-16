import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import test from 'node:test';
import { deflateSync } from 'node:zlib';

import { compareC182AlphaCompositorArtifacts } from './compare-c182-alpha-compositor-pixels.mjs';

const samples = [
  { id: 'background-anchor', cssPixel: [2, 2], rgba: [17, 34, 51, 255] },
  { id: 'transparent-interior', cssPixel: [12, 20], rgba: [17, 34, 51, 255] },
  { id: 'transparent-right-edge', cssPixel: [15, 20], rgba: [17, 34, 51, 255] },
  { id: 'partial-left-edge', cssPixel: [16, 20], rgba: [118, 37, 65, 255] },
  { id: 'partial-interior', cssPixel: [20, 20], rgba: [118, 37, 65, 255] },
  { id: 'partial-right-edge', cssPixel: [23, 20], rgba: [118, 37, 65, 255] },
  { id: 'opaque-left-edge', cssPixel: [24, 20], rgba: [32, 200, 64, 255] },
  { id: 'opaque-interior', cssPixel: [28, 20], rgba: [32, 200, 64, 255] },
  { id: 'opaque-right-edge', cssPixel: [31, 20], rgba: [32, 200, 64, 255] },
  { id: 'after-image-anchor', cssPixel: [34, 20], rgba: [17, 34, 51, 255] },
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc & 1) ? (0xedb88320 ^ (crc >>> 1)) : (crc >>> 1);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const name = Buffer.from(type, 'ascii');
  const chunk = Buffer.alloc(12 + data.length);
  chunk.writeUInt32BE(data.length, 0);
  name.copy(chunk, 4);
  data.copy(chunk, 8);
  chunk.writeUInt32BE(crc32(chunk.subarray(4, 8 + data.length)), 8 + data.length);
  return chunk;
}

function encodeRgbaPng(width, height, rgba) {
  const rowBytes = width * 4;
  const raw = Buffer.alloc(height * (rowBytes + 1));
  for (let y = 0; y < height; y += 1) {
    const rowOffset = y * (rowBytes + 1);
    raw[rowOffset] = 0;
    rgba.copy(raw, rowOffset + 1, y * rowBytes, (y + 1) * rowBytes);
  }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header[8] = 8;
  header[9] = 6;
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  return Buffer.concat([
    signature,
    pngChunk('IHDR', header),
    pngChunk('IDAT', deflateSync(raw)),
    pngChunk('IEND', Buffer.alloc(0)),
  ]);
}

function blankRgba(width, height, color) {
  const rgba = Buffer.alloc(width * height * 4);
  for (let offset = 0; offset < rgba.length; offset += 4) {
    rgba.set(color, offset);
  }
  return rgba;
}

function writePixel(rgba, width, x, y, color) {
  rgba.set(color, (y * width + x) * 4);
}

function fixture() {
  const screenshotSize = [140, 60];
  const surfaceSize = [80, 40];
  const surfaceOrigin = [10, 10];
  const background = [14, 19, 31, 255];
  const screenshot = blankRgba(...screenshotSize, background);
  const surface = blankRgba(...surfaceSize, background);
  const sampleRecords = samples.map((sample) => {
    const surfacePixel = sample.cssPixel;
    const screenPixel = [surfaceOrigin[0] + surfacePixel[0], surfaceOrigin[1] + surfacePixel[1]];
    writePixel(surface, surfaceSize[0], ...surfacePixel, sample.rgba);
    writePixel(screenshot, screenshotSize[0], ...screenPixel, sample.rgba);
    return { ...sample, surfacePixel, screenPixel };
  });
  const anchors = [
    { id: 'title', screenPixel: [1, 1], rgba: [255, 0, 0, 255] },
    { id: 'subtitle', screenPixel: [100, 1], rgba: [0, 255, 0, 255] },
    { id: 'status', screenPixel: [1, 58], rgba: [0, 0, 255, 255] },
    { id: 'background-anchor', screenPixel: [100, 58], rgba: [255, 255, 0, 255] },
  ];
  for (const anchor of anchors) {
    writePixel(screenshot, screenshotSize[0], ...anchor.screenPixel, anchor.rgba);
  }
  const screenshotBytes = encodeRgbaPng(...screenshotSize, screenshot);
  const surfaceBytes = encodeRgbaPng(...surfaceSize, surface);
  const geometry = [
    'screenshot_px=140x60',
    'content_origin_screen_px=0,0',
    'content_size_px=140x60',
    'canvas_origin_screen_px=10,10',
    'surface_size_px=80x40',
    'viewport_css_px=80x40',
    'screen_scale=1',
  ];
  const draw = [
    'generation=1', 'document_revision=27', 'render_tree_revision=5',
    'runtime_image_count=1', 'image_draw_count=1',
  ];
  const sampleLogs = sampleRecords.map((sample) => (
    `SpinonBootstrap[123:3] SPINON_C182_METAL_PIXEL_SAMPLE generation=1 id=${sample.id} `
    + `xy=${sample.surfacePixel.join(',')} rgba=${sample.rgba.join(',')} `
    + `expected=${sample.rgba.join(',')} exact=true`
  ));
  const logText = [
    `SpinonBootstrap[123:1] SPINON_C182_SCREEN_GEOMETRY ${geometry.join(' ')}`,
    `SpinonBootstrap[123:2] SPINON_C0410_DRAW ${draw.join(' ')}`,
    ...sampleLogs,
    'SpinonBootstrap[123:3] SPINON_C182_METAL_PIXEL_SUMMARY status=matched expected=10 observed=10 matched=10',
  ].join('\n');
  const manifest = {
    schema: 'spinon-c18-2-alpha-image-compositing-observation/v1',
    status: 'complete',
    platform: 'ios-simulator',
    appBinarySha256: 'a'.repeat(64),
    source: {
      encodedPngSha256: '89852885917d6f22c6be237035a10447bfb85caa8c58d4ec29205ee6f652624e',
      decodedRgbaSha256: '782a6e59a5a87aa2faa90a6e04039ae5d3e23e7d46c29f525115e428f59a25c9',
    },
    run: {
      processId: 123,
      surfaceGeneration: 1,
      documentRevision: 27,
      renderTreeRevision: 5,
      runtimeImageCount: 1,
      imageDrawCount: 1,
    },
    cssViewportPx: [80, 40],
    screenshot: {
      file: 'full-screen.png',
      sha256: sha256(screenshotBytes),
      sizePx: screenshotSize,
    },
    contentView: {
      originUiPoints: [0, 0],
      sizeUiPoints: screenshotSize,
      originScreenPx: [0, 0],
      sizeScreenPx: screenshotSize,
    },
    surface: {
      originUiPoints: surfaceOrigin,
      originScreenPx: surfaceOrigin,
      sizePx: surfaceSize,
      cssViewportPx: [80, 40],
      cssToSurfaceScale: [1, 1],
      screenScale: 1,
      generation: 1,
      format: 'Rgba8UnormSrgb',
      file: 'surface.png',
      sha256: sha256(surfaceBytes),
    },
    sourceLogFile: 'simulator-log.txt',
    sourceLogSha256: sha256(Buffer.from(logText)),
    processLogNeedle: 'SpinonBootstrap[123:',
    geometryLogContains: geometry,
    drawLogContains: draw,
    readbackLogMarker: 'SPINON_C182_METAL_PIXEL_SAMPLE',
    summaryLogMarker: 'SPINON_C182_METAL_PIXEL_SUMMARY',
    summaryLogContains: ['status=matched', 'expected=10', 'observed=10', 'matched=10'],
    samples: sampleRecords,
    anchors,
  };
  return { manifest, screenshotBytes, surfaceBytes, logText };
}

test('10개 표면 표본·screenshot pixel과 네 anchor를 exact 비교한다', () => {
  const value = fixture();
  assert.deepEqual(compareC182AlphaCompositorArtifacts(value), {
    status: 'matched',
    platform: 'ios-simulator',
    screenshotSizePx: [140, 60],
    surfaceSizePx: [80, 40],
    cssViewportPx: [80, 40],
    surfaceGeneration: 1,
    readbackSamples: 10,
    surfacePngSamples: 10,
    screenshotSamples: 10,
    screenshotAnchors: 4,
  });
});

test('manifest hash가 맞아도 screenshot pixel 불일치를 거부한다', () => {
  const value = fixture();
  const point = value.manifest.samples[0].screenPixel;
  const pixelOffset = (point[1] * value.manifest.screenshot.sizePx[0] + point[0]) * 4;
  const decoded = blankRgba(140, 60, [14, 19, 31, 255]);
  for (const sample of value.manifest.samples) {
    writePixel(decoded, 140, ...sample.screenPixel, sample.rgba);
  }
  for (const anchor of value.manifest.anchors) {
    writePixel(decoded, 140, ...anchor.screenPixel, anchor.rgba);
  }
  decoded.set([1, 2, 3, 255], pixelOffset);
  value.screenshotBytes = encodeRgbaPng(140, 60, decoded);
  value.manifest.screenshot.sha256 = sha256(value.screenshotBytes);
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /post-compositor screenshot RGBA/);
});

test('screenshot hash 불일치를 거부한다', () => {
  const value = fixture();
  value.manifest.screenshot.sha256 = 'd'.repeat(64);
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /screenshot PNG SHA-256/);
});

test('surface PixelCopy/Metal PNG의 pixel 불일치를 거부한다', () => {
  const value = fixture();
  const decoded = blankRgba(80, 40, [14, 19, 31, 255]);
  for (const sample of value.manifest.samples) {
    writePixel(decoded, 80, ...sample.surfacePixel, sample.rgba);
  }
  const point = value.manifest.samples[2].surfacePixel;
  writePixel(decoded, 80, ...point, [7, 8, 9, 255]);
  value.surfaceBytes = encodeRgbaPng(80, 40, decoded);
  value.manifest.surface.sha256 = sha256(value.surfaceBytes);
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /surface PNG RGBA/);
});

test('surface log에서 잘못된 좌표와 exact=false를 거부한다', () => {
  const value = fixture();
  const sample = value.manifest.samples[2];
  value.logText = value.logText.replace(
    `xy=${sample.surfacePixel.join(',')} rgba=${sample.rgba.join(',')}`,
    `xy=79,39 rgba=${sample.rgba.join(',')}`,
  ).replace('exact=true', 'exact=false');
  value.manifest.sourceLogSha256 = sha256(Buffer.from(value.logText));
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /exact=true/);
});

test('alpha 0 source RGB 누출 control을 거부한다', () => {
  const value = fixture();
  const sample = value.manifest.samples.find(({ id }) => id === 'transparent-interior');
  value.logText = value.logText.replace(
    `xy=${sample.surfacePixel.join(',')} rgba=${sample.rgba.join(',')}`,
    `xy=${sample.surfacePixel.join(',')} rgba=250,20,210,255`,
  );
  value.manifest.sourceLogSha256 = sha256(Buffer.from(value.logText));
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /원본 surface log RGBA가 oracle과 다릅니다/);
});

test('alpha 128을 불투명 RGB로 출력한 control을 거부한다', () => {
  const value = fixture();
  const sample = value.manifest.samples.find(({ id }) => id === 'partial-interior');
  value.logText = value.logText.replace(
    `xy=${sample.surfacePixel.join(',')} rgba=${sample.rgba.join(',')}`,
    `xy=${sample.surfacePixel.join(',')} rgba=220,40,80,255`,
  );
  value.manifest.sourceLogSha256 = sha256(Buffer.from(value.logText));
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /원본 surface log RGBA가 oracle과 다릅니다/);
});

test('geometry log와 canvas origin이 다른 manifest를 거부한다', () => {
  const value = fixture();
  value.manifest.geometryLogContains[3] = 'canvas_origin_screen_px=11,10';
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /post-compositor geometry/);
});

test('fixture 안에 놓인 screenshot anchor를 거부한다', () => {
  const value = fixture();
  value.manifest.anchors[0].screenPixel = [10, 10];
  value.manifest.anchors[0].rgba = [14, 19, 31, 255];
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /anchor가 fixture surface 안/);
});

test('같은 색상으로 중복된 anchor를 거부한다', () => {
  const value = fixture();
  value.manifest.anchors[1].rgba = value.manifest.anchors[0].rgba;
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /anchor 색상은 서로 달라야/);
});

test('중복 screenshot 표본 좌표를 거부한다', () => {
  const value = fixture();
  value.manifest.samples[1].screenPixel = value.manifest.samples[0].screenPixel;
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /surface-to-screenshot pixel transform/);
});

test('불완전한 surface summary를 거부한다', () => {
  const value = fixture();
  value.logText = value.logText.replace('status=matched', 'status=mismatch');
  value.manifest.sourceLogSha256 = sha256(Buffer.from(value.logText));
  value.manifest.summaryLogContains[0] = 'status=matched';
  assert.throws(() => compareC182AlphaCompositorArtifacts(value), /surface sample summary/);
});

test('표본이 아닌 surface PNG capture를 거부한다', () => {
  const value = fixture();
  value.surfaceBytes = null;
  value.manifest.surface.file = null;
  value.manifest.surface.sha256 = null;
  assert.equal(compareC182AlphaCompositorArtifacts(value).surfacePngSamples, null);
});
