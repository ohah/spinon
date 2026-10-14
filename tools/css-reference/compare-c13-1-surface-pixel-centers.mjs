import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { validateObservation } from './capture-c13-1-compact-surface-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(scriptDirectory, '../..');
const evidenceDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-1-full-surface-pixel-centers-2026-10-14',
);
const oracleEvidenceDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14',
);
const defaultActualPng = path.join(evidenceDirectory, 'android-api37-avd-pixelcopy.png');
const defaultObservationPath = path.join(oracleEvidenceDirectory, 'chrome155-observation.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

export function compareCssPixelCenters(reference, actual) {
  assert.ok(reference && actual, '기준 이미지와 PixelCopy 이미지가 필요합니다.');
  for (const [label, image] of [['기준', reference], ['PixelCopy', actual]]) {
    assert.ok(Number.isSafeInteger(image.width) && image.width > 0, `${label} 이미지 너비가 잘못되었습니다.`);
    assert.ok(Number.isSafeInteger(image.height) && image.height > 0, `${label} 이미지 높이가 잘못되었습니다.`);
    assert.equal(typeof image.getPixel, 'function', `${label} 이미지 픽셀 판독기가 없습니다.`);
  }

  const scaleX = actual.width / reference.width;
  const scaleY = actual.height / reference.height;
  assert.ok(scaleX >= 1 && scaleY >= 1,
    'PixelCopy 이미지가 CSS 기준 표면보다 작아 픽셀 중심을 일대일로 대응할 수 없습니다.');

  const mismatches = [];
  const sampledCoordinates = new Set();
  let comparedPixels = 0;
  let maximumChannelError = 0;
  for (let yCssPx = 0; yCssPx < reference.height; yCssPx += 1) {
    const yPixel = Math.floor((yCssPx + 0.5) * scaleY);
    assert.ok(yPixel >= 0 && yPixel < actual.height, `PixelCopy y 표본 범위 오류: ${yPixel}`);
    for (let xCssPx = 0; xCssPx < reference.width; xCssPx += 1) {
      const xPixel = Math.floor((xCssPx + 0.5) * scaleX);
      assert.ok(xPixel >= 0 && xPixel < actual.width, `PixelCopy x 표본 범위 오류: ${xPixel}`);
      const coordinate = `${xPixel},${yPixel}`;
      assert.equal(sampledCoordinates.has(coordinate), false,
        `서로 다른 CSS 픽셀 중심이 같은 PixelCopy 표본으로 매핑됩니다: ${coordinate}`);
      sampledCoordinates.add(coordinate);

      const expected = reference.getPixel(xCssPx, yCssPx);
      const observed = actual.getPixel(xPixel, yPixel);
      assert.equal(expected.length, 4, `기준 픽셀이 RGBA가 아닙니다: ${xCssPx},${yCssPx}`);
      assert.equal(observed.length, 4, `PixelCopy 픽셀이 RGBA가 아닙니다: ${xPixel},${yPixel}`);
      const channelErrors = expected.map((channel, index) => Math.abs(channel - observed[index]));
      maximumChannelError = Math.max(maximumChannelError, ...channelErrors);
      comparedPixels += 1;
      if (channelErrors.some((error) => error !== 0)) {
        mismatches.push({
          css: [xCssPx, yCssPx],
          pixelCopy: [xPixel, yPixel],
          expectedRgba: expected,
          actualRgba: observed,
        });
      }
    }
  }

  const mismatchBounds = mismatches.length === 0 ? null : {
    minX: Math.min(...mismatches.map(({ css: [x] }) => x)),
    minY: Math.min(...mismatches.map(({ css: [, y] }) => y)),
    maxX: Math.max(...mismatches.map(({ css: [x] }) => x)),
    maxY: Math.max(...mismatches.map(({ css: [, y] }) => y)),
  };
  return {
    status: mismatches.length === 0 ? 'matched' : 'mismatch',
    referenceCssSize: [reference.width, reference.height],
    pixelCopySize: [actual.width, actual.height],
    scale: [scaleX, scaleY],
    comparedPixels,
    comparedChannels: comparedPixels * 4,
    mismatchedPixels: mismatches.length,
    maximumChannelError,
    mismatchBounds,
    mismatches,
  };
}

export function comparePngFiles({ referencePath, actualPath, observationPath = defaultObservationPath }) {
  const observation = JSON.parse(readFileSync(observationPath, 'utf8'));
  validateObservation(observation, repositoryRoot);
  const resolvedReferencePath = path.resolve(referencePath ?? path.join(repositoryRoot, observation.screenshot.file));
  const resolvedActualPath = path.resolve(actualPath ?? defaultActualPng);
  const referenceBytes = readFileSync(resolvedReferencePath);
  const actualBytes = readFileSync(resolvedActualPath);
  const reference = decodeRgbaPng(referenceBytes);
  const actual = decodeRgbaPng(actualBytes);
  assert.deepEqual([reference.width, reference.height], [observation.viewport.width, observation.viewport.height],
    'Chrome reference 크기가 관측 viewport와 다릅니다.');
  const result = compareCssPixelCenters(reference, actual);
  return {
    ...result,
    oracle: {
      browser: observation.browser.chromeVersion,
      revision: observation.browser.revision,
      viewportCssPx: [observation.viewport.width, observation.viewport.height],
      screenshotSha256: observation.screenshot.sha256,
      observationPath: path.relative(repositoryRoot, observationPath),
    },
    pixelCopy: {
      file: path.relative(repositoryRoot, resolvedActualPath),
      sha256: sha256(actualBytes),
    },
  };
}

async function main() {
  const [, , actualArgument, outputArgument] = process.argv;
  const result = comparePngFiles({ actualPath: actualArgument });
  const outputPath = outputArgument ? path.resolve(outputArgument) : null;
  if (outputPath) writeFileSync(outputPath, `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify({
    status: result.status,
    comparedPixels: result.comparedPixels,
    comparedChannels: result.comparedChannels,
    mismatchedPixels: result.mismatchedPixels,
    maximumChannelError: result.maximumChannelError,
    mismatchBounds: result.mismatchBounds,
    scale: result.scale,
    pixelCopySize: result.pixelCopySize,
    outputPath: outputPath ? path.relative(repositoryRoot, outputPath) : null,
  }, null, 2));
  if (result.status !== 'matched') process.exitCode = 1;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main();
}
