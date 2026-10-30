import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
const serial = process.argv[3] ?? 'emulator-5554';
const simulatorUDID = process.argv[3] ?? 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const packageName = 'dev.spinon.bootstrap';
const androidApk = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosApp = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const evidenceRoot = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-grid-text-baseline-runtime-2026-10-30');
const runPrefix = `c114-text-baseline-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(evidenceRoot, target, runPrefix);

const scenes = [
  'grid-text-first-baseline-align-items',
  'grid-text-first-baseline-start-control',
  'grid-text-first-baseline-align-self',
  'grid-text-first-baseline-reversed',
  'grid-text-last-baseline-align-items-multiline',
  'grid-text-last-baseline-align-self-multiline',
  'grid-text-last-baseline-reversed-multiline',
  'grid-text-last-baseline-first-control',
  'grid-text-last-baseline-start-control',
].map((id) => {
  const last = id.startsWith('grid-text-last-baseline-');
  const reversed = id.endsWith('-reversed') || id.endsWith('-reversed-multiline');
  const firstControl = id.endsWith('-first-control');
  const start = id.endsWith('-start-control');
  const multilineLast = last && id.endsWith('-multiline');
  const scene = {
    id, rootId: `c114-${id}`, boxCount: 3, last, reversed, firstControl,
    start, multilineLast,
  };
  const small = {
    node: 2,
    x: last ? 0 : reversed ? 48 : 0,
    y: last ? (firstControl ? 8 : 0) : start ? 0 : 8,
    width: last ? 25 : 20,
    height: last ? (firstControl ? 10 : 20) : 10,
    fontSize: 10,
    text: last && !firstControl ? 'XX XX' : 'XX',
    lineTops: last && !firstControl ? [0, 10] : [0],
    baselines: last && !firstControl ? [8, 18] : [8],
  };
  const large = {
    node: 4,
    x: last ? 33 : reversed ? 0 : 28,
    y: last && multilineLast ? 2 : 0,
    width: last ? 40 : 40,
    height: 20,
    fontSize: 20,
    text: 'XX',
    lineTops: [0],
    baselines: [16],
  };
  scene.expectedItems = reversed ? [large, small] : [small, large];
  scene.expectedRootFrame = last
    ? `width=73,height=${multilineLast ? 22 : 20}` : 'width=68,height=20';
  scene.expectedItemFrame = reversed
    ? 'width=40,height=20'
    : `width=${small.width},height=${small.height}`;
  const rootFrame = `0:node=1,x=0,y=0,width=${last ? 73 : 68},height=${last
    ? (multilineLast ? 22 : 20) : 20}`;
  const nodeFrames = scene.expectedItems.map((item, index) =>
    `${index + 1}:node=${item.node},x=${item.x},y=${item.y},width=${item.width},height=${item.height}`);
  scene.expectedNodeFrames = `node_frames_css_px=[${[rootFrame, ...nodeFrames].join(';')}]`;
  return scene;
});

const sourcePaths = [
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_text_baseline.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
  'crates/spinon-layout/src/calc_tree.rs',
  'crates/spinon-layout/src/calc_tree/intrinsic_measurements.rs',
  'crates/spinon-layout/src/calc_tree/text.rs',
  'crates/spinon-layout/src/c15_text_layout_tests.rs',
  'crates/spinon-layout/src/c15_text_layout_tests/grid_text_baseline.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/grid_text_baseline.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/grid_text_baseline.rs',
  'crates/spinon-style-to-layout/src/lib.rs',
  'crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests.rs',
  'crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/first_baseline_admission.rs',
  'crates/spinon-style-to-layout/src/projection.rs',
  'crates/spinon-style-to-layout/src/projection/style_values.rs',
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  'tests/fixtures/css/c11/grid-last-baseline-multiline-direct-text.html',
  'tools/css-reference/capture-c11-4-grid-text-baseline-simulators.mjs',
];

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} failed: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function saveLog(path, result) {
  await writeFile(resolve(evidenceDirectory, path),
    `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function pixelSamples(scene) {
  const { id, rootId, last, reversed, start, firstControl } = scene;
  if (last) {
    return [
      { id: `${id}-grid-gap`, nodeId: rootId, xCssPx: 28, yCssPx: 5, rgba: [238, 243, 248, 255] },
      { id: `${id}-small-item`, nodeId: rootId,
        xCssPx: 10, yCssPx: firstControl ? 13 : 15, rgba: [229, 72, 77, 255] },
      { id: `${id}-large-item`, nodeId: rootId, xCssPx: 48, yCssPx: 10, rgba: [58, 134, 255, 255] },
    ];
  }
  return [
    { id: `${id}-grid-gap`, nodeId: rootId, xCssPx: reversed ? 44 : 24, yCssPx: 5, rgba: [238, 243, 248, 255] },
    { id: `${id}-small-item`, nodeId: rootId,
      xCssPx: reversed ? 58 : 10, yCssPx: start ? 5 : 13, rgba: [229, 72, 77, 255] },
    { id: `${id}-large-item`, nodeId: rootId,
      xCssPx: reversed ? 20 : 48, yCssPx: 10, rgba: [58, 134, 255, 255] },
  ];
}

