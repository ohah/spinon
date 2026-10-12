import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { closeSync, mkdirSync, openSync, readFileSync, writeFileSync } from 'node:fs';
import { spawn, spawnSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';
import { hasRunSummary } from './c11-grid-placement-pixel-capture.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const fixturePath = resolve(repositoryRoot, 'tests/fixtures/css/c11/grid-placement-pixel-samples.json');
const fixture = JSON.parse(readFileSync(fixturePath, 'utf8'));
const target = process.argv[2];
assert.ok(['android-avd', 'android-physical', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-2-1-grid-placement-pixels.mjs android-avd|android-physical|ios-simulator [Android serial]');
const serial = process.argv[3] ?? 'emulator-5554';
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const simulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const runID = `c1121-pixels-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-2-1-grid-placement-pixels-${date}`, `${target}-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const iosBinaryPath = resolve(iosAppPath, 'SpinonBootstrap');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function verifyChromeFixture() {
  assert.equal(fixture.schema, 'spinon-css-c11-2-1-grid-placement-pixel-samples/v1');
  assert.equal(fixture.fixtureId, 'c11-2-1-numeric-lines');
  const referenceBytes = readFileSync(resolve(repositoryRoot, fixture.oracle.referencePath));
  const screenshotBytes = readFileSync(resolve(repositoryRoot, fixture.oracle.screenshotPath));
  assert.equal(sha256(referenceBytes), fixture.oracle.referenceSha256, 'Chrome reference hash');
  assert.equal(sha256(screenshotBytes), fixture.oracle.screenshotSha256, 'Chrome screenshot hash');
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(reference.referenceId, fixture.oracle.referenceId);
  assert.equal(reference.oracle.product, fixture.oracle.product);
  const observation = reference.observations.find(({ deviceScaleFactor }) =>
    deviceScaleFactor === fixture.oracle.deviceScaleFactor);
  const scene = observation?.cases.find(({ id }) => id === fixture.sceneId);
  assert.ok(scene, '고정 Chrome reference에 numeric-lines 장면이 없습니다.');
  assert.ok(reference.screenshots.some(({ caseId, path, sha256: screenshotHash }) =>
    caseId === fixture.sceneId && path === fixture.oracle.screenshotPath
      && screenshotHash === fixture.oracle.screenshotSha256), 'Chrome scene screenshot 연결이 다릅니다.');
  const image = decodeRgbaPng(screenshotBytes);
  assert.deepEqual([image.width, image.height],
    [fixture.oracle.viewportCssPx.width, fixture.oracle.viewportCssPx.height]);
  for (const sample of fixture.samples) {
    assert.deepEqual(image.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba,
      `${sample.id}: fixture 표본이 pinned Chrome screenshot과 다릅니다.`);
  }
  assert.equal(scene.nodes.find(({ id }) => id === 'c1121-positive-lines').rect.width, 165);
  assert.equal(scene.nodes.find(({ id }) => id === 'c1121-negative-lines').rect.x, 170);
  return { referenceBytes, screenshotBytes, scene };
}

function samplePayload(encoding) {
  return Buffer.from(JSON.stringify(fixture.samples)).toString(encoding);
}

function currentAndroidDevice() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(devices.length, 1, `Android target가 연결되어 있지 않습니다: ${serial}`);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const qemu = adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim() === '1';
  assert.equal(qemu, target === 'android-avd', '요청한 Android target 종류와 실제 장치가 다릅니다.');
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  assert.ok(apiLevel >= 36, `Android API ${apiLevel}은 고정 C11 capture 대상보다 낮습니다.`);
  if (target === 'android-avd') assert.equal(apiLevel, 37, 'Android API 37 AVD가 아닙니다.');
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

async function runAndroid() {
  const { adb, metadata } = currentAndroidDevice();
  const build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'bun', 'run', 'build:android'], { timeout: 900_000 });
  writeFileSync(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const apkBytes = readFileSync(apkPath);
  const appSha256 = sha256(apkBytes);
  writeFileSync(resolve(evidenceDirectory, 'install.log'), adb(['install', '-r', apkPath]).stdout);
  adb(['shell', 'am', 'force-stop', packageName]);
  const launch = adb(['shell', 'am', 'start', '-n', component,
    '--ez', 'spinon_c1121_grid_placement', 'true',
    '--es', 'spinon_c11_grid_case', fixture.sceneId,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', samplePayload('base64url')]);
  writeFileSync(resolve(evidenceDirectory, 'launch.log'), `${launch.stdout}${launch.stderr}`);

  const deadline = Date.now() + 120_000;
  let log = '';
  let pid = null;
  while (Date.now() < deadline) {
    pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim().split(/\s+/)[0]) || null;
    const all = adb(['shell', 'logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
    log = pid === null ? all : all.split(/\r?\n/).filter((line) => {
      const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
      return match && Number(match[1]) === pid;
    }).join('\n');
    const captured = hasRunSummary(log, 'SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY', runID);
    if (captured) break;
    if (log.includes(`SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${runID}`)
        || log.includes(`SPINON_C111_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
        || log.includes('SPINON_C1121_GRID_ARGUMENT_ERROR')) break;
    sleep(300);
  }
  writeFileSync(resolve(evidenceDirectory, 'runtime.log'), `${log}\n`);
  assert.ok(hasRunSummary(log, 'SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY', runID),
    'Android PixelCopy summary marker가 없습니다.');
  assert.ok(log.includes(`SPINON_C1121_GRID_EVAL status=0`), 'C11.2.1 V8 runtime 결과가 없습니다.');
  assert.ok(log.includes('SPINON_C0410_DRAW') && log.includes('presented boxes=4'), 'WGPU present marker가 없습니다.');
  const capture = log.split(/\r?\n/).find((line) =>
    line.includes('SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok') && line.includes(`run_id=${runID}`));
  assert.ok(capture, '현재 run의 Android PixelCopy 결과가 없습니다.');
  const appPng = capture.match(/\bpng=(\S+)/)?.[1];
  assert.ok(appPng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c11-grid-surface-pixel-matrix/'));
  const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
    `cache/c11-grid-surface-pixel-matrix/${appPng.split('/').at(-1)}`], { encoding: null }).stdout;
  const comparison = compareC11AndroidPixelCopyScene(log, {
    scene: { id: fixture.sceneId, samples: fixture.samples }, runID, pngBytes,
  });
  writeFileSync(resolve(evidenceDirectory, 'pixelcopy.png'), pngBytes);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
  writeFileSync(resolve(evidenceDirectory, 'android-screen.png'), screenshot);
  return { device: metadata, appBinarySha256: appSha256, comparison,
    screenshotSha256: sha256(screenshot) };
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

