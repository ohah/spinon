import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  androidApkPath, androidComponent, command, createEvidenceDirectory, loadC1123Inputs,
  loadC1123PixelMatrix, newRunID, packageName, repositoryRoot, saveSceneEvidence, sha256,
  simulatorAppPath, simulatorUDID, writeRuntimeSummary,
} from './c11-2-3-grid-auto-placement-runtime-support.mjs';
import { compareC1123GridAutoPlacementScene } from './c11-2-3-grid-auto-placement-runtime-compare.mjs';
import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import { compareC11IOSMetalPixelLogs } from './compare-c11-ios-metal-surface-pixels.mjs';

function defaultAndroidPhysicalSerial() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => /^\S+\s+device\s/.test(line));
  const physical = devices.filter((line) => {
    const serial = line.split(/\s+/)[0];
    return command('adb', ['-s', serial, 'shell', 'getprop', 'ro.kernel.qemu'],
      { allowFailure: true }).stdout.trim() !== '1';
  }).map((line) => line.split(/\s+/)[0]);
  assert.equal(physical.length, 1,
    `Android 실기기 serial을 하나로 결정할 수 없습니다. 연결된 실기기: ${physical.join(', ') || '없음'}`);
  return physical[0];
}

const target = process.argv[2];
assert.ok(target === 'android' || target === 'ios',
  '사용법: node capture-c11-2-3-grid-auto-placement-runtime-matrix.mjs <android|ios> [serial 또는 Simulator UDID] [--surface-pixels]');
const requestedSerial = process.argv.slice(3).find((value) => !value.startsWith('--'));
const serialOrUDID = requestedSerial ?? (target === 'ios' ? simulatorUDID : defaultAndroidPhysicalSerial());
const surfacePixelsRequested = process.argv.includes('--surface-pixels');
assert.match(serialOrUDID, target === 'android' ? /^[A-Za-z0-9._:-]+$/ : /^[0-9A-Fa-f-]{36}$/,
  '대상 기기 식별자가 올바르지 않습니다.');