function traceJson(traces, scene, runID) {
  const completeTrace = traces.parts.map((part) => part.payload).join('');
  const marker = 'grid_track_sizing_diagnostics=';
  const markerIndex = completeTrace.indexOf(marker);
  assert.ok(markerIndex >= 0, `${scene.id}: C11.4 JSON trace marker is missing`);
  return JSON.parse(completeTrace.slice(markerIndex + marker.length));
}

function validateBaselineTrace(report, scene) {
  const expected = scene.expectedItems;
  const frames = report.gridItemFrames;
  assert.ok(Array.isArray(frames) && frames.length === 2, `${scene.id}: item frames missing`);
  for (const item of expected) {
    const frame = frames.find(({ node }) => node === item.node);
    assert.ok(frame, `${scene.id}: Grid item ${item.node} frame missing`);
    for (const key of ['x', 'y', 'width', 'height']) {
      assert.equal(frame[key], item[key], `${scene.id}: item ${item.node} ${key}`);
    }
  }

  assert.equal(report.layoutInputRevision.source.startsWith('HostDocument'), true);
  const layouts = report.gridItemTextLayouts;
  assert.ok(Array.isArray(layouts) && layouts.length === 2, `${scene.id}: text layouts missing`);
  for (const item of expected) {
    const layout = layouts.find(({ node }) => node === item.node);
    assert.ok(layout, `${scene.id}: text layout for ${item.node} missing`);
    assert.equal(layout.text, item.text);
    assert.equal(layout.fontRegistryRevision, 1);
    assert.equal(layout.paragraphStyle.fontFamily, 'Ahem');
    assert.equal(layout.paragraphStyle.fontSize, item.fontSize);
    assert.equal(layout.paragraphStyle.lineHeight, item.fontSize);
    assert.equal(layout.lines.length, item.baselines.length);
    for (const [index, line] of layout.lines.entries()) {
      assert.equal(line.lineTop, item.lineTops[index]);
      assert.equal(line.lineHeight, item.fontSize);
      assert.equal(line.baseline, item.baselines[index]);
    }
  }

  const trace = report.trackSizingTraces.find(({ grid }) => grid === 1);
  assert.ok(trace, `${scene.id}: Grid track-sizing trace missing`);
  const finalTrackSizes = (axis) => {
    const pass = trace.passes.find((candidate) => candidate.axis === axis);
    assert.ok(pass && pass.stages.length > 0, `${scene.id}: ${axis} pass missing`);
    return pass.stages.at(-1).tracks
      .filter((track) => !track.gutter && !track.collapsed)
      .map((track) => track.baseSize);
  };
  assert.deepEqual(finalTrackSizes('Columns'), scene.last ? [25, 40]
    : scene.reversed ? [40, 20] : [20, 40]);
  assert.deepEqual(finalTrackSizes('Rows'), [scene.multilineLast ? 22 : 20]);
}

function captureArguments(scene, runID, platform) {
  const samples = pixelSamples(scene);
  if (!samples) return [];
  if (platform === 'ios-simulator') {
    const encoded = Buffer.from(JSON.stringify(samples)).toString('base64');
    return [
      `--spinon-c11-grid-metal-surface-pixels=${runID}`,
      `--spinon-c11-grid-metal-surface-samples=${encoded}`,
    ];
  }
  const androidSamples = samples.map(({ id, nodeId, xCssPx, yCssPx }) => ({
    id, nodeId, xCssPx, yCssPx,
  }));
  const encoded = Buffer.from(JSON.stringify(androidSamples)).toString('base64url');
  return [
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', encoded,
  ];
}

