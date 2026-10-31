import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';
import {
  command, compareOrderScene, createOrderEvidenceDirectory, loadC1122GridItemOrderInputs,
  newOrderRunID, packageName, repositoryRoot, saveSceneEvidence, sha256,
  simulatorAppPath, simulatorUDID, writeOrderRuntimeSummary,
} from './c11-2-2-grid-item-order-runtime-support.mjs';

const udid = process.argv[2] ?? simulatorUDID;
assert.match(udid, /^[0-9A-Fa-f-]{36}$/,
  '사용법: node capture-c11-2-2-grid-item-order-ios-simulator.mjs [Simulator UDID]');
const runID = newOrderRunID('ios-simulator');
const evidenceDirectory = await createOrderEvidenceDirectory('ios-simulator', runID);
const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function bootedSimulator() {
  const state = JSON.parse(simctl(['list', 'devices', '--json']).stdout);
  const matches = Object.entries(state.devices)
    .flatMap(([runtime, devices]) => devices
      .filter((device) => device.udid === udid)
      .map((device) => ({ runtime, ...device })));
  assert.equal(matches.length, 1, `Simulator UDID가 없습니다: ${udid}`);
  const device = matches[0];
  assert.equal(device.state, 'Booted', `Simulator가 Booted 상태가 아닙니다: ${device.state}`);
  assert.equal(device.isAvailable, true, 'Simulator runtime을 사용할 수 없습니다.');
  assert.match(device.runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `iOS 26.2가 아닙니다: ${device.runtime}`);
  assert.equal(device.name, 'iPhone 17 Pro', `iPhone 17 Pro가 아닙니다: ${device.name}`);
  return device;
}

function processLog(pid) {
  const output = simctl([
    'spawn', udid, 'log', 'show', '--last', '3m', '--style', 'syslog',
    '--predicate', 'process == "SpinonBootstrap"',
  ], { timeout: 45_000, maxBuffer: 96 * 1024 * 1024 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

function waitFor(predicate, description, timeout = 120_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(350);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

async function main() {
  const inputs = await loadC1122GridItemOrderInputs();
  const device = bootedSimulator();
  const deviceEvidence = {
    ...device,
    udid,
    renderer: 'WGPU Metal drawable; 표본은 compositor 제출 전 drawable readback',
  };
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(deviceEvidence, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1', 'bun', 'run', 'build:ios-sim',
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const buildOutput = `${build.stdout}${build.stderr}`;
  assert.match(buildOutput, /c11_grid_metal_surface_pixels=1/,
    'iOS simulator build에 Metal pixel fixture flag가 없습니다.');
  const appInfo = resolve(simulatorAppPath, 'Info.plist');
  const executableName = command('/usr/libexec/PlistBuddy',
    ['-c', 'Print CFBundleExecutable', appInfo]).stdout.trim();
  const appSha256 = sha256(await readFile(resolve(simulatorAppPath, executableName)));
  const install = simctl(['install', udid, simulatorAppPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const observations = [];
  const pixelObservations = [];
  for (const fixtureCase of inputs.runtimeMatrix.cases) {
    const sceneRunID = `${runID}-${fixtureCase.id}`;
    const pixelCase = inputs.pixelMatrix.cases.find(({ id }) => id === fixtureCase.id);
    assert.ok(pixelCase, `${fixtureCase.id}: pixel scene 누락`);
    const samples = Buffer.from(JSON.stringify(pixelCase.samples)).toString('base64');
    const launch = simctl([
      'launch', '--terminate-running-process', udid, packageName,
      '--spinon-c1122-grid-auto-placement',
      `--spinon-c1122-grid-scene=${fixtureCase.id}`,
      `--spinon-c1122-grid-run-id=${sceneRunID}`,
      `--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
      `--spinon-c11-grid-metal-surface-samples=${samples}`,
    ], { timeout: 90_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`), `${launch.stdout}${launch.stderr}`);
    const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
    assert.ok(Number.isInteger(pid) && pid > 0, `${fixtureCase.id}: 새 Simulator app PID 누락`);
    const lines = waitFor(() => {
      const current = processLog(pid);
      const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
        && !line.includes('status=0 ') && !line.includes('status=-12 '));
      if (failedDraw) throw new Error(`${fixtureCase.id}: iOS draw 실패: ${failedDraw}`);
      const hasSummary = current.some((line) => line.includes('SPINON_C1122_GRID_SUMMARY ')
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
      const hasDraw = current.some((line) => line.includes('SPINON_C0410_DRAW')
        && /presented boxes=\d+/.test(line));
      const hasPixelReadback = current.some((line) =>
        line.includes('SPINON_C1122_METAL_PIXEL_SUMMARY status=matched')
          && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene=${fixtureCase.id}`));
      const failedPixelReadback = current.find((line) =>
        line.includes('SPINON_C1122_METAL_PIXEL_SUMMARY')
          && line.includes(`run_id=${sceneRunID}`) && !line.includes('status=matched'));
      if (failedPixelReadback) throw new Error(`${fixtureCase.id}: Metal 표본 실패: ${failedPixelReadback}`);
      return hasSummary && hasDraw && hasPixelReadback ? current : null;
    }, `${fixtureCase.id}: runtime, draw, Metal 표본`);
    const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.id}-ios-simulator.png`);
    const screenshot = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-screenshot.log`),
      `${screenshot.stdout}${screenshot.stderr}`);
    const screenshotBytes = await readFile(screenshotPath);
    const comparison = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase,
      chromeCase: null,
      lines,
      screenshotBytes,
      runID: sceneRunID,
      platform: 'ios-simulator',
      compareScene: (options) => compareOrderScene({ ...options, fixtureCase }),
    });
    const pixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
      fixture: {
        fixtureId: inputs.pixelMatrix.fixtureId,
        sceneId: fixtureCase.id,
        runtime: inputs.pixelMatrix.runtime,
        samples: pixelCase.samples,
      },
      runID: sceneRunID,
      markerPrefix: 'C1122',
    });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-pixel-comparison.json`),
      `${JSON.stringify(pixelComparison, null, 2)}\n`);
    comparison.surfacePixelComparison = pixelComparison;
    comparison.runID = sceneRunID;
    comparison.processId = pid;
    observations.push(comparison);
    pixelObservations.push(pixelComparison);
    process.stdout.write(`${fixtureCase.id}: ${comparison.outcome} · ${pixelComparison.matchedSamples} Metal samples\n`);
  }
  const summary = await writeOrderRuntimeSummary({
    directory: evidenceDirectory,
    platform: 'iOS 26.2 iPhone 17 Pro Simulator',
    runID,
    target: `${device.name} · ${device.runtime} · ${udid}`,
    device: deviceEvidence,
    appSha256,
    inputs,
    observations,
    pixelObservations,
  });
  simctl(['terminate', udid, packageName], { allowFailure: true, timeout: 30_000 });
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  simctl(['terminate', udid, packageName], { allowFailure: true, timeout: 30_000 });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
