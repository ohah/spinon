import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  selectAndroidAVDSerial,
  selectAndroidPhysicalSerial,
} from './c11-3-3-grid-template-areas-runtime-device.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runtimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
const mobilePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-mobile-surface-pixel-matrix-v1.json';
const runtime = JSON.parse(await readFile(resolve(repositoryRoot, runtimePath), 'utf8'));
const mobile = JSON.parse(await readFile(resolve(repositoryRoot, mobilePath), 'utf8'));
const cyclePixelsPath = 'tests/fixtures/css/c11/grid-template-areas-track-list-cycling-mobile-pixels-v1.json';
const cyclePixels = JSON.parse(await readFile(resolve(repositoryRoot, cyclePixelsPath), 'utf8'));
const cycleObservationBytes = await readFile(resolve(repositoryRoot, cyclePixels.inputs.chromeObservation.path));
const cycleScreenshotBytes = await readFile(resolve(repositoryRoot, cyclePixels.oracle.screenshotPath));
assert.equal(cyclePixels.inputs.runtimeMatrix.sha256,
  sha256(await readFile(resolve(repositoryRoot, runtimePath))));
assert.equal(cyclePixels.inputs.chromeObservation.sha256, sha256(cycleObservationBytes));
assert.equal(cyclePixels.oracle.observationSha256, sha256(cycleObservationBytes));
assert.equal(cyclePixels.oracle.screenshotSha256, sha256(cycleScreenshotBytes));
assert.equal(cyclePixels.scenes.length, 6);
const cyclePixelScenes = new Map(cyclePixels.scenes.map((entry) => [entry.sceneId, entry]));
const reference = JSON.parse(await readFile(resolve(repositoryRoot, mobile.oracle.referencePath), 'utf8'));
const observation = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
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
const target = process.argv[2];
assert.ok(['android-avd', 'android-physical'].includes(target),
  '사용법: node capture-c11-3-3-grid-template-areas-runtime-matrix.mjs android-avd|android-physical [ADB serial]');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const serialHint = process.argv[3] ?? (target === 'android-avd'
  ? process.env.SPINON_ANDROID_EMULATOR_SERIAL ?? 'emulator-5554'
  : process.env.ANDROID_SERIAL);
