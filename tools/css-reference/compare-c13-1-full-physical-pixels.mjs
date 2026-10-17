import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { validateObservation } from './capture-c13-1-compact-surface-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const supportedFormats = new Set([
  'Rgba8Unorm', 'Rgba8UnormSrgb', 'Bgra8Unorm', 'Bgra8UnormSrgb',
]);

export function comparePhysicalSurfaceImages({ reference, actual, width, height }) {
  assert.ok(reference && actual, '고정 Chrome 이미지와 surface 이미지가 필요합니다.');
  for (const [label, image] of [['Chrome', reference], ['surface', actual]]) {
    assert.ok(Number.isSafeInteger(image.width) && image.width > 0,
      `${label} 이미지 너비가 잘못되었습니다.`);
    assert.ok(Number.isSafeInteger(image.height) && image.height > 0,
      `${label} 이미지 높이가 잘못되었습니다.`);
    assert.equal(typeof image.getPixel, 'function', `${label} 이미지 픽셀 판독기가 없습니다.`);
  }
  assert.deepEqual([reference.width, reference.height], [width, height],
    'Chrome DPR screenshot 크기가 고정 physical drawable 크기와 다릅니다.');
  assert.deepEqual([actual.width, actual.height], [width, height],
    'surface 이미지 크기가 고정 physical drawable 크기와 다릅니다.');

  const mismatches = [];
  let comparedChannels = 0;
  let maximumChannelError = 0;
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const expectedRgba = reference.getPixel(x, y);
      const actualRgba = actual.getPixel(x, y);
      assert.equal(expectedRgba.length, 4, `Chrome RGBA 길이: ${x},${y}`);
      assert.equal(actualRgba.length, 4, `surface RGBA 길이: ${x},${y}`);
      const errors = expectedRgba.map((value, channel) =>
        Math.abs(value - actualRgba[channel]));
      maximumChannelError = Math.max(maximumChannelError, ...errors);
      comparedChannels += 4;
      if (errors.some((error) => error !== 0)) {
        mismatches.push({ pixel: [x, y], expectedRgba, actualRgba });
      }
    }
  }
  const mismatchBounds = mismatches.length === 0 ? null : {
    minX: Math.min(...mismatches.map(({ pixel: [x] }) => x)),
    minY: Math.min(...mismatches.map(({ pixel: [, y] }) => y)),
    maxX: Math.max(...mismatches.map(({ pixel: [x] }) => x)),
    maxY: Math.max(...mismatches.map(({ pixel: [, y] }) => y)),
  };
  return {
    status: mismatches.length === 0 ? 'matched' : 'mismatch',
    dimensions: [width, height],
    comparedPixels: width * height,
    comparedChannels,
    mismatchedPixels: mismatches.length,
    maximumChannelError,
    mismatchBounds,
    mismatches,
  };
}

function assertRunIdentity(manifest, expectedIdentity) {
  assert.ok(expectedIdentity && typeof expectedIdentity === 'object',
    'capture process에서 관찰한 run identity가 필요합니다.');
  assert.equal(manifest.runId, expectedIdentity.runId, 'run ID가 process log와 다릅니다.');
  assert.equal(manifest.generation, expectedIdentity.generation,
    'surface generation이 process log와 다릅니다.');
  assert.equal(manifest.processID, expectedIdentity.processID,
    'app process ID가 process launch 결과와 다릅니다.');
}

