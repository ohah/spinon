import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { access, lstat, mkdir, mkdtemp, readFile, realpath, rename, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const evidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-native-wheel-api29-2026-10-17',
);
const evidenceRoot = path.join(root, 'spec/internal/evidence');
let captureDirectory = null;
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const helperSource = path.join(root, 'tools/css-reference/fixtures/c13_3_android_wheel_uinput.c');
const bundleId = 'dev.spinon.bootstrap';
const activity = 'dev.spinon.bootstrap/.MainActivity';
const remoteHelper = '/data/local/tmp/spinon-c133-api29-wheel-matrix';
const cssWidth = 400;
const cssHeight = 240;
const scrollport = { left: 16, top: 16, right: 384, bottom: 224 };
const targetCssPx = [132, 112];
const deviceName = 'Spinon C13.3 API29 wheel matrix';
const directions = [
  { name: 'h-positive', h: 1, v: 0, delta: [64, 0], root: [0, 0], outer: [64, 0], inner: [86, 106] },
  { name: 'h-negative', h: -1, v: 0, delta: [-64, 0], root: [0, 0], outer: [0, 0], inner: [22, 106] },
  { name: 'v-positive', h: 0, v: 1, delta: [0, -64], root: [0, 0], outer: [0, 0], inner: [86, 42] },
  { name: 'v-negative', h: 0, v: -1, delta: [0, 64], root: [0, 0], outer: [0, 64], inner: [86, 106] },
  { name: 'diagonal-positive-positive', h: 1, v: 1, delta: [64, -64], root: [0, 0], outer: [64, 0], inner: [86, 42], screenshot: true },
  { name: 'diagonal-negative-negative', h: -1, v: -1, delta: [-64, 64], root: [0, 0], outer: [0, 64], inner: [22, 106] },
  { name: 'diagonal-positive-negative', h: 1, v: -1, delta: [64, 64], root: [0, 0], outer: [64, 64], inner: [86, 106] },
  { name: 'diagonal-negative-positive', h: -1, v: 1, delta: [-64, -64], root: [0, 0], outer: [0, 0], inner: [22, 42] },
];
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

function appLines(serial, pid = null) {
  const lines = adb(serial, 'logcat', '-d', '-v', 'brief')
    .split('\n')
    .filter((line) => line.includes('SpinonBootstrap(')
      && (line.includes('SPINON_C133_') || line.includes('SPINON_C0410_DRAW')
        || line.includes('SPINON_C0410_RENDERER=backend=')));
  return pid === null ? lines : lines.filter((line) => {
    const match = line.match(/SpinonBootstrap\(\s*(\d+)\)/);
    return match?.[1] === String(pid);
  });
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

function scaleFromBounds(bounds) {
  const scaleX = (bounds.right - bounds.left) / cssWidth;
  const scaleY = (bounds.bottom - bounds.top) / cssHeight;
  assert.ok(Number.isFinite(scaleX) && scaleX > 0);
  assert.ok(Number.isFinite(scaleY) && scaleY > 0);
  assert.ok(Math.abs(scaleX - scaleY) <= 0.01, `non-uniform SurfaceView scale ${scaleX},${scaleY}`);
  return (scaleX + scaleY) / 2;
}

function offsetsFromProcess(line) {
  const match = line.match(/scroll_state_revision=(\d+) offsets=root:([-\d.]+),([-\d.]+);outer:([-\d.]+),([-\d.]+);inner:([-\d.]+),([-\d.]+)/);
  assert.ok(match, `process log omitted owner offsets: ${line}`);
  return {
    revision: Number(match[1]),
    root: [Number(match[2]), Number(match[3])],
    outer: [Number(match[4]), Number(match[5])],
    inner: [Number(match[6]), Number(match[7])],
  };
}

function comparePixelRegions(beforeBytes, afterBytes, beforeBounds, afterBounds, scale, mode) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.deepEqual([before.width, before.height], [after.width, after.height]);
  const width = beforeBounds.right - beforeBounds.left;
  const height = beforeBounds.bottom - beforeBounds.top;
  assert.deepEqual([afterBounds.right - afterBounds.left, afterBounds.bottom - afterBounds.top], [width, height]);
  assert.deepEqual([width, height], [Math.round(cssWidth * scale), Math.round(cssHeight * scale)]);
  for (const [label, image, bounds] of [
    ['before', before, beforeBounds],
    ['after', after, afterBounds],
  ]) {
    assert.ok(bounds.left >= 0 && bounds.top >= 0
      && bounds.right <= image.width && bounds.bottom <= image.height,
    `${label} SurfaceView crop exceeds screenshot bounds`);
  }
  let comparedPixels = 0;
  let changedPixels = 0;
  let outsideChangedPixels = 0;
  let insideChangedPixels = 0;
  const left = Math.round(scrollport.left * scale);
  const right = Math.round(scrollport.right * scale);
  const top = Math.round(scrollport.top * scale);
  const bottom = Math.round(scrollport.bottom * scale);
  for (let localY = 0; localY < height; localY += 1) {
    for (let localX = 0; localX < width; localX += 1) {
      const first = before.getPixel(beforeBounds.left + localX, beforeBounds.top + localY);
      const second = after.getPixel(afterBounds.left + localX, afterBounds.top + localY);
      const changed = first.some((channel, index) => channel !== second[index]);
      comparedPixels += 1;
      if (changed) {
        changedPixels += 1;
        if (localX < left || localX >= right || localY < top || localY >= bottom) {
          outsideChangedPixels += 1;
        } else {
          insideChangedPixels += 1;
        }
      }
    }
  }
  if (mode === 'scroll') {
    assert.equal(outsideChangedPixels, 0, `scroll changed ${outsideChangedPixels} pixels outside root scrollport`);
    assert.ok(insideChangedPixels > 0, 'scroll did not change any pixels inside root scrollport');
  } else {
    assert.equal(changedPixels, 0, `rejected Control-wheel changed ${changedPixels} SurfaceView pixels`);
  }
  return { comparedPixels, changedPixels, outsideChangedPixels, insideChangedPixels, mode };
}