async function runIOS() {
  const simulator = bootedSimulator();
  const build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1',
    'bun', 'run', 'build:ios-sim'], { timeout: 900_000 });
  writeFileSync(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  command('xcrun', ['simctl', 'install', simulator.udid, iosAppPath]);
  const logPath = resolve(evidenceDirectory, 'unified.log');
  const fd = openSync(logPath, 'a');
  const stream = spawn('xcrun', ['simctl', 'spawn', simulator.udid, 'log', 'stream', '--style', 'compact',
    '--predicate', 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C111_METAL_PIXEL" OR eventMessage CONTAINS[c] "SPINON_C1121_GRID")'],
  { cwd: repositoryRoot, stdio: ['ignore', fd, fd] });
  closeSync(fd);
  try {
    sleep(700);
    const payload = samplePayload('base64');
    const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulator.udid,
      packageName, '--spinon-c1121-grid-placement', `--spinon-c11-grid-case=${fixture.sceneId}`,
      `--spinon-c11-grid-metal-surface-pixels=${runID}`,
      `--spinon-c11-grid-metal-surface-samples=${payload}`]);
    writeFileSync(resolve(evidenceDirectory, 'launch.log'), `${launch.stdout}${launch.stderr}`);
    const deadline = Date.now() + 120_000;
    let log = '';
    while (Date.now() < deadline) {
      log = readFileSync(logPath, 'utf8');
      if (hasRunSummary(log, 'SPINON_C111_METAL_PIXEL_SUMMARY', runID, 'matched')
          || hasRunSummary(log, 'SPINON_C111_METAL_PIXEL_SUMMARY', runID, 'mismatch')
          || log.includes(`SPINON_C111_METAL_PIXEL_ERROR run_id=${runID}`)
          || /SPINON_C1121_GRID_SUMMARY scene_id=numeric-lines status=-\d+/.test(log)) break;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 300));
    }
    writeFileSync(resolve(evidenceDirectory, 'runtime.log'), log.split(/\r?\n/)
      .filter((line) => line.includes(runID) || line.includes('SPINON_C1121_GRID')).join('\n') + '\n');
    assert.ok(log.includes('SPINON_C1121_GRID_SUMMARY scene_id=numeric-lines status=0 layout=ready boxes=4'),
      'C11.2.1 iOS V8 runtime 결과가 없습니다.');
    const comparison = compareC11IOSMetalPixelLogs(log, { fixture, runID });
    const screenshotPath = resolve(evidenceDirectory, 'ios-screen.png');
    command('xcrun', ['simctl', 'io', simulator.udid, 'screenshot', screenshotPath]);
    const screenshot = readFileSync(screenshotPath);
    assert.equal(screenshot.toString('hex', 0, 8), '89504e470d0a1a0a', 'iOS Simulator screenshot PNG');
    return { device: { name: simulator.name, udid: simulator.udid, runtime: simulator.runtime },
      appBinarySha256: sha256(readFileSync(iosBinaryPath)), comparison,
      screenshotSha256: sha256(screenshot) };
  } finally {
    stream.kill('SIGTERM');
    await Promise.race([new Promise((done) => stream.once('close', done)),
      new Promise((done) => setTimeout(done, 2_000))]);
    command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  }
}

