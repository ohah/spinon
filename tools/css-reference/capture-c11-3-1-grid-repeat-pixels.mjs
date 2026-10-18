import assert from 'node:assert/strict';
import { randomUUID, createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c11/grid-template-repeat-c1131-pixel-samples-v1.json';
const fixture = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
const target = process.argv[2];
const serial = process.argv[3] ?? 'emulator-5554';
assert.ok(['android-avd', 'android-physical', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-3-1-grid-repeat-pixels.mjs android-avd|android-physical|ios-simulator [Android serial]');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const simulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const runID = `c1131-${target}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-3-1-grid-repeat-pixels-${date}`, `${target}-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const iosBinaryPath = resolve(iosAppPath, 'SpinonBootstrap');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

function pixelPayload(encoding) {
  return Buffer.from(JSON.stringify(fixture.samples)).toString(encoding);
}

async function saveLog(path, output) {
  await writeFile(resolve(evidenceDirectory, path), `${output.stdout ?? ''}${output.stderr ?? ''}`);
}

function verifyChromeOracle() {
  const referenceBytes = Buffer.from(awaitRead(fixture.oracle.referencePath));
  const screenshotBytes = Buffer.from(awaitRead(fixture.oracle.screenshotPath));
  const comparison = validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes });
  return { referenceBytes, screenshotBytes, comparison };
}

function awaitRead(path) {
  // These small, pinned reference files are synchronously available through the cached inputs below.
  return chromeInputs.get(path);
}

const chromeInputs = new Map([
  [fixture.oracle.referencePath, await readFile(resolve(repositoryRoot, fixture.oracle.referencePath))],
  [fixture.oracle.screenshotPath, await readFile(resolve(repositoryRoot, fixture.oracle.screenshotPath))],
]);

function currentAndroidDevice() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(devices.length, 1, `Android 대상이 연결되어 있지 않습니다: ${serial}`);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const qemu = adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim() === '1';
  assert.equal(qemu, target === 'android-avd', '요청한 Android 대상과 실제 장치 종류가 다릅니다.');
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  assert.ok(apiLevel >= 36, `Android API ${apiLevel}은 고정 C11 검증 기준보다 낮습니다.`);
  if (target === 'android-avd') assert.equal(apiLevel, 37, '고정 Android API 37 AVD가 아닙니다.');
  return {
    adb,
    metadata: {
      serial,
      qemu,
      apiLevel,
      model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
      renderer: adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
        .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음',
    },
  };
}

