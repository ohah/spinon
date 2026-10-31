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
const expectedValues = JSON.parse(await readFile(resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/expected.json'),
{ encoding: 'utf8' }));
const evidenceRoot = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01');
const evidenceDirectory = resolve(evidenceRoot, target);
const runPrefix = `c114-overflow-wrap-${target}-${Date.now()}-${randomUUID().slice(0, 8)}`;
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
  const json = payload.slice(markerOffset + 'grid_track_sizing_diagnostics='.length)
    .replaceAll('\\134', '\\');
  return { report: JSON.parse(json), partCount: unique.size };
}

function deriveScene(scene) {
  const expected = expectedValues.oracle.independentExpectedMetrics[scene.overflowWrap];
  assert.ok(expected, `${scene.sceneId}: missing independent CSS expectation`);
  const fixedTrack = scene.track === '10px';
  const trackWidth = scene.track === 'min-content'
    ? expected.minContentInlineCssPx
    : scene.track === 'max-content' ? expected.maxContentInlineCssPx : 10;
  const rowHeight = scene.track === 'min-content'
    ? expected.minContentAutoRowCssPx
    : scene.track === 'max-content'
      ? expected.maxContentAutoRowCssPx
      : scene.overflowWrap === 'normal' ? 10 : 40;
  const lineCount = scene.track === 'min-content'
    ? rowHeight / 10
    : scene.track === 'max-content' || scene.overflowWrap === 'normal' ? 1 : 4;
  const lineAdvances = fixedTrack && scene.overflowWrap === 'normal'
    ? [40] : Array(lineCount).fill(trackWidth);
  assert.deepEqual(scene.columnsCssPx, [trackWidth], `${scene.sceneId}: derived track width`);
  assert.deepEqual(scene.rowsCssPx, [rowHeight], `${scene.sceneId}: derived auto row`);
  assert.equal(scene.textLineCount, lineCount, `${scene.sceneId}: derived line count`);
  return {
    trackWidth,
    rowHeight,
    lineCount,
    lineAdvances,
    textWidth: Math.max(trackWidth, ...lineAdvances),
    minContent: expected.minContentInlineCssPx,
    maxContent: expected.maxContentInlineCssPx,
    minContentHeight: expected.minContentAutoRowCssPx,
    maxContentHeight: expected.maxContentAutoRowCssPx,
  };
}

