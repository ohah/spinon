import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  androidApkPath, androidComponent, command, createEvidenceDirectory,
  loadC1122Inputs, loadC1122PixelMatrix, newRunID, packageName, repositoryRoot, saveSceneEvidence,
  sha256, writeRuntimeSummary,
} from './c11-2-2-grid-auto-placement-runtime-support.mjs';
import { compareC1122GridAutoPlacementScene } from './c11-2-2-grid-auto-placement-runtime-compare.mjs';
import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';

const serial = process.argv[2] ?? '';
assert.match(serial, /^[A-Za-z0-9._:-]+$/,
  '사용법: node capture-c11-2-2-grid-auto-placement-android-runtime-matrix.mjs <실기기 serial>');
const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
const runID = newRunID('android-physical');
const evidenceDirectory = await createEvidenceDirectory('android', runID);

function connectedDevice() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serial} `) && line.includes(' device')),
    `ADB target이 device 상태가 아닙니다: ${serial}`);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  assert.notEqual(qemu, '1', `실기기 전용 행렬인데 AVD가 선택됐습니다: ${serial}`);
  const release = getprop('ro.build.version.release');
  const sdk = Number(getprop('ro.build.version.sdk'));
  const model = getprop('ro.product.model');
  const fingerprint = getprop('ro.build.fingerprint');
  const density = adb(['shell', 'wm', 'density']).stdout.trim();
  const size = adb(['shell', 'wm', 'size']).stdout.trim();
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음';
  return {
    serial, qemu, release, sdk, model, fingerprint, density, size, renderer,
    platform: `Android ${release}/API ${sdk} 실기기 (${model})`,
  };
}

function pidForPackage() {
  return adb(['shell', 'pidof', packageName], { allowFailure: true })
    .stdout.trim().split(/\s+/).filter(Boolean).map(Number)[0] ?? null;
}

function processLog(pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime'], {
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
  const inputs = await loadC1122Inputs();
  const pixelMatrix = await loadC1122PixelMatrix(inputs);
  const pixelCases = new Map(pixelMatrix.matrix.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
  const device = connectedDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:android',
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const apkBytes = await readFile(androidApkPath);
  const apkSha256 = sha256(apkBytes);
  const install = adb(['install', '-r', androidApkPath], { timeout: 180_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const observations = [];
  const pixelObservations = [];
  for (const fixtureCase of inputs.matrix.cases) {
    const sceneRunID = `${runID}-${fixtureCase.id}`;
    const isSuccess = fixtureCase.expected.outcome === 'success';
    const pixelScene = pixelCases.get(fixtureCase.id);
    adb(['shell', 'am', 'force-stop', packageName]);
    adb(['logcat', '-c']);
    const launchArgs = [
      'shell', 'am', 'start', '-n', androidComponent,
      '--es', 'spinon_c1122_grid_auto_placement_scene', fixtureCase.id,
      '--es', 'spinon_c1122_grid_auto_placement_run_id', sceneRunID,
    ];
    if (isSuccess) {
      assert.ok(pixelScene && pixelScene.samples.length > 0,
        `${fixtureCase.id}: 성공 scene의 surface pixel fixture가 없습니다.`);
      const payload = Buffer.from(JSON.stringify(pixelScene.samples.map(({ id, nodeId, xCssPx, yCssPx }) => ({
        id, nodeId, xCssPx, yCssPx,
      })))).toString('base64url');
      launchArgs.push('--es', 'spinon_c11_grid_case', fixtureCase.id,
        '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
        '--es', 'spinon_c11_grid_pixel_matrix_samples', payload);
    }
    const launch = adb(launchArgs, { timeout: 30_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`),
      `${launch.stdout}${launch.stderr}`);
    const pid = waitFor(pidForPackage, `${fixtureCase.id}: Android app process`);
    const marker = isSuccess
      ? 'SPINON_C1122_GRID_FRAME_SUMMARY' : 'SPINON_C1122_GRID_DIAGNOSTIC';
    const lines = waitFor(() => {
      const current = processLog(pid);
      const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
        && !line.includes('status=0 ') && !line.includes('status=-12 '));
      if (failedDraw) throw new Error(`${fixtureCase.id}: Android draw 실패: ${failedDraw}`);
      const hasResult = current.some((line) => line.includes(marker)
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
      const hasDraw = current.some((line) => line.includes('SPINON_C0410_DRAW')
        && /presented boxes=\d+/.test(line));
      const hasPixelCapture = !isSuccess || current.some((line) =>
        line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_SUMMARY status=captured')
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene=${fixtureCase.id}`));
      const failedPixelCapture = isSuccess && current.find((line) =>
        line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_SUMMARY')
        && line.includes(`run_id=${sceneRunID}`) && !line.includes('status=captured'));
      if (failedPixelCapture) {
        throw new Error(`${fixtureCase.id}: Android PixelCopy 실패: ${failedPixelCapture}`);
      }
      return hasResult && (!isSuccess || hasDraw) && hasPixelCapture ? current : null;
    }, `${fixtureCase.id}: runtime 결과와 draw`);
    const screenshot = adb(['exec-out', 'screencap', '-p'], {
      encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
    }).stdout;
    const comparison = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase,
      chromeCase: inputs.referenceCases.find(({ id }) => id === fixtureCase.id),
      lines,
      screenshotBytes: screenshot,
      runID: sceneRunID,
      platform: 'android-physical',
      compareScene: compareC1122GridAutoPlacementScene,
    });
    if (isSuccess) {
      const captureLine = lines.find((line) => line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_CAPTURE status=ok')
        && line.includes(`run_id=${sceneRunID}`));
      assert.ok(captureLine, `${fixtureCase.id}: PixelCopy 성공 marker가 없습니다.`);
      const devicePng = captureLine.match(/\bpng=(\S+)/)?.[1];
      assert.ok(devicePng, `${fixtureCase.id}: PixelCopy PNG 경로가 없습니다.`);
      const pixelPng = adb(['exec-out', 'run-as', packageName, 'cat', devicePng], {
        encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
      }).stdout;
      const pixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
        scene: pixelScene,
        runID: sceneRunID,
        pngBytes: pixelPng,
        viewportCssPx: pixelMatrix.matrix.runtime.viewportCssPx,
        markerPrefix: 'C1122',
        cacheDirectory: 'c1122-grid-surface-pixel-matrix',
      });
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-surface-pixels.png`), pixelPng);
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-pixel-comparison.json`),
        `${JSON.stringify(pixelComparison, null, 2)}\n`);
      comparison.surfacePixelComparison = pixelComparison;
      pixelObservations.push(pixelComparison);
    } else {
      assert.ok(!lines.some((line) => line.includes('SPINON_C1122_GRID_PIXEL_MATRIX_BEGIN')
        && line.includes(`run_id=${sceneRunID}`)),
      `${fixtureCase.id}: expected rejection scene이 pixel readback을 시작했습니다.`);
    }
    comparison.runID = sceneRunID;
    comparison.processId = pid;
    observations.push(comparison);
    process.stdout.write(`${fixtureCase.id}: ${comparison.outcome} · pid=${pid}\n`);
  }

  const summary = await writeRuntimeSummary({
    directory: evidenceDirectory,
    platform: 'Android physical device',
    runID,
    target: `${serial} · Android ${device.release}/API ${device.sdk} · ${device.model}`,
    device,
    appSha256: apkSha256,
    inputs,
    observations,
    pixelMatrix,
    pixelObservations,
  });
  process.stdout.write(`${JSON.stringify({
    status: summary.status, runID, platform: summary.platform,
    summary: summary.summary, evidenceDirectory,
  }, null, 2)}\n`);
}

main().catch(async (error) => {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
