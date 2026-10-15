import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const evidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-touch-action-tap-arbitration-2026-10-15',
);
const apkPath = path.join(root, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const simulatorAppPath = path.join(
  root,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app',
);
const bundleID = 'dev.spinon.bootstrap';
const androidActivity = 'dev.spinon.bootstrap/.MainActivity';

function command(name, args, { timeout = 30_000, encoding = 'utf8' } = {}) {
  return execFileSync(name, args, {
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
    for (const entry of (await readdir(current, { withFileTypes: true }))
      .sort((left, right) => left.name.localeCompare(right.name))) {
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

function iosSimulator(udid) {
  const deviceMap = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']));
  for (const [runtime, devices] of Object.entries(deviceMap.devices)) {
    const device = devices.find((candidate) => candidate.udid === udid);
    if (device) {
      assert.equal(device.state, 'Booted', `iOS simulator ${udid} must be booted`);
      assert.equal(device.isAvailable, true, `iOS simulator ${udid} must be available`);
      assert.match(runtime, /iOS/, `target is not an iOS simulator: ${runtime}`);
      assert.match(device.deviceTypeIdentifier ?? '', /iPhone/, 'expected iPhone Simulator');
      return { ...device, runtime };
    }
  }
  assert.fail(`${udid} is not an available simulator UDID from simctl`);
}

function androidSimulator(serial) {
  assert.match(serial, /^emulator-\d+$/, `refusing non-emulator Android target ${serial}`);
  assert.equal(command('adb', ['-s', serial, 'get-state']).trim(), 'device');
  const device = {
    model: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.product.model']).trim(),
    qemu: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.kernel.qemu']).trim(),
    api: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.build.version.sdk']).trim(),
    release: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.build.version.release']).trim(),
    abi: command('adb', ['-s', serial, 'shell', 'getprop', 'ro.product.cpu.abi']).trim(),
    avd: command('adb', ['-s', serial, 'emu', 'avd', 'name']).split('\n')[0].trim(),
    screen: command('adb', ['-s', serial, 'shell', 'wm', 'size']).trim(),
    density: command('adb', ['-s', serial, 'shell', 'wm', 'density']).trim(),
  };
  assert.equal(device.qemu, '1', `Android target is not an emulator: ${serial}`);
  assert.equal(device.api, '37', `expected API 37 emulator, got ${device.api}`);
  assert.equal(device.abi, 'arm64-v8a', `expected ARM64 emulator, got ${device.abi}`);
  return device;
}

function iosLines(udid, pid) {
  return command('xcrun', [
    'simctl', 'spawn', udid, 'log', 'show', '--style', 'compact', '--last', '5m',
    '--predicate', 'process == "SpinonBootstrap" AND eventMessage CONTAINS "SPINON_C133"',
  ]).split('\n').filter((line) => line.includes(`SpinonBootstrap[${pid}:`));
}

function androidLines(serial) {
  return command('adb', ['-s', serial, 'logcat', '-d', '-v', 'brief']).split('\n')
    .filter((line) => line.includes('SPINON_C133_') && line.includes('SpinonBootstrap('));
}

async function waitFor(predicate, read, message) {
  for (let attempt = 0; attempt < 80; attempt += 1) {
    const value = read();
    if (predicate(value)) return value;
    await delay(250);
  }
  assert.fail(message);
}

function parseOffsets(line) {
  const named = line.match(/scroll_state_revision=(\d+) offsets=root:([^;]+);outer:([^;]+);inner:([^\s]+)/);
  if (named) {
    const pair = (value) => value.split(',').map(Number);
    return {
      revision: Number(named[1]),
      root: pair(named[2]),
      outer: pair(named[3]),
      inner: pair(named[4]),
    };
  }
  const compact = line.match(/scroll_state_revision=(\d+).*?scroll_offsets=\[([^\]]+)\]/);
  assert.ok(compact, `missing scroll revision and offsets: ${line}`);
  const owners = compact[2].split(';').map((owner) => owner.split(',').map(Number));
  assert.equal(owners.length, 3, `expected root, outer, inner offsets: ${compact[2]}`);
  return { revision: Number(compact[1]), root: owners[0], outer: owners[1], inner: owners[2] };
}

function lastStateLine(lines) {
  return lines.findLast((line) => /scroll_state_revision=\d+/.test(line)
    && (line.includes('offsets=root:') || line.includes('scroll_offsets=[')));
}

function assertNoScroll(state, label) {
  assert.equal(state.revision, 0, `${label}: a tap changed scroll revision`);
  assert.deepEqual(state.root, [0, 0], `${label}: root offset changed`);
  assert.deepEqual(state.outer, [0, 0], `${label}: outer offset changed`);
  assert.deepEqual(state.inner, [0, 0], `${label}: inner offset changed`);
}

function assertSwipe(state, label) {
  assert.ok(state.revision > 0, `${label}: pan did not advance scroll revision`);
  assert.deepEqual(state.root, [0, 0], `${label}: root offset changed`);
  assert.deepEqual(state.outer, [0, 0], `${label}: outer offset changed`);
  assert.ok(state.inner[0] > 0.5, `${label}: horizontal pan did not move inner X offset`);
  assert.ok(Math.abs(state.inner[1]) < 0.5, `${label}: horizontal pan changed inner Y offset`);
}

function assertAndroidFixtureScreenshot(bytes, device, point, state, markerVisible, label) {
  const png = decodeRgbaPng(bytes);
  const screenSize = device.screen.match(/(\d+)x(\d+)/);
  assert.ok(screenSize, `missing Android screen size: ${device.screen}`);
  assert.equal(png.width, Number(screenSize[1]), `${label}: screenshot width differs from AVD`);
  assert.equal(png.height, Number(screenSize[2]), `${label}: screenshot height differs from AVD`);
  const pixelAtCss = ([x, y]) => png.getPixel(
    Math.round(point.origin.x + x * point.scale),
    Math.round(point.origin.y + y * point.scale),
  );
  assert.deepEqual(pixelAtCss([5, 5]), [14, 19, 31, 255],
    `${label}: C13.3 scene background is missing from the screenshot`);
  assert.deepEqual(pixelAtCss([20, 40]), [52, 66, 86, 255],
    `${label}: C13.3 root scrollport is missing from the screenshot`);
  if (markerVisible) {
    const markerCenter = [225 - state.inner[0] + 12, 122];
    assert.deepEqual(pixelAtCss(markerCenter), [227, 84, 78, 255],
      `${label}: scrolled inner marker is not visible at its recorded offset`);
  } else {
    assert.deepEqual(pixelAtCss([237, 122]), [27, 52, 65, 255],
      `${label}: unscrolled inner marker escaped its clip`);
  }
}

async function captureAndroidScreenshot(serial, filename, device, point, state, markerVisible, label) {
  const bytes = command('adb', ['-s', serial, 'exec-out', 'screencap', '-p'], { encoding: 'buffer' });
  assertAndroidFixtureScreenshot(bytes, device, point, state, markerVisible, label);
  await writeFile(filename, bytes);
}

function androidBounds(serial) {
  const remote = '/sdcard/spinon-c13-3-tap-window.xml';
  command('adb', ['-s', serial, 'shell', 'uiautomator', 'dump', remote], { timeout: 20_000 });
  const xml = command('adb', ['-s', serial, 'shell', 'cat', remote]);
  const match = xml.match(/class="android\.view\.SurfaceView"[^>]*bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"/);
  assert.ok(match, 'Android fixture SurfaceView bounds were not found');
  return { left: Number(match[1]), top: Number(match[2]), right: Number(match[3]), bottom: Number(match[4]) };
}

function androidCssPoint(serial, readyLine, cssPoint) {
  const surface = readyLine.match(/surface_texture=(\d+)x(\d+)/);
  assert.ok(surface, `missing Android surface size: ${readyLine}`);
  const scaleX = Number(surface[1]) / 400;
  const scaleY = Number(surface[2]) / 240;
  assert.ok(Math.abs(scaleX - scaleY) <= 0.01, `non-uniform Android scale ${scaleX}/${scaleY}`);
  const viewSizeMatch = readyLine.match(/surface_view_size_px=(\d+)x(\d+)/);
  const viewSize = viewSizeMatch
    ? { width: Number(viewSizeMatch[1]), height: Number(viewSizeMatch[2]) }
    : { width: Number(surface[1]), height: Number(surface[2]) };
  assert.deepEqual(viewSize, { width: Number(surface[1]), height: Number(surface[2]) },
    'Android SurfaceView and texture dimensions must match before absolute-point mapping');
  const bounds = androidBounds(serial);
  const visibleWidth = bounds.right - bounds.left;
  const visibleHeight = bounds.bottom - bounds.top;
  const clippedWidth = viewSize.width - visibleWidth;
  const clippedHeight = viewSize.height - visibleHeight;
  assert.ok(clippedWidth >= 0 && clippedHeight >= 0,
    `Android visible bounds exceed SurfaceView size: bounds=${JSON.stringify(bounds)} view=${JSON.stringify(viewSize)}`);
  assert.equal(clippedWidth % 2, 0, 'Android SurfaceView horizontal clipping must be symmetric');
  assert.equal(clippedHeight % 2, 0, 'Android SurfaceView vertical clipping must be symmetric');
  const origin = {
    x: bounds.left - clippedWidth / 2,
    y: bounds.top - clippedHeight / 2,
  };
  const point = {
    bounds,
    origin,
    viewSize,
    scale: scaleX,
    x: Math.round(origin.x + cssPoint[0] * scaleX),
    y: Math.round(origin.y + cssPoint[1] * scaleY),
  };
  assert.ok(point.x > bounds.left && point.x < bounds.right
    && point.y > bounds.top && point.y < bounds.bottom, 'tap/swipe point is outside SurfaceView');
  return point;
}

function assertAndroidTouchTransform(line, point, cssPoint) {
  const local = line.match(/surface_local_physical_px=([\d.]+),([\d.]+)/);
  const screen = line.match(/screen_physical_px=([\d.]+),([\d.]+)/);
  const origin = line.match(/surface_origin_screen_px=(\d+),(\d+)/);
  const viewSize = line.match(/surface_view_size_px=(\d+)x(\d+)/);
  assert.ok(local && screen && origin && viewSize, `missing Android touch transform: ${line}`);
  assert.deepEqual({ x: Number(origin[1]), y: Number(origin[2]) }, point.origin,
    'Android runtime SurfaceView origin differs from the accessibility-clip reconstruction');
  assert.deepEqual({ width: Number(viewSize[1]), height: Number(viewSize[2]) }, point.viewSize,
    'Android runtime SurfaceView size differs from the draw surface');
  assert.deepEqual({ x: Number(screen[1]), y: Number(screen[2]) }, { x: point.x, y: point.y },
    'Android runtime screen coordinate differs from the injected ADB coordinate');
  assert.ok(Math.abs(Number(local[1]) / point.scale - cssPoint[0]) <= 1.5
    && Math.abs(Number(local[2]) / point.scale - cssPoint[1]) <= 1.5,
  `Android local SurfaceView point maps to the wrong CSS point: ${line}`);
}

function iosScreenPoint(readyLine, cssPoint) {
  const origin = readyLine.match(/canvas_origin_ui_points=([\d.]+),([\d.]+)/);
  const canvas = readyLine.match(/canvas_bounds=([\d.]+)x([\d.]+)/);
  const screen = readyLine.match(/screen_bounds_ui_points=([\d.]+)x([\d.]+)/);
  assert.ok(origin && canvas && screen, `missing iOS point transform in ready log: ${readyLine}`);
  const x = Number(origin[1]) + cssPoint[0] * Number(canvas[1]) / 400;
  const y = Number(origin[2]) + cssPoint[1] * Number(canvas[2]) / 240;
  const point = {
    uiX: x,
    uiY: y,
    xPercent: (100 * x / Number(screen[1])).toFixed(2),
    yPercent: (100 * y / Number(screen[2])).toFixed(2),
    screenX: Math.round(x),
    screenY: Math.round(y),
    canvasOrigin: [Number(origin[1]), Number(origin[2])],
    canvasSize: [Number(canvas[1]), Number(canvas[2])],
    screenSize: [Number(screen[1]), Number(screen[2])],
    screenScale: Number(readyLine.match(/screen_scale=([\d.]+)/)?.[1]),
  };
  assert.ok(Number.isFinite(point.screenScale) && point.screenScale > 0, 'missing iOS screen scale');
  return point;
}

function iOSFlowPath(name, steps) {
  const filename = path.join(evidenceDirectory, `${name}.yaml`);
  return writeFile(filename, [`appId: ${bundleID}`, '---', ...steps, ''].join('\n'))
    .then(() => filename);
}

async function runIOS(udid) {
  const device = iosSimulator(udid);
  command('xcrun', ['simctl', 'install', udid, simulatorAppPath], { timeout: 120_000 });
  const records = [];
  for (const scenario of [3, 4]) {
    const name = scenario === 3 ? 'none' : 'pan-x';
    const effective = name;
    const output = command('xcrun', [
      'simctl', 'launch', '--terminate-running-process', udid, bundleID,
      '--spinon-c133-scroll-gesture', '--spinon-c133-tap-probe',
      `--spinon-c133-policy-scenario-id=${scenario}`,
    ]);
    const pid = Number(output.match(/:\s*(\d+)\s*$/)?.[1]);
    assert.ok(Number.isInteger(pid) && pid > 0, `could not parse iOS PID: ${output}`);
    const lines = await waitFor(
      (current) => current.some((line) => line.includes('SPINON_C133_READY'))
        && current.some((line) => line.includes('SPINON_C133_TAP_IOS arbitration_ready=true')),
      () => iosLines(udid, pid),
      `iOS scenario ${scenario} did not reach tap-ready state`,
    );
    const readyLine = lines.findLast((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = lines.findLast((line) => line.includes('SPINON_C133_EVAL_READY'));
    assert.ok(readyLine && evalReadyLine, `iOS scenario ${scenario} has incomplete ready reports`);
    assert.ok(evalReadyLine.includes(`touch_action_effective=${effective}`), evalReadyLine);
    assert.ok(lines.some((line) => line.includes(`arbitration_ready=true effective=${effective}`)
      && line.includes(`native_pan_enabled=${scenario === 4}`)), `wrong native arbitration for ${name}`);
    const tapPoint = iosScreenPoint(readyLine, [148, 132]);
    const screenshotStem = `ios-${name}`;
    command('xcrun', ['simctl', 'io', udid, 'screenshot', path.join(evidenceDirectory, `${screenshotStem}-before.png`)]);
    const tapFlow = await iOSFlowPath(`${screenshotStem}-tap`, [
      '- tapOn:',
      `    point: "${tapPoint.screenX}, ${tapPoint.screenY}"`,
    ]);
    const tapOutput = command('mise', [
      'exec', '--', 'maestro', 'test', '--udid', udid,
      '--test-output-dir', evidenceDirectory, tapFlow,
    ], { timeout: 90_000 });
    let afterTapLines = await waitFor(
      (current) => current.some((line) => line.includes('SPINON_C133_TAP_IOS activation=true count=1'))
        && current.filter((line) => line.includes('SPINON_C133_TAP_IOS_DRAW')
          && /scroll_state_revision=\d+/.test(line)).length >= 2,
      () => iosLines(udid, pid),
      `iOS scenario ${scenario} did not record one tap and a post-tap draw`,
    );
    const tapLine = afterTapLines.findLast((line) => line.includes('SPINON_C133_TAP_IOS activation=true'));
    const tapCss = tapLine.match(/point_css_px=([\d.]+),([\d.]+)/);
    assert.ok(tapCss, `missing iOS CSS tap point: ${tapLine}`);
    assert.ok(Math.abs(Number(tapCss[1]) - 148) <= 2.5 && Math.abs(Number(tapCss[2]) - 132) <= 2.5,
      `iOS tapped wrong CSS point: ${tapLine}`);
    const postTapDraws = afterTapLines.filter((line) => line.includes('SPINON_C133_TAP_IOS_DRAW')
      && /scroll_state_revision=\d+/.test(line));
    const postTapState = parseOffsets(postTapDraws.at(-1));
    assertNoScroll(postTapState, `iOS ${name} tap`);
    assert.equal(afterTapLines.filter((line) => line.includes('SPINON_C133_TAP_IOS activation=true')).length, 1);
    assert.equal(afterTapLines.filter((line) => line.includes('action=pan-began')).length, 0,
      `iOS ${name} tap began a pan`);
    command('xcrun', ['simctl', 'io', udid, 'screenshot', path.join(evidenceDirectory, `${screenshotStem}-after-tap.png`)]);

    let finalState = postTapState;
    let finalLines = afterTapLines;
    let swipeOutput = '';
    if (scenario === 4) {
      const endPoint = iosScreenPoint(readyLine, [88, 132]);
      const swipeFlow = await iOSFlowPath(`${screenshotStem}-swipe`, [
        '- swipe:',
        `    start: ${tapPoint.screenX}, ${tapPoint.screenY}`,
        `    end: ${endPoint.screenX}, ${endPoint.screenY}`,
        '    duration: 500',
      ]);
      swipeOutput = command('mise', [
        'exec', '--', 'maestro', 'test', '--udid', udid,
        '--test-output-dir', evidenceDirectory, swipeFlow,
      ], { timeout: 90_000 });
      finalLines = await waitFor(
        (current) => current.some((line) => line.includes('action=pan-began'))
          && current.some((line) => line.includes('kind=End') || line.includes('kind=end')),
        () => iosLines(udid, pid),
        'iOS pan-x swipe did not begin and end a native pan',
      );
      const terminal = finalLines.findLast((line) => line.includes('kind=End') || line.includes('kind=end'));
      finalState = parseOffsets(terminal);
      assertSwipe(finalState, 'iOS pan-x swipe');
      assert.equal(finalLines.filter((line) => line.includes('SPINON_C133_TAP_IOS activation=true')).length, 1,
        'iOS pan was incorrectly recognized as a second tap');
      assert.equal(finalLines.filter((line) => line.includes('SPINON_C133_TAP_IOS activation_rejected=true')).length, 0);
      assert.equal(finalLines.filter((line) => line.includes('action=pan-began')).length, 1,
        'iOS expected exactly one native pan start');
      command('xcrun', ['simctl', 'io', udid, 'screenshot', path.join(evidenceDirectory, `${screenshotStem}-after-swipe.png`)]);
    }
    await writeFile(path.join(evidenceDirectory, `${screenshotStem}.log`), `${[
      `platform=iOS Simulator ${device.name} ${device.runtime}`,
      `udid=${udid} pid=${pid}`,
      `scenario=${scenario} effective=${effective} route=--spinon-c133-scroll-gesture tap_probe=true`,
      `tap_css=148,132 screen_transform=${JSON.stringify(tapPoint)}`,
      `initial_ready=${readyLine}`,
      `eval_ready=${evalReadyLine}`,
      `maestro_tap=${tapOutput.trim().replaceAll('\n', ' | ')}`,
      ...(scenario === 4 ? [`maestro_swipe=${swipeOutput.trim().replaceAll('\n', ' | ')}`] : []),
      `post_tap_state=${JSON.stringify(postTapState)}`,
      `final_state=${JSON.stringify(finalState)}`,
      '',
      ...finalLines,
      '',
    ].join('\n')}`);
    records.push({ platform: 'ios-simulator', scenario, name, tapPoint, postTapState, finalState });
    afterTapLines = [];
  }
  return { device, records };
}

async function runAndroid(serial, apkSha256) {
  const device = androidSimulator(serial);
  command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 120_000 });
  const records = [];
  for (const scenario of [3, 4]) {
    const name = scenario === 3 ? 'none' : 'pan-x';
    command('adb', ['-s', serial, 'shell', 'am', 'force-stop', bundleID]);
    command('adb', ['-s', serial, 'logcat', '-c']);
    command('adb', [
      '-s', serial, 'shell', 'am', 'start', '-W', '-n', androidActivity,
      '--ez', 'spinon_c133_gesture_fixture', 'true',
      '--ez', 'spinon_c133_tap_probe', 'true',
      '--ei', 'spinon_c133_policy_scenario_id', String(scenario),
    ]);
    const readyLines = await waitFor(
      (current) => current.some((line) => line.includes('SPINON_C133_READY'))
        && current.some((line) => line.includes('SPINON_C133_EVAL_READY')),
      () => androidLines(serial),
      `Android scenario ${scenario} did not reach fixture ready state`,
    );
    const readyLine = readyLines.findLast((line) => line.includes('SPINON_C133_READY'));
    const evalReadyLine = readyLines.findLast((line) => line.includes('SPINON_C133_EVAL_READY'));
    assert.ok(evalReadyLine.includes(`touch_action_effective=${name}`), evalReadyLine);
    const point = androidCssPoint(serial, readyLine, [148, 132]);
    const screenshotStem = `android-${name}`;
    await captureAndroidScreenshot(serial,
      path.join(evidenceDirectory, `${screenshotStem}-before.png`), device, point,
      { inner: [0, 0] }, false, `Android ${name} initial`);
    command('adb', [
      '-s', serial, 'shell', 'input', 'touchscreen', 'tap', String(point.x), String(point.y),
    ]);
    let afterTapLines = await waitFor(
      (current) => current.some((line) => line.includes('SPINON_C133_TAP_ANDROID activation=true count=1'))
        && current.some((line) => (line.includes('kind=End') || line.includes('kind=end'))
          && line.includes('scroll_state_revision=')),
      () => androidLines(serial),
      `Android scenario ${scenario} did not record one native click and C13.3 End`,
    );
    const tapLine = afterTapLines.findLast((line) => line.includes('SPINON_C133_TAP_ANDROID activation=true'));
    const cssPoint = tapLine.match(/tap_css_px=([\d.]+),([\d.]+)/);
    assert.ok(cssPoint, `missing Android CSS tap point: ${tapLine}`);
    assert.ok(Math.abs(Number(cssPoint[1]) - 148) <= 1.5
      && Math.abs(Number(cssPoint[2]) - 132) <= 1.5,
    `Android tapped wrong CSS point: ${tapLine}`);
    const tapEnd = afterTapLines.findLast((line) => (line.includes('kind=End') || line.includes('kind=end'))
      && line.includes('scroll_state_revision='));
    const postTapState = parseOffsets(tapEnd);
    assertNoScroll(postTapState, `Android ${name} tap`);
    assert.equal(afterTapLines.filter((line) => line.includes('SPINON_C133_TAP_ANDROID activation=true')).length, 1);
    assert.equal(afterTapLines.filter((line) => line.includes('SPINON_C133_TAP_ANDROID unexpected_click=true')).length, 0);
    assert.equal(afterTapLines.filter((line) => line.includes('action=pan-began')).length, 0,
      `Android ${name} tap began a pan`);
    await captureAndroidScreenshot(serial,
      path.join(evidenceDirectory, `${screenshotStem}-after-tap.png`), device, point,
      postTapState, false, `Android ${name} after tap`);

    let finalState = postTapState;
    let finalLines = afterTapLines;
    let swipeCoordinates = null;
    if (scenario === 4) {
      const endPoint = androidCssPoint(serial, readyLine, [88, 132]);
      swipeCoordinates = { from: [point.x, point.y], to: [endPoint.x, endPoint.y], scale: point.scale, bounds: point.bounds };
      command('adb', [
        '-s', serial, 'shell', 'input', 'touchscreen', 'swipe',
        String(point.x), String(point.y), String(endPoint.x), String(endPoint.y), '500',
      ]);
      finalLines = await waitFor(
        (current) => current.some((line) => line.includes('action=pan-began'))
          && current.filter((line) => (line.includes('kind=End') || line.includes('kind=end'))
            && line.includes('scroll_state_revision=')).length >= 2,
        () => androidLines(serial),
        'Android pan-x swipe did not begin and end a native pan',
      );
      const terminal = finalLines.findLast((line) => (line.includes('kind=End') || line.includes('kind=end'))
        && line.includes('scroll_state_revision='));
      finalState = parseOffsets(terminal);
      assertSwipe(finalState, 'Android pan-x swipe');
      assert.equal(finalLines.filter((line) => line.includes('SPINON_C133_TAP_ANDROID activation=true')).length, 1,
        'Android pan was incorrectly recognized as a second click');
      assert.equal(finalLines.filter((line) => line.includes('SPINON_C133_TAP_ANDROID unexpected_click=true')).length, 0,
        'Android produced an unexpected click after a pan');
      assert.equal(finalLines.filter((line) => line.includes('action=pan-began')).length, 1,
        'Android expected exactly one native pan start');
      await captureAndroidScreenshot(serial,
        path.join(evidenceDirectory, `${screenshotStem}-after-swipe.png`), device, point,
        finalState, true, `Android ${name} after swipe`);
    }
    await writeFile(path.join(evidenceDirectory, `${screenshotStem}.log`), `${[
      `platform=Android emulator ${device.model} API ${device.api} (${device.release})`,
      `serial=${serial} avd=${device.avd} abi=${device.abi} qemu=${device.qemu}`,
      `screen=${device.screen} density=${device.density} apk_sha256=${apkSha256}`,
      `scenario=${scenario} effective=${name} tap_css=148,132 surface_scale=${point.scale} bounds=${JSON.stringify(point.bounds)}`,
      `tap_screen_px=${point.x},${point.y} swipe=${JSON.stringify(swipeCoordinates)}`,
      `initial_ready=${readyLine}`,
      `eval_ready=${evalReadyLine}`,
      `post_tap_state=${JSON.stringify(postTapState)}`,
      `final_state=${JSON.stringify(finalState)}`,
      '',
      ...finalLines,
      '',
    ].join('\n')}`);
    records.push({ platform: 'android-emulator', scenario, name, tapPoint: point, swipeCoordinates, postTapState, finalState });
    afterTapLines = [];
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

  const manifest = {
    schema: 'spinon-c13-3-touch-action-tap-arbitration/v1',
    capturedAt: new Date().toISOString(),
    sourceCommit: command('git', ['rev-parse', 'HEAD']).trim(),
    worktreeDiffSha256: sha256(command('git', ['diff', '--binary', 'HEAD'], { encoding: 'buffer' })),
    sourceSha256: {
      androidGestureDemo: sha256(await readFile(path.join(root,
        'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'))),
      androidActivity: sha256(await readFile(path.join(root,
        'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'))),
      iosGestureDemo: sha256(await readFile(path.join(root, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'))),
      iosAppDelegate: sha256(await readFile(path.join(root, 'platforms/ios/Sources/AppDelegate.swift'))),
    },
    results: [],
    builds: {},
  };
  if (iosUDID) {
    manifest.builds.iosAppTreeSha256 = await treeSha256(simulatorAppPath);
    const ios = await runIOS(iosUDID);
    manifest.devices = { ...manifest.devices, iosSimulator: ios.device };
    manifest.results.push(...ios.records);
  }
  if (androidSerial) {
    const apkSha256 = sha256(await readFile(apkPath));
    manifest.builds.androidApkSha256 = apkSha256;
    const android = await runAndroid(androidSerial, apkSha256);
    manifest.devices = { ...manifest.devices, androidEmulator: android.device };
    manifest.results.push(...android.records);
  }
  const output = path.join(evidenceDirectory, 'run.json');
  await writeFile(output, `${JSON.stringify(manifest, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({
    evidence: path.relative(root, output),
    sha256: sha256(await readFile(output)),
    results: manifest.results.map(({ platform, scenario, name, postTapState, finalState }) => ({
      platform, scenario, name, postTapState, finalState,
    })),
  }, null, 2)}\n`);
}

main().catch((error) => {
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