export function compareMetalFullSurfaceArtifact({
  manifest,
  rgbaBytes,
  reference,
  expectedIdentity,
  width = 900,
  height = 300,
}) {
  assert.ok(manifest && typeof manifest === 'object', 'Metal capture manifest가 필요합니다.');
  assert.equal(manifest.schema, 'spinon-c13-1-ios-metal-full-surface-pixels/v1');
  assert.equal(manifest.status, 'complete', '미완성 Metal capture는 비교할 수 없습니다.');
  assert.match(manifest.runId, /^[a-f0-9]{32}$/);
  assert.ok(Number.isSafeInteger(manifest.generation) && manifest.generation > 0,
    'surface generation이 잘못되었습니다.');
  assert.deepEqual(manifest.viewportCssPx, [300, 100]);
  assert.deepEqual(manifest.drawablePixelSize, [width, height]);
  assert.ok(supportedFormats.has(manifest.surfaceFormat),
    '지원하지 않는 WGPU surface format입니다.');
  assert.equal(manifest.rgbaOrder, 'RGBA8');
  assert.equal(manifest.pixelCount, width * height);
  assert.equal(manifest.rgbaByteLength, width * height * 4);
  assert.equal(manifest.batchSize, 64);
  assert.equal(manifest.batchCount, Math.ceil(width * height / 64));
  assert.ok(Number.isSafeInteger(manifest.processID) && manifest.processID > 0,
    'app process ID가 잘못되었습니다.');
  assert.ok(Number.isSafeInteger(manifest.elapsedMilliseconds)
    && manifest.elapsedMilliseconds >= 0 && manifest.elapsedMilliseconds <= 300_000,
  'capture가 전체 실행 제한 시간을 넘었거나 elapsed 값이 잘못되었습니다.');
  assert.equal(path.basename(manifest.sampleFile), manifest.sampleFile,
    'packed artifact는 capture cache 안의 파일명이어야 합니다.');
  assert.match(manifest.sampleSha256, /^[a-f0-9]{64}$/);
  assert.ok(Buffer.isBuffer(rgbaBytes), 'packed RGBA artifact bytes가 필요합니다.');
  assert.equal(rgbaBytes.length, width * height * 4,
    'packed RGBA byte 길이가 전체 physical surface와 다릅니다.');
  assert.equal(sha256(rgbaBytes), manifest.sampleSha256,
    'packed RGBA SHA-256이 manifest와 다릅니다.');
  assertRunIdentity(manifest, expectedIdentity);

  const actual = {
    width,
    height,
    getPixel(x, y) {
      assert.ok(Number.isSafeInteger(x) && x >= 0 && x < width, 'surface x가 범위 밖입니다.');
      assert.ok(Number.isSafeInteger(y) && y >= 0 && y < height, 'surface y가 범위 밖입니다.');
      const offset = (y * width + x) * 4;
      return [...rgbaBytes.subarray(offset, offset + 4)];
    },
  };
  const result = comparePhysicalSurfaceImages({ reference, actual, width, height });
  return {
    ...result,
    runId: manifest.runId,
    generation: manifest.generation,
    processID: manifest.processID,
    surfaceFormat: manifest.surfaceFormat,
    rgbaByteLength: manifest.rgbaByteLength,
    sampleSha256: manifest.sampleSha256,
  };
}

export function compareAndroidPixelCopyPng({ reference, actual, runIdentity, width = 788, height = 263 }) {
  assert.ok(runIdentity && typeof runIdentity === 'object',
    'PixelCopy run ID, surface generation, process ID, PNG hash가 필요합니다.');
  // Android creates this value with Long.toHexString(elapsedRealtimeNanos()).
  assert.match(runIdentity.runId, /^[a-f0-9]{1,16}$/);
  assert.ok(Number.isSafeInteger(runIdentity.generation) && runIdentity.generation > 0);
  assert.ok(Number.isSafeInteger(runIdentity.processID) && runIdentity.processID > 0);
  assert.match(runIdentity.pngSha256, /^[a-f0-9]{64}$/);
  const result = comparePhysicalSurfaceImages({ reference, actual, width, height });
  return {
    ...result,
    runId: runIdentity.runId,
    generation: runIdentity.generation,
    processID: runIdentity.processID,
    pixelCopyPngSha256: runIdentity.pngSha256,
  };
}

