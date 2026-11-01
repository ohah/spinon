import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
const deviceID = process.argv[3];
const variant = process.argv[4] ?? 'baseline';
assert.ok(['baseline', 'loose'].includes(variant),
  'The optional capture variant must be baseline or loose');
const packageName = 'dev.spinon.bootstrap';
const androidApk = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosApp = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const evidenceName = variant === 'loose'
  ? 'c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01'
  : 'c11-4-anonymous-grid-text-line-break-runtime-2026-11-01';
const evidenceRoot = resolve(repositoryRoot, 'spec/internal/evidence', evidenceName);
const expectedValues = JSON.parse(await readFile(resolve(evidenceRoot, 'expected-values.json'),
  { encoding: 'utf8' }));
const runPrefix = `c114-computed-line-break-${target}-${Date.now()}-${randomUUID().slice(0, 8)}`;
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

function deriveScene(scene) {
  const reference = expectedValues.reference;
  const fontSize = reference.cssFontSizePx;
  const glyphAdvance = reference.notoSansKrCjkAdvanceUnits
    / reference.notoSansKrUnitsPerEm * fontSize;
  const allowsPrefixBreak = scene.text === '文〜文'
    ? scene.language === 'ja' && scene.lineBreak === 'normal'
    : scene.language === 'ja' && scene.lineBreak === 'loose';
  const minContent = allowsPrefixBreak ? glyphAdvance : 2 * glyphAdvance;
  const maxContent = 3 * glyphAdvance;
  const minContentLineCount = allowsPrefixBreak ? 3 : 2;
  const minContentHeight = minContentLineCount * fontSize;
  const maxContentHeight = fontSize;
  assert.ok(Math.abs(scene.minContentCssPx - minContent) <= 1e-9,
    `${scene.id}: independent min-content formula`);
  assert.ok(Math.abs(scene.maxContentCssPx - maxContent) <= 1e-9,
    `${scene.id}: independent max-content formula`);
  assert.equal(scene.minContentHeightCssPx, minContentHeight,
    `${scene.id}: independent min-content row formula`);
  assert.equal(scene.maxContentHeightCssPx, maxContentHeight,
    `${scene.id}: independent max-content row formula`);
  const formulaTrackWidth = scene.track === 'min'
    ? scene.minContentCssPx : scene.maxContentCssPx;
  assert.ok(Math.abs(scene.columnsPx - formulaTrackWidth)
    <= reference.serializationQuantumCssPx / 2 + 1e-9,
  `${scene.id}: independent column formula/serialization quantum`);
  assert.equal(scene.rowsPx, scene.track === 'min' ? scene.minContentHeightCssPx : scene.maxContentHeightCssPx,
    `${scene.id}: independent row formula`);
  const lineCount = scene.track === 'min' ? minContentLineCount : 1;
  assert.equal(scene.lineCount, lineCount, `${scene.id}: independent line count`);
  const trackWidth = scene.columnsPx;
  return {
    minContent: scene.minContentCssPx,
    minContentHeight,
    maxContent: scene.maxContentCssPx,
    maxContentHeight,
    trackWidth,
    rowHeight: scene.rowsPx,
    lineCount,
    layoutText: scene.text,
    fontSize,
  };
}

