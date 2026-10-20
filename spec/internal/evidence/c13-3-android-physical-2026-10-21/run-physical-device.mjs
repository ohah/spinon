import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { decodeRgbaPng } from '../../../../tools/css-reference/read-png-rgba.mjs';

const root = fileURLToPath(new URL('../../../..', import.meta.url));
const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const serial = 'R5KYB06SMPE';
const packageId = 'dev.spinon.bootstrap';
const activity = `${packageId}/.MainActivity`;
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const wait = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

function run(command, args, { binary = false, timeout = 30_000 } = {}) {
  const result = spawnSync(command, args, {
    cwd: root,
    encoding: binary ? null : 'utf8',
    timeout,
    maxBuffer: 48 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} exited ${result.status}: ${String(result.stderr ?? '').trim()}`);
  }
  return result.stdout;
}

function adb(args, options) {
  return run('adb', ['-s', serial, ...args], options);
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function timestamp() {
  return new Date().toISOString();
}

function normalizeCapturedText(value) {
  const lines = value.split(/\r?\n/).map((line) => line.replace(/[\t ]+$/u, ''));
  return `${lines.join('\n').replace(/\n+$/u, '')}\n`;
}

async function saveJson(name, value) {
  await writeFile(path.join(evidenceDirectory, name), `${JSON.stringify(value, null, 2)}\n`);
}

function filteredLines(log) {
  return log.split(/\r?\n/).filter((line) => line.includes('SPINON_C133_')
    || line.includes('SPINON_C0410_DRAW')
    || line.includes('SPINON_C0410_RENDERER=backend=')
    || line.includes('SPINON_R08_WGPU=')
    || line.includes('SPINON_R08_WGPU_SURFACE=')
    || line.includes('FATAL EXCEPTION'));
}

function parseSurface(lines) {
  const line = lines.find((item) => item.includes('SPINON_C133_SURFACE_BOUNDS phase=ready '));
  assert.ok(line, 'ready SurfaceView bounds were not recorded');
  const match = line.match(/screen_px=(-?\d+),(-?\d+) size_px=(\d+)x(\d+)/);
  assert.ok(match, `malformed SurfaceView bounds: ${line}`);
  const left = Number(match[1]);
  const top = Number(match[2]);
  const width = Number(match[3]);
  const height = Number(match[4]);
  const scaleX = width / 400;
  const scaleY = height / 240;
  assert.ok(Number.isFinite(scaleX) && scaleX > 0, `invalid surface scale ${scaleX}`);
  assert.ok(Math.abs(scaleX - scaleY) <= 0.01, `nonuniform surface scale ${scaleX}/${scaleY}`);
  return { left, top, width, height, scale: scaleX, line };
}

function parseOffsets(line) {
  const match = line.match(/scroll_state_revision=(\d+) offsets=root:([-\d.]+),([-\d.]+);outer:([-\d.]+),([-\d.]+);inner:([-\d.]+),([-\d.]+)/);
  const compact = line.match(/scroll_state_revision=(\d+) scroll_offsets=\[([-\d.]+),([-\d.]+);([-\d.]+),([-\d.]+);([-\d.]+),([-\d.]+)\]/);
  const values = match ?? compact;
  assert.ok(values, `offset report is missing or malformed: ${line}`);
  return {
    revision: Number(values[1]),
    root: [Number(values[2]), Number(values[3])],
    outer: [Number(values[4]), Number(values[5])],
    inner: [Number(values[6]), Number(values[7])],
  };
}

function compareScrollportPixels(beforeBytes, afterBytes, bounds, screenSize, expectScrollChange) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.deepEqual([before.width, before.height], [screenSize.width, screenSize.height]);
  assert.deepEqual([after.width, after.height], [screenSize.width, screenSize.height]);
  const visibleSurface = {
    left: Math.max(0, bounds.left),
    top: Math.max(0, bounds.top),
    right: Math.min(screenSize.width, bounds.left + bounds.width),
    bottom: Math.min(screenSize.height, bounds.top + bounds.height),
  };
  const rootScrollport = {
    left: bounds.left + Math.round(16 * bounds.scale),
    top: bounds.top + Math.round(16 * bounds.scale),
    right: bounds.left + Math.round(384 * bounds.scale),
    bottom: bounds.top + Math.round(224 * bounds.scale),
  };
  assert.ok(rootScrollport.left >= visibleSurface.left && rootScrollport.top >= visibleSurface.top
    && rootScrollport.right <= visibleSurface.right && rootScrollport.bottom <= visibleSurface.bottom,
  `root scrollport is not fully visible: ${JSON.stringify({ visibleSurface, rootScrollport })}`);
  let changedInside = 0;
  let changedOutside = 0;
  let outsideCompared = 0;
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = -1;
  let maxY = -1;
  for (let y = visibleSurface.top; y < visibleSurface.bottom; y += 1) {
    for (let x = visibleSurface.left; x < visibleSurface.right; x += 1) {
      const first = before.getPixel(x, y);
      const second = after.getPixel(x, y);
      if (first.every((channel, index) => channel === second[index])) continue;
      if (x >= rootScrollport.left && x < rootScrollport.right
          && y >= rootScrollport.top && y < rootScrollport.bottom) {
        changedInside += 1;
      } else {
        changedOutside += 1;
      }
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
  }
  const visibleArea = (visibleSurface.right - visibleSurface.left)
    * (visibleSurface.bottom - visibleSurface.top);
  outsideCompared = visibleArea - (rootScrollport.right - rootScrollport.left)
    * (rootScrollport.bottom - rootScrollport.top);
  if (expectScrollChange) {
    assert.ok(changedInside > 0, 'positive scroll did not change pixels inside the root scrollport');
  } else {
    assert.equal(changedInside, 0, 'below-slop input changed pixels inside the root scrollport');
  }
  assert.equal(changedOutside, 0, `scroll changed ${changedOutside} visible surface pixels outside the root scrollport`);
  return {
    visibleSurface, rootScrollport, outsideCompared, changedInside, changedOutside,
    changedBounds: { left: minX, top: minY, right: maxX + 1, bottom: maxY + 1 },
  };
}

async function pollReady(processId) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const log = adb(['logcat', '-d', `--pid=${processId}`, '-v', 'brief']);
    const lines = filteredLines(log);
    const ready = lines.some((line) => line.includes('SPINON_C133_READY input=MotionEvent'));
    const evalReady = lines.some((line) => line.includes('SPINON_C133_EVAL_READY input=MotionEvent'));
    const surface = lines.some((line) => line.includes('SPINON_C133_SURFACE_BOUNDS phase=ready '));
    if (ready && evalReady && surface) return { log, lines };
    await wait(100);
  }
  const currentPid = adb(['shell', 'pidof', packageId]).trim();
  const log = currentPid ? adb(['logcat', '-d', `--pid=${currentPid}`, '-v', 'brief']) : '';
  await writeFile(path.join(evidenceDirectory, 'startup-failure-logcat.txt'), normalizeCapturedText(log));
  assert.fail('C13.3 route did not reach runtime, draw, and SurfaceView readiness');
}

async function launchScenario(scenarioId, prefix) {
  adb(['shell', 'am', 'force-stop', packageId]);
  adb([
    'shell', 'am', 'start', '-W', '-n', activity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', String(scenarioId),
  ]);
  let pid = '';
  for (let attempt = 0; attempt < 60; attempt += 1) {
    pid = adb(['shell', 'pidof', packageId]).trim().split(/\s+/)[0] ?? '';
    if (pid) break;
    await wait(100);
  }
  assert.ok(pid, `${prefix}: app process did not start`);
  const ready = await pollReady(pid);
  await wait(250);
  const bounds = parseSurface(ready.lines);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { binary: true });
  await writeFile(path.join(evidenceDirectory, `${prefix}-before.png`), screenshot);
  await writeFile(path.join(evidenceDirectory, `${prefix}-startup-logcat.txt`), normalizeCapturedText(ready.log));
  const activityDump = adb(['shell', 'dumpsys', 'activity', 'activities']);
  assert.ok(activityDump.includes(packageId), `${prefix}: app is not present in activity dump`);
  await writeFile(path.join(evidenceDirectory, `${prefix}-activity-before.txt`), normalizeCapturedText(activityDump));
  return { pid, bounds, screenshot, log: ready.log, lines: ready.lines };
}

async function captureRun(prefix, pid) {
  await wait(1500);
  const log = adb(['logcat', '-d', `--pid=${pid}`, '-v', 'brief']);
  const lines = filteredLines(log);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { binary: true });
  const activityDump = adb(['shell', 'dumpsys', 'activity', 'activities']);
  await writeFile(path.join(evidenceDirectory, `${prefix}-logcat.txt`), normalizeCapturedText(log));
  await writeFile(path.join(evidenceDirectory, `${prefix}-relevant-logcat.txt`), normalizeCapturedText(lines.join('\n')));
  await writeFile(path.join(evidenceDirectory, `${prefix}-after.png`), screenshot);
  await writeFile(path.join(evidenceDirectory, `${prefix}-activity-after.txt`), normalizeCapturedText(activityDump));
  const lastDraw = lines.findLast((line) => line.includes('SPINON_C0410_DRAW')) ?? '';
  const processLines = lines.filter((line) => line.includes('SPINON_C133_PROCESS status=0'));
  const touchLines = lines.filter((line) => line.includes('SPINON_C133_TOUCH source=MotionEvent'));
  return { log, lines, screenshot, activityDump, lastDraw, processLines, touchLines };
}

await mkdir(evidenceDirectory, { recursive: true });
const devices = run('adb', ['devices', '-l']);
assert.ok(devices.split(/\r?\n/).some((line) => line.startsWith(`${serial} `) && line.includes(' device ')),
  `physical handset ${serial} is not connected as an authorized ADB device`);

const model = adb(['shell', 'getprop', 'ro.product.model']).trim();
const release = adb(['shell', 'getprop', 'ro.build.version.release']).trim();
const sdk = adb(['shell', 'getprop', 'ro.build.version.sdk']).trim();
const size = adb(['shell', 'wm', 'size']).trim();
const density = adb(['shell', 'wm', 'density']).trim();
const screenMatch = size.match(/(\d+)x(\d+)/);
assert.ok(screenMatch, `could not parse physical display size: ${size}`);
const screenSize = { width: Number(screenMatch[1]), height: Number(screenMatch[2]) };
assert.equal(model, 'SM-S731N', `unexpected physical handset model: ${model}`);
assert.equal(release, '16', `unexpected Android release: ${release}`);
assert.equal(sdk, '36', `unexpected Android API level: ${sdk}`);
assert.deepEqual(screenSize, { width: 1080, height: 2340 }, `unexpected physical display size: ${size}`);
assert.match(density, /(?:Physical density|Override density): 450\b/,
  `unexpected display density: ${density}`);
const localApk = await readFile(apkPath);
const localApkSha256 = sha256(localApk);
const apkRemotePath = adb(['shell', 'pm', 'path', packageId]).trim().replace(/^package:/, '');
assert.ok(apkRemotePath, 'installed app APK path was not found after installation');
const installedApkSha256 = adb(['shell', 'sha256sum', apkRemotePath]).trim().split(/\s+/)[0];
assert.equal(installedApkSha256, localApkSha256, 'installed APK does not match the tested local APK');
const gpu = {
  renderer: adb(['shell', 'getprop', 'ro.hardware.egl']).trim(),
  hardware: adb(['shell', 'getprop', 'ro.hardware']).trim(),
  gpu: adb(['shell', 'getprop', 'ro.soc.model']).trim(),
};
await saveJson('environment.json', {
  capturedUtc: timestamp(), serial, model, release, sdk, size, density,
  apk: { path: apkPath, sha256: localApkSha256, installedPath: apkRemotePath, installedSha256: installedApkSha256 },
  gpu, adbDevices: devices.trim().split(/\r?\n/).filter((line) => line.startsWith(`${serial} `)),
  testMode: 'Physical handset with ADB-generated input; synthetic MotionEvent, not human raw touch.',
  sourceHead: run('git', ['rev-parse', 'HEAD']).trim(),
});
await writeFile(path.join(evidenceDirectory, 'adb-devices.txt'), normalizeCapturedText(devices));

const positive = await launchScenario(1, 'gesture-positive');
const startCss = { x: 150, y: 150 };
const endCss = { x: 150, y: 60 };
const toScreen = ({ x, y }) => ({
  x: positive.bounds.left + Math.round(x * positive.bounds.scale),
  y: positive.bounds.top + Math.round(y * positive.bounds.scale),
});
const startScreen = toScreen(startCss);
const endScreen = toScreen(endCss);
for (const [label, point] of [['start', startScreen], ['end', endScreen]]) {
  assert.ok(point.x >= 0 && point.x < screenSize.width && point.y >= 0 && point.y < screenSize.height,
    `positive ${label} point is outside the physical display: ${JSON.stringify(point)} / ${JSON.stringify(screenSize)}`);
}
const positiveStartedUtc = timestamp();
const positiveCommand = [
  'shell', 'input', 'swipe', String(startScreen.x), String(startScreen.y),
  String(endScreen.x), String(endScreen.y), '500',
];
adb(positiveCommand);
const positiveFinishedUtc = timestamp();
const positiveRun = await captureRun('gesture-positive', positive.pid);
assert.ok(positiveRun.touchLines.some((line) => line.includes('action=down')),
  'positive gesture did not reach the Android MotionEvent down callback');
const positivePanLine = positiveRun.touchLines.find((line) => line.includes('action=pan-began'));
assert.ok(positivePanLine?.includes('admission=Queued'),
  'positive gesture did not cross native slop and enqueue pan-began');
assert.ok(positivePanLine.includes('event_source_bits=0x1002')
  && positivePanLine.includes('pointer_tool_type=1')
  && positivePanLine.includes('waits_for_worker=false'),
  `pan callback source or nonblocking behavior mismatch: ${positivePanLine}`);
assert.ok(positiveRun.touchLines.some((line) => line.includes('action=up')),
  'positive gesture did not reach the Android MotionEvent up callback');
assert.ok(positiveRun.processLines.length > 0, 'positive gesture queue worker did not report processing');
assert.ok(positiveRun.lastDraw.includes('scroll_state_revision='), 'positive gesture did not publish a draw');
const initialOffsets = parseOffsets(positive.lines.find((line) => line.includes('SPINON_C133_READY input=MotionEvent')));
assert.deepEqual(initialOffsets.root, [0, 0], 'positive fixture did not start at the root boundary');
assert.deepEqual(initialOffsets.outer, [0, 0], 'positive fixture did not start at the outer boundary');
assert.deepEqual(initialOffsets.inner, [86, 106], 'positive fixture did not start at the inner scroll boundary');
const finalPositiveProcess = positiveRun.processLines.at(-1);
const positiveOffsets = parseOffsets(finalPositiveProcess);
assert.ok(positiveOffsets.revision > initialOffsets.revision,
  `positive gesture did not advance scroll revision: ${initialOffsets.revision} -> ${positiveOffsets.revision}`);
assert.deepEqual(positiveOffsets.root, initialOffsets.root,
  'positive gesture unexpectedly moved the root owner');
assert.deepEqual(positiveOffsets.inner, initialOffsets.inner,
  'positive gesture moved the inner owner past its boundary');
assert.equal(positiveOffsets.outer[0], initialOffsets.outer[0],
  'vertical gesture unexpectedly moved the outer X offset');
assert.ok(positiveOffsets.outer[1] > initialOffsets.outer[1],
  'positive gesture did not chain the remaining Y delta to the outer owner');
assert.ok(positiveRun.lastDraw.includes(`scroll_state_revision=${positiveOffsets.revision}`),
  'latest draw did not publish the final processed scroll revision');
const rendererEvidence = positiveRun.lines.filter((line) => line.includes('SPINON_C0410_RENDERER=backend='));
assert.ok(rendererEvidence.some((line) => line.includes('backend=Vulkan') && line.includes('Samsung Xclipse 940')),
  `physical Vulkan/Xclipse renderer was not confirmed: ${rendererEvidence.join('\n')}`);
const pixelBoundary = compareScrollportPixels(
  positive.screenshot, positiveRun.screenshot, positive.bounds, screenSize, true,
);
await saveJson('pixel-boundary-analysis.json', pixelBoundary);

const belowSlop = await launchScenario(0, 'below-slop');
const negativeStartCss = { x: 150, y: 150 };
const negativeStart = {
  x: belowSlop.bounds.left + Math.round(negativeStartCss.x * belowSlop.bounds.scale),
  y: belowSlop.bounds.top + Math.round(negativeStartCss.y * belowSlop.bounds.scale),
};
const negativeEnd = { x: negativeStart.x, y: negativeStart.y - 5 };
for (const [label, point] of [['start', negativeStart], ['end', negativeEnd]]) {
  assert.ok(point.x >= 0 && point.x < screenSize.width && point.y >= 0 && point.y < screenSize.height,
    `below-slop ${label} point is outside the physical display: ${JSON.stringify(point)} / ${JSON.stringify(screenSize)}`);
}
const belowSlopStartedUtc = timestamp();
const negativeCommand = [
  'shell', 'input', 'swipe', String(negativeStart.x), String(negativeStart.y),
  String(negativeEnd.x), String(negativeEnd.y), '500',
];
adb(negativeCommand);
const belowSlopFinishedUtc = timestamp();
const belowSlopRun = await captureRun('below-slop', belowSlop.pid);
const belowSlopMoveLines = belowSlopRun.touchLines.filter((line) => line.includes('action=below-slop'));
assert.ok(belowSlopMoveLines.length > 0, 'small movement did not reach the below-slop callback');
assert.ok(belowSlopMoveLines.every((line) => line.includes('accepted=false')),
  'one or more below-slop MotionEvents were accepted');
assert.ok(!belowSlopRun.touchLines.some((line) => line.includes('action=pan-began')),
  'below-slop input unexpectedly began a pan');
assert.ok(belowSlopRun.processLines.length > 0, 'below-slop gesture terminal was not processed');
const belowSlopReady = belowSlop.lines.find((line) => line.includes('SPINON_C133_READY input=MotionEvent'));
const belowSlopInitial = parseOffsets(belowSlopReady);
assert.deepEqual(belowSlopInitial, { revision: 0, root: [0, 0], outer: [0, 0], inner: [0, 0] },
  'below-slop fixture did not start from its documented zero-offset baseline');
const evalReady = belowSlop.lines.find((line) => line.includes('SPINON_C133_EVAL_READY input=MotionEvent'));
const slopMatch = evalReady?.match(/native_slop_px=(\d+)/);
assert.ok(slopMatch, 'Android ViewConfiguration touch slop was not recorded');
const nativeTouchSlopPx = Number(slopMatch[1]);
assert.ok(5 < nativeTouchSlopPx, `negative control is not below native touch slop: 5/${nativeTouchSlopPx}`);
const belowSlopFinalProcess = belowSlopRun.processLines.at(-1);
const belowSlopFinal = belowSlopFinalProcess ? parseOffsets(belowSlopFinalProcess) : belowSlopInitial;
assert.deepEqual(belowSlopFinal, belowSlopInitial, 'below-slop input changed scroll offset or revision');
const belowSlopPixelBoundary = compareScrollportPixels(
  belowSlop.screenshot, belowSlopRun.screenshot, belowSlop.bounds, screenSize, false,
);
await saveJson('below-slop-pixel-boundary-analysis.json', belowSlopPixelBoundary);
const appForegroundAfterRun = /topResumedActivity=ActivityRecord\{[^}]*dev\.spinon\.bootstrap/.test(belowSlopRun.activityDump);
assert.ok(appForegroundAfterRun, 'C13.3 test app did not remain foreground after the physical run');
const failureMarkers = [...positiveRun.lines, ...belowSlopRun.lines]
  .filter((line) => line.includes('FATAL EXCEPTION')
    || /SPINON_C133_.*(failed=true|_ERROR|REJECTED)/.test(line)
    || /SPINON_C133_PROCESS status=(?!0\b)/.test(line));
assert.deepEqual(failureMarkers, [], `C13.3 failure markers were recorded: ${failureMarkers.join('\n')}`);

const summary = {
  schema: 'spinon-c133-android-physical-gesture-check-v1',
  capturedUtc: timestamp(),
  sourceHead: run('git', ['rev-parse', 'HEAD']).trim(),
  serial, model, release, sdk, displaySize: screenSize,
  nativeTouchSlopPx,
  apkSha256: localApkSha256,
  mode: 'ADB shell input swipe on physical handset; Android MotionEvent synthetic input, not raw touchscreen contact.',
  rendererEvidence,
  pixelBoundary,
  positive: {
    scenario: 1,
    bounds: positive.bounds,
    surfaceClippedByDisplay: positive.bounds.left < 0 || positive.bounds.top < 0
      || positive.bounds.left + positive.bounds.width > screenSize.width
      || positive.bounds.top + positive.bounds.height > screenSize.height,
    offscreenPhysicalPixels: {
      left: Math.max(0, -positive.bounds.left),
      top: Math.max(0, -positive.bounds.top),
      right: Math.max(0, positive.bounds.left + positive.bounds.width - screenSize.width),
      bottom: Math.max(0, positive.bounds.top + positive.bounds.height - screenSize.height),
    },
    startCss, endCss, startScreen, endScreen,
    startedUtc: positiveStartedUtc,
    finishedUtc: positiveFinishedUtc,
    inputCommand: ['adb', '-s', serial, ...positiveCommand],
    initialOffsets, finalOffsets: positiveOffsets,
    processedBatchCount: positiveRun.processLines.length,
    downSeen: true, panBeganSeen: true, upSeen: true,
    drawPublishedFinalRevision: positiveRun.lastDraw.includes(`scroll_state_revision=${positiveOffsets.revision}`),
    screenshotBeforeSha256: sha256(positive.screenshot),
    screenshotAfterSha256: sha256(positiveRun.screenshot),
  },
  belowSlop: {
    scenario: 0,
    bounds: belowSlop.bounds,
    startScreen: negativeStart,
    endScreen: negativeEnd,
    physicalPixelDistance: 5,
    nativeTouchSlopPx,
    startedUtc: belowSlopStartedUtc,
    finishedUtc: belowSlopFinishedUtc,
    inputCommand: ['adb', '-s', serial, ...negativeCommand],
    initialOffsets: belowSlopInitial,
    finalOffsets: belowSlopFinal,
    belowSlopRejected: true,
    pixelBoundary: belowSlopPixelBoundary,
    screenshotBeforeSha256: sha256(belowSlop.screenshot),
    screenshotAfterSha256: sha256(belowSlopRun.screenshot),
  },
  appForegroundAfterRun,
  failureMarkers,
};
await saveJson('summary.json', summary);

const files = (await readdir(evidenceDirectory, { withFileTypes: true }))
  .filter((entry) => entry.isFile() && entry.name !== 'SHA256SUMS.txt')
  .map((entry) => entry.name)
  .sort();
const checksums = [];
for (const name of files) {
  checksums.push(`${sha256(await readFile(path.join(evidenceDirectory, name)))}  ${name}`);
}
await writeFile(path.join(evidenceDirectory, 'SHA256SUMS.txt'), `${checksums.join('\n')}\n`);
console.log(JSON.stringify(summary, null, 2));
