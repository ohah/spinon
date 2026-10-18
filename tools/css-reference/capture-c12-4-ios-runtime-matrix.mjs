import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

import { buildC124RuntimeMatrix } from './build-c12-4-runtime-matrix.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const expectedDeviceUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const referencePath = resolve(repositoryRoot,
  'tests/fixtures/css/references/c12-4-stacking-context-v1.json');
const runID = `c124-ios-matrix-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-ios-runtime-matrix-${new Date().toISOString().slice(0, 10)}`,
  `run-${runID}`);

function command(binary, args, { timeout = 120_000, allowFailure = false } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot,
    encoding: 'utf8',
    timeout,
    maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr ?? result.stdout}`);
  }
  return { status: result.status, stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function bootedSimulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const booted = Object.entries(devices).flatMap(([runtime, items]) =>
    items.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted Simulator가 하나여야 합니다. 다른 Simulator를 종료하세요.');
  const simulator = booted[0];
  assert.equal(simulator.udid, expectedDeviceUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(simulator.name, 'iPhone 17 Pro', '대상 Simulator 이름이 다릅니다.');
  assert.match(simulator.runtime, /iOS-26-2$/, '대상 iOS runtime이 26.2가 아닙니다.');
  return simulator;
}

function waitForResult(path, timeoutMilliseconds = 300_000) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    if (existsSync(path)) {
      const result = JSON.parse(readFileSync(path, 'utf8'));
      if (result.runID === runID) return result;
    }
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 300);
  }
  throw new Error(`Simulator가 ${timeoutMilliseconds / 1000}초 안에 현재 run 결과를 기록하지 않았습니다.`);
}

function sha256(content) {
  return createHash('sha256').update(content).digest('hex');
}

async function main() {
  mkdirSync(evidenceDirectory, { recursive: true });
  const simulator = bootedSimulator();
  const matrix = await buildC124RuntimeMatrix();
  assert.equal(matrix.summary.cases, 26);
  assert.equal(matrix.summary.nodes, 77);
  const matrixBytes = Buffer.from(`${JSON.stringify(matrix)}\n`);
  const matrixPath = resolve(evidenceDirectory, 'c12-4-runtime-matrix-input.json');
  writeFileSync(matrixPath, matrixBytes);
  writeFileSync(resolve(evidenceDirectory, 'inventory.sha256'), `${matrix.inventory.sha256}\n`);

  const buildLogPath = resolve(evidenceDirectory, 'ios-simulator-build.log');
  const build = spawnSync('mise', [
    'exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:ios-sim',
  ], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    timeout: 900_000,
    maxBuffer: 64 * 1024 * 1024,
  });
  writeFileSync(buildLogPath, `${build.stdout ?? ''}${build.stderr ?? ''}`);
  if (build.error || build.status !== 0) {
    throw new Error(`iOS Simulator C04.10 runtime build 실패. 로그: ${buildLogPath}`);
  }
  assert.ok(readFileSync(buildLogPath, 'utf8').includes('** BUILD SUCCEEDED **'),
    'xcodebuild 성공 marker가 없습니다.');
  assert.ok(existsSync(appPath), `앱 산출물이 없습니다: ${appPath}`);

  const install = command('xcrun', ['simctl', 'install', simulator.udid, appPath]);
  writeFileSync(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);
  const dataContainer = command('xcrun', ['simctl', 'get_app_container', simulator.udid, bundleID, 'data'])
    .stdout.trim();
  const documents = resolve(dataContainer, 'Documents');
  mkdirSync(documents, { recursive: true });
  const inputInContainer = resolve(documents, 'c12-4-runtime-matrix-input.json');
  const resultInContainer = resolve(documents, `c124-runtime-matrix-${runID}.json`);
  copyFileSync(matrixPath, inputInContainer);
  rmSync(resultInContainer, { force: true });
  assert.equal(sha256(readFileSync(inputInContainer)), sha256(matrixBytes),
    'Simulator sandbox에 복사한 matrix 입력 hash가 다릅니다.');

  const launch = command('xcrun', [
    'simctl', 'launch', '--terminate-running-process', simulator.udid, bundleID,
    '--spinon-c124-runtime-matrix', `--spinon-c124-run-id=${runID}`,
  ]);
  writeFileSync(resolve(evidenceDirectory, 'launch.log'), launch.stdout + launch.stderr);
  const result = waitForResult(resultInContainer);
  const resultPath = resolve(evidenceDirectory, 'ios-runtime-matrix.json');
  copyFileSync(resultInContainer, resultPath);
  assert.equal(result.schema, 'spinon-c124-ios-runtime-matrix/v1');
  assert.equal(result.runID, runID);
  assert.equal(result.status, 'matched', `iOS runtime matrix status=${result.status}: ${result.error ?? ''}`);
  assert.equal(result.inventorySHA256, matrix.inventory.sha256);
  assert.equal(result.platform, 'iOS Simulator');
  assert.equal(result.device, 'iPhone');
  assert.equal(result.osVersion, '26.2');

  const comparison = command(process.execPath, [
    'tools/css-reference/compare-c12-4-runtime-matrix.mjs', resultPath,
  ], { timeout: 60_000 });
  writeFileSync(resolve(evidenceDirectory, 'comparison.json'), comparison.stdout);
  writeFileSync(resolve(evidenceDirectory, 'comparison-stderr.log'), comparison.stderr);
  const xcodeVersion = command('xcodebuild', ['-version']).stdout.trim();
  const chromiumReference = JSON.parse(readFileSync(referencePath, 'utf8'));
  assert.equal(chromiumReference.fixture.inventorySha256, matrix.inventory.sha256,
    'iOS matrix와 Chromium reference가 서로 다른 inventory를 사용합니다.');
  const appHash = sha256(readFileSync(resolve(appPath, 'SpinonBootstrap')));
  const referenceHash = sha256(readFileSync(referencePath));
  const metadata = {
    runID,
    branchHead: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
    simulator: { udid: simulator.udid, name: simulator.name, runtime: simulator.runtime },
    xcodeVersion,
    appBundleID: bundleID,
    appExecutableSHA256: appHash,
    matrixInputSHA256: sha256(matrixBytes),
    inventorySHA256: matrix.inventory.sha256,
    chromiumReferencePath: 'tests/fixtures/css/references/c12-4-stacking-context-v1.json',
    chromiumReferenceID: chromiumReference.referenceId,
    chromiumReferenceSHA256: referenceHash,
    geometryToleranceCssPx: 0.5,
    comparisonScope: '런타임 geometry와 기대 거부 진단 비교; Metal 표면 pixel readback 미포함',
  };
  writeFileSync(resolve(evidenceDirectory, 'capture-metadata.json'),
    `${JSON.stringify(metadata, null, 2)}\n`);
  const comparisonResult = JSON.parse(comparison.stdout);
  writeFileSync(resolve(evidenceDirectory, 'README.md'), [
    '# C12.4 iOS Simulator 런타임 행렬',
    '',
    `- 실행 ID: \`${runID}\``,
    `- Simulator: ${simulator.name} / iOS 26.2 / \`${simulator.udid}\``,
    `- 입력: 26개 scene, source node 77개; inventory SHA-256 \`${matrix.inventory.sha256}\``,
    `- 결과: ${comparisonResult.cases}/26 case 일치; 지원 ${comparisonResult.supportedCases}개, 기대 거부 ${comparisonResult.rejectedCases}개`,
    `- 지원 fixture frame 비교: ${comparisonResult.comparedNodes}개; 최대 오차 ${comparisonResult.maximumAbsoluteErrorCssPx} CSS px (허용 ${comparisonResult.toleranceCssPx} CSS px)`,
    '- 범위: 고정 Chromium 154 결과와 runtime geometry 및 기대 거부 진단을 비교했다.',
    '- Metal 표면 pixel, WPT 적합성, iPhone 실기기 동작, 전체 CSS 지원은 측정하지 않았다.',
    '',
    '산출물: `ios-runtime-matrix.json`, `comparison.json`, `capture-metadata.json`, `c12-4-runtime-matrix-input.json`, `ios-simulator-build.log`, `install.log`, `launch.log`, `ios-runtime-matrix-unified-log.txt`.',
  ].join('\n') + '\n');
  const log = command('xcrun', [
    'simctl', 'spawn', simulator.udid, 'log', 'show', '--style', 'compact', '--last', '10m',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS "SPINON_C124_IOS_MATRIX"',
  ], { timeout: 60_000, allowFailure: true });
  writeFileSync(resolve(evidenceDirectory, 'ios-runtime-matrix-unified-log.txt'), log.stdout + log.stderr);

  process.stdout.write(`${JSON.stringify({
    status: result.status,
    runID,
    simulator: metadata.simulator,
    inputBytes: matrixBytes.length,
    inputSHA256: sha256(matrixBytes),
    inventorySHA256: metadata.inventorySHA256,
    appExecutableSHA256: metadata.appExecutableSHA256,
    chromiumReferenceSHA256: metadata.chromiumReferenceSHA256,
    result: comparisonResult,
    evidenceDirectory,
  }, null, 2)}\n`);
}

await main();
