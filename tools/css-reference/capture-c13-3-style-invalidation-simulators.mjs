import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = path.resolve(fileURLToPath(new URL('../..', import.meta.url)));
const evidenceRoot = path.join(root, 'spec/internal/evidence/c13-3-style-invalidation-2026-10-15');
const runID = `${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = path.join(evidenceRoot, runID);
const v8Directory = '/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8';
const v8Revision = readFileSync(path.join(root, 'tools/v8/v8-revision.txt'), 'utf8').trim();
const androidApk = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosApp = path.join(root, 'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const bundleID = 'dev.spinon.bootstrap';
const androidActivity = `${bundleID}/.MainActivity`;
const scenarios = [
  { id: 8, name: 'style-policy-invalidation', requiredGestures: 2 },
  { id: 9, name: 'geometry-invalidation', requiredGestures: 2 },
  { id: 10, name: 'paint-invalidation', requiredGestures: 1 },
];

function command(binary, args, { timeout = 30_000, encoding = 'utf8', allowFailure = false } = {}) {
  try {
    return execFileSync(binary, args, {
      cwd: root,
      encoding,
      stdio: ['ignore', 'pipe', 'pipe'],
      timeout,
      maxBuffer: 128 * 1024 * 1024,
    });
  } catch (error) {
    if (allowFailure) return { status: error.status ?? -1, stdout: error.stdout, stderr: error.stderr };
    throw new Error(`${binary} ${args.join(' ')} 실패: ${error.stderr?.toString() ?? error.message}`);
  }
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function worktreeDiffSha256() {
  const chunks = [Buffer.from('tracked-diff\0')];
  chunks.push(command('git', ['diff', '--binary', 'HEAD'], { encoding: 'buffer' }));
  const untracked = command('git', ['ls-files', '--others', '--exclude-standard', '-z'], { encoding: 'buffer' })
    .toString('utf8')
    .split('\0')
    .filter((file) => file && !file.startsWith('spec/internal/evidence/'))
    .sort();
  for (const file of untracked) {
    chunks.push(Buffer.from(`\0untracked:${file}\0`));
    chunks.push(await readFile(path.join(root, file)));
  }
  return sha256(Buffer.concat(chunks));
}

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function count(text, pattern) {
  return [...text.matchAll(pattern)].length;
}

export function validateStyleInvalidationLog(scenario, logText) {
  const styleReports = new Map();
  const rawStyleLines = logText.split(/\r?\n/).filter((line) => line.includes('SPINON_C133_STYLE_UPDATE '));
  for (const line of rawStyleLines) {
    const marker = 'SPINON_C133_STYLE_UPDATE ';
    const report = line.slice(line.indexOf(marker) + marker.length);
    // Android logcat may truncate duplicate reports at different points in the trailing eval diagnostics.
    const identity = report.split(' eval=', 1)[0];
    if (!styleReports.has(identity) || line.length > styleReports.get(identity).length) {
      styleReports.set(identity, line);
    }
  }
  const styleLines = [...styleReports.values()];
  assert.equal(styleLines.length, 1, `${scenario.name}: style update marker가 정확히 한 번이어야 합니다`);
  const style = styleLines[0];
  assert.ok(style.includes(`c13_3_style_invalidation=applied mode=${scenario.id}`), style);
  assert.ok(/document_revision=\d+→\d+/.test(style), style);
  assert.ok(/render_tree_revision=\d+→\d+/.test(style), style);
  assert.ok(/style_revision=\d+→\d+/.test(style), style);

  const batchReports = new Map();
  for (const line of logText.split(/\r?\n/)) {
    const inputPrefix = 'SPINON_C133_INPUT_BATCH ';
    const processPrefix = 'SPINON_C133_PROCESS ';
    const inputIndex = line.indexOf(inputPrefix);
    const processIndex = line.indexOf(processPrefix);
    const report = inputIndex >= 0
      ? line.slice(inputIndex + inputPrefix.length)
      : processIndex >= 0
        ? line.slice(processIndex + processPrefix.length)
        : null;
    if (report && report.includes('c13_3_input_batch=processed')) batchReports.set(report, report);
  }
  const batches = [...batchReports.values()];
  assert.ok(batches.length > 0, `${scenario.name}: 입력 처리 batch 로그가 없습니다`);
  const text = batches.join('\n');
  assert.ok(text.includes('result=pan_started'), `${scenario.name}: active pan이 시작되지 않았습니다`);
  assert.ok(count(text, /kind=End/g) >= scenario.requiredGestures,
    `${scenario.name}: terminal End가 부족합니다`);

  if (scenario.id === 8) {
    assert.ok(style.includes('geometry_unchanged=true'), style);
    assert.ok(style.includes('scroll_state_revision=') && /scroll_state_revision=(\d+)→\1/.test(style), style);
    assert.ok(text.includes('policy_epoch=1'), `${scenario.name}: 새 style policy epoch의 Move가 없습니다`);
    assert.match(text, /gesture=2[^\n]*result=pan_rejected/,
      `${scenario.name}: 새 gesture의 touch-action:none이 적용되지 않았습니다`);
  } else if (scenario.id === 9) {
    assert.ok(style.includes('geometry_unchanged=false'), style);
    assert.ok(style.includes('inner_offset=66,'), style);
    assert.ok(style.includes('inner_range=66,'), style);
    assert.ok(text.includes('cancelled GeometryChanged'),
      `${scenario.name}: geometry 변경이 active pan을 취소하지 않았습니다`);
    assert.match(text, /gesture=2[^\n]*result=pan_started/,
      `${scenario.name}: terminal 뒤 새 geometry에서 pan이 시작되지 않았습니다`);
  } else if (scenario.id === 10) {
    assert.ok(style.includes('geometry_unchanged=true'), style);
    assert.ok(/scroll_state_revision=(\d+)→\1/.test(style), style);
    assert.ok(text.includes('policy_epoch=1'), `${scenario.name}: paint update 다음 revision Move가 없습니다`);
    assert.match(text, /policy_epoch=1[^\n]*result=moved/,
      `${scenario.name}: paint-only update가 active pan을 취소했습니다`);
  } else {
    assert.fail(`알 수 없는 style invalidation scenario ${scenario.id}`);
  }

  return {
    styleUpdate: style,
    duplicateStyleUpdateReports: rawStyleLines.length - styleLines.length,
    batches: batches.length,
    terminalEnds: count(text, /kind=End/g),
  };
}

function sceneOrigin(image, scale) {
  const target = [240, 198, 116, 255];
  let minX = image.width;
  let minY = image.height;
  let hits = 0;
  for (let y = 0; y < image.height; y += 1) {
    for (let x = 0; x < image.width; x += 1) {
      if (image.getPixel(x, y).every((value, index) => value === target[index])) {
        minX = Math.min(minX, x);
        minY = Math.min(minY, y);
        hits += 1;
      }
    }
  }
  assert.ok(hits > 100, '화면에서 root border 색을 찾지 못했습니다');
  return { x: minX - Math.round(16 * scale), y: minY - Math.round(16 * scale) };
}

export function validatePaintScreenshot(beforeBytes, afterBytes, scale) {
  const before = decodeRgbaPng(beforeBytes);
  const after = decodeRgbaPng(afterBytes);
  assert.deepEqual([after.width, after.height], [before.width, before.height]);
  const origin = sceneOrigin(after, scale);
  const point = (image, x, y) => image.getPixel(
    origin.x + Math.floor((x + 0.5) * scale),
    origin.y + Math.floor((y + 0.5) * scale),
  );
  const paintBefore = point(before, 85, 90);
  const paintAfter = point(after, 85, 90);
  const clippedOutside = point(after, 200, 100);
  assert.deepEqual(paintBefore, [23, 37, 58, 255], 'paint mutation 이전 inner content pixel');
  assert.deepEqual(paintAfter, [14, 86, 80, 255], 'paint mutation 이후 inner content pixel');
  assert.deepEqual(clippedOutside, [27, 52, 65, 255],
    'inner content가 inner scrollport border 바깥으로 새지 않아야 합니다');
  return {
    screenshot: [after.width, after.height],
    sceneOrigin: origin,
    samples: {
      beforeInnerContent: { css: [85, 90], rgba: paintBefore },
      afterInnerContent: { css: [85, 90], rgba: paintAfter },
      outsideInnerScrollport: { css: [200, 100], rgba: clippedOutside },
    },
  };
}

function parseSurface(readyLine) {
  const match = readyLine.match(/surface_texture=(\d+)x(\d+)/);
  assert.ok(match, `surface_texture 크기를 찾지 못했습니다: ${readyLine}`);
  const width = Number(match[1]);
  const height = Number(match[2]);
  const scale = width / 400;
  assert.ok(Math.abs(height / 240 - scale) < 0.01, 'surface의 CSS scale이 균일하지 않습니다');
  return { width, height, scale };
}

function androidSurfaceBounds(serial) {
  const xmlPath = '/sdcard/spinon-c13-3-style-window.xml';
  command('adb', ['-s', serial, 'shell', 'uiautomator', 'dump', xmlPath], { timeout: 20_000 });
  const xml = command('adb', ['-s', serial, 'shell', 'cat', xmlPath]);
  const match = xml.match(/class="android\.view\.SurfaceView"[^>]*bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"/);
  assert.ok(match, 'Android SurfaceView bounds를 찾지 못했습니다');
  return { left: Number(match[1]), top: Number(match[2]), right: Number(match[3]), bottom: Number(match[4]) };
}

function androidCoordinates(serial, readyLine) {
  const surface = parseSurface(readyLine);
  const bounds = androidSurfaceBounds(serial);
  return {
    startX: Math.round(bounds.left + 100 * surface.scale),
    startY: Math.round(bounds.top + 100 * surface.scale),
    endX: Math.round(bounds.left + 50 * surface.scale),
    endY: Math.round(bounds.top + 100 * surface.scale),
    surface,
    bounds,
  };
}

function processLinesForAndroid(serial) {
  return command('adb', ['-s', serial, 'logcat', '-d', '-v', 'brief']).split(/\r?\n/)
    .filter((line) => line.includes('SPINON_C133_') && line.includes('SpinonBootstrap('));
}

function currentPidForAndroid(serial) {
  const pid = Number(command('adb', ['-s', serial, 'shell', 'pidof', bundleID]).trim().split(/\s+/)[0]);
  assert.ok(Number.isInteger(pid) && pid > 0, 'Android 앱 process가 없습니다');
  return pid;
}

function processLinesForIOS(udid, pid) {
  return command('xcrun', [
    'simctl', 'spawn', udid, 'log', 'show', '--style', 'compact', '--last', '5m',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS "SPINON_C133"',
  ]).split(/\r?\n/).filter((line) => line.includes(`SpinonBootstrap[${pid}:`));
}

async function waitFor(readLines, predicate, description, timeoutMs = 30_000) {
  const endAt = Date.now() + timeoutMs;
  let latest = [];
  while (Date.now() < endAt) {
    latest = readLines();
    if (predicate(latest)) return latest;
    await delay(250);
  }
  assert.fail(`${description} 대기 시간 초과\n${latest.join('\n')}`);
}

function androidSimulator(serial) {
  assert.match(serial, /^emulator-\d+$/, `실기기 입력을 막기 위해 emulator serial만 허용합니다: ${serial}`);
  assert.equal(command('adb', ['-s', serial, 'get-state']).trim(), 'device');
  const qemu = command('adb', ['-s', serial, 'shell', 'getprop', 'ro.kernel.qemu']).trim();
  const api = command('adb', ['-s', serial, 'shell', 'getprop', 'ro.build.version.sdk']).trim();
  assert.equal(qemu, '1', `${serial}이 Android emulator가 아닙니다`);
  assert.equal(api, '37', `${serial}의 Android API가 37이 아닙니다`);
  return {
    serial,
    model: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.product.model']).trim(),
    api: Number(api),
    screen: command('adb', ['-s', serial, 'shell', 'wm', 'size']).trim(),
    density: command('adb', ['-s', serial, 'shell', 'wm', 'density']).trim(),
  };
}

function iosSimulator(udid) {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']));
  for (const [runtime, entries] of Object.entries(devices.devices)) {
    const device = entries.find((entry) => entry.udid === udid);
    if (!device) continue;
    assert.equal(device.state, 'Booted');
    assert.equal(device.isAvailable, true);
    assert.match(runtime, /iOS/);
    assert.match(device.deviceTypeIdentifier ?? '', /iPhone/);
    return { ...device, runtime };
  }
  assert.fail(`부팅된 iPhone Simulator UDID가 아닙니다: ${udid}`);
}

async function screenshotAndroid(serial, filename) {
  const png = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
  assert.equal(png.toString('hex', 0, 8), '89504e470d0a1a0a');
  await writeFile(path.join(evidenceDirectory, filename), png);
  return png;
}

async function screenshotIOS(udid, filename) {
  const target = path.join(evidenceDirectory, filename);
  command('xcrun', ['simctl', 'io', udid, 'screenshot', target]);
  const png = await readFile(target);
  assert.equal(png.toString('hex', 0, 8), '89504e470d0a1a0a');
  return png;
}

function androidLaunch(serial, scenarioID) {
  command('adb', ['-s', serial, 'shell', 'am', 'force-stop', bundleID]);
  command('adb', ['-s', serial, 'logcat', '-c']);
  command('adb', [
    '-s', serial, 'shell', 'am', 'start', '-W', '-n', androidActivity,
    '--ez', 'spinon_c133_gesture_fixture', 'true',
    '--ei', 'spinon_c133_policy_scenario_id', String(scenarioID),
  ], { timeout: 90_000 });
}

function iosLaunch(udid, scenarioID) {
  const output = command('xcrun', [
    'simctl', 'launch', '--terminate-running-process', udid, bundleID,
    '--spinon-c133-scroll-gesture', `--spinon-c133-policy-scenario-id=${scenarioID}`,
  ]);
  const pid = Number(output.match(/:\s*(\d+)\s*$/)?.[1]);
  assert.ok(Number.isInteger(pid) && pid > 0, `iOS process id를 읽지 못했습니다: ${output}`);
  return pid;
}

async function runIOSSwipes(udid, scenario, gestures) {
  const flow = path.join(evidenceDirectory, `ios-${scenario.name}.yaml`);
  const swipe = [
    '- swipe:',
    '    start: 25%, 48%',
    '    end: 13%, 48%',
    '    duration: 500',
  ].join('\n');
  await writeFile(flow, [`appId: ${bundleID}`, '---', ...Array(gestures).fill(swipe), ''].join('\n'));
  return command('mise', ['exec', '--', 'maestro', 'test', '--udid', udid,
    '--test-output-dir', evidenceDirectory, flow], { timeout: 120_000 });
}

async function captureIOS(udid, selectedScenarios) {
  const device = iosSimulator(udid);
  command('xcrun', ['simctl', 'install', udid, iosApp], { timeout: 120_000 });
  const records = [];
  for (const scenario of selectedScenarios) {
    const pid = iosLaunch(udid, scenario.id);
    const readLines = () => processLinesForIOS(udid, pid);
    const ready = await waitFor(readLines, (lines) =>
      lines.some((line) => line.includes('SPINON_C133_READY'))
      && lines.some((line) => line.includes('SPINON_C133_EVAL_READY')),
    `${scenario.name} iOS ready`);
    const readyLine = ready.find((line) => line.includes('SPINON_C133_READY'));
    const before = scenario.id === 10
      ? await screenshotIOS(udid, 'ios-paint-invalidation-before.png') : null;
    const maestroOutput = await runIOSSwipes(udid, scenario, scenario.requiredGestures);
    const lines = await waitFor(readLines, (current) => {
      const log = current.join('\n');
      return log.includes('SPINON_C133_STYLE_UPDATE')
        && count(log, /kind=End/g) >= scenario.requiredGestures;
    }, `${scenario.name} iOS runtime completion`);
    const after = await screenshotIOS(udid, `ios-${scenario.name}-after.png`);
    const text = lines.join('\n');
    const result = validateStyleInvalidationLog(scenario, text);
    const pixels = scenario.id === 10 ? validatePaintScreenshot(before, after, 3) : null;
    const logPath = path.join(evidenceDirectory, `ios-${scenario.name}.log`);
    await writeFile(logPath, [
      `platform=iOS Simulator ${device.name} ${device.runtime}`,
      `udid=${udid} pid=${pid} gestures=${scenario.requiredGestures}`,
      `ready=${readyLine}`,
      `maestro=${maestroOutput.trim().replaceAll('\n', ' | ')}`,
      `pixels=${JSON.stringify(pixels)}`,
      '', text, '',
    ].join('\n'));
    records.push({ platform: 'ios-simulator', scenario: scenario.name, pid, ...result, pixels,
      screenshot: `ios-${scenario.name}-after.png`, log: path.basename(logPath) });
  }
  return { device, records };
}

async function captureAndroid(serial, selectedScenarios) {
  const device = androidSimulator(serial);
  command('adb', ['-s', serial, 'install', '-r', androidApk], { timeout: 120_000 });
  const records = [];
  for (const scenario of selectedScenarios) {
    androidLaunch(serial, scenario.id);
    const pid = currentPidForAndroid(serial);
    const readLines = () => processLinesForAndroid(serial).filter((line) => line.includes(`(${pid}):`));
    const ready = await waitFor(readLines, (lines) =>
      lines.some((line) => line.includes('SPINON_C133_READY'))
      && lines.some((line) => line.includes('SPINON_C133_EVAL_READY')),
    `${scenario.name} Android ready`);
    const readyLine = ready.find((line) => line.includes('SPINON_C133_READY'));
    const coordinates = androidCoordinates(serial, readyLine);
    const before = scenario.id === 10
      ? await screenshotAndroid(serial, 'android-paint-invalidation-before.png') : null;
    for (let gesture = 0; gesture < scenario.requiredGestures; gesture += 1) {
      command('adb', [
        '-s', serial, 'shell', 'input', 'touchscreen', 'swipe',
        String(coordinates.startX), String(coordinates.startY),
        String(coordinates.endX), String(coordinates.endY), '500',
      ]);
      await waitFor(readLines, (lines) => count(lines.join('\n'), /kind=End/g) >= gesture + 1,
        `${scenario.name} Android gesture ${gesture + 1} terminal`);
    }
    const lines = await waitFor(readLines, (current) => current.some((line) => line.includes('SPINON_C133_STYLE_UPDATE')),
      `${scenario.name} Android style update`);
    const after = await screenshotAndroid(serial, `android-${scenario.name}-after.png`);
    const text = lines.join('\n');
    const result = validateStyleInvalidationLog(scenario, text);
    const pixels = scenario.id === 10 ? validatePaintScreenshot(before, after, coordinates.surface.scale) : null;
    const logPath = path.join(evidenceDirectory, `android-${scenario.name}.log`);
    await writeFile(logPath, [
      `platform=Android emulator ${device.model} API ${device.api}`,
      `serial=${serial} pid=${pid} qemu=1 gestures=${scenario.requiredGestures}`,
      `surface=${coordinates.surface.width}x${coordinates.surface.height} scale=${coordinates.surface.scale}`,
      `surfaceViewBounds=${JSON.stringify(coordinates.bounds)}`,
      `swipe=(${coordinates.startX},${coordinates.startY})->(${coordinates.endX},${coordinates.endY}) duration_ms=500`,
      `pixels=${JSON.stringify(pixels)}`,
      '', text, '',
    ].join('\n'));
    records.push({ platform: 'android-emulator', scenario: scenario.name, pid, ...result, pixels,
      screenshot: `android-${scenario.name}-after.png`, log: path.basename(logPath) });
  }
  return { device, records };
}

async function buildApps() {
  assert.equal(command('git', ['-C', v8Directory, 'rev-parse', 'HEAD']).trim(), v8Revision,
    '공유 V8 checkout이 저장소 고정 revision과 다릅니다');
  const androidArchive = path.join(v8Directory, 'out/boson-android-mac/obj/libv8_monolith.a');
  const iosArchive = path.join(v8Directory, 'out/boson-ios-sim/obj/libv8_monolith.a');
  for (const archive of [androidArchive, iosArchive]) {
    assert.ok((await stat(archive)).size > 0, `V8 archive가 비어 있습니다: ${archive}`);
  }
  const envArgs = [
    `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1',
  ];
  const ios = command('mise', ['exec', '--', 'env', ...envArgs, 'bun', 'run', 'build:ios-sim'],
    { timeout: 1_200_000 });
  await writeFile(path.join(evidenceDirectory, 'ios-build.log'), ios);
  const android = command('mise', ['exec', '--', 'env', ...envArgs, 'bun', 'run', 'build:android'],
    { timeout: 1_200_000 });
  await writeFile(path.join(evidenceDirectory, 'android-build.log'), android);
}

