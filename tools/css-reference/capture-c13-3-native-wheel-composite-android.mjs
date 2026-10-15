import assert from 'node:assert/strict';
import { spawn, execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  realpath,
  lstat,
  rm,
  writeFile,
} from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const defaultEvidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-native-wheel-composite-avd-2026-10-16',
);
const evidenceRoot = path.join(root, 'spec/internal/evidence');
const evidenceDirectory = path.resolve(
  process.env.SPINON_C133_COMPOSITE_EVIDENCE_DIR || defaultEvidenceDirectory,
);
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const helperSource = path.join(
  root,
  'tools/css-reference/fixtures/c13_3_android_composite_wheel_uinput.c',
);
const remoteHelper = '/data/local/tmp/spinon-c13-3-composite-wheel-uinput';
const deviceName = 'Spinon C13.3 composite mouse touchpad probe';
const bundleId = 'dev.spinon.bootstrap';
const activity = 'dev.spinon.bootstrap/.MainActivity';
const surfaceScale = 2.625;
const viewportCssPx = [400, 240];
const targetCssPx = [132, 112];
const scrollport = { left: 16, top: 16, right: 384, bottom: 224 };
const scenarios = [
  { id: 'mouse-event-on-composite-device', relativeX: 135, relativeY: 479, expectAdmitted: true },
  { id: 'mouse-event-outside-surface-on-composite-device', relativeX: 362, relativeY: 1285, expectAdmitted: false },
];

