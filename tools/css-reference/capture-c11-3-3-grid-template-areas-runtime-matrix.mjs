import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import { selectAndroidAVDSerial } from './c11-3-3-grid-template-areas-runtime-device.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const runtimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
const mobilePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-mobile-surface-pixel-matrix-v1.json';
const runtime = JSON.parse(await readFile(resolve(repositoryRoot, runtimePath), 'utf8'));
const mobile = JSON.parse(await readFile(resolve(repositoryRoot, mobilePath), 'utf8'));
const reference = JSON.parse(await readFile(resolve(repositoryRoot, mobile.oracle.referencePath), 'utf8'));
const observation = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
const target = process.argv[2];
assert.equal(target, 'android-avd',
  '사용법: node capture-c11-3-3-grid-template-areas-runtime-matrix.mjs android-avd [emulator serial]');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const serialHint = process.argv[3] ?? process.env.SPINON_ANDROID_EMULATOR_SERIAL ?? 'emulator-5554';
const runID = `c1133-matrix-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-3-3-grid-template-areas-runtime-${date}`, `android-avd-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
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
  await writeFile(resolve(evidenceDirectory, path), `${result.stdout ?? ''}${result.stderr ?? ''}`);
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

function compareGeometry(scene, log) {
  const chromeCase = observation.cases.find(({ id }) => id === scene.id);
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

function androidDevice() {
  const devices = command('adb', ['devices', '-l']).stdout;
  const serial = selectAndroidAVDSerial(devices, serialHint);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const sdk = Number(getprop('ro.build.version.sdk'));
  assert.equal(getprop('ro.kernel.qemu'), '1', '선택한 Android 대상이 AVD가 아닙니다.');
  assert.equal(sdk, 37, `고정 Android API 37 AVD가 아닙니다: API ${sdk}`);
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음';
  assert.match(renderer, /SwiftShader/i, `Android AVD가 SwiftShader를 사용하지 않습니다: ${renderer}`);
  return {
    adb,
    metadata: {
      serial, model: getprop('ro.product.model'), release: getprop('ro.build.version.release'),
      sdk, qemu: true, fingerprint: getprop('ro.build.fingerprint'), renderer,
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
  for (const scene of runtime.cases) {
    const sceneRunID = `${runID}-${String(scene.index).padStart(2, '0')}-${scene.id}`;
    const expectedReject = scene.expected.outcome === 'expected-rejection';
    const basicPixelSamples = scene.id === 'basic-rectangles' ? mobile.samples : null;
    const launch = launchScene(adb, scene.id, sceneRunID, { samples: basicPixelSamples });
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
    await writeFile(resolve(evidenceDirectory, logPath), `${log}\n`);
    if (!expectedReject) {
      const evalLine = log.split(/\r?\n/).find((line) => line.includes(evalMarker));
      assert.ok(evalLine?.includes('status=0 layout=ready'),
        `${scene.id}: runtime 평가가 성공하지 않았습니다: ${evalLine ?? log}`);
      log = waitForLog(() => currentProcessLog(adb, pid), (value) =>
        value.includes(`SPINON_C0410_DRAW status=0 presented boxes=`)
          && (scene.id !== 'basic-rectangles'
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${sceneRunID}`)
            || value.includes(`SPINON_C1133_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)),
      `${scene.id}: GPU presentation / PixelCopy 결과`, 60_000);
      await writeFile(resolve(evidenceDirectory, logPath), `${log}\n`);
    }
    if (expectedReject) {
      observations.push({ index: scene.index, sceneId: scene.id,
        outcome: 'expected-rejection', rejection: validateReject(scene, log, sceneRunID), logPath });
      continue;
    }
    const geometry = compareGeometry(scene, log);
    let pixel = null;
    if (scene.id === 'basic-rectangles') {
      const capture = log.split(/\r?\n/).find((line) => line.includes(
        `SPINON_C1133_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${sceneRunID}`));
      assert.ok(capture, 'basic-rectangles: PixelCopy capture marker가 없습니다.');
      const devicePng = capture.match(/\bpng=(\S+)/)?.[1];
      assert.ok(devicePng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1133-grid-surface-pixel-matrix/'),
        'PixelCopy 경로가 C11.3.3 전용 cache 경로가 아닙니다.');
      const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
        `cache/c1133-grid-surface-pixel-matrix/${devicePng.split('/').at(-1)}`], { encoding: null }).stdout;
      pixel = compareC11AndroidPixelCopyScene(log, {
        scene: { id: mobile.sceneId, samples: mobile.samples }, runID: sceneRunID, pngBytes,
        viewportCssPx: mobile.oracle.viewportCssPx, markerPrefix: 'C1133',
        cacheDirectory: 'c1133-grid-surface-pixel-matrix',
      });
      await writeFile(resolve(evidenceDirectory, 'pixelcopy.png'), pngBytes);
      pixelComparison = pixel;
    }
    observations.push({
      index: scene.index, sceneId: scene.id, outcome: matrixCaseStatus(scene),
      boxes: geometry.boxes, frames: geometry.frames.length,
      comparisons: geometry.comparisons, maximumGeometryErrorCssPx: Math.max(
        0, ...geometry.comparisons.map(({ maximumErrorCssPx }) => maximumErrorCssPx)),
      pixel, logPath,
    });
  }
  const screen = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
  await writeFile(resolve(evidenceDirectory, 'android-screen.png'), screen);
  return { appSha256, observations, pixelComparison, screenSha256: sha256(screen) };
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
  await writeFile(resolve(evidenceDirectory, 'negative-pixel.log'), `${pixelLog}\n`);

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
  await writeFile(resolve(evidenceDirectory, 'negative-scene.log'), `${routeLog}\n`);
  return {
    pixelSamples: { status: 'rejected-before-readback', runID: basicRunID },
    sceneRoute: { status: 'rejected-before-runtime-draw', runID: badSceneRunID },
  };
}

await mkdir(evidenceDirectory, { recursive: true });
try {
  assert.equal(runtime.summary.sceneCount, 26);
  assert.equal(runtime.summary.nodeCount, 65);
  assert.equal(observation.cases.length, runtime.cases.length);
  assert.equal(sha256(await readFile(resolve(repositoryRoot, runtimePath))), mobile.inputs.runtimeMatrix.sha256);
  const { adb, metadata } = androidDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const platform = await captureMatrix(adb);
  const negative = await runNegativeControls(adb);
  const outcomeCounts = Object.fromEntries(['success', 'cascade-fallback', 'expected-rejection']
    .map((outcome) => [outcome, platform.observations.filter((item) => item.outcome === outcome).length]));
  assert.deepEqual(outcomeCounts, { success: 19, 'cascade-fallback': 5, 'expected-rejection': 2 });
  const totalComparisons = platform.observations.reduce((sum, item) => sum + (item.comparisons?.length ?? 0), 0);
  const result = {
    schema: 'spinon-c11-3-3-grid-template-areas-runtime-matrix-result/v1',
    status: 'matched', runID, target, metadata,
    fixture: { path: runtimePath, sha256: sha256(await readFile(resolve(repositoryRoot, runtimePath))) },
    oracle: {
      product: reference.oracle.product, referenceId: reference.referenceId,
      referenceSha256: sha256(await readFile(resolve(repositoryRoot, mobile.oracle.referencePath))),
      dpr: 1, viewportCssPx: mobile.oracle.viewportCssPx,
    },
    matrix: { sceneCount: 26, nodeCount: 65, outcomeCounts, comparedVisibleNodes: totalComparisons,
      maximumGeometryErrorCssPx: Math.max(0, ...platform.observations
        .map(({ maximumGeometryErrorCssPx = 0 }) => maximumGeometryErrorCssPx)) },
    platform: { appSha256: platform.appSha256, pixelComparison: platform.pixelComparison,
      screenSha256: platform.screenSha256 },
    negative, observations: platform.observations,
    interpretation: 'Android API 37 AVD, SwiftShader software renderer; this is fixture runtime evidence and does not establish physical-device, hardware-GPU, WPT, or general CSS Grid support.',
  };
  await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.3.3 Grid template areas Android runtime matrix', '',
    `- Run: \`${runID}\``, '- Target: Android API 37 AVD with SwiftShader',
    '- Runtime matrix: 26 scenes / 65 nodes; 19 success, 5 cascade fallback, 2 expected rejection.',
    `- Chrome DPR 1 geometry: ${totalComparisons} visible node frames compared; maximum error ${result.matrix.maximumGeometryErrorCssPx} CSS px.`,
    `- Basic scene PixelCopy: ${platform.pixelComparison.matchedSamples}/${platform.pixelComparison.expectedSamples} exact; max channel error 0.`,
    `- Negative controls: pixel samples ${negative.pixelSamples.status}; unknown scene ${negative.sceneRoute.status}.`,
    '- AVD evidence does not establish physical-device or hardware-GPU behavior, WPT conformance, or general CSS Grid support.',
    '', '## Files', '', '- `runtime-*.log`: per-scene runtime, geometry, rejection and draw evidence',
    '- `result.json`: aggregate fixture, geometry and negative-control result',
    '- `pixelcopy.png`: basic scene Android SurfaceView PixelCopy',
    '- `android-screen.png`: final AVD screen capture', '',
  ].join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({ ...result, evidenceDirectory }, null, 2)}\n`);
} catch (error) {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  throw error;
}
