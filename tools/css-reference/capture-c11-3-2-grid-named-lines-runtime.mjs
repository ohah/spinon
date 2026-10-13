import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { compareC11AndroidPixelCopyScene } from './compare-c11-android-surface-pixel-matrix.mjs';
import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c11/grid-named-lines-c1132-mobile-surface-pixel-matrix-v1.json';
const fixture = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
const target = process.argv[2];
assert.ok(['android-physical', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-3-2-grid-named-lines-runtime.mjs android-physical [serial]|ios-simulator [Simulator UDID]');

const packageName = 'dev.spinon.bootstrap';
const androidComponent = `${packageName}/.MainActivity`;
const defaultSimulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const serialOrUDID = process.argv[3] ?? defaultSimulatorUDID;
const runID = `c1132-${target}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-3-2-grid-named-lines-runtime-${date}`, `${target}-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const iosBinaryPath = resolve(iosAppPath, 'SpinonBootstrap');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

async function saveLog(path, result) {
  await writeFile(resolve(evidenceDirectory, path), `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function samplePayload(encoding, samples = fixture.samples) {
  return Buffer.from(JSON.stringify(samples)).toString(encoding);
}

function assertNamedLineGeometry(log, runID) {
  const summary = log.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C1132_GRID_SUMMARY run_id=${runID} scene_id=${fixture.sceneId} `));
  assert.ok(summary?.includes('status=0 layout=ready boxes=2'),
    'named line success summary가 없거나 기대 node 수가 아닙니다.');
  const frameLines = log.split(/\r?\n/).filter((line) =>
    line.includes('SPINON_C1132_GRID_NODE_FRAME'));
  const frames = frameLines.map((line) => {
    const match = line.match(/\b(\d+):node=(\d+),x=([^,\s]+),y=([^,\s]+),width=([^,\s]+),height=([^,\s]+)/);
    assert.ok(match, `node frame 형식이 잘못되었습니다: ${line}`);
    return {
      index: Number(match[1]), nodeId: Number(match[2]),
      x: Number(match[3]), y: Number(match[4]), width: Number(match[5]), height: Number(match[6]),
    };
  });
  assert.equal(frames.length, 2, '성공 장면 node frame은 두 개여야 합니다.');
  assert.deepEqual(frames.map(({ index }) => index), [0, 1], 'node preorder가 다릅니다.');
  assert.equal(new Set(frames.map(({ nodeId }) => nodeId)).size, 2, '중복 node ID입니다.');
  assert.deepEqual(frames.map(({ x, y, width, height }) => ({ x, y, width, height })), [
    { x: 0, y: 0, width: 160, height: 120 },
    { x: 80, y: 0, width: 80, height: 40 },
  ]);
  const logLines = log.split(/\r?\n/);
  const rejectionLine = logLines.find((line) =>
    line.includes(`SPINON_C1132_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=missing-name-implicit-line `))
    ?? logLines.find((line) =>
      line.includes(`SPINON_C1132_GRID_REJECTION run_id=${runID} scene_id=missing-name-implicit-line `));
  assert.ok(rejectionLine?.includes('expected=true status=-11')
      && rejectionLine.includes('unsupported_named_grid_line')
      && (rejectionLine.includes('property=Some("grid-column-start")')
        || rejectionLine.includes('property=grid-column-start'))
      && (!rejectionLine.includes('SPINON_C1132_GRID_REJECTION_SUMMARY')
        || rejectionLine.includes('frames=absent')),
  '없는 named line 거부가 기대 code·property로 기록되지 않았습니다.');
  assert.ok(!rejectionLine.includes('node_frames_css_px=['),
    '기대 거부 결과에 layout frame이 들어있습니다.');
  assert.ok(log.includes('SPINON_C0410_DRAW') && log.includes('presented boxes=2'),
    'WGPU surface draw/present가 완료되지 않았습니다.');
  return frames;
}

function currentAndroidProcessLog(adb, pid) {
  const output = adb(['shell', 'logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function androidDevice() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/);
  assert.ok(devices.some((line) => line.startsWith(`${serialOrUDID} `) && line.includes(' device ')),
    `Android 대상이 연결되지 않았습니다: ${serialOrUDID}`);
  const adb = (args, options) => command('adb', ['-s', serialOrUDID, ...args], options);
  const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
  const qemu = getprop('ro.kernel.qemu');
  assert.notEqual(qemu, '1', `Android 실기기 대상으로 AVD가 지정됐습니다: ${serialOrUDID}`);
  const release = getprop('ro.build.version.release');
  const sdk = Number(getprop('ro.build.version.sdk'));
  return {
    adb,
    metadata: {
      serial: serialOrUDID, model: getprop('ro.product.model'), release, sdk,
      fingerprint: getprop('ro.build.fingerprint'), qemu,
      renderer: adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
        .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음',
    },
  };
}

function bootedSimulator() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const matches = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === serialOrUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(matches.length, 1, `Booted iOS Simulator를 찾지 못했습니다: ${serialOrUDID}`);
  assert.match(matches[0].runtime, /iOS-26-2$/, `iOS 26.2 Simulator가 아닙니다: ${matches[0].runtime}`);
  return matches[0];
}

function iosProcessLog(udid, pid) {
  const output = command('xcrun', ['simctl', 'spawn', udid, 'log', 'show', '--last', '5m',
    '--style', 'syslog', '--predicate', 'process == "SpinonBootstrap"'], { timeout: 45_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForLog(readLog, predicate, message, timeout = 150_000) {
  const deadline = Date.now() + timeout;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    if (predicate(latest)) return latest;
    sleep(300);
  }
  throw new Error(`${message} 대기 시간 초과\n${latest}`);
}

function waitForAndroidPid(adb, description, timeout = 30_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]);
    if (Number.isInteger(pid) && pid > 0) return pid;
    sleep(200);
  }
  throw new Error(`${description}: Android process 대기 시간이 초과되었습니다.`);
}

async function runAndroid() {
  const { adb, metadata } = androidDevice();
  await writeFile(resolve(evidenceDirectory, 'device.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const build = command('mise', ['exec', '--', 'env',
    `SPINON_V8_DIR=${process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8')}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'bun', 'run', 'build:android'], { timeout: 900_000 });
  await saveLog('build.log', build);
  const apkBytes = await readFile(apkPath);
  const appSha256 = sha256(apkBytes);
  await saveLog('install.log', adb(['install', '-r', apkPath]));
  const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', androidComponent,
    '--ez', 'spinon_c1132_grid_named_lines', 'true',
    '--es', 'spinon_c1132_grid_named_lines_run_id', runID,
    '--es', 'spinon_c11_grid_case', fixture.sceneId,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', samplePayload('base64url')]);
  await saveLog('launch.log', launch);
  const pid = waitForAndroidPid(adb, 'C11.3.2 실행');
  const log = waitForLog(() => currentAndroidProcessLog(adb, pid), (value) =>
    value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`)
      || value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`)
      || value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_CAPTURE status=error run_id=${runID}`)
      || value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX status=error run_id=${runID}`),
  'Android C11.3.2 runtime/pixel 결과');
  await writeFile(resolve(evidenceDirectory, 'runtime.log'), `${log}\n`);
  const frames = assertNamedLineGeometry(log, runID);
  const capture = log.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_CAPTURE status=ok run_id=${runID}`));
  assert.ok(capture, 'Android PixelCopy capture marker가 없습니다.');
  const devicePng = capture.match(/\bpng=(\S+)/)?.[1];
  assert.ok(devicePng?.startsWith('/data/user/0/dev.spinon.bootstrap/cache/c1132-grid-surface-pixel-matrix/'),
    'PixelCopy 경로가 C11.3.2 전용 cache 경로가 아닙니다.');
  const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
    `cache/c1132-grid-surface-pixel-matrix/${devicePng.split('/').at(-1)}`], { encoding: null }).stdout;
  const pixelComparison = compareC11AndroidPixelCopyScene(log, {
    scene: { id: fixture.sceneId, samples: fixture.samples }, runID, pngBytes,
    viewportCssPx: fixture.runtime.viewportCssPx, markerPrefix: 'C1132',
    cacheDirectory: 'c1132-grid-surface-pixel-matrix',
  });
  await writeFile(resolve(evidenceDirectory, 'pixelcopy.png'), pngBytes);
  const screen = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
  await writeFile(resolve(evidenceDirectory, 'android-screen.png'), screen);
  const negative = await runAndroidNegative(adb);
  return { metadata, appSha256, frames, pixelComparison, screenSha256: sha256(screen), negative };
}

async function runAndroidNegative(adb) {
  const negativeRunID = `${runID}-bad-samples`;
  const invalidSamples = fixture.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', androidComponent,
    '--ez', 'spinon_c1132_grid_named_lines', 'true',
    '--es', 'spinon_c1132_grid_named_lines_run_id', negativeRunID,
    '--es', 'spinon_c11_grid_case', fixture.sceneId,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', negativeRunID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', samplePayload('base64url', invalidSamples)]);
  await saveLog('android-pixel-allowlist-negative-launch.log', launch);
  const pid = waitForAndroidPid(adb, 'C11.3.2 negative control');
  const log = waitForLog(() => currentAndroidProcessLog(adb, pid), (value) =>
    value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX status=error run_id=${negativeRunID}`)
      || value.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_BEGIN run_id=${negativeRunID}`),
  'Android pixel allowlist negative control', 30_000);
  const rejected = log.includes(`SPINON_C1132_GRID_PIXEL_MATRIX status=error run_id=${negativeRunID}`);
  const readbackStarted = log.includes(`SPINON_C1132_GRID_PIXEL_MATRIX_BEGIN run_id=${negativeRunID}`);
  await writeFile(resolve(evidenceDirectory, 'android-pixel-allowlist-negative-control.log'), `${log}\n`);
  assert.ok(rejected, 'Android가 변경된 표본 좌표를 거부하지 않았습니다.');
  assert.equal(readbackStarted, false, 'Android가 좌표 거부 전 PixelCopy를 시작했습니다.');
  return { status: 'rejected-before-readback', runID: negativeRunID, readbackStarted };
}

async function runIOS() {
  const simulator = bootedSimulator();
  const build = command('mise', ['exec', '--', 'env',
    `SPINON_V8_DIR=${process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8')}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1',
    'bun', 'run', 'build:ios-sim'], { timeout: 900_000 });
  await saveLog('build.log', build);
  const buildOutput = `${build.stdout}${build.stderr}`;
  const buildFlags = buildOutput.match(/^SPINON_IOS_SIM_BUILD_FLAGS (.*)$/m)?.[1];
  assert.ok(buildFlags?.includes('c04_runtime_gpu=1'), 'iOS runtime GPU fixture flag가 없습니다.');
  assert.ok(buildFlags?.includes('c11_grid_metal_surface_pixels=1'), 'iOS Metal pixel flag가 없습니다.');
  const appSha256 = sha256(await readFile(iosBinaryPath));
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulator.udid, iosAppPath]));
  const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulator.udid,
    packageName, '--spinon-c1132-grid-named-lines', `--spinon-c1132-grid-run-id=${runID}`,
    `--spinon-c11-grid-metal-surface-pixels=${runID}`,
    `--spinon-c11-grid-metal-surface-samples=${samplePayload('base64')}`], { timeout: 90_000 });
  await saveLog('launch.log', launch);
  try {
    const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
    assert.ok(Number.isInteger(pid) && pid > 0, 'iOS Simulator launch가 PID를 반환하지 않았습니다.');
    const log = waitForLog(() => iosProcessLog(simulator.udid, pid), (value) =>
      value.includes(`SPINON_C1132_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`)
        || value.includes(`SPINON_C1132_METAL_PIXEL_SUMMARY status=mismatch run_id=${runID}`)
        || value.includes(`SPINON_C1132_METAL_PIXEL_ERROR run_id=${runID}`),
    'iOS C11.3.2 runtime/Metal pixel 결과');
    await writeFile(resolve(evidenceDirectory, 'runtime.log'), `${log}\n`);
    const frames = assertNamedLineGeometry(log, runID);
    const pixelComparison = compareC11IOSMetalPixelLogs(log, {
      fixture, runID, markerPrefix: 'C1132',
    });
    await writeFile(resolve(evidenceDirectory, 'metal-surface-comparison.json'),
      `${JSON.stringify(pixelComparison, null, 2)}\n`);
    const screenshotPath = resolve(evidenceDirectory, 'ios-screen.png');
    await saveLog('screenshot.log', command('xcrun', ['simctl', 'io', simulator.udid,
      'screenshot', screenshotPath]));
    const screen = await readFile(screenshotPath);
    assert.equal(screen.toString('hex', 0, 8), '89504e470d0a1a0a', 'iOS 화면 캡처가 PNG가 아닙니다.');
    const negative = await runIOSNegative(simulator.udid);
    return {
      metadata: { name: simulator.name, udid: simulator.udid, runtime: simulator.runtime },
      appSha256, frames, pixelComparison, screenSha256: sha256(screen), negative,
    };
  } finally {
    command('xcrun', ['simctl', 'terminate', simulator.udid, packageName], { allowFailure: true });
  }
}

