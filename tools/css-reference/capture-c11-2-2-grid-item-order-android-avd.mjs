import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  androidApkPath, androidComponent, command, compareOrderScene, createOrderEvidenceDirectory,
  loadC1122GridItemOrderInputs, newOrderRunID, packageName, repositoryRoot, saveSceneEvidence,
  sha256, writeOrderRuntimeSummary,
} from './c11-2-2-grid-item-order-runtime-support.mjs';

const serial = process.argv[2] ?? 'emulator-5554';
assert.match(serial, /^[A-Za-z0-9._:-]+$/,
  '사용법: node capture-c11-2-2-grid-item-order-android-avd.mjs [AVD serial]');
const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const runID = newOrderRunID('android-avd');
const evidenceDirectory = await createOrderEvidenceDirectory('android-avd', runID);
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function connectedAvd() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serial} `) && line.includes(' device')),
    `ADB target이 device 상태가 아닙니다: ${serial}`);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  assert.equal(qemu, '1', `AVD가 아닌 target입니다: ${serial}`);
  const release = getprop('ro.build.version.release');
  const sdk = Number(getprop('ro.build.version.sdk'));
  const model = getprop('ro.product.model');
  const abi = getprop('ro.product.cpu.abi');
  assert.equal(sdk, 37, `Android API 37이 아닙니다: ${sdk}`);
  assert.equal(abi, 'arm64-v8a', `ARM64 AVD가 아닙니다: ${abi}`);
  return {
    serial, qemu, release, sdk, model, abi,
    fingerprint: getprop('ro.build.fingerprint'),
    density: adb(['shell', 'wm', 'density']).stdout.trim(),
    size: adb(['shell', 'wm', 'size']).stdout.trim(),
    renderer: adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
      .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음',
  };
}

function pidForPackage() {
  return adb(['shell', 'pidof', packageName], { allowFailure: true })
    .stdout.trim().split(/\s+/).filter(Boolean).map(Number)[0] ?? null;
}

function processLog(pid) {
  const output = adb(['shell', 'logcat', '-d', '-v', 'threadtime'], {
    timeout: 30_000, maxBuffer: 96 * 1024 * 1024,
  }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const fields = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return fields && Number(fields[1]) === pid;
  });
}

function waitFor(predicate, description, timeout = 90_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(250);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

async function main() {
  const inputs = await loadC1122GridItemOrderInputs();
  const device = connectedAvd();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:android',
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const apkBytes = await readFile(androidApkPath);
  const appSha256 = sha256(apkBytes);
  const install = adb(['install', '-r', androidApkPath], { timeout: 180_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const observations = [];
  const pixelObservations = [];
  for (const fixtureCase of inputs.runtimeMatrix.cases) {
    const sceneRunID = `${runID}-${fixtureCase.id}`;
    const pixelCase = inputs.pixelMatrix.cases.find(({ id }) => id === fixtureCase.id);
    assert.ok(pixelCase, `${fixtureCase.id}: pixel scene 누락`);
    adb(['shell', 'am', 'force-stop', packageName]);
    adb(['logcat', '-c']);
    const payload = Buffer.from(JSON.stringify(pixelCase.samples.map(({ id, nodeId, xCssPx, yCssPx }) => ({
      id, nodeId, xCssPx, yCssPx,
    })))).toString('base64url');
    const launch = adb([
      'shell', 'am', 'start', '-n', androidComponent,
      '--es', 'spinon_c1122_grid_auto_placement_scene', fixtureCase.id,
      '--es', 'spinon_c1122_grid_auto_placement_run_id', sceneRunID,
      '--es', 'spinon_c11_grid_case', fixtureCase.id,
      '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
      '--es', 'spinon_c11_grid_pixel_matrix_samples', payload,
    ], { timeout: 30_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`), `${launch.stdout}${launch.stderr}`);
    const pid = waitFor(pidForPackage, `${fixtureCase.id}: app process`);
    const lines = waitFor(() => {
      const current = processLog(pid);
      const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
        && !line.includes('status=0 ') && !line.includes('status=-12 '));
      if (failedDraw) throw new Error(`${fixtureCase.id}: Android draw 실패: ${failedDraw}`);
      const hasSummary = current.some((line) => line.includes('SPINON_C1122_GRID_SUMMARY ')
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
      const hasDraw = current.some((line) => line.includes('SPINON_C0410_DRAW')
        && /presented boxes=\d+/.test(line));
      const hasPixelCapture = current.some((line) =>
        line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_SUMMARY status=captured')
          && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene=${fixtureCase.id}`));
      const failedPixelCapture = current.find((line) =>
        line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_SUMMARY')
          && line.includes(`run_id=${sceneRunID}`) && !line.includes('status=captured'));
      if (failedPixelCapture) throw new Error(`${fixtureCase.id}: PixelCopy 실패: ${failedPixelCapture}`);
      return hasSummary && hasDraw && hasPixelCapture ? current : null;
    }, `${fixtureCase.id}: runtime, draw, PixelCopy`);
    const screenshotBytes = adb(['exec-out', 'screencap', '-p'], {
      encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
    }).stdout;
    const comparison = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase,
      chromeCase: null,
      lines,
      screenshotBytes,
      runID: sceneRunID,
      platform: 'android-avd',
      compareScene: (options) => compareOrderScene({ ...options, fixtureCase }),
    });
    const captureLine = lines.find((line) => line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_CAPTURE status=ok')
      && line.includes(`run_id=${sceneRunID}`));
    assert.ok(captureLine, `${fixtureCase.id}: PixelCopy 성공 marker 누락`);
    const devicePng = captureLine.match(/\bpng=(\S+)/)?.[1];
    assert.ok(devicePng, `${fixtureCase.id}: PixelCopy PNG 경로 누락`);
    const pixelPng = adb(['exec-out', 'run-as', packageName, 'cat', devicePng], {
      encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
    }).stdout;
    const pixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
      scene: pixelCase,
      runID: sceneRunID,
      pngBytes: pixelPng,
      viewportCssPx: inputs.pixelMatrix.runtime.viewportCssPx,
      markerPrefix: 'C1122',
      cacheDirectory: 'c1122-grid-surface-pixel-matrix',
    });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-surface-pixels.png`), pixelPng);
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-pixel-comparison.json`),
      `${JSON.stringify(pixelComparison, null, 2)}\n`);
    comparison.surfacePixelComparison = pixelComparison;
    comparison.runID = sceneRunID;
    comparison.processId = pid;
    observations.push(comparison);
    pixelObservations.push(pixelComparison);
    process.stdout.write(`${fixtureCase.id}: ${comparison.outcome} · ${pixelComparison.matchedSamples} PixelCopy samples\n`);
  }
  const summary = await writeOrderRuntimeSummary({
    directory: evidenceDirectory,
    platform: 'Android API 37 ARM64 AVD',
    runID,
    target: `${serial} · Android ${device.release}/API ${device.sdk} · ${device.model} · ${device.abi}`,
    device,
    appSha256,
    inputs,
    observations,
    pixelObservations,
  });
  adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
