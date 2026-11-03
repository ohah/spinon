import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  androidApkPath, androidComponent, command, packageName, repositoryRoot,
  simulatorAppPath,
} from './c11-2-3-grid-auto-placement-runtime-support.mjs';
import {
  compareC11AndroidPixelCopyScene,
  compareC11AndroidRootHtmlEnvironment,
} from './compare-c11-android-surface-pixel-matrix.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';

const target = process.argv[2];
assert.ok(target === 'android' || target === 'ios',
  '사용법: node capture-c11-root-html-dense-auto-placement-runtime-matrix.mjs <android|ios> <serial 또는 Simulator UDID>');
const serialOrUDID = process.argv[3];
assert.ok(serialOrUDID, '시뮬레이터 serial 또는 Simulator UDID가 필요합니다.');
assert.match(serialOrUDID, target === 'android' ? /^[A-Za-z0-9._:-]+$/ : /^[0-9A-Fa-f-]{36}$/,
  '시뮬레이터 식별자 형식이 잘못되었습니다.');

const matrixPath = 'tests/fixtures/css/c11/root-html-grid-dense-auto-placement-runtime-matrix-v1.json';
const matrixBytes = await readFile(resolve(repositoryRoot, matrixPath));
const matrix = JSON.parse(matrixBytes.toString('utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runID = `c11rhd-${target === 'android' ? 'avd' : 'ios'}-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
const kstDate = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-root-html-dense-auto-placement-runtime-${kstDate}/${runID}`);
await mkdir(evidenceDirectory, { recursive: true });
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function waitFor(predicate, description, timeoutMillis) {
  const deadline = Date.now() + timeoutMillis;
  while (Date.now() < deadline) {
    const value = predicate();
    if (value) return value;
    sleep(500);
  }
  throw new Error(`${description} 대기 시간이 초과되었습니다.`);
}

function androidTarget() {
  const adb = (args, options) => command('adb', ['-s', serialOrUDID, ...args], options);
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serialOrUDID} `) && line.includes(' device')),
    `ADB 시뮬레이터가 연결되지 않았습니다: ${serialOrUDID}`);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  assert.equal(qemu, '1', `Android 시뮬레이터가 아닙니다: ${serialOrUDID}`);
  const sdk = Number(getprop('ro.build.version.sdk'));
  assert.equal(sdk, 37, `검증 대상으로 고정한 Android API 37이 아닙니다: ${sdk}`);
  const device = {
    serial: serialOrUDID,
    sdk,
    release: getprop('ro.build.version.release'),
    model: getprop('ro.product.model'),
    fingerprint: getprop('ro.build.fingerprint'),
    qemu,
    density: adb(['shell', 'wm', 'density']).stdout.trim(),
    size: adb(['shell', 'wm', 'size']).stdout.trim(),
    renderer: 'Android AVD SurfaceView PixelCopy',
  };
  return { adb, device };
}

function iosTarget() {
  const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
  const state = JSON.parse(simctl(['list', 'devices', '--json']).stdout);
  const matches = Object.entries(state.devices).flatMap(([runtime, devices]) => devices
    .filter((device) => device.udid === serialOrUDID)
    .map((device) => ({ runtime, ...device })));
  assert.equal(matches.length, 1, `iOS Simulator UDID가 없습니다: ${serialOrUDID}`);
  assert.equal(matches[0].state, 'Booted', 'iOS Simulator가 Booted 상태가 아닙니다.');
  assert.equal(matches[0].isAvailable, true, 'iOS Simulator runtime을 사용할 수 없습니다.');
  assert.match(matches[0].runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `검증 대상으로 고정한 iOS 26.2가 아닙니다: ${matches[0].runtime}`);
  assert.equal(matches[0].name, 'iPhone 17 Pro', `예상하지 않은 iOS Simulator입니다: ${matches[0].name}`);
  return {
    simctl,
    device: { ...matches[0], udid: serialOrUDID,
      renderer: 'WGPU Metal drawable 직접 readback; OS compositor 제출 전' },
  };
}

function androidProcessLog(adb, pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime'], {
    timeout: 30_000, maxBuffer: 96 * 1024 * 1024,
  }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const fields = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return fields && Number(fields[1]) === pid;
  });
}

