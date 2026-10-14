import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { validateObservation } from './capture-c13-1-compact-surface-matrix.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(scriptDirectory, '../..');
const oracleDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14',
);
const defaultObservationPath = path.join(oracleDirectory, 'chrome155-observation.json');
const defaultReferencePath = path.join(oracleDirectory, 'chrome155-surface.png');
const sampleKeys = [
  'cssX', 'cssY', 'drawableX', 'drawableY', 'generation', 'index', 'rgba', 'runId',
].sort();
const supportedFormats = new Set([
  'Rgba8Unorm', 'Rgba8UnormSrgb', 'Bgra8Unorm', 'Bgra8UnormSrgb',
]);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function assertInteger(value, label, minimum = 0) {
  assert.ok(Number.isSafeInteger(value) && value >= minimum, `${label} 정수가 잘못되었습니다.`);
}

function borderOverflowOracleSamples(observation) {
  const found = [];
  for (const item of observation.cases) {
    for (const sample of item.samples) {
      if (sample.role !== 'target-beyond-frame') continue;
      const { x, y, width, height } = item.frame.box;
      if (sample.x < x || sample.x >= x + width || sample.y < y || sample.y >= y + height) {
        found.push({ caseId: item.id, css: [sample.x, sample.y], expectedRgba: sample.rgba });
      }
    }
  }
  return found;
}

export function compareMetalPixelArtifacts({
  manifest,
  sampleBytes,
  reference,
  observation,
}) {
  assert.ok(manifest && typeof manifest === 'object', 'Metal manifest JSON이 필요합니다.');
  assert.equal(manifest.schema, 'spinon-c13-1-ios-metal-pixel-centers/v1');
  assert.equal(manifest.status, 'complete', '미완성 Metal capture는 비교할 수 없습니다.');
  assert.match(manifest.runId, /^[a-f0-9]{32}$/);
  assertInteger(manifest.generation, 'surface generation', 1);
  assert.deepEqual(manifest.viewportCssPx, [300, 100]);
  assert.equal(Array.isArray(manifest.drawablePixelSize), true);
  assert.equal(manifest.drawablePixelSize.length, 2);
  const [drawableWidth, drawableHeight] = manifest.drawablePixelSize;
  assertInteger(drawableWidth, 'drawable width', 300);
  assertInteger(drawableHeight, 'drawable height', 100);
  assert.ok(supportedFormats.has(manifest.surfaceFormat), '지원하지 않는 WGPU texture format입니다.');
  assert.equal(manifest.rgbaOrder, 'RGBA8');
  assert.equal(manifest.sampleCount, 30_000);
  assert.equal(manifest.batchSize, 64);
  assert.equal(manifest.batchCount, 469);
  assertInteger(manifest.processID, 'process ID', 1);
  assertInteger(manifest.elapsedMilliseconds, 'elapsed milliseconds');
  assert.ok(manifest.elapsedMilliseconds <= 300_000, '전체 capture 제한 시간을 넘었습니다.');
  assert.equal(path.basename(manifest.sampleFile), manifest.sampleFile,
    'sample file은 artifact directory 안의 파일명이어야 합니다.');
  assert.match(manifest.sampleSha256, /^[a-f0-9]{64}$/);
  assert.equal(Buffer.isBuffer(sampleBytes), true, 'JSONL artifact bytes가 필요합니다.');
  assert.equal(sha256(sampleBytes), manifest.sampleSha256, 'JSONL SHA-256이 manifest와 다릅니다.');

  const scaleX = drawableWidth / 300;
  const scaleY = drawableHeight / 100;
  assert.deepEqual(manifest.scaleX, scaleX);
  assert.deepEqual(manifest.scaleY, scaleY);
  assert.ok(reference && observation, 'Chrome 155 oracle PNG와 observation이 필요합니다.');
  assert.deepEqual([reference.width, reference.height], [300, 100]);
  assert.deepEqual([observation.viewport.width, observation.viewport.height], [300, 100]);

  const lines = sampleBytes.toString('utf8').split('\n');
  if (lines.at(-1) === '') lines.pop();
  assert.equal(lines.length, 30_000, 'JSONL 표본 수가 30,000개가 아닙니다.');
  const drawableCoordinates = new Set();
  const mismatches = [];
  let comparedChannels = 0;
  let maximumChannelError = 0;
  for (let index = 0; index < lines.length; index += 1) {
    let sample;
    try {
      sample = JSON.parse(lines[index]);
    } catch {
      assert.fail(`JSONL ${index}행이 올바른 JSON이 아닙니다.`);
    }
    assert.deepEqual(Object.keys(sample).sort(), sampleKeys, `JSONL ${index}행 shape`);
    assert.equal(sample.index, index, `JSONL ${index}행 index/순서`);
    const cssX = index % 300;
    const cssY = Math.floor(index / 300);
    assert.equal(sample.cssX, cssX, `JSONL ${index}행 CSS x/row-major 순서`);
    assert.equal(sample.cssY, cssY, `JSONL ${index}행 CSS y/row-major 순서`);
    assert.equal(sample.runId, manifest.runId, `JSONL ${index}행 run ID`);
    assert.equal(sample.generation, manifest.generation, `JSONL ${index}행 surface generation`);
    assertInteger(sample.drawableX, `JSONL ${index}행 drawable x`);
    assertInteger(sample.drawableY, `JSONL ${index}행 drawable y`);
    const expectedX = Math.floor((cssX + 0.5) * scaleX);
    const expectedY = Math.floor((cssY + 0.5) * scaleY);
    assert.deepEqual([sample.drawableX, sample.drawableY], [expectedX, expectedY],
      `JSONL ${index}행 pixel-center 매핑`);
    assert.ok(sample.drawableX < drawableWidth && sample.drawableY < drawableHeight,
      `JSONL ${index}행 drawable 범위`);
    const coordinate = `${sample.drawableX},${sample.drawableY}`;
    assert.equal(drawableCoordinates.has(coordinate), false,
      `JSONL ${index}행이 이미 사용한 drawable pixel을 재사용합니다.`);
    drawableCoordinates.add(coordinate);
    assert.equal(Array.isArray(sample.rgba), true, `JSONL ${index}행 RGBA 배열`);
    assert.equal(sample.rgba.length, 4, `JSONL ${index}행 RGBA 길이`);
    for (const [channel, value] of sample.rgba.entries()) {
      assertInteger(value, `JSONL ${index}행 RGBA channel ${channel}`);
      assert.ok(value <= 255, `JSONL ${index}행 RGBA channel ${channel} 범위`);
    }
    const expected = reference.getPixel(cssX, cssY);
    const channelErrors = expected.map((value, channel) => Math.abs(value - sample.rgba[channel]));
    maximumChannelError = Math.max(maximumChannelError, ...channelErrors);
    comparedChannels += 4;
    if (channelErrors.some((error) => error !== 0)) {
      mismatches.push({
        index,
        css: [cssX, cssY],
        drawable: [sample.drawableX, sample.drawableY],
        expectedRgba: expected,
        actualRgba: sample.rgba,
      });
    }
  }
  assert.equal(drawableCoordinates.size, 30_000);
  const outsideBorderSamples = borderOverflowOracleSamples(observation);
  assert.ok(outsideBorderSamples.length > 0,
    'Chrome oracle에서 container border 바깥의 painted overflow 표본을 찾지 못했습니다.');

  return {
    status: mismatches.length === 0 ? 'matched' : 'mismatch',
    runId: manifest.runId,
    generation: manifest.generation,
    processID: manifest.processID,
    viewportCssPx: manifest.viewportCssPx,
    drawablePixelSize: manifest.drawablePixelSize,
    surfaceFormat: manifest.surfaceFormat,
    rgbaOrder: manifest.rgbaOrder,
    scale: [scaleX, scaleY],
    sampleCount: lines.length,
    comparedChannels,
    mismatchedPixels: mismatches.length,
    maximumChannelError,
    sampleSha256: manifest.sampleSha256,
    outsideBorderSamples,
    mismatches,
  };
}

