import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  command, createEvidenceDirectory, loadC1122Inputs, loadC1122PixelMatrix, newRunID, packageName,
  repositoryRoot, saveSceneEvidence, sha256, simulatorAppPath, simulatorUDID,
  writeRuntimeSummary,
} from './c11-2-2-grid-auto-placement-runtime-support.mjs';
import { compareC1122GridAutoPlacementScene } from './c11-2-2-grid-auto-placement-runtime-compare.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';

const udid = process.argv[2] ?? simulatorUDID;
assert.match(udid, /^[0-9A-Fa-f-]{36}$/,
  '사용법: node capture-c11-2-2-grid-auto-placement-ios-runtime-matrix.mjs [Simulator UDID]');
const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
const runID = newRunID('ios-simulator');
const evidenceDirectory = await createEvidenceDirectory('ios-simulator', runID);

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
    `검증 대상으로 고정한 iOS 26.2가 아닙니다: ${device.runtime}`);
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
  const inputs = await loadC1122Inputs();
  const pixelMatrix = await loadC1122PixelMatrix(inputs);
  const pixelCases = new Map(pixelMatrix.matrix.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
  const device = bootedSimulator();
  const deviceEvidence = {
    ...device,
    udid,
    renderer: 'WGPU Metal drawable; pixel matrix uses internal drawable readback, screenshot is supplementary',
  };
  await writeFile(resolve(evidenceDirectory, 'device.json'),
    `${JSON.stringify(deviceEvidence, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1', 'bun', 'run', 'build:ios-sim',
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  const appInfo = resolve(simulatorAppPath, 'Info.plist');
  const executableName = command('/usr/libexec/PlistBuddy',
    ['-c', 'Print CFBundleExecutable', appInfo]).stdout.trim();
  const executableBytes = await readFile(resolve(simulatorAppPath, executableName));
  const appSha256 = sha256(executableBytes);
  const install = simctl(['install', udid, simulatorAppPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const observations = [];
  const pixelObservations = [];
  for (const fixtureCase of inputs.matrix.cases) {
    const sceneRunID = `${runID}-${fixtureCase.id}`;
    const isSuccess = fixtureCase.expected.outcome === 'success';
    const pixelScene = pixelCases.get(fixtureCase.id);
    simctl(['terminate', udid, packageName], { allowFailure: true, timeout: 30_000 });
    const launchArgs = [
      'launch', '--terminate-running-process', udid, packageName,
      '--spinon-c1122-grid-auto-placement',
      `--spinon-c1122-grid-scene=${fixtureCase.id}`,
      `--spinon-c1122-grid-run-id=${sceneRunID}`,
    ];
    if (isSuccess) {
      assert.ok(pixelScene && pixelScene.samples.length > 0,
        `${fixtureCase.id}: 성공 scene의 surface pixel fixture가 없습니다.`);
      const payload = Buffer.from(JSON.stringify(pixelScene.samples.map((sample) => ({
        id: sample.id,
        nodeId: sample.nodeId,
        xCssPx: sample.xCssPx,
        yCssPx: sample.yCssPx,
        rgba: sample.rgba,
      })))).toString('base64');
      launchArgs.push(
        `--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
        `--spinon-c11-grid-metal-surface-samples=${payload}`,
      );
    }
    const launch = simctl(launchArgs, { timeout: 90_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`),
      `${launch.stdout}${launch.stderr}`);
    const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
    assert.ok(Number.isInteger(pid) && pid > 0,
      `${fixtureCase.id}: Simulator launch가 새 PID를 반환하지 않았습니다.`);
    const marker = isSuccess
      ? 'SPINON_C1122_GRID_FRAME_SUMMARY' : 'SPINON_C1122_GRID_DIAGNOSTIC';
    const lines = waitFor(() => {
      const current = processLog(pid);
      const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
        && !line.includes('status=0 ') && !line.includes('status=-12 '));
      if (failedDraw) throw new Error(`${fixtureCase.id}: iOS draw 실패: ${failedDraw}`);
      const hasResult = current.some((line) => line.includes(marker)
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
      const hasDraw = current.some((line) => line.includes('SPINON_C0410_DRAW')
        && /presented boxes=\d+/.test(line));
      const hasPixelReadback = !isSuccess || current.some((line) =>
        line.includes('SPINON_C1122_METAL_PIXEL_SUMMARY status=matched')
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene=${fixtureCase.id}`));
      const failedPixelReadback = isSuccess && current.find((line) =>
        line.includes('SPINON_C1122_METAL_PIXEL_SUMMARY')
        && line.includes(`run_id=${sceneRunID}`) && !line.includes('status=matched'));
      if (failedPixelReadback) {
        throw new Error(`${fixtureCase.id}: iOS Metal surface 표본 실패: ${failedPixelReadback}`);
      }
      return hasResult && (!isSuccess || hasDraw) && hasPixelReadback ? current : null;
    }, `${fixtureCase.id}: runtime 결과와 draw`);
    const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.id}-ios-simulator.png`);
    const screenshotLog = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-screenshot.log`),
      `${screenshotLog.stdout}${screenshotLog.stderr}`);
    const screenshotBytes = await readFile(screenshotPath);
    const comparison = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase,
      chromeCase: inputs.referenceCases.find(({ id }) => id === fixtureCase.id),
      lines,
      screenshotBytes,
      runID: sceneRunID,
      platform: 'ios-simulator',
      compareScene: compareC1122GridAutoPlacementScene,
    });
    if (isSuccess) {
      const pixelFixture = {
        fixtureId: pixelMatrix.matrix.fixtureId,
        sceneId: fixtureCase.id,
        runtime: pixelMatrix.matrix.runtime,
        samples: pixelScene.samples,
      };
      const pixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
        fixture: pixelFixture,
        runID: sceneRunID,
        markerPrefix: 'C1122',
      });
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-pixel-comparison.json`),
        `${JSON.stringify(pixelComparison, null, 2)}\n`);
      comparison.surfacePixelComparison = pixelComparison;
      pixelObservations.push(pixelComparison);
    } else {
      assert.ok(!lines.some((line) => line.includes('SPINON_C1122_METAL_PIXEL_BEGIN')
        && line.includes(`run_id=${sceneRunID}`)),
      `${fixtureCase.id}: expected rejection scene이 Metal readback을 시작했습니다.`);
    }
    comparison.runID = sceneRunID;
    comparison.processId = pid;
    observations.push(comparison);
    process.stdout.write(`${fixtureCase.id}: ${comparison.outcome} · pid=${pid}\n`);
  }

  const summary = await writeRuntimeSummary({
    directory: evidenceDirectory,
    platform: `iOS Simulator (${device.name}, iOS 26.2)`,
    runID,
    target: `${device.name} · ${device.runtime} · ${udid}`,
    device: deviceEvidence,
    appSha256,
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
