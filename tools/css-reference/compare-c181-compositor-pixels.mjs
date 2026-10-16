import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const expectedSamples = [
  { id: 'visible-content-positive', cssPixel: [9, 9], rgba: [255, 255, 64, 255] },
  { id: 'visible-border-overpaint', cssPixel: [25, 10], rgba: [0, 255, 64, 255] },
  { id: 'visible-overflow-first', cssPixel: [26, 10], rgba: [0, 255, 64, 255] },
  { id: 'visible-overflow-middle', cssPixel: [29, 10], rgba: [0, 255, 64, 255] },
  { id: 'visible-overflow-last', cssPixel: [32, 10], rgba: [0, 255, 64, 255] },
  { id: 'visible-after-image', cssPixel: [33, 10], rgba: [16, 24, 39, 255] },
  { id: 'hidden-padding-positive', cssPixel: [60, 10], rgba: [0, 255, 64, 255] },
  { id: 'hidden-border-positive', cssPixel: [61, 10], rgba: [240, 198, 116, 255] },
  { id: 'hidden-outside', cssPixel: [62, 10], rgba: [16, 24, 39, 255] },
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertHash(value, label) {
  assert.match(value, /^[a-f0-9]{64}$/, `${label} SHA-256 형식이 잘못되었습니다.`);
}

function assertPixel(value, label) {
  assert.ok(Array.isArray(value) && value.length === 2, `${label} 좌표가 올바르지 않습니다.`);
  for (const coordinate of value) {
    assert.ok(Number.isSafeInteger(coordinate) && coordinate >= 0,
      `${label} 좌표는 0 이상의 정수여야 합니다.`);
  }
}

function assertRgba(value, label) {
  assert.ok(Array.isArray(value) && value.length === 4, `${label} RGBA 형식이 올바르지 않습니다.`);
  for (const channel of value) {
    assert.ok(Number.isInteger(channel) && channel >= 0 && channel <= 255,
      `${label} RGBA 채널이 8-bit 범위 밖입니다.`);
  }
}

function assertSize(value, label) {
  assert.ok(Array.isArray(value) && value.length === 2, `${label} 크기가 올바르지 않습니다.`);
  for (const dimension of value) {
    assert.ok(Number.isSafeInteger(dimension) && dimension > 0,
      `${label} 크기는 양의 정수여야 합니다.`);
  }
}

function assertScale(value, label) {
  assert.ok(Array.isArray(value) && value.length === 2, `${label} scale가 올바르지 않습니다.`);
  for (const component of value) {
    assert.ok(Number.isFinite(component) && component > 0, `${label} scale는 양수여야 합니다.`);
  }
}

function assertFileName(value, label) {
  assert.equal(typeof value, 'string', `${label} 파일명이 필요합니다.`);
  assert.notEqual(value, '', `${label} 파일명이 비어 있습니다.`);
  assert.equal(path.basename(value), value, `${label}은 근거 폴더의 파일명이어야 합니다.`);
}

function assertLogLine(logText, marker, expectedParts, label) {
  const lines = logText.split(/\r?\n/).filter((line) => line.includes(marker));
  const matching = lines.filter((line) => expectedParts.every((part) => line.includes(part)));
  assert.equal(matching.length, 1, `${label} 원본 로그가 정확히 하나 필요합니다.`);
  return matching[0];
}

function parseRgba(text, label) {
  const channels = text.split(',').map(Number);
  assertRgba(channels, label);
  return channels;
}

function sampleMapFromLog(logText, manifest) {
  const lines = logText.split(/\r?\n/).filter((line) => line.includes(manifest.readbackLogMarker));
  assert.equal(lines.length, expectedSamples.length, 'surface readback 표본 로그가 9개가 아닙니다.');
  const found = new Map();
  for (const sample of expectedSamples) {
    const matching = lines.filter((line) => line.includes(`id=${sample.id} `));
    assert.equal(matching.length, 1, `${sample.id} surface 표본 로그가 정확히 하나 필요합니다.`);
    const line = matching[0];
    const coordinateMatch = line.match(/\bxy=(\d+),(\d+)\b/);
    const rgbaMatch = line.match(/\brgba=(\d+,\d+,\d+,\d+)\b/);
    assert.ok(coordinateMatch, `${sample.id} surface 좌표 로그가 없습니다.`);
    assert.ok(rgbaMatch, `${sample.id} surface RGBA 로그가 없습니다.`);
    const observedPixel = [Number(coordinateMatch[1]), Number(coordinateMatch[2])];
    const observedRgba = parseRgba(rgbaMatch[1], `${sample.id} surface`);
    const expectedMatch = line.match(/\bexpected=(\d+,\d+,\d+,\d+)\b/);
    if (expectedMatch) {
      assert.deepEqual(parseRgba(expectedMatch[1], `${sample.id} expected`), sample.rgba,
        `${sample.id} log의 예상 색이 고정 oracle과 다릅니다.`);
    }
    assert.ok(line.includes('exact=true'), `${sample.id} surface 표본이 exact=true가 아닙니다.`);
    found.set(sample.id, { pixel: observedPixel, rgba: observedRgba });
  }
  return found;
}

export function compareC181CompositorArtifacts({
  manifest,
  screenshotBytes,
  logText,
  surfaceBytes = null,
}) {
  assert.ok(manifest && typeof manifest === 'object', 'C18.1 compositor manifest가 필요합니다.');
  assert.equal(manifest.schema, 'spinon-c18-1-compositor-observation/v1');
  assert.equal(manifest.status, 'complete', '완료되지 않은 compositor capture는 비교할 수 없습니다.');
  assert.ok(['android-avd', 'ios-simulator'].includes(manifest.platform),
    'Android AVD 또는 iOS Simulator capture만 비교합니다.');
  assert.ok(Buffer.isBuffer(screenshotBytes), '전체 화면 PNG bytes가 필요합니다.');
  assert.equal(typeof logText, 'string', '플랫폼 원본 로그가 필요합니다.');

  assertHash(manifest.appBinarySha256, '앱 binary');
  assert.ok(manifest.source && typeof manifest.source === 'object', 'source identity가 필요합니다.');
  assertHash(manifest.source.encodedPngSha256, 'encoded PNG');
  assertHash(manifest.source.decodedRgbaSha256, 'decoded RGBA');
  assert.ok(manifest.run && typeof manifest.run === 'object', '실행 identity가 필요합니다.');
  for (const [key, minimum] of [
    ['processId', 1], ['surfaceGeneration', 1], ['documentRevision', 0],
    ['renderTreeRevision', 0], ['runtimeImageCount', 2], ['imageDrawCount', 2],
  ]) {
    assert.ok(Number.isSafeInteger(manifest.run[key]) && manifest.run[key] >= minimum,
      `run.${key} 값이 올바르지 않습니다.`);
  }
  assert.equal(manifest.run.runtimeImageCount, 2, 'live document image가 두 개여야 합니다.');
  assert.equal(manifest.run.imageDrawCount, 2, 'WGPU image draw가 두 번이어야 합니다.');
  assert.equal(manifest.surface.generation, manifest.run.surfaceGeneration,
    'surface와 draw generation이 다릅니다.');

  assert.ok(manifest.screenshot && typeof manifest.screenshot === 'object', 'screenshot 정보가 필요합니다.');
  assertFileName(manifest.screenshot.file, 'screenshot');
  assertHash(manifest.screenshot.sha256, 'screenshot');
  assert.equal(sha256(screenshotBytes), manifest.screenshot.sha256, 'screenshot PNG SHA-256이 다릅니다.');
  assertSize(manifest.screenshot.sizePx, 'screenshot');
  assertPixel(manifest.cssViewportPx, 'CSS viewport');
  assert.deepEqual(manifest.cssViewportPx, [80, 40], 'C18.1 fixture viewport는 80×40이어야 합니다.');

  const screenshot = decodeRgbaPng(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], manifest.screenshot.sizePx,
    'screenshot PNG 크기가 manifest와 다릅니다.');

  assert.ok(manifest.contentView && typeof manifest.contentView === 'object', 'content view mapping이 필요합니다.');
  assertPixel(manifest.contentView.originUiPoints, 'content view UI origin');
  assertSize(manifest.contentView.sizeUiPoints, 'content view UI');
  assertPixel(manifest.contentView.originScreenPx, 'content view screen origin');
  assertSize(manifest.contentView.sizeScreenPx, 'content view screen');
  const screenScale = manifest.surface.screenScale;
  assert.ok(Number.isFinite(screenScale) && screenScale > 0, 'screen scale가 올바르지 않습니다.');
  assert.deepEqual(manifest.contentView.originScreenPx,
    manifest.contentView.originUiPoints.map((value) => value * screenScale),
    'content view UI origin과 screenshot pixel origin이 다릅니다.');
  assert.deepEqual(manifest.contentView.sizeScreenPx,
    manifest.contentView.sizeUiPoints.map((value) => value * screenScale),
    'content view UI 크기와 screenshot pixel 크기가 다릅니다.');
  assert.deepEqual(manifest.contentView.originScreenPx, [0, 0],
    '전체 화면 screenshot은 content view 원점에서 시작해야 합니다.');
  assert.deepEqual(manifest.contentView.sizeScreenPx, manifest.screenshot.sizePx,
    '전체 화면 screenshot 크기와 content view 크기가 다릅니다.');

  assert.ok(manifest.surface && typeof manifest.surface === 'object', 'surface mapping이 필요합니다.');
  assertPixel(manifest.surface.originUiPoints, 'surface UI origin');
  assertPixel(manifest.surface.originScreenPx, 'surface screen origin');
  assertSize(manifest.surface.sizePx, 'surface');
  assertScale(manifest.surface.cssToSurfaceScale, 'CSS-to-surface');
  assert.deepEqual(manifest.surface.cssViewportPx, manifest.cssViewportPx,
    'surface viewport와 fixture viewport가 다릅니다.');
  assert.deepEqual(manifest.surface.originScreenPx,
    manifest.surface.originUiPoints.map((value) => value * screenScale),
    'surface UI origin과 screenshot pixel origin이 다릅니다.');
  const [viewportWidth, viewportHeight] = manifest.cssViewportPx;
  const [surfaceWidth, surfaceHeight] = manifest.surface.sizePx;
  const scaleX = surfaceWidth / viewportWidth;
  const scaleY = surfaceHeight / viewportHeight;
  assert.deepEqual(manifest.surface.cssToSurfaceScale, [scaleX, scaleY],
    'CSS-to-surface scale이 surface 크기에서 계산한 값과 다릅니다.');
  assert.ok(Math.abs(scaleX - scaleY) < 0.000001, 'surface scale이 균일하지 않습니다.');
  assert.ok(manifest.surface.originScreenPx[0] + surfaceWidth <= screenshot.width
    && manifest.surface.originScreenPx[1] + surfaceHeight <= screenshot.height,
  'surface 전체가 screenshot 범위 안에 있어야 합니다.');

  assertFileName(manifest.sourceLogFile, '플랫폼 log');
  assertHash(manifest.sourceLogSha256, '플랫폼 log');
  assert.equal(sha256(Buffer.from(logText)), manifest.sourceLogSha256,
    '플랫폼 원본 로그 SHA-256이 다릅니다.');
  assert.equal(typeof manifest.processLogNeedle, 'string');
  assert.ok(logText.includes(manifest.processLogNeedle), '원본 로그가 manifest의 앱 process를 포함하지 않습니다.');
  assertLogLine(logText, 'SPINON_C181_SCREEN_GEOMETRY', manifest.geometryLogContains,
    'post-compositor geometry');
  assertLogLine(logText, 'SPINON_C0410_DRAW', manifest.drawLogContains, 'WGPU draw');
  assertLogLine(logText, manifest.summaryLogMarker, manifest.summaryLogContains,
    'surface sample summary');

  const readbackSamples = sampleMapFromLog(logText, manifest);
  const hasSurfacePng = surfaceBytes !== null;
  if (hasSurfacePng) {
    assertFileName(manifest.surface.file, 'surface');
    assertHash(manifest.surface.sha256, 'surface');
    assert.equal(sha256(surfaceBytes), manifest.surface.sha256, 'surface PNG SHA-256이 다릅니다.');
  } else {
    assert.equal(manifest.surface.file, null, 'surface PNG가 없으면 file은 null이어야 합니다.');
    assert.equal(manifest.surface.sha256, null, 'surface PNG가 없으면 SHA-256은 null이어야 합니다.');
  }
  const surface = hasSurfacePng ? decodeRgbaPng(surfaceBytes) : null;
  if (surface) {
    assert.deepEqual([surface.width, surface.height], manifest.surface.sizePx,
      'surface PNG 크기가 실제 drawable 크기와 다릅니다.');
  }

  assert.ok(Array.isArray(manifest.samples), 'screenshot pixel 표본이 필요합니다.');
  assert.equal(manifest.samples.length, expectedSamples.length, 'screenshot pixel 표본은 9개여야 합니다.');
  const sampleIds = new Set();
  const screenshotCoordinates = new Set();
  for (const expected of expectedSamples) {
    const matches = manifest.samples.filter((sample) => sample.id === expected.id);
    assert.equal(matches.length, 1, `${expected.id} screenshot 표본이 정확히 하나 필요합니다.`);
    const sample = matches[0];
    sampleIds.add(sample.id);
    assert.deepEqual(sample.cssPixel, expected.cssPixel, `${sample.id} CSS 좌표가 oracle과 다릅니다.`);
    assert.deepEqual(sample.rgba, expected.rgba, `${sample.id} RGBA oracle이 다릅니다.`);
    assertPixel(sample.surfacePixel, `${sample.id} surface`);
    assertPixel(sample.screenPixel, `${sample.id} screenshot`);
    assert.deepEqual(sample.surfacePixel, [
      Math.floor((expected.cssPixel[0] + 0.5) * scaleX),
      Math.floor((expected.cssPixel[1] + 0.5) * scaleY),
    ], `${sample.id} CSS-to-surface pixel transform이 다릅니다.`);
    assert.deepEqual(sample.screenPixel, [
      manifest.surface.originScreenPx[0] + sample.surfacePixel[0],
      manifest.surface.originScreenPx[1] + sample.surfacePixel[1],
    ], `${sample.id} surface-to-screenshot pixel transform이 다릅니다.`);
    assertRgba(readbackSamples.get(sample.id).rgba, `${sample.id} readback`);
    assert.deepEqual(readbackSamples.get(sample.id).pixel, sample.surfacePixel,
      `${sample.id} 원본 surface log 좌표와 manifest가 다릅니다.`);
    assert.deepEqual(readbackSamples.get(sample.id).rgba, expected.rgba,
      `${sample.id} 원본 surface log RGBA가 oracle과 다릅니다.`);
    if (surface) {
      assert.deepEqual(surface.getPixel(...sample.surfacePixel), expected.rgba,
        `${sample.id} PixelCopy/Metal surface PNG RGBA가 oracle과 다릅니다.`);
    }
    const screenKey = sample.screenPixel.join(',');
    assert.equal(screenshotCoordinates.has(screenKey), false,
      `${sample.id} screenshot pixel 좌표가 중복되었습니다.`);
    screenshotCoordinates.add(screenKey);
    assert.deepEqual(screenshot.getPixel(...sample.screenPixel), expected.rgba,
      `${sample.id} post-compositor screenshot RGBA가 oracle과 다릅니다.`);
  }
  assert.equal(sampleIds.size, expectedSamples.length, 'screenshot 표본 ID가 중복되었습니다.');

  assert.ok(Array.isArray(manifest.anchors), 'fixture 바깥 screenshot anchor가 필요합니다.');
  assert.equal(manifest.anchors.length, 4, 'fixture 바깥 anchor는 네 점이어야 합니다.');
  const anchorIds = new Set();
  const anchorColors = new Set();
  const anchorCoordinates = new Set();
  const [surfaceOriginX, surfaceOriginY] = manifest.surface.originScreenPx;
  for (const anchor of manifest.anchors) {
    assert.ok(typeof anchor.id === 'string' && anchor.id.length > 0, 'anchor ID가 필요합니다.');
    assert.equal(anchorIds.has(anchor.id), false, 'anchor ID가 중복되었습니다.');
    anchorIds.add(anchor.id);
    assertPixel(anchor.screenPixel, `${anchor.id} anchor`);
    assertRgba(anchor.rgba, `${anchor.id} anchor`);
    const [x, y] = anchor.screenPixel;
    assert.ok(x < surfaceOriginX || x >= surfaceOriginX + surfaceWidth
      || y < surfaceOriginY || y >= surfaceOriginY + surfaceHeight,
    `${anchor.id} anchor가 fixture surface 안에 있습니다.`);
    const coordinateKey = anchor.screenPixel.join(',');
    assert.equal(anchorCoordinates.has(coordinateKey), false, 'anchor 좌표가 중복되었습니다.');
    anchorCoordinates.add(coordinateKey);
    const color = anchor.rgba.join(',');
    assert.equal(anchorColors.has(color), false, 'anchor 색상은 서로 달라야 합니다.');
    anchorColors.add(color);
    assert.deepEqual(screenshot.getPixel(...anchor.screenPixel), anchor.rgba,
      `${anchor.id} screenshot anchor RGBA가 다릅니다.`);
  }

  return {
    status: 'matched',
    platform: manifest.platform,
    screenshotSizePx: manifest.screenshot.sizePx,
    surfaceSizePx: manifest.surface.sizePx,
    cssViewportPx: manifest.cssViewportPx,
    surfaceGeneration: manifest.run.surfaceGeneration,
    readbackSamples: readbackSamples.size,
    surfacePngSamples: surface ? expectedSamples.length : null,
    screenshotSamples: screenshotCoordinates.size,
    screenshotAnchors: anchorIds.size,
  };
}

function main() {
  const manifestPath = process.argv[2];
  assert.ok(manifestPath, '사용법: node compare-c181-compositor-pixels.mjs <manifest.json>');
  const absoluteManifestPath = path.resolve(manifestPath);
  const directory = path.dirname(absoluteManifestPath);
  const manifest = JSON.parse(readFileSync(absoluteManifestPath, 'utf8'));
  const screenshotBytes = readFileSync(path.join(directory, manifest.screenshot.file));
  const logText = readFileSync(path.join(directory, manifest.sourceLogFile), 'utf8');
  const surfaceBytes = manifest.surface.file === null
    ? null : readFileSync(path.join(directory, manifest.surface.file));
  const result = compareC181CompositorArtifacts({ manifest, screenshotBytes, logText, surfaceBytes });
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`C18.1 screenshot compare rejected: ${error.message}\n`);
    process.exitCode = 1;
  }
}
