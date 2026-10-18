import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { closeSync, mkdirSync, openSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const matrixPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/mobile-surface-pixel-matrix.json');
const matrix = JSON.parse(readFileSync(matrixPath, 'utf8'));
const packageName = 'dev.spinon.bootstrap';
const bundleID = packageName;
const component = `${packageName}/.MainActivity`;
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const target = process.argv[2];
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-mobile-surface-pixel-matrix.mjs android-avd|ios-simulator');

const runID = `c11-pixels-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-1-mobile-surface-pixel-matrix-${date}`, `${target}-${runID}`);
const androidApkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const iosAppBinaryPath = resolve(iosAppPath, 'SpinonBootstrap');

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
const readText = (path) => {
  try { return readFileSync(path, 'utf8'); } catch { return ''; }
};

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8', env = process.env } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, env, timeout, maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function bootedIOSSimulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const booted = Object.entries(devices).flatMap(([runtime, items]) =>
    items.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted Simulator가 하나여야 합니다.');
  assert.equal(booted[0].udid, expectedSimulatorUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(booted[0].name, 'iPhone 17 Pro');
  assert.match(booted[0].runtime, /iOS-26-2$/, '대상 iOS runtime이 26.2가 아닙니다.');
  return booted[0];
}

function connectedAndroidAVD() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => /^emulator-\d+\s+device\s/.test(line));
  assert.equal(devices.length, 1, 'Android AVD가 정확히 하나 연결되어 있어야 합니다.');
  const serial = devices[0].split(/\s+/)[0];
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  assert.equal(adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(), '1', '대상은 AVD여야 합니다.');
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  assert.equal(apiLevel, 37, '고정 Android API 37 AVD가 아닙니다.');
  return {
    serial,
    apiLevel,
    model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
    renderer: adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
      .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음',
    adb,
  };
}

function waitUntil(predicate, description, timeoutMilliseconds = 90_000) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    const value = predicate();
    if (value) return value;
    sleep(250);
  }
  throw new Error(`${description}: ${timeoutMilliseconds / 1000}초 대기 시간 초과`);
}

function samplePayload(scene, encoding) {
  return Buffer.from(JSON.stringify(scene.samples.map((sample) => ({
    id: sample.id,
    nodeId: sample.nodeId,
    xCssPx: sample.xCssPx,
    yCssPx: sample.yCssPx,
    rgba: sample.rgba,
  })))).toString(encoding);
}

function build(targetName) {
  const flags = targetName === 'android-avd'
    ? ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1']
    : ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1'];
  const build = command('mise', ['exec', '--', 'env', ...flags,
    'bun', 'run', targetName === 'android-avd' ? 'build:android' : 'build:ios-sim'],
  { timeout: 900_000 });
  const output = `${build.stdout}${build.stderr}`;
  assert.match(output, targetName === 'android-avd' ? /BUILD SUCCESSFUL/i : /\*\* BUILD SUCCEEDED \*\*/);
  writeFileSync(resolve(evidenceDirectory, `${targetName}-build.log`), output);
}