function iosProcessLog(simctl, pid) {
  const output = simctl([
    'spawn', serialOrUDID, 'log', 'show', '--last', '3m', '--style', 'syslog',
    '--predicate', 'process == "SpinonBootstrap"',
  ], { timeout: 45_000, maxBuffer: 96 * 1024 * 1024 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

function stopAndroidApp(adb) {
  adb(['shell', 'am', 'force-stop', packageName]);
  waitFor(() => {
    const pids = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim();
    return pids.length === 0 ? true : null;
  }, 'Android app force-stop', 30_000);
}

function assertRect(actual, expected, context) {
  let maximumError = 0;
  for (const field of ['x', 'y', 'width', 'height']) {
    const error = Math.abs(actual[field] - expected[field]);
    maximumError = Math.max(maximumError, error);
    assert.ok(error <= 0.5,
      `${context}/${field}: 실제=${actual[field]} Chrome=${expected[field]} 오차=${error} CSS px`);
  }
  return maximumError;
}

function parseFrames(lines) {
  const marker = target === 'android'
    ? 'SPINON_C111RHD_NODE_FRAME ' : 'SPINON_C111RHD_NODE_FRAME_SWIFT ';
  return lines.flatMap((line) => {
    const index = line.indexOf(marker);
    if (index < 0) return [];
    const match = line.slice(index + marker.length).match(/(?:state=[^ ]+\s+)?(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/);
    assert.ok(match, `frame 로그 형식이 잘못되었습니다: ${line}`);
    return [{ preorder: Number(match[1]), nodeId: Number(match[2]), rect: {
      x: Number(match[3]), y: Number(match[4]),
      width: Number(match[5]), height: Number(match[6]),
    } }];
  }).sort((left, right) => left.preorder - right.preorder);
}

function compareRuntimeScene(fixtureCase, lines, runID) {
  const success = fixtureCase.expected.outcome === 'success';
  const summaryMarker = success
    ? 'SPINON_C111_ROOT_HTML_DENSE_SUMMARY' : 'SPINON_C111_ROOT_HTML_DENSE_REJECTION';
  const summaries = lines.filter((line) => line.includes(`${summaryMarker} `)
    && line.includes(`run_id=${runID}`) && line.includes(`scene_id=${fixtureCase.id}`));
  assert.equal(summaries.length, 1, `${fixtureCase.id}: 실행 결과 표식이 하나여야 합니다.`);
  const summary = summaries[0];
  const status = Number(summary.match(/\bstatus=(-?\d+)\b/)?.[1]);
  assert.ok(Number.isInteger(status), `${fixtureCase.id}: runtime status가 없습니다.`);
  const frames = parseFrames(lines);
  const drawCounts = lines.flatMap((line) => [...line.matchAll(/presented boxes=(\d+)/g)]
    .map((match) => Number(match[1])));

  if (!success) {
    assert.equal(status, -11, `${fixtureCase.id}: 기대 거부 status`);
    assert.ok(summary.includes('expected=true'), `${fixtureCase.id}: 기대 거부로 표시되지 않았습니다.`);
    assert.ok(summary.includes(`code=${fixtureCase.expected.code}`), `${fixtureCase.id}: 오류 코드가 다릅니다.`);
    assert.ok(summary.includes(`property=Some("${fixtureCase.expected.property}`),
      `${fixtureCase.id}: 오류 property가 다릅니다.`);
    assert.ok(summary.includes(`node=Some(${fixtureCase.expected.appRuntimeNodeId})`),
      `${fixtureCase.id}: 앱 Root HTML 오류 NodeId가 다릅니다.`);
    assert.equal(frames.length, 0, `${fixtureCase.id}: 거부 장면에 frame이 남았습니다.`);
    assert.equal(drawCounts.some((count) => count > 0), false,
      `${fixtureCase.id}: 거부 장면이 빈 프레임이 아닌 draw를 제출했습니다.`);
    const pixelBegin = target === 'android'
      ? 'SPINON_C111RHD_GRID_PIXEL_MATRIX_BEGIN' : 'SPINON_C111RHD_METAL_PIXEL_BEGIN';
    assert.equal(lines.some((line) => line.includes(pixelBegin) && line.includes(`run_id=${runID}`)), false,
      `${fixtureCase.id}: 거부 장면에서 surface readback을 시작했습니다.`);
    return { sceneId: fixtureCase.id, outcome: 'expected-rejection', status,
      diagnostic: { code: fixtureCase.expected.code, property: fixtureCase.expected.property,
        runtimeNodeId: fixtureCase.expected.runtimeNodeId }, nodeCount: fixtureCase.nodeIds.length,
      frameCount: 0, positiveDrawCount: 0, maximumAbsoluteErrorCssPx: 0 };
  }

  assert.equal(status, 0, `${fixtureCase.id}: 성공 scene status`);
  assert.ok(summary.includes('layout=ready'), `${fixtureCase.id}: layout이 준비되지 않았습니다.`);
  const visibleNodes = fixtureCase.chrome.nodes.filter(({ display }) => display !== 'none');
  // Parsed Root HTML contributes the app wrapper plus four stable document
  // frames before the authored scene nodes.
  const expectedBoxCount = visibleNodes.length + 5;
  assert.ok(summary.includes(`boxes=${expectedBoxCount}`), `${fixtureCase.id}: runtime box 수가 다릅니다.`);
  assert.ok(drawCounts.includes(expectedBoxCount), `${fixtureCase.id}: 기대 box 수의 draw/present가 없습니다.`);

  const byNodeId = new Map(frames.map((frame) => [frame.nodeId, frame]));
  assert.equal(byNodeId.size, frames.length, `${fixtureCase.id}: 중복 frame node ID가 있습니다.`);
  const wrapper = byNodeId.get(1);
  assert.ok(wrapper, `${fixtureCase.id}: 360×800 runtime wrapper frame이 없습니다.`);
  let maximumAbsoluteErrorCssPx = assertRect(wrapper.rect,
    { x: 0, y: 0, width: 360, height: 800 }, `${fixtureCase.id}/wrapper`);
  const authoredFrames = frames.slice(-visibleNodes.length);
  assert.equal(authoredFrames.length, visibleNodes.length,
    `${fixtureCase.id}: authored visible node frame 수가 다릅니다.`);
  for (const [index, expectedNode] of visibleNodes.entries()) {
    const frame = authoredFrames[index];
    maximumAbsoluteErrorCssPx = Math.max(maximumAbsoluteErrorCssPx,
      assertRect(frame.rect, expectedNode.documentRect, `${fixtureCase.id}/${expectedNode.id}`));
  }
  return { sceneId: fixtureCase.id, outcome: 'matched', status, nodeCount: fixtureCase.nodeIds.length,
    frameCount: frames.length, maximumAbsoluteErrorCssPx, toleranceCssPx: 0.5,
    positiveDrawCount: drawCounts.filter((count) => count === expectedBoxCount).length };
}

async function main() {
  assert.equal(matrix.schema, 'spinon-css-c11-root-html-dense-auto-placement-runtime-matrix/v1');
  assert.equal(matrix.cases.length, 14);
  assert.equal(matrix.summary.sceneCount, 14);
  assert.equal(matrix.summary.implementationCandidateCount, 14);
  assert.equal(matrix.summary.successCount, 12);
  assert.equal(matrix.summary.expectedRejectCount, 2);
  assert.equal(matrix.summary.nodeCount, 60);
  const expectedPixelSampleCount = matrix.cases.reduce((total, fixtureCase) =>
    total + fixtureCase.pixelSamples.length, 0);
  assert.equal(expectedPixelSampleCount, 52);
  const generatorCheck = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-root-html-dense-auto-placement-runtime-matrix.mjs'),
    '--check',
  ]);
  const inputHashes = {};
  for (const [key, input] of Object.entries(matrix.inputs)) {
    const bytes = await readFile(resolve(repositoryRoot, input.path));
    inputHashes[key] = sha256(bytes);
    assert.equal(inputHashes[key], input.sha256, `${input.path}: 입력 SHA-256이 fixture와 다릅니다.`);
  }
  const { adb, simctl, device } = target === 'android' ? { ...androidTarget(), simctl: null } : { ...iosTarget(), adb: null };
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'input-check.log'), generatorCheck.stdout);

  const buildArgs = [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_1_ROOT_HTML_DENSE_PLACEMENT_FIXTURE_TEST=1',
    ...(target === 'ios' ? ['SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1'] : []),
    ...(target === 'android' ? ['bun', 'run', 'build:android'] : ['bun', 'run', 'build:ios-sim']),
  ];
  const build = command('mise', buildArgs, { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  let appSha256;
  if (target === 'android') {
    const appBytes = await readFile(androidApkPath);
    appSha256 = sha256(appBytes);
    const install = adb(['install', '-r', androidApkPath], { timeout: 180_000 });
    await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);
  } else {
    const executableName = command('/usr/libexec/PlistBuddy', [
      '-c', 'Print CFBundleExecutable', resolve(simulatorAppPath, 'Info.plist'),
    ]).stdout.trim();
    appSha256 = sha256(await readFile(resolve(simulatorAppPath, executableName)));
    const install = simctl(['install', serialOrUDID, simulatorAppPath], { timeout: 120_000 });
    await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);
  }

  const observations = [];
  const pixelObservations = [];
  try {
    for (const fixtureCase of matrix.cases) {
      const sceneRunID = `${runID}-${fixtureCase.id}`;
      const success = fixtureCase.expected.outcome === 'success';
      const pixelSamples = fixtureCase.pixelSamples;
      let pid;
      let launch;
      if (target === 'android') {
        stopAndroidApp(adb);
        adb(['logcat', '-c']);
        const args = ['shell', 'am', 'start', '-S', '--activity-clear-task',
          '-n', androidComponent,
          '--ez', 'spinon_c111_root_html_dense_placement', 'true',
          '--es', 'spinon_c111_root_html_dense_scene', fixtureCase.id,
          '--es', 'spinon_c111_root_html_dense_run_id', sceneRunID];
        if (success) {
          const payload = Buffer.from(JSON.stringify(pixelSamples.map(({ id, nodeId, xCssPx, yCssPx }) =>
            ({ id, nodeId, xCssPx, yCssPx })))).toString('base64url');
          args.push('--es', 'spinon_c11_grid_case', fixtureCase.id,
            '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
            '--es', 'spinon_c11_grid_pixel_matrix_samples', payload);
        }
        launch = adb(args, { timeout: 30_000 });
        pid = waitFor(() => {
          const value = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim();
          const parsed = Number(value.split(/\s+/)[0]);
          return Number.isInteger(parsed) && parsed > 0 ? parsed : null;
        }, `${fixtureCase.id}: Android app PID`, 90_000);
      } else {
        simctl(['terminate', serialOrUDID, packageName], { allowFailure: true, timeout: 30_000 });
        const args = ['launch', '--terminate-running-process', serialOrUDID, packageName,
          '--spinon-c111-root-html-dense-placement',
          `--spinon-c111-root-html-dense-scene=${fixtureCase.id}`,
          `--spinon-c111-root-html-dense-run-id=${sceneRunID}`];
        if (success) {
          const payload = Buffer.from(JSON.stringify(pixelSamples)).toString('base64');
          args.push(`--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
            `--spinon-c11-grid-metal-surface-samples=${payload}`);
        }
        launch = simctl(args, { timeout: 90_000 });
        pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
        assert.ok(Number.isInteger(pid) && pid > 0,
          `${fixtureCase.id}: Simulator가 새 앱 PID를 반환하지 않았습니다.`);
      }
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`),
        `${launch.stdout}${launch.stderr}`);

      const lines = waitFor(() => {
        const current = target === 'android' ? androidProcessLog(adb, pid) : iosProcessLog(simctl, pid);
        const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
          && !line.includes('status=0 ') && !line.includes('status=-12 '));
        if (failedDraw) throw new Error(`${fixtureCase.id}: draw 실패: ${failedDraw}`);
        const resultMarker = success
          ? 'SPINON_C111_ROOT_HTML_DENSE_SUMMARY' : 'SPINON_C111_ROOT_HTML_DENSE_REJECTION';
        const hasResult = current.some((line) => line.includes(`${resultMarker} `)
          && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
        const hasFrames = !success || parseFrames(current).length >= 1;
        const hasDraw = !success || current.some((line) => /presented boxes=\d+/.test(line));
        const pixelSummaryMarker = target === 'android'
          ? 'SPINON_C111RHD_GRID_PIXEL_MATRIX_SUMMARY' : 'SPINON_C111RHD_METAL_PIXEL_SUMMARY';
        const hasPixelSummary = !success || current.some((line) => line.includes(pixelSummaryMarker)
          && line.includes(`run_id=${sceneRunID}`)
          && line.includes(target === 'android' ? 'status=captured' : 'status=matched'));
        const failedPixel = success && current.find((line) => line.includes(pixelSummaryMarker)
          && line.includes(`run_id=${sceneRunID}`)
          && !line.includes(target === 'android' ? 'status=captured' : 'status=matched'));
        if (failedPixel) throw new Error(`${fixtureCase.id}: 표면 pixel 검증 실패: ${failedPixel}`);
        return hasResult && hasFrames && hasDraw && hasPixelSummary ? current : null;
      }, `${fixtureCase.id}: runtime/draw/pixel 결과`, target === 'android' ? 90_000 : 150_000);
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-runtime.log`), `${lines.join('\n')}\n`);

      const runtime = compareRuntimeScene(fixtureCase, lines, sceneRunID);
      const screenshotBytes = target === 'android'
        ? adb(['exec-out', 'screencap', '-p'], {
          encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
        }).stdout
        : null;
      let screenshot = screenshotBytes;
      if (target === 'ios') {
        const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.id}-ios-simulator.png`);
        const capture = simctl(['io', serialOrUDID, 'screenshot', screenshotPath], { timeout: 30_000 });
        await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-screenshot.log`),
          `${capture.stdout}${capture.stderr}`);
        screenshot = await readFile(screenshotPath);
      }
      assert.deepEqual([...screenshot.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10],
        `${fixtureCase.id}: simulator screenshot PNG signature`);
      const screenshotName = `${fixtureCase.id}-${target}.png`;
      await writeFile(resolve(evidenceDirectory, screenshotName), screenshot);
      runtime.screenshot = screenshotName;
      runtime.screenshotSha256 = sha256(screenshot);
      runtime.runID = sceneRunID;
      runtime.processId = pid;

      if (success) {
        let pixelComparison;
        if (target === 'android') {
          const captureLine = lines.find((line) => line.includes(
            'SPINON_C111RHD_GRID_PIXEL_MATRIX_CAPTURE status=ok') && line.includes(`run_id=${sceneRunID}`));
          assert.ok(captureLine, `${fixtureCase.id}: Android PixelCopy success 표식이 없습니다.`);
          const devicePngPath = captureLine.match(/\bpng=(\S+)/)?.[1];
          assert.ok(devicePngPath, `${fixtureCase.id}: PixelCopy PNG 경로가 없습니다.`);
          const devicePng = adb(['exec-out', 'run-as', packageName, 'cat', devicePngPath], {
            encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
          }).stdout;
          pixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
            scene: { id: fixtureCase.id, samples: pixelSamples },
            runID: sceneRunID,
            pngBytes: devicePng,
            viewportCssPx: matrix.viewportCssPx,
            markerPrefix: 'C111RHD',
            cacheDirectory: 'c111-root-html-dense-surface-pixel-matrix',
          });
          runtime.environmentMapping = compareC11AndroidRootHtmlEnvironment(lines.join('\n'), {
            viewportCssPx: matrix.viewportCssPx,
            density: pixelComparison.density,
            surfaceScale: pixelComparison.scale,
          });
          const surfacePng = `${fixtureCase.id}-surface-pixels.png`;
          await writeFile(resolve(evidenceDirectory, surfacePng), devicePng);
          pixelComparison.png = surfacePng;
        } else {
          pixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
            fixture: { fixtureId: fixtureCase.id, sceneId: fixtureCase.id,
              runtime: { viewportCssPx: matrix.viewportCssPx }, samples: pixelSamples },
            runID: sceneRunID,
            markerPrefix: 'C111RHD',
          });
        }
        const pixelFile = `${fixtureCase.id}-pixel-comparison.json`;
        await writeFile(resolve(evidenceDirectory, pixelFile), `${JSON.stringify(pixelComparison, null, 2)}\n`);
        runtime.surfacePixelComparison = pixelComparison;
        runtime.pixelComparison = pixelFile;
        pixelObservations.push({ ...pixelComparison, scene: fixtureCase.id });
      }
      observations.push(runtime);
      process.stdout.write(`${fixtureCase.id}: ${runtime.outcome}${success
        ? ` · geometry ${runtime.maximumAbsoluteErrorCssPx} CSS px · pixel ${runtime.surfacePixelComparison.maximumChannelError}`
        : ` · expected ${runtime.diagnostic.code}`}\n`);
    }
  } finally {
    if (target === 'android') stopAndroidApp(adb);
    else simctl(['terminate', serialOrUDID, packageName], { allowFailure: true, timeout: 30_000 });
  }

  const successfulScenes = observations.filter(({ outcome }) => outcome === 'matched').length;
  const rejectedScenes = observations.filter(({ outcome }) => outcome === 'expected-rejection').length;
  const nodeCount = observations.reduce((total, scene) => total + scene.nodeCount, 0);
  const pixelSampleCount = pixelObservations.reduce((total, scene) => total + scene.expectedSamples, 0);
  assert.equal(successfulScenes, 12);
  assert.equal(rejectedScenes, 2);
  assert.equal(nodeCount, 60);
  assert.equal(pixelObservations.length, 12);
  assert.equal(pixelSampleCount, expectedPixelSampleCount);
  const summary = {
    schema: 'spinon-c11-root-html-dense-auto-placement-mobile-runtime-matrix/v1',
    status: 'matched', runID, target: serialOrUDID, platform: device, appSha256,
    inputs: { matrix: sha256(matrixBytes), ...inputHashes },
    generatorCheck: generatorCheck.stdout.trim(),
    summary: {
      scenes: observations.length, successfulScenes, rejectedScenes, authoredNodes: nodeCount,
      geometryToleranceCssPx: 0.5,
      maximumAbsoluteErrorCssPx: Math.max(0,
        ...observations.map(({ maximumAbsoluteErrorCssPx = 0 }) => maximumAbsoluteErrorCssPx)),
      surfacePixelReadback: {
        status: 'matched', scenes: pixelObservations.length, samples: pixelSampleCount,
        maximumChannelError: Math.max(0, ...pixelObservations.map(({ maximumChannelError }) => maximumChannelError)),
        renderer: device.renderer,
      },
      freshProcessPerScene: true,
      screenshotBytesAreNotPixelParityEvidence: true,
    },
    observations,
    limitations: [
      '이 검증은 고정 Root HTML dense runtime fixture와 독립 Chrome 156 기대 geometry·색 표본에 한정되며 전체 CSS Grid 지원 또는 WPT 적합성을 뜻하지 않는다.',
      ...(target === 'android'
        ? ['Android 결과는 API 37 ARM64 AVD의 emulator SurfaceView PixelCopy이며 실기기나 hardware GPU 결과가 아니다.']
        : ['iOS 결과는 iPhone 17 Pro, iOS 26.2 Simulator의 Metal drawable 직접 readback이며 OS compositor 이후 화면 결과가 아니다.']),
      '스크린샷은 실행 상태를 보여 주며 자체적으로 pixel parity 근거로 사용하지 않았다.',
    ],
  };
  await writeFile(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 Root HTML dense auto-placement 앱 시뮬레이터 검증', '',
    `- 실행 ID: \`${runID}\``, `- 대상: \`${serialOrUDID}\``,
    `- 결과: 14 scenes · 성공 ${successfulScenes}/12 · 기대 거부 ${rejectedScenes}/2 · authored node ${nodeCount}/60`,
    `- geometry 최대 오차: ${summary.summary.maximumAbsoluteErrorCssPx} CSS px (허용 0.5 CSS px)`,
    `- surface pixel: ${pixelObservations.length} scenes · ${pixelSampleCount}/${pixelSampleCount} exact · 최대 채널 오차 ${summary.summary.surfacePixelReadback.maximumChannelError}`,
    `- 앱 SHA-256: \`${appSha256}\``, '',
    '기계 판독 결과와 장면별 로그·화면은 [comparison.json](comparison.json) 및 이 디렉터리의 scene artifact에서 확인할 수 있다.',
  ].join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID, platform: device,
    summary: summary.summary, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