function traceParts(log, runID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  const parts = log.split(/\r?\n/).flatMap((line) => {
    const markerOffset = line.indexOf(marker);
    if (markerOffset < 0) return [];
    const match = line.slice(markerOffset + marker.length)
      .match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
  const total = parts[0]?.total ?? 0;
  const unique = new Map(parts.map((part) => [part.index, part]));
  const complete = total > 0 && unique.size === total
    && [...unique.values()].every((part) => part.total === total
      && part.index >= 1 && part.index <= total);
  return { complete, total, parts: [...unique.values()].sort((a, b) => a.index - b.index) };
}

function validateLog(log, scene, runID, platform) {
  assert.ok(log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `),
    `${scene.id}: runtime summary missing\n${log}`);
  assert.ok(log.includes(`expected_root_frame=${scene.expectedRootFrame}`)
      && log.includes(`expected_item_frame=${scene.expectedItemFrame}`),
    `${scene.id}: root/item frame expectation differs\n${log}`);
  assert.ok(log.includes(scene.expectedNodeFrames),
    `${scene.id}: native node-frame assertion differs\n${log}`);
  assert.ok(log.includes(`layout=ready boxes=${scene.boxCount}`),
    `${scene.id}: layout or draw box count differs\n${log}`);
  assert.ok(log.includes(`SPINON_C0410_DRAW`) && log.includes(`presented boxes=${scene.boxCount}`),
    `${scene.id}: native GPU draw summary missing\n${log}`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`),
    `${scene.id}: native fixture assertion failed\n${log}`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ARGUMENT_ERROR`),
    `${scene.id}: route rejected arguments\n${log}`);
  const traces = traceParts(log, runID);
  assert.ok(traces.complete,
    `${scene.id}: track trace chunks incomplete (${traces.parts.length}/${traces.total})\n${log}`);
  validateBaselineTrace(traceJson(traces, scene, runID), scene);
  const expectedPixelSummary = platform === 'ios-simulator'
    ? `SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`
    : `SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`;
  if (pixelSamples(scene)) assert.ok(log.includes(expectedPixelSummary),
    `${scene.id}: fixed surface pixel samples did not match\n${log}`);
  return {
    id: scene.id,
    boxCount: scene.boxCount,
    traceParts: traces.parts.length,
    pixelSamples: pixelSamples(scene)?.length ?? 0,
  };
}

function androidDevice() {
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const listed = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(listed.length, 1, `Android emulator is not connected: ${serial}`);
  const metadata = {
    target: 'android-avd', serial,
    qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(),
    apiLevel: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
    abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
    model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
    fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    displaySize: adb(['shell', 'wm', 'size']).stdout.trim(),
    displayDensity: adb(['shell', 'wm', 'density']).stdout.trim(),
  };
  assert.equal(metadata.qemu, '1');
  assert.equal(metadata.apiLevel, 37);
  assert.equal(metadata.abi, 'arm64-v8a');
  return { adb, metadata };
}

function androidLog(adb, pid) {
  const output = adb(['logcat', '-d', '-T', '1000', '-v', 'threadtime', `--pid=${pid}`,
    'SpinonBootstrap:V', '*:S'], { timeout: 30_000 }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function iosLog(pid) {
  const predicate = 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C114_GRID" '
    + 'OR eventMessage CONTAINS[c] "SPINON_C114_METAL_PIXEL" '
    + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW" '
    + 'OR eventMessage CONTAINS[c] "node_frames_css_px=")';
  const output = command('xcrun', ['simctl', 'spawn', simulatorUDID,
    'log', 'show', '--last', '45s', '--style', 'syslog', '--predicate', predicate],
  { timeout: 45_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForAndroid(adb, scene, runID) {
  const deadline = Date.now() + 150_000;
  let log = '';
  while (Date.now() < deadline) {
    const pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]) || null;
    if (pid) log = androidLog(adb, pid);
    if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `)
        && traceParts(log, runID).complete
        && log.includes(`presented boxes=${scene.boxCount}`)
        && (!pixelSamples(scene)
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`))) {
      return log;
    }
    if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')
        || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
        || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`)) return log;
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 300);
  }
  throw new Error(`${scene.id}: Android run timed out\n${log}`);
}

