import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import {
  c1133IOSSceneRunID,
  compareC1133IOSSceneGeometry,
  isC1133IOSGridRuntimeSuperseded,
  validateC1133IOSMatrixInputs,
  validateC1133IOSRejection,
  validateC1133IOSUnknownScene,
} from './c11-3-3-grid-template-areas-ios-matrix.mjs';
import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runtimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v2.json';
const historicalRuntimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
const historicalRuntimeBytes = await readFile(resolve(repositoryRoot, historicalRuntimePath));
const historicalRuntimeSha256 = sha256(historicalRuntimeBytes);
const mobilePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-mobile-surface-pixel-matrix-v1.json';
const runtime = JSON.parse(await readFile(resolve(repositoryRoot, runtimePath), 'utf8'));
const mobile = JSON.parse(await readFile(resolve(repositoryRoot, mobilePath), 'utf8'));
const cyclePixelsPath = 'tests/fixtures/css/c11/grid-template-areas-track-list-cycling-mobile-pixels-v1.json';
const cyclePixels = JSON.parse(await readFile(resolve(repositoryRoot, cyclePixelsPath), 'utf8'));
const cycleObservationBytes = await readFile(resolve(repositoryRoot, cyclePixels.inputs.chromeObservation.path));
const cycleScreenshotBytes = await readFile(resolve(repositoryRoot, cyclePixels.oracle.screenshotPath));
const typedAutoPixelsPath = 'tests/fixtures/css/c11/grid-template-areas-typed-auto-track-sizing-mobile-pixels-v1.json';
const typedAutoPixelsBytes = await readFile(resolve(repositoryRoot, typedAutoPixelsPath));
const typedAutoPixels = JSON.parse(typedAutoPixelsBytes.toString('utf8'));
const typedAutoHtmlBytes = await readFile(resolve(repositoryRoot, typedAutoPixels.inputs.html.path));
const typedAutoObservationBytes = await readFile(resolve(repositoryRoot, typedAutoPixels.inputs.observation.path));
const typedAutoScreenshotBytes = await readFile(resolve(repositoryRoot, typedAutoPixels.oracle.screenshotPath));
assert.equal(mobile.inputs.runtimeMatrix.sha256, historicalRuntimeSha256);
assert.equal(cyclePixels.inputs.runtimeMatrix.sha256, historicalRuntimeSha256);
assert.deepEqual(runtime.inputs.historicalRuntimeMatrix, {
  path: historicalRuntimePath, sha256: historicalRuntimeSha256,
});
assert.equal(cyclePixels.inputs.chromeObservation.sha256, sha256(cycleObservationBytes));
assert.equal(cyclePixels.oracle.observationSha256, sha256(cycleObservationBytes));
assert.equal(cyclePixels.oracle.screenshotSha256, sha256(cycleScreenshotBytes));
assert.equal(cyclePixels.scenes.length, 6);
const cyclePixelScenes = new Map(cyclePixels.scenes.map((entry) => [entry.sceneId, entry]));
assert.equal(typedAutoPixels.schema,
  'spinon-css-c11-3-3-template-area-typed-auto-track-sizing-mobile-pixels/v1');
assert.equal(typedAutoPixels.inputs.html.sha256, sha256(typedAutoHtmlBytes));
assert.equal(typedAutoPixels.inputs.observation.sha256, sha256(typedAutoObservationBytes));
assert.equal(typedAutoPixels.oracle.observationSha256, sha256(typedAutoObservationBytes));
assert.equal(typedAutoPixels.oracle.screenshotSha256, sha256(typedAutoScreenshotBytes));
const typedAutoPixelScenes = new Map(typedAutoPixels.scenes.map((entry) => [entry.sceneId, entry]));
const reference = JSON.parse(await readFile(resolve(repositoryRoot, mobile.oracle.referencePath), 'utf8'));
const observation = validateC1133IOSMatrixInputs(runtime, reference);
const unknownAreaPixelsPath = 'tests/fixtures/css/c11/unknown-area-fixed-implicit-tracks-mobile-pixels-v1.json';
const unknownAreaPixels = JSON.parse(await readFile(resolve(repositoryRoot, unknownAreaPixelsPath), 'utf8'));
const unknownAreaReferenceBytes = await readFile(resolve(repositoryRoot, unknownAreaPixels.inputs.reference.path));
const unknownAreaReference = JSON.parse(unknownAreaReferenceBytes.toString('utf8'));
const leadingNamedLinesPixelsPath = 'tests/fixtures/css/c11/grid-leading-named-implicit-lines-mobile-pixels-v1.json';
const leadingNamedLinesPixels = JSON.parse(await readFile(resolve(repositoryRoot, leadingNamedLinesPixelsPath), 'utf8'));
const leadingNamedLinesReferenceBytes = await readFile(resolve(repositoryRoot,
  leadingNamedLinesPixels.oracle.referencePath));
const leadingNamedLinesReference = JSON.parse(leadingNamedLinesReferenceBytes.toString('utf8'));
const unknownAreaObservation = unknownAreaReference.observations.find(({ deviceScaleFactor }) =>
  deviceScaleFactor === 1);
const unknownAreaChromeSourceCase = unknownAreaObservation.cases.find(({ id }) =>
  id === 'unknown-template-area-with-sibling');
