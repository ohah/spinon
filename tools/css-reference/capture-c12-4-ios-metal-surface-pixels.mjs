import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

import {
  C124_IOS_PIXEL_CASES,
  compareC124IOSMetalPixelLogs,
} from './compare-c12-4-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const expectedDeviceUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const referencePath = resolve(repositoryRoot,
  'tests/fixtures/css/references/c12-4-stacking-context-v1.json');
const fixturePath = resolve(repositoryRoot, 'tests/fixtures/css/c12/runtime-stacking-context.js');
const batchID = `c124-ios-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-ios-metal-surface-pixels-${new Date().toISOString().slice(0, 10)}`,
  `run-${batchID}`);

function command(binary, args, { allowFailure = false, timeout = 120_000, env = process.env } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot,
    encoding: 'utf8',
    env,
    timeout,
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr ?? result.stdout}`);
  }
  return { status: result.status, stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function sha256(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function delay(milliseconds) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));
}

function readText(path) {
  try {
    return readFileSync(path, 'utf8');
  } catch {
    return '';
  }
}

async function waitForRun(logPath, runID, timeoutMilliseconds = 60_000) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    const log = readText(logPath);
    const line = log.split(/\r?\n/).find((entry) =>
      entry.includes(`SPINON_C124_METAL_PIXEL status=`)
      && entry.includes(`run_id=${runID}`));
    if (line) return { line, log };
    await delay(200);
  }
  throw new Error(`${runID}: ${timeoutMilliseconds / 1000}초 안에 Metal 표본 결과를 받지 못했습니다.`);
}

function bootedSimulator() {
  const output = command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout;
  const devices = JSON.parse(output).devices;
  const booted = Object.entries(devices).flatMap(([runtime, items]) =>
    items.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted Simulator가 하나여야 합니다. 다른 시뮬레이터를 끄고 다시 실행하세요.');
  const simulator = booted[0];
  assert.equal(simulator.udid, expectedDeviceUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(simulator.name, 'iPhone 17 Pro', '대상 simulator 이름이 다릅니다.');
  assert.match(simulator.runtime, /iOS-26-2$/, '대상 iOS runtime이 26.2가 아닙니다.');
  return simulator;
}

async function main() {
  mkdirSync(evidenceDirectory, { recursive: true });
  const simulator = bootedSimulator();
  const xcode = command('xcodebuild', ['-version']).stdout.trim();
  const reference = JSON.parse(readFileSync(referencePath, 'utf8'));
  const referenceHash = sha256(referencePath);
  const fixtureHash = sha256(fixturePath);
  const buildLogPath = resolve(evidenceDirectory, 'ios-simulator-build.log');
  const build = spawnSync('mise', [
    'exec', '--', 'env',
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=1',
    'bun', 'run', 'build:ios-sim',
  ], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    timeout: 900_000,
    maxBuffer: 64 * 1024 * 1024,
  });
  writeFileSync(buildLogPath, `${build.stdout ?? ''}${build.stderr ?? ''}`);
  if (build.error || build.status !== 0) {
    throw new Error(`내부 readback fixture iOS Simulator build 실패. 로그: ${buildLogPath}`);
  }
  assert.ok(readText(buildLogPath).includes('** BUILD SUCCEEDED **'), 'xcodebuild 성공 marker가 없습니다.');
  assert.ok(existsSync(appPath), `앱 산출물이 없습니다: ${appPath}`);

  const appBinary = resolve(appPath, 'SpinonBootstrap');
  const install = command('xcrun', ['simctl', 'install', simulator.udid, appPath]);
  writeFileSync(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const unifiedLogPath = resolve(evidenceDirectory, 'unified.log');
  const logFile = openSync(unifiedLogPath, 'a');
  const logStream = spawn('xcrun', [
    'simctl', 'spawn', simulator.udid, 'log', 'stream', '--style', 'compact',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_C124_METAL_"',
  ], { cwd: repositoryRoot, stdio: ['ignore', logFile, logFile] });
  closeSync(logFile);
  await delay(800);

  const perCaseRunIDs = {};
  const capturedLines = [];
  try {
    for (const fixtureCase of C124_IOS_PIXEL_CASES) {
      const caseRunID = `${batchID}-${fixtureCase.caseID}-${randomUUID().slice(0, 6)}`;
      perCaseRunIDs[fixtureCase.caseID] = caseRunID;
      const launchPath = resolve(evidenceDirectory, `${fixtureCase.caseID}.launch.txt`);
      const screenshotPath = resolve(evidenceDirectory, `${fixtureCase.caseID}.png`);
      command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
      const launch = command('xcrun', [
        'simctl', 'launch', '--terminate-running-process', simulator.udid, bundleID,
        `--spinon-c124-metal-surface-readback=${fixtureCase.direction},${fixtureCase.blueOrder},${fixtureCase.redOrder}`,
        `--spinon-c124-metal-surface-run=${caseRunID}`,
      ]);
      writeFileSync(launchPath, `${launch.stdout}${launch.stderr}`);
      const { line } = await waitForRun(unifiedLogPath, caseRunID);
      capturedLines.push(line);
      const screenshot = command('xcrun', [
        'simctl', 'io', simulator.udid, 'screenshot', screenshotPath,
      ], { timeout: 30_000 });
      assert.equal(screenshot.status, 0);
      const parsed = compareC124IOSMetalPixelLogs(line, {
        runID: caseRunID,
        expectedCases: [fixtureCase],
      });
      assert.equal(parsed.comparisonCount, 1);
      command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
    }
  } finally {
    logStream.kill('SIGTERM');
    await Promise.race([
      new Promise((resolveClose) => logStream.once('close', resolveClose)),
      delay(2_000),
    ]);
    command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
  }

  const completeLog = capturedLines.join('\n');
  const comparison = compareC124IOSMetalPixelLogs(completeLog, {
    runID: batchID,
    runIDs: perCaseRunIDs,
  });
  const result = {
    ...comparison,
    batchID,
    simulator: {
      name: simulator.name,
      udid: simulator.udid,
      runtime: simulator.runtime,
      deviceTypeIdentifier: simulator.deviceTypeIdentifier,
    },
    xcode,
    renderer: 'wgpu Metal CAMetalLayer surface texture COPY_SRC readback',
    fixtures: {
      referenceSha256: referenceHash,
      fixtureSha256: fixtureHash,
      chromeReferenceBrowserVersion: reference.chromium.version,
    },
    perCaseRunIDs,
    limitations: [
      'iOS Simulator 결과이며 실제 iPhone GPU 결과가 아니다.',
      '고정 Chromium 표본의 8개 C12.4 reverse Flex 조합만 비교했다.',
      'WPT 적합성 판정, 전체 C12.4 scene, 전체 CSS 지원을 의미하지 않는다.',
    ],
  };
  writeFileSync(resolve(evidenceDirectory, 'surface-pixel-records.log'), `${completeLog}\n`);
  writeFileSync(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(result, null, 2)}\n`);
  writeFileSync(resolve(evidenceDirectory, 'metadata.json'), `${JSON.stringify({
    batchID,
    simulator: result.simulator,
    xcode,
    appBundleID: bundleID,
    appBinarySha256: sha256(appBinary),
    referenceSha256: referenceHash,
    fixtureSha256: fixtureHash,
    startedAt: batchID,
  }, null, 2)}\n`);
  writeFileSync(resolve(evidenceDirectory, 'README.md'), [
    '# C12.4 iOS Metal surface 표면 표본',
    '',
    `- 결과: ${result.comparisonCount}/8 통과, 채널별 허용 오차 0`,
    `- 장치: ${simulator.name} · ${simulator.runtime} · ${simulator.udid}`,
    `- 빌드: ${xcode.replaceAll('\n', ' / ')}`,
    '- 각 조합은 새 앱 프로세스에서 V8 fixture를 변경하고 실제 `CAMetalLayer` WGPU surface texture를 복사해 읽었다.',
    '- 화면 이미지는 case별 `.png`, 원본 통합 로그는 `unified.log`, 비교 결과는 `comparison.json`에 있다.',
    '- 이 결과는 Simulator의 고정 8개 compatibility 표본이며 WPT 적합성이나 전체 CSS 지원 증거가 아니다.',
    '',
  ].join('\n'));
  process.stdout.write(`iOS Metal surface 표본 8/8 통과 · ${evidenceDirectory}\n`);
}

main().catch((error) => {
  try {
    mkdirSync(evidenceDirectory, { recursive: true });
    writeFileSync(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
    const simulator = bootedSimulator();
    const logs = command('xcrun', [
      'simctl', 'spawn', simulator.udid, 'log', 'show', '--last', '10m', '--style', 'compact',
      '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_C124_METAL_"',
    ], { allowFailure: true, timeout: 30_000 });
    writeFileSync(resolve(evidenceDirectory, 'failure-unified.log'), `${logs.stdout}${logs.stderr}`);
  } catch {
    // Failure evidence is best-effort; preserve the primary failure below.
  }
  process.stderr.write(`${error.stack ?? error}\n증거 경로: ${evidenceDirectory}\n`);
  process.exitCode = 1;
});
