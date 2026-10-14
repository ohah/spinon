import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const v8Directory = process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8');
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const requestedUDID = process.argv[2] ?? expectedSimulatorUDID;
assert.match(requestedUDID, /^[0-9A-Fa-f-]{36}$/,
  '사용법: node capture-c13-1-ios-app-hit-identity-grid.mjs [Simulator UDID]');
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const runID = `c13-1-ios-hit-grid-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c13-1-ios-app-hit-identity-grid-${date}/${runID}`);
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

function simulator() {
  const devices = JSON.parse(simctl(['list', 'devices', '--json']).stdout).devices;
  const matches = Object.entries(devices).flatMap(([runtime, items]) => items
    .filter((device) => device.udid === requestedUDID)
    .map((device) => ({ ...device, runtime })));
  assert.equal(matches.length, 1, `Simulator UDID를 찾을 수 없습니다: ${requestedUDID}`);
  assert.equal(matches[0].state, 'Booted', `Simulator가 Booted 상태가 아닙니다: ${matches[0].state}`);
  assert.equal(matches[0].isAvailable, true, 'Simulator runtime을 사용할 수 없습니다.');
  assert.equal(matches[0].udid, expectedSimulatorUDID,
    '계획에서 고정한 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(matches[0].name, 'iPhone 17 Pro');
  assert.match(matches[0].runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/,
    `계획에서 고정한 iOS 26.2가 아닙니다: ${matches[0].runtime}`);
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
    'spawn', udid, 'log', 'show', '--last', '5m', '--style', 'syslog',
    '--predicate', `processIdentifier == ${pid}`,
  ], { timeout: 45_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line));
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

async function stopPreviousProcess(udid, label) {
  const termination = simctl(['terminate', udid, bundleID], { timeout: 30_000, allowFailure: true });
  await writeFile(resolve(evidenceDirectory, `${label}-terminate.log`),
    `${termination.stdout}${termination.stderr}`);
  waitFor(() => runningAppPID(udid) === null, `${label}: 이전 app process 종료`, 30_000);
}

function launchPID(launchOutput) {
  const pid = Number(launchOutput.match(new RegExp(`${bundleID.replaceAll('.', '\\.')}[:,]\\s*(\\d+)`))?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, `simctl launch 응답에서 PID를 찾지 못했습니다: ${launchOutput}`);
  return pid;
}