const unknownAreaScene = {
  id: unknownAreaPixels.sceneId,
  index: runtime.summary.sceneCount,
  nodeIds: ['fixed-implicit-grid', 'fixed-known-area', 'fixed-unknown-area'],
  expected: { outcome: 'success' },
};
const unknownAreaChromeCase = {
  id: unknownAreaScene.id,
  nodes: unknownAreaChromeSourceCase.nodes.map((node, index) => ({
    ...node,
    id: unknownAreaScene.nodeIds[index],
  })),
};
const simulatorUDID = process.argv[2] ?? 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
assert.match(simulatorUDID, /^[A-F0-9-]{36}$/i, '사용법: node capture-c11-3-3-grid-template-areas-ios-simulator-matrix.mjs [iOS Simulator UDID]');

const packageName = 'dev.spinon.bootstrap';
const runID = 'c1133-ios-matrix-' + new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')
  + '-' + randomUUID().slice(0, 8);
const appRunIDPrefix = 'c1133m-' + new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')
  + '-' + randomUUID().slice(0, 8);
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  'spec/internal/evidence/c11-3-3-grid-template-areas-runtime-' + date, 'ios-simulator-' + runID);
const iosAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const iosBinaryPath = resolve(iosAppPath, 'SpinonBootstrap');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(binary + ' ' + args.join(' ') + ' 실패: '
      + (result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()));
    error.result = result;
    throw error;
  }
  return result;
}

async function saveLog(path, result) {
  const log = '' + (result.stdout ?? '') + (result.stderr ?? '');
  await writeFile(resolve(evidenceDirectory, path), log.replace(/[ \t]+$/gm, ''));
}

function formatLog(log) {
  return log.replace(/[ \t]+$/gm, '');
}

function bootedSimulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const matches = Object.entries(devices).flatMap(([runtimeName, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime: runtimeName })));
  assert.equal(matches.length, 1, 'Booted iOS Simulator를 찾지 못했습니다: ' + simulatorUDID);
  assert.match(matches[0].runtime, /iOS-26-2$/,
    'iOS 26.2 Simulator가 아닙니다: ' + matches[0].runtime);
  assert.equal(matches[0].name, 'iPhone 17 Pro', '고정 iPhone 17 Pro Simulator가 아닙니다.');
  return matches[0];
}

function iosProcessLog(udid, pid) {
  const output = command('xcrun', ['simctl', 'spawn', udid, 'log', 'show', '--last', '5m',
    '--style', 'syslog', '--predicate', 'process == "SpinonBootstrap"'], { timeout: 45_000 }).stdout;
  const processMarker = new RegExp('SpinonBootstrap\\[' + pid + '(?::|\\])');
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForLog(readLog, predicate, message, timeout = 150_000) {
  const deadline = Date.now() + timeout;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    if (predicate(latest)) return latest;
    sleep(300);
  }
  throw new Error(message + ' 대기 시간 초과\n' + latest);
}

function launchIOSScene(simulator, sceneID, sceneRunID, samples) {
  const args = ['simctl', 'launch', '--terminate-running-process', simulator.udid, packageName,
    '--spinon-c1133-grid-template-areas',
    '--spinon-c1133-grid-run-id=' + sceneRunID,
    '--spinon-c1133-grid-scene=' + sceneID];
  if (samples) {
    args.push('--spinon-c11-grid-metal-surface-pixels=' + sceneRunID,
      '--spinon-c11-grid-metal-surface-samples=' + Buffer.from(JSON.stringify(samples)).toString('base64'));
  }
  return command('xcrun', args, { timeout: 90_000 });
}

function launchedPID(launch, description) {
  const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, description + ' launch가 PID를 반환하지 않았습니다.');
  return pid;
}

function c1133SuccessfulDrawAfterPaint(log, sceneID, sceneRunID) {
  const lines = log.split(/\r?\n/);
  const paintMarker = 'SPINON_C1133_GRID_PAINT_ORDER_RUN run_id=' + sceneRunID
    + ' scene_id=' + sceneID + ' ';
  const paintIndex = lines.findIndex((line) => line.includes(paintMarker));
  return paintIndex >= 0 && lines.some((line, index) => index > paintIndex
    && line.includes('SPINON_C0410_DRAW')
    && /\bstatus=0(?: presented)? boxes=\d+\b/.test(line));
}

function c1133SuccessLogsComplete(log, sceneID, sceneRunID, requireMetalResult) {
  const scoped = (marker) => log.includes(marker + ' run_id=' + sceneRunID + ' scene_id=' + sceneID + ' ');
  const successfulDraw = c1133SuccessfulDrawAfterPaint(log, sceneID, sceneRunID);
  const metalDone = !requireMetalResult || log.split(/\r?\n/).some((line) =>
    line.includes('SPINON_C1133_METAL_PIXEL_SUMMARY status=matched run_id=' + sceneRunID)
      || line.includes('SPINON_C1133_METAL_PIXEL_SUMMARY status=mismatch run_id=' + sceneRunID)
      || line.includes('SPINON_C1133_METAL_PIXEL_ERROR run_id=' + sceneRunID));
  return scoped('SPINON_C1133_GRID_SUMMARY')
    && scoped('SPINON_C1133_GRID_FRAME_SUMMARY')
    && scoped('SPINON_C1133_GRID_PAINT_ORDER_RUN')
    && successfulDraw && metalDone;
}