const runID = `c1133-matrix-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-3-3-grid-template-areas-runtime-${date}`, `${target}-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
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
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

async function saveLog(path, result) {
  const log = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  await writeFile(resolve(evidenceDirectory, path), log.replace(/[ \t]+$/gm, ''));
}

function formatLog(log) {
  return log.replace(/[ \t]+$/gm, '');
}

function samplePayload(samples = mobile.samples) {
  return Buffer.from(JSON.stringify(samples)).toString('base64url');
}

function currentProcessLog(adb, pid) {
  const output = adb(['shell', 'logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function waitForProcess(adb, timeoutMs = 30_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]);
    if (Number.isInteger(pid) && pid > 0) return pid;
    sleep(200);
  }
  throw new Error('Android 앱 process가 시작되지 않았습니다.');
}

function waitForLog(readLog, predicate, message, timeoutMs = 90_000) {
  const deadline = Date.now() + timeoutMs;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    if (predicate(latest)) return latest;
    sleep(250);
  }
  throw new Error(`${message} 대기 시간이 초과되었습니다.\n${latest}`);
}

function parseFrames(log, scene) {
  const frames = log.split(/\r?\n/).filter((line) =>
    line.includes('SPINON_C1133_GRID_NODE_FRAME ')).map((line) => {
    const match = line.match(/\b(\d+):node=(\d+),x=([^,\s]+),y=([^,\s]+),width=([^,\s]+),height=([^,\s]+)/);
    assert.ok(match, `${scene.id}: frame 형식 오류: ${line}`);
    return {
      index: Number(match[1]), nodeId: Number(match[2]),
      x: Number(match[3]), y: Number(match[4]), width: Number(match[5]), height: Number(match[6]),
    };
  });
  assert.deepEqual(frames.map(({ index }) => index), frames.map((_, index) => index),
    `${scene.id}: frame index가 연속적이지 않습니다.`);
  assert.equal(new Set(frames.map(({ nodeId }) => nodeId)).size, frames.length,
    `${scene.id}: frame node ID가 중복되었습니다.`);
  return frames;
}

function compareGeometry(scene, log, chromeCaseOverride = null) {
  const chromeCase = chromeCaseOverride ?? scene.oracle ?? observation.cases.find(({ id }) => id === scene.id);
  assert.ok(chromeCase, `${scene.id}: Chrome DPR 1 장면이 없습니다.`);
  const frames = parseFrames(log, scene);
  const byNodeId = new Map(frames.map((frame) => [frame.nodeId, frame]));
  const comparisons = [];
  for (let index = 0; index < scene.nodeIds.length; index++) {
    const nodeId = index + 1;
    const nodeName = scene.nodeIds[index];
    const expectedNode = chromeCase.nodes.find(({ id }) => id === nodeName);
    assert.ok(expectedNode, `${scene.id}/${nodeName}: Chrome node geometry가 없습니다.`);
    const expected = expectedNode.rect;
    const frame = byNodeId.get(nodeId);
    const hasArea = expected.width > 0 && expected.height > 0;
    if (!frame) {
      assert.equal(hasArea, false,
        `${scene.id}/${nodeName}: Chrome에 보이는 node인데 runtime frame이 없습니다.`);
      continue;
    }
    const delta = Object.fromEntries(['x', 'y', 'width', 'height']
      .map((key) => [key, Math.abs(frame[key] - expected[key])]));
    assert.ok(Object.values(delta).every((value) => value <= 0.5),
      `${scene.id}/${nodeName}: Chrome geometry 불일치: expected=${JSON.stringify(expected)} actual=${JSON.stringify(frame)}`);
    comparisons.push({ nodeId, nodeName, expected, actual: {
      x: frame.x, y: frame.y, width: frame.width, height: frame.height,
    }, maximumErrorCssPx: Math.max(...Object.values(delta)) });
  }
  for (const frame of frames) {
    assert.ok(frame.nodeId >= 1 && frame.nodeId <= scene.nodeIds.length,
      `${scene.id}: 등록되지 않은 runtime node ID ${frame.nodeId}`);
  }
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C1133_GRID_SUMMARY run_id=`) && line.includes(`scene_id=${scene.id} `));
  assert.ok(summary?.includes('status=0 layout=ready'), `${scene.id}: 성공 summary가 없습니다.`);
  const boxes = Number(summary.match(/\bbox(?:es)?=(\d+)/)?.[1]);
  const renderSummary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C1133_GRID_EVAL `) && line.includes(`scene_id=${scene.id} `)
    && line.includes('paint_node_order='));
  const paintOrder = renderSummary?.match(/\bpaint_node_order=\[([^\]]*)\]/)?.[1];
  assert.notEqual(paintOrder, undefined, `${scene.id}: paint node order가 없습니다.`);
  const paintedNodeIds = [...paintOrder.matchAll(/(?:^|,)\d+:(\d+)/g)]
    .map((match) => Number(match[1]));
  assert.equal(new Set(paintedNodeIds).size, paintedNodeIds.length,
    `${scene.id}: paint node ID가 중복되었습니다.`);
  assert.equal(boxes, paintedNodeIds.length,
    `${scene.id}: summary boxes와 paint node 수가 다릅니다.`);
  assert.ok(paintedNodeIds.every((nodeId) => byNodeId.has(nodeId)),
    `${scene.id}: paint node에 layout frame이 없는 노드가 있습니다.`);
  return { frames, comparisons, boxes };
}

function matrixCaseStatus(scene) {
  return scene.expected.outcome === 'expected-rejection'
    ? 'expected-rejection' : scene.expected.outcome;
}

function validateReject(scene, log, runId) {
  const expected = scene.expected;
  const expectedNodeId = scene.nodeIds.indexOf(expected.nodeId) + 1;
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C1133_GRID_REJECTION_SUMMARY run_id=${runId} scene_id=${scene.id} `));
  assert.ok(summary, `${scene.id}: rejection summary가 없습니다.`);
  assert.ok(summary.includes('expected=true status=-11')
      && summary.includes(`code=${expected.code}`)
      && summary.includes(`property=${expected.property}`)
      && summary.includes(`node_id=${expectedNodeId}`)
      && summary.includes('frames=absent'),
  `${scene.id}: 기대 rejection 계약 불일치: ${summary}`);
  assert.equal(log.includes('SPINON_C1133_GRID_NODE_FRAME'), false,
    `${scene.id}: 거부 결과에 layout frame이 있습니다.`);
  assert.equal(log.includes('SPINON_C0410_DRAW'), false,
    `${scene.id}: 거부 전에 GPU draw/present가 발생했습니다.`);
  assert.equal(log.includes('SPINON_C1133_GRID_PIXEL_MATRIX_BEGIN'), false,
    `${scene.id}: 거부 전에 PixelCopy가 시작됐습니다.`);
  return { status: 'expected-rejection', code: expected.code, property: expected.property,
    nodeId: expected.nodeId, runtimeNodeId: expectedNodeId };
}

