import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const v8Directory = process.env.SPINON_V8_DIR
  ?? '/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8';
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const runID = `c13-1-stacking-ios-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/ios-simulator/${runID}`);
const identitySha256 = '866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483';
const oracleSha256 = '279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2';

function command(binary, args, { timeout = 60_000, allowFailure = false, maxBuffer = 96 * 1024 * 1024 } = {}) {
  const result = spawnSync(binary, args, { cwd: repositoryRoot, encoding: 'utf8', timeout, maxBuffer });
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
    .filter((device) => device.udid === expectedSimulatorUDID)
    .map((device) => ({ ...device, runtime })));
  assert.equal(matches.length, 1, '고정한 iOS Simulator UDID를 찾지 못했습니다.');
  assert.equal(matches[0].state, 'Booted', `Simulator가 Booted 상태가 아닙니다: ${matches[0].state}`);
  assert.equal(matches[0].isAvailable, true);
  assert.equal(matches[0].name, 'iPhone 17 Pro');
  assert.match(matches[0].runtime, /^com\.apple\.CoreSimulator\.SimRuntime\.iOS-26-2$/);
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
  const marker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => marker.test(line));
}

function waitFor(predicate, description, timeout = 150_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(450);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

async function stopPreviousProcess(udid, label) {
  const termination = simctl(['terminate', udid, bundleID], { timeout: 30_000, allowFailure: true });
  await writeFile(resolve(evidenceDirectory, `${label}-terminate.log`), `${termination.stdout}${termination.stderr}`);
  waitFor(() => runningAppPID(udid) === null, `${label}: 이전 앱 process 종료`, 30_000);
}

function launchPID(output) {
  const escapedBundle = bundleID.replaceAll('.', '\\.');
  const pid = Number(output.match(new RegExp(`${escapedBundle}[:,]\\s*(\\d+)`))?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, `simctl launch 응답에서 PID를 찾지 못했습니다: ${output}`);
  return pid;
}

function revisionTuple(line, prefix) {
  const match = line.match(new RegExp(`${prefix}generation=(\\d+) document_revision=(\\d+) render_tree_revision=(\\d+) style_revision=(\\d+)`));
  assert.ok(match, `revision tuple이 없습니다: ${line}`);
  return match.slice(1).map(Number);
}

async function captureSuccessfulRun(udid, index, priorPIDs) {
  const name = `run-${index}`;
  await stopPreviousProcess(udid, name);
  const launch = simctl(['launch', udid, bundleID, '--spinon-c131-stacking-descendant-clip'], {
    timeout: 90_000,
  });
  await writeFile(resolve(evidenceDirectory, `${name}-launch.log`), launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  assert.equal(priorPIDs.has(pid), false, `${name}: 새 앱 process PID가 이전 run과 같습니다.`);

  const result = waitFor(() => {
    const lines = processLog(udid, pid);
    const gridLine = lines.find((line) => line.includes(
      'SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=matched'));
    const evalLine = lines.find((line) => line.includes(`SPINON_C131_STACKING_CLIP_EVAL_RUN run_id=`));
    const identityLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_RUNTIME_IDENTITY '));
    const drawLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_DRAW_IDENTITY '));
    const fixtureLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_FIXTURE_SUMMARY status=ok'));
    const hitLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_HIT_TEST_SUMMARY status=ok'));
    const metalLine = lines.find((line) => line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE '));
    const failure = lines.find((line) => line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=error')
      || line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=mismatch')
      || line.includes('SPINON_C131_STACKING_CLIP_FIXTURE_SUMMARY status=error')
      || line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_ERROR '));
    if (failure) throw new Error(`${name}: iOS Simulator 검증 실패\n${lines.join('\n')}`);
    const runMatch = gridLine?.match(/run_id=([0-9a-f]{32}) runtime_generation=(\d+)/);
    const drawMatch = drawLine?.match(/run_id=([0-9a-f]{32}) surface_generation=(\d+)/);
    const evalMatch = evalLine?.match(/run_id=([0-9a-f]{32}) status=0 layout=ready boxes=13/);
    const evalTuple = identityLine && revisionTuple(identityLine, '');
    const drawTuple = drawLine && revisionTuple(drawLine, '');
    const gridValid = runMatch && runMatch[1] === evalMatch?.[1]
      && runMatch[1] === drawMatch?.[1]
      && runMatch[1] === identityLine?.match(/run_id=([0-9a-f]{32})/)?.[1]
      && Number(runMatch[2]) === evalTuple?.[0]
      && evalTuple?.every((value, tupleIndex) => value === drawTuple?.[tupleIndex])
      && gridLine.includes('points=69120 matched=69120 mismatches=0 edge_controls=11/11 preview=0')
      && gridLine.includes(`expected_identity_sha256=${identitySha256}`)
      && gridLine.includes(`identity_sha256=${identitySha256}`)
      && gridLine.includes(`oracle_sha256=${oracleSha256}`);
    const frames = lines.filter((line) => line.includes('SPINON_C131_STACKING_CLIP_NODE_FRAME '));
    const hitSamples = lines.filter((line) => line.includes('SPINON_C131_STACKING_CLIP_HIT_TEST id='));
    const hitsValid = hitSamples.length === 18 && hitSamples.every((line) => line.includes('status=matched'));
    const metalValid = metalLine?.includes(`run_id=${runMatch?.[1]}`)
      && metalLine.includes(`generation=${drawMatch?.[2]}`)
      && metalLine.includes('samples=18 batches=1');
    const screenDraw = lines.some((line) => /SPINON_C0410_DRAW generation=\d+ status=0 boxes=13 surface_texture=/.test(line));
    if (gridValid && frames.length === 13 && hitsValid && fixtureLine && hitLine
        && metalValid && screenDraw) {
      return { lines, gridLine, evalLine, identityLine, drawLine, metalLine,
        runID: runMatch[1], generation: Number(drawMatch[2]) };
    }
    return null;
  }, `${name}: iOS 13 frames, 18 samples, 69,120 hit identities and Metal readback`);

  await writeFile(resolve(evidenceDirectory, `${name}-runtime.log`), `${result.lines.join('\n')}\n`);
  const sampleFile = result.metalLine.match(/sample_file=(\S+)/)?.[1];
  const manifestFile = result.metalLine.match(/manifest_file=(\S+)/)?.[1];
  assert.ok(sampleFile && manifestFile, `${name}: Metal sample artifact names are missing.`);
  const container = simctl(['get_app_container', udid, bundleID, 'data']).stdout.trim();
  const cacheDirectory = resolve(container, 'Library/Caches/c13-1-metal-surface-pixel-centers');
  const metalSamples = await readFile(resolve(cacheDirectory, sampleFile));
  const metalManifest = await readFile(resolve(cacheDirectory, manifestFile));
  await writeFile(resolve(evidenceDirectory, `${name}-metal-samples.jsonl`), metalSamples);
  await writeFile(resolve(evidenceDirectory, `${name}-metal-manifest.json`), metalManifest);

  sleep(1_000);
  const screenshotPath = resolve(evidenceDirectory, `${name}-ios-simulator.png`);
  const screenshot = simctl(['io', udid, 'screenshot', screenshotPath], { timeout: 30_000 });
  await writeFile(resolve(evidenceDirectory, `${name}-screenshot.log`), screenshot.stdout + screenshot.stderr);
  const screenshotBytes = await readFile(screenshotPath);
  assert.equal(screenshotBytes.toString('hex', 0, 8), '89504e470d0a1a0a', `${name}: screenshot PNG signature`);
  assert.deepEqual([screenshotBytes.readUInt32BE(16), screenshotBytes.readUInt32BE(20)], [1206, 2622]);
  return {
    name, pid, runID: result.runID, surfaceGeneration: result.generation,
    revisions: revisionTuple(result.identityLine, ''), points: 69_120, matched: 69_120,
    edgeControls: 11, fixedHitSamples: 18, nodeFrames: 13,
    identitySha256, oracleSha256,
    metalSamples: { file: `${name}-metal-samples.jsonl`, sha256: sha256(metalSamples), count: metalSamples.toString('utf8').trim().split(/\r?\n/).length },
    metalManifest: `${name}-metal-manifest.json`,
    screenshot: `${name}-ios-simulator.png`, screenshotSha256: sha256(screenshotBytes),
    runtimeLog: `${name}-runtime.log`,
  };
}

async function captureStaleRevisionControl(udid) {
  await stopPreviousProcess(udid, 'stale-revision-control');
  const launch = simctl(['launch', udid, bundleID,
    '--spinon-c131-stacking-descendant-clip',
    '--spinon-c131-stacking-descendant-clip-stale-revision'], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, 'stale-revision-control-launch.log'), launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  const lines = waitFor(() => {
    const filtered = processLog(udid, pid);
    return filtered.some((line) => line.includes(
      'SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL name=stale-document-revision status=rejected field=document_revision surface_readback=not-requested'))
      ? filtered : null;
  }, 'iOS stale document revision negative control');
  assert.ok(lines.some((line) => line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=matched')));
  assert.equal(lines.some((line) => line.includes('SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE ')), false,
    'stale draw must be rejected before Metal surface readback completes.');
  await writeFile(resolve(evidenceDirectory, 'stale-revision-control.log'), `${lines.join('\n')}\n`);
  return { status: 'rejected-before-metal-readback', pid, field: 'document_revision' };
}

async function captureRouteConflictControl(udid) {
  await stopPreviousProcess(udid, 'route-conflict-control');
  const launch = simctl(['launch', udid, bundleID,
    '--spinon-c131-stacking-descendant-clip', '--spinon-c131-compact-surface-matrix'], { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, 'route-conflict-control-launch.log'), launch.stdout + launch.stderr);
  const pid = launchPID(launch.stdout);
  const lines = waitFor(() => {
    const filtered = processLog(udid, pid);
    return filtered.some((line) => line.includes('SPINON_C0410_ARGUMENT_ERROR conflicting=multiple-fixture-routes'))
      ? filtered : null;
  }, 'iOS conflicting route negative control');
  assert.equal(lines.some((line) => line.includes('SPINON_C131_HOST_CREATED')), false);
  assert.equal(lines.some((line) => line.includes('SPINON_C131_STACKING_CLIP_EVAL_RUN')), false);
  await writeFile(resolve(evidenceDirectory, 'route-conflict-control.log'), `${lines.join('\n')}\n`);
  return { status: 'rejected-before-host-init', pid, hostCreated: false, evalStarted: false };
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const device = simulator();
  const requiredV8Revision = (await readFile(resolve(repositoryRoot, 'tools/v8/v8-revision.txt'), 'utf8')).trim();
  const actualV8Revision = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD']).stdout.trim();
  assert.equal(actualV8Revision, requiredV8Revision, `V8 checkout revision mismatch: ${actualV8Revision}`);
  assert.equal(command('git', ['-C', v8Directory, 'status', '--porcelain']).stdout.trim(), '', 'V8 checkout is dirty.');
  const v8Archive = resolve(v8Directory, 'out/boson-ios-sim/obj/libv8_monolith.a');
  await access(v8Archive);
  const build = command('mise', ['exec', '--', 'env', `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1',
    'bun', 'run', 'build:ios-sim'], { timeout: 900_000 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.match(`${build.stdout}${build.stderr}`, /\*\* BUILD SUCCEEDED \*\*/);
  assert.match(`${build.stdout}${build.stderr}`, /c13_1_overflow_clip_fixture=1/);

  const appExecutable = await readFile(resolve(appPath, 'SpinonBootstrap'));
  const install = simctl(['install', device.udid, appPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);
  const processIDs = new Set();
  const runs = [];
  for (let index = 1; index <= 3; index += 1) {
    const result = await captureSuccessfulRun(device.udid, index, processIDs);
    assert.equal(processIDs.has(result.pid), false);
    assert.equal(runs.some((run) => run.runID === result.runID), false);
    processIDs.add(result.pid);
    runs.push(result);
    process.stdout.write(`${result.name}: ${result.matched}/${result.points} hit identity · pid=${result.pid}\n`);
  }
  const staleRevisionControl = await captureStaleRevisionControl(device.udid);
  const routeConflictControl = await captureRouteConflictControl(device.udid);
  await stopPreviousProcess(device.udid, 'final-app-shutdown');
  const summary = {
    schema: 'spinon-c13-1-stacking-full-viewport-ios-simulator/v1',
    status: 'matched', runID,
    sourceRevision: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
    simulator: { name: device.name, udid: device.udid, runtime: device.runtime },
    v8: { revision: actualV8Revision, archive: v8Archive },
    appExecutableSha256: sha256(appExecutable),
    oracle: { identitySha256, oracleSha256 },
    expected: { nodeFrames: 13, fixedHitSamples: 18, edgeControls: 11, gridPoints: 69_120 },
    runs, staleRevisionControl, routeConflictControl,
    xcodeVersion: command('xcodebuild', ['-version']).stdout.trim(),
    scope: 'iPhone 17 Pro iOS 26.2 Simulator app route and Metal 18-point readback; no physical device, DOM event dispatch, WPT, or full C13.1 claim.',
  };
  await writeFile(resolve(evidenceDirectory, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C13.1 stacking full viewport · iOS Simulator', '',
    `- 실행 묶음: \`${runID}\``,
    `- 대상: ${device.name} · ${device.runtime} · ${device.udid}`,
    '- 새 앱 프로세스 3개에서 각각 13개 frame, 18개 hit/RGBA 표본, 11개 경계 제어와 69,120개 integer CSS point identity를 확인했다.',
    '- 각 run에서 evaluation/draw의 generation·document/render-tree/style revision tuple이 같고 Chrome 155 identity digest가 일치했다.',
    '- stale document revision과 route collision negative control을 보존했다. 성공 run 앱은 각 화면 캡처 뒤 종료했다.',
    '- [실행 요약](./summary.json) · build/install 로그, process별 runtime/Metal 표본과 Simulator 화면을 보존했다.',
    '- iOS Simulator 및 제한된 Metal sample만 확인했다. 실기기, full drawable pixel parity, DOM event dispatch, WPT 및 C13.1 전체 지원 완료를 뜻하지 않는다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID,
    simulator: summary.simulator, runs: runs.length, staleRevisionControl,
    routeConflictControl, appExecutableSha256: summary.appExecutableSha256, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  try {
    simctl(['terminate', expectedSimulatorUDID, bundleID], { timeout: 30_000, allowFailure: true });
  } catch {}
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