function c1133SupersededAttemptComplete(log, sceneID, sceneRunID, requireMetalResult) {
  const successfulDraw = c1133SuccessfulDrawAfterPaint(log, sceneID, sceneRunID);
  const metalDone = !requireMetalResult || log.split(/\r?\n/).some((line) =>
    line.includes('SPINON_C1133_METAL_PIXEL_SUMMARY status=matched run_id=' + sceneRunID)
      || line.includes('SPINON_C1133_METAL_PIXEL_SUMMARY status=mismatch run_id=' + sceneRunID)
      || line.includes('SPINON_C1133_METAL_PIXEL_ERROR run_id=' + sceneRunID));
  return isC1133IOSGridRuntimeSuperseded(log, sceneRunID, sceneID) && successfulDraw && metalDone;
}

async function captureLeadingNamedImplicitLines(simulator) {
  assert.equal(leadingNamedLinesPixels.schema,
    'spinon-css-c11-3-3-leading-named-implicit-lines-mobile-pixels/v1');
  assert.equal(sha256(leadingNamedLinesReferenceBytes),
    leadingNamedLinesPixels.oracle.referenceSha256);
  assert.equal(leadingNamedLinesReference.referenceId, leadingNamedLinesPixels.oracle.referenceId);
  assert.equal(leadingNamedLinesPixels.scenes.length, 7);
  const observations = [];
  for (const [index, entry] of leadingNamedLinesPixels.scenes.entries()) {
    const sceneIndex = runtime.summary.sceneCount + 1 + index;
    const scene = {
      id: entry.id,
      index: sceneIndex,
      nodeIds: [entry.rootId, entry.itemId],
      expected: { outcome: 'success' },
    };
    const chromeCase = {
      id: entry.id,
      nodes: [
        { id: entry.rootId, rect: Object.fromEntries(['x', 'y', 'width', 'height']
          .map((key, coordinate) => [key, entry.rootRect[coordinate]])) },
        { id: entry.itemId, rect: Object.fromEntries(['x', 'y', 'width', 'height']
          .map((key, coordinate) => [key, entry.itemRect[coordinate]])) },
      ],
    };
    const attempts = [];
    let effectiveRunID = '';
    let log = '';
    let logPath = '';
    for (let attempt = 0; attempt < 2; attempt += 1) {
      effectiveRunID = c1133IOSSceneRunID(appRunIDPrefix, sceneIndex, attempt);
      const launch = launchIOSScene(simulator, scene.id, effectiveRunID, entry.samples);
      const launchPath = `launch-leading-${String(index).padStart(2, '0')}-${scene.id}`
        + (attempt === 0 ? '' : `-retry-${attempt}`) + '.log';
      await saveLog(launchPath, launch);
      const pid = launchedPID(launch, scene.id);
      log = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
        c1133SuccessLogsComplete(value, scene.id, effectiveRunID, true)
          || c1133SupersededAttemptComplete(value, scene.id, effectiveRunID, true)
          || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'),
      scene.id + ': leading named implicit line iOS runtime 결과', 45_000);
      assert.equal(log.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'), false,
        `${scene.id}: iOS route에서 scene이 거부되었습니다.`);
      const completed = c1133SuccessLogsComplete(log, scene.id, effectiveRunID, true);
      const superseded = !completed && isC1133IOSGridRuntimeSuperseded(
        log, effectiveRunID, scene.id);
      logPath = `runtime-leading-${String(index).padStart(2, '0')}-${scene.id}`
        + (attempt === 0 ? '' : `-retry-${attempt}`) + '.log';
      await writeFile(resolve(evidenceDirectory, logPath), formatLog(log) + '\n');
      attempts.push({
        runID: effectiveRunID,
        status: completed ? 'matched' : 'superseded-retried',
        logPath,
      });
      if (completed) break;
      assert.ok(superseded && attempt === 0,
        `${scene.id}: 성공 또는 단회 startup superseded 외 결과가 나왔습니다.`);
      command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
    }
    const geometry = compareC1133IOSSceneGeometry(scene, log, chromeCase, effectiveRunID);
    const pixel = compareC11IOSMetalPixelLogs(log, {
      fixture: {
        sceneId: scene.id,
        runtime: leadingNamedLinesPixels.runtime,
        samples: entry.samples,
      },
      runID: effectiveRunID,
      markerPrefix: 'C1133',
    });
    assert.equal(pixel.status, 'matched', `${scene.id}: Metal 표면 표본이 일치하지 않습니다.`);
    assert.equal(pixel.matchedSamples, 1, `${scene.id}: Metal 표본 수가 다릅니다.`);
    assert.equal(pixel.maximumChannelError, 0, `${scene.id}: Metal RGBA가 다릅니다.`);
    const comparisonPath = `metal-leading-${scene.id}-comparison.json`;
    await writeFile(resolve(evidenceDirectory, comparisonPath), JSON.stringify(pixel, null, 2) + '\n');
    const screenshotPath = resolve(evidenceDirectory, `ios-leading-${scene.id}-screen.png`);
    await saveLog(`screenshot-leading-${scene.id}.log`, command('xcrun', [
      'simctl', 'io', simulator.udid, 'screenshot', screenshotPath,
    ]));
    const screenshotBytes = await readFile(screenshotPath);
    assert.equal(screenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a',
      `${scene.id}: iOS Simulator screenshot이 PNG가 아닙니다.`);
    observations.push({
      index,
      sceneId: scene.id,
      expectedTracksPx: entry.columnsPx,
      attempts,
      boxes: geometry.boxes,
      frames: geometry.frames.length,
      paintOrder: geometry.paintOrder,
      comparisons: geometry.comparisons,
      maximumGeometryErrorCssPx: Math.max(0, ...geometry.comparisons
        .map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
      pixel,
      screenshotSha256: sha256(screenshotBytes),
      logPath,
      pixelComparisonPath: comparisonPath,
    });
    command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  }
  return {
    fixturePath: leadingNamedLinesPixelsPath,
    fixtureSha256: sha256(await readFile(resolve(repositoryRoot, leadingNamedLinesPixelsPath))),
    referencePath: leadingNamedLinesPixels.oracle.referencePath,
    referenceSha256: sha256(leadingNamedLinesReferenceBytes),
    sceneCount: observations.length,
    comparedVisibleNodes: observations.reduce((sum, scene) => sum + scene.comparisons.length, 0),
    maximumGeometryErrorCssPx: Math.max(0, ...observations
      .map(({ maximumGeometryErrorCssPx }) => maximumGeometryErrorCssPx)),
    matchedPixelSamples: observations.reduce((sum, scene) => sum + scene.pixel.matchedSamples, 0),
    scenes: observations,
  };
}

