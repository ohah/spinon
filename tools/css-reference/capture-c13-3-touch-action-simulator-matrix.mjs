import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, readdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const evidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-touch-action-simulator-matrix-2026-10-15',
);
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const simulatorAppPath = path.join(
  root,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app',
);
const bundleID = 'dev.spinon.bootstrap';
const androidActivity = 'dev.spinon.bootstrap/.MainActivity';
const androidCssScale = 2.625;
const cases = [
  { id: 3, name: 'touch-action-none', effective: 'none', allowedAxis: null },
  { id: 4, name: 'touch-action-pan-x', effective: 'pan-x', allowedAxis: 'x' },
  { id: 5, name: 'touch-action-pan-y', effective: 'pan-y', allowedAxis: 'y' },
  { id: 6, name: 'touch-action-target-pan-x', effective: 'pan-x', allowedAxis: 'x' },
  { id: 7, name: 'touch-action-intersection-none', effective: 'none', allowedAxis: null },
];
const directions = [
  { name: 'horizontal-left', axis: 'x', start: [25, 48], end: [13, 48], deltaCss: [-50, 0] },
  { name: 'vertical-up', axis: 'y', start: [25, 48], end: [25, 42], deltaCss: [0, -50] },
];

function command(commandName, args, { timeout = 30_000, encoding = 'utf8' } = {}) {
  return execFileSync(commandName, args, {
    cwd: root,
    encoding,
    stdio: ['ignore', 'pipe', 'pipe'],
    timeout,
    maxBuffer: 32 * 1024 * 1024,
  });
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function treeSha256(directory) {
  const entries = [];
  async function visit(current) {
    for (const entry of (await readdir(current, { withFileTypes: true })).sort((a, b) => a.name.localeCompare(b.name))) {
      const absolute = path.join(current, entry.name);
      if (entry.isDirectory()) await visit(absolute);
      else if (entry.isFile()) {
        const relative = path.relative(directory, absolute).split(path.sep).join('/');
        entries.push(`${relative}\0${sha256(await readFile(absolute))}`);
      }
    }
  }
  await visit(directory);
  return sha256(entries.join('\n'));
}

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function lastEvalReadyLine(lines, pid) {
  return lines.findLast((line) => line.includes(`[${pid}:`) && line.includes('SPINON_C133_EVAL_READY'));
}

function parseOffsets(line) {
  const match = line.match(/scroll_state_revision=(\d+) offsets=root:([^;]+);outer:([^;]+);inner:([^\s]+)/);
  assert.ok(match, `missing offsets and revision in final runtime line: ${line}`);
  const pair = (value) => value.split(',').map(Number);
  return {
    revision: Number(match[1]),
    root: pair(match[2]),
    outer: pair(match[3]),
    inner: pair(match[4]),
  };
}

function validateResult({ record, lines, readyLine, evalReadyLine, caseDefinition, direction }) {
  assert.ok(readyLine, `${record.name} ${direction.name}: no presented-surface ready report`);
  assert.ok(evalReadyLine, `${record.name} ${direction.name}: no runtime evaluation ready report`);
  assert.ok(readyLine.includes('status=0 presented boxes=11'), `${record.name}: first draw was not ready`);
  assert.ok(
    evalReadyLine.includes(`scenario=${caseDefinition.name}`),
    `${record.name} ${direction.name}: scenario mismatch: ${evalReadyLine}`,
  );
  assert.ok(
    evalReadyLine.includes(`touch_action_effective=${caseDefinition.effective}`),
    `${record.name} ${direction.name}: effective touch-action mismatch: ${evalReadyLine}`,
  );
  if (caseDefinition.id === 6) {
    assert.ok(evalReadyLine.includes('touch_action_inner=auto touch_action_inner_content=pan-x'));
  }
  if (caseDefinition.id === 7) {
    assert.ok(evalReadyLine.includes('touch_action_inner=pan-y touch_action_inner_content=pan-x'));
  }

  const processLines = lines.filter((line) =>
    line.includes('SPINON_C133_PROCESS') || line.includes('SPINON_C133_INPUT_BATCH'));
  assert.ok(processLines.length > 0, `${record.name} ${direction.name}: no processed input log`);
  const terminalLine = processLines.findLast((line) => line.includes('kind=End') || line.includes('kind=end'))
    ?? processLines.at(-1);
  assert.ok(terminalLine.includes('result=ended'), `${record.name} ${direction.name}: no terminal End result`);
  const allowed = caseDefinition.allowedAxis === direction.axis;
  const offsets = parseOffsets(terminalLine);
  const rejected = processLines.some((line) => line.includes('result=pan_rejected'));
  const movedTarget = processLines.some((line) => line.includes('result=moved target=7'));
  assert.deepEqual(offsets.root, [0, 0], `${record.name} ${direction.name}: root moved`);
  assert.deepEqual(offsets.outer, [0, 0], `${record.name} ${direction.name}: outer moved`);
  if (allowed) {
    assert.ok(movedTarget, `${record.name} ${direction.name}: allowed pan did not move target 7`);
    assert.ok(offsets.revision > 0, `${record.name} ${direction.name}: allowed pan did not advance revision`);
    if (direction.axis === 'x') {
      assert.ok(offsets.inner[0] > 0.5, `${record.name}: allowed x offset did not advance`);
      assert.ok(Math.abs(offsets.inner[1]) < 0.5, `${record.name}: x pan changed y offset`);
    } else {
      assert.ok(offsets.inner[1] > 0.5, `${record.name}: allowed y offset did not advance`);
      assert.ok(Math.abs(offsets.inner[0]) < 0.5, `${record.name}: y pan changed x offset`);
    }
  } else {
    assert.ok(rejected, `${record.name} ${direction.name}: disallowed pan was not rejected`);
    assert.equal(offsets.revision, 0, `${record.name} ${direction.name}: rejected pan advanced revision`);
    assert.deepEqual(offsets.inner, [0, 0], `${record.name} ${direction.name}: rejected pan changed offsets`);
  }

  return { allowed, rejected, movedTarget, finalLine: terminalLine, offsets };
}

function iosSimulator(udid) {
  const deviceMap = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']));
  for (const [runtime, devices] of Object.entries(deviceMap.devices)) {
    const device = devices.find((item) => item.udid === udid);
    if (device) {
      assert.equal(device.state, 'Booted', `iOS simulator ${udid} must be booted`);
      assert.equal(device.isAvailable, true, `iOS simulator ${udid} must be available`);
      assert.match(runtime, /iOS/, `target is not an iOS simulator: ${runtime}`);
      assert.match(
        device.deviceTypeIdentifier ?? '',
        /iPhone/,
        `expected an iPhone simulator, got ${device.deviceTypeIdentifier ?? 'unknown device type'}`,
      );
      return { ...device, runtime };
    }
  }
  assert.fail(`${udid} is not an available simulator UDID from simctl`);
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
  assert.equal(properties.qemu, '1', `Android target is not an emulator: ${serial}`);
  assert.equal(properties.api, '37', `expected API 37 emulator, got ${properties.api}`);
  return properties;
}

function processLinesForIOS(udid, pid) {
  const raw = command('xcrun', [
    'simctl', 'spawn', udid, 'log', 'show', '--style', 'compact', '--last', '3m',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS "SPINON_C133"',
  ]);
  return raw.split('\n').filter((line) => line.includes(`SpinonBootstrap[${pid}:`));
}

function processLinesForAndroid(serial) {
  const raw = command('adb', ['-s', serial, 'logcat', '-d', '-v', 'brief']);
  return raw.split('\n').filter((line) =>
    line.includes('SPINON_C133_') && line.includes('SpinonBootstrap('));
}

async function waitUntilReadyForIOS(udid, pid) {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const lines = processLinesForIOS(udid, pid);
    const readyLine = lines.find((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    if (readyLine && evalReadyLine) return { readyLine, evalReadyLine };
    await delay(250);
  }
  assert.fail(`iOS process ${pid} never reported SPINON_C133_READY`);
}

async function waitUntilReadyForAndroid(serial) {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const lines = processLinesForAndroid(serial);
    const readyLine = lines.find((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.find((line) => line.includes('SPINON_C133_EVAL_READY'));
    if (readyLine && evalReadyLine) return { readyLine, evalReadyLine };
    await delay(250);
  }
  assert.fail(`${serial} never reported SPINON_C133_READY`);
}

function androidSurfaceBounds(serial) {
  const remotePath = '/sdcard/spinon-c13-3-touch-action-window.xml';
  command('adb', ['-s', serial, 'shell', 'uiautomator', 'dump', remotePath], { timeout: 20_000 });
  const xml = command('adb', ['-s', serial, 'shell', 'cat', remotePath]);
  const match = xml.match(/class="android\.view\.SurfaceView"[^>]*bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"/);
  assert.ok(match, 'Android fixture SurfaceView bounds were not found');
  return { left: Number(match[1]), top: Number(match[2]), right: Number(match[3]), bottom: Number(match[4]) };
}

function androidSwipeCoordinates(serial, direction, readyLine) {
  const surface = readyLine.match(/surface_texture=(\d+)x(\d+)/);
  assert.ok(surface, `missing Android surface size: ${readyLine}`);
  const scaleX = Number(surface[1]) / 400;
  const scaleY = Number(surface[2]) / 240;
  assert.ok(Math.abs(scaleX - scaleY) <= 0.01, `non-uniform Android fixture scale ${scaleX}/${scaleY}`);
  assert.ok(Math.abs(scaleX - androidCssScale) <= 0.01, `unexpected Android CSS scale ${scaleX}`);
  const bounds = androidSurfaceBounds(serial);
  const cssStart = [100, 100];
  const cssEnd = direction.axis === 'x' ? [50, 100] : [100, 50];
  return {
    bounds,
    scale: scaleX,
    startX: Math.round(bounds.left + cssStart[0] * scaleX),
    startY: Math.round(bounds.top + cssStart[1] * scaleY),
    endX: Math.round(bounds.left + cssEnd[0] * scaleX),
    endY: Math.round(bounds.top + cssEnd[1] * scaleY),
  };
}

function iosFlowPath(direction) {
  const from = direction.start.map((value) => `${value}%`).join(', ');
  const to = direction.end.map((value) => `${value}%`).join(', ');
  const filename = path.join(evidenceDirectory, `ios-${direction.name}.yaml`);
  return writeFile(filename, [
    `appId: ${bundleID}`,
    '---',
    '- swipe:',
    `    start: ${from}`,
    `    end: ${to}`,
    '    duration: 500',
    '',
  ].join('\n')).then(() => filename);
}

function launchIOS(udid, scenarioID) {
  const output = command('xcrun', [
    'simctl', 'launch', '--terminate-running-process', udid, bundleID,
    '--spinon-c133-scroll-gesture', `--spinon-c133-policy-scenario-id=${scenarioID}`,
  ]);
  const pid = Number(output.match(/:\s*(\d+)\s*$/)?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, `could not parse iOS app PID: ${output}`);
  return pid;
}

function launchAndroid(serial, scenarioID) {
  command('adb', ['-s', serial, 'shell', 'am', 'force-stop', bundleID]);
  command('adb', ['-s', serial, 'logcat', '-c']);
  command('adb', [
    '-s', serial, 'shell', 'am', 'start', '-W', '-n', androidActivity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', String(scenarioID),
  ]);
}

async function captureIOS(udid) {
  const simulator = iosSimulator(udid);
  const records = [];
  for (const caseDefinition of cases) {
    for (const direction of directions) {
      const pid = launchIOS(udid, caseDefinition.id);
      const { readyLine, evalReadyLine } = await waitUntilReadyForIOS(udid, pid);
      const screenshotStem = `ios-${caseDefinition.name}-${direction.name}`;
      const captureScreens = (caseDefinition.id === 4 && direction.axis === 'x')
        || (caseDefinition.id === 5 && direction.axis === 'y');
      if (captureScreens) {
        command('xcrun', ['simctl', 'io', udid, 'screenshot', path.join(evidenceDirectory, `${screenshotStem}-before.png`)]);
      }
      const flow = await iosFlowPath(direction);
      const maestroOutput = command('maestro', [
        'test', '--udid', udid, '--test-output-dir', evidenceDirectory, flow,
      ], { timeout: 90_000 });
      const lines = processLinesForIOS(udid, pid);
      const readiness = lines.findLast((line) => line.includes(`[${pid}:`) && line.includes('SPINON_C133_READY'));
      const evaluationReadiness = lastEvalReadyLine(lines, pid);
      const record = { platform: 'ios-simulator', scenarioID: caseDefinition.id, name: caseDefinition.name, direction: direction.name, readyLine, maestroOutput };
      if (captureScreens) {
        command('xcrun', ['simctl', 'io', udid, 'screenshot', path.join(evidenceDirectory, `${screenshotStem}-after.png`)]);
      }
      const logPath = path.join(evidenceDirectory, `${screenshotStem}.log`);
      await writeFile(logPath, `${[
        `platform=iOS Simulator ${simulator.name} ${simulator.runtime}`,
        `udid=${udid}`,
        `scenario=${caseDefinition.name} (${caseDefinition.id}) direction=${direction.name}`,
        `expectedEffectiveTouchAction=${caseDefinition.effective} expectedAllowed=${caseDefinition.allowedAxis === direction.axis}`,
        evaluationReadiness,
        `maestro=${maestroOutput.trim().replaceAll('\n', ' | ')}`,
        '',
        ...lines,
        '',
      ].join('\n')}`);
      const result = validateResult({
        record,
        lines,
        readyLine: readiness,
        evalReadyLine: evaluationReadiness,
        caseDefinition,
        direction,
      });
      records.push({ ...record, ...result, log: path.basename(logPath) });
    }
  }
  return { simulator, records };
}

async function captureAndroid(serial) {
  const device = androidSimulator(serial);
  command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 120_000 });
  const records = [];
  for (const caseDefinition of cases) {
    for (const direction of directions) {
      launchAndroid(serial, caseDefinition.id);
      const { readyLine, evalReadyLine } = await waitUntilReadyForAndroid(serial);
      const coordinates = androidSwipeCoordinates(serial, direction, readyLine);
      const screenshotStem = `android-${caseDefinition.name}-${direction.name}`;
      const captureScreens = (caseDefinition.id === 4 && direction.axis === 'x')
        || (caseDefinition.id === 5 && direction.axis === 'y');
      if (captureScreens) {
        const bytes = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
        await writeFile(path.join(evidenceDirectory, `${screenshotStem}-before.png`), bytes);
      }
      command('adb', [
        '-s', serial, 'shell', 'input', 'touchscreen', 'swipe',
        String(coordinates.startX), String(coordinates.startY),
        String(coordinates.endX), String(coordinates.endY), '500',
      ]);
      let lines = [];
      for (let attempt = 0; attempt < 20; attempt += 1) {
        lines = processLinesForAndroid(serial);
        if (lines.some((line) => line.includes('kind=End') || line.includes('kind=end'))) break;
        await delay(100);
      }
      const record = { platform: 'android-emulator', serial, scenarioID: caseDefinition.id, name: caseDefinition.name, direction: direction.name, coordinates, readyLine };
      if (captureScreens) {
        const bytes = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
        await writeFile(path.join(evidenceDirectory, `${screenshotStem}-after.png`), bytes);
      }
      const logPath = path.join(evidenceDirectory, `${screenshotStem}.log`);
      await writeFile(logPath, `${[
        `platform=Android emulator ${device.model} API ${device.api} (${device.release})`,
        `serial=${serial} qemu=${device.qemu}`,
        `surfaceViewBounds=${JSON.stringify(coordinates.bounds)} scale=${coordinates.scale}`,
        `screen=${device.size} density=${device.density}`,
        `scenario=${caseDefinition.name} (${caseDefinition.id}) direction=${direction.name}`,
        `swipe=(${coordinates.startX},${coordinates.startY})->(${coordinates.endX},${coordinates.endY}) duration_ms=500`,
        `expectedEffectiveTouchAction=${caseDefinition.effective} expectedAllowed=${caseDefinition.allowedAxis === direction.axis}`,
        evalReadyLine,
        '',
        ...lines,
        '',
      ].join('\n')}`);
      const result = validateResult({ record, lines, readyLine, evalReadyLine, caseDefinition, direction });
      records.push({ ...record, ...result, log: path.basename(logPath) });
    }
  }
  return { device, records };
}

async function main() {
  const args = process.argv.slice(2);
  const iosIndex = args.indexOf('--ios-udid');
  const androidIndex = args.indexOf('--android-serial');
  const iosUDID = iosIndex >= 0 ? args[iosIndex + 1] : null;
  const androidSerial = androidIndex >= 0 ? args[androidIndex + 1] : null;
  assert.ok(iosUDID || androidSerial, 'pass --ios-udid and/or --android-serial');
  await mkdir(evidenceDirectory, { recursive: true });

  const summary = {
    schema: 'spinon-c13-3-touch-action-simulator-matrix/v1',
    capturedAt: new Date().toISOString(),
    sourceCommit: command('git', ['rev-parse', 'HEAD']).trim(),
    worktreeDiffSha256: sha256(command('git', ['diff', '--binary', 'HEAD'], { encoding: 'buffer' })),
    nativeBuilds: {},
    results: [],
  };
  if (iosUDID) {
    await stat(simulatorAppPath);
    summary.nativeBuilds.iosAppTreeSha256 = await treeSha256(simulatorAppPath);
    const ios = await captureIOS(iosUDID);
    summary.platforms = { ...summary.platforms, iosSimulator: ios.simulator };
    summary.results.push(...ios.records);
  }
  if (androidSerial) {
    summary.nativeBuilds.androidApkSha256 = sha256(await readFile(apkPath));
    const android = await captureAndroid(androidSerial);
    summary.platforms = { ...summary.platforms, androidEmulator: android.device };
    summary.results.push(...android.records);
  }
  assert.equal(summary.results.length, (iosUDID ? 10 : 0) + (androidSerial ? 10 : 0));
  const summaryPath = path.join(evidenceDirectory, 'matrix.json');
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({
    summary: path.relative(root, summaryPath),
    sha256: sha256(await readFile(summaryPath)),
    cases: summary.results.map(({ platform, name, direction, allowed, offsets, log }) => ({ platform, name, direction, allowed, offsets, log })),
  }, null, 2)}\n`);
}

main().catch((error) => {
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