function validateScene(log, scene, runID) {
  const expected = deriveScene(scene);
  const { trackWidth, rowHeight } = expected;
  const tolerance = expectedValues.reference.runtimeToleranceCssPx;
  const near = (actual, value, label) => assert.ok(Number.isFinite(actual)
    && Math.abs(actual - value) <= tolerance,
  `${scene.id}: ${label} ${actual} differs from ${value}`);
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `));
  assert.ok(summary, `${scene.id}: summary missing`);
  assert.ok(summary.includes(`expected_root_frame=width=120,height=${rowHeight}`),
    `${scene.id}: root frame mismatch`);
  const ready = summary.match(/status=0 layout=ready boxes=1 generation=(\d+) document_revision=(\d+)/);
  assert.ok(ready, `${scene.id}: runtime status/box count mismatch`);
  const generation = Number(ready[1]);
  const documentRevision = Number(ready[2]);
  assert.ok(generation > 0 && documentRevision > 0,
    `${scene.id}: runtime status/box count mismatch`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
    && !log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: runtime assertion failed`);

  const { report, partCount } = parseTrace(log, runID, scene.id);
  const revision = report.layoutInputRevision;
  assert.ok(revision?.source?.startsWith('HostDocument')
    && revision.source.includes(`DocumentGeneration(${generation})`)
    && revision.source.includes(`DocumentRevision(${documentRevision})`)
    && ['style', 'environment', 'intrinsicSize'].every((key) =>
      Number.isFinite(revision[key]) && revision[key] >= 0),
  `${scene.id}: invalid layout input revision`);
  const itemFrame = report.gridItemFrames?.find(({ node }) => node === 2);
  assert.ok(itemFrame, `${scene.id}: Grid item frame missing`);
  near(itemFrame.x, 0, 'Grid item x');
  near(itemFrame.y, 0, 'Grid item y');
  near(itemFrame.width, trackWidth, 'Grid item width');
  near(itemFrame.height, rowHeight, 'Grid item height');
  const layout = report.gridItemTextLayouts?.find(({ node }) => node === 2);
  assert.ok(layout, `${scene.id}: text layout missing`);
  assert.equal(layout.text, expected.layoutText);
  near(layout.width, trackWidth, 'text layout width');
  near(layout.height, rowHeight, 'text layout height');
  assert.equal(layout.paragraphStyle.fontFamily, scene.fontFamily);
  assert.equal(layout.paragraphStyle.locale, scene.language);
  assert.equal(layout.paragraphStyle.wordBreak, 'Normal');
  assert.equal(layout.paragraphStyle.lineBreak, scene.runtimeLineBreak);
  assert.equal(layout.paragraphStyle.fontSize, expected.fontSize);
  assert.equal(layout.paragraphStyle.lineHeight, expected.fontSize);
  assert.equal(layout.lines.length, expected.lineCount, `${scene.id}: line count`);
  for (const line of layout.lines) {
    assert.equal(line.lineHeight, expected.fontSize,
      `${scene.id}: used line-height`);
    assert.ok(Number.isFinite(line.baseline) && Number.isFinite(line.lineTop),
      `${scene.id}: non-finite line geometry`);
    assert.ok(Number.isFinite(line.advance) && line.advance <= trackWidth + tolerance,
      `${scene.id}: line advance exceeds the selected track`);
  }
  const source = report.gridItemIntrinsicInputs?.find(({ node }) => node === 2)?.source;
  assert.ok(source, `${scene.id}: intrinsic contribution missing`);
  near(source.minContent, expected.minContent, 'intrinsic min-content');
  near(source.minContentHeight, expected.minContentHeight, 'intrinsic min-content height');
  near(source.maxContent, expected.maxContent, 'intrinsic max-content');
  near(source.maxContentHeight, expected.maxContentHeight, 'intrinsic max-content height');
  const passes = report.trackSizingTraces?.find(({ grid }) => grid === 1)?.passes;
  const finalTrackSizes = (axis) => passes?.find((pass) => pass.axis === axis)?.stages?.at(-1)
    ?.tracks?.filter((track) => !track.gutter && !track.collapsed).map((track) => track.baseSize);
  near(finalTrackSizes('Columns')?.[0], trackWidth, 'final intrinsic column');
  near(finalTrackSizes('Rows')?.[0], rowHeight, 'final auto row track');
  assert.equal(finalTrackSizes('Columns')?.length, 1, `${scene.id}: final column track count`);
  assert.equal(finalTrackSizes('Rows')?.length, 1, `${scene.id}: final row track count`);
  near(layout.lines.reduce((sum, line) => sum + line.advance, 0), expected.maxContent,
    'sum of line advances');
  return { scene: scene.id, result: 'PASS', lineBreak: scene.lineBreak,
    language: scene.language, fontFamily: scene.fontFamily,
    track: scene.track, lineCount: expected.lineCount, gridColumnPx: trackWidth,
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
      if (id === 'line-break-ja-normal-min-content') {
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
    `scene\tresult\tline_break\tlanguage\tfont_family\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\n`
      + rows.map(({ scene, result, lineBreak, language, fontFamily, track, lineCount,
        gridColumnPx, gridRowPx, traceParts }) =>
        `${scene}\t${result}\t${lineBreak}\t${language}\t${fontFamily}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}`).join('\n') + '\n');
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
  const attempts = [];
  const writeAttempts = () => writeFile(resolve(evidenceDirectory, 'attempts.tsv'),
    `scene\tattempt\trun_id\tresult\tlog\n`
      + attempts.map(({ scene, attempt, runID, result, log }) =>
        `${scene}\t${attempt}\t${runID}\t${result}\t${log}`).join('\n') + '\n');
  try {
    for (const [index, item] of expectedValues.cases.entries()) {
      const scene = item;
      const { id } = scene;
      let row;
      for (let attempt = 1; attempt <= 2 && !row; attempt += 1) {
        const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}-a${attempt}`;
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
        const superseded = log.split(/\r?\n/).some((line) =>
          line.includes(`SPINON_C114_GRID_EVAL_RUN run_id=${runID} `)
            && /\bstatus=-12\b/.test(line));
        if (superseded && attempt === 1) {
          const supersededLog = `${id}.superseded-attempt-${attempt}.log`;
          await writeFile(resolve(evidenceDirectory, supersededLog), evidenceLog(log));
          attempts.push({ scene: id, attempt, runID, result: 'superseded-retry', log: supersededLog });
          await writeAttempts();
          continue;
        }
        await writeFile(resolve(evidenceDirectory, `${id}.log`), evidenceLog(log));
        row = validateScene(log, scene, runID);
        rows.push(row);
        attempts.push({ scene: id, attempt, runID, result: 'passed', log: `${id}.log` });
        await writeAttempts();
      }
      assert.ok(row, `${id}: iOS scene remained superseded after one retry`);
      if (id === 'line-break-ja-normal-min-content') {
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
    `scene\tresult\tline_break\tlanguage\tfont_family\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\n`
      + rows.map(({ scene, result, lineBreak, language, fontFamily, track, lineCount,
        gridColumnPx, gridRowPx, traceParts }) =>
        `${scene}\t${result}\t${lineBreak}\t${language}\t${fontFamily}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}`).join('\n') + '\n');
}

assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-anonymous-grid-text-line-break-simulators.mjs <android-avd|ios-simulator> <serial|UDID> [baseline|loose]');
assert.ok(deviceID, 'An explicit Android serial or iOS Simulator UDID is required');
await mkdir(evidenceDirectory, { recursive: true });
if (target === 'android-avd') await runAndroid(deviceID);
else await runIOS(deviceID);