async function captureMatrix(simulator) {
  const build = command('mise', ['exec', '--', 'env',
    'SPINON_V8_DIR=' + (process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8')),
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1',
    'bun', 'run', 'build:ios-sim'], { timeout: 900_000 });
  await saveLog('build.log', build);
  const buildOutput = '' + build.stdout + build.stderr;
  const buildFlags = buildOutput.match(/^SPINON_IOS_SIM_BUILD_FLAGS (.*)$/m)?.[1];
  assert.ok(buildFlags?.includes('c04_runtime_gpu=1'), 'iOS runtime GPU fixture feature가 없습니다.');
  assert.ok(buildFlags?.includes('c11_2_3_grid_fixture=1'), 'C11.2.3 Objective-C++ wrapper feature가 없습니다.');
  assert.ok(buildFlags?.includes('c11_grid_metal_surface_pixels=1'), 'iOS Metal pixel test feature가 없습니다.');
  const appSha256 = sha256(await readFile(iosBinaryPath));
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulator.udid, iosAppPath]));

  const observations = [];
  let pixelComparison = null;
  let screenshotSha256 = null;
  const cyclePixelComparisons = [];
  const typedAutoPixelComparisons = [];
  const typedAutoScreenCaptures = [];
  let trackListCycleScreenshotSha256 = null;
  for (const scene of runtime.cases) {
    const cyclePixelScene = cyclePixelScenes.get(scene.id);
    const typedAutoPixelScene = typedAutoPixelScenes.get(scene.id);
    const samples = scene.id === mobile.sceneId ? mobile.samples
      : cyclePixelScene?.samples ?? typedAutoPixelScene?.samples ?? null;
    const requiresMetalResult = samples !== null;
    const isReject = scene.expected.outcome === 'expected-rejection';
    const attempts = [];
    let effectiveRunID = '';
    let log = '';
    let logPath = '';
    for (let attempt = 0; attempt < (isReject ? 1 : 2); attempt += 1) {
      effectiveRunID = c1133IOSSceneRunID(appRunIDPrefix, scene.index, attempt);
      const launch = launchIOSScene(simulator, scene.id, effectiveRunID, samples);
      const launchName = 'launch-' + String(scene.index).padStart(2, '0') + '-' + scene.id
        + (attempt === 0 ? '' : '-retry-' + attempt) + '.log';
      await saveLog(launchName, launch);
      const pid = launchedPID(launch, scene.id);
      log = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
        (isReject
          ? value.includes('SPINON_C1133_GRID_REJECTION_SUMMARY run_id=' + effectiveRunID + ' scene_id=' + scene.id + ' ')
          : c1133SuccessLogsComplete(value, scene.id, effectiveRunID, requiresMetalResult)
            || c1133SupersededAttemptComplete(value, scene.id, effectiveRunID, requiresMetalResult))
          || value.includes('SPINON_C1133_METAL_PIXEL_ERROR run_id=' + effectiveRunID)
          || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'),
      scene.id + ': iOS runtime 결과', 45_000);
      logPath = 'runtime-' + String(scene.index).padStart(2, '0') + '-' + scene.id
        + (attempt === 0 ? '' : '-retry-' + attempt) + '.log';
      await writeFile(resolve(evidenceDirectory, logPath), formatLog(log) + '\n');
      const completed = isReject
        ? log.includes('SPINON_C1133_GRID_REJECTION_SUMMARY run_id=' + effectiveRunID + ' scene_id=' + scene.id + ' ')
        : c1133SuccessLogsComplete(log, scene.id, effectiveRunID, requiresMetalResult);
      const superseded = !isReject && isC1133IOSGridRuntimeSuperseded(log, effectiveRunID, scene.id);
      attempts.push({
        runID: effectiveRunID,
        status: isReject ? 'expected-rejection' : completed ? 'matched' : 'superseded-retried',
        logPath,
      });
      if (completed) break;
      assert.ok(superseded && attempt === 0,
        scene.id + ': 성공, expected rejection 또는 단회 superseded 재실행 외 결과가 나왔습니다.');
      command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
    }

    if (isReject) {
      const rejection = validateC1133IOSRejection(scene, log, effectiveRunID);
      observations.push({ index: scene.index, sceneId: scene.id, outcome: 'expected-rejection', rejection, attempts, logPath });
      command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
      continue;
    }

    const chromeCase = scene.oracle ?? observation.cases.find(({ id }) => id === scene.id);
    const geometry = compareC1133IOSSceneGeometry(scene, log, chromeCase, effectiveRunID);
    let pixel = null;
    if (requiresMetalResult) {
      const pixelFixture = cyclePixelScene
        ? { sceneId: scene.id, samples, runtime: cyclePixels.runtime }
        : typedAutoPixelScene ? { sceneId: scene.id, samples, runtime: typedAutoPixels.runtime } : mobile;
      pixel = compareC11IOSMetalPixelLogs(log, {
        fixture: pixelFixture,
        runID: effectiveRunID,
        markerPrefix: 'C1133',
      });
      assert.equal(pixel.status, 'matched', `${scene.id}: Metal 표면 pixel이 일치하지 않습니다.`);
      assert.equal(pixel.matchedSamples, samples.length);
      assert.equal(pixel.maximumChannelError, 0);
      if (scene.id === mobile.sceneId) {
        pixelComparison = pixel;
        await writeFile(resolve(evidenceDirectory, 'metal-surface-comparison.json'),
          JSON.stringify(pixel, null, 2) + '\n');
        const screenshotPath = resolve(evidenceDirectory, 'ios-screen.png');
        await saveLog('screenshot.log', command('xcrun', ['simctl', 'io', simulator.udid,
          'screenshot', screenshotPath]));
        const screenBytes = await readFile(screenshotPath);
        assert.equal(screenBytes.toString('hex', 0, 8), '89504e470d0a1a0a',
          'iOS Simulator 화면 캡처가 PNG가 아닙니다.');
        screenshotSha256 = sha256(screenBytes);
      } else if (typedAutoPixelScene) {
        typedAutoPixelComparisons.push({ sceneId: scene.id, ...pixel });
        const screenshotPath = resolve(evidenceDirectory, `${scene.id}-screen.png`);
        await saveLog(`${scene.id}-screenshot.log`, command('xcrun', ['simctl', 'io',
          simulator.udid, 'screenshot', screenshotPath]));
        const screenBytes = await readFile(screenshotPath);
        assert.equal(screenBytes.toString('hex', 0, 8), '89504e470d0a1a0a',
          `${scene.id}: iOS Simulator 화면 캡처가 PNG가 아닙니다.`);
        typedAutoScreenCaptures.push({
          sceneId: scene.id, path: `${scene.id}-screen.png`, sha256: sha256(screenBytes),
        });
      } else {
        cyclePixelComparisons.push({ sceneId: scene.id, ...pixel });
      }
      if (scene.id === 'both-axis-cycle') {
        const screenshotPath = resolve(evidenceDirectory, 'ios-track-list-cycle-screen.png');
        await saveLog('track-list-cycle-screenshot.log', command('xcrun', ['simctl', 'io',
          simulator.udid, 'screenshot', screenshotPath]));
        const screenBytes = await readFile(screenshotPath);
        assert.equal(screenBytes.toString('hex', 0, 8), '89504e470d0a1a0a',
          'track-list cycling iOS Simulator screenshot이 PNG가 아닙니다.');
        trackListCycleScreenshotSha256 = sha256(screenBytes);
      }
    }
    observations.push({
      index: scene.index,
      sceneId: scene.id,
      outcome: scene.expected.outcome,
      attempts,
      boxes: geometry.boxes,
      frames: geometry.frames.length,
      paintOrder: geometry.paintOrder,
      comparisons: geometry.comparisons,
      maximumGeometryErrorCssPx: Math.max(0, ...geometry.comparisons
        .map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
      pixel,
      logPath,
    });
    command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  }

  assert.equal(sha256(unknownAreaReferenceBytes), unknownAreaPixels.inputs.reference.sha256);
  assert.equal(unknownAreaPixels.oracle.product, unknownAreaReference.oracle.product);
  assert.equal(unknownAreaChromeSourceCase.nodes.length, unknownAreaScene.nodeIds.length);
  let unknownAreaRunID = '';
  let unknownAreaLog = '';
  let unknownAreaLogPath = '';
  const unknownAreaAttempts = [];
  for (let attempt = 0; attempt < 2; attempt += 1) {
    unknownAreaRunID = c1133IOSSceneRunID(appRunIDPrefix, unknownAreaScene.index, attempt);
    const launch = launchIOSScene(simulator, unknownAreaScene.id, unknownAreaRunID,
      unknownAreaPixels.samples);
    const launchName = 'launch-unknown-area-fixed-implicit-tracks'
      + (attempt === 0 ? '' : '-retry-' + attempt) + '.log';
    await saveLog(launchName, launch);
    const pid = launchedPID(launch, 'unknown-area-fixed-implicit-tracks');
    unknownAreaLog = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
      c1133SuccessLogsComplete(value, unknownAreaScene.id, unknownAreaRunID, true)
        || c1133SupersededAttemptComplete(value, unknownAreaScene.id, unknownAreaRunID, true)
        || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'),
    'unknown-area-fixed-implicit-tracks iOS runtime 결과', 45_000);
    unknownAreaLogPath = 'runtime-unknown-area-fixed-implicit-tracks'
      + (attempt === 0 ? '' : '-retry-' + attempt) + '.log';
    await writeFile(resolve(evidenceDirectory, unknownAreaLogPath), formatLog(unknownAreaLog) + '\n');
    assert.equal(unknownAreaLog.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'), false,
      '고정 unknown-area overflow scene이 iOS route에서 거부되었습니다.');
    const completed = c1133SuccessLogsComplete(
      unknownAreaLog, unknownAreaScene.id, unknownAreaRunID, true);
    const superseded = !completed && isC1133IOSGridRuntimeSuperseded(
      unknownAreaLog, unknownAreaRunID, unknownAreaScene.id);
    unknownAreaAttempts.push({
      runID: unknownAreaRunID,
      status: completed ? 'matched' : 'superseded-retried',
      logPath: unknownAreaLogPath,
    });
    if (completed) break;
    assert.ok(superseded && attempt === 0,
      'unknown-area overflow scene은 성공하거나 첫 실행의 superseded 응답만 재시도할 수 있습니다.');
    command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  }
  const unknownAreaGeometry = compareC1133IOSSceneGeometry(
    unknownAreaScene, unknownAreaLog, unknownAreaChromeCase, unknownAreaRunID);
  const unknownAreaPixel = compareC11IOSMetalPixelLogs(unknownAreaLog, {
    fixture: unknownAreaPixels,
    runID: unknownAreaRunID,
    markerPrefix: 'C1133',
  });
  assert.equal(unknownAreaPixel.status, 'matched');
  assert.equal(unknownAreaPixel.matchedSamples, unknownAreaPixels.samples.length);
  assert.equal(unknownAreaPixel.maximumChannelError, 0);
  await writeFile(resolve(evidenceDirectory, 'unknown-area-overflow-metal-comparison.json'),
    JSON.stringify(unknownAreaPixel, null, 2) + '\n');
  const unknownAreaScreenshotPath = resolve(evidenceDirectory, 'ios-unknown-area-overflow-screen.png');
  await saveLog('unknown-area-overflow-screenshot.log', command('xcrun', ['simctl', 'io',
    simulator.udid, 'screenshot', unknownAreaScreenshotPath]));
  const unknownAreaScreenshotBytes = await readFile(unknownAreaScreenshotPath);
  assert.equal(unknownAreaScreenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a',
    'unknown-area overflow Simulator screenshot이 PNG가 아닙니다.');
  const unknownAreaOverflow = {
    sceneId: unknownAreaScene.id,
    outcome: 'success',
    attempts: unknownAreaAttempts,
    boxes: unknownAreaGeometry.boxes,
    frames: unknownAreaGeometry.frames.length,
    comparisons: unknownAreaGeometry.comparisons,
    maximumGeometryErrorCssPx: Math.max(0, ...unknownAreaGeometry.comparisons
      .map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
    pixel: unknownAreaPixel,
    screenshotSha256: sha256(unknownAreaScreenshotBytes),
    logPath: unknownAreaLogPath,
  };
  await writeFile(resolve(evidenceDirectory, unknownAreaLogPath), formatLog(unknownAreaLog) + '\n');
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });

  const leadingNamedImplicitLines = await captureLeadingNamedImplicitLines(simulator);

  assert.ok(pixelComparison, '기본 scene Metal 표면 비교가 실행되지 않았습니다.');
  const negative = await runNegativeControls(simulator);
  const routeNegative = await runUnknownSceneNegative(simulator);
  return {
    metadata: { name: simulator.name, udid: simulator.udid, runtime: simulator.runtime },
    appSha256,
    observations,
    unknownAreaOverflow,
    leadingNamedImplicitLines,
    pixelComparison,
    screenshotSha256,
    cyclePixelComparisons,
    typedAutoPixelComparisons,
    typedAutoScreenCaptures,
    trackListCycleScreenshotSha256,
    negative,
    routeNegative,
  };
}

