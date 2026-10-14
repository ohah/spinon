import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { comparePngFiles } from './compare-c13-1-surface-pixel-centers.mjs';
import { selectAndroidAVDSerial } from './c11-3-3-grid-template-areas-runtime-device.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const serialOrUDID = process.argv[2] ?? process.env.SPINON_ANDROID_EMULATOR_SERIAL ?? 'emulator-5554';
const gitCommonDirectory = spawnSync('git', [
  'rev-parse', '--path-format=absolute', '--git-common-dir',
], { cwd: repositoryRoot, encoding: 'utf8' }).stdout.trim();
const localV8Directory = resolve(repositoryRoot, 'build/v8-source/v8');
const sharedV8Directory = resolve(dirname(gitCommonDirectory), 'build/v8-source/v8');
const v8Directory = process.env.SPINON_V8_DIR
  ?? (existsSync(localV8Directory) ? localV8Directory : sharedV8Directory);
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const timestamp = new Date().toISOString().replaceAll(/[^0-9TZ]/g, '');
const runID = `c13-1-android-avd-${timestamp}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c13-1-android-avd-full-surface-${date}/${runID}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function command(binary, args, {
  timeout = 120_000, allowFailure = false, encoding = 'utf8', maxBuffer = 96 * 1024 * 1024,
} = {}) {
  const result = spawnSync(binary, args, { cwd: repositoryRoot, encoding, timeout, maxBuffer });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function waitFor(predicate, description, timeout = 180_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(650);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function processLog(adb, pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  });
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const connected = command('adb', ['devices', '-l']).stdout;
  const serial = selectAndroidAVDSerial(connected, serialOrUDID);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const device = {
    serial,
    model: getprop('ro.product.model'),
    release: getprop('ro.build.version.release'),
    sdk: Number(getprop('ro.build.version.sdk')),
    abi: getprop('ro.product.cpu.abi'),
    qemu: getprop('ro.kernel.qemu') === '1',
    fingerprint: getprop('ro.build.fingerprint'),
    renderer: adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
      .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음',
  };
  assert.match(serial, /^emulator-\d+$/);
  assert.equal(device.qemu, true, '실기기는 실행하지 않습니다.');
  assert.equal(device.sdk, 37, `Android API 37 AVD가 아닙니다: ${device.sdk}`);
  assert.match(device.abi, /arm64-v8a/);
  assert.match(device.renderer, /SwiftShader/i,
    `고정한 AVD software renderer가 아닙니다: ${device.renderer}`);

  const build = command('mise', [
    'exec', '--', 'env',
    `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1',
    'bun', 'run', 'build:android',
  ], { timeout: 900_000, allowFailure: true });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.equal(build.status, 0,
    `Android AVD build 실패; 자세한 내용은 ${evidenceDirectory}/build.log`);
  assert.match(`${build.stdout}${build.stderr}`, /BUILD SUCCESSFUL/);
  assert.match(`${build.stdout}${build.stderr}`,
    /SPINON_ANDROID_BUILD_FLAGS c04_runtime_gpu=1 c13_1_overflow_clip_fixture_test=1/);

  const apk = await readFile(apkPath);
  const install = adb(['install', '-r', apkPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);
  const forceStop = adb(['shell', 'am', 'force-stop', packageName]);
  await writeFile(resolve(evidenceDirectory, 'force-stop.log'), forceStop.stdout + forceStop.stderr);
  waitFor(() => {
    const result = adb(['shell', 'pidof', packageName], { allowFailure: true });
    return result.status !== 0 || !result.stdout.trim();
  }, 'Android AVD: 이전 앱 process 종료', 30_000);
  await writeFile(resolve(evidenceDirectory, 'logcat-clear.log'),
    adb(['logcat', '-c'], { allowFailure: true }).stdout ?? '');
  const launch = adb([
    'shell', 'am', 'start', '-n', component,
    '--ez', 'spinon_c131_compact_surface_matrix', 'true',
  ], { timeout: 60_000 });
  await writeFile(resolve(evidenceDirectory, 'launch.log'), launch.stdout + launch.stderr);
  const pid = waitFor(() => {
    const result = adb(['shell', 'pidof', packageName], { allowFailure: true });
    const value = Number(result.stdout.trim().split(/\s+/)[0]);
    return Number.isInteger(value) && value > 0 ? value : null;
  }, 'Android AVD: 앱 PID 시작', 30_000);

  const ready = waitFor(() => {
    const lines = processLog(adb, pid);
    const pixelLine = lines.find((line) => line.includes(
      'SPINON_C131_COMPACT_PIXELCOPY_SUMMARY status=matched'));
    const pixelMatch = pixelLine?.match(/run_id=([a-f0-9]+) width=(\d+) height=(\d+) scale_x=([^ ]+) scale_y=([^ ]+) expected_samples=52 observed_samples=52 matched_samples=52 attempts=(\d+) sha256=([a-f0-9]{64}) png=(\S+)/);
    const gridLine = lines.find((line) => line.includes(
      'SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=matched'));
    const evalLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_EVAL_RUN '));
    const frameLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_NODE_FRAME '));
    const hitLines = lines.filter((line) => line.includes('SPINON_C131_COMPACT_HIT_TEST id='));
    const failure = lines.find((line) => line.includes('SPINON_C131_COMPACT_PIXELCOPY status=error')
      || line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error')
      || line.includes('SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=mismatch')
      || line.includes('SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error'));
    if (failure) throw new Error(`Android AVD 앱 검증 실패\n${lines.join('\n')}`);
    const runId = pixelMatch?.[1];
    const gridRunId = gridLine?.match(/run_id=([a-f0-9]+)/)?.[1];
    const appRunMatches = runId && gridRunId === runId && evalLines.length === 1
      && evalLines[0].includes(`SPINON_C131_COMPACT_EVAL_RUN run_id=${runId} status=0`)
      && /\blayout=ready boxes=41\b/.test(evalLines[0]);
    const gridReady = gridLine?.includes('points=30000 matched=30000 mismatches=0 preview=0');
    const framesByIndex = new Map();
    let frameRowsValid = true;
    for (const line of frameLines) {
      const payload = line.split('SPINON_C131_COMPACT_NODE_FRAME ')[1] ?? '';
      const index = Number(payload.match(/^(\d+):node=/)?.[1]);
      if (!Number.isInteger(index) || index < 0 || index >= 41) {
        frameRowsValid = false;
        continue;
      }
      const prior = framesByIndex.get(index);
      if (prior !== undefined && prior !== payload) frameRowsValid = false;
      framesByIndex.set(index, payload);
    }
    const completeFrames = frameRowsValid && framesByIndex.size === 41
      && Array.from({ length: 41 }, (_, index) => framesByIndex.has(index)).every(Boolean);
    const hitIds = hitLines.map((line) =>
      line.match(/SPINON_C131_COMPACT_HIT_TEST id=([^ ]+)/)?.[1]);
    const completeHits = hitLines.length === 52 && hitIds.every(Boolean)
      && new Set(hitIds).size === 52
      && hitLines.every((line) => line.includes('status=matched'));
    const fixedPixels = lines.filter((line) => line.includes(`SPINON_C131_COMPACT_PIXEL run_id=${runId} `));
    const drawReady = lines.some((line) =>
      /SPINON_C0410_DRAW generation=\d+ status=0 presented boxes=41/.test(line));
    if (appRunMatches && gridReady && completeFrames && completeHits
        && fixedPixels.length === 52 && drawReady && pixelMatch) {
      return { lines, pixelMatch, runId };
    }
    return null;
  }, 'Android API 37 AVD: 41 frames, 52 fixed samples, 30,000 identity points and PixelCopy');

  await writeFile(resolve(evidenceDirectory, 'runtime.log'), `${ready.lines.join('\n')}\n`);
  const appCachePng = ready.pixelMatch[8];
  const pulled = adb(['exec-out', 'run-as', packageName, 'cat', appCachePng], {
    encoding: 'buffer', timeout: 45_000,
  });
  assert.equal(sha256(pulled.stdout), ready.pixelMatch[7], '앱 log PNG SHA와 추출한 PixelCopy가 다릅니다.');
  const pixelCopyPath = resolve(evidenceDirectory, 'android-api37-avd-pixelcopy.png');
  await writeFile(pixelCopyPath, pulled.stdout);
  const screen = adb(['exec-out', 'screencap', '-p'], { encoding: 'buffer', timeout: 45_000 });
  const screenPath = resolve(evidenceDirectory, 'android-api37-avd-screen.png');
  await writeFile(screenPath, screen.stdout);
  const comparison = comparePngFiles({ actualPath: pixelCopyPath });
  await writeFile(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(comparison, null, 2)}\n`);
  assert.equal(comparison.status, 'matched');
  assert.equal(comparison.comparedPixels, 30_000);
  assert.equal(comparison.comparedChannels, 120_000);
  assert.equal(comparison.mismatchedPixels, 0);

  const sourceRevision = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const summary = {
    schema: 'spinon-c13-1-android-avd-full-surface/v1',
    status: 'matched',
    runID: ready.runId,
    sourceRevision,
    device,
    apkSha256: sha256(apk),
    appPid: pid,
    pixelCopy: {
      width: Number(ready.pixelMatch[2]), height: Number(ready.pixelMatch[3]),
      scaleX: Number(ready.pixelMatch[4]), scaleY: Number(ready.pixelMatch[5]),
      samples: comparison.comparedPixels,
      channels: comparison.comparedChannels,
      mismatches: comparison.mismatchedPixels,
      maximumChannelError: comparison.maximumChannelError,
      imageSha256: comparison.pixelCopy.sha256,
      outsideBorderCss: [270, 72],
    },
    hitIdentity: { points: 30_000, matched: 30_000, fixedSamples: 52, nodeFrames: 41 },
    oracle: comparison.oracle,
    comparison: 'comparison.json',
    pixelCopyImage: 'android-api37-avd-pixelcopy.png',
    screenImage: 'android-api37-avd-screen.png',
  };
  await writeFile(resolve(evidenceDirectory, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C13.1 Android API 37 AVD full surface regression', '',
    `- 실행 ID: \`${ready.runId}\``,
    `- 대상: ${device.serial} · ${device.model} · API ${device.sdk} · ${device.abi}`,
    '- C13.1 compact fixture 앱에서 41/41 frame, 52/52 fixed PixelCopy sample, 30,000/30,000 hit identity를 확인했다.',
    `- ${comparison.pixelCopySize[0]}×${comparison.pixelCopySize[1]} PixelCopy에서 CSS pixel center 30,000개와 RGBA channel 120,000개를 Chrome 155와 exact 비교했다. 불일치 ${comparison.mismatchedPixels}개다.`,
    '- Chrome target이 overflow container border 바깥까지 paint하는 `(270,72)`를 전체 좌표 scan에 포함했다.',
    '- API 37 ARM64 AVD의 ANGLE/Vulkan/SwiftShader만 검증한다. 실기기·hardware GPU·전체 physical pixel byte, DOM event dispatch, WPT 및 C13.1 완료를 뜻하지 않는다.',
    '- [실행 요약](./summary.json) · build/install/launch/runtime 로그, PixelCopy와 앱 화면을 함께 보존했다.', '',
  ].join('\n'));
  await writeFile(resolve(evidenceDirectory, 'artifacts.sha256'), [
    `${sha256(await readFile(pixelCopyPath))}  android-api37-avd-pixelcopy.png`,
    `${sha256(await readFile(screenPath))}  android-api37-avd-screen.png`,
    `${sha256(await readFile(resolve(evidenceDirectory, 'comparison.json')))}  comparison.json`,
  ].join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({
    status: summary.status,
    serial,
    runID: ready.runId,
    pixels: comparison.comparedPixels,
    channels: comparison.comparedChannels,
    mismatches: comparison.mismatchedPixels,
    pixelCopySize: comparison.pixelCopy.pixelCopySize,
    evidenceDirectory,
  }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