async function runIOSNegative(udid) {
  const negativeRunID = `${runID}-bad-samples`;
  const invalidSamples = fixture.samples.map((sample) => sample.id === 'item-blue'
    ? { ...sample, xCssPx: sample.xCssPx + 1 } : sample);
  const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', udid,
    packageName, '--spinon-c1132-grid-named-lines', `--spinon-c1132-grid-run-id=${negativeRunID}`,
    `--spinon-c11-grid-metal-surface-pixels=${negativeRunID}`,
    `--spinon-c11-grid-metal-surface-samples=${samplePayload('base64', invalidSamples)}`],
  { timeout: 90_000 });
  await saveLog('ios-pixel-allowlist-negative-launch.log', launch);
  const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, 'iOS negative-control process가 없습니다.');
  const log = waitForLog(() => iosProcessLog(udid, pid), (value) =>
    value.includes(`SPINON_C1132_METAL_PIXEL_ERROR run_id=${negativeRunID}`)
      || value.includes(`SPINON_C1132_METAL_PIXEL_BEGIN run_id=${negativeRunID}`),
  'iOS Metal pixel allowlist negative control', 30_000);
  const rejected = log.includes(`SPINON_C1132_METAL_PIXEL_ERROR run_id=${negativeRunID}`);
  const readbackStarted = log.includes(`SPINON_C1132_METAL_PIXEL_BEGIN run_id=${negativeRunID}`);
  await writeFile(resolve(evidenceDirectory, 'ios-pixel-allowlist-negative-control.log'), `${log}\n`);
  assert.ok(rejected, 'iOS가 변경된 표본 좌표를 거부하지 않았습니다.');
  assert.equal(readbackStarted, false, 'iOS가 좌표 거부 전 Metal readback을 시작했습니다.');
  return { status: 'rejected-before-readback', runID: negativeRunID, readbackStarted };
}