async function main() {
  mkdirSync(evidenceDirectory, { recursive: true });
  const oracle = verifyChromeFixture();
  const result = target === 'ios-simulator' ? await runIOS() : await runAndroid();
  const report = {
    schema: 'spinon-c11-2-1-grid-placement-pixels-result/v1',
    status: 'matched', runID, target,
    fixture: { path: 'tests/fixtures/css/c11/grid-placement-pixel-samples.json',
      sha256: sha256(readFileSync(fixturePath)), sampleCount: fixture.samples.length },
    oracle: { referenceId: fixture.oracle.referenceId,
      referenceSha256: sha256(oracle.referenceBytes), screenshotSha256: sha256(oracle.screenshotBytes),
      product: fixture.oracle.product, scene: fixture.sceneId, deviceScaleFactor: 1 },
    device: result.device,
    renderer: target === 'ios-simulator'
      ? 'WGPU Metal drawable texture before compositor submission'
      : 'Android SurfaceView PixelCopy',
    appBinarySha256: result.appBinarySha256,
    comparison: result.comparison,
    screenshotSha256: result.screenshotSha256,
  };
  writeFileSync(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(report, null, 2)}\n`);
  const screenshotName = target === 'ios-simulator' ? 'ios-screen.png' : 'android-screen.png';
  writeFileSync(resolve(evidenceDirectory, 'README.md'), [
    '# C11.2.1 numeric-lines 표면 픽셀 실행', '',
    `- 결과: ${report.status} · ${target} · ${report.comparison.matchedSamples}/${report.comparison.expectedSamples} exact`,
    `- 실행 ID: ${runID}`,
    `- Chrome oracle: ${fixture.oracle.product} · ${fixture.oracle.referenceId}`,
    `- 앱 산출물 SHA-256: ${report.appBinarySha256}`,
    `- 캡처: ${report.renderer}; 최종 OS compositor/display 픽셀 검증은 아님`,
    `- 근거: [comparison.json](comparison.json) · [빌드 로그](build.log) · [런타임 로그](runtime.log) · [화면 캡처](${screenshotName})`,
    '',
  ].join('\n'));
  process.stdout.write(`${target}: ${report.comparison.matchedSamples}/${report.comparison.expectedSamples} exact · ${evidenceDirectory}\n`);
}

main().catch((error) => {
  writeFileSync(resolve(evidenceDirectory, 'failure.log'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