async function ensureEvidenceDirectory() {
  const realRoot = await realpath(root);
  const realEvidenceRoot = await realpath(evidenceRoot);
  assert.ok(realEvidenceRoot.startsWith(`${realRoot}${path.sep}`), 'evidence root resolves outside repository');
  const relative = path.relative(evidenceRoot, evidenceDirectory);
  assert.ok(relative && !relative.startsWith(`..${path.sep}`) && relative !== '..');
  let current = evidenceRoot;
  for (const segment of relative.split(path.sep)) {
    current = path.join(current, segment);
    try {
      await lstat(current);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      await mkdir(current);
    }
    const stat = await lstat(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe evidence path: ${current}`);
    assert.ok((await realpath(current)).startsWith(`${realEvidenceRoot}${path.sep}`));
  }
}

function androidApi29Avd(serial) {
  assert.match(serial, /^emulator-\d+$/, `refusing non-emulator Android target ${serial}`);
  assert.equal(adb(serial, 'get-state').trim(), 'device');
  const device = {
    model: adb(serial, 'shell', 'getprop', 'ro.product.model').trim(),
    qemu: adb(serial, 'shell', 'getprop', 'ro.kernel.qemu').trim(),
    avdName: adb(serial, 'shell', 'getprop', 'ro.kernel.qemu.avd_name').trim(),
    api: adb(serial, 'shell', 'getprop', 'ro.build.version.sdk').trim(),
    release: adb(serial, 'shell', 'getprop', 'ro.build.version.release').trim(),
    abi: adb(serial, 'shell', 'getprop', 'ro.product.cpu.abi').trim(),
    pageSize: adb(serial, 'shell', 'getconf', 'PAGE_SIZE').trim(),
    size: adb(serial, 'shell', 'wm', 'size').trim(),
    density: adb(serial, 'shell', 'wm', 'density').trim(),
  };
  assert.equal(device.qemu, '1', `refusing non-AVD target ${serial}`);
  assert.equal(device.api, '29', `expected Android API 29, received ${device.api}`);
  assert.equal(device.abi, 'arm64-v8a', `expected ARM64 AVD, received ${device.abi}`);
  assert.equal(device.pageSize, '4096', `expected API 29 4 KB pages, received ${device.pageSize}`);
  assert.equal(device.avdName, 'spinon_r05_api29_fallback', `unexpected API29 AVD ${device.avdName}`);
  assert.equal(device.size, 'Physical size: 1080x1920', `unexpected API29 AVD size ${device.size}`);
  assert.equal(device.density, 'Physical density: 420', `unexpected API29 AVD density ${device.density}`);
  return device;
}

async function findAndroidClang() {
  const sdkRoot = process.env.ANDROID_SDK_ROOT
    || process.env.ANDROID_HOME
    || path.join(os.homedir(), 'Library/Android/sdk');
  const ndkVersion = (await readFile(path.join(root, 'tools/android-ndk-version.txt'), 'utf8')).trim();
  const ndkRoot = path.join(sdkRoot, 'ndk', ndkVersion);
  for (const hostTag of ['darwin-arm64', 'darwin-x86_64']) {
    const clang = path.join(ndkRoot, 'toolchains/llvm/prebuilt', hostTag, 'bin/aarch64-linux-android29-clang');
    try {
      await access(clang);
      return { clang, ndkRoot, ndkVersion };
    } catch {
      // Try the other installed macOS NDK host toolchain.
    }
  }
  assert.fail(`pinned Android NDK ${ndkVersion} API 29 clang was not found under ${ndkRoot}`);
}

function compileHelper(clang, outputPath) {
  command(clang, [
    '-O2', '-Wall', '-Wextra', '-Werror', '-Wl,-z,max-page-size=4096',
    '-o', outputPath, helperSource,
  ], { timeout: 60_000 });
  return readFile(outputPath);
}

async function ready(serial) {
  for (let attempt = 0; attempt < 120; attempt += 1) {
    let pidOutput = '';
    try {
      pidOutput = adb(serial, 'shell', 'pidof', bundleId).trim();
    } catch {
      await wait(100);
      continue;
    }
    const pid = pidOutput.split(/\s+/).map(Number).find(Number.isFinite);
    const lines = pid ? appLines(serial, pid) : [];
    const readyLine = lines.find((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    const bounds = surfaceBounds(lines, 'ready');
    if (readyLine && evalReadyLine && bounds && latestDraw(lines)?.includes('scroll_state_revision=1')) {
      assert.ok(evalReadyLine.includes('scenario=auto-boundary'));
      assert.ok(evalReadyLine.includes('offsets=root:0,0;outer:0,0;inner:86,106'));
      assert.ok(latestDraw(lines).includes('scroll_offsets=[0,0;0,0;86,106]'));
      await wait(500);
      return { lines, readyLine, evalReadyLine, bounds, pid };
    }
    await wait(100);
  }
  assert.fail(`${serial}: API29 fixture did not publish its initial revision-1 surface`);
}

async function launch(serial) {
  adb(serial, 'shell', 'am', 'force-stop', bundleId);
  adb(serial, 'logcat', '-c', '-b', 'all');
  adb(serial, 'shell', 'am', 'start', '-W', '-n', activity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', '1');
  return ready(serial);
}

function inputScreenPoint(bounds) {
  const scale = scaleFromBounds(bounds);
  return {
    scale,
    point: [
      bounds.left + Math.round(targetCssPx[0] * scale),
      bounds.top + Math.round(targetCssPx[1] * scale),
    ],
  };
}

async function injectWheel(serial, screenPoint, h, v, control) {
  const result = command('adb', ['-s', serial, 'shell', remoteHelper,
    String(screenPoint[0]), String(screenPoint[1]), String(h), String(v), control],
  { timeout: 15_000 });
  assert.ok(result.includes('device-created') && result.includes('device-destroyed'),
    `uinput helper did not report a complete create/destroy lifecycle: ${result}`);
  const devices = adb(serial, 'shell', 'getevent', '-lp');
  assert.ok(!devices.includes(deviceName), 'uinput mouse remained registered after event completion');
  return result;
}

async function writeRunLog(fileName, device, serial, apkSha256, preamble, lines, comparison) {
  await writeFile(path.join(captureDirectory, fileName), [
    `platform=Android API ${device.api} (${device.release}) AVD ${device.avdName}`,
    `serial=${serial} model=${device.model} abi=${device.abi} page_size=${device.pageSize}`,
    `display=${device.size} density=${device.density} qemu=${device.qemu}`,
    `apk_sha256=${apkSha256}`,
    preamble,
    ...lines,
    comparison ? `pixel_comparison=${JSON.stringify(comparison)}` : '',
    '',
  ].filter(Boolean).join('\n'));
}

async function captureDirection(serial, device, apkSha256, direction) {
  const { readyLine, evalReadyLine, bounds, pid } = await launch(serial);
  const { scale, point } = inputScreenPoint(bounds);
  const beforePath = path.join(captureDirectory, `android-${direction.name}-before.png`);
  const afterPath = path.join(captureDirectory, `android-${direction.name}-after.png`);
  const before = direction.screenshot ? adbBuffer(serial, 'exec-out', 'screencap', '-p') : null;
  const helperOutput = await injectWheel(serial, point, direction.h, direction.v, 'none');
  let lines = [];
  for (let attempt = 0; attempt < 50; attempt += 1) {
    lines = appLines(serial, pid);
    if (lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'))
        && latestDraw(lines)?.includes('scroll_state_revision=2')
        && surfaceBounds(lines, 'wheel')) break;
    await wait(100);
  }

  const wheelLines = lines.filter((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=MouseWheel'));
  const processLines = lines.filter((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'));
  assert.equal(wheelLines.length, 1, `${direction.name}: expected exactly one native wheel callback`);
  assert.equal(processLines.length, 1, `${direction.name}: expected exactly one wheel queue commit`);
  const wheelLine = wheelLines[0];
  const processLine = processLines[0];
  assert.ok(wheelLine.includes('action=8 source_bits=0x2002 event_source_bits=0x2002'));
  assert.ok(wheelLine.includes('pointer_tool=mouse'));
  assert.ok(wheelLine.includes(`hscroll=${direction.h}.0`));
  assert.ok(wheelLine.includes(`vscroll=${direction.v}.0`));
  assert.ok(wheelLine.includes('unit=AndroidPhysicalPixels phase=None modifiers=0'));
  assert.ok(wheelLine.includes('sequence=1 admitted=true admission=Queued target=7'));

  const factors = wheelLine.match(/hfactor_px=([-\d.]+) vfactor_px=([-\d.]+).*scale_px_per_css_px=([-\d.]+)/);
  const pointMatch = wheelLine.match(/point_css_px=([-\d.]+),([-\d.]+) delta_css_px=([-\d.]+),([-\d.]+)/);
  assert.ok(factors && pointMatch, `${direction.name}: callback omitted factor, scale, point, or normalized delta`);
  const horizontalFactor = Number(factors[1]);
  const verticalFactor = Number(factors[2]);
  const reportedScale = Number(factors[3]);
  assert.ok(horizontalFactor > 0 && verticalFactor > 0 && reportedScale > 0);
  assert.ok(Math.abs(reportedScale - scale) <= 0.001, `${direction.name}: callback scale differs from SurfaceView bounds`);
  const observedPoint = [Number(pointMatch[1]), Number(pointMatch[2])];
  const observedDelta = [Number(pointMatch[3]), Number(pointMatch[4])];
  const pointTolerance = 0.5 / scale + 0.001;
  for (let axis = 0; axis < 2; axis += 1) {
    assert.ok(Math.abs(observedPoint[axis] - targetCssPx[axis]) <= pointTolerance,
      `${direction.name}: point ${observedPoint} exceeds screen-pixel rounding tolerance`);
  }
  const expectedFormulaDelta = [
    direction.h * horizontalFactor / scale,
    -direction.v * verticalFactor / scale,
  ];
  for (let axis = 0; axis < 2; axis += 1) {
    assert.ok(Math.abs(observedDelta[axis] - expectedFormulaDelta[axis]) <= 0.01,
      `${direction.name}: delta ${observedDelta} does not match Android factor/scale formula ${expectedFormulaDelta}`);
    assert.ok(Math.abs(observedDelta[axis] - direction.delta[axis]) <= 0.01,
      `${direction.name}: delta ${observedDelta} differs from fixed C13.3 oracle ${direction.delta}`);
  }

  const offsetsMatch = processLine.match(/scroll_state_revision=(\d+) offsets=/);
  assert.ok(offsetsMatch);
  const offsets = offsetsFromProcess(processLine);
  assert.equal(offsets.revision, 2);
  assert.deepEqual(offsets.root, direction.root);
  assert.deepEqual(offsets.outer, direction.outer);
  assert.deepEqual(offsets.inner, direction.inner);
  const drawLine = latestDraw(lines);
  assert.ok(drawLine?.includes('status=0 presented'));
  assert.ok(drawLine.includes('scroll_state_revision=2'));
  assert.ok(drawLine.includes(`scroll_offsets=[${direction.root.join(',')};${direction.outer.join(',')};${direction.inner.join(',')}]`));
  const callbackOrigin = wheelLine.match(/surface_origin_screen_px=(\d+),(\d+)/);
  assert.ok(callbackOrigin);
  assert.deepEqual([Number(callbackOrigin[1]), Number(callbackOrigin[2])], [bounds.left, bounds.top]);

  let pixelComparison;
  let screenshotSha256 = null;
  let afterBounds;
  if (direction.screenshot) {
    await writeFile(beforePath, before);
    await wait(250);
    lines = appLines(serial, pid);
    afterBounds = surfaceBounds(lines, 'wheel');
    assert.ok(afterBounds, 'missing post-wheel SurfaceView bounds');
    const after = adbBuffer(serial, 'exec-out', 'screencap', '-p');
    await writeFile(afterPath, after);
    pixelComparison = comparePixelRegions(before, after, bounds, afterBounds, scale, 'scroll');
    screenshotSha256 = { before: sha256(before), after: sha256(after) };
  }

  const record = {
    name: direction.name,
    screenPointPx: point,
    targetCssPx,
    input: { source: 'virtual-uinput-mouse', action: 'ACTION_SCROLL', hscroll: direction.h, vscroll: direction.v },
    observedPointCssPx: observedPoint,
    factorsPx: [horizontalFactor, verticalFactor],
    scalePxPerCssPx: reportedScale,
    deltaCssPx: observedDelta,
    expected: { deltaCssPx: direction.delta, root: direction.root, outer: direction.outer, inner: direction.inner },
    offsets,
    processId: pid,
    renderer: lines.find((line) => line.includes('SPINON_C0410_RENDERER=backend=')) || '',
    queueSequence: 1,
    target: 7,
    drawRevision: 2,
    helperLifecycle: helperOutput.trim().split('\n'),
    pixelComparison,
    screenshotSha256,
    surfaceBoundsBefore: bounds,
    surfaceBoundsAfter: afterBounds,
  };
  await writeRunLog(`android-${direction.name}.log`, device, serial, apkSha256,
    `input=REL_HWHEEL:${direction.h} REL_WHEEL:${direction.v} screen_px=${point.join(',')} expected_css_point=${targetCssPx.join(',')} screenshot_sha256=${JSON.stringify(screenshotSha256)}`,
    [evalReadyLine, readyLine, ...lines], pixelComparison);
  return record;
}

async function captureControlReject(serial, device, apkSha256, input) {
  const { readyLine, evalReadyLine, bounds, pid } = await launch(serial);
  const { scale, point } = inputScreenPoint(bounds);
  const before = input.screenshot ? adbBuffer(serial, 'exec-out', 'screencap', '-p') : null;
  const helperOutput = await injectWheel(serial, point, input.h, input.v, input.control);
  await wait(250);
  const lines = appLines(serial, pid);
  const rejection = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL_REJECT reason=control-wheel'));
  assert.ok(rejection, `${input.control}-${input.axis}: missing Control-wheel rejection`);
  assert.ok(rejection.includes('source=MouseWheel'));
  assert.ok(rejection.includes('modifiers=2'));
  assert.ok(rejection.includes(`hscroll=${input.h}.0`));
  assert.ok(rejection.includes(`vscroll=${input.v}.0`));
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=MouseWheel')),
    'Control-wheel rejection reached enqueue');
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel')),
    'Control-wheel rejection changed queue state');
  const draw = latestDraw(lines);
  assert.ok(draw?.includes('scroll_state_revision=1'));
  assert.ok(draw.includes('scroll_offsets=[0,0;0,0;86,106]'));

  let pixelComparison;
  let screenshotSha256 = null;
  let afterBounds;
  if (input.screenshot) {
    await writeFile(path.join(captureDirectory, `android-control-${input.control}-${input.axis}-before.png`), before);
    afterBounds = surfaceBounds(lines, 'ready');
    assert.ok(afterBounds);
    const after = adbBuffer(serial, 'exec-out', 'screencap', '-p');
    await writeFile(path.join(captureDirectory, `android-control-${input.control}-${input.axis}-after.png`), after);
    pixelComparison = comparePixelRegions(before, after, bounds, afterBounds, scale, 'reject');
    screenshotSha256 = { before: sha256(before), after: sha256(after) };
  }

  const record = {
    name: `${input.control}-${input.axis}`,
    source: 'virtual-uinput-mouse',
    hscroll: input.h,
    vscroll: input.v,
    rejection,
    queueOrProcess: false,
    revision: 1,
    processId: pid,
    offsets: [0, 0, 0, 0, 86, 106],
    helperLifecycle: helperOutput.trim().split('\n'),
    pixelComparison,
    screenshotSha256,
  };
  await writeRunLog(`android-control-${input.control}-${input.axis}.log`, device, serial, apkSha256,
    `negative=control-wheel key=${input.control} axis=${input.axis} point_css=${targetCssPx.join(',')} scale=${scale} screenshot_sha256=${JSON.stringify(screenshotSha256)}`,
    [evalReadyLine, readyLine, ...lines], pixelComparison);
  return record;
}

async function main() {
  const serialIndex = process.argv.indexOf('--android-serial');
  const serial = serialIndex >= 0 ? process.argv[serialIndex + 1] : null;
  assert.ok(serial, 'pass --android-serial emulator-N');
  await ensureEvidenceDirectory();
  const device = androidApi29Avd(serial);
  const apk = await readFile(apkPath);
  assert.ok(apk.length > 0, 'opt-in Android APK is missing or empty; build it before capture');
  const apkSha256 = sha256(apk);
  const toolchain = await findAndroidClang();
  const tempDirectory = await mkdtemp(path.join(os.tmpdir(), 'spinon-c133-api29-wheel-'));
  captureDirectory = await mkdtemp(path.join(evidenceDirectory, '.capture-'));
  const helperPath = path.join(tempDirectory, 'spinon-c133-api29-wheel');
  let successRecords = [];
  let controlRecords = [];
  let classifierOutput = '';

  try {
    const helperBytes = await compileHelper(toolchain.clang, helperPath);
    const helperBinarySha256 = sha256(helperBytes);
    classifierOutput = command('mise', ['exec', '--', 'node', '--test',
      'tools/css-reference/c13-3-native-wheel-source-classifier.test.mjs']);
    await writeFile(path.join(captureDirectory, 'classifier-test.log'), classifierOutput);
    adb(serial, 'install', '-r', apkPath);
    adb(serial, 'push', helperPath, remoteHelper);
    adb(serial, 'shell', 'chmod', '755', remoteHelper);
    for (const direction of directions) {
      successRecords.push(await captureDirection(serial, device, apkSha256, direction));
    }
    for (let index = 0; index < controlCases.length; index += 1) {
      controlRecords.push(await captureControlReject(serial, device, apkSha256, {
        ...controlCases[index],
        screenshot: index === 0,
      }));
    }

    const sourceFiles = [
      'plan/c13-3-native-wheel-input.md',
      'spec/internal/evidence/c13-3-native-wheel-api29-2026-10-17/precomparison.md',
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/WheelSourceClassifier.java',
      'tools/android-ndk-version.txt',
      'tools/css-reference/capture-c13-3-android-api29-wheel-runtime.mjs',
      'tools/css-reference/fixtures/c13_3_android_wheel_uinput.c',
    ];
    const sourceFileSha256 = Object.fromEntries(await Promise.all(sourceFiles.map(async (file) => [
      file,
      sha256(await readFile(path.join(root, file))),
    ])));
    const sourceFingerprint = sha256(Buffer.from(JSON.stringify(sourceFileSha256)));
    const renderer = successRecords[0]?.renderer || '';
    assert.ok(renderer.includes('backend=Gl') && renderer.includes('SwiftShader'),
      `expected the recorded API29 software renderer, received: ${renderer}`);
    const capturedAt = new Date().toISOString();
    const runDirectoryName = `run-${capturedAt.replace(/[-:.]/g, '')}`;
    const summary = {
      schema: 'spinon-c13-3-native-wheel-android-api29-avd-v1',
      capturedAt,
      runDirectory: runDirectoryName,
      sourceRevision: command('git', ['rev-parse', 'HEAD']).trim(),
      sourceFingerprint,
      helper: {
        source: path.relative(root, helperSource),
        sourceSha256: sha256(await readFile(helperSource)),
        binarySha256: helperBinarySha256,
        ndkVersion: toolchain.ndkVersion,
        clang: path.relative(toolchain.ndkRoot, toolchain.clang),
      },
      sourceFileSha256,
      android: { serial, ...device },
      build: {
        artifact: path.relative(root, apkPath),
        sha256: apkSha256,
        optInFlags: ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1'],
      },
      renderer,
      fixture: {
        scenario: 'auto-boundary',
        viewportCssPx: [cssWidth, cssHeight],
        targetCssPx,
        scrollportCssPx: scrollport,
        directions: successRecords,
      },
      controlWheelRejections: controlRecords,
      classifierTest: {
        command: 'mise exec -- node --test tools/css-reference/c13-3-native-wheel-source-classifier.test.mjs',
        output: classifierOutput,
      },
      limits: [
        'Android 10/API 29 Google APIs ARM64 AVD의 opt-in fixture만 검증했다.',
        'Linux uinput virtual mouse를 사용했으며 physical mouse, external trackpad, real device 입력이 아니다.',
        'AVD software renderer 결과이며 hardware GPU나 compositor 이후 scanout 근거가 아니다.',
        'iOS callback/sign/adapter, public DOM events, Android API 28 이하, C13.3 전체 완료를 증명하지 않는다.',
      ],
    };
    await writeFile(path.join(captureDirectory, 'source-revision.json'), `${JSON.stringify({
      sourceRevision: summary.sourceRevision,
      sourceFingerprint,
      helper: summary.helper,
      sourceFileSha256,
      apkSha256,
    }, null, 2)}\n`);
    await writeFile(path.join(captureDirectory, 'android-api29-wheel-matrix.json'), `${JSON.stringify(summary, null, 2)}\n`);
    const pixelScroll = successRecords.find((record) => record.name === 'diagonal-positive-positive')?.pixelComparison;
    const pixelReject = controlRecords.find((record) => record.name === 'left-horizontal')?.pixelComparison;
    await writeFile(path.join(captureDirectory, 'README.md'), [
      '# C13.3 Android API 29 native wheel AVD 검증 · 2026-10-17',
      '',
      `실행 시각 ${summary.capturedAt} · API ${device.api} / Android ${device.release} · AVD \`${device.avdName}\` · APK SHA-256 \`${apkSha256}\` · 소스 기준 \`${summary.sourceRevision}\`.`,
      '',
      '## 사전 기준과 입력 경로',
      '',
      '[사전 판정](../precomparison.md)에 고정한 400×240 CSS px fixture, 방향별 owner offset, queue 및 pixel 경계를 사용했다. API 29 ADB `input`에는 `mouse scroll`이 없어 API 29 ABI로 컴파일한 test-only uinput mouse가 Linux relative wheel axes를 보냈다. 실제 OS `ACTION_SCROLL` callback을 거쳤고 FFI direct enqueue는 사용하지 않았다.',
      '',
      `±H/V와 대각 방향 ${successRecords.length}/8 통과: 각 입력은 fresh launch, `
        + 'callback source/tool/axes/factor/scale/point/delta, queue sequence 1·target 7, revision 2, owner offsets와 presented draw를 확인했다.',
      'SurfaceView는 1050×630 px / 2.625 px per CSS px였고, 실제 GL/ANGLE/SwiftShader renderer log는 machine-readable JSON에 보존했다.',
      '',
      `좌우 Control × H/V 거부 ${controlRecords.length}/4 통과: queue/process 부재, revision 1, 초기 offsets를 보존했다. Control 좌수평 pixel positive-control 대조는 SurfaceView \`${pixelReject?.comparedPixels ?? 0}\`개에서 changed ${pixelReject?.changedPixels ?? '미측정'}개였다.`,
      '',
      `Positive-positive 대각 surface 비교: 바깥 changed ${pixelScroll?.outsideChangedPixels ?? '미측정'}개, 안쪽 changed ${pixelScroll?.insideChangedPixels ?? '미측정'}개, 비교 ${pixelScroll?.comparedPixels ?? 0}개. 입력 전후 화면은 [입력 전](./android-diagonal-positive-positive-before.png), [입력 후](./android-diagonal-positive-positive-after.png)다.`,
      '',
      '![Android API 29 AVD wheel 입력 전](./android-diagonal-positive-positive-before.png)',
      '',
      '![Android API 29 AVD 대각 wheel 입력 후](./android-diagonal-positive-positive-after.png)',
      '',
      '## 근거와 한계',
      '',
      '원본 callback/process/draw는 `android-*.log`, machine-readable 입력·offset·hash는 `android-api29-wheel-matrix.json`, classifier 회귀는 `classifier-test.log`에 있다. 실행 helper는 각 성공·거부 입력 뒤 `UI_DEV_DESTROY`를 호출했고 device 목록에서 helper 이름이 사라지는 것을 검사했다.',
      '적대적 검토: [구현·실행 검토 20개](../implementation-review-20.md) · [PR 변경 검토 30개](../pr-delta-review-30.md).',
      '',
      '이 결과는 Android 10/API 29 Google APIs ARM64 AVD와 소프트웨어 renderer의 opt-in fixture 검증이다. 실기기, hardware GPU, 외장 mouse/trackpad, API 28 이하, iOS native callback/sign/adapter, public DOM events, compositor 이후 presentation 및 C13.3 전체 지원은 미검증이다. 공식 완료 checkbox는 변경하지 않았다.',
      '',
      '재현: opt-in Android APK를 빌드한 뒤 `node tools/css-reference/capture-c13-3-android-api29-wheel-runtime.mjs --android-serial emulator-5556`을 실행한다. 전용 AVD는 `spinon_r05_api29_fallback`이며 물리 기기 serial은 스크립트가 거부한다.',
      '',
    ].join('\n'));
    const finalRunDirectory = path.join(evidenceDirectory, runDirectoryName);
    await rename(captureDirectory, finalRunDirectory);
    captureDirectory = null;
    const indexReadmePath = path.join(evidenceDirectory, 'README.md');
    const stagedIndexReadme = path.join(evidenceDirectory, `.README-${process.pid}.tmp`);
    try {
      await writeFile(stagedIndexReadme, [
        '# C13.3 Android API 29 native wheel 검증',
        '',
        `최신 완료 실행: [실행 조건·행렬·제한](./${runDirectoryName}/README.md) · [machine-readable 결과](./${runDirectoryName}/android-api29-wheel-matrix.json) · [source/APK fingerprint](./${runDirectoryName}/source-revision.json).`,
        '',
        `구현·실행 검토: [20개 실패 관점](./implementation-review-20.md) · PR 변경 검토: [30개 실패 관점](./pr-delta-review-30.md).`,
        '',
        `![API 29 AVD wheel 입력 전](./${runDirectoryName}/android-diagonal-positive-positive-before.png)`,
        '',
        `![API 29 AVD 대각 wheel 입력 후](./${runDirectoryName}/android-diagonal-positive-positive-after.png)`,
        '',
        `실행 시각 ${summary.capturedAt} · API ${device.api} / Android ${device.release} · AVD \`${device.avdName}\` · 방향 ${successRecords.length}/8 · Control 거부 ${controlRecords.length}/4 · software renderer.`,
        '',
      ].join('\n'));
      await rename(stagedIndexReadme, indexReadmePath);
    } finally {
      await rm(stagedIndexReadme, { force: true });
    }
    process.stdout.write(`${JSON.stringify({
      evidenceDirectory: path.relative(root, finalRunDirectory),
      android: summary.android,
      renderer,
      apkSha256,
      successCount: successRecords.length,
      controlRejectCount: controlRecords.length,
      pixelScroll,
      pixelReject,
    }, null, 2)}\n`);
  } finally {
    try { adb(serial, 'shell', 'rm', '-f', remoteHelper); } catch { /* AVD may have exited. */ }
    try { adb(serial, 'shell', 'am', 'force-stop', bundleId); } catch { /* AVD may have exited. */ }
    if (captureDirectory) await rm(captureDirectory, { recursive: true, force: true });
    await rm(tempDirectory, { recursive: true, force: true });
  }
}

await main();