await mkdir(evidenceDirectory, { recursive: true });
try {
const referenceBytes = await readFile(resolve(repositoryRoot, fixture.oracle.referencePath));
const chromePngBytes = await readFile(resolve(repositoryRoot, fixture.oracle.screenshotPath));
const oracleComparison = validateC11IOSChromeOracle({
  fixture, referenceBytes, screenshotBytes: chromePngBytes,
});
const platform = target === 'android-physical' ? await runAndroid() : await runIOS();
const result = {
  schema: 'spinon-c11-3-2-grid-named-lines-runtime-result/v1',
  status: 'matched', runID, target,
  fixture: { path: fixturePath, sha256: sha256(await readFile(resolve(repositoryRoot, fixturePath))) },
  oracle: {
    product: fixture.oracle.product, referenceId: fixture.oracle.referenceId,
    referenceSha256: sha256(referenceBytes), screenshotSha256: sha256(chromePngBytes),
    sampleCount: fixture.samples.length, maximumChannelError: 0, oracleComparison,
  },
  platform,
  interpretation: target === 'ios-simulator'
    ? 'Metal drawable 표면을 OS compositor 제출 전에 읽어 고정 Chrome RGBA와 비교했다. 별도 화면 캡처는 UI 확인 자료다.'
    : '연결된 Android 실기기 SurfaceView의 PixelCopy를 고정 Chrome RGBA와 비교했다. 결과는 이 기기와 빌드에 한정된다.',
};
await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
await writeFile(resolve(evidenceDirectory, 'README.md'), [
  '# C11.3.2 Grid named line 모바일 실행', '',
  `- 실행 ID: \`${runID}\``, `- 대상: \`${target}\``,
  `- Chrome 기준: ${fixture.oracle.product} · DPR ${fixture.oracle.deviceScaleFactor} · \`${fixture.sceneId}\``,
  `- runtime: 성공 장면 1개 · 기대 거부 장면 1개 · node 4개`,
  `- 성공 geometry: root 160×120 CSS px, item 80×40 CSS px at (80, 0)`,
  `- 표면 표본: ${platform.pixelComparison.matchedSamples}/${platform.pixelComparison.expectedSamples} exact · 최대 채널 오차 0`,
  `- 좌표 allowlist negative control: ${platform.negative.status}`,
  target === 'ios-simulator'
    ? '- 표면 비교는 Metal drawable readback이며 OS compositor 이후 픽셀은 확인하지 않았다.'
    : '- 표면 비교는 Android 실기기 SurfaceView PixelCopy다. 다른 기기·GPU로 일반화하지 않는다.',
  '- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.',
  '- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.',
  '',
  '## 산출물', '',
  '- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그',
  target === 'ios-simulator' ? '- `ios-screen.png`: iOS Simulator 화면' : '- `pixelcopy.png`: PixelCopy 표면 PNG',
  target === 'ios-simulator' ? '- `metal-surface-comparison.json`: Metal 표본 비교' : '- `android-screen.png`: Android 화면',
  '- `result.json`: fixture·oracle·앱 hash와 실행 결과', '',
].join('\n') + '\n');
process.stdout.write(`${JSON.stringify({ ...result, evidenceDirectory }, null, 2)}\n`);
} catch (error) {
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  throw error;
}