async function captureUnknownAreaOverflow(adb) {
  assert.equal(sha256(unknownAreaReferenceBytes), unknownAreaPixels.inputs.reference.sha256);
  assert.equal(unknownAreaPixels.oracle.product, unknownAreaReference.oracle.product);
  assert.equal(unknownAreaChromeSourceCase.nodes.length, unknownAreaScene.nodeIds.length);
  const sceneRunID = `${runID}-unknown-area-fixed-implicit-tracks`;
  const launch = launchScene(adb, unknownAreaScene.id, sceneRunID, { samples: unknownAreaPixels.samples });
  await saveLog('launch-unknown-area-fixed-implicit-tracks.log', launch);
  const pid = waitForProcess(adb);
  const evalMarker = `SPINON_C1133_GRID_EVAL_RUN run_id=${sceneRunID} scene_id=${unknownAreaScene.id} `;
  let log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
    value.includes(evalMarker) || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'),
  'unknown-area-fixed-implicit-tracks runtime 결과');
  assert.ok(!log.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'),
    '고정 unknown-area overflow scene이 앱 route에서 거부되었습니다.');
  assert.ok(log.includes(`${evalMarker}status=0 layout=ready`),
    '고정 unknown-area overflow scene의 layout 계산이 성공하지 않았습니다.');
  log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
    value.includes('SPINON_C0410_DRAW status=0 presented boxes=')
      && (value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${sceneRunID}`)
        || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${sceneRunID}`)
        || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${sceneRunID}`)
        || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)),
  'unknown-area-fixed-implicit-tracks draw / PixelCopy 결과', 60_000);
  await writeFile(resolve(evidenceDirectory, 'runtime-unknown-area-fixed-implicit-tracks.log'), `${formatLog(log)}\n`);
  const geometry = compareGeometry(unknownAreaScene, log, unknownAreaChromeCase);
  const capture = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${sceneRunID}`));
  assert.ok(capture, 'unknown-area overflow PixelCopy capture marker가 없습니다.');
  const devicePng = capture.match(/\bpng=(\S+)/)?.[1];
  assert.ok(devicePng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1133-grid-surface-pixel-matrix/'),
    'unknown-area PixelCopy 경로가 C11.3.3 전용 cache 경로가 아닙니다.');
  const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
    `cache/c1133-grid-surface-pixel-matrix/${devicePng.split('/').at(-1)}`], { encoding: null }).stdout;
  const pixel = compareC11AndroidPixelCopyScene(log, {
    scene: { id: unknownAreaPixels.sceneId, samples: unknownAreaPixels.samples },
    runID: sceneRunID,
    pngBytes,
    viewportCssPx: unknownAreaPixels.runtime.viewportCssPx,
    markerPrefix: 'C1133',
    cacheDirectory: 'c1133-grid-surface-pixel-matrix',
  });
  await writeFile(resolve(evidenceDirectory, 'unknown-area-overflow-pixelcopy.png'), pngBytes);
  return {
    sceneId: unknownAreaScene.id,
    outcome: 'success',
    boxes: geometry.boxes,
    frames: geometry.frames.length,
    comparisons: geometry.comparisons,
    maximumGeometryErrorCssPx: Math.max(0, ...geometry.comparisons
      .map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
    pixel,
    logPath: 'runtime-unknown-area-fixed-implicit-tracks.log',
  };
}

