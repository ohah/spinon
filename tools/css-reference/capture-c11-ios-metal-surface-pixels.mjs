import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const expectedSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const fixturePath = resolve(repositoryRoot, 'tests/fixtures/css/c11/metal-surface-pixel-samples.json');
const fixture = JSON.parse(readFileSync(fixturePath, 'utf8'));
const referencePath = resolve(repositoryRoot, fixture.oracle.referencePath);
const screenshotReferencePath = resolve(repositoryRoot, fixture.oracle.screenshotPath);
const sourcePaths = [
  'crates/spinon-render-wgpu/src/renderer.rs',
  'crates/spinon-render-wgpu/src/renderer/surface_readback.rs',
  'crates/spinon-ffi/Cargo.toml',
  'crates/spinon-ffi/include/spinon_ffi.h',
  'crates/spinon-ffi/src/runtime_gpu.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi.rs',
  'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  'platforms/ios/Sources/SpinonRunner.h',
  'platforms/ios/Sources/SpinonRunner.mm',
  'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj',
  'package.json',
  'tools/build-ios-sim.sh',
  'tools/build-ios-native.sh',
  'tools/css-reference/compare-c11-ios-metal-surface-pixels.mjs',
  'tools/css-reference/compare-c11-ios-metal-surface-pixels.test.mjs',
  'tools/css-reference/capture-c11-ios-metal-surface-pixels.mjs',
  'tests/fixtures/css/c11/metal-surface-pixel-samples.json',
];
const runID = `c11-ios-metal-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-1-ios-metal-surface-pixels-${date}`, `run-${runID}`);