function currentAndroidProcessLog(adb, pid) {
  const all = adb(['shell', 'logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return all.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

async function runAndroid() {
  const { adb, metadata } = currentAndroidDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  let build;
  try {
    build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
      'bun', 'run', 'build:android'], { timeout: 900_000 });
  } catch (error) {
    if (error.result) await saveLog('build.log', error.result);
    throw error;
  }
  await saveLog('build.log', build);
  const apkBytes = await readFile(apkPath);
  const installArtifactPath = 'platforms/android/app/build/outputs/apk/debug/app-debug.apk';
  const installArtifactSha256 = sha256(apkBytes);
  await saveLog('install.log', adb(['install', '-r', apkPath]));
  await saveLog('force-stop.log', adb(['shell', 'am', 'force-stop', packageName]));
  await saveLog('launch.log', adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', component,
    '--ez', 'spinon_c1131_grid_repeat', 'true',
    '--es', 'spinon_c1131_grid_repeat_scene', fixture.sceneId,
    '--es', 'spinon_c1131_grid_repeat_run_id', runID,
    '--es', 'spinon_c11_grid_case', fixture.sceneId,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', pixelPayload('base64url')]));

  const deadline = Date.now() + 150_000;
  let pid = null;
  let log = '';
  while (Date.now() < deadline) {
    pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]) || null;
    if (pid) log = currentAndroidProcessLog(adb, pid);
    if (log.includes(`SPINON_C1131_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`)
        || log.includes(`SPINON_C1131_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`)
        || log.includes(`SPINON_C1131_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${runID}`)
        || log.includes(`SPINON_C1131_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
        || log.includes('SPINON_C1131_GRID_ARGUMENT_ERROR')) break;
    sleep(300);
  }
  await writeFile(resolve(evidenceDirectory, 'runtime.log'), `${log}\n`);
  assert.ok(pid && log.includes(`SPINON_C1131_GRID_SUMMARY run_id=${runID} scene_id=${fixture.sceneId} status=0 layout=ready boxes=4`),
    '현재 Android process의 C11.3.1 runtime layout summary가 없습니다.');
  assert.ok(log.includes('SPINON_C0410_DRAW') && log.includes('presented boxes=4'),
    'Android WGPU draw/present marker가 없습니다.');
  assert.ok(log.includes(`SPINON_C1131_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`),
    'Android PixelCopy 표본 비교가 matched가 아닙니다.');
  const capture = log.split(/\r?\n/).find((line) =>
    line.includes('SPINON_C1131_GRID_PIXEL_MATRIX_CAPTURE status=ok') && line.includes(`run_id=${runID}`));
  assert.ok(capture, '현재 실행의 PixelCopy capture marker가 없습니다.');
  const appPng = capture.match(/\bpng=(\S+)/)?.[1];
  assert.ok(appPng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1131-grid-surface-pixel-matrix/'),
    'PixelCopy PNG 경로가 C11.3.1 앱 캐시 경로와 다릅니다.');
  const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
    `cache/c1131-grid-surface-pixel-matrix/${appPng.split('/').at(-1)}`], { encoding: null }).stdout;
  const comparison = compareC11AndroidPixelCopyScene(log, {
    scene: { id: fixture.sceneId, samples: fixture.samples },
    runID,
    pngBytes,
    viewportCssPx: fixture.runtime.viewportCssPx,
    markerPrefix: 'C1131',
    cacheDirectory: 'c1131-grid-surface-pixel-matrix',
  });
  await writeFile(resolve(evidenceDirectory, 'pixelcopy.png'), pngBytes);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
  await writeFile(resolve(evidenceDirectory, 'android-screen.png'), screenshot);

  const negativeRunID = `${runID}-bad-samples`;
  const invalidSamples = fixture.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  await saveLog('pixel-allowlist-negative-force-stop.log',
    adb(['shell', 'am', 'force-stop', packageName]));
  await saveLog('pixel-allowlist-negative-launch.log', adb(['shell', 'am', 'start', '-S',
    '-f', '0x10008000', '-n', component,
    '--ez', 'spinon_c1131_grid_repeat', 'true',
    '--es', 'spinon_c1131_grid_repeat_scene', fixture.sceneId,
    '--es', 'spinon_c1131_grid_repeat_run_id', negativeRunID,
    '--es', 'spinon_c11_grid_case', fixture.sceneId,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', negativeRunID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples',
    Buffer.from(JSON.stringify(invalidSamples)).toString('base64url')]));
  const negativeDeadline = Date.now() + 30_000;
  let negativePID = null;
  let negativeLog = '';
  while (Date.now() < negativeDeadline) {
    negativePID = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]) || null;
    if (negativePID) negativeLog = currentAndroidProcessLog(adb, negativePID);
    if (negativeLog.includes(`SPINON_C1131_GRID_PIXEL_MATRIX status=error run_id=${negativeRunID}`)
        || negativeLog.includes(`SPINON_C1131_GRID_PIXEL_MATRIX_BEGIN run_id=${negativeRunID}`)) break;
    sleep(300);
  }
  const negativeRejected = negativeLog.includes(
    `SPINON_C1131_GRID_PIXEL_MATRIX status=error run_id=${negativeRunID}`);
  const negativePixelCopyStarted = negativeLog.includes(
    `SPINON_C1131_GRID_PIXEL_MATRIX_BEGIN run_id=${negativeRunID}`);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.log'),
    `${negativeLog}\n`);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.json'),
    `${JSON.stringify({ runID: negativeRunID, processId: negativePID,
      invalidSample: 'item-blue xCssPx + 1', rejected: negativeRejected,
      pixelCopyStarted: negativePixelCopyStarted }, null, 2)}\n`);
  assert.ok(negativeRejected,
    'Android가 잘못된 C11.3.1 표본 좌표를 거부하지 않았습니다.');
  assert.equal(negativePixelCopyStarted, false,
    'Android가 잘못된 C11.3.1 표본 요청에서 PixelCopy를 시작했습니다.');
  return {
    device: metadata,
    installArtifactPath,
    installArtifactSha256,
    comparison,
    screenshotSha256: sha256(screenshot),
    sampleAllowlistNegativeControl: {
      status: 'rejected', runID: negativeRunID, processId: negativePID,
      invalidSample: 'item-blue xCssPx + 1', pixelCopyStarted: false,
    },
  };
}

function bootedSimulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const booted = Object.entries(devices).flatMap(([runtime, values]) =>
    values.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted iOS Simulator가 하나여야 합니다.');
  assert.equal(booted[0].udid, simulatorUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.match(booted[0].runtime, /iOS-26-2$/, '고정 iOS 26.2 runtime이 아닙니다.');
  return booted[0];
}

function iosProcessLog(udid, pid) {
  const output = command('xcrun', ['simctl', 'spawn', udid, 'log', 'show', '--last', '5m',
    '--style', 'syslog', '--predicate', 'process == "SpinonBootstrap"'],
  { timeout: 45_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

function iosProcessID(udid) {
  const output = command('xcrun', ['simctl', 'spawn', udid, 'launchctl', 'list']).stdout;
  const line = output.split(/\r?\n/).find((value) =>
    value.includes(`UIKitApplication:${packageName}[`));
  const pid = line?.match(/^(\d+)\s/)?.[1];
  return pid ? Number(pid) : null;
}

async function runIOS() {
  const simulator = bootedSimulator();
  let build;
  try {
    build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
      'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1', 'bun', 'run', 'build:ios-sim'],
    { timeout: 900_000 });
  } catch (error) {
    if (error.result) await saveLog('build.log', error.result);
    throw error;
  }
  await saveLog('build.log', build);
  const buildOutput = `${build.stdout}${build.stderr}`;
  const buildFlagLine = buildOutput.match(/^SPINON_IOS_SIM_BUILD_FLAGS (.*)$/m)?.[1];
  assert.ok(buildFlagLine, 'iOS simulator build flags were not reported.');
  const buildFlags = Object.fromEntries(buildFlagLine.split(/\s+/)
    .map((flag) => flag.split('=')));
  assert.equal(buildFlags.c04_runtime_gpu, '1',
    'iOS simulator build did not enable the runtime GPU fixture.');
  assert.equal(buildFlags.c11_grid_metal_surface_pixels, '1',
    'iOS simulator build did not enable C11 Metal surface readback.');
  assert.match(buildOutput,
    /SPINON_IOS_NATIVE_FEATURES ffi=[^\n]*c11-grid-metal-surface-pixels-test[^\n]*c11_grid_cpp=-DSPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1/,
    'iOS native FFI/C++ pixel feature flags are missing.');
  const appInfo = resolve(iosAppPath, 'Info.plist');
  const executableName = command('/usr/libexec/PlistBuddy', ['-c', 'Print CFBundleExecutable', appInfo]).stdout.trim();
  const installArtifactPath = `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/${executableName}`;
  const installArtifactSha256 = sha256(await readFile(resolve(iosAppPath, executableName)));
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulator.udid, iosAppPath]));
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulator.udid,
    packageName, '--spinon-c1131-grid-repeat',
    `--spinon-c1131-grid-repeat-scene=${fixture.sceneId}`,
    `--spinon-c1131-grid-repeat-run-id=${runID}`,
    `--spinon-c11-grid-case=${fixture.sceneId}`,
    `--spinon-c11-grid-metal-surface-pixels=${runID}`,
    `--spinon-c11-grid-metal-surface-samples=${pixelPayload('base64')}`], { timeout: 90_000 });
  await saveLog('launch.log', launch);
  const launchPID = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1] ?? iosProcessID(simulator.udid));
  assert.ok(Number.isInteger(launchPID) && launchPID > 0, 'iOS Simulator launch process ID를 확인하지 못했습니다.');

  const deadline = Date.now() + 150_000;
  let lines = [];
  let log = '';
  while (Date.now() < deadline) {
    lines = iosProcessLog(simulator.udid, launchPID);
    log = lines.join('\n');
    if (log.includes(`SPINON_C1131_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`)
        || log.includes(`SPINON_C1131_METAL_PIXEL_SUMMARY status=mismatch run_id=${runID}`)
        || log.includes(`SPINON_C1131_METAL_PIXEL_ERROR run_id=${runID}`)
        || log.includes('SPINON_C1131_GRID_ARGUMENT_ERROR')) break;
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 300));
  }
  await writeFile(resolve(evidenceDirectory, 'runtime.log'), `${log}\n`);
  assert.ok(log.includes(`SPINON_C1131_GRID_EVAL_RUN run_id=${runID} scene_id=${fixture.sceneId} status=0 layout=ready boxes=4`),
    '현재 iOS process의 C11.3.1 runtime layout summary가 없습니다.');
  assert.ok(log.includes('SPINON_C0410_DRAW') && log.includes('presented boxes=4'),
    'iOS WGPU draw/present marker가 없습니다.');
  const comparison = compareC11IOSMetalPixelLogs(log, {
    fixture: { ...fixture, sceneId: fixture.sceneId },
    runID,
    markerPrefix: 'C1131',
  });
  await writeFile(resolve(evidenceDirectory, 'metal-surface-comparison.json'),
    `${JSON.stringify(comparison, null, 2)}\n`);
  const screenshotPath = resolve(evidenceDirectory, 'ios-screen.png');
  const screenshotResult = command('xcrun', ['simctl', 'io', simulator.udid, 'screenshot', screenshotPath]);
  await saveLog('screenshot.log', screenshotResult);
  const screenshot = await readFile(screenshotPath);
  assert.equal(screenshot.toString('hex', 0, 8), '89504e470d0a1a0a', 'iOS Simulator screenshot PNG가 아닙니다.');
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });

  const negativeRunID = `${runID}-bad-samples`;
  const invalidSamples = fixture.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  const invalidLaunch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
    simulator.udid, packageName,
    '--spinon-c1131-grid-repeat',
    `--spinon-c1131-grid-repeat-scene=${fixture.sceneId}`,
    `--spinon-c1131-grid-repeat-run-id=${negativeRunID}`,
    `--spinon-c11-grid-case=${fixture.sceneId}`,
    `--spinon-c11-grid-metal-surface-pixels=${negativeRunID}`,
    `--spinon-c11-grid-metal-surface-samples=${Buffer.from(JSON.stringify(invalidSamples)).toString('base64')}`],
  { timeout: 90_000 });
  await saveLog('pixel-allowlist-negative-launch.log', invalidLaunch);
  const negativePID = Number(invalidLaunch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]
    ?? iosProcessID(simulator.udid));
  assert.ok(Number.isInteger(negativePID) && negativePID > 0,
    'iOS Metal sample negative control process ID를 확인하지 못했습니다.');
  const negativeDeadline = Date.now() + 30_000;
  let negativeLog = '';
  while (Date.now() < negativeDeadline) {
    negativeLog = iosProcessLog(simulator.udid, negativePID).join('\n');
    if (negativeLog.includes(`SPINON_C1131_METAL_PIXEL_ERROR run_id=${negativeRunID}`)) break;
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 300));
  }
  const negativeRejected = negativeLog.includes(
    `SPINON_C1131_METAL_PIXEL_ERROR run_id=${negativeRunID}`);
  const negativeMetalReadbackStarted = negativeLog.includes(
    `SPINON_C1131_METAL_PIXEL_BEGIN run_id=${negativeRunID}`);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.log'),
    `${negativeLog}\n`);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.json'),
    `${JSON.stringify({ runID: negativeRunID, processId: negativePID,
      invalidSample: 'item-blue xCssPx + 1', rejected: negativeRejected,
      metalReadbackStarted: negativeMetalReadbackStarted }, null, 2)}\n`);
  assert.ok(negativeRejected,
    'iOS가 잘못된 C11.3.1 sample 좌표를 거부하지 않았습니다.');
  assert.equal(negativeMetalReadbackStarted, false,
    'iOS가 잘못된 C11.3.1 sample 요청에서 Metal readback을 시작했습니다.');
  command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  return {
    device: { name: simulator.name, udid: simulator.udid, runtime: simulator.runtime },
    installArtifactPath,
    installArtifactSha256,
    comparison,
    screenshotSha256: sha256(screenshot),
    sampleAllowlistNegativeControl: {
      status: 'rejected', runID: negativeRunID, processId: negativePID,
      invalidSample: 'item-blue xCssPx + 1', metalReadbackStarted: false,
    },
  };
}

await mkdir(evidenceDirectory, { recursive: true });
const { referenceBytes, screenshotBytes, comparison: chromeComparison } = verifyChromeOracle();
const platformResult = target === 'ios-simulator' ? await runIOS() : await runAndroid();
const result = {
  schema: 'spinon-c11-3-1-grid-repeat-pixels-result/v1',
  status: 'matched',
  runID,
  target,
  fixture: { path: fixturePath, sha256: sha256(await readFile(resolve(repositoryRoot, fixturePath))),
    sampleCount: fixture.samples.length },
  oracle: {
    referenceId: fixture.oracle.referenceId,
    referenceSha256: sha256(referenceBytes),
    screenshotSha256: sha256(screenshotBytes),
    product: fixture.oracle.product,
    scene: fixture.sceneId,
    deviceScaleFactor: fixture.oracle.deviceScaleFactor,
    chromeOracleValidation: chromeComparison,
  },
  platform: platformResult,
  interpretation: target === 'ios-simulator'
    ? 'Metal drawable 표면의 compositor 제출 전 RGBA를 고정 Chrome 표본과 비교했습니다. 화면 캡처는 UI 상태 증거이며 픽셀 비교 입력이 아닙니다.'
    : platformResult.device.qemu
      ? 'Android API 37 AVD의 PixelCopy 결과입니다. AVD가 보고한 renderer는 하드웨어 실기기 GPU 성능 증거가 아닙니다.'
      : '연결된 Android 실기기의 PixelCopy 결과입니다. 이 캡처만으로 다른 기기·GPU의 동등성을 주장하지 않습니다.',
};
await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
await writeFile(resolve(evidenceDirectory, 'README.md'), [
  '# C11.3.1 Grid repeat 표면 픽셀 검증',
  '',
  `- 실행 ID: \`${runID}\``,
  `- 대상: \`${target}\``,
  `- 고정 Chrome 기준: ${fixture.oracle.product}, DPR ${fixture.oracle.deviceScaleFactor}, \`${fixture.sceneId}\``,
  `- 비교 표본: ${fixture.samples.length}개. Chrome screenshot의 PNG와 표본 RGBA를 fixture hash와 함께 고정했습니다.`,
  target === 'ios-simulator'
    ? '- 비교 범위: WGPU Metal drawable의 compositor 제출 전 readback입니다. iOS 화면 캡처와 OS compositor 이후 픽셀을 뜻하지 않습니다.'
    : platformResult.device.qemu
      ? '- 비교 범위: Android AVD SurfaceView의 PixelCopy입니다. AVD 렌더러가 소프트웨어이면 실기기 GPU 결과로 해석하지 않습니다.'
      : '- 비교 범위: Android 실기기 SurfaceView의 PixelCopy입니다. 다른 기기로 일반화하지 않습니다.',
  '',
  '## 결과',
  '',
  `- 상태: \`${result.status}\``,
  `- 일치한 표본: ${platformResult.comparison.matchedSamples}/${platformResult.comparison.expectedSamples}`,
  `- 최대 채널 오차: ${platformResult.comparison.maximumChannelError}`,
  `- 설치 검증 산출물: \`${platformResult.installArtifactPath}\` (SHA-256 \`${platformResult.installArtifactSha256}\`)`,
  '',
  '## 파일',
  '',
  '- `result.json`: 장치 정보, 고정 oracle, 실행 hash 및 비교 결과',
  '- `runtime.log`: 해당 앱 process의 run ID 로그',
  target === 'ios-simulator' ? '- `metal-surface-comparison.json`: Metal 표본별 비교' : '- `pixelcopy.png`: PixelCopy 원본 표면',
  target === 'ios-simulator' ? '- `ios-screen.png`: Simulator 화면 상태' : '- `android-screen.png`: Android 화면 상태',
  '',
].join('\n'));
process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