async function runNegativeControls(simulator) {
  const negativeRunID = runID + '-bad-samples';
  const invalidSamples = mobile.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  const launch = launchIOSScene(simulator, mobile.sceneId, negativeRunID, invalidSamples);
  await saveLog('negative-pixel-launch.log', launch);
  const pid = launchedPID(launch, 'pixel sample negative control');
  const log = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
    value.includes('SPINON_C1133_METAL_PIXEL_ERROR run_id=' + negativeRunID)
      || value.includes('SPINON_C1133_METAL_PIXEL_BEGIN run_id=' + negativeRunID),
  'iOS Metal pixel allowlist negative control', 30_000);
  await writeFile(resolve(evidenceDirectory, 'negative-pixel-control.log'), formatLog(log) + '\n');
  assert.ok(log.includes('SPINON_C1133_METAL_PIXEL_ERROR run_id=' + negativeRunID),
    'iOS가 변경된 sample 좌표를 거부하지 않았습니다.');
  assert.equal(log.includes('SPINON_C1133_METAL_PIXEL_BEGIN run_id=' + negativeRunID), false,
    'iOS가 좌표 거부 전에 Metal readback을 시작했습니다.');
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  return { status: 'rejected-before-readback', runID: negativeRunID, readbackStarted: false };
}