function command(binary, args, { allowFailure = false, timeout = 120_000, env = process.env } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding: 'utf8', env, timeout, maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr ?? result.stdout}`);
  }
  return { status: result.status, stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const delay = (milliseconds) => new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));
const readText = (path) => {
  try { return readFileSync(path, 'utf8'); } catch { return ''; }
};

function bootedSimulator() {
  const output = command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout;
  const devices = JSON.parse(output).devices;
  const booted = Object.entries(devices).flatMap(([runtime, items]) =>
    items.filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted Simulator가 하나여야 합니다.');
  const simulator = booted[0];
  assert.equal(simulator.udid, expectedSimulatorUDID, '고정 iPhone 17 Pro Simulator가 아닙니다.');
  assert.equal(simulator.name, 'iPhone 17 Pro');
  assert.match(simulator.runtime, /iOS-26-2$/, 'iOS runtime이 26.2가 아닙니다.');
  return simulator;
}

async function waitForRun(logPath, expectedRunID, timeoutMilliseconds = 60_000) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    const log = readText(logPath);
    const line = log.split(/\r?\n/).find((entry) =>
      entry.includes('SPINON_C111_METAL_PIXEL_SUMMARY') && entry.includes(`run_id=${expectedRunID}`));
    if (line) return { line, log };
    const failure = log.split(/\r?\n/).find((entry) =>
      entry.includes('SPINON_C111_METAL_PIXEL_ERROR') && entry.includes(`run_id=${expectedRunID}`));
    if (failure) throw new Error(`Metal 표본 실행에서 오류 로그를 받았습니다: ${failure}`);
    await delay(200);
  }
  throw new Error(`${expectedRunID}: ${timeoutMilliseconds / 1000}초 안에 Metal 표본 결과를 받지 못했습니다.`);
}

async function main() {
  mkdirSync(evidenceDirectory, { recursive: true });
  const simulator = bootedSimulator();
  const referenceBytes = readFileSync(referencePath);
  const chromeScreenshotBytes = readFileSync(screenshotReferencePath);
  const oracle = validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes: chromeScreenshotBytes });
  const fixtureBytes = readFileSync(fixturePath);
  const sourceHashes = Object.fromEntries(sourcePaths.map((path) => [
    path,
    sha256(readFileSync(resolve(repositoryRoot, path))),
  ]));
  const gitHead = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const gitWorktreeClean = command('git', ['status', '--porcelain']).stdout.trim().length === 0;
  const buildLogPath = resolve(evidenceDirectory, 'ios-simulator-build.log');
  const build = spawnSync('mise', [
    'exec', '--', 'env',
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1',
    'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1',
    'bun', 'run', 'build:ios-sim',
  ], {
    cwd: repositoryRoot, encoding: 'utf8', timeout: 900_000, maxBuffer: 64 * 1024 * 1024,
  });
  writeFileSync(buildLogPath, `${build.stdout ?? ''}${build.stderr ?? ''}`);
  if (build.error || build.status !== 0) {
    throw new Error(`C11 Metal 표본 iOS Simulator build 실패. 로그: ${buildLogPath}`);
  }
  assert.match(readText(buildLogPath), /\*\* BUILD SUCCEEDED \*\*/);
  assert.ok(existsSync(appPath), `앱 산출물이 없습니다: ${appPath}`);

  const appBinary = resolve(appPath, 'SpinonBootstrap');
  const xcode = command('xcodebuild', ['-version']).stdout.trim();
  const install = command('xcrun', ['simctl', 'install', simulator.udid, appPath]);
  writeFileSync(resolve(evidenceDirectory, 'install.log'), `${install.stdout}${install.stderr}`);

  const logPath = resolve(evidenceDirectory, 'unified.log');
  const descriptor = openSync(logPath, 'a');
  const stream = spawn('xcrun', [
    'simctl', 'spawn', simulator.udid, 'log', 'stream', '--style', 'compact',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_C111_METAL_PIXEL"',
  ], { cwd: repositoryRoot, stdio: ['ignore', descriptor, descriptor] });
  closeSync(descriptor);
  let captured = '';
  try {
    await delay(800);
    command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
    const samplePayload = Buffer.from(JSON.stringify(fixture.samples.map((sample) => ({
      id: sample.id,
      nodeId: sample.id,
      xCssPx: sample.xCssPx,
      yCssPx: sample.yCssPx,
      rgba: sample.rgba,
    })))).toString('base64');
    const launch = command('xcrun', [
      'simctl', 'launch', '--terminate-running-process', simulator.udid, bundleID,
      '--spinon-c11-grid-case=fixed-tracks-gap',
      `--spinon-c11-grid-metal-surface-pixels=${runID}`,
      `--spinon-c11-grid-metal-surface-samples=${samplePayload}`,
    ]);
    writeFileSync(resolve(evidenceDirectory, 'launch.log'), `${launch.stdout}${launch.stderr}`);
    const { log } = await waitForRun(logPath, runID);
    captured = log;
    const comparison = compareC11IOSMetalPixelLogs(captured, { fixture, runID });
    assert.equal(comparison.status, 'matched');
    assert.equal(comparison.matchedSamples, fixture.samples.length);

    const simulatorScreenshotPath = resolve(evidenceDirectory, 'simulator-screen.png');
    command('xcrun', ['simctl', 'io', simulator.udid, 'screenshot', simulatorScreenshotPath], { timeout: 30_000 });
    const simulatorScreenshot = readFileSync(simulatorScreenshotPath);
    assert.equal(simulatorScreenshot.toString('hex', 0, 8), '89504e470d0a1a0a', 'Simulator screenshot이 PNG가 아닙니다.');

    const result = {
      ...comparison,
      simulator: {
        name: simulator.name,
        udid: simulator.udid,
        runtime: simulator.runtime,
        deviceTypeIdentifier: simulator.deviceTypeIdentifier,
      },
      xcode,
      renderer: 'wgpu Metal CAMetalLayer drawable texture readback before compositor submission',
      oracle: { ...oracle, referencePath: fixture.oracle.referencePath,
        referenceSha256: sha256(referenceBytes), screenshotSha256: sha256(chromeScreenshotBytes) },
      appBinarySha256: sha256(readFileSync(appBinary)),
      fixtureSha256: sha256(fixtureBytes),
      source: { gitHead, gitWorktreeClean, sha256ByPath: sourceHashes },
      simulatorScreenshot: { path: 'simulator-screen.png', sha256: sha256(simulatorScreenshot) },
      limitations: [
        '고정 iPhone 17 Pro iOS 26.2 Simulator에서 실행했다. iPhone 실기기의 GPU 결과가 아니다.',
        'Chrome reference 360x800과 Spinon 320x240 viewport의 공통 좌상단 Grid 영역에서 7개 표본만 비교했다.',
        'WGPU drawable texture의 compositor 제출 전 RGBA이며 최종 화면 합성·색상 관리 뒤의 pixel과 같음을 보장하지 않는다.',
        '다른 C11 scene의 픽셀, WPT 적합성, 전체 CSS 지원을 입증하지 않는다.',
      ],
    };
    writeFileSync(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(result, null, 2)}\n`);
    writeFileSync(resolve(evidenceDirectory, 'metadata.json'), `${JSON.stringify({
      runID,
      simulator: result.simulator,
      xcode,
      appBundleID: bundleID,
      appBinarySha256: result.appBinarySha256,
      fixtureSha256: result.fixtureSha256,
      gitHead,
      gitWorktreeClean,
      sourceSha256ByPath: sourceHashes,
      chromeReferenceSha256: result.oracle.referenceSha256,
      chromeScreenshotSha256: result.oracle.screenshotSha256,
      buildFlags: {
        SPINON_ENABLE_C04_RUNTIME_GPU: 1,
        SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: 1,
      },
    }, null, 2)}\n`);
    writeFileSync(resolve(evidenceDirectory, 'source-hashes.json'), `${JSON.stringify({
      gitHead,
      gitWorktreeClean,
      sha256ByPath: sourceHashes,
    }, null, 2)}\n`);
    writeFileSync(resolve(evidenceDirectory, 'README.md'), [
      '# C11.1 iOS Metal surface pixel 표본', '',
      `- 결과: ${result.matchedSamples}/${result.expectedSamples} exact · 채널 오차 0`,
      `- 장치: ${simulator.name} · ${simulator.runtime} · ${simulator.udid}`,
      `- drawable: ${result.drawable.join('x')} px · CSS viewport: ${result.runtimeViewportCssPx.join('x')} px · scale: ${result.scale.join('/')}`,
      `- Chrome oracle: ${fixture.oracle.product} · 360×800 DPR 1 · reference/screenshot hash 고정`,
      '- 로그는 `unified.log`, 비교는 `comparison.json`, 실행 인수와 build/install 출력은 각 로그 파일에 보존했다.',
      '- 앱 binary, fixture, source 파일, Chrome reference와 screenshot hash를 `comparison.json`, `metadata.json`, `source-hashes.json`에 묶었다.',
      '- 결과는 drawable texture가 화면 합성기에 제출되기 전의 표본이다. 최종 display pixels, WPT 적합성, 전체 CSS 지원을 뜻하지 않는다.',
      '',
    ].join('\n'));
    process.stdout.write(`${JSON.stringify({ status: result.status, runID, samples: result.matchedSamples,
      simulator: result.simulator, drawable: result.drawable, evidenceDirectory }, null, 2)}\n`);
  } finally {
    stream.kill('SIGTERM');
    await Promise.race([
      new Promise((resolveClose) => stream.once('close', resolveClose)),
      delay(2_000),
    ]);
    command('xcrun', ['simctl', 'terminate', simulator.udid, bundleID], { allowFailure: true });
  }
}

main().catch((error) => {
  try {
    mkdirSync(evidenceDirectory, { recursive: true });
    writeFileSync(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  } catch { /* Preserve the primary failure. */ }
  process.stderr.write(`${error.stack ?? error}\n증거 경로: ${evidenceDirectory}\n`);
  process.exitCode = 1;
});
