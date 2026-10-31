import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
const deviceID = process.argv[3];
const packageName = 'dev.spinon.bootstrap';
const androidApk = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosApp = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const evidenceRoot = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-anonymous-grid-text-computed-font-size-runtime-2026-10-31');
const expectedValues = JSON.parse(await readFile(resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-anonymous-grid-text-computed-font-size-precomparison-2026-10-31/expected-values.json'),
{ encoding: 'utf8' }));
const runPrefix = `c114-computed-font-size-${target}-${Date.now()}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(evidenceRoot, target);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} failed: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function evidenceLog(log) {
  return `${log.replace(/[ \t]+$/gm, '').replace(/(?:\r?\n)*$/, '')}\n`;
}

function parseTrace(log, runID, sceneID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  const parts = log.split(/\r?\n/).flatMap((line) => {
    const markerOffset = line.indexOf(marker);
    if (markerOffset < 0) return [];
    const match = line.slice(markerOffset + marker.length)
      .match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
  const total = parts[0]?.total ?? 0;
  const unique = new Map(parts.map((part) => [part.index, part]));
  assert.ok(total > 0 && parts.length === total && unique.size === total
    && [...unique.values()].every((part) => part.total === total
      && part.index >= 1 && part.index <= total),
  `${sceneID}: incomplete trace chunks (${unique.size}/${total})`);
  const payload = [...unique.values()].sort((a, b) => a.index - b.index)
    .map((part) => part.payload).join('');
  const markerOffset = payload.indexOf('grid_track_sizing_diagnostics=');
  assert.ok(markerOffset >= 0, `${sceneID}: trace JSON marker missing`);
  // Apple unified log prints JSON backslashes as the octal escape \134.
  const json = payload.slice(markerOffset + 'grid_track_sizing_diagnostics='.length)
    .replaceAll('\\134', '\\');
  return {
    report: JSON.parse(json),
    partCount: unique.size,
  };
}

function validateScene(log, scene, runID) {
  const trackWidth = scene.expectedTrackPx;
  const rowHeight = scene.expectedRowPx;
  assert.ok(log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `),
    `${scene.id}: summary missing`);
  assert.ok(log.includes(`expected_root_frame=width=20,height=${rowHeight}`),
    `${scene.id}: root frame mismatch`);
  assert.ok(log.includes('status=0 layout=ready boxes=1 generation=1 document_revision=7'),
    `${scene.id}: runtime status/box count mismatch`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
    && !log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: runtime assertion failed`);

  const { report, partCount } = parseTrace(log, runID, scene.id);
  const revision = report.layoutInputRevision;
  assert.ok(revision?.source?.startsWith('HostDocument')
    && ['style', 'environment', 'intrinsicSize'].every((key) =>
      Number.isFinite(revision[key]) && revision[key] >= 0),
  `${scene.id}: invalid layout input revision`);
  const itemFrame = report.gridItemFrames?.find(({ node }) => node === 2);
  assert.ok(itemFrame, `${scene.id}: Grid item frame missing`);
  assert.deepEqual(
    { x: itemFrame.x, y: itemFrame.y, width: itemFrame.width, height: itemFrame.height },
    { x: 0, y: 0, width: trackWidth, height: rowHeight },
    `${scene.id}: Grid item frame`,
  );
  const layout = report.gridItemTextLayouts?.find(({ node }) => node === 2);
  assert.ok(layout, `${scene.id}: text layout missing`);
  assert.equal(layout.text, 'XX XX');
  assert.equal(layout.width, trackWidth, `${scene.id}: text layout width`);
  assert.equal(layout.height, rowHeight, `${scene.id}: text layout height`);
  assert.equal(layout.paragraphStyle.fontFamily, 'Ahem');
  assert.equal(layout.paragraphStyle.fontSize, scene.fontSizePx);
  assert.equal(layout.paragraphStyle.lineHeight, 10);
  assert.equal(layout.lines.length, scene.lineCount, `${scene.id}: line count`);
  for (const line of layout.lines) {
    assert.equal(line.lineHeight, 10, `${scene.id}: used line-height`);
    assert.ok(Number.isFinite(line.baseline) && Number.isFinite(line.lineTop),
      `${scene.id}: non-finite line geometry`);
  }
  const source = report.gridItemIntrinsicInputs?.find(({ node }) => node === 2)?.source;
  assert.ok(source, `${scene.id}: intrinsic contribution missing`);
  assert.deepEqual({
    minContent: source.minContent,
    minContentHeight: source.minContentHeight,
    maxContent: source.maxContent,
    maxContentHeight: source.maxContentHeight,
  }, {
    minContent: 2 * scene.fontSizePx,
    minContentHeight: 20,
    maxContent: 5 * scene.fontSizePx,
    maxContentHeight: 10,
  }, `${scene.id}: intrinsic min/max contribution`);
  const passes = report.trackSizingTraces?.find(({ grid }) => grid === 1)?.passes;
  const finalTrackSizes = (axis) => passes?.find((pass) => pass.axis === axis)?.stages?.at(-1)
    ?.tracks?.filter((track) => !track.gutter && !track.collapsed).map((track) => track.baseSize);
  assert.deepEqual(finalTrackSizes('Columns'), [trackWidth], `${scene.id}: final intrinsic column`);
  assert.deepEqual(finalTrackSizes('Rows'), [rowHeight], `${scene.id}: final auto row track`);
  const expectedAdvances = scene.lineCount === 2
    ? [2 * scene.fontSizePx, 2 * scene.fontSizePx] : [5 * scene.fontSizePx];
  assert.deepEqual(layout.lines.map(({ advance }) => advance), expectedAdvances,
    `${scene.id}: line advances`);
  return { scene: scene.id, result: 'PASS', fontSizePx: scene.fontSizePx,
    track: scene.track, lineCount: scene.lineCount, gridColumnPx: trackWidth,
    gridRowPx: rowHeight, traceParts: partCount };
}

function androidDevice(serial) {
  const adb = (args, options) => command('/opt/homebrew/bin/adb', ['-s', serial, ...args], options);
  const listed = command('/opt/homebrew/bin/adb', ['devices', '-l']).stdout
    .split(/\r?\n/).filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(listed.length, 1, `Android target is not connected: ${serial}`);
  const metadata = {
    target: 'Android API 37 ARM64 AVD', serial,
    qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(),
    apiLevel: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
    abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
    model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
  };
  assert.equal(metadata.qemu, '1');
  assert.equal(metadata.apiLevel, 37);
  assert.equal(metadata.abi, 'arm64-v8a');
  return { adb, metadata };
}

function androidLog(adb, pid) {
  return adb(['logcat', '-d', '-v', 'threadtime', `--pid=${pid}`,
    'SpinonBootstrap:V', '*:S'], { timeout: 30_000 }).stdout;
}

function iosLog(udid, runID) {
  const predicate = `process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "${runID}"`;
  return command('xcrun', ['simctl', 'spawn', udid, 'log', 'show', '--last', '2m',
    '--style', 'compact', '--predicate', predicate], { timeout: 45_000 }).stdout;
}

async function runAndroid(serial) {
  const { adb, metadata } = androidDevice(serial);
  const apk = await readFile(androidApk);
  const apkHash = sha256(apk);
  adb(['install', '-r', androidApk]);
  const installedPath = adb(['shell', 'pm', 'path', packageName]).stdout
    .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim())
    .find((path) => path.endsWith('/base.apk'));
  assert.ok(installedPath, 'Installed APK path missing');
  const installedHash = adb(['shell', 'sha256sum', installedPath]).stdout.trim().split(/\s+/)[0];
  assert.equal(installedHash, apkHash, 'Installed APK hash differs from the built APK');
  metadata.apkSha256 = apkHash;
  metadata.installedApkSha256 = installedHash;
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);

  const rows = [];
  try {
    for (const [index, item] of expectedValues.cases.entries()) {
      const scene = item;
      const { id } = scene;
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      adb(['shell', 'am', 'start', '-S', '-W', '-n', `${packageName}/.MainActivity`,
        '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', id, '--es', 'spinon_c114_grid_run_id', runID]);
      const pid = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim().split(/\s+/)[0];
      assert.ok(pid, `${id}: Android process missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      while (Date.now() < deadline) {
        log = androidLog(adb, pid);
        if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${id} `)) break;
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(300);
      }
      await writeFile(resolve(evidenceDirectory, `${id}.log`), evidenceLog(log));
      const row = validateScene(log, scene, runID);
      rows.push(row);
      if (id === 'font-size-20-max-content') {
        sleep(1500);
        const screenshot = command('/opt/homebrew/bin/adb', ['-s', serial, 'exec-out', 'screencap', '-p'],
          { encoding: null, maxBuffer: 16 * 1024 * 1024 });
        await writeFile(resolve(evidenceDirectory, 'representative-screen.png'), screenshot.stdout);
      }
      console.log(`android ${index + 1}/${expectedValues.cases.length}: ${id} PASS`);
    }
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    `scene\tresult\tfont_size_px\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\n`
      + rows.map(({ scene, result, fontSizePx, track, lineCount, gridColumnPx, gridRowPx, traceParts }) =>
        `${scene}\t${result}\t${fontSizePx}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}`).join('\n') + '\n');
}

async function runIOS(udid) {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid: candidate, state }) => candidate === udid && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator not found: ${udid}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  const modelIdentifier = command('xcrun', ['simctl', 'getenv', udid,
    'SIMULATOR_MODEL_IDENTIFIER']).stdout.trim();
  assert.equal(modelIdentifier, 'iPhone18,1');
  const executablePath = resolve(iosApp, 'SpinonBootstrap');
  const executableHash = sha256(await readFile(executablePath));
  command('xcrun', ['simctl', 'install', udid, iosApp]);
  const installedAppPath = command('xcrun', ['simctl', 'get_app_container', udid,
    packageName, 'app']).stdout.trim();
  const installedHash = sha256(await readFile(resolve(installedAppPath, 'SpinonBootstrap')));
  assert.equal(installedHash, executableHash, 'Installed executable hash differs from build');
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target: 'iPhone 17 Pro / iOS 26.2 Simulator', udid, name: simulator[0].name,
    modelIdentifier, runtime: simulator[0].runtime, executableSha256: executableHash,
    installedExecutableSha256: installedHash,
  }, null, 2)}\n`);

  const rows = [];
  try {
    for (const [index, item] of expectedValues.cases.entries()) {
      const scene = item;
      const { id } = scene;
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        udid, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-scene=${id}`, `--spinon-c114-grid-run-id=${runID}`]);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${id}: iOS process ID missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      while (Date.now() < deadline) {
        log = iosLog(udid, runID);
        if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${id} `)) break;
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(500);
      }
      await writeFile(resolve(evidenceDirectory, `${id}.log`), evidenceLog(log));
      const row = validateScene(log, scene, runID);
      rows.push(row);
      if (id === 'font-size-20-max-content') {
        sleep(1500);
        command('xcrun', ['simctl', 'io', udid, 'screenshot',
          resolve(evidenceDirectory, 'representative-screen.png')]);
      }
      console.log(`ios ${index + 1}/${expectedValues.cases.length}: ${id} PASS`);
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', udid, packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    `scene\tresult\tfont_size_px\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\n`
      + rows.map(({ scene, result, fontSizePx, track, lineCount, gridColumnPx, gridRowPx, traceParts }) =>
        `${scene}\t${result}\t${fontSizePx}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}`).join('\n') + '\n');
}

assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-computed-font-size-simulators.mjs <android-avd|ios-simulator> <serial|UDID>');
assert.ok(deviceID, 'An explicit Android serial or iOS Simulator UDID is required');
await mkdir(evidenceDirectory, { recursive: true });
if (target === 'android-avd') await runAndroid(deviceID);
else await runIOS(deviceID);