async function runUnknownSceneNegative(simulator) {
  const negativeRunID = runID + '-unknown-scene';
  const launch = launchIOSScene(simulator, 'unknown-c1133-scene', negativeRunID, null);
  await saveLog('negative-scene-launch.log', launch);
  const pid = launchedPID(launch, 'unknown scene negative control');
  const log = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
    value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true')
      || value.includes('SPINON_C1133_GRID_EVAL_RUN run_id=' + negativeRunID)
      || value.includes('SPINON_C0410_DRAW'),
  'iOS unknown scene route negative control', 30_000);
  await writeFile(resolve(evidenceDirectory, 'negative-scene-control.log'), formatLog(log) + '\n');
  const result = validateC1133IOSUnknownScene(log, negativeRunID);
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  return result;
}

await mkdir(evidenceDirectory, { recursive: true });
try {
  const simulator = bootedSimulator();
  const fixtureBytes = await readFile(resolve(repositoryRoot, runtimePath));
  const mobileBytes = await readFile(resolve(repositoryRoot, mobilePath));
  const cyclePixelsBytes = await readFile(resolve(repositoryRoot, cyclePixelsPath));
  const typedAutoPixelsBytes = await readFile(resolve(repositoryRoot, typedAutoPixelsPath));
  const referenceBytes = await readFile(resolve(repositoryRoot, mobile.oracle.referencePath));
  const unknownAreaPixelsBytes = await readFile(resolve(repositoryRoot, unknownAreaPixelsPath));
  const leadingNamedLinesPixelsBytes = await readFile(resolve(repositoryRoot, leadingNamedLinesPixelsPath));
  assert.equal(sha256(leadingNamedLinesReferenceBytes), leadingNamedLinesPixels.oracle.referenceSha256);
  const screenshotBytes = await readFile(resolve(repositoryRoot, mobile.oracle.screenshotPath));
  const oracleComparison = validateC11IOSChromeOracle({
    fixture: mobile,
    referenceBytes,
    screenshotBytes,
  });
  const platform = await captureMatrix(simulator);
  const outcomeCounts = { successCount: 0, cascadeFallbackCount: 0, expectedRejectCount: 0 };
  for (const item of platform.observations) {
    const key = item.outcome === 'expected-rejection'
      ? 'expectedRejectCount'
      : item.outcome === 'cascade-fallback' ? 'cascadeFallbackCount' : 'successCount';
    outcomeCounts[key] += 1;
  }
  assert.deepEqual(outcomeCounts, {
    successCount: runtime.summary.successCount,
    cascadeFallbackCount: runtime.summary.cascadeFallbackCount,
    expectedRejectCount: runtime.summary.expectedRejectCount,
  });
  const comparedNodes = platform.observations.reduce((sum, item) => sum + (item.comparisons?.length ?? 0), 0);
  const maximumGeometryErrorCssPx = Math.max(0, ...platform.observations
    .map(({ maximumGeometryErrorCssPx: error = 0 }) => error));
  const startupSupersededRetryCount = platform.observations.reduce((count, item) => count
    + item.attempts.filter(({ status }) => status === 'superseded-retried').length, 0);
  assert.equal(platform.observations.length, runtime.summary.sceneCount);
  assert.ok(maximumGeometryErrorCssPx <= 0.5);
  assert.equal(platform.pixelComparison.matchedSamples, mobile.samples.length);
  assert.equal(platform.pixelComparison.maximumChannelError, 0);
  assert.equal(platform.cyclePixelComparisons.length, 6);
  assert.equal(platform.cyclePixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0), 28);
  assert.ok(platform.cyclePixelComparisons.every((item) =>
    item.status === 'matched' && item.maximumChannelError === 0));
  assert.equal(platform.leadingNamedImplicitLines.sceneCount, 7);
  assert.equal(platform.leadingNamedImplicitLines.comparedVisibleNodes, 14);
  assert.ok(platform.leadingNamedImplicitLines.maximumGeometryErrorCssPx <= 0.5);
  assert.equal(platform.leadingNamedImplicitLines.matchedPixelSamples, 7);

  const result = {
    schema: 'spinon-c11-3-3-grid-template-areas-ios-runtime-matrix-result/v1',
    status: 'matched',
    runID,
    target: 'ios-simulator',
    source: {
      commit: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
      runtimeMatrixSha256: sha256(fixtureBytes),
      mobileFixtureSha256: sha256(mobileBytes),
      trackListCyclePixelsSha256: sha256(cyclePixelsBytes),
      typedAutoPixelsSha256: sha256(typedAutoPixelsBytes),
      trackListCycleObservationSha256: sha256(cycleObservationBytes),
      trackListCycleChromeScreenshotSha256: sha256(cycleScreenshotBytes),
      chromeReferenceSha256: sha256(referenceBytes),
      unknownAreaPixelFixtureSha256: sha256(unknownAreaPixelsBytes),
      unknownAreaReferenceSha256: sha256(unknownAreaReferenceBytes),
      leadingNamedLinesPixelFixtureSha256: sha256(leadingNamedLinesPixelsBytes),
      leadingNamedLinesReferenceSha256: sha256(leadingNamedLinesReferenceBytes),
      captureScriptSha256: sha256(await readFile(fileURLToPath(import.meta.url))),
      matrixValidationSha256: sha256(await readFile(resolve(repositoryRoot,
        'tools/css-reference/c11-3-3-grid-template-areas-ios-matrix.mjs'))),
      iosRuntimeSourceSha256: sha256(await readFile(resolve(repositoryRoot,
        'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'))),
      appSha256: platform.appSha256,
    },
    oracle: {
      product: reference.oracle.product,
      referenceId: reference.referenceId,
      deviceScaleFactor: 1,
      viewportCssPx: mobile.oracle.viewportCssPx,
      sampleCount: mobile.samples.length,
      oracleComparison,
    },
    platform: {
      metadata: platform.metadata,
      outcomeCounts,
      sceneCount: platform.observations.length,
      nodeCount: runtime.summary.nodeCount,
      comparedNodeFrames: comparedNodes,
      maximumGeometryErrorCssPx,
      startupSupersededRetryCount,
      pixelComparison: platform.pixelComparison,
      screenshotSha256: platform.screenshotSha256,
      cyclePixelComparisons: platform.cyclePixelComparisons,
      typedAutoPixelComparisons: platform.typedAutoPixelComparisons,
      typedAutoScreenCaptures: platform.typedAutoScreenCaptures,
      trackListCycleScreenshotSha256: platform.trackListCycleScreenshotSha256,
    typedAutoPixels: {
      path: typedAutoPixelsPath,
      sha256: sha256(typedAutoPixelsBytes),
      observations: platform.typedAutoPixelComparisons,
      totalSamples: platform.typedAutoPixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0),
      screenshots: platform.typedAutoScreenCaptures,
    },
      unknownAreaOverflow: platform.unknownAreaOverflow,
      leadingNamedImplicitLines: platform.leadingNamedImplicitLines,
      negative: platform.negative,
      routeNegative: platform.routeNegative,
    },
    observations: platform.observations,
    interpretation: 'iPhone 17 Pro/iOS 26.2 Simulator에서 기존 26개 Chrome 154 scene과 13개 Chrome 156 supplemental scene을 포함한 C11.3.3 39개 scene·126개 node 행렬, track-list cycling 6개 scene·28개 Metal 표본, leading named implicit line 7개 scene·14개 node를 검증했다. Metal drawable readback은 OS compositor 제출 전이며, 이는 실기기·hardware GPU·Spinon WPT 또는 전체 CSS Grid 지원 근거가 아니다.',
  };
  await writeFile(resolve(evidenceDirectory, 'result.json'), JSON.stringify(result, null, 2) + '\n');
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.3.3 Grid template areas iOS Simulator 전체 행렬',
    '',
    '- 실행 ID: ' + runID,
    '- 대상: ' + platform.metadata.name + ' / ' + platform.metadata.runtime + ' Simulator',
    '- 고정 Chrome DPR 1 기준: ' + runtime.summary.sceneCount + ' scene / ' + runtime.summary.nodeCount + ' node',
    '- 결과: 성공 ' + outcomeCounts.successCount + ', cascade fallback ' + outcomeCounts.cascadeFallbackCount
      + ', 기대 거부 ' + outcomeCounts.expectedRejectCount,
    '- node frame 비교: ' + comparedNodes + '개, 최대 geometry 오차 ' + maximumGeometryErrorCssPx + ' CSS px',
    '- 초기 runtime 평가 `status=-12` 재실행: ' + startupSupersededRetryCount + '회 (최초 로그와 재실행 로그 모두 보존)',
    '- basic-rectangles Metal drawable 표본: ' + platform.pixelComparison.matchedSamples + '/'
      + platform.pixelComparison.expectedSamples + ' RGBA exact; 최대 채널 오차 0',
    '- track-list cycling Metal drawable 표본: ' + platform.cyclePixelComparisons.length
      + ' scene / ' + platform.cyclePixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0)
      + ' RGBA exact; 최대 채널 오차 0',
    '- typed auto-track Metal drawable 표본: ' + platform.typedAutoPixelComparisons.length
      + ' scene / ' + platform.typedAutoPixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0)
      + ' RGBA exact; 최대 채널 오차 0',
    '- unknown-area overflow: ' + platform.unknownAreaOverflow.comparisons.length
      + '개 Chrome geometry node, ' + platform.unknownAreaOverflow.pixel.matchedSamples + '/'
      + platform.unknownAreaOverflow.pixel.expectedSamples
      + '개 Metal 표본 exact; grid root 밖 item 포함',
    '- leading named implicit lines: ' + platform.leadingNamedImplicitLines.sceneCount
      + ' scene / ' + platform.leadingNamedImplicitLines.comparedVisibleNodes
      + ' visible node, 최대 geometry 오차 '
      + platform.leadingNamedImplicitLines.maximumGeometryErrorCssPx + ' CSS px; '
      + platform.leadingNamedImplicitLines.matchedPixelSamples + '/7 Metal 표본 exact',
    '- negative controls: sample 좌표 ' + platform.negative.status + ', unknown scene '
      + platform.routeNegative.status,
    '- Metal drawable은 OS compositor 제출 전이다. 실기기·hardware GPU·WPT 적합성·전체 CSS Grid 지원은 이 실행에 포함하지 않는다.',
    '',
    '## 산출물',
    '',
    '- runtime-*.log: scene별 평가, frame, 거부 결과',
    '- ios-screen.png: basic-rectangles UI 화면 캡처',
    '- ios-track-list-cycle-screen.png: 양 축 track-list cycling 화면 캡처',
    '- typed-area-*-screen.png: typed auto-track scene별 iOS Simulator 화면 캡처',
    '- ios-unknown-area-overflow-screen.png: fixed implicit-track overflow 장면 화면 캡처',
    '- ios-leading-*-screen.png: leading named implicit line 장면 화면 캡처',
    '- metal-surface-comparison.json: Chrome 고정 sample과 Metal drawable 비교',
    '- result.json: fixture hash, geometry, pixel 및 negative-control 결과',
  ].join('\n') + '\n');
  process.stdout.write(JSON.stringify({ ...result, evidenceDirectory }, null, 2) + '\n');
} catch (error) {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), (error.stack ?? error) + '\n');
  throw error;
}
