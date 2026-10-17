import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { createReadStream, existsSync } from 'node:fs';
import { access, copyFile, mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { compareMetalArtifactFiles } from './compare-c13-1-ios-metal-pixel-centers.mjs';
import { compareMetalFullSurfaceFiles } from './compare-c13-1-full-physical-pixels.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const fullSurfaceCapture = process.argv.includes('--full-physical-surface');
const captureArgument = fullSurfaceCapture
  ? '--spinon-c131-full-physical-metal-pixels'
  : '--spinon-c131-compact-metal-pixel-centers';
const expectedSampleCount = fullSurfaceCapture ? 270_000 : 30_000;
const expectedChannelCount = expectedSampleCount * 4;
const expectedBatchCount = Math.ceil(expectedSampleCount / 64);
const chromeFullObservationPath = resolve(repositoryRoot,
  'spec/internal/evidence/c13-1-full-physical-pixel-parity-2026-10-18/chrome155-dpr-3-observation.json');
const gitCommonDirectory = command('git', [
  'rev-parse', '--path-format=absolute', '--git-common-dir',
]).stdout.trim();
const localV8Directory = resolve(repositoryRoot, 'build/v8-source/v8');
const sharedV8Directory = resolve(dirname(gitCommonDirectory), 'build/v8-source/v8');
const v8Directory = process.env.SPINON_V8_DIR
  ?? (existsSync(localV8Directory) ? localV8Directory : sharedV8Directory);
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const requestedUDID = process.argv.slice(2).find((argument) => /^[0-9A-Fa-f-]{36}$/.test(argument))
  ?? expectedSimulatorUDID;
assert.match(requestedUDID, /^[0-9A-Fa-f-]{36}$/,
  '사용법: node capture-c13-1-ios-metal-pixel-centers.mjs [Simulator UDID]');
const timestamp = new Date().toISOString().replaceAll(/[^0-9TZ]/g, '');
const runID = `${fullSurfaceCapture ? 'c13-1-ios-full-physical' : 'c13-1-ios-metal'}-${timestamp}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/${fullSurfaceCapture ? 'c13-1-ios-full-physical-pixels' : 'c13-1-ios-metal-pixel-centers'}-${date}/${runID}`);
const identitySha256 = '93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87';
const gridOracleSha256 = '35e88e57edc16e526e389317b0d18ef227e35d1356a2353b92476c75ef6414f6';
const chromeObservationSha256 = '515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299';

