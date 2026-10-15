import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { access, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const evidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16',
);
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const helperSource = path.join(root, 'tools/css-reference/fixtures/c13_3_android_control_wheel_uinput.c');
const remoteHelper = '/data/local/tmp/spinon-c13-3-control-wheel-uinput';
const bundleID = 'dev.spinon.bootstrap';
const activity = 'dev.spinon.bootstrap/.MainActivity';
const surfaceScale = 2.625;
const targetCssPx = [132, 112];
const controlCases = [
  { control: 'left', axis: 'horizontal', h: 1, v: 0 },
  { control: 'left', axis: 'vertical', h: 0, v: 1 },
  { control: 'right', axis: 'horizontal', h: 1, v: 0 },
  { control: 'right', axis: 'vertical', h: 0, v: 1 },
];

function command(name, args, { timeout = 30_000, encoding = 'utf8' } = {}) {
  return execFileSync(name, args, {
    cwd: root,
    encoding,
    stdio: ['ignore', 'pipe', 'pipe'],
    timeout,
    maxBuffer: 48 * 1024 * 1024,
  });
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function wait(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function adb(serial, ...args) {
  return command('adb', ['-s', serial, ...args]);
}

function adbBuffer(serial, ...args) {
  return command('adb', ['-s', serial, ...args], { encoding: 'buffer' });
}

function processLines(serial) {
  return adb(serial, 'logcat', '-d', '-v', 'brief')
    .split('\n')
    .filter((line) => line.includes('SpinonBootstrap(')
      && (line.includes('SPINON_C133_') || line.includes('SPINON_C0410_DRAW')));
}

function latestDraw(lines) {
  return lines.filter((line) => line.includes('SPINON_C0410_DRAW')).at(-1);
}

function surfaceBounds(lines, phase) {
  const line = lines.findLast((item) => item.includes(`SPINON_C133_SURFACE_BOUNDS phase=${phase} `));
  if (!line) return null;
  const match = line.match(/screen_px=(\d+),(\d+) size_px=(\d+)x(\d+)/);
  assert.ok(match, `malformed SurfaceView bounds: ${line}`);
  const left = Number(match[1]);
  const top = Number(match[2]);
  return {
    left,
    top,
    right: left + Number(match[3]),
    bottom: top + Number(match[4]),
  };
}

function androidSimulator(serial) {
  assert.match(serial, /^emulator-\d+$/, `refusing non-emulator Android target ${serial}`);
  assert.equal(adb(serial, 'get-state').trim(), 'device');
  const device = {
    model: adb(serial, 'shell', 'getprop', 'ro.product.model').trim(),
    qemu: adb(serial, 'shell', 'getprop', 'ro.kernel.qemu').trim(),
    api: adb(serial, 'shell', 'getprop', 'ro.build.version.sdk').trim(),
    abi: adb(serial, 'shell', 'getprop', 'ro.product.cpu.abi').trim(),
    release: adb(serial, 'shell', 'getprop', 'ro.build.version.release').trim(),
    pageSize: adb(serial, 'shell', 'getconf', 'PAGE_SIZE').trim(),
    size: adb(serial, 'shell', 'wm', 'size').trim(),
    density: adb(serial, 'shell', 'wm', 'density').trim(),
  };
  assert.equal(device.qemu, '1', `refusing non-AVD target ${serial}`);
  assert.equal(device.api, '37', `expected Android API 37, received ${device.api}`);
  assert.equal(device.abi, 'arm64-v8a', `expected ARM64 AVD, received ${device.abi}`);
  assert.equal(device.pageSize, '16384', `unexpected AVD page size ${device.pageSize}`);
  return device;
}

async function findAndroidClang() {
  const explicitNdk = process.env.ANDROID_NDK_HOME || process.env.ANDROID_NDK_ROOT;
  const sdkRoot = process.env.ANDROID_SDK_ROOT
    || process.env.ANDROID_HOME
    || path.join(os.homedir(), 'Library/Android/sdk');
  const pinnedVersion = (await readFile(path.join(root, 'tools/android-ndk-version.txt'), 'utf8')).trim();
  const ndkRoots = [];
  if (explicitNdk && path.basename(explicitNdk) === pinnedVersion) ndkRoots.push(explicitNdk);
  const pinnedSdkNdk = path.join(sdkRoot, 'ndk', pinnedVersion);
  if (!ndkRoots.includes(pinnedSdkNdk)) ndkRoots.push(pinnedSdkNdk);
  for (const ndkRoot of ndkRoots) {
    for (const hostTag of ['darwin-arm64', 'darwin-x86_64', 'linux-x86_64']) {
      const clang = path.join(ndkRoot, 'toolchains/llvm/prebuilt', hostTag, 'bin/aarch64-linux-android35-clang');
      try {
        await access(clang);
        return {
          clang,
          readelf: path.join(ndkRoot, 'toolchains/llvm/prebuilt', hostTag, 'bin/llvm-readelf'),
          ndkRoot,
        };
      } catch {
        // Try another installed host toolchain.
      }
    }
  }
  assert.fail(`Android NDK ${pinnedVersion} clang not found; install the pinned NDK or set ANDROID_SDK_ROOT`);
}

function compileHelper(toolchain, outputPath) {
  command(toolchain.clang, [
    '-O2', '-Wall', '-Wextra', '-Werror', '-Wl,-z,max-page-size=16384',
    '-o', outputPath, helperSource,
  ], { timeout: 60_000 });
  const programHeaders = command(toolchain.readelf, ['-l', outputPath]);
  const loadAlignments = programHeaders.split('\n')
    .filter((line) => line.trimStart().startsWith('LOAD'))
    .map((line) => line.match(/\s(0x[0-9a-f]+)$/)?.[1]);
  assert.ok(loadAlignments.length > 0, 'uinput helper ELF has no LOAD segments');
  assert.ok(loadAlignments.every((alignment) => alignment === '0x4000'),
    `uinput helper LOAD segments must all use 16 KB alignment: ${loadAlignments.join(',')}`);
  return { programHeaders, binary: readFileSync(outputPath) };
}

async function ready(serial) {
  for (let attempt = 0; attempt < 120; attempt += 1) {
    const lines = processLines(serial);
    const readyLine = lines.find((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    const bounds = surfaceBounds(lines, 'ready');
    if (readyLine && evalReadyLine && bounds
        && latestDraw(lines)?.includes('scroll_state_revision=1')) {
      await wait(1_000);
      return { lines, readyLine, evalReadyLine, bounds };
    }
    await wait(100);
  }
  assert.fail(`${serial}: fixture did not reach a presented revision-1 surface`);
}

async function launch(serial) {
  adb(serial, 'shell', 'am', 'force-stop', bundleID);
  adb(serial, 'logcat', '-c');
  adb(serial, 'shell', 'am', 'start', '-W', '-n', activity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', '1');
  return ready(serial);
}

function compareSurface(beforeBytes, afterBytes, beforeBounds, afterBounds) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.equal(before.width, after.width);
  assert.equal(before.height, after.height);
  const width = beforeBounds.right - beforeBounds.left;
  const height = beforeBounds.bottom - beforeBounds.top;
  assert.deepEqual([width, height], [afterBounds.right - afterBounds.left, afterBounds.bottom - afterBounds.top]);
  assert.ok(beforeBounds.left >= 0 && beforeBounds.top >= 0
    && beforeBounds.right <= before.width && beforeBounds.bottom <= before.height,
  'SurfaceView bounds exceed the before screenshot');
  assert.ok(afterBounds.left >= 0 && afterBounds.top >= 0
    && afterBounds.right <= after.width && afterBounds.bottom <= after.height,
  'SurfaceView bounds exceed the after screenshot');
  let changedPixels = 0;
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const first = before.getPixel(beforeBounds.left + x, beforeBounds.top + y);
      const second = after.getPixel(afterBounds.left + x, afterBounds.top + y);
      if (first.some((channel, index) => channel !== second[index])) changedPixels += 1;
    }
  }
  return { width, height, comparedPixels: width * height, changedPixels };
}

function validateInputPoint(bounds) {
  const screenPx = [
    bounds.left + Math.round(targetCssPx[0] * surfaceScale),
    bounds.top + Math.round(targetCssPx[1] * surfaceScale),
  ];
  assert.ok(screenPx[0] > 0 && screenPx[1] > 0);
  return screenPx;
}

function validateReleasedInputState(dump) {
  assert.ok(!dump.includes('Spinon C13.3 virtual wheel probe'), 'virtual input device was not removed');
  const keyDownLines = dump.split('\n').filter((line) => line.includes('KeyDowns:'));
  const metaStateLines = dump.split('\n').filter((line) => line.includes('MetaState:'));
  assert.ok(keyDownLines.length > 0 && keyDownLines.every((line) => line.includes('0 keys currently down')),
    'a key remained down after uinput device destruction');
  assert.ok(metaStateLines.length > 0 && metaStateLines.every((line) => line.includes('MetaState: 0x0')),
    'Android InputReader retained a modifier after uinput device destruction');
  return { virtualDeviceRemoved: true, keyDowns: '0', metaState: '0x0' };
}

async function captureRejectedCase(serial, device, definition, binarySha256, captureScreenshots) {
  const { lines: initialLines, readyLine, evalReadyLine, bounds } = await launch(serial);
  const screenPx = validateInputPoint(bounds);
  const beforeBytes = adbBuffer(serial, 'exec-out', 'screencap', '-p');
  const helperOutput = adb(serial, 'shell', remoteHelper, definition.control, definition.axis,
    String(screenPx[0]), String(screenPx[1])).trim();
  await wait(700);
  const lines = processLines(serial);
  const rejection = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL_REJECT'));
  assert.ok(rejection, `${definition.control}/${definition.axis}: no native rejection log`);
  assert.ok(rejection.includes('reason=control-wheel'), rejection);
  assert.ok(rejection.includes('source=MouseWheel'), rejection);
  const expectedMeta = definition.control === 'left' ? 12288 : 20480;
  assert.ok(rejection.includes(`meta=${expectedMeta} modifiers=2`), rejection);
  const expectedAxes = `hscroll=${definition.h.toFixed(1)} vscroll=${definition.v.toFixed(1)}`;
  assert.ok(rejection.includes(expectedAxes), `${definition.control}/${definition.axis}: ${rejection}`);
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=')),
    `${definition.control}/${definition.axis}: rejected input reached native enqueue`);
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel')),
    `${definition.control}/${definition.axis}: rejected input reached the queue consumer`);
  const drawLines = lines.filter((line) => line.includes('SPINON_C0410_DRAW'));
  assert.ok(drawLines.length > 0, 'fixture emitted no baseline draw');
  assert.ok(drawLines.every((line) => line.includes('scroll_state_revision=1')
    && line.includes('scroll_offsets=[0,0;0,0;86,106]')),
  `${definition.control}/${definition.axis}: rejected input changed draw state`);
  const drawLine = latestDraw(lines);
  const afterBounds = surfaceBounds(lines, 'ready') || bounds;
  const afterBytes = adbBuffer(serial, 'exec-out', 'screencap', '-p');
  const pixelComparison = compareSurface(beforeBytes, afterBytes, bounds, afterBounds);
  assert.equal(pixelComparison.changedPixels, 0,
    `${definition.control}/${definition.axis}: rejected input changed SurfaceView pixels`);
  const releasedInputState = validateReleasedInputState(adb(serial, 'shell', 'dumpsys', 'input'));

  const prefix = `android-control-${definition.control}-${definition.axis}`;
  if (captureScreenshots) {
    await writeFile(path.join(evidenceDirectory, `${prefix}-before.png`), beforeBytes);
    await writeFile(path.join(evidenceDirectory, `${prefix}-after.png`), afterBytes);
  }
  const record = {
    control: definition.control,
    axis: definition.axis,
    input: {
      device: 'virtual uinput pointer',
      key: definition.control === 'left' ? 'KEY_LEFTCTRL' : 'KEY_RIGHTCTRL',
      relativeAxis: definition.axis === 'horizontal' ? 'REL_HWHEEL' : 'REL_WHEEL',
      relativeValue: 1,
      targetCssPx,
      screenPx,
    },
    observedRejection: rejection,
    nativeEnqueueCount: 0,
    queueProcessCount: 0,
    drawLine,
    pixelComparison,
    releasedInputState,
    helperOutput,
  };
  const log = [
    `platform=Android API ${device.api} ARM64 AVD ${device.model} (${device.release})`,
    `serial=${serial} qemu=${device.qemu} page_size=${device.pageSize} display=${device.size} density=${device.density}`,
    `helper_sha256=${binarySha256}`,
    `control=${record.input.key} axis=${record.input.relativeAxis} value=${record.input.relativeValue}`,
    `target_css_px=${targetCssPx.join(',')} screen_px=${screenPx.join(',')} scale_px_per_css_px=${surfaceScale}`,
    `expected_rejection=control-wheel hscroll=${definition.h} vscroll=${definition.v}`,
    `helper=${helperOutput}`,
    'fixture_ready=', evalReadyLine, readyLine,
    'observed_logs=', ...lines,
    '',
    `pixel_comparison=${JSON.stringify(pixelComparison)}`,
    `post_input_state=${JSON.stringify(releasedInputState)}`,
  ].join('\n');
  await writeFile(path.join(evidenceDirectory, `${prefix}.log`), `${log}\n`);
  return record;
}