export function loadFullSurfaceChromeObservation(observationPath) {
  const resolvedObservationPath = path.resolve(observationPath);
  const observationBytes = readFileSync(resolvedObservationPath);
  const observation = JSON.parse(observationBytes);
  assert.equal(observation.schema, 'spinon-c13-1-full-physical-pixel-observation/v1');
  assert.equal(observation.status, 'captured');
  assert.equal(observation.browser.product, 'Google Chrome');
  assert.equal(observation.browser.version, '155.0.8059.85');
  assert.equal(observation.browser.revision, '0e66b94549a8bd85f03fbdf189b752d6e7b63c32');
  assert.equal(observation.browser.binarySha256,
    '6e156f2271cef44afce9db36a5b920e94518857b94a4160f082696f8c3bd1c9d');
  assert.match(observation.browser.userAgent, /HeadlessChrome\/155\.0\.0\.0/);
  assert.match(observation.captureTool,
    /^agent-browser 0\.31\.1 connected to Chrome DevTools Protocol/);
  assert.deepEqual(observation.screenshotSettings, {
    headless: true,
    hideScrollbars: true,
  });
  assert.deepEqual([observation.viewport.width, observation.viewport.height], [300, 100]);
  assert.equal(observation.viewport.deviceScaleFactorMatched, true);
  assert.deepEqual(observation.viewport.visualViewport, [300, 100, 1]);
  assert.equal(observation.viewport.scrollX, 0);
  assert.equal(observation.viewport.scrollY, 0);
  assert.equal(observation.viewport.visualScale, 1);
  assert.equal(observation.viewport.colorGamut, 'srgb');
  assert.equal(observation.viewport.screenshotColorProfile, null);

  const expectedFixtureSources = [
    ['wrapper', 'html', 'htmlSha256'],
    ['domCss', 'fixture', 'fixtureSha256'],
    ['agentEval', 'agentEval', 'agentEvalSha256'],
  ];
  const geometryObservationBytes = readFileSync(
    path.resolve(repositoryRoot, observation.geometryOracle.file),
  );
  assert.equal(sha256(geometryObservationBytes), observation.geometryOracle.sha256,
    '13-case Chrome geometry observation SHA-256이 다릅니다.');
  const geometryObservation = JSON.parse(geometryObservationBytes);
  validateObservation(geometryObservation, repositoryRoot);
  for (const [key, sourceKey, hashKey] of expectedFixtureSources) {
    const source = observation.fixture[key];
    assert.equal(source.file, geometryObservation.sources[sourceKey],
      `${key} 경로가 geometry observation과 다릅니다.`);
    assert.equal(source.sha256, geometryObservation.sources[hashKey],
      `${key} SHA-256이 geometry observation과 다릅니다.`);
    const sourceBytes = readFileSync(path.resolve(repositoryRoot, source.file));
    assert.equal(sha256(sourceBytes), source.sha256,
      `${key} source hash가 Chrome observation과 다릅니다.`);
  }
  const sourceURL = new URL(observation.page);
  assert.equal(sourceURL.pathname,
    '/tests/fixtures/css/c13/c13-1-compact-surface-matrix.html');
  const screenshotPath = path.resolve(repositoryRoot, observation.screenshot.file);
  const screenshotBytes = readFileSync(screenshotPath);
  assert.equal(sha256(screenshotBytes), observation.screenshot.sha256,
    'Chrome DPR screenshot SHA-256이 observation과 다릅니다.');
  const reference = decodeRgbaPng(screenshotBytes);
  assert.deepEqual([reference.width, reference.height],
    [observation.screenshot.width, observation.screenshot.height],
    'Chrome screenshot dimensions가 observation과 다릅니다.');
  assert.equal(reference.width * reference.height,
    observation.screenshot.width * observation.screenshot.height);

  return {
    observation,
    observationSha256: sha256(observationBytes),
    screenshotSha256: sha256(screenshotBytes),
    fixtureSourceHashes: Object.fromEntries(expectedFixtureSources.map(([key]) =>
      [key, observation.fixture[key].sha256])),
    geometryObservation,
    geometryObservationSha256: sha256(geometryObservationBytes),
    reference,
  };
}

export function outsideBorderPhysicalSamples({ geometryObservation, reference, scaleX, scaleY }) {
  const samples = [];
  for (const item of geometryObservation.cases) {
    const { x, y, width, height } = item.frame.box;
    for (const sample of item.samples) {
      if (sample.role !== 'target-beyond-frame') continue;
      if (sample.x >= x && sample.x < x + width && sample.y >= y && sample.y < y + height) continue;
      const physicalX = Math.floor((sample.x + 0.5) * scaleX);
      const physicalY = Math.floor((sample.y + 0.5) * scaleY);
      assert.ok(physicalX >= 0 && physicalX < reference.width
        && physicalY >= 0 && physicalY < reference.height,
      `border 밖 표본이 DPR screenshot 범위를 벗어났습니다: ${physicalX},${physicalY}`);
      samples.push({
        caseId: item.id,
        css: [sample.x, sample.y],
        frame: { x, y, width, height },
        physicalPixelCenter: [physicalX, physicalY],
        chromeRgba: reference.getPixel(physicalX, physicalY),
      });
    }
  }
  assert.ok(samples.length > 0, 'container border 바깥 paint 좌표가 없습니다.');
  assert.ok(samples.some((sample) => sample.caseId === 'positioned-escape'
    && sample.css[0] === 270 && sample.css[1] === 72),
  '고정 positioned-escape border 바깥 표본이 없습니다.');
  return samples;
}

