import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';
import {
  command, createEvidenceDirectory, loadFixedInputs, loadPixelMatrix, newRunID, packageName,
  repositoryRoot, saveSceneEvidence, sha256, simulatorAppPath, simulatorUDID,
  writeMatrixSummary, referenceCase,
} from './c11-2-1-grid-placement-runtime-support.mjs';

const udid = process.argv[2] ?? simulatorUDID;
assert.match(udid, /^[0-9A-Fa-f-]{36}$/, '사용법: node capture-c11-2-1-grid-placement-ios-runtime-matrix.mjs [Simulator UDID]');
const runID = newRunID('iossim');
const evidenceDirectory = await createEvidenceDirectory('ios-simulator', runID);
const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function bootedSimulator() {
  const state = JSON.parse(simctl(['list', 'devices', '--json']).stdout);
  const matches = Object.entries(state.devices)
    .flatMap(([runtime, devices]) => devices
      .filter((device) => device.udid === udid)
      .map((device) => ({ runtime, ...device })));
  assert.equal(matches.length, 1, `Simulator UDID를 찾을 수 없습니다: ${udid}`);
  const device = matches[0];
  assert.equal(device.state, 'Booted', `Simulator가 Booted 상태가 아닙니다: ${device.state}`);
  assert.equal(device.isAvailable, true, 'Simulator runtime을 사용할 수 없습니다.');
  assert.match(device.runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `계획에서 고정한 iOS 26.2가 아닙니다: ${device.runtime}`);
  return device;
}