export function compareMetalArtifactFiles({ manifestPath, samplePath, referencePath, observationPath }) {
  const resolvedManifestPath = path.resolve(manifestPath);
  const resolvedSamplePath = path.resolve(samplePath);
  const resolvedObservationPath = path.resolve(observationPath ?? defaultObservationPath);
  const resolvedReferencePath = path.resolve(referencePath ?? defaultReferencePath);
  const manifest = JSON.parse(readFileSync(resolvedManifestPath, 'utf8'));
  const sampleBytes = readFileSync(resolvedSamplePath);
  assert.equal(path.basename(resolvedSamplePath), manifest.sampleFile,
    'JSONL 파일명이 manifest sampleFile과 다릅니다.');
  const observationBytes = readFileSync(resolvedObservationPath);
  const observation = JSON.parse(observationBytes);
  validateObservation(observation, repositoryRoot);
  const referenceBytes = readFileSync(resolvedReferencePath);
  assert.equal(observation.screenshot.sha256, sha256(referenceBytes),
    'Chrome oracle PNG SHA-256이 observation과 다릅니다.');
  const reference = decodeRgbaPng(referenceBytes);
  return {
    ...compareMetalPixelArtifacts({ manifest, sampleBytes, reference, observation }),
    oracle: {
      browser: observation.browser.chromeVersion,
      revision: observation.browser.revision,
      observationSha256: sha256(observationBytes),
      screenshotSha256: sha256(referenceBytes),
      observationFile: path.relative(repositoryRoot, resolvedObservationPath),
      screenshotFile: path.relative(repositoryRoot, resolvedReferencePath),
    },
    artifact: {
      manifestFile: path.relative(repositoryRoot, resolvedManifestPath),
      manifestSha256: sha256(readFileSync(resolvedManifestPath)),
      sampleFile: path.relative(repositoryRoot, resolvedSamplePath),
      sampleSha256: sha256(sampleBytes),
    },
  };
}

async function main() {
  const [, , manifestPath, samplePath, outputPath] = process.argv;
  assert.ok(manifestPath && samplePath,
    '사용법: node compare-c13-1-ios-metal-pixel-centers.mjs <manifest.json> <samples.jsonl> [output.json]');
  const result = compareMetalArtifactFiles({ manifestPath, samplePath });
  if (outputPath) {
    writeFileSync(path.resolve(outputPath), `${JSON.stringify(result, null, 2)}\n`);
  }
  console.log(JSON.stringify({
    status: result.status,
    runId: result.runId,
    generation: result.generation,
    sampleCount: result.sampleCount,
    comparedChannels: result.comparedChannels,
    mismatchedPixels: result.mismatchedPixels,
    maximumChannelError: result.maximumChannelError,
    outsideBorderSamples: result.outsideBorderSamples.length,
    drawablePixelSize: result.drawablePixelSize,
    surfaceFormat: result.surfaceFormat,
    outputFile: outputPath ? path.relative(repositoryRoot, path.resolve(outputPath)) : null,
  }, null, 2));
  if (result.status !== 'matched') process.exitCode = 1;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main();
}