async function captureLeadingNamedImplicitLines(adb) {
  assert.equal(leadingNamedLinesPixels.schema,
    'spinon-css-c11-3-3-leading-named-implicit-lines-mobile-pixels/v1');
  assert.equal(sha256(leadingNamedLinesReferenceBytes),
    leadingNamedLinesPixels.oracle.referenceSha256);
  assert.equal(leadingNamedLinesReference.referenceId, leadingNamedLinesPixels.oracle.referenceId);
  assert.equal(leadingNamedLinesPixels.scenes.length, 7);
  const observations = [];
  for (const [index, entry] of leadingNamedLinesPixels.scenes.entries()) {
    const sceneRunID = `${runID}-leading-${String(index).padStart(2, '0')}`;
    assert.match(sceneRunID, /^[A-Za-z0-9_-]{1,80}$/,
      `${entry.id}: Android route run ID가 고정 입력 길이를 벗어났습니다.`);
    const scene = {
      id: entry.id,
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
    const launch = launchScene(adb, entry.id, sceneRunID, { samples: entry.samples });
    await saveLog(`launch-leading-${String(index).padStart(2, '0')}-${entry.id}.log`, launch);
    const pid = waitForProcess(adb);
    const evalMarker = `SPINON_C1133_GRID_EVAL_RUN run_id=${sceneRunID} scene_id=${entry.id} `;
    let log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
      value.includes(evalMarker) || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'),
    `${entry.id}: leading named implicit line runtime 결과`);
    assert.ok(!log.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'),
      `${entry.id}: Android route에서 scene이 거부되었습니다.`);
    assert.ok(log.includes(`${evalMarker}status=0 layout=ready`),
      `${entry.id}: layout 평가가 성공하지 않았습니다.`);
    log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
      value.includes('SPINON_C0410_DRAW status=0 presented boxes=')
        && (value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${sceneRunID}`)
          || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${sceneRunID}`)
          || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${sceneRunID}`)
          || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)),
    `${entry.id}: GPU draw / PixelCopy 결과`, 60_000);
    const logPath = `runtime-leading-${String(index).padStart(2, '0')}-${entry.id}.log`;
    await writeFile(resolve(evidenceDirectory, logPath), `${formatLog(log)}\n`);
    const geometry = compareGeometry(scene, log, chromeCase);
    const capture = log.split(/\r?\n/).find((line) => line.includes(
      `SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${sceneRunID}`));
    assert.ok(capture, `${entry.id}: PixelCopy capture marker가 없습니다.`);
    const devicePng = capture.match(/\bpng=(\S+)/)?.[1];
    assert.ok(devicePng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1133-grid-surface-pixel-matrix/'),
      `${entry.id}: PixelCopy 경로가 C11.3.3 전용 cache가 아닙니다.`);
    const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
      `cache/c1133-grid-surface-pixel-matrix/${devicePng.split('/').at(-1)}`],
    { encoding: null }).stdout;
    const pixel = compareC11AndroidPixelCopyScene(log, {
      scene: { id: entry.id, samples: entry.samples },
      runID: sceneRunID,
      pngBytes,
      viewportCssPx: leadingNamedLinesPixels.runtime.viewportCssPx,
      markerPrefix: 'C1133',
      cacheDirectory: 'c1133-grid-surface-pixel-matrix',
    });
    await writeFile(resolve(evidenceDirectory, `pixelcopy-leading-${entry.id}.png`), pngBytes);
    observations.push({
      index,
      sceneId: entry.id,
      expectedTracksPx: entry.columnsPx,
      boxes: geometry.boxes,
      frames: geometry.frames.length,
      comparisons: geometry.comparisons,
      maximumGeometryErrorCssPx: Math.max(0, ...geometry.comparisons
        .map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
      pixel,
      logPath,
    });
  }
  return {
    fixturePath: leadingNamedLinesPixelsPath,
    fixtureSha256: sha256(await readFile(resolve(repositoryRoot, leadingNamedLinesPixelsPath))),
    referencePath: leadingNamedLinesPixels.oracle.referencePath,
    referenceSha256: sha256(leadingNamedLinesReferenceBytes),
    scenes: observations,
    sceneCount: observations.length,
    comparedVisibleNodes: observations.reduce((sum, scene) => sum + scene.comparisons.length, 0),
    maximumGeometryErrorCssPx: Math.max(0, ...observations
      .map(({ maximumGeometryErrorCssPx }) => maximumGeometryErrorCssPx)),
    matchedPixelSamples: observations.reduce((sum, scene) => sum + scene.pixel.matchedSamples, 0),
  };
}

function androidDevice() {
  const devices = command('adb', ['devices', '-l']).stdout;
  const serial = target === 'android-avd'
    ? selectAndroidAVDSerial(devices, serialHint)
    : selectAndroidPhysicalSerial(devices, serialHint);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const sdk = Number(getprop('ro.build.version.sdk'));
  const qemu = getprop('ro.kernel.qemu') === '1';
  assert.equal(qemu, target === 'android-avd', `선택한 Android 대상 종류가 요청과 다릅니다: ${serial}`);
  if (target === 'android-avd') {
    assert.equal(sdk, 37, `고정 Android API 37 AVD가 아닙니다: API ${sdk}`);
  } else {
    assert.equal(getprop('ro.product.model'), 'SM-S731N', '고정 Android 실기기 모델이 아닙니다.');
    assert.equal(sdk, 36, `고정 Android 16/API 36 실기기가 아닙니다: API ${sdk}`);
  }
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음';
  if (target === 'android-avd') {
    assert.match(renderer, /SwiftShader/i, `Android AVD가 SwiftShader를 사용하지 않습니다: ${renderer}`);
  }
  return {
    adb,
    metadata: {
      serial: '[REDACTED]', model: getprop('ro.product.model'), product: getprop('ro.product.name'),
      device: getprop('ro.product.device'), manufacturer: getprop('ro.product.manufacturer'),
      release: getprop('ro.build.version.release'), sdk, qemu,
      fingerprint: getprop('ro.build.fingerprint'), renderer,
    },
  };
}

function launchScene(adb, sceneId, sceneRunID, { samples } = {}) {
  const args = ['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', component,
    '--ez', 'spinon_c1133_grid_template_areas', 'true',
    '--es', 'spinon_c1133_grid_template_areas_run_id', sceneRunID,
    '--es', 'spinon_c11_grid_case', sceneId];
  if (samples) {
    args.push('--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
      '--es', 'spinon_c11_grid_pixel_matrix_samples', samplePayload(samples));
  }
  return adb(args, { timeout: 90_000 });
}

async function captureMatrix(adb) {
  const build = command('mise', ['exec', '--', 'env',
    `SPINON_V8_DIR=${process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8')}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:android'], { timeout: 900_000 });
  await saveLog('build.log', build);
  const appSha256 = sha256(await readFile(apkPath));
  await saveLog('install.log', adb(['install', '-r', apkPath]));

  const observations = [];
  let pixelComparison;
  const cyclePixelComparisons = [];
  let trackListCycleScreenSha256 = null;
  for (const scene of runtime.cases) {
    const sceneRunID = `${runID}-${String(scene.index).padStart(2, '0')}-${scene.id}`;
    const expectedReject = scene.expected.outcome === 'expected-rejection';
    const cyclePixelScene = cyclePixelScenes.get(scene.id);
    const pixelSamples = scene.id === mobile.sceneId ? mobile.samples : cyclePixelScene?.samples ?? null;
    const requiresPixelResult = pixelSamples !== null;
    const launch = launchScene(adb, scene.id, sceneRunID, { samples: pixelSamples });
    await saveLog(`launch-${scene.index.toString().padStart(2, '0')}-${scene.id}.log`, launch);
    const pid = waitForProcess(adb);
    const evalMarker = `SPINON_C1133_GRID_EVAL_RUN run_id=${sceneRunID} scene_id=${scene.id} `;
    let log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
      (expectedReject
        ? value.includes(`SPINON_C1133_GRID_REJECTION_SUMMARY run_id=${sceneRunID} scene_id=${scene.id} `)
        : value.includes(evalMarker))
        || value.includes(`SPINON_C1133_GRID_ARGUMENT_ERROR`),
    `${scene.id}: runtime 결과`);
    const logPath = `runtime-${scene.index.toString().padStart(2, '0')}-${scene.id}.log`;
    await writeFile(resolve(evidenceDirectory, logPath), `${formatLog(log)}\n`);
    if (!expectedReject) {
      const evalLine = log.split(/\r?\n/).find((line) => line.includes(evalMarker));
      assert.ok(evalLine?.includes('status=0 layout=ready'),
        `${scene.id}: runtime 평가가 성공하지 않았습니다: ${evalLine ?? log}`);
      log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
        value.includes(`SPINON_C0410_DRAW status=0 presented boxes=`)
          && (!requiresPixelResult
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)),
      `${scene.id}: GPU presentation / PixelCopy 결과`, 60_000);
      await writeFile(resolve(evidenceDirectory, logPath), `${formatLog(log)}\n`);
    }
    if (expectedReject) {
      observations.push({ index: scene.index, sceneId: scene.id,
        outcome: 'expected-rejection', rejection: validateReject(scene, log, sceneRunID), logPath });
      continue;
    }
    const geometry = compareGeometry(scene, log);
    let pixel = null;
    if (requiresPixelResult) {
      const capture = log.split(/\r?\n/).find((line) => line.includes(
        `SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${sceneRunID}`));
      assert.ok(capture, `${scene.id}: PixelCopy capture marker가 없습니다.`);
      const devicePng = capture.match(/\bpng=(\S+)/)?.[1];
      assert.ok(devicePng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1133-grid-surface-pixel-matrix/'),
        'PixelCopy 경로가 C11.3.3 전용 cache 경로가 아닙니다.');
      const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
        `cache/c1133-grid-surface-pixel-matrix/${devicePng.split('/').at(-1)}`], { encoding: null }).stdout;
      pixel = compareC11AndroidPixelCopyScene(log, {
        scene: { id: scene.id, samples: pixelSamples }, runID: sceneRunID, pngBytes,
        viewportCssPx: cyclePixelScene ? cyclePixels.runtime.viewportCssPx : mobile.oracle.viewportCssPx,
        markerPrefix: 'C1133',
        cacheDirectory: 'c1133-grid-surface-pixel-matrix',
      });
      const pixelPath = scene.id === mobile.sceneId ? 'pixelcopy.png' : `pixelcopy-${scene.id}.png`;
      await writeFile(resolve(evidenceDirectory, pixelPath), pngBytes);
      if (scene.id === mobile.sceneId) pixelComparison = pixel;
      else cyclePixelComparisons.push({ sceneId: scene.id, pixelPath, ...pixel });
      if (scene.id === 'both-axis-cycle' && target === 'android-avd') {
        const screen = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
        await writeFile(resolve(evidenceDirectory, 'android-track-list-cycle-screen.png'), screen);
        trackListCycleScreenSha256 = sha256(screen);
      }
    }
    observations.push({
      index: scene.index, sceneId: scene.id, outcome: matrixCaseStatus(scene),
      boxes: geometry.boxes, frames: geometry.frames.length,
      comparisons: geometry.comparisons, maximumGeometryErrorCssPx: Math.max(
        0, ...geometry.comparisons.map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
      pixel, logPath,
    });
  }
  const unknownAreaOverflow = await captureUnknownAreaOverflow(adb);
  const leadingNamedImplicitLines = await captureLeadingNamedImplicitLines(adb);
  let screenSha256 = null;
  if (target === 'android-avd') {
    const screen = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    await writeFile(resolve(evidenceDirectory, 'android-screen.png'), screen);
    screenSha256 = sha256(screen);
  }
  return { appSha256, observations, pixelComparison, cyclePixelComparisons,
    trackListCycleScreenSha256, unknownAreaOverflow, leadingNamedImplicitLines, screenSha256 };
}