export function compareMetalFullSurfaceFiles({
  manifestPath,
  rgbaPath,
  observationPath,
  expectedIdentity,
}) {
  const manifestFilePath = path.resolve(manifestPath);
  const rgbaFilePath = path.resolve(rgbaPath);
  const manifestBytes = readFileSync(manifestFilePath);
  const manifest = JSON.parse(manifestBytes);
  const rgbaBytes = readFileSync(rgbaFilePath);
  assert.equal(path.basename(rgbaFilePath), manifest.sampleFile,
    'packed RGBA 파일명과 manifest가 다릅니다.');
  const chrome = loadFullSurfaceChromeObservation(observationPath);
  assert.deepEqual([chrome.reference.width, chrome.reference.height], [900, 300]);
  assert.equal(chrome.observation.viewport.deviceScaleFactor, 3);
  const comparison = compareMetalFullSurfaceArtifact({
    manifest,
    rgbaBytes,
    reference: chrome.reference,
    expectedIdentity,
  });
  return {
    ...comparison,
    outsideBorderSamples: outsideBorderPhysicalSamples({
      geometryObservation: chrome.geometryObservation,
      reference: chrome.reference,
      scaleX: 3,
      scaleY: 3,
    }),
    oracle: {
      browser: `${chrome.observation.browser.product} ${chrome.observation.browser.version}`,
      revision: chrome.observation.browser.revision,
      screenshotSha256: chrome.screenshotSha256,
      observationSha256: chrome.observationSha256,
      geometryObservationSha256: chrome.geometryObservationSha256,
      fixtureSourceHashes: chrome.fixtureSourceHashes,
    },
    artifact: {
      manifestFile: path.relative(repositoryRoot, manifestFilePath),
      manifestSha256: sha256(manifestBytes),
      rgbaFile: path.relative(repositoryRoot, rgbaFilePath),
      rgbaSha256: sha256(rgbaBytes),
    },
  };
}

export function compareAndroidPixelCopyFiles({
  actualPath, observationPath, runIdentity, width = 900, height = 300,
}) {
  const actualBytes = readFileSync(path.resolve(actualPath));
  const chrome = loadFullSurfaceChromeObservation(observationPath);
  assert.deepEqual([chrome.reference.width, chrome.reference.height], [width, height]);
  assert.deepEqual([chrome.observation.viewport.deviceScaleFactor,
    chrome.observation.viewport.reportedDeviceScaleFactor], [3, 3]);
  const actual = decodeRgbaPng(actualBytes);
  const comparison = compareAndroidPixelCopyPng({
    reference: chrome.reference,
    actual,
    runIdentity: { ...runIdentity, pngSha256: sha256(actualBytes) },
    width,
    height,
  });
  return {
    ...comparison,
    outsideBorderSamples: outsideBorderPhysicalSamples({
      geometryObservation: chrome.geometryObservation,
      reference: chrome.reference,
      scaleX: width / 300,
      scaleY: height / 100,
    }),
    oracle: {
      browser: `${chrome.observation.browser.product} ${chrome.observation.browser.version}`,
      revision: chrome.observation.browser.revision,
      screenshotSha256: chrome.screenshotSha256,
      observationSha256: chrome.observationSha256,
      geometryObservationSha256: chrome.geometryObservationSha256,
      fixtureSourceHashes: chrome.fixtureSourceHashes,
    },
    pixelCopy: {
      file: path.relative(repositoryRoot, path.resolve(actualPath)),
      sha256: sha256(actualBytes),
    },
  };
}