async function captureSuccessfulRun(udid, index, priorPIDs) {
  const name = `run-${index}`;
  await stopPreviousProcess(udid, name);
  const launch = simctl(['launch', udid, bundleID, '--spinon-c131-compact-hit-identity-grid'], {
    timeout: 90_000,
  });
  await writeFile(resolve(evidenceDirectory, `${name}-launch.log`), launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  assert.equal(priorPIDs.has(pid), false, `${name}: 새 앱 프로세스 PID가 이전 실행과 같습니다.`);

  let lines = [];
  const ready = waitFor(() => {
    lines = processLog(udid, pid);
    const gridLine = lines.find((line) => line.includes(
      'SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=matched'));
    const runID = gridLine?.match(/run_id=([0-9a-f]{32})/)?.[1];
    const runtimeGeneration = gridLine?.match(/runtime_generation=(\d+)/)?.[1];
    const runContext = runID && runtimeGeneration
      ? `run_id=${runID} runtime_generation=${runtimeGeneration}` : '';
    const evalReady = runID && lines.some((line) => line.includes(
      `SPINON_C131_COMPACT_EVAL_RUN run_id=${runID} status=0 layout=ready boxes=41`));
    const fixedSamples = runContext && lines.some((line) => line.includes(
      `SPINON_C131_COMPACT_HIT_TEST_SUMMARY status=ok matched=52 total=52 ids=fixed ${runContext}`));
    const fixtureReady = runContext && lines.some((line) => line.includes(
      `SPINON_C131_COMPACT_FIXTURE_SUMMARY status=ok boxes=41 frames=41 hit_samples=52 hit_matches=52 ${runContext}`));
    const frameLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_NODE_FRAME '));
    const hitSampleLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_HIT_TEST id='));
    const geometryReady = runContext && lines.some((line) => line.includes(
      `SPINON_C131_COMPACT_GEOMETRY_SUMMARY status=ok frames=41 viewport=300x100 ${runContext}`));
    const framesCorrelated = frameLines.length === 41
      && frameLines.every((line) => line.includes(runContext));
    const samplesCorrelated = hitSampleLines.length === 52
      && hitSampleLines.every((line) => line.includes(runContext));
    const drawReady = lines.some((line) =>
      /SPINON_C0410_DRAW generation=\d+ status=0 presented boxes=41/.test(line));
    const gridReady = gridLine?.includes('points=30000 matched=30000 mismatches=0 preview=0')
      && gridLine.includes(`expected_identity_sha256=${identitySha256}`)
      && gridLine.includes(`identity_sha256=${identitySha256}`)
      && gridLine.includes(`oracle_sha256=${gridOracleSha256}`)
      && gridLine.includes(`observation_sha256=${chromeObservationSha256}`);
    const failure = lines.find((line) => line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error')
      || line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=mismatch')
      || line.includes('SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error'));
    if (failure) throw new Error(`${name}: 앱 검증 실패\n${lines.join('\n')}`);
    return runID && runtimeGeneration && evalReady && fixedSamples && fixtureReady && gridReady
      && geometryReady && framesCorrelated && samplesCorrelated && drawReady
      ? { runID, runtimeGeneration, gridLine } : null;
  }, `${name}: iOS Simulator 41-frame·52-sample·30,000-point hit identity 검증`);

  await writeFile(resolve(evidenceDirectory, `${name}-runtime.log`), `${lines.join('\n')}\n`);
  // UIKit labels and the Metal canvas update asynchronously after the runtime report.
  sleep(1_200);
  const screenshotPath = resolve(evidenceDirectory, `${name}-ios-simulator.png`);
  const screenshot = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
  await writeFile(resolve(evidenceDirectory, `${name}-screenshot.log`), screenshot.stdout + screenshot.stderr);
  const screenshotBytes = await readFile(screenshotPath);
  assert.equal(screenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a', `${name}: screenshot PNG signature`);
  assert.equal(screenshotBytes.readUInt32BE(16), 1206, `${name}: iPhone 17 Pro screenshot width`);
  assert.equal(screenshotBytes.readUInt32BE(20), 2622, `${name}: iPhone 17 Pro screenshot height`);

  const summaryFields = ready.gridLine.match(/SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=matched run_id=[0-9a-f]{32} runtime_generation=(\d+) points=30000 matched=30000 mismatches=0 preview=0/);
  assert.ok(summaryFields, `${name}: hit identity summary format`);
  return {
    name, pid, runID: ready.runID, runtimeGeneration: Number(summaryFields[1]),
    points: 30_000, matched: 30_000, mismatches: 0,
    fixedHitSamples: 52, nodeFrames: 41,
    correlatedFrameLogs: 41, correlatedHitSampleLogs: 52,
    identitySha256, gridOracleSha256, chromeObservationSha256,
    screenshot: `${name}-ios-simulator.png`, screenshotSha256: sha256(screenshotBytes),
    runtimeLog: `${name}-runtime.log`,
  };
}

async function captureRouteConflictControl(udid) {
  await stopPreviousProcess(udid, 'route-conflict-negative-control');
  const launch = simctl([
    'launch', udid, bundleID,
    '--spinon-c131-compact-hit-identity-grid', '--spinon-c131-compact-surface-matrix',
  ], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, 'route-conflict-negative-control-launch.log'),
    launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  const lines = waitFor(() => {
    const filtered = processLog(udid, pid);
    return filtered.some((line) => line.includes(
      'SPINON_C131_ARGUMENT_ERROR conflicting=multiple-fixture-routes')) ? filtered : null;
  }, 'C13.1 중복 route 거부 negative control');
  assert.equal(lines.some((line) => line.includes('SPINON_C0410_HOST_RETURNED')), false,
    '중복 route negative control은 runtime host를 만들면 안 됩니다.');
  assert.equal(lines.some((line) => line.includes('SPINON_C131_COMPACT_EVAL_RUN')), false,
    '중복 route negative control은 fixture eval을 실행하면 안 됩니다.');
  await writeFile(resolve(evidenceDirectory, 'route-conflict-negative-control.log'),
    `${lines.join('\n')}\n`);
  return { status: 'rejected-before-host-init', pid, conflictingRoutes: 2, hostCreated: false, evalStarted: false };
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const device = simulator();
  const requiredV8Revision = (await readFile(resolve(repositoryRoot, 'tools/v8/v8-revision.txt'), 'utf8')).trim();
  const actualV8Revision = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD']).stdout.trim();
  assert.equal(actualV8Revision, requiredV8Revision,
    `V8 checkout revision이 고정값과 다릅니다: ${actualV8Revision}`);
  assert.equal(command('git', ['-C', v8Directory, 'status', '--porcelain']).stdout.trim(), '',
    'V8 checkout에 수정 파일이 있습니다.');
  const v8Archive = resolve(v8Directory, 'out/boson-ios-sim/obj/libv8_monolith.a');
  await access(v8Archive);
  const build = command('mise', [
    'exec', '--', 'env', `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1', 'bun', 'run', 'build:ios-sim',
  ], { timeout: 900_000 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.match(`${build.stdout}${build.stderr}`, /\*\* BUILD SUCCEEDED \*\*/);
  assert.match(`${build.stdout}${build.stderr}`,
    /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=1[^\n]*c13_1_overflow_clip_fixture=1/);

  const appExecutable = await readFile(resolve(appPath, 'SpinonBootstrap'));
  const install = simctl(['install', device.udid, appPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);

  const pids = new Set();
  const runs = [];
  for (let index = 1; index <= 3; index += 1) {
    const result = await captureSuccessfulRun(device.udid, index, pids);
    pids.add(result.pid);
    runs.push(result);
    process.stdout.write(`${result.name}: ${result.matched}/${result.points} identity · pid=${result.pid}\n`);
  }
  const negativeControl = await captureRouteConflictControl(device.udid);
  const xcodeVersion = command('xcodebuild', ['-version']).stdout.trim();
  const summary = {
    schema: 'spinon-c13-1-ios-app-hit-identity-grid/v1',
    status: 'matched', runID,
    simulator: { name: device.name, udid: device.udid, runtime: device.runtime },
    v8: { revision: actualV8Revision, archive: v8Archive },
    appExecutableSha256: sha256(appExecutable),
    chromeOracle: { product: 'Chrome for Testing 155.0.8059.40',
      identitySha256, gridOracleSha256, chromeObservationSha256 },
    expected: { nodeFrames: 41, fixedHitSamples: 52, gridPoints: 30_000 },
    runs, negativeControl, xcodeVersion,
    scope: 'iOS 26.2 iPhone 17 Pro Simulator app route; no physical device or drawable pixel readback.',
  };
  await writeFile(resolve(evidenceDirectory, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C13.1 iOS Simulator app full hit identity grid', '',
    `- 실행 묶음: \`${runID}\``,
    `- 대상: ${device.name} · ${device.runtime} · ${device.udid}`,
    '- 새 앱 프로세스 3개에서 각각 41개 frame, 52개 고정 hit sample, 30,000개 정수 좌표 hit identity를 검증했다.',
    '- 세 실행 모두 Chrome 155 oracle identity, grid JSON, observation digest가 고정값과 일치했다.',
    '- 중복 C13.1 route negative control은 runtime host 초기화 전에 거부됐다.',
    '- [실행 요약](./summary.json) · build/install 로그, process별 runtime 로그와 화면을 보존했다.',
    '- iOS Simulator 증거만 포함한다. 실기기, Metal drawable pixel readback, DOM event dispatch, WPT 및 전체 C13.1 지원을 뜻하지 않는다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID,
    simulator: summary.simulator, runs: runs.length, negativeControl,
    appExecutableSha256: summary.appExecutableSha256, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
