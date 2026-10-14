import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  compareC132SurfacePixelCapture,
  compareC132SurfacePixelRuns,
  decodeC132SurfacePng,
  parseC132SurfacePixelLog,
  repositoryRoot,
  validateC132ChromeOracle,
} from './compare-c13-2-visible-overflow-surface-pixels.mjs';

const evidenceDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-2-visible-overflow-surface-pixels-2026-10-15',
);
const planDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-2-visible-overflow-surface-pixels-plan-2026-10-15',
);
const runID = 'c132-surface-20261015-run5';
const androidLog = readFileSync(path.join(evidenceDirectory, 'android-pixelcopy-run5.log'), 'utf8');
const iosLog = readFileSync(path.join(evidenceDirectory, 'ios-metal-run5.log'), 'utf8');
const androidPngBytes = readFileSync(path.join(evidenceDirectory, 'android-api37-avd-pixelcopy-run5.png'));
const androidImage = decodeC132SurfacePng(androidPngBytes);

function parsed() {
  return {
    android: parseC132SurfacePixelLog('android', androidLog),
    ios: parseC132SurfacePixelLog('ios', iosLog),
  };
}

function chromeOracle() {
  const observation = JSON.parse(readFileSync(path.join(planDirectory, 'chrome155-observation.json'), 'utf8'));
  return validateC132ChromeOracle({
    observation,
    screenshotBytes: readFileSync(path.join(planDirectory, observation.screenshot.file)),
    fixtureBytes: readFileSync(path.join(repositoryRoot, observation.fixture)),
    sourceFixtureBytes: readFileSync(path.join(repositoryRoot, observation.source_fixture)),
  });
}

test('고정 Chrome oracle PNG·fixture hash와 Android/iOS 5개 표본이 일치한다', () => {
  const captures = parsed();
  const result = compareC132SurfacePixelRuns({
    androidCapture: captures.android,
    androidImage,
    iosCapture: captures.ios,
    expectedRunId: runID,
  });
  assert.equal(result.status, 'matched');
  assert.equal(result.platforms.android.samples.length, 5);
  assert.equal(result.platforms.ios.samples.length, 5);
  assert.equal(result.platforms.android.samples[0].pixel[0] + 0.5 > 130 * 2.625, true);
  assert.deepEqual(result.platforms.android.revisions, {
    document: 583, renderTree: 124, style: 0, environment: 0, scroll: 8,
  });
  assert.deepEqual(result.platforms.ios.revisions, {
    document: 583, renderTree: 124, style: 0, environment: 1, scroll: 8,
  });
  assert.equal(chromeOracle().browser, '155.0.8059.40');
});

test('escaped sample 색상 변조와 parent 안 좌표 이동을 거부한다', () => {
  const { android } = parsed();
  const changedColor = structuredClone(android);
  changedColor.samples[0].rgba[0] -= 1;
  assert.throws(() => compareC132SurfacePixelCapture(changedColor, {
    expectedRunId: runID,
    imageInfo: androidImage,
  }), /visible-escape RGBA/);

  const changedCoordinate = structuredClone(android);
  changedCoordinate.samples[0].x = 129 * 2.625;
  assert.throws(() => compareC132SurfacePixelCapture(changedCoordinate, {
    expectedRunId: runID,
    imageInfo: androidImage,
  }), /visible-escape surface coordinate/);
});

test('표본 누락과 중복을 거부한다', () => {
  const { ios } = parsed();
  const missing = structuredClone(ios);
  missing.samples.pop();
  assert.throws(() => compareC132SurfacePixelCapture(missing, { expectedRunId: runID }), /표본 개수/);

  const duplicate = structuredClone(ios);
  duplicate.samples[1] = structuredClone(duplicate.samples[0]);
  assert.throws(() => compareC132SurfacePixelCapture(duplicate, { expectedRunId: runID }), /순서\/ID/);
});

test('stale run ID와 surface generation을 거부한다', () => {
  const { ios } = parsed();
  const staleRun = structuredClone(ios);
  staleRun.samples[0].runId = 'old-run';
  assert.throws(() => compareC132SurfacePixelCapture(staleRun, { expectedRunId: runID }), /run ID/);

  const staleGeneration = structuredClone(ios);
  staleGeneration.samples[0].surfaceGeneration += 1;
  assert.throws(() => compareC132SurfacePixelCapture(staleGeneration, { expectedRunId: runID }), /surface generation/);
});

test('잘못된 PixelCopy PNG 크기와 hash를 거부한다', () => {
  const { android } = parsed();
  assert.throws(() => compareC132SurfacePixelCapture(android, {
    expectedRunId: runID,
    imageInfo: { ...androidImage, width: androidImage.width - 1 },
  }), /PNG 크기/);
  assert.throws(() => compareC132SurfacePixelCapture(android, {
    expectedRunId: runID,
    imageInfo: { ...androidImage, sha256: '0'.repeat(64) },
  }), /PNG SHA-256/);
});

test('Chrome 기준 관찰값·fixture·PNG 변경을 거부한다', () => {
  const observation = JSON.parse(readFileSync(path.join(planDirectory, 'chrome155-observation.json'), 'utf8'));
  const screenshotBytes = readFileSync(path.join(planDirectory, observation.screenshot.file));
  const fixtureBytes = readFileSync(path.join(repositoryRoot, observation.fixture));
  const sourceFixtureBytes = readFileSync(path.join(repositoryRoot, observation.source_fixture));
  assert.throws(() => validateC132ChromeOracle({
    observation: { ...observation, fixture_sha256: '0'.repeat(64) },
    screenshotBytes,
    fixtureBytes,
    sourceFixtureBytes,
  }), /strictly equal/);
  assert.throws(() => validateC132ChromeOracle({
    observation,
    screenshotBytes: Buffer.concat([screenshotBytes, Buffer.from('mutation')]),
    fixtureBytes,
    sourceFixtureBytes,
  }), /strictly equal/);
});