async function runNegativeControls(adb) {
  const basicRunID = `${runID}-bad-samples`;
  const badSamples = mobile.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  const launch = launchScene(adb, 'basic-rectangles', basicRunID, { samples: badSamples });
  await saveLog('negative-pixel-launch.log', launch);
  const pid = waitForProcess(adb);
  const pixelLog = waitForLog(() => currentProcessLog(adb, pid), (value) =>
    value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${basicRunID}`)
      || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_BEGIN run_id=${basicRunID}`),
  'PixelCopy 좌표 allowlist negative control');
  assert.ok(pixelLog.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${basicRunID}`),
    'Android가 변경된 표본 좌표를 거부하지 않았습니다.');
  assert.equal(pixelLog.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_BEGIN run_id=${basicRunID}`), false,
    'Android가 좌표 거부 전에 PixelCopy를 시작했습니다.');
  await writeFile(resolve(evidenceDirectory, 'negative-pixel.log'), `${formatLog(pixelLog)}\n`);

  const badSceneRunID = `${runID}-bad-scene`;
  const routeLaunch = launchScene(adb, 'unknown-c1133-scene', badSceneRunID, { samples: mobile.samples });
  await saveLog('negative-scene-launch.log', routeLaunch);
  const routePid = waitForProcess(adb);
  const routeLog = waitForLog(() => currentProcessLog(adb, routePid), (value) =>
    value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true')
      || value.includes('SPINON_C1133_GRID_SUMMARY')
      || value.includes('SPINON_C0410_DRAW'),
  '알 수 없는 scene route negative control');
  assert.ok(routeLog.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'),
    'MainActivity가 알 수 없는 C11.3.3 scene을 거부하지 않았습니다.');
  assert.equal(routeLog.includes('SPINON_C1133_GRID_SUMMARY'), false,
    '알 수 없는 scene이 runtime evaluation으로 진행됐습니다.');
  assert.equal(routeLog.includes('SPINON_C0410_DRAW'), false,
    '알 수 없는 scene이 GPU draw를 제출했습니다.');
  assert.equal(routeLog.includes('SPINON_C1133_GRID_PIXEL_MATRIX_BEGIN'), false,
    '알 수 없는 scene이 PixelCopy를 시작했습니다.');
  await writeFile(resolve(evidenceDirectory, 'negative-scene.log'), `${formatLog(routeLog)}\n`);
  return {
    pixelSamples: { status: 'rejected-before-readback', runID: basicRunID },
    sceneRoute: { status: 'rejected-before-runtime-draw', runID: badSceneRunID },
  };
}

