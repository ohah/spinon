import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { hasC11GridDrawn, parseC11GridFrames } from './c11-grid-ios-runtime-log.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const matrixPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/runtime-grid-matrix.json');
const referencePath = resolve(repositoryRoot, 'tests/fixtures/css/references/c11-grid-precomparison-v2.json');
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const runID = `c11-grid-ios-matrix-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-grid-ios-simulator-${date}/${runID}`);

function command(binary, args, { timeout = 60_000, encoding = 'utf8', allowFailure = false } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function simulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const booted = Object.entries(devices).flatMap(([runtime, items]) =>
    items.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted Simulator가 하나여야 합니다.');
  assert.equal(booted[0].udid, expectedSimulatorUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(booted[0].name, 'iPhone 17 Pro');
  assert.match(booted[0].runtime, /iOS-26-2$/, '대상 iOS runtime이 26.2가 아닙니다.');
  return booted[0];
}

function unifiedLog(udid) {
  return command('xcrun', [
    'simctl', 'spawn', udid, 'log', 'show', '--style', 'compact', '--last', '2m',
    '--predicate', 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_C111_GRID" OR eventMessage CONTAINS "SPINON_C0410_DRAW" OR eventMessage CONTAINS "SPINON_C10_FRAME_REPORT_ERROR")',
  ], { timeout: 60_000 }).stdout;
}

function logForPid(log, pid) {
  return log.split(/\r?\n/).filter((line) =>
    line.includes(`SpinonBootstrap[${pid}:`)).join('\n');
}

function assertFrames(actual, expected, sceneId, nodeIdOffset = 0) {
  assert.equal(actual.length, expected.length, `${sceneId}: frame 수`);
  for (const [index, expectedNode] of expected.entries()) {
    const frame = actual[index];
    assert.equal(frame.nodeId, index + 1 + nodeIdOffset, `${sceneId}/${expectedNode.id}: preorder NodeId`);
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Math.abs(frame.rect[field] - expectedNode.rect[field]) <= 0.5,
        `${sceneId}/${expectedNode.id}/${field}: actual=${frame.rect[field]} expected=${expectedNode.rect[field]}`);
    }
  }
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const device = simulator();
  const matrixBytes = await readFile(matrixPath);
  const matrix = JSON.parse(matrixBytes.toString('utf8'));
  const referenceBytes = await readFile(referencePath);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const expectedCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.nodeCount, 40);
  assert.deepEqual(matrix.cases.map(({ id }) => id), expectedCases.map(({ id }) => id));
  assert.equal(matrix.inputs.chromiumReference.sha256, sha256(referenceBytes));
  command('mise', ['exec', '--', 'node', 'tools/css-reference/build-c11-grid-runtime-matrix.mjs', '--check']);

  const build = spawnSync('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'bun', 'run', 'build:ios-sim',
  ], {
    cwd: repositoryRoot, encoding: 'utf8', timeout: 900_000, maxBuffer: 64 * 1024 * 1024,
  });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  if (build.error || build.status !== 0) {
    throw new Error(`iOS Simulator build 실패. 로그: ${resolve(evidenceDirectory, 'build.log')}`);
  }
  assert.match(`${build.stdout}${build.stderr}`, /\*\* BUILD SUCCEEDED \*\*/);
  await writeFile(resolve(evidenceDirectory, 'install.log'),
    command('xcrun', ['simctl', 'install', device.udid, appPath]).stdout);

  const appExecutable = await readFile(resolve(appPath, 'SpinonBootstrap'));
  const xcodeVersion = command('xcodebuild', ['-version']).stdout.trim();
  const observations = [];
  for (const [index, fixture] of matrix.cases.entries()) {
    const expectedCase = expectedCases[index];
    assert.equal(expectedCase.id, fixture.id);
    const launch = command('xcrun', [
      'simctl', 'launch', '--terminate-running-process', device.udid, bundleID,
      `--spinon-c11-grid-case=${fixture.id}`,
    ]);
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-launch.log`), launch.stdout + launch.stderr);
    const pidMatch = launch.stdout.match(new RegExp(`${bundleID}:\\s*(\\d+)`));
    assert.ok(pidMatch, `${fixture.id}: launch PID가 없습니다: ${launch.stdout}`);
    const pid = Number(pidMatch[1]);

    let log = '';
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      log = logForPid(unifiedLog(device.udid), pid);
      const failed = log.match(new RegExp(`SPINON_C111_GRID_SUMMARY scene_id=${fixture.id} status=(-\\d+)`));
      if (failed) {
        await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), log);
        throw new Error(`${fixture.id}: iOS runtime eval 실패: ${failed[0]}`);
      }
      const evaluated = log.includes(`SPINON_C111_GRID_SUMMARY scene_id=${fixture.id} status=0 layout=ready boxes=${fixture.nodeIds.length + 1}`);
      const frames = parseC11GridFrames(log);
      const drawn = hasC11GridDrawn(log, fixture.nodeIds.length + 1);
      if (evaluated && frames.length === fixture.nodeIds.length + 1 && drawn) break;
      sleep(300);
    }
    if (!log.includes(`SPINON_C111_GRID_SUMMARY scene_id=${fixture.id} status=0 layout=ready boxes=${fixture.nodeIds.length + 1}`)
        || parseC11GridFrames(log).length !== fixture.nodeIds.length + 1
        || !hasC11GridDrawn(log, fixture.nodeIds.length + 1)) {
      await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), log);
      throw new Error(`${fixture.id}: iOS runtime/frame/WGPU marker 대기 시간 초과`);
    }
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), log);

    const renderFrames = parseC11GridFrames(log);
    const wrapper = renderFrames[0];
    assert.equal(wrapper.nodeId, 1, `${fixture.id}: wrapper preorder NodeId`);
    for (const [field, expected] of Object.entries({ x: 0, y: 0, width: 360, height: 800 })) {
      assert.ok(Math.abs(wrapper.rect[field] - expected) <= 0.5,
        `${fixture.id}/wrapper/${field}: actual=${wrapper.rect[field]} expected=${expected}`);
    }
    const frames = renderFrames.slice(1);
    assertFrames(frames, expectedCase.nodes, fixture.id, 1);

    const screenshotPath = resolve(evidenceDirectory, `${fixture.id}-simulator.png`);
    command('xcrun', ['simctl', 'io', device.udid, 'screenshot', screenshotPath]);
    const screenshot = await readFile(screenshotPath);
    assert.equal(screenshot.toString('hex', 0, 8), '89504e470d0a1a0a', `${fixture.id}: screenshot PNG`);
    observations.push({ sceneId: fixture.id, processId: pid, nodeCount: frames.length,
      wrapperFrame: wrapper, frames, screenshot: `${fixture.id}-simulator.png` });
    process.stdout.write(`${fixture.id}: ${frames.length}/${expectedCase.nodes.length} frame · pid=${pid} · matched\n`);
  }

  const comparison = {
    schema: 'spinon-c11-grid-ios-simulator-runtime-matrix/v1',
    status: 'matched', runID,
    simulator: { udid: device.udid, name: device.name, runtime: device.runtime },
    appExecutableSha256: sha256(appExecutable), matrixSha256: sha256(matrixBytes),
    oracle: { referenceId: reference.referenceId, chrome: reference.oracle.product,
      referenceSha256: sha256(referenceBytes), toleranceCssPx: 0.5 },
    summary: { expectedScenes: 10, matchedScenes: observations.length,
      expectedNodes: 40, matchedNodes: observations.reduce((sum, scene) => sum + scene.nodeCount, 0),
      freshProcessPerScene: true, surfacePixelReadback: false },
    observations,
    limitations: [
      'iPhone 17 Pro iOS 26.2 Simulator 한 대의 고정 C11.1 fixture geometry 비교다.',
      'Simulator screenshot은 보존했지만 Metal surface texture 직접 pixel readback은 하지 않았다.',
      'Android 실기기 결과와 합산하지 않으며, WPT 적합성·전체 CSS 지원을 뜻하지 않는다.',
    ],
    xcodeVersion,
  };
  await writeFile(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(comparison, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 iOS Simulator 10-scene runtime 비교', '',
    `- 실행 ID: ${runID}`, `- Simulator: ${device.name} · ${device.runtime} · ${device.udid}`,
    `- 앱 실행 파일 SHA-256: \`${comparison.appExecutableSha256}\``,
    '- 고정 Chrome 154 reference의 10개 scene·40개 node를 각각 새 앱 프로세스에서 실행했다.',
    '- wrapper frame과 모든 data-node x/y/width/height를 CSS px 기준 ±0.5 이내로 비교했다.',
    '- Metal surface texture 직접 pixel readback은 하지 않았다. simulator screenshot은 보조 시각 증거다.',
    '- [상세 비교](./comparison.json) · 각 scene 로그와 screenshot 및 build/install 로그를 함께 보존했다.',
    '- iOS 실기기, WPT 적합성, 전체 CSS 지원은 별도 관문이다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: 'matched', runID, simulator: comparison.simulator,
    scenes: observations.length, nodes: comparison.summary.matchedNodes,
    appExecutableSha256: comparison.appExecutableSha256, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
