import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  androidApkPath, androidComponent, command, packageName, repositoryRoot,
  simulatorAppPath,
} from './c11-2-3-grid-auto-placement-runtime-support.mjs';
import {
  androidDensityFromWmOutput,
  compareC11AndroidPixelCopyScene,
  compareC11AndroidRootHtmlEnvironment,
} from './compare-c11-android-surface-pixel-matrix.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';

const target = process.argv[2];
assert.ok(target === 'android' || target === 'ios',
  '사용법: node capture-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs <android|ios> <serial 또는 Simulator UDID>');
const serialOrUDID = process.argv[3];
assert.ok(serialOrUDID, '시뮬레이터 serial 또는 Simulator UDID가 필요합니다.');
assert.match(serialOrUDID, target === 'android' ? /^[A-Za-z0-9._:-]+$/ : /^[0-9A-Fa-f-]{36}$/,
  '시뮬레이터 식별자 형식이 잘못되었습니다.');

const matrixPath = 'tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime-matrix-v1.json';
const matrixBytes = await readFile(resolve(repositoryRoot, matrixPath));
const matrix = JSON.parse(matrixBytes.toString('utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runID = `c111cfm-${target === 'android' ? 'avd' : 'ios'}-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
const kstDate = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-root-html-column-flow-mixed-runtime-${kstDate}/${runID}`);
await mkdir(evidenceDirectory, { recursive: true });
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

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
  assert.equal(sdk, 37, `검증 대상을 API 37로 고정했으나 실제 값이 ${sdk}입니다.`);
  return {
    adb,
    device: {
      serial: serialOrUDID,
      sdk,
      release: getprop('ro.build.version.release'),
      model: getprop('ro.product.model'),
      fingerprint: getprop('ro.build.fingerprint'),
      qemu,
      density: adb(['shell', 'wm', 'density']).stdout.trim(),
      size: adb(['shell', 'wm', 'size']).stdout.trim(),
      renderer: 'Android API 37 ARM64 AVD SurfaceView PixelCopy',
    },
  };
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
    `검증 대상은 iOS 26.2이나 실제 runtime은 ${matches[0].runtime}입니다.`);
  assert.equal(matches[0].name, 'iPhone 17 Pro', `예상하지 않은 iOS Simulator입니다: ${matches[0].name}`);
  return { simctl, device: { ...matches[0], udid: serialOrUDID,
    renderer: 'iPhone 17 Pro iOS 26.2 Simulator Metal drawable 직접 readback (OS compositor 전)' } };
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
    const value = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim();
    return value ? null : true;
  }, 'Android app force-stop', 30_000);
}

function parseFrames(lines) {
  const marker = target === 'android'
    ? 'SPINON_C111CFM_NODE_FRAME ' : 'SPINON_C111CFM_NODE_FRAME_SWIFT ';
  return lines.flatMap((line) => {
    const index = line.indexOf(marker);
    if (index < 0) return [];
    const match = line.slice(index + marker.length).match(
      /(?:state=[^ ]+\s+)?(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/,
    );
    assert.ok(match, `frame 로그 형식이 잘못되었습니다: ${line}`);
    return [{ preorder: Number(match[1]), nodeId: Number(match[2]), rect: {
      x: Number(match[3]), y: Number(match[4]),
      width: Number(match[5]), height: Number(match[6]),
    } }];
  }).sort((left, right) => left.preorder - right.preorder);
}

function assertRect(actual, expected, context) {
  for (const [field, index] of [['x', 0], ['y', 1], ['width', 2], ['height', 3]]) {
    assert.equal(actual[field], expected[index],
      `${context}/${field}: 실제=${actual[field]} 기대=${expected[index]} CSS px`);
  }
}

function sceneMarkers(fixtureCase) {
  return fixtureCase.outcome === 'success'
    ? ['SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_SUMMARY',
      'SPINON_C111CFM_GRID_PIXEL_MATRIX_SUMMARY', 'SPINON_C111CFM_METAL_PIXEL_SUMMARY']
    : ['SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_REJECTION'];
}

function compareRuntimeScene(fixtureCase, lines, sceneRunID) {
  const summaryMarker = fixtureCase.outcome === 'success'
    ? 'SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_SUMMARY'
    : 'SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_REJECTION';
  const summaries = lines.filter((line) => line.includes(`${summaryMarker} `)
    && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
  assert.equal(summaries.length, 1, `${fixtureCase.id}: 실행 결과 표식 수`);
  const summary = summaries[0];
  const status = Number(summary.match(/\bstatus=(-?\d+)\b/)?.[1]);
  assert.ok(Number.isInteger(status), `${fixtureCase.id}: runtime status 없음`);
  const frames = parseFrames(lines);
  const drawCounts = lines.flatMap((line) => [...line.matchAll(/presented boxes=(\d+)/g)]
    .map((match) => Number(match[1])));

  if (fixtureCase.outcome === 'expected-rejection') {
    const diagnostic = fixtureCase.expectedDiagnostic;
    assert.equal(status, -11, `${fixtureCase.id}: rejection status`);
    assert.ok(summary.includes('expected=true'), `${fixtureCase.id}: expected rejection marker`);
    assert.ok(summary.includes(`code=${diagnostic.code}`), `${fixtureCase.id}: diagnostic code`);
    assert.ok(summary.includes(`property=Some("${diagnostic.property}`),
      `${fixtureCase.id}: diagnostic property`);
    const nodeId = Number(summary.match(/\bnode=Some\((\d+)\)/)?.[1]);
    assert.ok(Number.isInteger(nodeId) && nodeId > 0, `${fixtureCase.id}: runtime node ID`);
    assert.equal(nodeId, diagnostic.nodeId, `${fixtureCase.id}: rejected node ID`);
    assert.equal(frames.length, 0, `${fixtureCase.id}: rejected scene published frames`);
    assert.equal(drawCounts.some((count) => count > 0), false,
      `${fixtureCase.id}: rejected scene submitted a non-empty draw`);
    assert.equal(lines.some((line) => line.includes('SPINON_C111CFM_GRID_PIXEL_MATRIX_BEGIN ')
      || line.includes('SPINON_C111CFM_METAL_PIXEL_BEGIN ')), false,
    `${fixtureCase.id}: rejected scene started surface readback`);
    return { sceneId: fixtureCase.id, outcome: 'expected-rejection', status,
      diagnostic: { ...diagnostic, observedNodeId: nodeId }, frameCount: 0,
      positiveDrawCount: 0, maximumAbsoluteErrorCssPx: 0 };
  }

  assert.equal(status, 0, `${fixtureCase.id}: success status`);
  assert.ok(summary.includes('layout=ready'), `${fixtureCase.id}: layout not ready`);
  const visibleItems = fixtureCase.items.filter(({ frameCssPx }) => frameCssPx !== null);
  const expectedPaintedBoxCount = fixtureCase.items.filter(({ frameCssPx, visibility }) =>
    frameCssPx !== null && visibility !== 'hidden').length + 3;
  assert.ok(summary.includes(`boxes=${expectedPaintedBoxCount}`), `${fixtureCase.id}: box count`);
  assert.ok(drawCounts.includes(expectedPaintedBoxCount), `${fixtureCase.id}: non-empty draw/present absent`);
  const evalMarker = target === 'android'
    ? 'SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_EVAL '
    : 'SPINON_C111_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_EVAL_RUN ';
  assert.equal(lines.filter((line) => line.includes(evalMarker)
    && line.includes(`run_id=${sceneRunID}`)).length, 1,
  `${fixtureCase.id}: mixed column-flow eval marker`);
  const legacyEvalMarker = target === 'android'
    ? 'SPINON_C111_ROOT_HTML_DENSE_EVAL '
    : 'SPINON_C111_ROOT_HTML_DENSE_EVAL_RUN ';
  assert.equal(lines.filter((line) => line.includes(legacyEvalMarker)
    && line.includes(`run_id=${sceneRunID}`)).length, 0,
  `${fixtureCase.id}: leaked dense-flow run marker`);

  const byNodeId = new Map(frames.map((frame) => [frame.nodeId, frame]));
  assert.equal(byNodeId.size, frames.length, `${fixtureCase.id}: duplicate frame node IDs`);
  const root = byNodeId.get(1);
  assert.ok(root, `${fixtureCase.id}: root viewport frame absent`);
  assertRect(root.rect, [0, 0, ...Object.values(fixtureCase.viewportCssPx)],
    `${fixtureCase.id}/root`);
  const itemFrames = frames.slice(-visibleItems.length);
  assert.equal(itemFrames.length, visibleItems.length, `${fixtureCase.id}: authored frame count`);
  for (const [index, item] of visibleItems.entries()) {
    assertRect(itemFrames[index].rect, item.frameCssPx, `${fixtureCase.id}/${item.id}`);
  }
  return { sceneId: fixtureCase.id, outcome: 'matched', status,
    authoredItemCount: fixtureCase.items.length,
    participatingItemCount: visibleItems.length,
    frameCount: frames.length,
    maximumAbsoluteErrorCssPx: 0,
    geometryToleranceCssPx: 0,
    positiveDrawCount: drawCounts.filter((count) => count === expectedPaintedBoxCount).length };
}