function processLog(pid) {
  const output = simctl([
    'spawn', udid, 'log', 'show', '--last', '5m', '--style', 'syslog',
    '--predicate', 'process == "SpinonBootstrap"',
  ], { timeout: 45_000, maxBuffer: 96 * 1024 * 1024 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

function runningAppPid() {
  const output = simctl(['spawn', udid, 'launchctl', 'list'], { timeout: 20_000 }).stdout;
  const line = output.split(/\r?\n/).find((value) =>
    value.includes(`UIKitApplication:${packageName}[`));
  const pid = line?.match(/^(\d+)\s/)?.[1];
  return pid ? Number(pid) : null;
}

function waitFor(predicate, description, timeout = 120_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(400);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function pixelSamplePayload(scene) {
  return Buffer.from(JSON.stringify(scene.samples)).toString('base64');
}

async function main() {
  const inputs = await loadFixedInputs();
  const { matrix: pixelMatrix } = await loadPixelMatrix(inputs);
  const pixelCases = new Map(pixelMatrix.cases.map((scene) => [scene.id, scene]));
  const device = bootedSimulator();
  const deviceEvidence = { ...device, udid,
    renderer: 'WGPU Metal drawable texture readback before compositor submission' };
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(deviceEvidence, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1', 'bun', 'run', 'build:ios-sim',
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const buildOutput = `${build.stdout}${build.stderr}`;
  assert.match(buildOutput,
    /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c11_grid_metal_surface_pixels=1/,
    'iOS simulator 빌드가 C11 Metal surface pixel flag를 받지 않았습니다.');
  assert.match(buildOutput,
    /SPINON_IOS_NATIVE_FEATURES ffi=[^\n]*c11-grid-metal-surface-pixels-test[^\n]*c11_grid_cpp=-DSPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1/,
    'iOS native 빌드에 Grid FFI feature 또는 C++ define이 전달되지 않았습니다.');
  const appInfo = resolve(simulatorAppPath, 'Info.plist');
  const executableName = command('/usr/libexec/PlistBuddy',
    ['-c', 'Print CFBundleExecutable', appInfo]).stdout.trim();
  const executablePath = resolve(simulatorAppPath, executableName);
  const executableBytes = await readFile(executablePath);
  const executableSha256 = sha256(executableBytes);
  const install = simctl(['install', udid, simulatorAppPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);

  const observations = [];
  for (const fixture of inputs.matrix.cases) {
    const sceneRunID = `${runID}-${fixture.id}`;
    const pixelFixture = pixelCases.get(fixture.id);
    assert.ok(pixelFixture, `${fixture.id}: surface pixel matrix case 없음`);
    const capturesPixels = pixelFixture.outcome === 'surface-pixel-compare';
    const termination = simctl(['terminate', udid, packageName], {
      timeout: 30_000, allowFailure: true,
    });
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-terminate.log`),
      `${termination.stdout}${termination.stderr}`);
    waitFor(() => runningAppPid() === null, `${fixture.id}: 이전 app process 종료`, 30_000);
    let launchText = '';
    let recoveredPID = null;
    try {
      const launchArgs = [
        'launch', udid, packageName,
      '--spinon-c1121-grid-placement',
      `--spinon-c1121-grid-scene=${fixture.id}`,
      `--spinon-c1121-grid-run-id=${sceneRunID}`,
      ];
      if (capturesPixels) {
        launchArgs.push(`--spinon-c11-grid-case=${fixture.id}`,
          `--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
          `--spinon-c11-grid-metal-surface-samples=${pixelSamplePayload(pixelFixture)}`);
      }
      const launch = simctl(launchArgs, { timeout: 90_000 });
      launchText = `${launch.stdout}${launch.stderr}`;
    } catch (error) {
      recoveredPID = runningAppPid();
      if (!recoveredPID) throw error;
      launchText = `simctl launch 응답 시간 초과 후 새 process를 확인했습니다: pid=${recoveredPID}\n${error.message}\n`;
    }
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-launch.log`), launchText);
    const pidMatch = launchText.match(new RegExp(`${packageName.replaceAll('.', '\\.')}[:,]\\s+(\\d+)`));
    const pid = Number(pidMatch?.[1] ?? recoveredPID ?? runningAppPid());
    assert.ok(Number.isInteger(pid) && pid > 0,
      `${fixture.id}: simctl launch 후 새 process ID를 찾지 못했습니다: ${launchText}`);
    const marker = `SPINON_C1121_GRID_SUMMARY run_id=${sceneRunID} scene_id=${fixture.id}`;
    const isSuccess = fixture.expected.outcome === 'success';
    let lines = [];
    waitFor(() => {
      lines = processLog(pid);
      const summaryIndex = lines.findIndex((line) => line.includes(marker));
      const expectedBoxes = fixture.nodeIds.length + 1;
      const drawReady = lines.some((line, index) => index > summaryIndex
        && line.includes('SPINON_C0410_DRAW')
        && line.includes(`presented boxes=${expectedBoxes}`));
      const pixelReady = !capturesPixels || lines.some((line) =>
        line.includes('SPINON_C111_METAL_PIXEL_SUMMARY status=matched')
          && line.includes(`run_id=${sceneRunID}`));
      const pixelFailed = capturesPixels && lines.some((line) =>
        line.includes(`SPINON_C111_METAL_PIXEL_ERROR run_id=${sceneRunID}`)
          || line.includes(`SPINON_C111_METAL_PIXEL_SUMMARY status=mismatch run_id=${sceneRunID}`));
      if (pixelFailed) throw new Error(`${fixture.id}: Metal surface pixel readback 실패\n${lines.join('\n')}`);
      return summaryIndex >= 0 && (!isSuccess || drawReady) && pixelReady;
    }, `${fixture.id}: process-filtered runtime summary와 WGPU draw`);
    if (!isSuccess) sleep(750);
    lines = processLog(pid);
    const screenshotPath = resolve(evidenceDirectory, `${fixture.id}-ios-simulator.png`);
    const screenshot = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-screenshot.log`), screenshot.stdout + screenshot.stderr);
    const screenshotBytes = await readFile(screenshotPath);
    const result = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase: fixture,
      chromeCase: referenceCase(inputs.referenceCases, fixture.id),
      lines,
      screenshotBytes,
      runID: sceneRunID,
      platform: 'ios-simulator',
    });
    if (capturesPixels) {
      const surfacePixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
        fixture: { sceneId: fixture.id, runtime: pixelMatrix.runtime, samples: pixelFixture.samples },
        runID: sceneRunID,
      });
      await writeFile(resolve(evidenceDirectory, `${fixture.id}-surface-pixel-comparison.json`),
        `${JSON.stringify(surfacePixelComparison, null, 2)}\n`);
      result.surfacePixelComparison = surfacePixelComparison;
      process.stdout.write(`  Metal drawable: ${surfacePixelComparison.matchedSamples}/${surfacePixelComparison.expectedSamples} exact\n`);
    } else {
      assert.equal(pixelFixture.expected.outcome, 'expected-rejection');
      assert.equal(lines.some((line) => line.includes(`SPINON_C111_METAL_PIXEL_BEGIN run_id=${sceneRunID}`)), false,
        `${fixture.id}: 거부 장면에서 Metal pixel readback이 시작되었습니다.`);
    }
    result.runID = sceneRunID;
    result.processId = pid;
    observations.push(result);
    process.stdout.write(`${fixture.id}: ${result.outcome} · pid=${pid}\n`);
  }

  const invalidSceneID = 'deferred-implicit-track';
  const invalidRunID = `${runID}-pixel-allowlist-negative`;
  const termination = simctl(['terminate', udid, packageName], {
    timeout: 30_000, allowFailure: true,
  });
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-terminate.log'),
    `${termination.stdout}${termination.stderr}`);
  waitFor(() => runningAppPid() === null, '지원 밖 pixel scene 이전 app process 종료', 30_000);
  const invalidLaunch = simctl([
    'launch', udid, packageName,
    '--spinon-c1121-grid-placement',
    `--spinon-c1121-grid-scene=${invalidSceneID}`,
    `--spinon-c1121-grid-run-id=${invalidRunID}`,
    `--spinon-c11-grid-case=${invalidSceneID}`,
    `--spinon-c11-grid-metal-surface-pixels=${invalidRunID}`,
    `--spinon-c11-grid-metal-surface-samples=${pixelSamplePayload(pixelCases.get('numeric-lines'))}`,
  ], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-launch.log'),
    invalidLaunch.stdout + invalidLaunch.stderr);
  const invalidPID = Number(invalidLaunch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]
    ?? runningAppPid());
  assert.ok(Number.isInteger(invalidPID) && invalidPID > 0,
    '지원 밖 pixel scene negative control 새 process ID 없음');
  const invalidLines = waitFor(() => {
    const lines = processLog(invalidPID);
    return lines.some((line) => line.includes('SPINON_C111_METAL_PIXEL_ERROR reason=invalid-run-scene-or-samples'))
      ? lines : null;
  }, '지원 밖 pixel scene Metal parser 거부 marker');
  assert.equal(invalidLines.some((line) => line.includes(`SPINON_C111_METAL_PIXEL_BEGIN run_id=${invalidRunID}`)), false);
  assert.equal(invalidLines.some((line) => line.includes(`SPINON_C111_METAL_PIXEL_SUMMARY status=matched run_id=${invalidRunID}`)), false);
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.log'),
    `${invalidLines.join('\n')}\n`);
  const pixelIntentNegativeControl = {
    status: 'rejected', runID: invalidRunID, processId: invalidPID,
    requestedScene: invalidSceneID, expectedScene: 'numeric-lines',
    argumentRejected: true, metalReadbackStarted: false,
  };
  await writeFile(resolve(evidenceDirectory, 'pixel-allowlist-negative-control.json'),
    `${JSON.stringify(pixelIntentNegativeControl, null, 2)}\n`);

  const summary = await writeMatrixSummary({
    directory: evidenceDirectory,
    platform: `${device.runtime.replace('com.apple.CoreSimulator.SimRuntime.', '').replaceAll('-', '.')} Simulator (${device.name})`,
    runID,
    target: `${device.name} · ${device.runtime} · ${udid}`,
    appSha256: executableSha256,
    device: deviceEvidence,
    inputs,
    pixelMatrix,
    pixelIntentNegativeControl,
    observations,
  });
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID,
    platform: summary.platform, executableSha256, summary: summary.summary,
    evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
