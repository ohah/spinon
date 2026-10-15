import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { lstat, mkdir, readFile, realpath, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const defaultEvidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-native-wheel-input-2026-10-16',
);
const evidenceRoot = path.resolve(root, 'spec/internal/evidence');
const evidenceDirectory = path.resolve(
  root,
  process.env.SPINON_C133_ANDROID_EVIDENCE_DIR || defaultEvidenceDirectory,
);
assert.ok(evidenceDirectory.startsWith(`${evidenceRoot}${path.sep}`),
  'SPINON_C133_ANDROID_EVIDENCE_DIR must stay under spec/internal/evidence');
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const bundleID = 'dev.spinon.bootstrap';
const activity = 'dev.spinon.bootstrap/.MainActivity';
const cssWidth = 400;
const cssHeight = 240;
const surfaceScale = 2.625;
const scrollport = { left: 16, top: 16, right: 384, bottom: 224 };
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

function processLines(serial) {
  return command('adb', ['-s', serial, 'logcat', '-d', '-v', 'brief'])
    .split('\n')
    .filter((line) => line.includes('SpinonBootstrap(')
      && (line.includes('SPINON_C133_') || line.includes('SPINON_C0410_DRAW')));
}

async function ready(serial) {
  for (let attempt = 0; attempt < 80; attempt += 1) {
    const lines = processLines(serial);
    const readyLine = lines.find((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    const readyBounds = surfaceBoundsFromLines(lines, 'ready');
    if (readyLine && evalReadyLine && readyBounds) return { lines, readyLine, evalReadyLine, readyBounds };
    await wait(100);
  }
  assert.fail(`${serial}: no initial Android surface/runtime readiness report`);
}

async function launch(serial) {
  command('adb', ['-s', serial, 'shell', 'am', 'force-stop', bundleID]);
  command('adb', ['-s', serial, 'logcat', '-c']);
  command('adb', [
    '-s', serial, 'shell', 'am', 'start', '-W', '-n', activity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', '1',
  ]);
  const result = await ready(serial);
  await wait(500);
  return result;
}

function offsetsFromProcess(line) {
  const match = line.match(/scroll_state_revision=(\d+) offsets=root:([-\d.]+),([-\d.]+);outer:([-\d.]+),([-\d.]+);inner:([-\d.]+),([-\d.]+)/);
  assert.ok(match, `process log did not include final owner offsets: ${line}`);
  return {
    revision: Number(match[1]),
    root: [Number(match[2]), Number(match[3])],
    outer: [Number(match[4]), Number(match[5])],
    inner: [Number(match[6]), Number(match[7])],
  };
}

function latestDraw(lines) {
  return lines.filter((line) => line.includes('SPINON_C0410_DRAW')).at(-1);
}

function surfaceBoundsFromLines(lines, phase) {
  const line = lines.findLast((item) => item.includes(`SPINON_C133_SURFACE_BOUNDS phase=${phase} `));
  if (!line) return null;
  const match = line.match(/screen_px=(\d+),(\d+) size_px=(\d+)x(\d+)/);
  assert.ok(match, `SurfaceView bounds log is malformed: ${line}`);
  const left = Number(match[1]);
  const top = Number(match[2]);
  return { left, top, right: left + Number(match[3]), bottom: top + Number(match[4]) };
}

function pixelBoundaryComparison(beforeBytes, afterBytes, beforeBounds, afterBounds, scale) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.equal(before.width, after.width);
  assert.equal(before.height, after.height);
  const surfaceWidthPx = beforeBounds.right - beforeBounds.left;
  const surfaceHeightPx = beforeBounds.bottom - beforeBounds.top;
  assert.deepEqual([surfaceWidthPx, surfaceHeightPx], [Math.round(cssWidth * scale), Math.round(cssHeight * scale)]);
  assert.deepEqual([afterBounds.right - afterBounds.left, afterBounds.bottom - afterBounds.top],
    [surfaceWidthPx, surfaceHeightPx], 'SurfaceView size changed between screenshots');
  const left = Math.round(scrollport.left * scale);
  const right = Math.round(scrollport.right * scale);
  const top = Math.round(scrollport.top * scale);
  const bottom = Math.round(scrollport.bottom * scale);
  let outsideCompared = 0;
  let outsideChanged = 0;
  let insideChanged = 0;
  for (let localY = 0; localY < surfaceHeightPx; localY += 1) {
    for (let localX = 0; localX < surfaceWidthPx; localX += 1) {
      const beforeX = beforeBounds.left + localX;
      const beforeY = beforeBounds.top + localY;
      const afterX = afterBounds.left + localX;
      const afterY = afterBounds.top + localY;
      assert.ok(beforeX >= 0 && beforeX < before.width && beforeY >= 0 && beforeY < before.height,
        `before SurfaceView pixel ${beforeX},${beforeY} outside screenshot`);
      assert.ok(afterX >= 0 && afterX < after.width && afterY >= 0 && afterY < after.height,
        `after SurfaceView pixel ${afterX},${afterY} outside screenshot`);
      const isOutsideScrollport = localX < left || localX >= right || localY < top || localY >= bottom;
      const first = before.getPixel(beforeX, beforeY);
      const second = after.getPixel(afterX, afterY);
      if (first.some((channel, index) => channel !== second[index])) {
        if (isOutsideScrollport) outsideChanged += 1;
        else insideChanged += 1;
      }
      if (isOutsideScrollport) outsideCompared += 1;
    }
  }
  assert.ok(outsideCompared > 100_000, `too few pixels were checked outside scrollport: ${outsideCompared}`);
  assert.equal(outsideChanged, 0, `scroll changed ${outsideChanged} pixels outside the outer scrollport border`);
  assert.ok(insideChanged > 0, 'successful wheel input did not change any pixel inside the scrollport');
  return {
    outsideCompared,
    outsideChanged,
    insideChanged,
    alignedSurfaceBounds: { before: beforeBounds, after: afterBounds },
    rootBorderCssPx: scrollport,
  };
}

function validateSuccessfulInput(lines, direction, targetCssPx) {
  const wheelLine = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=MouseWheel'));
  const processLine = lines.find((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'));
  assert.ok(wheelLine, `${direction.name}: native ACTION_SCROLL callback was not logged`);
  assert.ok(processLine, `${direction.name}: wheel queue result was not processed`);
  assert.ok(wheelLine.includes('source_bits=0x2002 event_source_bits=0x2002'),
    `${direction.name}: event source bits were not logged: ${wheelLine}`);
  assert.ok(wheelLine.includes('pointer_tool=mouse'),
    `${direction.name}: pointer tool was not classified as mouse: ${wheelLine}`);
  assert.match(wheelLine, /device_supported_source_bits=0x[0-9a-f]+/,
    `${direction.name}: device-supported source union was not diagnostic-only: ${wheelLine}`);
  assert.ok(wheelLine.includes(`hscroll=${direction.h}.0`), `${direction.name}: horizontal axis mismatch: ${wheelLine}`);
  assert.ok(wheelLine.includes(`vscroll=${direction.v}.0`), `${direction.name}: vertical axis mismatch: ${wheelLine}`);
  assert.ok(wheelLine.includes('hfactor_px=168.0 vfactor_px=168.0'));
  assert.ok(wheelLine.includes('scale_px_per_css_px=2.625'));
  assert.ok(wheelLine.includes(`delta_css_px=${direction.delta[0]}.0,${direction.delta[1] === 0 ? '-0.0' : `${direction.delta[1]}.0`}`),
    `${direction.name}: normalized delta mismatch: ${wheelLine}`);
  assert.ok(wheelLine.includes('unit=AndroidPhysicalPixels phase=None modifiers=0'));
  assert.ok(wheelLine.includes('sequence=1 admitted=true admission=Queued target=7'));
  const pointMatch = wheelLine.match(/point_css_px=([-\d.]+),([-\d.]+) delta_css_px=/);
  assert.ok(pointMatch, `${direction.name}: native point was not logged: ${wheelLine}`);
  const pointCssPx = [Number(pointMatch[1]), Number(pointMatch[2])];
  const pointToleranceCssPx = 0.5 / surfaceScale + 0.001;
  for (let axis = 0; axis < 2; axis += 1) {
    assert.ok(Math.abs(pointCssPx[axis] - targetCssPx[axis]) <= pointToleranceCssPx,
      `${direction.name}: normalized native point exceeds one-pixel rounding tolerance: ${pointCssPx}`);
  }
  const offsets = offsetsFromProcess(processLine);
  assert.equal(offsets.revision, 2, `${direction.name}: scroll revision did not advance exactly once`);
  assert.deepEqual(offsets.root, direction.root, `${direction.name}: root offset mismatch`);
  assert.deepEqual(offsets.outer, direction.outer, `${direction.name}: outer offset mismatch`);
  assert.deepEqual(offsets.inner, direction.inner, `${direction.name}: inner offset mismatch`);
  const drawLine = latestDraw(lines);
  assert.ok(drawLine?.includes('scroll_state_revision=2'), `${direction.name}: draw did not publish revision 2`);
  assert.ok(drawLine.includes(`scroll_offsets=[${direction.root.join(',')};${direction.outer.join(',')};${direction.inner.join(',')}]`),
    `${direction.name}: presented draw offsets mismatch: ${drawLine}`);
  return { wheelLine, processLine, drawLine, offsets, pointCssPx };
}

async function captureDirection(serial, device, direction) {
  const { readyLine, evalReadyLine, readyBounds } = await launch(serial);
  const targetCssPx = [132, 112];
  const screenPx = [
    readyBounds.left + Math.round(targetCssPx[0] * surfaceScale),
    readyBounds.top + Math.round(targetCssPx[1] * surfaceScale),
  ];
  const beforePath = path.join(evidenceDirectory, `android-${direction.name}-before.png`);
  const afterPath = path.join(evidenceDirectory, `android-${direction.name}-after.png`);
  let beforeBounds;
  let before;
  if (direction.screenshot) {
    beforeBounds = readyBounds;
    before = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
  }
  command('adb', [
    '-s', serial, 'shell', 'input', 'mouse', 'scroll', String(screenPx[0]), String(screenPx[1]),
    '--axis', `HSCROLL,${direction.h}`,
    '--axis', `VSCROLL,${direction.v}`,
  ]);
  let lines = [];
  for (let attempt = 0; attempt < 40; attempt += 1) {
    lines = processLines(serial);
    if (lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'))
        && latestDraw(lines)?.includes('scroll_state_revision=2')
        && surfaceBoundsFromLines(lines, 'wheel')) break;
    await wait(100);
  }
  const result = validateSuccessfulInput(lines, direction, targetCssPx);
  const originMatch = result.wheelLine.match(/surface_origin_screen_px=(\d+),(\d+)/);
  assert.ok(originMatch, 'wheel callback did not report SurfaceView screen origin');
  const origin = { x: Number(originMatch[1]), y: Number(originMatch[2]) };
  assert.deepEqual([origin.x, origin.y], [readyBounds.left, readyBounds.top],
    `${direction.name}: SurfaceView moved between ready point calculation and native input`);
  let pixelComparison;
  let afterBounds;
  if (direction.screenshot) {
    await writeFile(beforePath, before);
    await wait(250);
    lines = processLines(serial);
    afterBounds = surfaceBoundsFromLines(lines, 'wheel');
    assert.ok(afterBounds, 'SurfaceView bounds after wheel status/layout were not logged');
    const after = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
    await writeFile(afterPath, after);
    pixelComparison = pixelBoundaryComparison(before, after, beforeBounds, afterBounds, surfaceScale);
  }
  const record = {
    name: direction.name,
    input: { source: 'SOURCE_MOUSE', action: 'ACTION_SCROLL', hscroll: direction.h, vscroll: direction.v, screenPx, targetCssPx },
    expected: { deltaCssPx: direction.delta, offsets: { root: direction.root, outer: direction.outer, inner: direction.inner }, revision: 2 },
    observations: result,
    pixelComparison,
    surfaceOriginAtWheelEventPx: origin,
    surfaceBoundsBefore: beforeBounds,
    surfaceBoundsAfter: afterBounds,
    screenshots: direction.screenshot ? [path.basename(beforePath), path.basename(afterPath)] : [],
  };
  await writeFile(path.join(evidenceDirectory, `android-${direction.name}.log`), [
    `platform=Android API ${device.api} emulator ${device.model} (${device.release})`,
    `serial=${serial} qemu=${device.qemu} display=${device.size} density=${device.density}`,
    `apk_sha256=${sha256(await readFile(apkPath))}`,
    `surface_origin_at_wheel_event_px=${origin.x},${origin.y} expected_scale_px_per_css_px=${surfaceScale}`,
    `input=mouse ACTION_SCROLL HSCROLL:${direction.h} VSCROLL:${direction.v} screen_px=${screenPx.join(',')} expected_point_css_px=${targetCssPx.join(',')}`,
    `expected_delta_css_px=${direction.delta.join(',')} expected_offsets=root:${direction.root.join(',')};outer:${direction.outer.join(',')};inner:${direction.inner.join(',')}`,
    evalReadyLine,
    '',
    readyLine,
    ...lines,
    '',
    `pixel_comparison=${JSON.stringify(pixelComparison)}`,
  ].join('\n'));
  return record;
}

async function captureNegative(serial, device, definition) {
  const { readyLine, evalReadyLine, readyBounds } = await launch(serial);
  const inputCommand = typeof definition.command === 'function'
    ? definition.command(readyBounds)
    : definition.command;
  command('adb', ['-s', serial, 'shell', ...inputCommand]);
  await wait(350);
  const lines = processLines(serial);
  const rejection = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL_REJECT'));
  assert.ok(rejection, `${definition.name}: native wheel rejection was not logged`);
  assert.ok(rejection.includes(`reason=${definition.reason}`), `${definition.name}: wrong rejection: ${rejection}`);
  for (const diagnostic of definition.expectedDiagnostics || []) {
    assert.ok(rejection.includes(diagnostic),
      `${definition.name}: rejection omitted ${diagnostic}: ${rejection}`);
  }
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=')),
    `${definition.name}: rejected input reached native enqueue`);
  assert.ok(!lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel')),
    `${definition.name}: rejected input changed queue state`);
  const drawLine = latestDraw(lines);
  assert.ok(drawLine?.includes('scroll_state_revision=1'), `${definition.name}: no-op changed draw revision: ${drawLine}`);
  assert.ok(drawLine.includes('scroll_offsets=[0,0;0,0;86,106]'),
    `${definition.name}: no-op changed scroll offsets: ${drawLine}`);
  const record = {
    name: definition.name,
    inputCommand: inputCommand.join(' '),
    expectedRejection: definition.reason,
    observedRejection: rejection,
    drawLine,
  };
  await writeFile(path.join(evidenceDirectory, `android-${definition.name}.log`), [
    `platform=Android API ${device.api} emulator ${device.model} (${device.release})`,
    `serial=${serial} qemu=${device.qemu} display=${device.size} density=${device.density}`,
    `apk_sha256=${sha256(await readFile(apkPath))}`,
    `negative_control=${definition.name} expected_reason=${definition.reason}`,
    `input=adb shell ${inputCommand.join(' ')}`,
    evalReadyLine,
    '',
    readyLine,
    ...lines,
    '',
  ].join('\n'));
  return record;
}

function androidSimulator(serial) {
  assert.match(serial, /^emulator-\d+$/, `refusing non-emulator Android target ${serial}`);
  assert.equal(command('adb', ['-s', serial, 'get-state']).trim(), 'device');
  const properties = {
    model: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.product.model']).trim(),
    qemu: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.kernel.qemu']).trim(),
    api: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.build.version.sdk']).trim(),
    release: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.build.version.release']).trim(),
    size: command('adb', ['-s', serial, 'shell', 'wm', 'size']).trim(),
    density: command('adb', ['-s', serial, 'shell', 'wm', 'density']).trim(),
  };
  assert.equal(properties.qemu, '1');
  assert.equal(properties.api, '37');
  return properties;
}

async function ensureEvidenceDirectory() {
  const realRepositoryRoot = await realpath(root);
  const realEvidenceRoot = await realpath(evidenceRoot);
  assert.ok(realEvidenceRoot.startsWith(`${realRepositoryRoot}${path.sep}`),
    'spec/internal/evidence resolves outside the repository root');
  const relativeDirectory = path.relative(evidenceRoot, evidenceDirectory);
  assert.ok(relativeDirectory && relativeDirectory !== '..'
    && !relativeDirectory.startsWith(`..${path.sep}`),
  'SPINON_C133_ANDROID_EVIDENCE_DIR must be a child of spec/internal/evidence');

  let currentDirectory = evidenceRoot;
  for (const segment of relativeDirectory.split(path.sep)) {
    currentDirectory = path.join(currentDirectory, segment);
    try {
      await lstat(currentDirectory);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      await mkdir(currentDirectory);
    }

    const stat = await lstat(currentDirectory);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(),
      `evidence output path contains a non-directory or symlink: ${currentDirectory}`);
    const resolvedDirectory = await realpath(currentDirectory);
    assert.ok(resolvedDirectory.startsWith(`${realEvidenceRoot}${path.sep}`),
      'SPINON_C133_ANDROID_EVIDENCE_DIR resolves outside spec/internal/evidence');
  }
}

async function main() {
  const serialIndex = process.argv.indexOf('--android-serial');
  const serial = serialIndex >= 0 ? process.argv[serialIndex + 1] : null;
  assert.ok(serial, 'pass --android-serial emulator-N');
  await ensureEvidenceDirectory();
  const device = androidSimulator(serial);
  const classifierCommand = ['node', '--test', 'tools/css-reference/c13-3-native-wheel-source-classifier.test.mjs'];
  const classifierTestOutput = command(classifierCommand[0], classifierCommand.slice(1));
  await writeFile(path.join(evidenceDirectory, 'classifier-test.log'), classifierTestOutput);
  command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 120_000 });
  const successRecords = [];
  for (const direction of directions) successRecords.push(await captureDirection(serial, device, direction));
  const negativeDefinitions = [
    {
      name: 'outside-surface',
      reason: 'point-outside-surface',
      expectedDiagnostics: ['event_source_bits=0x2002', 'pointer_tool=mouse'],
      command: (bounds) => [
        'input', 'mouse', 'scroll', String(bounds.left + 345), String(bounds.top - 1),
        '--axis', 'HSCROLL,1', '--axis', 'VSCROLL,0',
      ],
    },
    {
      name: 'touchpad-missing-position',
      reason: 'point-outside-surface',
      expectedDiagnostics: ['event_source_bits=0x100008', 'pointer_tool=finger'],
      command: ['input', 'touchpad', 'scroll', '--axis', 'HSCROLL,1', '--axis', 'VSCROLL,0'],
    },
    {
      name: 'zero-axes',
      reason: 'zero-or-nonfinite-axis',
      expectedDiagnostics: ['event_source_bits=0x2002', 'pointer_tool=mouse'],
      command: ['input', 'mouse', 'scroll', '360', '1300', '--axis', 'HSCROLL,0', '--axis', 'VSCROLL,0'],
    },
  ];
  const negativeRecords = [];
  for (const definition of negativeDefinitions) {
    negativeRecords.push(await captureNegative(serial, device, definition));
  }
  const apk = await readFile(apkPath);
  const sourceFiles = [
    'plan/c13-3-native-wheel-input.md',
    'spec/STATUS.md',
    'spec/internal/0063-c13-3-scroll-gestures.md',
    'spec/internal/README.md',
    'spec/public/roadmap.html',
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/WheelSourceClassifier.java',
    'tools/css-reference/capture-c13-3-native-wheel-input-android.mjs',
    'tools/css-reference/c13-3-native-wheel-source-classifier.test.mjs',
    'tools/css-reference/fixtures/WheelSourceClassifierJvmTest.java',
  ];
  const sourceFileSha256 = Object.fromEntries(await Promise.all(sourceFiles.map(async (file) => [
    file,
    sha256(await readFile(path.join(root, file))),
  ])));
  const summary = {
    schema: 'spinon-c13-3-native-wheel-android-avd-v2',
    capturedAt: new Date().toISOString(),
    sourceRevision: command('git', ['rev-parse', 'HEAD']).trim(),
    sourceDiffSha256: sha256(command('git', [
      'diff', '--binary', 'HEAD', '--', ...sourceFiles,
    ], { encoding: 'buffer' })),
    android: { serial, ...device },
    build: {
      artifact: path.relative(root, apkPath),
      sha256: sha256(apk),
      optInFlags: ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1'],
    },
    sourceFileSha256,
    fixture: {
      scenario: 'auto-boundary',
      viewportCssPx: [cssWidth, cssHeight],
      surfacePxPerCssPx: surfaceScale,
      scrollportCssPx: scrollport,
      targetCssPx: [132, 112],
      screenPoint: 'surface screen origin + rounded CSS point * surface scale',
    },
    successes: successRecords,
    negativeControls: negativeRecords,
    limits: [
      'Android API 37 ARM64 AVD만 확인했으며 실기기·hardware GPU 근거가 아니다.',
      'ADB touchpad source에는 pointer 좌표가 없어 거부됐다. target 가능한 실제 touchpad 입력은 미검증이다.',
      'iOS native callback/sign은 관찰되지 않아 별도 gate가 남아 있다.',
      'ADB virtual keyboard Control meta state가 mouse event에 전달되지 않았다. 코드 guard는 있으나 Control-wheel runtime pass로 표시하지 않는다.',
    ],
  };
  await writeFile(path.join(evidenceDirectory, 'source-revision.json'), `${JSON.stringify({
    sourceRevision: summary.sourceRevision,
    sourceDiffSha256: summary.sourceDiffSha256,
    sourceFileSha256: summary.sourceFileSha256,
  }, null, 2)}\n`);
  await writeFile(path.join(evidenceDirectory, 'android-avd-matrix.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(path.join(evidenceDirectory, 'README.md'), [
    '# C13.3 Android native wheel AVD 실행 근거 · 2026-10-16',
    '',
    `캡처 시각: ${summary.capturedAt} · APK SHA-256: \`${summary.build.sha256}\` · 소스 기준: \`${summary.sourceRevision}\`.`,
    '',
    `API ${device.api} ARM64 AVD \`${serial}\`에서 mouse \`ACTION_SCROLL\`의 H/V 네 방향과 네 대각 조합을 실제 앱 callback → bounded queue → render draw까지 실행했다. 각 방향은 별도 앱 실행으로 fixture 시작 오프셋을 고정했다.`,
    '',
    `ViewConfiguration factor ${surfaceScale * 64} px, surface scale ${surfaceScale} px/CSS px에서 각 native axis 1은 64 CSS px다. 성공 8/8은 입력 axis, normalized delta, sequence 1 admission, target 7, revision 2, owner offsets와 presented draw를 검사했다.`,
    '',
    `거부 경로 ${negativeRecords.length}/${negativeRecords.length}: SurfaceView 밖 좌표, 좌표가 없는 ADB touchpad sample, 영 축 입력은 각각 기대 reason으로 거부됐고 revision 1 / 시작 offsets \`root:0,0;outer:0,0;inner:86,106\`를 유지했다.`,
    '',
    'Android 분류는 event source와 pointer tool type을 사용하며 `InputDevice.getSources()`는 진단용으로만 기록한다. JVM classifier harness는 mouse+finger, mouse+mouse, unknown tool/source 및 복합 장치의 legacy source-union negative control을 검사한다. AVD 입력별 `event_source_bits`, `pointer_tool`, `device_supported_source_bits`는 각 로그에 보존한다.',
    '',
    `스크린샷 \`android-diagonal-positive-positive-before.png\`와 \`android-diagonal-positive-positive-after.png\`의 SurfaceView 안쪽 ${successRecords.find((record) => record.name === 'diagonal-positive-positive').pixelComparison.outsideCompared.toLocaleString()}개 픽셀 중 scrollport border 바깥은 변경 ${successRecords.find((record) => record.name === 'diagonal-positive-positive').pixelComparison.outsideChanged}개, 안쪽 변경은 ${successRecords.find((record) => record.name === 'diagonal-positive-positive').pixelComparison.insideChanged.toLocaleString()}개였다. 비교 범위는 고정 fixture의 root scrollport \`left=16, top=16, right=384, bottom=224 CSS px\` 바깥 영역이다.`,
    '',
    '![Android API 37 AVD wheel 입력 전 화면](./android-diagonal-positive-positive-before.png)',
    '',
    '![Android API 37 AVD 대각 wheel 입력 후 화면](./android-diagonal-positive-positive-after.png)',
    '',
    '원본 callback, queue, draw 로그는 각 `android-*.log`, 전체 입력·offset 표는 `android-avd-matrix.json`에 있다.',
    '',
    '기본/opt-in APK의 `BuildConfig` 값, DEX/native SHA-256, Rust FFI symbol 경계는 [Android artifact 기록](android-artifact-boundary.md)에 있다.',
    '',
    'PR 본문의 Android 화면 표시 확인은 [Agent Browser 검증 기록](pr-body-image-verification.json)에 있다.',
    '',
    '이 근거는 Android API 37 AVD의 opt-in fixture 경로만 확인한다. AVD에 mouse와 touchpad를 함께 지원하는 복합 입력 장치는 없어 classifier negative control은 JVM에서 수행했다. 물리 장치, hardware GPU, targetable trackpad, Control-wheel runtime, iOS, public DOM events, compositor 이후 scanout 또는 C13.3 전체 완료를 증명하지 않는다.',
    '',
    '재현: `bun tools/css-reference/capture-c13-3-native-wheel-input-android.mjs --android-serial emulator-5554` (먼저 opt-in Android debug APK를 빌드한다).',
    '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({
    evidenceDirectory: path.relative(root, evidenceDirectory),
    android: summary.android,
    apkSha256: summary.build.sha256,
    successCount: successRecords.length,
    negativeControlCount: negativeRecords.length,
    classifierTest: 'tools/css-reference/c13-3-native-wheel-source-classifier.test.mjs',
    pixelComparison: successRecords.find((record) => record.name === 'diagonal-positive-positive').pixelComparison,
  }, null, 2)}\n`);
}

await main();