async function main() {
  const args = process.argv.slice(2);
  const iosIndex = args.indexOf('--ios-udid');
  const androidIndex = args.indexOf('--android-serial');
  const scenarioIndex = args.indexOf('--scenario-id');
  const iosUDID = iosIndex >= 0 ? args[iosIndex + 1] : null;
  const androidSerial = androidIndex >= 0 ? args[androidIndex + 1] : null;
  const scenarioID = scenarioIndex >= 0 ? Number(args[scenarioIndex + 1]) : null;
  const selectedScenarios = scenarioID === null
    ? scenarios
    : scenarios.filter((scenario) => scenario.id === scenarioID);
  assert.ok(iosUDID || androidSerial, '사용법: ... --ios-udid <booted iPhone> --android-serial <emulator-serial>');
  assert.ok(selectedScenarios.length > 0, `알 수 없는 C13.3 scenario id=${scenarioID}`);
  await mkdir(evidenceDirectory, { recursive: true });
  if (!args.includes('--skip-build')) await buildApps();

  const matrix = {
    schema: 'spinon-c13-3-style-invalidation-simulator-matrix/v1',
    capturedAt: new Date().toISOString(),
    runID,
    sourceCommit: command('git', ['rev-parse', 'HEAD']).trim(),
    worktreeDiffSha256: await worktreeDiffSha256(),
    v8Revision,
    results: [],
  };
  if (iosUDID) {
    matrix.ios = await captureIOS(iosUDID, selectedScenarios);
    matrix.iosAppPath = path.relative(root, iosApp);
  }
  if (androidSerial) {
    matrix.android = await captureAndroid(androidSerial, selectedScenarios);
    matrix.androidApkPath = path.relative(root, androidApk);
  }
  matrix.results = [matrix.ios?.records ?? [], matrix.android?.records ?? []].flat();
  const matrixPath = path.join(evidenceDirectory, 'matrix.json');
  await writeFile(matrixPath, `${JSON.stringify(matrix, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({
    evidence: path.relative(root, evidenceDirectory),
    matrixSha256: sha256(await readFile(matrixPath)),
    results: matrix.results.map(({ platform, scenario, screenshot, log, pixels }) => ({
      platform, scenario, screenshot, log, pixels,
    })),
  }, null, 2)}\n`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