function command(binary, args, { timeout = 60_000, allowFailure = false, maxBuffer = 96 * 1024 * 1024 } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding: 'utf8', timeout, maxBuffer,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr ?? result.stdout}`);
  }
  return result;
}

const simctl = (args, options) => command('xcrun', ['simctl', ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function sha256File(filePath) {
  return new Promise((resolveHash, reject) => {
    const hash = createHash('sha256');
    const stream = createReadStream(filePath);
    stream.on('data', (chunk) => hash.update(chunk));
    stream.on('error', reject);
    stream.on('end', () => resolveHash(hash.digest('hex')));
  });
}

function simulator() {
  const devices = JSON.parse(simctl(['list', 'devices', '--json']).stdout).devices;
  const matches = Object.entries(devices).flatMap(([runtime, items]) => items
    .filter((device) => device.udid === requestedUDID)
    .map((device) => ({ ...device, runtime })));
  assert.equal(matches.length, 1, `Simulator UDID를 찾을 수 없습니다: ${requestedUDID}`);
  assert.equal(matches[0].state, 'Booted', `Simulator가 Booted 상태가 아닙니다: ${matches[0].state}`);
  assert.equal(matches[0].isAvailable, true, 'Simulator runtime을 사용할 수 없습니다.');
  assert.equal(matches[0].udid, expectedSimulatorUDID,
    '고정한 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(matches[0].name, 'iPhone 17 Pro');
  assert.match(matches[0].runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `고정한 iOS 26.2가 아닙니다: ${matches[0].runtime}`);
  return matches[0];
}

function runningAppPID(udid) {
  const output = simctl(['spawn', udid, 'launchctl', 'list'], { timeout: 20_000 }).stdout;
  const line = output.split(/\r?\n/).find((value) => value.includes(`UIKitApplication:${bundleID}[`));
  const pid = line?.match(/^(\d+)\s/)?.[1];
  return pid ? Number(pid) : null;
}

function processLog(udid, pid) {
  const output = simctl([
    'spawn', udid, 'log', 'show', '--last', '10m', '--style', 'syslog',
    '--predicate', `processIdentifier == ${pid}`,
  ], { timeout: 50_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
}

function waitFor(predicate, description, timeout = 360_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(650);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

async function stopPreviousProcess(udid, label) {
  const termination = simctl(['terminate', udid, bundleID], {
    timeout: 30_000, allowFailure: true,
  });
  await writeFile(resolve(evidenceDirectory, `${label}-terminate.log`),
    `${termination.stdout}${termination.stderr}`);
  waitFor(() => runningAppPID(udid) === null, `${label}: 이전 앱 process 종료`, 30_000);
}

function launchPID(launchOutput) {
  const pid = Number(launchOutput.match(new RegExp(`${bundleID.replaceAll('.', '\\.')}[:,]\\s*(\\d+)`))?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0,
    `simctl launch 응답에서 PID를 찾지 못했습니다: ${launchOutput}`);
  return pid;
}

function hasLine(lines, fragment) {
  return lines.some((line) => line.includes(fragment));
}

function assertPixelBatchSequence(lines, appRunID) {
  const requests = lines.filter((line) => line.includes('SPINON_C131_METAL_PIXEL_BATCH_REQUESTED '));
  const batches = requests.map((line) => {
    assert.ok(line.includes(`run_id=${appRunID} `), 'pixel batch run ID correlation');
    const match = line.match(/batch=(\d+) count=(\d+) generation=(\d+) drawable=(\d+)x(\d+)/);
    assert.ok(match, `batch request format: ${line}`);
    return match.slice(1).map(Number);
  });
  assert.equal(batches.length, expectedBatchCount, 'Metal pixel batch request count');
  const finalBatchSize = expectedSampleCount - (expectedBatchCount - 1) * 64;
  for (let index = 0; index < batches.length; index += 1) {
    assert.deepEqual(batches[index].slice(0, 2),
      [index, index === expectedBatchCount - 1 ? finalBatchSize : 64],
      `Metal batch ${index} 순서와 크기`);
    assert.equal(batches[index][2], batches[0][2], 'batch surface generation consistency');
    assert.deepEqual(batches[index].slice(3), batches[0].slice(3), 'batch drawable size consistency');
  }
  const completedBatches = lines.filter((line) =>
    line.includes('SPINON_C131_METAL_PIXEL_BATCH_COMPLETE '));
  assert.equal(completedBatches.length, expectedBatchCount - 1, 'accepted non-final batch count');
  for (const [index, line] of completedBatches.entries()) {
    assert.ok(line.includes(`run_id=${appRunID} `), `batch completion ${index} run ID`);
    assert.ok(line.includes(`samples=${(index + 1) * 64}/${expectedSampleCount}`),
      `batch completion ${index} count`);
  }
  return { requests: requests.length, acceptedNonFinal: completedBatches.length };
}

async function captureSuccessfulRun(udid, index, priorPIDs) {
  const name = `run-${index}`;
  await stopPreviousProcess(udid, name);
  const launch = simctl([
    'launch', udid, bundleID,
    '--spinon-c131-compact-hit-identity-grid',
    captureArgument,
  ], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, `${name}-launch.log`), launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  assert.equal(priorPIDs.has(pid), false, `${name}: 새 앱 process PID가 이전 실행과 같습니다.`);

  let lines = [];
  const ready = waitFor(() => {
    lines = processLog(udid, pid);
    const gridLine = lines.find((line) => line.includes(
      'SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=matched'));
    const appRunID = gridLine?.match(/run_id=([0-9a-f]{32})/)?.[1];
    const layoutGeneration = gridLine?.match(/runtime_generation=(\d+)/)?.[1];
    const captureLine = lines.find((line) => line.includes(
      'SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE '));
    const capture = captureLine?.match(new RegExp(
      `run_id=([0-9a-f]{32}) generation=(\\d+) samples=${expectedSampleCount} batches=${expectedBatchCount} sample_file=([^ ]+) manifest_file=([^ ]+)`,
    ));
    const sameRun = appRunID && capture && capture[1] === appRunID;
    const evalReady = appRunID && hasLine(lines,
      `SPINON_C131_COMPACT_EVAL_RUN run_id=${appRunID} status=0 layout=ready boxes=41`);
    const fixedSamples = sameRun && hasLine(lines,
      `SPINON_C131_COMPACT_HIT_TEST_SUMMARY status=ok matched=52 total=52 ids=fixed run_id=${appRunID} runtime_generation=${layoutGeneration}`);
    const fixtureReady = sameRun && hasLine(lines,
      `SPINON_C131_COMPACT_FIXTURE_SUMMARY status=ok boxes=41 frames=41 hit_samples=52 hit_matches=52 run_id=${appRunID} runtime_generation=${layoutGeneration}`);
    const gridReady = gridLine?.includes('points=30000 matched=30000 mismatches=0 preview=0')
      && gridLine.includes(`expected_identity_sha256=${identitySha256}`)
      && gridLine.includes(`identity_sha256=${identitySha256}`)
      && gridLine.includes(`oracle_sha256=${gridOracleSha256}`)
      && gridLine.includes(`observation_sha256=${chromeObservationSha256}`);
    const frameLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_NODE_FRAME '));
    const hitLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_HIT_TEST id='));
    const correlatedFrames = frameLines.length === 41
      && frameLines.every((line) => line.includes(`run_id=${appRunID} runtime_generation=${layoutGeneration}`));
    const correlatedHits = hitLines.length === 52
      && hitLines.every((line) => line.includes(`run_id=${appRunID} runtime_generation=${layoutGeneration}`));
    const drawReady = lines.some((line) =>
      /SPINON_C0410_DRAW generation=\d+ status=0 boxes=41 surface_texture=\d+x\d+/.test(line));
    const failure = lines.find((line) => line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_ERROR ')
      || line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error')
      || line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=mismatch')
      || line.includes('SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error'));
    if (failure) throw new Error(`${name}: iOS Simulator 앱 검증 실패\n${lines.join('\n')}`);
    return sameRun && evalReady && fixedSamples && fixtureReady && gridReady
      && correlatedFrames && correlatedHits && drawReady && capture
      ? { appRunID, layoutGeneration: Number(layoutGeneration), surfaceGeneration: Number(capture[2]),
        sampleFile: capture[3], manifestFile: capture[4], captureLine }
      : null;
  }, `${name}: iOS Simulator ${expectedSampleCount} Metal pixel·hit identity capture`);

  const batchCounts = assertPixelBatchSequence(lines, ready.appRunID);
  await writeFile(resolve(evidenceDirectory, `${name}-runtime.log`), `${lines.join('\n')}\n`);
  const container = simctl(['get_app_container', udid, bundleID, 'data'], { timeout: 30_000 }).stdout.trim();
  const appArtifactDirectory = resolve(container, fullSurfaceCapture
    ? 'Library/Caches/c13-1-metal-surface-pixels'
    : 'Library/Caches/c13-1-metal-surface-pixel-centers');
  const samplePath = resolve(evidenceDirectory, ready.sampleFile);
  const manifestPath = resolve(evidenceDirectory, ready.manifestFile);
  await access(resolve(appArtifactDirectory, ready.sampleFile));
  await access(resolve(appArtifactDirectory, ready.manifestFile));
  await copyFile(resolve(appArtifactDirectory, ready.sampleFile), samplePath);
  await copyFile(resolve(appArtifactDirectory, ready.manifestFile), manifestPath);
  const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));
  assert.equal(manifest.runId, ready.appRunID);
  assert.equal(manifest.generation, ready.surfaceGeneration);
  assert.equal(manifest.processID, pid);
  assert.equal(manifest.sampleFile, ready.sampleFile);

  const comparisonPath = resolve(evidenceDirectory, `${name}-comparison.json`);
  const comparison = fullSurfaceCapture
    ? compareMetalFullSurfaceFiles({
      manifestPath,
      rgbaPath: samplePath,
      observationPath: chromeFullObservationPath,
      expectedIdentity: {
        runId: ready.appRunID,
        generation: ready.surfaceGeneration,
        processID: pid,
      },
    })
    : compareMetalArtifactFiles({ manifestPath, samplePath });
  await writeFile(comparisonPath, `${JSON.stringify(comparison, null, 2)}\n`);
  assert.equal(comparison.status, 'matched', `${name}: Chrome 155 exact RGBA comparison`);
  const comparedPixels = fullSurfaceCapture ? comparison.comparedPixels : comparison.sampleCount;
  const comparedChannels = comparison.comparedChannels;
  const drawablePixelSize = fullSurfaceCapture
    ? comparison.dimensions : comparison.drawablePixelSize;
  assert.equal(comparedPixels, expectedSampleCount);
  assert.equal(comparedChannels, expectedChannelCount);
  assert.equal(comparison.mismatchedPixels, 0);

  sleep(1_200);
  const screenshotPath = resolve(evidenceDirectory, `${name}-ios-simulator.png`);
  const screenshot = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
  await writeFile(resolve(evidenceDirectory, `${name}-screenshot.log`), screenshot.stdout + screenshot.stderr);
  const screenshotBytes = await readFile(screenshotPath);
  assert.equal(screenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a', `${name}: screenshot PNG signature`);
  assert.deepEqual([screenshotBytes.readUInt32BE(16), screenshotBytes.readUInt32BE(20)],
    [1206, 2622], `${name}: iPhone 17 Pro screenshot dimensions`);
  const maxDrawLineLength = Math.max(...lines
    .filter((line) => line.includes('SPINON_C0410_DRAW '))
    .map((line) => line.length));
  assert.ok(maxDrawLineLength < 1_000,
    `${name}: draw report logging must not serialize large sample batches`);

  return {
    name,
    pid,
    appRunID: ready.appRunID,
    layoutGeneration: ready.layoutGeneration,
    surfaceGeneration: ready.surfaceGeneration,
    metal: {
      samples: comparedPixels,
      channels: comparedChannels,
      mismatches: comparison.mismatchedPixels,
      maximumChannelError: comparison.maximumChannelError,
      drawablePixelSize,
      surfaceFormat: comparison.surfaceFormat,
      sampleSha256: comparison.sampleSha256,
      batches: batchCounts,
      outsideBorderSamples: comparison.outsideBorderSamples,
    },
    hitIdentity: { points: 30_000, matched: 30_000, mismatches: 0, fixedSamples: 52, nodeFrames: 41 },
    comparison: `${name}-comparison.json`,
    captureMode: fullSurfaceCapture ? 'full-physical-surface' : 'css-pixel-centers',
    manifest: ready.manifestFile,
    samples: ready.sampleFile,
    screenshot: `${name}-ios-simulator.png`,
    screenshotSha256: sha256(screenshotBytes),
    runtimeLog: `${name}-runtime.log`,
    maxDrawLineLength,
  };
}

async function captureArgumentNegativeControl(udid) {
  await stopPreviousProcess(udid, 'metal-route-argument-negative-control');
  const launch = simctl(['launch', udid, bundleID, captureArgument], {
    timeout: 90_000,
  });
  await writeFile(resolve(evidenceDirectory, 'metal-route-argument-negative-control-launch.log'),
    launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  const lines = waitFor(() => {
    const filtered = processLog(udid, pid);
    return hasLine(filtered,
      'SPINON_C131_METAL_PIXEL_ARGUMENT_ERROR reason=requires-compact-hit-identity-grid')
      ? filtered : null;
  }, 'pixel route 단독 실행 거부 negative control', 30_000);
  assert.equal(lines.some((line) => line.includes('SPINON_C0410_HOST_RETURNED')), false,
    '잘못된 pixel route는 runtime host 생성 전에 거부되어야 합니다.');
  assert.equal(lines.some((line) => line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE')), false);
  await writeFile(resolve(evidenceDirectory, 'metal-route-argument-negative-control.log'),
    `${lines.join('\n')}\n`);
  return { status: 'rejected-before-host-init', pid, hostCreated: false, captureCompleted: false };
}

async function captureRouteConflictControl(udid) {
  await stopPreviousProcess(udid, 'route-conflict-negative-control');
  const launch = simctl([
    'launch', udid, bundleID,
    '--spinon-c131-compact-hit-identity-grid',
    '--spinon-c131-overflow-clip',
    captureArgument,
  ], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, 'route-conflict-negative-control-launch.log'),
    launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  const lines = waitFor(() => {
    const filtered = processLog(udid, pid);
    return hasLine(filtered,
      'SPINON_C0410_ARGUMENT_ERROR conflicting=multiple-fixture-routes') ? filtered : null;
  }, '중복 C13.1 route 거부 negative control', 30_000);
  assert.equal(lines.some((line) => line.includes('SPINON_C0410_HOST_RETURNED')), false,
    '중복 route는 runtime host 생성 전에 거부되어야 합니다.');
  assert.equal(lines.some((line) => line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE')), false);
  await writeFile(resolve(evidenceDirectory, 'route-conflict-negative-control.log'),
    `${lines.join('\n')}\n`);
  return { status: 'rejected-before-host-init', pid, routes: 2, hostCreated: false, captureCompleted: false };
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const device = simulator();
  const requiredV8Revision = (await readFile(resolve(repositoryRoot, 'tools/v8/v8-revision.txt'), 'utf8')).trim();
  const actualV8Revision = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD']).stdout.trim();
  assert.equal(actualV8Revision, requiredV8Revision,
    `V8 checkout revision이 pin과 다릅니다: ${actualV8Revision}`);
  assert.equal(command('git', ['-C', v8Directory, 'status', '--porcelain']).stdout.trim(), '',
    'V8 checkout에 수정 파일이 있습니다.');
  const v8Archive = resolve(v8Directory, 'out/boson-ios-sim/obj/libv8_monolith.a');
  await access(v8Archive);
  const build = command('mise', [
    'exec', '--', 'env', `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1',
    'bun', 'run', 'build:ios-sim',
  ], { timeout: 900_000, allowFailure: true });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.equal(build.status, 0,
    `iOS Simulator build 실패; 자세한 내용은 ${evidenceDirectory}/build.log`);
  assert.match(`${build.stdout}${build.stderr}`, /\*\* BUILD SUCCEEDED \*\*/);
  assert.match(`${build.stdout}${build.stderr}`,
    /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=1[^\n]*c13_1_overflow_clip_fixture=1/);

  const appExecutable = await readFile(resolve(appPath, 'SpinonBootstrap'));
  const install = simctl(['install', device.udid, appPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);
  const priorPIDs = new Set();
  const runs = [];
  for (let index = 1; index <= 3; index += 1) {
    const result = await captureSuccessfulRun(device.udid, index, priorPIDs);
    priorPIDs.add(result.pid);
    runs.push(result);
    process.stdout.write(`${result.name}: ${result.metal.samples}/${expectedSampleCount} exact · ${result.metal.batches.requests} batches · pid=${result.pid}\n`);
  }
  const argumentNegativeControl = await captureArgumentNegativeControl(device.udid);
  const routeNegativeControl = await captureRouteConflictControl(device.udid);
  await stopPreviousProcess(device.udid, 'final-app-process-cleanup');
  const sourceRevision = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const sourceStatus = command('git', ['status', '--short']).stdout.trim();
  const xcodeVersion = command('xcodebuild', ['-version']).stdout.trim();
  const latestComparison = JSON.parse(await readFile(
    resolve(evidenceDirectory, 'run-3-comparison.json'), 'utf8',
  ));
  const summary = {
    schema: fullSurfaceCapture
      ? 'spinon-c13-1-ios-metal-full-physical-pixels/v1'
      : 'spinon-c13-1-ios-metal-pixel-centers/v1',
    status: 'matched',
    captureMode: fullSurfaceCapture ? 'full-physical-surface' : 'css-pixel-centers',
    captureRunID: runID,
    source: { revision: sourceRevision, status: sourceStatus || 'clean' },
    simulator: { name: device.name, udid: device.udid, runtime: device.runtime },
    v8: { revision: actualV8Revision, archiveSha256: await sha256File(v8Archive) },
    appExecutableSha256: sha256(appExecutable),
    chromeOracle: {
      product: 'Google Chrome 155.0.8059.85',
      observationSha256: chromeObservationSha256,
      screenshotSha256: fullSurfaceCapture
        ? latestComparison.oracle.screenshotSha256
        : 'b682308e3604ad599becff2c8d8c801c80a2a5b8547f3ac3316f133d47232601',
    },
    expected: {
      viewportCssPx: [300, 100],
      drawablePixelSize: fullSurfaceCapture ? [900, 300] : [900, 300],
      samples: expectedSampleCount,
      channels: expectedChannelCount,
      batchSize: 64,
      batches: expectedBatchCount,
    },
    runs,
    negativeControls: { argument: argumentNegativeControl, routeConflict: routeNegativeControl },
    xcodeVersion,
    scope: fullSurfaceCapture
      ? 'iOS 26.2 iPhone 17 Pro Simulator Metal drawable all-texel readback before compositor submission; no physical device, display scanout, hardware GPU, DOM event dispatch, WPT, or full C13.1 support.'
      : 'iOS 26.2 iPhone 17 Pro Simulator Metal drawable pixel centers before compositor submission; no physical device, display scanout, hardware GPU, DOM event dispatch, WPT, or full C13.1 support.',
  };
  await writeFile(resolve(evidenceDirectory, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    fullSurfaceCapture
      ? '# C13.1 iOS Simulator Metal full physical surface'
      : '# C13.1 iOS Simulator Metal surface pixel centers', '',
    `- Capture 묶음: \`${runID}\``,
    `- 대상: ${device.name} · ${device.runtime} · ${device.udid}`,
    `- 새 앱 process 3개에서 각각 ${expectedBatchCount.toLocaleString()} batch로 ${expectedSampleCount.toLocaleString()} ${fullSurfaceCapture ? 'physical texel' : 'CSS pixel center'} / ${expectedChannelCount.toLocaleString()} RGBA channel을 Chrome 155와 exact 비교했다.`,
    '- 각 실행에서 41/41 node frame, 52/52 고정 hit query와 30,000/30,000 hit identity가 같은 app run ID에 연결됐다.',
    fullSurfaceCapture
      ? '- row-major `.rgba` artifact는 각 Metal readback의 256-byte padded row에서 RGBA 네 byte만 packed 저장하고 surface generation·format·byte length·SHA-256을 manifest에 고정한다.'
      : '- 각 JSONL은 surface generation, drawable 좌표·크기, RGBA, run ID를 기록하고 SHA-256을 manifest에 고정한다.',
    fullSurfaceCapture
      ? '- `positioned-escape`의 border 밖 texel을 포함해 전체 drawable을 비교하고 네 channel 1-byte mutation, 누락·추가·순서·padding·identity 변조를 거부했다.'
      : '- `positioned-escape`의 `(270,72)` 표본은 overflow container의 오른쪽 border 밖에서 target paint가 유지되는 좌표다. 전체 표면 대조와 각 RGBA byte mutation control을 함께 검사했다.',
    '- 잘못된 Metal route 단독 실행과 중복 C13.1 route가 host 생성 전에 거부됐다.',
    '- [실행 요약](./summary.json) · build/install, 세 process log, screenshots, 직접 readback artifact/manifest와 Chrome 비교 결과를 보존했다.',
    '- 이 결과는 iOS Simulator의 Metal drawable 표본만 검증한다. compositor scanout, 실기기·hardware GPU, DOM events, WPT 및 C13.1 전체 완료를 뜻하지 않는다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({
    status: summary.status,
    captureRunID: runID,
    simulator: summary.simulator,
    runs: runs.map(({ name, pid, appRunID, metal }) => ({ name, pid, appRunID, samples: metal.samples, mismatches: metal.mismatches })),
    negativeControls: summary.negativeControls,
    appExecutableSha256: summary.appExecutableSha256,
    evidenceDirectory,
  }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