async function main() {
  assert.equal(matrix.schema,
    'spinon-css-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix/v1');
  assert.deepEqual(matrix.summary, {
    sceneCount: 9,
    successfulScenes: 8,
    expectedRejectScenes: 1,
    participatingItems: 31,
    pixelSampleCount: 31,
    geometryToleranceCssPx: 0,
  });
  const builderPath = resolve(repositoryRoot,
    'tools/css-reference/build-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs');
  const builder = command(process.execPath, [builderPath, '--check']);
  const { adb, simctl, device } = target === 'android'
    ? { ...androidTarget(), simctl: null } : { ...iosTarget(), adb: null };
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'matrix-check.log'), `${builder.stdout}${builder.stderr}`);

  const buildArgs = [
    'exec', '--', 'env',
    `SPINON_V8_DIR=${resolve(repositoryRoot, 'build/c11-v8-rebuild/v8-source/v8')}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C11_1_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_FIXTURE_TEST=1',
    ...(target === 'ios' ? ['SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1'] : []),
    ...(target === 'android'
      ? ['ANDROID_HOME=/Users/yoonhb/Library/Android/sdk',
        'ANDROID_SDK_ROOT=/Users/yoonhb/Library/Android/sdk', 'bun', 'run', 'build:android']
      : ['bun', 'run', 'build:ios-sim']),
  ];
  const build = command('mise', buildArgs,
    { cwd: repositoryRoot, timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);

  let appSha256;
  if (target === 'android') {
    appSha256 = sha256(await readFile(androidApkPath));
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
  const rejectionNodeIDs = [];
  try {
    for (const fixtureCase of matrix.scenes) {
      const sceneRunID = `${runID}-${fixtureCase.id}`;
      const success = fixtureCase.outcome === 'success';
      const samples = fixtureCase.pixelSamples;
      let pid;
      let launch;
      if (target === 'android') {
        stopAndroidApp(adb);
        adb(['logcat', '-c']);
        const args = ['shell', 'am', 'start', '-S', '--activity-clear-task',
          '-n', androidComponent,
          '--ez', 'spinon_c111_root_html_column_flow_mixed_cross_axis', 'true',
          '--es', 'spinon_c111_root_html_column_flow_mixed_cross_axis_scene', fixtureCase.id,
          '--es', 'spinon_c111_root_html_column_flow_mixed_cross_axis_run_id', sceneRunID];
        if (success) {
          const payload = Buffer.from(JSON.stringify(samples.map(({ id, nodeId, xCssPx, yCssPx }) =>
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
          '--spinon-c111-root-html-column-flow-mixed-cross-axis',
          `--spinon-c111-root-html-column-flow-mixed-cross-axis-scene=${fixtureCase.id}`,
          `--spinon-c111-root-html-column-flow-mixed-cross-axis-run-id=${sceneRunID}`];
        if (success) {
          const payload = Buffer.from(JSON.stringify(samples)).toString('base64');
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

      const markers = sceneMarkers(fixtureCase);
      const lines = waitFor(() => {
        const current = target === 'android'
          ? androidProcessLog(adb, pid) : iosProcessLog(simctl, pid);
        const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
          && !line.includes('status=0 ') && !line.includes('status=-12 '));
        if (failedDraw) throw new Error(`${fixtureCase.id}: draw 실패: ${failedDraw}`);
        const hasResult = current.some((line) => line.includes(`${markers[0]} `)
          && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
        const hasFrames = !success || parseFrames(current).length >= 1;
        const hasDraw = !success || current.some((line) => /presented boxes=\d+/.test(line));
        const pixelSummaryMarker = target === 'android'
          ? 'SPINON_C111CFM_GRID_PIXEL_MATRIX_SUMMARY'
          : 'SPINON_C111CFM_METAL_PIXEL_SUMMARY';
        const hasPixelSummary = !success || current.some((line) => line.includes(pixelSummaryMarker)
          && line.includes(`run_id=${sceneRunID}`)
          && line.includes(target === 'android' ? 'status=captured' : 'status=matched'));
        const failedPixel = success && current.find((line) => line.includes(pixelSummaryMarker)
          && line.includes(`run_id=${sceneRunID}`)
          && !line.includes(target === 'android' ? 'status=captured' : 'status=matched'));
        if (failedPixel) throw new Error(`${fixtureCase.id}: 표면 pixel 검증 실패: ${failedPixel}`);
        return hasResult && hasFrames && hasDraw && hasPixelSummary ? current : null;
      }, `${fixtureCase.id}: runtime/draw/pixel 결과`, target === 'android' ? 90_000 : 150_000);
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-runtime.log`),
        `${lines.join('\n')}\n`);

      const runtime = compareRuntimeScene(fixtureCase, lines, sceneRunID);
      const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.id}-${target}.png`);
      const screenshot = target === 'android'
        ? adb(['exec-out', 'screencap', '-p'], {
          encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
        }).stdout
        : null;
      let screenshotBytes = screenshot;
      if (target === 'ios') {
        const capture = simctl(['io', serialOrUDID, 'screenshot', screenshotPath], { timeout: 30_000 });
        await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-screenshot.log`),
          `${capture.stdout}${capture.stderr}`);
        screenshotBytes = await readFile(screenshotPath);
      }
      assert.deepEqual([...screenshotBytes.subarray(0, 8)],
        [137, 80, 78, 71, 13, 10, 26, 10], `${fixtureCase.id}: screenshot PNG signature`);
      if (target === 'android') await writeFile(screenshotPath, screenshotBytes);
      runtime.screenshot = `${fixtureCase.id}-${target}.png`;
      runtime.screenshotSha256 = sha256(screenshotBytes);
      runtime.runID = sceneRunID;
      runtime.processId = pid;

      if (success) {
        let pixelComparison;
        if (target === 'android') {
          const captureLine = lines.find((line) => line.includes(
            'SPINON_C111CFM_GRID_PIXEL_MATRIX_CAPTURE status=ok')
            && line.includes(`run_id=${sceneRunID}`));
          assert.ok(captureLine, `${fixtureCase.id}: PixelCopy success marker`);
          const devicePngPath = captureLine.match(/\bpng=(\S+)/)?.[1];
          assert.ok(devicePngPath, `${fixtureCase.id}: PixelCopy PNG path`);
          const devicePng = adb(['exec-out', 'run-as', packageName, 'cat', devicePngPath], {
            encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
          }).stdout;
          pixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
            scene: { id: fixtureCase.id, samples },
            runID: sceneRunID,
            pngBytes: devicePng,
            viewportCssPx: fixtureCase.viewportCssPx,
            markerPrefix: 'C111CFM',
            cacheDirectory: 'c111-root-html-column-flow-mixed-cross-axis-surface-pixel-matrix',
          });
          assert.equal(pixelComparison.density, androidDensityFromWmOutput(device.density),
            `${fixtureCase.id}: PixelCopy density가 독립 ADB wm density와 다릅니다.`);
          pixelComparison.environmentMapping = compareC11AndroidRootHtmlEnvironment(lines.join('\n'), {
            viewportCssPx: fixtureCase.viewportCssPx,
            density: pixelComparison.density,
            surfaceScale: pixelComparison.scale,
          });
          const surfacePng = `${fixtureCase.id}-surface-pixels.png`;
          await writeFile(resolve(evidenceDirectory, surfacePng), devicePng);
          pixelComparison.png = surfacePng;
        } else {
          pixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
            fixture: {
              fixtureId: fixtureCase.id,
              sceneId: fixtureCase.id,
              runtime: { viewportCssPx: fixtureCase.viewportCssPx },
              samples,
            },
            runID: sceneRunID,
            markerPrefix: 'C111CFM',
          });
        }
        assert.equal(pixelComparison.maximumChannelError, 0,
          `${fixtureCase.id}: exact surface pixel matrix`);
        const pixelFile = `${fixtureCase.id}-pixel-comparison.json`;
        await writeFile(resolve(evidenceDirectory, pixelFile),
          `${JSON.stringify(pixelComparison, null, 2)}\n`);
        runtime.surfacePixelComparison = pixelComparison;
        runtime.pixelComparison = pixelFile;
        pixelObservations.push({ ...pixelComparison, scene: fixtureCase.id });
      } else {
        const nodeId = runtime.diagnostic.observedNodeId;
        rejectionNodeIDs.push(nodeId);
        if (fixtureCase.expectedDiagnostic.nodeId !== null) {
          assert.equal(nodeId, fixtureCase.expectedDiagnostic.nodeId,
            `${fixtureCase.id}: pinned runtime diagnostic NodeId`);
        }
      }
      observations.push(runtime);
      process.stdout.write(`${fixtureCase.id}: ${runtime.outcome}${success
        ? ` · geometry ${runtime.maximumAbsoluteErrorCssPx} CSS px · pixel ${runtime.surfacePixelComparison.maximumChannelError}`
        : ` · ${runtime.diagnostic.code}/${runtime.diagnostic.property}/node:${runtime.diagnostic.observedNodeId}`
      }\n`);
    }
  } finally {
    if (target === 'android') stopAndroidApp(adb);
    else simctl(['terminate', serialOrUDID, packageName], { allowFailure: true, timeout: 30_000 });
  }

  const successfulScenes = observations.filter(({ outcome }) => outcome === 'matched').length;
  const rejectedScenes = observations.filter(({ outcome }) => outcome === 'expected-rejection').length;
  const participatingItems = observations.reduce((total, scene) =>
    total + (scene.participatingItemCount ?? 0), 0);
  const pixelSampleCount = pixelObservations.reduce((total, scene) => total + scene.expectedSamples, 0);
  assert.equal(successfulScenes, 8);
  assert.equal(rejectedScenes, 1);
  assert.equal(participatingItems, 31);
  assert.equal(pixelObservations.length, 8);
  assert.equal(pixelSampleCount, 31);
  assert.equal(new Set(rejectionNodeIDs).size, 1,
    'Android/iOS rejection paths do not report the same runtime node ID.');
  const summary = {
    schema: 'spinon-c11-root-html-column-flow-mixed-cross-axis-mobile-runtime-matrix/v1',
    status: 'matched',
    target: serialOrUDID,
    runID,
    platform: device,
    appSha256,
    inputs: {
      matrix: sha256(matrixBytes),
      ...matrix.inputs,
    },
    summary: {
      scenes: observations.length,
      successfulScenes,
      rejectedScenes,
      participatingItems,
      pixelScenes: pixelObservations.length,
      pixelSamples: pixelSampleCount,
      maximumAbsoluteErrorCssPx: Math.max(0,
        ...observations.map(({ maximumAbsoluteErrorCssPx = 0 }) => maximumAbsoluteErrorCssPx)),
      maximumChannelError: Math.max(0,
        ...pixelObservations.map(({ maximumChannelError }) => maximumChannelError)),
      rejectionRuntimeNodeId: rejectionNodeIDs[0],
      geometryToleranceCssPx: 0,
      freshProcessPerScene: true,
      screenshotBytesAreNotPixelParityEvidence: true,
    },
    observations,
    limitations: [
      '이 행렬은 고정 Root HTML column-flow 혼합 placement fixture의 geometry와 31개 색상 표본에 한정되며 전체 CSS Grid 지원 또는 WPT 적합성을 뜻하지 않는다.',
      ...(target === 'android'
        ? ['Android 결과는 API 37 ARM64 AVD의 SurfaceView PixelCopy이며 실기기·hardware GPU 근거가 아니다.']
        : ['iOS 결과는 iPhone 17 Pro iOS 26.2 Simulator의 Metal drawable 직접 readback이며 OS compositor 이후 화면 근거가 아니다.']),
      'screenshots는 실행 화면 보조 자료이며 pixel parity는 실제 PixelCopy/Metal readback으로 판정했다.',
    ],
  };
  await writeFile(resolve(evidenceDirectory, 'comparison.json'),
    `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 Root HTML 혼합 column-flow 시뮬레이터 행렬', '',
    `- 실행 ID: \`${runID}\``, `- 대상: \`${serialOrUDID}\``,
    `- 결과: 9 scenes · 정상 ${successfulScenes}/8 · 기대 거부 ${rejectedScenes}/1 · participating item ${participatingItems}/31`,
    `- geometry 최대 오차: ${summary.summary.maximumAbsoluteErrorCssPx} CSS px (허용 0 CSS px)`,
    `- surface pixel: ${pixelObservations.length} scenes · ${pixelSampleCount}/31 exact · 최대 채널 오차 ${summary.summary.maximumChannelError}`,
    `- 거부 진단 runtime NodeId: ${rejectionNodeIDs[0]}`,
    `- 앱 SHA-256: \`${appSha256}\``, '',
    '장면별 로그·PNG·PixelCopy/Metal 비교 결과는 [comparison.json](comparison.json)과 같은 디렉터리의 artifact에 있다.',
  ].join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID,
    platform: device, summary: summary.summary, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