await mkdir(evidenceDirectory, { recursive: true });
try {
  assert.equal(runtime.summary.sceneCount, 37);
  assert.equal(runtime.summary.nodeCount, 118);
  assert.equal(observation.cases.length, 26);
  assert.equal(sha256(await readFile(resolve(repositoryRoot, runtimePath))), mobile.inputs.runtimeMatrix.sha256);
  assert.equal(sha256(await readFile(resolve(repositoryRoot, runtimePath))), cyclePixels.inputs.runtimeMatrix.sha256);
  const { adb, metadata } = androidDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const platform = await captureMatrix(adb);
  const negative = await runNegativeControls(adb);
  const outcomeCounts = Object.fromEntries(['success', 'cascade-fallback', 'expected-rejection']
    .map((outcome) => [outcome, platform.observations.filter((item) => item.outcome === outcome).length]));
  assert.deepEqual(outcomeCounts, { success: 31, 'cascade-fallback': 5, 'expected-rejection': 1 });
  const totalComparisons = platform.observations.reduce((sum, item) => sum + (item.comparisons?.length ?? 0), 0);
  const result = {
    schema: 'spinon-c11-3-3-grid-template-areas-runtime-matrix-result/v1',
    status: 'matched', runID, target, metadata,
    fixture: { path: runtimePath, sha256: sha256(await readFile(resolve(repositoryRoot, runtimePath))) },
    trackListCyclePixels: {
      path: cyclePixelsPath,
      sha256: sha256(await readFile(resolve(repositoryRoot, cyclePixelsPath))),
      observations: platform.cyclePixelComparisons,
      totalSamples: platform.cyclePixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0),
      screenshotSha256: platform.trackListCycleScreenSha256,
    },
    unknownAreaOverflowFixture: {
      path: unknownAreaPixelsPath,
      sha256: sha256(await readFile(resolve(repositoryRoot, unknownAreaPixelsPath))),
      referencePath: unknownAreaPixels.inputs.reference.path,
      referenceSha256: sha256(unknownAreaReferenceBytes),
    },
    leadingNamedImplicitLinesFixture: {
      path: platform.leadingNamedImplicitLines.fixturePath,
      sha256: platform.leadingNamedImplicitLines.fixtureSha256,
      referencePath: platform.leadingNamedImplicitLines.referencePath,
      referenceSha256: platform.leadingNamedImplicitLines.referenceSha256,
      sceneCount: platform.leadingNamedImplicitLines.sceneCount,
      comparedVisibleNodes: platform.leadingNamedImplicitLines.comparedVisibleNodes,
      maximumGeometryErrorCssPx: platform.leadingNamedImplicitLines.maximumGeometryErrorCssPx,
      matchedPixelSamples: platform.leadingNamedImplicitLines.matchedPixelSamples,
      scenes: platform.leadingNamedImplicitLines.scenes,
    },
    oracle: {
      product: reference.oracle.product, referenceId: reference.referenceId,
      referenceSha256: sha256(await readFile(resolve(repositoryRoot, mobile.oracle.referencePath))),
      dpr: 1, viewportCssPx: mobile.oracle.viewportCssPx,
    },
    matrix: { sceneCount: 37, nodeCount: 118, outcomeCounts, comparedVisibleNodes: totalComparisons,
      maximumGeometryErrorCssPx: Math.max(0, ...platform.observations
        .map(({ maximumGeometryErrorCssPx = 0 }) => maximumGeometryErrorCssPx)) },
    platform: { appSha256: platform.appSha256, pixelComparison: platform.pixelComparison,
      cyclePixelComparisons: platform.cyclePixelComparisons,
      trackListCycleScreenSha256: platform.trackListCycleScreenSha256,
      unknownAreaOverflow: platform.unknownAreaOverflow,
      leadingNamedImplicitLines: platform.leadingNamedImplicitLines,
      screenSha256: platform.screenSha256 },
    negative, observations: platform.observations,
    interpretation: target === 'android-physical'
      ? 'Android SM-S731N physical-device fixture runtime evidence; SurfaceFlinger renderer is recorded verbatim and this does not establish other devices or drivers, WPT, or general CSS Grid support.'
      : 'Android API 37 AVD, SwiftShader software renderer; this is fixture runtime evidence and does not establish physical-device, hardware-GPU, WPT, or general CSS Grid support.',
  };
  await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.3.3 Grid template areas Android runtime matrix', '',
    `- Run: \`${runID}\``, `- Target: ${target === 'android-physical'
      ? `Android 실기기 ${metadata.model} / Android ${metadata.release} (API ${metadata.sdk}); renderer: ${metadata.renderer}`
      : 'Android API 37 AVD with SwiftShader'}`,
    '- Runtime matrix: 37 scenes / 118 nodes; 31 success, 5 cascade fallback, 1 expected rejection.',
    `- Chrome DPR 1 geometry: ${totalComparisons} visible node frames compared; maximum error ${result.matrix.maximumGeometryErrorCssPx} CSS px.`,
    `- Basic scene PixelCopy: ${platform.pixelComparison.matchedSamples}/${platform.pixelComparison.expectedSamples} exact; max channel error 0.`,
    `- Track-list cycling PixelCopy: ${platform.cyclePixelComparisons.length} scenes / ${platform.cyclePixelComparisons.reduce((sum, item) => sum + item.matchedSamples, 0)} RGBA samples exact; max channel error 0.`,
    `- Fixed unknown-area overflow: ${platform.unknownAreaOverflow.comparisons.length} Chrome geometry nodes; ${platform.unknownAreaOverflow.pixel.matchedSamples}/${platform.unknownAreaOverflow.pixel.expectedSamples} exact PixelCopy samples, including the red item outside the grid root.`,
    `- Leading named implicit lines: ${platform.leadingNamedImplicitLines.sceneCount} scenes / ${platform.leadingNamedImplicitLines.comparedVisibleNodes} visible nodes; maximum geometry error ${platform.leadingNamedImplicitLines.maximumGeometryErrorCssPx} CSS px; ${platform.leadingNamedImplicitLines.matchedPixelSamples}/7 PixelCopy samples exact.`,
    `- Negative controls: pixel samples ${negative.pixelSamples.status}; unknown scene ${negative.sceneRoute.status}.`,
    target === 'android-physical'
      ? '- 이 결과는 기록한 단일 실기기·OS·renderer에 한정되며 다른 실기기·driver, WPT 적합성, 일반 CSS Grid 지원을 입증하지 않습니다.'
      : '- AVD evidence does not establish physical-device or hardware-GPU behavior, WPT conformance, or general CSS Grid support.',
    '', '## Files', '', '- `runtime-*.log`: per-scene runtime, geometry, rejection and draw evidence',
    '- `result.json`: aggregate fixture, geometry and negative-control result',
    '- `pixelcopy.png`: basic scene Android SurfaceView PixelCopy',
    '- `pixelcopy-*.png`: track-list cycling Android SurfaceView PixelCopy samples',
    '- `unknown-area-overflow-pixelcopy.png`: fixed implicit-track overflow SurfaceView PixelCopy',
    '- `pixelcopy-leading-*.png`: named implicit line SurfaceView PixelCopy samples',
    ...(target === 'android-avd' ? [
      '- `android-track-list-cycle-screen.png`: both-axis cycling AVD screen capture',
      '- `android-screen.png`: final AVD screen capture',
    ] : []),
  ].join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({ ...result, evidenceDirectory }, null, 2)}\n`);
} catch (error) {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  throw error;
}