function validateScene(log, scene, runID) {
  const expected = deriveScene(scene);
  const tolerance = 1 / 64;
  const near = (actual, value, label) => assert.ok(Number.isFinite(actual)
    && Math.abs(actual - value) <= tolerance,
  `${scene.sceneId}: ${label} ${actual} differs from ${value}`);
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.sceneId} `));
  assert.ok(summary, `${scene.sceneId}: summary missing`);
  assert.ok(summary.includes(`expected_root_frame=width=120,height=${expected.rowHeight}`),
    `${scene.sceneId}: root frame mismatch`);
  const ready = summary.match(/status=0 layout=ready boxes=1 generation=(\d+) document_revision=(\d+)/);
  assert.ok(ready, `${scene.sceneId}: runtime status/box count mismatch`);
  const generation = Number(ready[1]);
  const documentRevision = Number(ready[2]);
  assert.ok(generation > 0 && documentRevision > 0,
    `${scene.sceneId}: invalid runtime generation or revision`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
    && !log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'),
  `${scene.sceneId}: runtime assertion failed`);

  const { report, partCount } = parseTrace(log, runID, scene.sceneId);
  const revision = report.layoutInputRevision;
  assert.ok(revision?.source?.startsWith('HostDocument')
    && revision.source.includes(`DocumentGeneration(${generation})`)
    && revision.source.includes(`DocumentRevision(${documentRevision})`)
    && ['style', 'environment', 'intrinsicSize'].every((key) =>
      Number.isFinite(revision[key]) && revision[key] >= 0),
  `${scene.sceneId}: invalid layout input revision`);
  const itemFrame = report.gridItemFrames?.find(({ node }) => node === 2);
  assert.ok(itemFrame, `${scene.sceneId}: Grid item frame missing`);
  near(itemFrame.x, 0, 'Grid item x');
  near(itemFrame.y, 0, 'Grid item y');
  near(itemFrame.width, expected.trackWidth, 'Grid item width');
  near(itemFrame.height, expected.rowHeight, 'Grid item height');

  const layout = report.gridItemTextLayouts?.find(({ node }) => node === 2);
  assert.ok(layout, `${scene.sceneId}: text layout missing`);
  assert.equal(layout.text, expectedValues.oracle.text);
  near(layout.width, expected.textWidth, 'text layout width');
  near(layout.height, expected.rowHeight, 'text layout height');
  assert.equal(layout.paragraphStyle.fontFamily, 'Ahem');
  assert.equal(layout.paragraphStyle.locale, null);
  assert.equal(layout.paragraphStyle.wordBreak, 'Normal');
  assert.equal(layout.paragraphStyle.overflowWrap,
    scene.overflowWrap === 'break-word'
      ? 'BreakWord' : scene.overflowWrap === 'anywhere' ? 'Anywhere' : 'Normal');
  assert.equal(layout.lines.length, expected.lineCount, `${scene.sceneId}: line count`);
  for (const [index, line] of layout.lines.entries()) {
    near(line.advance, expected.lineAdvances[index], `line ${index + 1} advance`);
    near(line.lineTop, index * 10, `line ${index + 1} top`);
    near(line.lineHeight, 10, `line ${index + 1} height`);
    assert.ok(Number.isFinite(line.baseline), `${scene.sceneId}: non-finite baseline`);
  }

  const source = report.gridItemIntrinsicInputs?.find(({ node }) => node === 2)?.source;
  assert.ok(source, `${scene.sceneId}: intrinsic contribution missing`);
  near(source.minContent, expected.minContent, 'min-content inline size');
  near(source.maxContent, expected.maxContent, 'max-content inline size');
  near(source.minContentHeight, expected.minContentHeight, 'min-content block size');
  near(source.maxContentHeight, expected.maxContentHeight, 'max-content block size');
  const passes = report.trackSizingTraces?.find(({ grid }) => grid === 1)?.passes;
  const finalTrackSizes = (axis) => passes?.find((pass) => pass.axis === axis)?.stages?.at(-1)
    ?.tracks?.filter((track) => !track.gutter && !track.collapsed).map((track) => track.baseSize);
  near(finalTrackSizes('Columns')?.[0], expected.trackWidth, 'final column track');
  near(finalTrackSizes('Rows')?.[0], expected.rowHeight, 'final auto row track');
  assert.equal(finalTrackSizes('Columns')?.length, 1, `${scene.sceneId}: column track count`);
  assert.equal(finalTrackSizes('Rows')?.length, 1, `${scene.sceneId}: row track count`);
  return {
    scene: scene.sceneId,
    result: 'PASS',
    overflowWrap: scene.overflowWrap,
    track: scene.track,
    lineCount: expected.lineCount,
    gridColumnPx: expected.trackWidth,
    gridRowPx: expected.rowHeight,
    traceParts: partCount,
  };
}

function androidDevice(serial) {
  const adb = (args, options) => command('/opt/homebrew/bin/adb', ['-s', serial, ...args], options);
  const listed = command('/opt/homebrew/bin/adb', ['devices', '-l']).stdout
    .split(/\r?\n/).filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(listed.length, 1, `Android target is not connected: ${serial}`);
  const metadata = {
    target: 'Android API 37 ARM64 AVD',
    serial,
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

function writeMatrix(rows) {
  return writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    'scene\tresult\toverflow_wrap\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\n'
      + rows.map(({ scene, result, overflowWrap, track, lineCount, gridColumnPx, gridRowPx, traceParts }) =>
        `${scene}\t${result}\t${overflowWrap}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}`).join('\n')
      + '\n');
}

async function runAndroid(serial) {
  const { adb, metadata } = androidDevice(serial);
  const apkHash = sha256(await readFile(androidApk));
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
    for (const [index, scene] of expectedValues.candidateScenes.entries()) {
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      adb(['shell', 'am', 'start', '-S', '-W', '-n', `${packageName}/.MainActivity`,
        '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.sceneId,
        '--es', 'spinon_c114_grid_run_id', runID]);
      const pid = adb(['shell', 'pidof', packageName], { allowFailure: true })
        .stdout.trim().split(/\s+/)[0];
      assert.ok(pid, `${scene.sceneId}: Android process missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      while (Date.now() < deadline) {
        log = androidLog(adb, pid);
        if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.sceneId} `)) break;
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(300);
      }
      await writeFile(resolve(evidenceDirectory, `${scene.sceneId}.log`), evidenceLog(log));
      rows.push(validateScene(log, scene, runID));
      if (scene.sceneId === 'overflow-wrap-normal-fixed-10px'
          || scene.sceneId === 'overflow-wrap-break-word-fixed-10px') {
        sleep(1500);
        const screenshot = command('/opt/homebrew/bin/adb', ['-s', serial, 'exec-out', 'screencap', '-p'],
          { encoding: null, maxBuffer: 16 * 1024 * 1024 });
        await writeFile(resolve(evidenceDirectory, `${scene.sceneId}-screen.png`), screenshot.stdout);
      }
      console.log(`android ${index + 1}/${expectedValues.candidateScenes.length}: ${scene.sceneId} PASS`);
    }
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
  await writeMatrix(rows);
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
    target: 'iPhone 17 Pro / iOS 26.2 Simulator',
    udid,
    name: simulator[0].name,
    modelIdentifier,
    runtime: simulator[0].runtime,
    executableSha256: executableHash,
    installedExecutableSha256: installedHash,
  }, null, 2)}\n`);

  const rows = [];
  const attempts = [];
  const writeAttempts = () => writeFile(resolve(evidenceDirectory, 'attempts.tsv'),
    'scene\tattempt\trun_id\tresult\tlog\n'
      + attempts.map(({ scene, attempt, runID, result, log }) =>
        `${scene}\t${attempt}\t${runID}\t${result}\t${log}`).join('\n') + '\n');
  try {
    for (const [index, scene] of expectedValues.candidateScenes.entries()) {
      let row;
      for (let attempt = 1; attempt <= 2 && !row; attempt += 1) {
        const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}-a${attempt}`;
        const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
          udid, packageName, '--spinon-c114-grid-track-sizing',
          `--spinon-c114-grid-scene=${scene.sceneId}`, `--spinon-c114-grid-run-id=${runID}`]);
        const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
        assert.ok(Number.isInteger(pid) && pid > 0, `${scene.sceneId}: iOS process ID missing`);
        let log = '';
        const deadline = Date.now() + 90_000;
        while (Date.now() < deadline) {
          log = iosLog(udid, runID);
          if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.sceneId} `)) break;
          if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
              || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
          sleep(500);
        }
        const superseded = log.split(/\r?\n/).some((line) =>
          line.includes(`SPINON_C114_GRID_EVAL_RUN run_id=${runID} `)
            && /\bstatus=-12\b/.test(line));
        if (superseded && attempt === 1) {
          const supersededLog = `${scene.sceneId}.superseded-attempt-${attempt}.log`;
          await writeFile(resolve(evidenceDirectory, supersededLog), evidenceLog(log));
          attempts.push({ scene: scene.sceneId, attempt, runID, result: 'superseded-retry', log: supersededLog });
          await writeAttempts();
          continue;
        }
        const logPath = `${scene.sceneId}.log`;
        await writeFile(resolve(evidenceDirectory, logPath), evidenceLog(log));
        row = validateScene(log, scene, runID);
        rows.push(row);
        attempts.push({ scene: scene.sceneId, attempt, runID, result: 'passed', log: logPath });
        await writeAttempts();
      }
      assert.ok(row, `${scene.sceneId}: iOS scene remained superseded after one retry`);
      if (scene.sceneId === 'overflow-wrap-normal-fixed-10px'
          || scene.sceneId === 'overflow-wrap-break-word-fixed-10px') {
        sleep(1500);
        command('xcrun', ['simctl', 'io', udid, 'screenshot',
          resolve(evidenceDirectory, `${scene.sceneId}-screen.png`)]);
      }
      console.log(`ios ${index + 1}/${expectedValues.candidateScenes.length}: ${scene.sceneId} PASS`);
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', udid, packageName], { allowFailure: true });
  }
  await writeMatrix(rows);
}

assert.equal(expectedValues.candidateScenes.length, 9, 'Expected exactly nine fixed scenes');
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-anonymous-grid-text-overflow-wrap-simulators.mjs <android-avd|ios-simulator> <serial|UDID>');
assert.ok(deviceID, 'An explicit Android serial or iOS Simulator UDID is required');
await mkdir(evidenceDirectory, { recursive: true });
if (target === 'android-avd') await runAndroid(deviceID);
else await runIOS(deviceID);