const runID = newRunID(target === 'android' ? 'android-physical' : 'ios-simulator');
const platform = target === 'android' ? 'android-physical' : 'ios-simulator';
const evidenceDirectory = await createEvidenceDirectory(platform, runID);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function waitFor(predicate, description, timeoutMillis) {
  const deadline = Date.now() + timeoutMillis;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(300);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function androidTarget() {
  const adb = (args, options) => command('adb', ['-s', serialOrUDID, ...args], options);
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serialOrUDID} `) && line.includes(' device')),
    `ADB 대상이 연결되지 않았거나 device 상태가 아닙니다: ${serialOrUDID}`);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  assert.notEqual(qemu, '1', `Android 실기기 대상에 AVD가 선택됐습니다: ${serialOrUDID}`);
  const release = getprop('ro.build.version.release');
  const sdk = Number(getprop('ro.build.version.sdk'));
  const model = getprop('ro.product.model');
  const device = {
    serial: serialOrUDID, qemu, release, sdk, model,
    fingerprint: getprop('ro.build.fingerprint'),
    density: adb(['shell', 'wm', 'density']).stdout.trim(),
    size: adb(['shell', 'wm', 'size']).stdout.trim(),
    platform: `Android ${release}/API ${sdk} 실기기 (${model})`,
  };
  return { adb, device };
}

function iosTarget() {
  const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
  const state = JSON.parse(simctl(['list', 'devices', '--json']).stdout);
  const matches = Object.entries(state.devices).flatMap(([runtime, devices]) => devices
    .filter((device) => device.udid === serialOrUDID)
    .map((device) => ({ runtime, ...device })));
  assert.equal(matches.length, 1, `iOS Simulator UDID가 없습니다: ${serialOrUDID}`);
  assert.equal(matches[0].state, 'Booted', `iOS Simulator가 Booted 상태가 아닙니다.`);
  assert.equal(matches[0].isAvailable, true, 'iOS Simulator runtime을 사용할 수 없습니다.');
  assert.match(matches[0].runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `검증 대상으로 고정한 iOS 26.2가 아닙니다: ${matches[0].runtime}`);
  return {
    simctl,
      device: { ...matches[0], udid: serialOrUDID,
      renderer: surfacePixelsRequested
        ? 'WGPU Metal drawable 직접 readback'
        : 'WGPU Metal drawable; surface pixel readback 미수행' },
  };
}

function androidProcessLog(adb, pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime'], {
    timeout: 30_000, maxBuffer: 96 * 1024 * 1024,
  }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const fields = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return fields && Number(fields[1]) === pid;
  });
}

function iosProcessLog(simctl, pid) {
  const output = simctl([
    'spawn', serialOrUDID, 'log', 'show', '--last', '3m', '--style', 'syslog',
    '--predicate', 'process == "SpinonBootstrap"',
  ], { timeout: 45_000, maxBuffer: 96 * 1024 * 1024 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

async function main() {
  const inputs = await loadC1123Inputs();
  const pixelMatrix = surfacePixelsRequested ? await loadC1123PixelMatrix(inputs) : null;
  const pixelCases = new Map(pixelMatrix?.matrix.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]) ?? []);
  const { device, adb, simctl } = target === 'android' ? androidTarget() : iosTarget();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
  const build = command('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1',
    ...(target === 'ios' && surfacePixelsRequested
      ? ['SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1'] : []),
    ...(target === 'ios' ? ['bun', 'run', 'build:ios-sim'] : ['bun', 'run', 'build:android']),
  ], { timeout: 900_000, maxBuffer: 96 * 1024 * 1024 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);

  let appSha256;
  if (target === 'android') {
    appSha256 = sha256(await readFile(androidApkPath));
    const install = adb(['install', '-r', androidApkPath], { timeout: 180_000 });
    await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);
  } else {
    const appInfo = resolve(simulatorAppPath, 'Info.plist');
    const executableName = command('/usr/libexec/PlistBuddy',
      ['-c', 'Print CFBundleExecutable', appInfo]).stdout.trim();
    appSha256 = sha256(await readFile(resolve(simulatorAppPath, executableName)));
    const install = simctl(['install', serialOrUDID, simulatorAppPath], { timeout: 120_000 });
    await writeFile(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);
  }

  const observations = [];
  const pixelObservations = [];
  for (const fixtureCase of inputs.matrix.cases) {
    const sceneRunID = `${runID}-${fixtureCase.id}`;
    const isSuccess = fixtureCase.expected.outcome === 'success';
    const pixelScene = pixelCases.get(fixtureCase.id);
    let pid;
    let launch;
    if (target === 'android') {
      adb(['shell', 'am', 'force-stop', packageName]);
      adb(['logcat', '-c']);
      const launchArgs = [
        'shell', 'am', 'start', '-n', androidComponent,
        '--es', 'spinon_c1123_grid_auto_placement_scene', fixtureCase.id,
        '--es', 'spinon_c1123_grid_auto_placement_run_id', sceneRunID,
      ];
      if (surfacePixelsRequested && isSuccess) {
        assert.ok(pixelScene?.samples.length > 0,
          `${fixtureCase.id}: 성공 scene의 pixel fixture가 없습니다.`);
        const payload = Buffer.from(JSON.stringify(pixelScene.samples.map(({ id, nodeId, xCssPx, yCssPx }) => ({
          id, nodeId, xCssPx, yCssPx,
        })))).toString('base64url');
        launchArgs.push('--es', 'spinon_c11_grid_case', fixtureCase.id,
          '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
          '--es', 'spinon_c11_grid_pixel_matrix_samples', payload);
      }
      launch = adb(launchArgs, { timeout: 30_000 });
      pid = waitFor(() => {
        const result = adb(['shell', 'pidof', packageName], { allowFailure: true })
          .stdout.trim().split(/\s+/).filter(Boolean).map(Number)[0];
        return Number.isInteger(result) && result > 0 ? result : null;
      }, `${fixtureCase.id}: Android app process`, 90_000);
    } else {
      simctl(['terminate', serialOrUDID, packageName], { allowFailure: true, timeout: 30_000 });
      const launchArgs = [
        'launch', '--terminate-running-process', serialOrUDID, packageName,
        '--spinon-c1123-grid-auto-placement',
        `--spinon-c1123-grid-scene=${fixtureCase.id}`,
        `--spinon-c1123-grid-run-id=${sceneRunID}`,
      ];
      if (surfacePixelsRequested && isSuccess) {
        assert.ok(pixelScene?.samples.length > 0,
          `${fixtureCase.id}: 성공 scene의 pixel fixture가 없습니다.`);
        const payload = Buffer.from(JSON.stringify(pixelScene.samples.map(({ id, nodeId, xCssPx, yCssPx, rgba }) => ({
          id, nodeId, xCssPx, yCssPx, rgba,
        })))).toString('base64');
        launchArgs.push(
          `--spinon-c11-grid-metal-surface-pixels=${sceneRunID}`,
          `--spinon-c11-grid-metal-surface-samples=${payload}`,
        );
      }
      launch = simctl(launchArgs, { timeout: 90_000 });
      pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0,
        `${fixtureCase.id}: Simulator launch가 새 PID를 반환하지 않았습니다.`);
    }
    await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-launch.log`),
      `${launch.stdout}${launch.stderr}`);

    const resultMarker = isSuccess
      ? 'SPINON_C1123_GRID_FRAME_SUMMARY' : 'SPINON_C1123_GRID_DIAGNOSTIC';
    const lines = waitFor(() => {
      const current = target === 'android'
        ? androidProcessLog(adb, pid) : iosProcessLog(simctl, pid);
      const failedDraw = current.find((line) => line.includes('SPINON_C0410_DRAW generation=')
        && !line.includes('status=0 ') && !line.includes('status=-12 '));
      if (failedDraw) throw new Error(`${fixtureCase.id}: runtime draw 실패: ${failedDraw}`);
      const hasResult = current.some((line) => line.includes(resultMarker)
        && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene_id=${fixtureCase.id}`));
      const hasDraw = current.some((line) => line.includes('SPINON_C0410_DRAW')
        && /presented boxes=\d+/.test(line));
      const pixelSummaryMarker = target === 'android'
        ? 'SPINON_C1123_GRID_PIXEL_MATRIX_SUMMARY' : 'SPINON_C1123_METAL_PIXEL_SUMMARY';
      const pixelSuccessStatus = target === 'android' ? 'status=captured' : 'status=matched';
      const hasPixelReadback = !surfacePixelsRequested || !isSuccess || current.some((line) =>
        line.includes(pixelSummaryMarker) && line.includes(pixelSuccessStatus)
          && line.includes(`run_id=${sceneRunID}`) && line.includes(`scene=${fixtureCase.id}`));
      const failedPixelReadback = surfacePixelsRequested && isSuccess && current.find((line) =>
        line.includes(pixelSummaryMarker) && line.includes(`run_id=${sceneRunID}`)
          && !line.includes(pixelSuccessStatus));
      if (failedPixelReadback) {
        throw new Error(`${fixtureCase.id}: surface pixel capture 실패: ${failedPixelReadback}`);
      }
      return hasResult && (!isSuccess || hasDraw) && hasPixelReadback ? current : null;
    }, `${fixtureCase.id}: runtime 결과와 draw`, target === 'android' ? 90_000 : 120_000);

    let screenshotBytes;
    if (target === 'android') {
      screenshotBytes = adb(['exec-out', 'screencap', '-p'], {
        encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
      }).stdout;
    } else {
      const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.id}-ios-simulator.png`);
      const capture = simctl(['io', serialOrUDID, 'screenshot', screenshotPath], { timeout: 30_000 });
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-screenshot.log`),
        `${capture.stdout}${capture.stderr}`);
      screenshotBytes = await readFile(screenshotPath);
    }
    const comparison = await saveSceneEvidence({
      directory: evidenceDirectory,
      fixtureCase,
      chromeCase: inputs.referenceCases.find(({ id }) => id === fixtureCase.id),
      lines,
      screenshotBytes,
      runID: sceneRunID,
      platform,
      compareScene: compareC1123GridAutoPlacementScene,
    });
    if (surfacePixelsRequested && isSuccess) {
      let pixelComparison;
      if (target === 'android') {
        const captureLine = lines.find((line) =>
          line.includes('SPINON_C1123_GRID_PIXEL_MATRIX_CAPTURE status=ok')
            && line.includes(`run_id=${sceneRunID}`));
        assert.ok(captureLine, `${fixtureCase.id}: PixelCopy success marker가 없습니다.`);
        const devicePng = captureLine.match(/\bpng=(\S+)/)?.[1];
        assert.ok(devicePng, `${fixtureCase.id}: PixelCopy PNG 경로가 없습니다.`);
        const pixelPng = adb(['exec-out', 'run-as', packageName, 'cat', devicePng], {
          encoding: 'buffer', timeout: 30_000, maxBuffer: 32 * 1024 * 1024,
        }).stdout;
        pixelComparison = compareC11AndroidPixelCopyScene(lines.join('\n'), {
          scene: pixelScene,
          runID: sceneRunID,
          pngBytes: pixelPng,
          viewportCssPx: pixelMatrix.matrix.runtime.viewportCssPx,
          markerPrefix: 'C1123',
          cacheDirectory: 'c1123-grid-surface-pixel-matrix',
        });
        await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-surface-pixels.png`), pixelPng);
      } else {
        pixelComparison = compareC11IOSMetalPixelLogs(lines.join('\n'), {
          fixture: {
            fixtureId: pixelMatrix.matrix.fixtureId,
            sceneId: fixtureCase.id,
            runtime: pixelMatrix.matrix.runtime,
            samples: pixelScene.samples,
          },
          runID: sceneRunID,
          markerPrefix: 'C1123',
        });
      }
      await writeFile(resolve(evidenceDirectory, `${fixtureCase.id}-pixel-comparison.json`),
        `${JSON.stringify(pixelComparison, null, 2)}\n`);
      comparison.surfacePixelComparison = pixelComparison;
      pixelObservations.push({ ...pixelComparison, scene: fixtureCase.id });
    } else if (surfacePixelsRequested) {
      const pixelBeginMarker = target === 'android'
        ? 'SPINON_C1123_GRID_PIXEL_MATRIX_BEGIN' : 'SPINON_C1123_METAL_PIXEL_BEGIN';
      assert.ok(!lines.some((line) => line.includes(pixelBeginMarker)
        && line.includes(`run_id=${sceneRunID}`)),
      `${fixtureCase.id}: expected rejection이 surface readback을 시작했습니다.`);
    }
    comparison.runID = sceneRunID;
    comparison.processId = pid;
    observations.push(comparison);
    process.stdout.write(`${fixtureCase.id}: ${comparison.outcome} · pid=${pid}\n`);
  }

  const summary = await writeRuntimeSummary({
    directory: evidenceDirectory,
    platform: target === 'android' ? 'Android physical device' : `iOS Simulator (${device.name}, iOS 26.2)`,
    runID,
    target: target === 'android'
      ? `${serialOrUDID} · Android ${device.release}/API ${device.sdk} · ${device.model}`
      : `${serialOrUDID} · ${device.name} · iOS 26.2`,
    device,
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
