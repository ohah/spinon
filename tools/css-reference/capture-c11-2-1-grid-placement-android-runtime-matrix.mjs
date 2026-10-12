import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import { command, component, androidApkPath, createEvidenceDirectory,
  loadFixedInputs, loadPixelMatrix, newRunID, packageName, repositoryRoot, saveSceneEvidence,
  sha256, writeMatrixSummary, referenceCase } from './c11-2-1-grid-placement-runtime-support.mjs';

const serial = process.argv[2] ?? '';
assert.match(serial, /^[A-Za-z0-9._:-]+$/, '사용법: node capture-c11-2-1-grid-placement-android-runtime-matrix.mjs <adb serial>');
const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
const runID = newRunID('android');
const evidenceDirectory = await createEvidenceDirectory('android', runID);

function connectedDevice() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serial} `) && line.includes(' device')),
    `ADB target이 device 상태가 아닙니다: ${serial}`);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  const sdk = getprop('ro.build.version.sdk');
  const model = getprop('ro.product.model');
  const release = getprop('ro.build.version.release');
  const fingerprint = getprop('ro.build.fingerprint');
  const density = adb(['shell', 'wm', 'density']).stdout.trim();
  const size = adb(['shell', 'wm', 'size']).stdout.trim();
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음';
  const platform = qemu === '1'
    ? `Android ${release}/API ${sdk} AVD (${model})`
    : `Android ${release}/API ${sdk} physical device (${model})`;
  return { qemu, sdk: Number(sdk), model, release, fingerprint, density, size, renderer, platform };
}

function pidForPackage() {
  return adb(['shell', 'pidof', packageName], { allowFailure: true })
    .stdout.trim().split(/\s+/).filter(Boolean).map(Number)[0] ?? null;
}

function processLog(pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime'], {
    timeout: 20_000, maxBuffer: 64 * 1024 * 1024,
  }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const fields = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return fields && Number(fields[1]) === pid;
  });
}

function waitFor(predicate, description, timeout = 120_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(250);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function pixelSamplePayload(scene) {
  return Buffer.from(JSON.stringify(scene.samples.map(({ id, nodeId, xCssPx, yCssPx }) => ({
    id, nodeId, xCssPx, yCssPx,
  })))).toString('base64url');
}

async function main() {
  const inputs = await loadFixedInputs();
  const { matrix: pixelMatrix } = await loadPixelMatrix(inputs);
  const pixelCases = new Map(pixelMatrix.cases.map((scene) => [scene.id, scene]));
  const device = connectedDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify({ serial, ...device }, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'bun', 'run', 'build:android',
  ], { timeout: 900_000, maxBuffer: 64 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const apkBytes = await readFile(androidApkPath);
  const apkSha256 = sha256(apkBytes);
  const install = adb(['install', '-r', androidApkPath], { timeout: 180_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);

  const observations = [];
  for (const fixture of inputs.matrix.cases) {
    const sceneRunID = `${runID}-${fixture.id}`;
    const pixelFixture = pixelCases.get(fixture.id);
    assert.ok(pixelFixture, `${fixture.id}: surface pixel matrix case 없음`);
    const capturesPixels = pixelFixture.outcome === 'surface-pixel-compare';
    adb(['shell', 'am', 'force-stop', packageName]);
    const launchArgs = [
      'shell', 'am', 'start', '-n', component,
      '--ez', 'spinon_c1121_grid_placement', 'true',
      '--es', 'spinon_c1121_grid_scene', fixture.id,
      '--es', 'spinon_c1121_grid_run_id', sceneRunID,
    ];
    if (capturesPixels) {
      launchArgs.push('--es', 'spinon_c11_grid_case', fixture.id,
        '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
        '--es', 'spinon_c11_grid_pixel_matrix_samples', pixelSamplePayload(pixelFixture));
    }
    const launch = adb(launchArgs);
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-launch.log`), launch.stdout + launch.stderr);
    const pid = waitFor(pidForPackage, `${fixture.id}: 앱 process`);
    let lines = [];
    const marker = `SPINON_C1121_GRID_SUMMARY run_id=${sceneRunID} scene_id=${fixture.id}`;
    const isSuccess = fixture.expected.outcome === 'success';
    waitFor(() => {
      lines = processLog(pid);
      const summaryIndex = lines.findIndex((line) => line.includes(marker));
      const expectedBoxes = fixture.nodeIds.length + 1;
      const drawReady = lines.some((line, index) => index > summaryIndex
        && line.includes('SPINON_C0410_DRAW')
        && line.includes(`presented boxes=${expectedBoxes}`));
      const pixelReady = !capturesPixels || lines.some((line) =>
        line.includes('SPINON_C111_GRID_PIXEL_MATRIX_SUMMARY status=captured')
          && line.includes(`run_id=${sceneRunID}`));
      const pixelFailed = capturesPixels && lines.some((line) =>
        line.includes(`SPINON_C111_GRID_PIXEL_MATRIX status=error run_id=${sceneRunID}`)
          || line.includes(`SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${sceneRunID}`));
      if (pixelFailed) throw new Error(`${fixture.id}: PixelCopy 실패\n${lines.join('\n')}`);
      return summaryIndex >= 0 && (!isSuccess || drawReady) && pixelReady;
    }, `${fixture.id}: runtime summary와 WGPU draw`);
    if (!isSuccess) sleep(750);
    lines = processLog(pid);
    const png = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], {
      encoding: 'buffer', timeout: 20_000, maxBuffer: 32 * 1024 * 1024,
    }).stdout;
    const result = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase: fixture,
      chromeCase: referenceCase(inputs.referenceCases, fixture.id),
      lines,
      screenshotBytes: png,
      runID: sceneRunID,
      platform: device.qemu === '1' ? 'android-avd' : 'android-physical',
    });
    if (capturesPixels) {
      const captureLine = lines.find((line) => line.includes('SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok ')
        && line.includes(`run_id=${sceneRunID}`));
      assert.ok(captureLine, `${fixture.id}: PixelCopy capture marker 없음`);
      const devicePngPath = captureLine.match(/\bpng=(\S+)/)?.[1];
      assert.ok(devicePngPath?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c11-grid-surface-pixel-matrix/'));
      const pixelPng = adb(['exec-out', 'run-as', packageName, 'cat',
        `cache/c11-grid-surface-pixel-matrix/${devicePngPath.split('/').at(-1)}`], {
        encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
      }).stdout;
      const surfacePixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
        scene: { id: fixture.id, samples: pixelFixture.samples }, runID: sceneRunID, pngBytes: pixelPng,
      });
      const pixelPngPath = resolve(evidenceDirectory, `${fixture.id}-pixelcopy.png`);
      await writeFile(pixelPngPath, pixelPng);
      await writeFile(resolve(evidenceDirectory, `${fixture.id}-surface-pixel-comparison.json`),
        `${JSON.stringify(surfacePixelComparison, null, 2)}\n`);
      result.surfacePixelComparison = surfacePixelComparison;
      process.stdout.write(`  PixelCopy: ${surfacePixelComparison.matchedSamples}/${surfacePixelComparison.expectedSamples} exact\n`);
    } else {
      assert.equal(pixelFixture.expected.outcome, 'expected-rejection');
      assert.equal(lines.some((line) => line.includes(`SPINON_C111_GRID_PIXEL_MATRIX_BEGIN run_id=${sceneRunID}`)), false,
        `${fixture.id}: 거부 장면에서 pixel capture가 시작되었습니다.`);
    }
    result.runID = sceneRunID;
    result.processId = pid;
    observations.push(result);
    process.stdout.write(`${fixture.id}: ${result.outcome} · pid=${pid}\n`);
  }
  const invalidRunID = `${runID}-pixel-allowlist-negative`;
  adb(['shell', 'am', 'force-stop', packageName]);
  const invalidLaunch = adb([
    'shell', 'am', 'start', '-n', component,
    '--ez', 'spinon_c1121_grid_placement', 'true',
    '--es', 'spinon_c1121_grid_scene', 'deferred-implicit-track',
    '--es', 'spinon_c1121_grid_run_id', invalidRunID,
    '--es', 'spinon_c11_grid_case', 'deferred-implicit-track',
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', invalidRunID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', pixelSamplePayload(
      pixelCases.get('numeric-lines')),
  ]);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-launch.log'),
    invalidLaunch.stdout + invalidLaunch.stderr);
  const invalidPID = waitFor(pidForPackage, '지원 밖 pixel scene negative control app process');
  const invalidLines = waitFor(() => {
    const lines = processLog(invalidPID);
    return lines.some((line) => line.includes('SPINON_C111_GRID_ARGUMENT_ERROR rejected=true'))
      ? lines : null;
  }, '지원 밖 pixel scene 거부 marker');
  assert.equal(invalidLines.some((line) => line.includes(`SPINON_C111_GRID_PIXEL_MATRIX_BEGIN run_id=${invalidRunID}`)), false);
  assert.equal(invalidLines.some((line) => line.includes(`SPINON_C111_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${invalidRunID}`)), false);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.log'),
    `${invalidLines.join('\n')}\n`);
  const pixelIntentNegativeControl = {
    status: 'rejected', runID: invalidRunID, processId: invalidPID,
    requestedScene: 'deferred-implicit-track', expectedScene: 'numeric-lines',
    argumentRejected: true, pixelCopyStarted: false,
  };
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.json'),
    `${JSON.stringify(pixelIntentNegativeControl, null, 2)}\n`);
  const summary = await writeMatrixSummary({
    directory: evidenceDirectory,
    platform: device.platform,
    runID,
    target: `${serial} · qemu=${device.qemu} · API ${device.sdk}`,
    appSha256: apkSha256,
    device,
    inputs,
    pixelMatrix,
    pixelIntentNegativeControl,
    observations,
  });
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID, platform: device.platform,
    apkSha256, summary: summary.summary, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