async function runAndroid() {
  const device = connectedAndroidAVD();
  build('android-avd');
  const apkBytes = readFileSync(androidApkPath);
  const apkSha256 = sha256(apkBytes);
  const install = device.adb(['install', '-r', androidApkPath], { timeout: 180_000 });
  writeFileSync(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);
  const observations = [];
  for (const scene of matrix.scenes) {
    const sceneRunID = `${runID}-${scene.index}`;
    device.adb(['shell', 'am', 'force-stop', packageName]);
    device.adb(['logcat', '-c']);
    const launch = device.adb(['shell', 'am', 'start', '-W', '-n', component,
      '--es', 'spinon_c11_grid_case', scene.id,
      '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
      '--es', 'spinon_c11_grid_pixel_matrix_samples', samplePayload(scene, 'base64url')]);
    writeFileSync(resolve(evidenceDirectory, `${scene.id}-launch.log`), `${launch.stdout}${launch.stderr}`);
    assert.match(launch.stdout, /Status:\s*ok/, `${scene.id}: Android Activity launch`);
    let log = '';
    log = waitUntil(() => {
      const current = device.adb(['logcat', '-d', '-v', 'threadtime', '-s', 'SpinonBootstrap:I']).stdout;
      if (current.includes(`SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY `)
          && current.includes(`run_id=${sceneRunID}`)) return current;
      if (current.includes(`SPINON_C111_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)) {
        throw new Error(`${scene.id}: 잘못된 Android 표본 입력\n${current}`);
      }
      return null;
    }, `${scene.id}: Android PixelCopy`);
    const pid = log.match(/SPINON_C111_GRID_PIXEL_MATRIX_BEGIN run_id=\S+ pid=(\d+)/)?.[1];
    assert.ok(pid, `${scene.id}: PixelCopy process ID`);
    assert.match(log, new RegExp(`SPINON_C111_GRID_EVAL status=0 scene_id=${scene.id} `),
      `${scene.id}: C11 runtime eval`);
    assert.match(log, /SPINON_C0410_DRAW .*status=0 presented boxes=/, `${scene.id}: WGPU draw`);
    const captureLine = log.split(/\r?\n/).find((entry) => entry.includes('SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok ')
      && entry.includes(`run_id=${sceneRunID}`));
    assert.ok(captureLine, `${scene.id}: PNG capture path`);
    const devicePngPath = captureLine.match(/\bpng=(\S+)/)?.[1];
    assert.ok(devicePngPath?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c11-grid-surface-pixel-matrix/'));
    const pngBytes = device.adb(['exec-out', 'run-as', packageName, 'cat',
      `cache/c11-grid-surface-pixel-matrix/${devicePngPath.split('/').at(-1)}`], { encoding: null }).stdout;
    const comparison = compareC11AndroidPixelCopyScene(log, {
      scene, runID: sceneRunID, pngBytes,
      viewportCssPx: matrix.runtime.viewportCssPx,
    });
    const pngPath = `${scene.id}-pixelcopy.png`;
    writeFileSync(resolve(evidenceDirectory, pngPath), pngBytes);
    writeFileSync(resolve(evidenceDirectory, `${scene.id}-runtime.log`), log);
    observations.push({ ...comparison, processId: Number(pid), png: pngPath });
    process.stdout.write(`Android AVD ${scene.id}: ${comparison.matchedSamples}/${comparison.expectedSamples} exact\n`);
  }
  return { device: { serial: device.serial, model: device.model, apiLevel: device.apiLevel,
    qemu: true, renderer: device.renderer }, appSha256: apkSha256, observations };
}

async function runIOS() {
  const simulator = bootedIOSSimulator();
  build('ios-simulator');
  command('xcrun', ['simctl', 'install', simulator.udid, iosAppPath]);
  writeFileSync(resolve(evidenceDirectory, 'install.log'), `installed=${bundleID}\nudid=${simulator.udid}\n`);
  const logPath = resolve(evidenceDirectory, 'unified.log');
  const descriptor = openSync(logPath, 'a');
  const stream = spawn('xcrun', ['simctl', 'spawn', simulator.udid, 'log', 'stream', '--style', 'compact',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_C111_METAL_PIXEL"'],
  { cwd: repositoryRoot, stdio: ['ignore', descriptor, descriptor] });
  closeSync(descriptor);
  const observations = [];
  try {
    sleep(700);
    for (const scene of matrix.scenes) {
      const sceneRunID = `${runID}-${scene.index}`;
      const payload = samplePayload(scene, 'base64');
      command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulator.udid, bundleID,
        `--spinon-c11-grid-case=${scene.id}`,
        `--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
        `--spinon-c11-grid-metal-surface-samples=${payload}`]);
      const log = waitUntil(() => {
        const current = readText(logPath);
        if (current.includes(`SPINON_C111_METAL_PIXEL_SUMMARY `)
            && current.includes(`run_id=${sceneRunID}`)) return current;
        if (current.includes(`SPINON_C111_METAL_PIXEL_ERROR run_id=${sceneRunID}`)) {
          throw new Error(`${scene.id}: Metal readback 오류\n${current.split(/\r?\n/).filter((line) => line.includes(sceneRunID)).join('\n')}`);
        }
        return null;
      }, `${scene.id}: iOS Metal pixel readback`);
      const fixture = { fixtureId: scene.id, runtime: { viewportCssPx: matrix.viewportCssPx }, samples: scene.samples };
      const comparison = compareC11IOSMetalPixelLogs(log, { fixture, runID: sceneRunID });
      assert.equal(comparison.status, 'matched');
      assert.equal(comparison.matchedSamples, scene.samples.length);
      const screenshotPath = `${scene.id}-simulator.png`;
      const screenshotFilePath = resolve(evidenceDirectory, screenshotPath);
      command('xcrun', ['simctl', 'io', simulator.udid, 'screenshot', screenshotFilePath]);
      const screenshotBytes = readFileSync(screenshotFilePath);
      assert.equal(screenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a', `${scene.id}: Simulator screenshot PNG`);
      writeFileSync(resolve(evidenceDirectory, `${scene.id}-runtime.log`),
        log.split(/\r?\n/).filter((line) => line.includes(sceneRunID)).join('\n') + '\n');
      observations.push({ ...comparison, scene: scene.id,
        screenshot: screenshotPath, screenshotSha256: sha256(screenshotBytes) });
      process.stdout.write(`iOS Simulator ${scene.id}: ${comparison.matchedSamples}/${comparison.expectedSamples} exact\n`);
    }
  } finally {
    stream.kill('SIGTERM');
    await Promise.race([new Promise((done) => stream.once('close', done)), new Promise((done) => setTimeout(done, 2_000))]);
    command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
  }
  return { device: { name: simulator.name, udid: simulator.udid, runtime: simulator.runtime,
    deviceTypeIdentifier: simulator.deviceTypeIdentifier },
  appSha256: sha256(readFileSync(iosAppBinaryPath)), observations };
}

async function main() {
  mkdirSync(evidenceDirectory, { recursive: true });
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.sampleCount, 36);
  const reference = JSON.parse(readFileSync(resolve(repositoryRoot, matrix.oracle.referencePath), 'utf8'));
  const sourceBytes = readFileSync(matrixPath);
  const gitHead = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const result = target === 'android-avd' ? await runAndroid() : await runIOS();
  const matchedScenes = result.observations.length;
  const matchedSamples = result.observations.reduce((sum, scene) => sum
    + (scene.sampleCount ?? scene.matchedSamples), 0);
  assert.equal(matchedScenes, 10);
  assert.equal(matchedSamples, 36);
  const report = {
    schema: 'spinon-c11-mobile-surface-pixel-matrix-result/v1',
    status: 'matched', runID, target, gitHead,
    fixture: { path: 'tests/fixtures/css/c11/mobile-surface-pixel-matrix.json', sha256: sha256(sourceBytes),
      sceneCount: matrix.summary.sceneCount, sampleCount: matrix.summary.sampleCount },
    oracle: { product: reference.oracle.product, referenceId: reference.referenceId,
      referenceSha256: matrix.oracle.referenceSha256, dpr: matrix.oracle.dpr },
    device: result.device,
    renderer: target === 'android-avd'
      ? 'Android SurfaceView PixelCopy; AVD software/virtual GPU path'
      : 'wgpu Metal CAMetalLayer drawable texture readback before compositor submission',
    appBinarySha256: result.appSha256,
    summary: { expectedScenes: 10, matchedScenes, expectedSamples: 36, matchedSamples,
      maximumChannelError: 0, screenshotOrCompositorReadback: false },
    observations: result.observations,
    limitations: target === 'android-avd' ? [
      'Android API 37 AVD에서 실행했다. SM-S731N 실기기 또는 하드웨어 GPU 동작을 증명하지 않는다.',
      'PixelCopy는 앱 SurfaceView 표면의 결과다. OS compositor 이후 최종 display pixel 비교가 아니다.',
      '이 고정 10-scene fixture는 WPT 적합성이나 전체 CSS 지원을 증명하지 않는다.',
    ] : [
      'iPhone 17 Pro iOS 26.2 Simulator에서 실행했다. iOS 실기기 GPU 결과가 아니다.',
      'Metal drawable texture readback은 compositor 제출 전이다. 최종 display pixel과 같음을 보장하지 않는다.',
      '이 고정 10-scene fixture는 WPT 적합성이나 전체 CSS 지원을 증명하지 않는다.',
    ],
  };
  writeFileSync(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 모바일 표면 픽셀 비교', '',
    `- 상태: ${matchedScenes}/10 scene · ${matchedSamples}/36 sample · RGBA exact · 최대 채널 오차 0`,
    `- 실행 ID: ${runID}`, `- 대상: ${JSON.stringify(result.device)}`,
    `- renderer: ${report.renderer}`,
    `- oracle: Chrome ${reference.oracle.product} · ${matrix.oracle.referenceId} · DPR ${matrix.oracle.dpr}`,
    '- Android는 PixelCopy PNG 자체의 표본 픽셀·SHA-256과 로그를 교차 확인했다. iOS는 Metal drawable readback 로그를 고정 Chrome 표본과 비교했다.',
    '- 이 결과는 고정 C11.1 10 scene의 GPU surface 표본만 다룬다. 최종 화면 합성, 실기기 GPU, Spinon WPT 적합성, 전체 CSS 지원을 의미하지 않는다.',
    '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: report.status, target, runID, scenes: matchedScenes,
    samples: matchedSamples, device: result.device, evidenceDirectory }, null, 2)}\n`);
}

main().catch((error) => {
  try {
    mkdirSync(evidenceDirectory, { recursive: true });
    writeFileSync(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  } catch { /* Keep the primary validation failure. */ }
  process.stderr.write(`${error.stack ?? error}\n증거 경로: ${evidenceDirectory}\n`);
  process.exitCode = 1;
});