async function capturePositiveControl(serial, device) {
  const { lines: initialLines, readyLine, evalReadyLine, bounds } = await launch(serial);
  const screenPx = validateInputPoint(bounds);
  adb(serial, 'shell', 'input', 'mouse', 'scroll', String(screenPx[0]), String(screenPx[1]),
    '--axis', 'HSCROLL,1', '--axis', 'VSCROLL,0');
  let lines = [];
  for (let attempt = 0; attempt < 40; attempt += 1) {
    lines = processLines(serial);
    if (lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'))
        && latestDraw(lines)?.includes('scroll_state_revision=2')) break;
    await wait(100);
  }
  const native = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=MouseWheel'));
  const process = lines.find((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'));
  const draw = latestDraw(lines);
  assert.ok(native?.includes('modifiers=0'), `ordinary wheel carried a modifier: ${native}`);
  assert.ok(native.includes('sequence=1 admitted=true admission=Queued target=7'), native);
  assert.ok(process?.includes('scroll_state_revision=2'), process);
  assert.ok(draw?.includes('scroll_state_revision=2')
    && draw.includes('scroll_offsets=[0,0;64,0;86,106]'), draw);
  const record = { input: { source: 'adb input mouse', screenPx }, native, process, draw };
  await writeFile(path.join(evidenceDirectory, 'ordinary-wheel-positive-control.log'), [
    `platform=Android API ${device.api} ARM64 AVD ${device.model} (${device.release})`,
    `serial=${serial} qemu=${device.qemu} page_size=${device.pageSize} display=${device.size} density=${device.density}`,
    'input=ordinary horizontal wheel with no Control key',
    `target_css_px=${targetCssPx.join(',')} screen_px=${screenPx.join(',')}`,
    'fixture_ready=', evalReadyLine, readyLine,
    'observed_logs=', ...lines,
  ].join('\n') + '\n');
  return record;
}

async function main() {
  const serialIndex = process.argv.indexOf('--android-serial');
  const serial = serialIndex >= 0 ? process.argv[serialIndex + 1] : null;
  assert.ok(serial, 'pass --android-serial emulator-N');
  await mkdir(evidenceDirectory, { recursive: true });
  const device = androidSimulator(serial);
  const toolchain = await findAndroidClang();
  const temporaryDirectory = await mkdtemp(path.join(os.tmpdir(), 'spinon-c13-3-control-wheel-'));
  const helperPath = path.join(temporaryDirectory, 'control-wheel-uinput');
  let remoteInstalled = false;
  let successful = false;
  try {
    const compiled = compileHelper(toolchain, helperPath);
    const helperSha256 = sha256(compiled.binary);
    const helperSourceSha256 = sha256(await readFile(helperSource));
    const programHeaders = `${compiled.programHeaders.replace(/[ \t]+$/gm, '').trimEnd()}\n`;
    await writeFile(path.join(evidenceDirectory, 'helper-program-headers.txt'), programHeaders);
    const apk = await readFile(apkPath);
    assert.ok(apk.length > 0, `empty Android APK at ${apkPath}`);
    adb(serial, 'install', '-r', apkPath);
    remoteInstalled = true;
    adb(serial, 'push', helperPath, remoteHelper);
    adb(serial, 'shell', 'chmod', '755', remoteHelper);
    adb(serial, 'logcat', '-c');

    const cases = [];
    for (let index = 0; index < controlCases.length; index += 1) {
      cases.push(await captureRejectedCase(serial, device, controlCases[index], helperSha256, index === 0));
    }
    const positiveControl = await capturePositiveControl(serial, device);
    const summary = {
      schema: 'spinon-c13-3-android-control-wheel-runtime-v1',
      capturedAt: new Date().toISOString(),
      sourceRevision: command('git', ['rev-parse', 'HEAD']).trim(),
      android: { serial, ...device },
      build: {
        apkSha256: sha256(apk),
        helperSha256,
        helperSourceSha256,
        captureScriptSha256: sha256(await readFile(fileURLToPath(import.meta.url))),
        ndk: path.basename(toolchain.ndkRoot),
        helperElfAlignment: '16384 bytes',
        optInFlags: ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1'],
      },
      fixture: {
        scenario: 'auto-boundary',
        viewportCssPx: [400, 240],
        targetCssPx,
        surfacePxPerCssPx: surfaceScale,
      },
      controlWheelCases: cases,
      positiveControl,
      limits: [
        'Android API 37 ARM64 AVD InputReader path only; no physical device or external trackpad was used.',
        'This proves the opt-in Android fixture rejection path, not public DOM events or product-level support.',
        'iPadOS Simulator callback/sign/adapter remains unobserved and outside this evidence.',
        'C13.3 and its official completed-item count remain incomplete/unchanged.',
      ],
    };
    await writeFile(path.join(evidenceDirectory, 'android-control-wheel-runtime.json'),
      `${JSON.stringify(summary, null, 2)}\n`);
    const readme = [
      '# C13.3 Android Control-wheel AVD 실행 근거 · 2026-10-16',
      '',
      `캡처 시각: ${summary.capturedAt} · APK SHA-256: \`${summary.build.apkSha256}\` · helper SHA-256: \`${helperSha256}\`.`,
      '',
      `Android API ${device.api} ARM64 AVD \`${serial}\`에서 가상 uinput pointer의 Control-left/right와 horizontal/vertical wheel을 앱 native \`ACTION_SCROLL\` callback까지 전달했다. 좌우 Control × 두 축 ${cases.length}/${cases.length} case 모두 좌 Ctrl \`meta=12288\`, 우 Ctrl \`meta=20480\`, normalized \`modifiers=2\`와 \`reason=control-wheel\`로 queue 전에 거부됐다.`,
      '',
      `각 case는 fixture를 fresh launch하고 화면 point \`(132,112) CSS px\`를 SurfaceView screen origin에 scale \`${surfaceScale}\`로 더해 계산했다. 네 case에서 enqueue/process는 0건, draw revision 1, offsets \`[0,0;0,0;86,106]\`, SurfaceView \`${cases[0].pixelComparison.comparedPixels.toLocaleString()}\` pixels 중 changed 0개였다. 입력 종료 뒤 virtual uinput device는 제거됐고 Android key-down/meta state는 모두 0이다.`,
      '',
      '![Control-wheel 거부 전 Android API 37 AVD 화면](./android-control-left-horizontal-before.png)',
      '',
      '![Control-wheel 거부 후 Android API 37 AVD 화면](./android-control-left-horizontal-after.png)',
      '',
      `일반 wheel positive control은 modifier 0, target 7 queue admission, revision 2와 offsets \`[0,0;64,0;86,106]\`을 만들었다. 전체 raw logs는 \`android-control-*.log\`와 \`ordinary-wheel-positive-control.log\`, machine record는 \`android-control-wheel-runtime.json\`에 있다.`,
      '',
      '이전 ADB 시도에서 `input keyevent`와 별도 `input mouse scroll` 명령은 Control meta를 공유하지 않아 callback에 modifier가 0으로 들어왔다. 이 helper는 한 Linux uinput device에서 Control key-down, wheel event, key-up을 순서대로 보내 같은 Android InputReader device의 modifier state를 사용한다. 16 KB AVD를 위해 ELF LOAD alignment 16 KB를 적용한다. 실패 시 helper는 `UI_DEV_DESTROY`와 close로 device를 정리한다.',
      '',
      '이 결과는 API 37 ARM64 AVD의 opt-in fixture 경로만 확인한다. 실기기, 외장 trackpad, iOS callback/sign, public DOM events, compositor 이후 scanout 및 C13.3 전체 완료를 증명하지 않는다. C13.3은 미완료이고 Tailnet 로드맵 전체 완료 수는 61/197 그대로다.',
      '',
      'opt-in APK 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 mise exec -- bun run build:android` (고정 V8 source/build와 Android NDK 설정은 `platforms/android/README.md`를 따른다).',
      '',
      '실행: `bun tools/css-reference/capture-c13-3-android-control-wheel-runtime.mjs --android-serial emulator-5554`. 이 도구는 API 37 ARM64 AVD `emulator-N`만 허용하며 물리 device와 `localhost` alias를 거부한다.',
      '',
    ].join('\n');
    await writeFile(path.join(evidenceDirectory, 'README.md'), readme);
    successful = true;
    process.stdout.write(`${JSON.stringify({
      evidenceDirectory: path.relative(root, evidenceDirectory),
      android: summary.android,
      apkSha256: summary.build.apkSha256,
      helperSha256,
      controlCases: cases.length,
      changedSurfacePixels: cases.map((item) => item.pixelComparison.changedPixels),
      positiveControl: {
        modifier: '0',
        queued: positiveControl.native.includes('admission=Queued'),
        drawRevision: '2',
      },
    }, null, 2)}\n`);
  } finally {
    if (remoteInstalled) {
      try {
        adb(serial, 'shell', 'rm', '-f', remoteHelper);
      } catch (error) {
        if (successful) throw error;
        // Preserve the original failure; device removal is asserted before success.
      }
    }
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
}

await main();