function command(program, args, { encoding = 'utf8', timeout = 30_000, maxBuffer = 64 * 1024 * 1024 } = {}) {
  try {
    return execFileSync(program, args, {
      cwd: root,
      encoding,
      timeout,
      maxBuffer,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
  } catch (error) {
    const stderr = Buffer.isBuffer(error.stderr) ? error.stderr.toString('utf8') : error.stderr;
    const stdout = Buffer.isBuffer(error.stdout) ? error.stdout.toString('utf8') : error.stdout;
    throw new Error(`${program} ${args.join(' ')} failed: ${stderr || stdout || error.message}`);
  }
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function trimLineEndWhitespace(text) {
  return text.split('\n').map((line) => line.replace(/[\t ]+$/u, '')).join('\n');
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

async function ensureEvidenceDirectory() {
  const repository = await realpath(root);
  const realEvidenceRoot = await realpath(evidenceRoot);
  assert.ok(realEvidenceRoot.startsWith(`${repository}${path.sep}`),
    'spec/internal/evidence resolves outside the repository root');
  const relativeDirectory = path.relative(evidenceRoot, evidenceDirectory);
  assert.ok(relativeDirectory && relativeDirectory !== '..'
    && !relativeDirectory.startsWith(`..${path.sep}`),
  'SPINON_C133_COMPOSITE_EVIDENCE_DIR must be a child of spec/internal/evidence');

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
      `evidence output contains a non-directory or symlink: ${currentDirectory}`);
    const resolvedDirectory = await realpath(currentDirectory);
    assert.ok(resolvedDirectory.startsWith(`${realEvidenceRoot}${path.sep}`),
      'evidence output resolves outside spec/internal/evidence');
  }
}

function androidSimulator(serial) {
  assert.match(serial, /^emulator-\d+$/, `refusing non-emulator Android target ${serial}`);
  assert.equal(command('adb', ['-s', serial, 'get-state']).trim(), 'device');
  const device = {
    serial,
    model: adb(serial, 'shell', 'getprop', 'ro.product.model').trim(),
    qemu: adb(serial, 'shell', 'getprop', 'ro.kernel.qemu').trim(),
    api: adb(serial, 'shell', 'getprop', 'ro.build.version.sdk').trim(),
    release: adb(serial, 'shell', 'getprop', 'ro.build.version.release').trim(),
    size: adb(serial, 'shell', 'wm', 'size').trim(),
    density: adb(serial, 'shell', 'wm', 'density').trim(),
  };
  assert.equal(device.qemu, '1');
  assert.equal(device.api, '37');
  assert.match(device.model, /sdk_gphone16k_arm64/);
  assert.match(device.size, /1080x2400/);
  assert.match(device.density, /420/);
  return device;
}

async function androidToolchain() {
  const version = (await readFile(path.join(root, 'tools/android-ndk-version.txt'), 'utf8')).trim();
  const candidates = [];
  for (const configured of [process.env.ANDROID_NDK_HOME, process.env.ANDROID_NDK_ROOT]) {
    if (configured && path.basename(configured) === version) candidates.push(configured);
  }
  const sdk = process.env.ANDROID_SDK_ROOT || process.env.ANDROID_HOME
    || path.join(os.homedir(), 'Library/Android/sdk');
  candidates.push(path.join(sdk, 'ndk', version));

  for (const ndkRoot of candidates) {
    const prebuiltRoot = path.join(ndkRoot, 'toolchains/llvm/prebuilt');
    let hosts;
    try {
      hosts = await readdir(prebuiltRoot);
    } catch {
      continue;
    }
    for (const host of hosts) {
      const bin = path.join(prebuiltRoot, host, 'bin');
      const clang = path.join(bin, 'aarch64-linux-android21-clang');
      const readelf = path.join(bin, 'llvm-readelf');
      try {
        await lstat(clang);
        await lstat(readelf);
        return { ndkRoot, clang, readelf, host };
      } catch {
        // Try the next pinned host toolchain directory.
      }
    }
  }
  throw new Error(`Android NDK ${version} clang/readelf were not found`);
}

async function compileHelper(toolchain, temporaryDirectory) {
  const binary = path.join(temporaryDirectory, 'composite-wheel-uinput');
  command(toolchain.clang, [
    '-Wall', '-Wextra', '-Werror', '-O2', '-Wl,-z,max-page-size=16384',
    helperSource, '-o', binary,
  ]);
  const headers = command(toolchain.readelf, ['-lW', binary]);
  const loadAlignments = headers.split('\n')
    .filter((line) => /^\s*LOAD\s/.test(line))
    .map((line) => Number.parseInt(line.trim().split(/\s+/).at(-1), 16));
  assert.ok(loadAlignments.length > 0, 'uinput helper ELF has no LOAD segments');
  assert.ok(loadAlignments.every((alignment) => alignment === 16_384),
    `uinput helper LOAD alignment is not 16 KB: ${loadAlignments.join(',')}`);
  await writeFile(path.join(evidenceDirectory, 'helper-program-headers.txt'), trimLineEndWhitespace(headers));
  return {
    binary,
    sha256: sha256(await readFile(binary)),
    loadAlignments,
    programHeaders: headers,
    programHeadersSha256: sha256(Buffer.from(headers)),
  };
}

function screenBounds(lines, phase) {
  const line = lines.findLast((item) => item.includes(`SPINON_C133_SURFACE_BOUNDS phase=${phase} `));
  if (!line) return null;
  const match = line.match(/screen_px=(\d+),(\d+) size_px=(\d+)x(\d+)/);
  assert.ok(match, `malformed ${phase} SurfaceView bounds: ${line}`);
  const left = Number(match[1]);
  const top = Number(match[2]);
  return { left, top, right: left + Number(match[3]), bottom: top + Number(match[4]), line };
}

function relevantAppLines(serial) {
  return adb(serial, 'logcat', '-d', '-v', 'brief').split('\n')
    .filter((line) => line.includes('SpinonBootstrap(')
      && (line.includes('SPINON_C133_') || line.includes('SPINON_C0410_DRAW')));
}

async function launchFixture(serial) {
  adb(serial, 'shell', 'am', 'force-stop', bundleId);
  adb(serial, 'logcat', '-c');
  const launch = adb(serial, 'shell', 'am', 'start', '-W', '-n', activity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', '1');
  assert.match(launch, /Status: ok/);
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const lines = relevantAppLines(serial);
    const evalReady = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    const ready = lines.find((line) => line.includes('SPINON_C133_READY'));
    const bounds = screenBounds(lines, 'ready');
    if (evalReady && ready && bounds) {
      // READY can precede the first stable SurfaceView composite. Match the
      // established Android wheel capture path: allow its post-layout frame
      // to settle, then bind the baseline screenshot to the refreshed bounds.
      await wait(500);
      const settledLines = relevantAppLines(serial);
      const settledEvalReady = settledLines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
      const settledReady = settledLines.find((line) => line.includes('SPINON_C133_READY'));
      const settledBounds = screenBounds(settledLines, 'ready');
      const draw = settledLines.findLast((line) => line.includes('SPINON_C0410_DRAW'));
      assert.ok(settledEvalReady && settledReady && settledBounds,
        'gesture fixture readiness disappeared while waiting for a stable frame');
      assert.ok(draw?.includes('scroll_state_revision=1')
        && draw.includes('scroll_offsets=[0,0;0,0;86,106]'), draw);
      return { lines: settledLines, evalReady: settledEvalReady, ready: settledReady, bounds: settledBounds };
    }
    await wait(100);
  }
  assert.fail(`${serial} did not produce a ready gesture fixture`);
}

function deviceSection(text, needle, startPattern, nextPattern) {
  const lines = text.split('\n');
  const index = lines.findIndex((line) => line.includes(needle));
  assert.ok(index >= 0, `input diagnostics did not list ${needle}`);
  let start = index;
  while (start > 0 && !startPattern.test(lines[start])) start -= 1;
  let end = index + 1;
  while (end < lines.length && !nextPattern.test(lines[end])) end += 1;
  return lines.slice(start, end).join('\n').trimEnd();
}

async function runCompositeInput(serial, helperArgs, evidenceStem) {
  const child = spawn('adb', ['-s', serial, 'shell', remoteHelper, ...helperArgs], {
    cwd: root,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let stdout = '';
  let stderr = '';
  let markerResolve;
  let markerReject;
  const created = new Promise((resolve, reject) => {
    markerResolve = resolve;
    markerReject = reject;
  });
  const finished = new Promise((resolve) => {
    child.on('close', (code, signal) => resolve({ code, signal }));
  });
  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString('utf8');
    if (stdout.includes('device-created ')) markerResolve();
  });
  child.stderr.on('data', (chunk) => { stderr += chunk.toString('utf8'); });
  child.on('error', markerReject);
  const markerTimeout = setTimeout(() => markerReject(
    new Error('uinput helper did not report device-created'),
  ), 10_000);
  try {
    await Promise.race([
      created,
      finished.then(({ code, signal }) => {
        throw new Error(`uinput helper exited before device creation: code=${code} signal=${signal}`);
      }),
    ]);
  } catch (error) {
    child.kill('SIGTERM');
    await finished;
    throw error;
  } finally {
    clearTimeout(markerTimeout);
  }

  const inputDump = adb(serial, 'shell', 'dumpsys', 'input');
  assert.ok(inputDump.includes(deviceName), 'InputReader did not enumerate the virtual device');
  const eventHubBlock = deviceSection(
    inputDump,
    deviceName,
    /^\s*\d+: /,
    /^\s*\d+: /,
  );
  assert.match(eventHubBlock, /Classes:.*CURSOR.*TOUCHPAD/,
    'InputReader did not classify the same virtual device as cursor and touchpad');
  const geteventOutput = command('adb', ['-s', serial, 'shell', 'getevent', '-pl']);
  const geteventBlock = deviceSection(
    geteventOutput,
    `name:     "${deviceName}"`,
    /^add device \d+:/,
    /^add device \d+:/,
  );
  await writeFile(path.join(evidenceDirectory, `${evidenceStem}-input-device.log`), trimLineEndWhitespace(eventHubBlock));
  await writeFile(path.join(evidenceDirectory, `${evidenceStem}-getevent-device.log`), trimLineEndWhitespace(geteventBlock));

  const timeout = setTimeout(() => child.kill('SIGTERM'), 20_000);
  const result = await finished;
  clearTimeout(timeout);
  await writeFile(path.join(evidenceDirectory, `${evidenceStem}-uinput.log`), `${stdout}${stderr}`);
  assert.equal(result.code, 0, `uinput helper failed: ${stdout}${stderr}`);
  assert.match(stdout, /device-destroyed=true/, 'uinput helper did not destroy its device');
  return { stdout: stdout.trim(), stderr: stderr.trim(), inputDevice: eventHubBlock, geteventDevice: geteventBlock };
}

function compareSurface(beforeBytes, afterBytes, beforeBounds, afterBounds, expectChangedInside) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.deepEqual([before.width, before.height], [after.width, after.height]);
  const width = beforeBounds.right - beforeBounds.left;
  const height = beforeBounds.bottom - beforeBounds.top;
  assert.deepEqual([width, height], [1050, 630]);
  assert.deepEqual(
    [afterBounds.right - afterBounds.left, afterBounds.bottom - afterBounds.top],
    [width, height],
    'SurfaceView size changed between screenshots',
  );
  const border = {
    left: Math.round(scrollport.left * surfaceScale),
    top: Math.round(scrollport.top * surfaceScale),
    right: Math.round(scrollport.right * surfaceScale),
    bottom: Math.round(scrollport.bottom * surfaceScale),
  };
  let outsideCompared = 0;
  let outsideChanged = 0;
  let insideChanged = 0;
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const outside = x < border.left || x >= border.right || y < border.top || y >= border.bottom;
      const first = before.getPixel(beforeBounds.left + x, beforeBounds.top + y);
      const second = after.getPixel(afterBounds.left + x, afterBounds.top + y);
      if (outside) outsideCompared += 1;
      if (first.some((channel, index) => channel !== second[index])) {
        if (outside) outsideChanged += 1;
        else insideChanged += 1;
      }
    }
  }
  assert.ok(outsideCompared > 100_000, `too few outside pixels were checked: ${outsideCompared}`);
  assert.equal(outsideChanged, 0, `wheel changed ${outsideChanged} pixels outside the root scrollport`);
  if (expectChangedInside) assert.ok(insideChanged > 0, 'admitted wheel did not change pixels inside the root scrollport');
  else assert.equal(insideChanged, 0, `rejected wheel changed ${insideChanged} pixels inside the root scrollport`);
  return {
    outsideCompared,
    outsideChanged,
    insideChanged,
    surfaceCropPx: [width, height],
    rootScrollportCssPx: scrollport,
    rootScrollportPx: border,
    alignedBounds: { before: beforeBounds, after: afterBounds },
  };
}

