import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';

import { buildC124RuntimeMatrix } from './build-c12-4-runtime-matrix.mjs';
import { compareC124AndroidSurfaceCase } from './compare-c12-4-android-surface-pixels.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const [serial = 'emulator-5554'] = process.argv.slice(2);
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const adb = 'adb';
const runToken = `c124-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-android-surface-pixels-${new Date().toISOString().slice(0, 10)}`,
  `run-${runToken}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const inventoryPath = resolve(repositoryRoot, 'tests/fixtures/css/c12/stacking-context-inventory.json');
const referencePath = resolve(repositoryRoot,
  'tests/fixtures/css/references/c12-4-stacking-context-v1.json');

function adbText(args, timeout = 15_000) {
  const result = spawnSync(adb, ['-s', serial, ...args], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    timeout,
    maxBuffer: 16 * 1024 * 1024,
  });
  if (result.error || result.status !== 0) {
    throw new Error(`adb ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr}`);
  }
  return result.stdout.trim();
}

function adbBinary(args, timeout = 15_000) {
  const result = spawnSync(adb, ['-s', serial, ...args], {
    cwd: repositoryRoot,
    encoding: null,
    timeout,
    maxBuffer: 24 * 1024 * 1024,
  });
  if (result.error || result.status !== 0) {
    throw new Error(`adb ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString()}`);
  }
  return result.stdout;
}

function parseDpi(output) {
  const matches = [...output.matchAll(/(?:Override|Physical) density:\s*(\d+)/g)];
  assert.ok(matches.length > 0, `AVD density를 읽지 못했습니다: ${output}`);
  return Number(matches.at(-1)[1]);
}

function parseDisplaySize(output) {
  const matches = [...output.matchAll(/(?:Override|Physical) size:\s*(\d+)x(\d+)/g)];
  assert.ok(matches.length > 0, `AVD display size를 읽지 못했습니다: ${output}`);
  const match = matches.at(-1);
  return { width: Number(match[1]), height: Number(match[2]) };
}

function wait(milliseconds) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
}

function currentPid() {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const result = adbText(['shell', 'pidof', packageName]);
    const pid = result.split(/\s+/).find(Boolean);
    if (pid) return pid;
    wait(100);
  }
  throw new Error(`${packageName} 프로세스가 시작되지 않았습니다.`);
}

function logsForPid(pid) {
  return adbText(['logcat', '-d', `--pid=${pid}`, '-s', 'SpinonBootstrap:I'], 10_000);
}

function waitForCapture(pid, caseId, caseRunId) {
  for (let attempt = 0; attempt < 160; attempt += 1) {
    const log = logsForPid(pid);
    const errorPrefix = `SPINON_C124_SURFACE_CAPTURE status=error run_id=${caseRunId}`;
    if (log.includes(errorPrefix)) throw new Error(`${caseId}: PixelCopy 실패 기록이 있습니다.\n${log}`);
    if (log.includes(`SPINON_C124_SURFACE_CAPTURE status=ok run_id=${caseRunId}`)) return log;
    wait(250);
  }
  throw new Error(`${caseId}: 40초 안에 PixelCopy 성공을 받지 못했습니다.\n${logsForPid(pid)}`);
}

function rendererLine(log, caseId) {
  const line = log.split(/\r?\n/).find((item) => item.includes('SPINON_C0410_RENDERER='));
  assert.ok(line, `${caseId}: WGPU renderer metadata가 없습니다.`);
  return line.slice(line.indexOf('SPINON_C0410_RENDERER=') + 'SPINON_C0410_RENDERER='.length).trim();
}

async function main() {
  assert.match(serial, /^emulator-\d+$/, '실기기 serial은 허용하지 않습니다. AVD serial을 지정하세요.');
  assert.equal(adbText(['get-state']), 'device', `${serial}이 online 상태가 아닙니다.`);
  assert.equal(adbText(['shell', 'getprop', 'ro.kernel.qemu']), '1', `${serial}은 Android emulator가 아닙니다.`);
  assert.equal(adbText(['shell', 'getprop', 'ro.build.version.sdk']), '37', '고정 API 37 AVD가 아닙니다.');

  const matrix = await buildC124RuntimeMatrix();
  const inventory = JSON.parse(await readFile(inventoryPath, 'utf8'));
  const reference = JSON.parse(await readFile(referencePath, 'utf8'));
  const referenceDpr1 = reference.screenshotFiles.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(referenceDpr1, '고정 Chrome DPR 1 screenshot 표본이 없습니다.');
  assert.equal(matrix.inventory.sha256,
    createHash('sha256').update(await readFile(inventoryPath)).digest('hex'),
    'runtime matrix inventory hash가 입력과 다릅니다.');
  const supportedCases = matrix.cases.filter(({ expected }) => expected.kind === 'supported');
  assert.equal(supportedCases.length, 21, '지원 surface case 수가 바뀌었습니다. 계획을 먼저 갱신하세요.');
  assert.equal(supportedCases.reduce((count, item) => count + item.samples.length, 0), 22,
    '지원 surface 표본 수가 바뀌었습니다. 계획을 먼저 갱신하세요.');

  const emulatorModel = adbText(['shell', 'getprop', 'ro.product.model']);
  const emulatorBuild = adbText(['shell', 'getprop', 'ro.build.fingerprint']);
  const densityDpi = parseDpi(adbText(['shell', 'wm', 'density']));
  const density = densityDpi / 160;
  const displaySize = parseDisplaySize(adbText(['shell', 'wm', 'size']));
  const qemuEgl = adbText(['shell', 'getprop', 'ro.hardware.egl']);
  const glDescription = adbText(['shell', 'dumpsys', 'SurfaceFlinger'])
    .split(/\r?\n/).find((line) => /GLES:.*ANGLE|SwiftShader/i.test(line)) ?? 'renderer details unavailable';
  assert.ok(Number.isFinite(density) && density > 0, 'AVD density가 올바르지 않습니다.');
  assert.ok(displaySize.width > 0 && displaySize.height > 0, 'AVD 화면 크기가 올바르지 않습니다.');

  const apk = await readFile(apkPath);
  assert.ok(apk.length > 0, `Android APK가 없습니다: ${apkPath}`);
  adbText(['install', '-r', apkPath], 60_000);
  await mkdir(dirname(evidenceDirectory), { recursive: true });
  await mkdir(evidenceDirectory, { recursive: false });

  const captureResults = [];
  const failures = [];
  for (const fixtureCase of supportedCases) {
    const fixtureIndex = inventory.cases.findIndex(({ id }) => id === fixtureCase.id);
    assert.notEqual(fixtureIndex, -1, `${fixtureCase.id}: inventory index가 없습니다.`);
    const chromeOrigin = {
      x: inventory.layout.originCssPx.x
        + (fixtureIndex % inventory.layout.columns) * inventory.layout.columnStepCssPx,
      y: inventory.layout.originCssPx.y
        + Math.floor(fixtureIndex / inventory.layout.columns) * inventory.layout.rowStepCssPx,
    };
    const caseRunId = `${runToken}_${fixtureCase.id}`;
    const imageName = `${fixtureCase.id}.png`;
    const logName = `${fixtureCase.id}.log`;
    try {
      adbText(['shell', 'am', 'force-stop', packageName]);
      const start = adbText([
        'shell', 'am', 'start', '-W', '-n', component,
        '--es', 'spinon_c124_surface_case', fixtureCase.id,
        '--es', 'spinon_c124_surface_run_id', caseRunId,
      ], 20_000);
      assert.match(start, /Status:\s*ok/, `${fixtureCase.id}: Android Activity가 시작되지 않았습니다.`);
      const pid = currentPid();
      const log = waitForCapture(pid, fixtureCase.id, caseRunId);
      await writeFile(join(evidenceDirectory, logName), `${log}\n`);
      const image = adbBinary([
        'exec-out', 'run-as', packageName, 'cat',
        `cache/c124-surface-captures/${fixtureCase.id}-${caseRunId}.png`,
      ]);
      await writeFile(join(evidenceDirectory, imageName), image, { flag: 'wx' });
      const caseResult = compareC124AndroidSurfaceCase({
        log,
        pngBytes: image,
        caseId: fixtureCase.id,
        runId: caseRunId,
        fixtureCase,
        referenceSamples: referenceDpr1.samples,
        stage: matrix.stage,
        chromeOrigin,
        viewportCssPx: matrix.viewportCssPx,
        inventorySha256: matrix.inventory.sha256,
        tolerance: inventory.comparison.maximumPixelChannelError,
        expectedDensity: density,
      });
      caseResult.pngPath = imageName;
      caseResult.logPath = logName;
      caseResult.renderer = rendererLine(log, fixtureCase.id);
      captureResults.push(caseResult);
      process.stdout.write(`통과 ${fixtureCase.id}: ${caseResult.samples.length}개 표본, 최대 채널 오차 ${caseResult.maximumChannelError}\n`);
    } catch (error) {
      const failure = `${fixtureCase.id}: ${error instanceof Error ? error.message : String(error)}`;
      failures.push(failure);
      await writeFile(join(evidenceDirectory, `${fixtureCase.id}.failure.txt`), `${failure}\n`);
      process.stderr.write(`실패 ${failure}\n`);
    }
  }

  const summary = {
    schema: 'spinon-c12-4-android-surface-pixels/v1',
    status: failures.length === 0 && captureResults.length === 21 ? 'matched' : 'mismatch',
    runId: runToken,
    device: {
      serial,
      model: emulatorModel,
      sdk: 37,
      buildFingerprint: emulatorBuild,
      displaySize,
      densityDpi,
      density,
      egl: qemuEgl,
      surfaceFlingerGles: glDescription,
      classification: 'Android AVD; simulator-only surface pixel result',
    },
    inputs: {
      inventory: { path: 'tests/fixtures/css/c12/stacking-context-inventory.json', sha256: matrix.inventory.sha256 },
      reference: {
        path: 'tests/fixtures/css/references/c12-4-stacking-context-v1.json',
        sha256: createHash('sha256').update(await readFile(referencePath)).digest('hex'),
        chromium: reference.chromium,
        screenshotDeviceScaleFactor: 1,
      },
      apk: {
        path: 'platforms/android/app/build/outputs/apk/debug/app-debug.apk',
        bytes: apk.length,
        sha256: createHash('sha256').update(apk).digest('hex'),
      },
    },
    expected: { supportedCases: 21, supportedNodes: 64, samples: 22, rejectedCases: 5 },
    actual: {
      supportedCases: captureResults.length,
      samples: captureResults.reduce((sum, item) => sum + item.samples.length, 0),
      maximumChannelError: Math.max(0, ...captureResults.map(({ maximumChannelError }) => maximumChannelError)),
    },
    captures: captureResults,
    failures,
  };
  await writeFile(join(evidenceDirectory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ ...summary, captures: `${captureResults.length}/21`, evidenceDirectory }, null, 2)}\n`);
  if (summary.status !== 'matched') process.exitCode = 1;
}

await main();