function waitForIOS(scene, runID, pid) {
  const deadline = Date.now() + 150_000;
  let log = '';
  while (Date.now() < deadline) {
    log = iosLog(pid);
    if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `)
        && traceParts(log, runID).complete
        && log.includes(`presented boxes=${scene.boxCount}`)
        && (!pixelSamples(scene)
          || log.includes(`SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`))) {
      return log;
    }
    if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')
        || log.includes(`SPINON_C114_METAL_PIXEL_ERROR run_id=${runID}`)
        || log.includes(`SPINON_C114_METAL_PIXEL_SUMMARY status=mismatch run_id=${runID}`)) return log;
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 700);
  }
  throw new Error(`${scene.id}: iOS run timed out\n${log}`);
}

async function runAndroid() {
  const { adb, metadata } = androidDevice();
  const apk = await readFile(androidApk);
  const localApkSha256 = sha256(apk);
  await saveLog('install.log', adb(['install', '-r', androidApk]));
  const baseApk = adb(['shell', 'pm', 'path', packageName]).stdout
    .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim())
    .find((path) => path.endsWith('/base.apk'));
  assert.ok(baseApk, 'Installed base.apk path is missing');
  const installedApkSha256 = adb(['shell', 'sha256sum', baseApk]).stdout.trim().split(/\s+/)[0];
  assert.equal(installedApkSha256, localApkSha256);
  metadata.localApkSha256 = localApkSha256;
  metadata.installedApkSha256 = installedApkSha256;
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const observations = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const runID = `${runPrefix}-${index}`;
      const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n',
        `${packageName}/.MainActivity`, '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.id,
        '--es', 'spinon_c114_grid_run_id', runID,
        ...captureArguments(scene, runID, 'android-avd')]);
      await saveLog(`launch-${scene.id}.log`, launch);
      const log = waitForAndroid(adb, scene, runID);
      await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
      observations.push(validateLog(log, scene, runID, 'android-avd'));
      adb(['shell', 'cmd', 'statusbar', 'collapse'], { allowFailure: true });
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 300);
      const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
      await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
      console.log(`android-avd ${index + 1}/${scenes.length}: ${scene.id} passed`);
    }
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
  return observations;
}

async function runIOS() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator not found: ${simulatorUDID}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  const modelIdentifier = command('xcrun', ['simctl', 'getenv', simulatorUDID,
    'SIMULATOR_MODEL_IDENTIFIER']).stdout.trim();
  assert.equal(modelIdentifier, 'iPhone18,1');
  const executablePath = resolve(iosApp, 'SpinonBootstrap');
  const executableSha256 = sha256(await readFile(executablePath));
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulatorUDID, iosApp]));
  const installedAppPath = command('xcrun', ['simctl', 'get_app_container', simulatorUDID,
    packageName, 'app']).stdout.trim();
  const installedExecutableSha256 = sha256(await readFile(resolve(installedAppPath, 'SpinonBootstrap')));
  assert.equal(installedExecutableSha256, executableSha256);
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target: 'ios-simulator', udid: simulatorUDID, name: simulator[0].name,
    modelIdentifier, runtime: simulator[0].runtime, executableSha256,
    installedExecutableSha256,
  }, null, 2)}\n`);
  const observations = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const runID = `${runPrefix}-${index}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        simulatorUDID, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-run-id=${runID}`, `--spinon-c114-grid-scene=${scene.id}`,
        ...captureArguments(scene, runID, 'ios-simulator')]);
      await saveLog(`launch-${scene.id}.log`, launch);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: launch PID missing`);
      const log = waitForIOS(scene, runID, pid);
      await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
      observations.push(validateLog(log, scene, runID, 'ios-simulator'));
      await saveLog(`screenshot-${scene.id}.log`, command('xcrun', ['simctl', 'io',
        simulatorUDID, 'screenshot', resolve(evidenceDirectory, `${scene.id}.png`)]));
      console.log(`ios-simulator ${index + 1}/${scenes.length}: ${scene.id} passed`);
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', simulatorUDID, packageName], { allowFailure: true });
  }
  return observations;
}

assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-grid-text-baseline-simulators.mjs <android-avd|ios-simulator> [serial|UDID]');
await mkdir(evidenceDirectory, { recursive: true });
const sourceHead = command('git', ['rev-parse', 'HEAD']).stdout.trim();
const sourceDirty = command('git', ['status', '--short', '--', ...sourcePaths]).stdout.trim();
const observations = target === 'android-avd' ? await runAndroid() : await runIOS();
await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, sourceHead, sourceDirty,
  sceneCount: scenes.length, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