function validateCompositeLines(lines, expectedAdmitted) {
  const diagnosticNeedle = 'device_supported_source_bits=';
  const eventLine = lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=MouseWheel'))
    || lines.find((line) => line.includes('SPINON_C133_NATIVE_WHEEL_REJECT'));
  assert.ok(eventLine, 'composite uinput input did not reach the native wheel callback');
  assert.ok(eventLine.includes('event_source_bits=0x2002'), eventLine);
  assert.ok(eventLine.includes('pointer_count=1 pointer_tool=mouse pointer_tool_type=3'), eventLine);
  assert.ok(eventLine.includes(diagnosticNeedle), eventLine);
  const unionMatch = eventLine.match(/device_supported_source_bits=0x([0-9a-f]+)/);
  assert.ok(unionMatch, eventLine);
  const supportedSourceUnion = Number.parseInt(unionMatch[1], 16);
  assert.equal(supportedSourceUnion & 0x2002, 0x2002, `mouse is absent from source union: ${eventLine}`);
  assert.equal(supportedSourceUnion & 0x100008, 0x100008,
    `touchpad is absent from source union: ${eventLine}`);

  if (!expectedAdmitted) {
    assert.ok(eventLine.includes('reason=point-outside-surface'), eventLine);
    assert.ok(!lines.some((line) => line.includes('SPINON_C133_NATIVE_WHEEL source=')),
      'out-of-surface composite wheel reached queue admission');
    assert.ok(!lines.some((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel')),
      'out-of-surface composite wheel reached queue processing');
    const draw = lines.findLast((line) => line.includes('SPINON_C0410_DRAW'));
    assert.ok(draw?.includes('scroll_state_revision=1')
      && draw.includes('scroll_offsets=[0,0;0,0;86,106]'), draw);
    return { eventLine, sourceUnionHex: unionMatch[1], draw };
  }

  assert.ok(eventLine.includes('hscroll=1.0 vscroll=0.0'), eventLine);
  assert.ok(eventLine.includes('source=MouseWheel'), eventLine);
  assert.ok(eventLine.includes('sequence=1 admitted=true admission=Queued target=7'), eventLine);
  assert.ok(eventLine.includes('delta_css_px=64.0,-0.0'), eventLine);
  const pointMatch = eventLine.match(/point_css_px=([\d.]+),([\d.]+)/);
  assert.ok(pointMatch, eventLine);
  const pointCss = [Number(pointMatch[1]), Number(pointMatch[2])];
  assert.ok(Math.abs(pointCss[0] - targetCssPx[0]) <= 0.5, `unexpected target x ${pointCss[0]}`);
  assert.ok(Math.abs(pointCss[1] - targetCssPx[1]) <= 0.75, `unexpected target y ${pointCss[1]}`);
  const process = lines.find((line) => line.includes('SPINON_C133_PROCESS') && line.includes('event=wheel'));
  assert.ok(process?.includes('scroll_state_revision=2')
    && process.includes('offsets=root:0,0;outer:64,0;inner:86,106'), process);
  const draw = lines.findLast((line) => line.includes('SPINON_C0410_DRAW'));
  assert.ok(draw?.includes('scroll_state_revision=2')
    && draw.includes('scroll_offsets=[0,0;64,0;86,106]'), draw);
  return { eventLine, pointCss, sourceUnionHex: unionMatch[1], process, draw };
}

async function captureScenario(serial, device, scenario, helperSha256) {
  const fixture = await launchFixture(serial);
  const stem = scenario.expectAdmitted ? 'composite-admitted' : 'composite-outside-surface';
  const before = adbBuffer(serial, 'exec-out', 'screencap', '-p');
  await writeFile(path.join(evidenceDirectory, `${stem}-before.png`), before);
  const helper = await runCompositeInput(serial, [
    String(scenario.relativeX),
    String(scenario.relativeY),
    'h',
    '4',
  ], stem);
  await wait(500);
  const lines = relevantAppLines(serial);
  const callbackPath = path.join(evidenceDirectory, `${stem}-callback.log`);
  const earlyBounds = screenBounds(lines, 'wheel') || fixture.bounds;
  await writeFile(callbackPath, [
    `platform=Android API ${device.api} ARM64 AVD (${device.model}, Android ${device.release})`,
    `serial=${device.serial} display=${device.size} density=${device.density}`,
    `helper_sha256=${helperSha256}`,
    `relative_cursor_delta=${scenario.relativeX},${scenario.relativeY}`,
    `fixture_eval_ready=${fixture.evalReady}`,
    `fixture_ready=${fixture.ready}`,
    `surface_ready=${fixture.bounds.line}`,
    ...lines,
    `surface_after=${earlyBounds.line || fixture.bounds.line}`,
    'diagnostic_phase=post-input-before-assertions',
    '',
  ].join('\n'));
  const validation = validateCompositeLines(lines, scenario.expectAdmitted);
  const after = adbBuffer(serial, 'exec-out', 'screencap', '-p');
  await writeFile(path.join(evidenceDirectory, `${stem}-after.png`), after);
  const afterBounds = screenBounds(lines, 'wheel') || fixture.bounds;
  let pixelComparison;
  let pixelComparisonError;
  try {
    pixelComparison = compareSurface(before, after, fixture.bounds, afterBounds,
      scenario.expectAdmitted);
  } catch (error) {
    pixelComparisonError = error.message;
  }
  const postInputDump = adb(serial, 'shell', 'dumpsys', 'input');
  const deviceRemoved = !postInputDump.includes(deviceName);
  await writeFile(callbackPath, [
    `platform=Android API ${device.api} ARM64 AVD (${device.model}, Android ${device.release})`,
    `serial=${device.serial} display=${device.size} density=${device.density}`,
    `helper_sha256=${helperSha256}`,
    `relative_cursor_delta=${scenario.relativeX},${scenario.relativeY}`,
    `fixture_eval_ready=${fixture.evalReady}`,
    `fixture_ready=${fixture.ready}`,
    `surface_ready=${fixture.bounds.line}`,
    ...lines,
    `surface_after=${screenBounds(lines, 'wheel')?.line || fixture.bounds.line}`,
    `post_input_device_removed=${deviceRemoved}`,
    pixelComparison
      ? `pixel_comparison=${JSON.stringify(pixelComparison)}`
      : `pixel_comparison_error=${pixelComparisonError}`,
    '',
  ].join('\n'));
  assert.ok(deviceRemoved, 'virtual composite device remained registered after helper exit');
  if (pixelComparisonError) throw new Error(pixelComparisonError);
  return {
    id: scenario.id,
    helper,
    validation,
    pixelComparison,
    beforeSha256: sha256(before),
    afterSha256: sha256(after),
    deviceRemoved: !postInputDump.includes(deviceName),
  };
}

async function main() {
  const serialIndex = process.argv.indexOf('--android-serial');
  const serial = serialIndex >= 0 ? process.argv[serialIndex + 1] : null;
  assert.ok(serial, 'pass --android-serial emulator-N');
  await ensureEvidenceDirectory();
  const device = androidSimulator(serial);
  const toolchain = await androidToolchain();
  const temporaryDirectory = await mkdtemp(path.join(os.tmpdir(), 'spinon-c133-composite-wheel-'));
  let installed = false;
  let successful = false;
  try {
    const helper = await compileHelper(toolchain, temporaryDirectory);
    const apk = await readFile(apkPath);
    assert.ok(apk.length > 0, `empty Android APK at ${apkPath}`);
    command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 120_000 });
    installed = true;
    adb(serial, 'push', helper.binary, remoteHelper);
    adb(serial, 'shell', 'chmod', '755', remoteHelper);
    const cases = [];
    for (const scenario of scenarios) {
      cases.push(await captureScenario(serial, device, scenario, helper.sha256));
    }
    const sourceFiles = [
      'plan/c13-3-native-wheel-input.md',
      'spec/STATUS.md',
      'spec/internal/0063-c13-3-scroll-gestures.md',
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/WheelSourceClassifier.java',
      'tools/css-reference/capture-c13-3-native-wheel-composite-android.mjs',
      'tools/css-reference/fixtures/c13_3_android_composite_wheel_uinput.c',
    ];
    const sourceFileSha256 = Object.fromEntries(await Promise.all(sourceFiles.map(async (file) => [
      file,
      sha256(await readFile(path.join(root, file))),
    ])));
    const summary = {
      schema: 'spinon-c13-3-native-wheel-composite-avd-v1',
      capturedAt: new Date().toISOString(),
      sourceRevision: command('git', ['rev-parse', 'HEAD']).trim(),
      android: device,
      build: {
        apkSha256: sha256(apk),
        helperSourceSha256: sha256(await readFile(helperSource)),
        helperSha256: helper.sha256,
        helperLoadAlignments: helper.loadAlignments,
        helperProgramHeadersSha256: helper.programHeadersSha256,
        helperProgramHeaders: helper.programHeaders,
        ndk: path.basename(toolchain.ndkRoot),
        ndkHost: toolchain.host,
        optInFlags: ['SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1'],
        sourceFileSha256,
      },
      fixture: {
        scenario: 'auto-boundary',
        viewportCssPx,
        targetCssPx,
        surfacePxPerCssPx: surfaceScale,
        syntheticDeviceCapabilities: ['mouse cursor', 'touchpad', 'multitouch axes'],
      },
      cases,
      limits: [
        'The composite input device is virtual uinput on an Android API 37 ARM64 AVD, not physical hardware.',
        'This proves event-source classification and the opt-in Android fixture route only.',
        'Targetable physical trackpad, iOS native callback/sign/adapter, public DOM events, and compositor scanout remain unverified.',
        'C13.3 remains incomplete.',
      ],
    };
    await writeFile(path.join(evidenceDirectory, 'composite-wheel-avd.json'),
      `${JSON.stringify(summary, null, 2)}\n`);
    const readme = [
      '# C13.3 Android AVD 복합 wheel source 분류 · 2026-10-16',
      '',
      `캡처 시각: ${summary.capturedAt} · APK SHA-256: \`${summary.build.apkSha256}\` · uinput helper SHA-256: \`${helper.sha256}\`.`,
      '',
      `Android API ${device.api} ARM64 AVD \`${serial}\`에서 하나의 test-only uinput 장치가 Android InputReader에 \`TOUCHPAD\`와 \`CURSOR\` 클래스로 등록되는 것을 확인했다. 이 장치는 \`SOURCE_TOUCHPAD (0x100008)\`와 \`SOURCE_MOUSE (0x2002)\`를 함께 지원한다.`,
      '',
      `positive control에서 실제 \`ACTION_SCROLL\` event는 \`event_source_bits=0x2002\`, pointer tool \`mouse\`, 지원 source 합집합 \`0x10200a\`로 도착했고 \`MouseWheel\`로 분류됐다. CSS point \`${cases[0].validation.pointCss.join(',')}\`, target 7, normalized \`(64,0) CSS px\`, queue sequence 1과 draw revision 2까지 확인했다. 따라서 device source union의 touchpad bit가 mouse event를 TrackpadScroll로 오분류하지 않는다.`,
      '',
      `같은 장치의 화면 밖 positive-looking scroll은 \`MouseWheel\`로 분류된 뒤 SurfaceView 밖 좌표에서 거부됐다. queue/process는 0건이고 revision/offset과 화면이 유지됐다. helper 종료 뒤 두 case 모두 InputReader에서 가상 장치가 사라졌다.`,
      '',
      `positive control surface 비교는 root scrollport 바깥 ${cases[0].pixelComparison.outsideCompared.toLocaleString()} pixels에서 changed ${cases[0].pixelComparison.outsideChanged}개, 안쪽 changed ${cases[0].pixelComparison.insideChanged}개였다. 거부 control은 바깥/안쪽 changed 0개다. SurfaceView crop은 캡처 간 위치 변화에 맞춰 정렬했다.`,
      '',
      '![복합 wheel positive control 전](./composite-admitted-before.png)',
      '',
      '![복합 wheel positive control 후](./composite-admitted-after.png)',
      '',
      '![화면 밖 복합 wheel 입력 거부 전](./composite-outside-surface-before.png)',
      '',
      '![화면 밖 복합 wheel 입력 거부 후](./composite-outside-surface-after.png)',
      '',
      'InputReader/getevent 장치 진단, 원본 callback·draw log, pixel manifest와 모든 source hash는 이 디렉터리에 저장했다.',
      '사람이 읽는 `.log`와 program-header text 파일은 줄 끝 공백만 정리한다. JSON에는 InputReader/getevent 원문 문자열과 compiler program-header 원문·SHA-256을 보존한다.',
      '',
      '재현:',
      '',
      '```sh',
      'SPINON_V8_DIR=/path/to/pinned/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 mise exec -- bun run build:android',
      'bun tools/css-reference/capture-c13-3-native-wheel-composite-android.mjs --android-serial emulator-5554',
      '```',
      '',
      '실기기와 terminal-browser는 사용하지 않았다. 이 결과는 가상 AVD input-path 검증이며 composite peripheral 하드웨어, iOS discrete/continuous callback, 공개 WheelEvent/scroll API, S04.10 displayed-frame target이나 C13.3 전체 완료를 증명하지 않는다.',
      '',
    ].join('\n');
    await writeFile(path.join(evidenceDirectory, 'README.md'), readme);
    successful = true;
    process.stdout.write(`${JSON.stringify({
      evidenceDirectory: path.relative(root, evidenceDirectory),
      android: device,
      apkSha256: summary.build.apkSha256,
      helperSha256: helper.sha256,
      cases: cases.map((item) => ({
        id: item.id,
        sourceUnion: item.validation.sourceUnionHex,
        admitted: item.id === 'mouse-event-on-composite-device',
        outsideChanged: item.pixelComparison.outsideChanged,
        insideChanged: item.pixelComparison.insideChanged,
        deviceRemoved: item.deviceRemoved,
      })),
    }, null, 2)}\n`);
  } finally {
    if (installed) {
      try {
        adb(serial, 'shell', 'rm', '-f', remoteHelper);
      } catch (error) {
        if (successful) throw error;
        process.stderr.write(`${error.message}\n`);
      }
    }
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
}

try {
  await main();
} catch (error) {
  process.stderr.write(`${error.stack || error.message}\n`);
  process.exitCode = 1;
}
