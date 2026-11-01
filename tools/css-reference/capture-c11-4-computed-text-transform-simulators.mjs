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
const expectedPath = resolve(repositoryRoot, process.argv[4]
  ?? 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/runtime-matrix-expected.json');
const expectedValues = JSON.parse(await readFile(expectedPath, 'utf8'));
const scenes = expectedValues.scenes;
const runPrefix = `c114-computed-text-transform-${target}-${Date.now()}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot, process.argv[5]
  ?? `spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-runtime-2026-11-01/${runPrefix}`);
const tolerance = expectedValues.toleranceCssPx;
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

function near(actual, expected, label) {
  assert.ok(Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance,
    `${label}: expected ${expected} ±${tolerance}, got ${actual}`);
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
  const minContent = scene.minContentPx;
  const maxContent = scene.canvasTextMetrics?.widthPx ?? scene.maxContentPx;
  const track = scene.canvasTextMetrics?.widthPx ?? scene.columnsPx;
  const lineAdvances = scene.canvasTextMetrics?.lineAdvancesPx ?? scene.lineAdvancesPx;
  for (const [name, value] of Object.entries({
    minContent, maxContent, track, minContentHeight: scene.minContentHeightPx,
    maxContentHeight: scene.maxContentHeightPx,
  })) {
    assert.ok(Number.isFinite(value) && value >= 0, `${scene.id}: invalid ${name}`);
  }
  assert.ok(Array.isArray(lineAdvances) && lineAdvances.length > 0,
    `${scene.id}: missing expected line advances`);
  if (scene.canvasTextMetrics) {
    const metric = scene.canvasTextMetrics;
    assert.equal(metric.fontFamily, expectedValues.font.family, `${scene.id}: Canvas font family`);
    assert.equal(metric.fontWeight, 400, `${scene.id}: Canvas font weight`);
    assert.equal(metric.fontSizeCssPx, expectedValues.font.fontSizeCssPx,
      `${scene.id}: Canvas font size`);
    assert.equal(metric.language, expectedValues.font.language, `${scene.id}: Canvas language`);
    assert.ok(Array.isArray(metric.lineAdvancesPx)
      && metric.lineAdvancesPx.length === scene.lineAdvancesPx.length,
    `${scene.id}: Canvas line advances`);
    near(scene.columnsPx, metric.widthPx, `${scene.id}: CSSOM track vs unquantized Canvas width`);
    for (const [index, advance] of metric.lineAdvancesPx.entries()) {
      near(scene.lineAdvancesPx[index], advance,
        `${scene.id}: CSSOM line advance vs unquantized Canvas width`);
    }
  }
  near(track, scene.track === 'min-content' ? minContent : maxContent,
    `${scene.id}: selected track`);
  assert.equal(scene.rowsPx, scene.track === 'min-content'
    ? scene.minContentHeightPx : scene.maxContentHeightPx,
    `${scene.id}: auto row height`);
  const expectedTextTransform = {
    none: 'None', uppercase: 'Uppercase', lowercase: 'Lowercase', capitalize: 'Capitalize',
  }[scene.textTransform];
  assert.ok(expectedTextTransform, `${scene.id}: unsupported transform ${scene.textTransform}`);
  return {
    track, rowHeight: scene.rowsPx, minContent, maxContent,
    minContentHeight: scene.minContentHeightPx,
    maxContentHeight: scene.maxContentHeightPx,
    expectedTextTransform, lineAdvances,
    layoutWidth: scene.track === 'min-content' ? minContent : maxContent,
    contribution: scene.track === 'min-content' ? minContent : maxContent,
  };
}

function runtimeSceneID(scene) {
  if (scene.runtimeSceneId) return scene.runtimeSceneId;
  const track = scene.track === 'min-content' ? 'min-content' : 'max-content';
  return `text-transform-${scene.textTransform}-${track}`;
}

function validateScene(log, scene, runID) {
  const expected = deriveScene(scene);
  assert.ok(log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `),
    `${scene.id}: summary missing`);
  assert.ok(log.includes(`expected_root_frame=width=120,height=${expected.rowHeight}`),
    `${scene.id}: root frame mismatch`);
  assert.ok(/status=0 layout=ready boxes=1 generation=1 document_revision=\d+/.test(log),
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
  near(itemFrame.x, 0, `${scene.id}: item x`);
  near(itemFrame.y, 0, `${scene.id}: item y`);
  near(itemFrame.width, expected.track, `${scene.id}: item width`);
  near(itemFrame.height, expected.rowHeight, `${scene.id}: item height`);

  const layout = report.gridItemTextLayouts?.find(({ node }) => node === 2);
  assert.ok(layout, `${scene.id}: text layout missing`);
  assert.equal(layout.text, scene.shapedText, `${scene.id}: transformed layout text`);
  const expectedTextSources = scene.sourceTextNodes ?? [scene.sourceText];
  assert.deepEqual(layout.sources?.map(({ node, text }) => ({ node, text })),
    expectedTextSources.map((text, index) => ({ node: 2 + index, text })),
    `${scene.id}: text layout source fragments`);
  near(layout.availableWidth, expected.track, `${scene.id}: available text width`);
  near(layout.width, expected.layoutWidth, `${scene.id}: shaped text width`);
  near(layout.height, expected.rowHeight, `${scene.id}: text layout height`);
  const style = layout.paragraphStyle;
  assert.equal(style.fontFamily, expectedValues.font.family);
  assert.equal(style.locale, expectedValues.font.language);
  assert.equal(style.textTransform, expected.expectedTextTransform);
  assert.equal(style.fontSize, 10);
  assert.equal(style.lineHeight, 10);
  assert.equal(style.lineHeightIsNormal, false);
  assert.equal(style.wordSpacing, 0);
  assert.equal(style.letterSpacing, 0);
  assert.equal(style.textIndent, 0);
  assert.equal(layout.lines.length, expected.lineAdvances.length,
    `${scene.id}: line count`);
  for (const [index, line] of layout.lines.entries()) {
    near(line.advance, expected.lineAdvances[index], `${scene.id}: line ${index} advance`);
    near(line.lineHeight, 10, `${scene.id}: line ${index} height`);
    assert.ok(Number.isFinite(line.lineTop) && Number.isFinite(line.baseline),
      `${scene.id}: invalid line geometry`);
  }
  const input = report.gridItemIntrinsicInputs?.find(({ node }) => node === 2);
  assert.ok(input, `${scene.id}: intrinsic input missing`);
  assert.deepEqual(input.textSources?.map(({ node, text }) => ({ node, text })),
    expectedTextSources.map((text, index) => ({ node: 2 + index, text })),
    `${scene.id}: original source fragments must be preserved`);
  near(input.source.minContent, expected.minContent, `${scene.id}: min-content input`);
  near(input.source.maxContent, expected.maxContent, `${scene.id}: max-content input`);
  near(input.source.minContentHeight, expected.minContentHeight,
    `${scene.id}: min-content height input`);
  near(input.source.maxContentHeight, expected.maxContentHeight,
    `${scene.id}: max-content height input`);

  const passes = report.trackSizingTraces?.find(({ grid }) => grid === 1)?.passes;
  const columnPass = passes?.find(({ axis }) => axis === 'Columns');
  const rowPass = passes?.find(({ axis }) => axis === 'Rows');
  const finalTrackSizes = (pass) => pass?.stages?.at(-1)?.tracks
    ?.filter((track) => !track.gutter && !track.collapsed).map((track) => track.baseSize);
  const columns = finalTrackSizes(columnPass);
  const rows = finalTrackSizes(rowPass);
  assert.equal(columns?.length, 1, `${scene.id}: column track count`);
  assert.equal(rows?.length, 1, `${scene.id}: row track count`);
  near(columns[0], expected.track, `${scene.id}: final column track`);
  near(rows[0], expected.rowHeight, `${scene.id}: final row track`);
  const expectedContributionKind = scene.track === 'min-content' ? 'MinContent' : 'MaxContent';
  assert.ok(columnPass?.contributions?.some((contribution) =>
    contribution.kind === expectedContributionKind
      && Math.abs(contribution.value - expected.contribution) <= tolerance),
  `${scene.id}: expected ${expectedContributionKind} contribution missing`);
  return {
    scene: scene.id, result: 'PASS', textTransform: scene.textTransform,
    track: scene.track, lineCount: expected.lineAdvances.length,
    gridColumnPx: columns[0], gridRowPx: rows[0], traceParts: partCount,
  };
}

function androidDevice(serial) {
  const adb = (args, options) => command('/opt/homebrew/bin/adb', ['-s', serial, ...args], options);
  const listed = command('/opt/homebrew/bin/adb', ['devices', '-l']).stdout
    .split(/\r?\n/).filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(listed.length, 1, `Android target is not connected: ${serial}`);
  const getprop = (key) => adb(['shell', 'getprop', key]).stdout.trim();
  const metadata = {
    target: 'Android API 37 ARM64 AVD', serial,
    qemu: getprop('ro.kernel.qemu'),
    apiLevel: Number(getprop('ro.build.version.sdk')),
    abi: getprop('ro.product.cpu.abi'),
    model: getprop('ro.product.model'),
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
  metadata.sourceHead = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  await writeFile(resolve(evidenceDirectory, 'simulator.json'),
    `${JSON.stringify(metadata, null, 2)}\n`);

  const rows = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const sceneID = runtimeSceneID(scene);
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      const launch = adb(['shell', 'am', 'start', '-S', '-W', '-n', `${packageName}/.MainActivity`,
        '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', sceneID,
        '--es', 'spinon_c114_grid_run_id', runID]);
      let pid = '';
      const launchDeadline = Date.now() + 10_000;
      while (Date.now() < launchDeadline) {
        pid = adb(['shell', 'pidof', packageName], { allowFailure: true })
          .stdout.trim().split(/\s+/)[0] ?? '';
        if (pid) break;
        sleep(100);
      }
      assert.ok(pid, `${scene.id}: Android process missing: ${launch.stdout.trim()}`);
      let log = '';
      const deadline = Date.now() + 90_000;
      while (Date.now() < deadline) {
        log = androidLog(adb, pid);
        if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${sceneID} `)) break;
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(300);
      }
      await writeFile(resolve(evidenceDirectory, `${sceneID}.log`), evidenceLog(log));
      try {
        rows.push(validateScene(log, { ...scene, id: sceneID }, runID));
        console.log(`android ${index + 1}/${scenes.length}: ${sceneID} PASS`);
      } catch (error) {
        rows.push({ scene: sceneID, result: 'FAIL', textTransform: scene.textTransform,
          track: scene.track, lineCount: scene.lineAdvancesPx.length,
          gridColumnPx: '', gridRowPx: '', traceParts: '',
          error: String(error?.message ?? error).replace(/[\t\r\n]+/g, ' ') });
        console.error(`android ${index + 1}/${scenes.length}: ${sceneID} FAIL: ${error?.message ?? error}`);
      }
      if (index === scenes.length - 1) {
        sleep(1000);
        const screenshot = command('/opt/homebrew/bin/adb',
          ['-s', serial, 'exec-out', 'screencap', '-p'],
          { encoding: null, maxBuffer: 16 * 1024 * 1024 });
        await writeFile(resolve(evidenceDirectory, 'representative-screen.png'), screenshot.stdout);
      }
    }
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    'scene\tresult\ttext_transform\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\terror\n'
      + rows.map(({ scene, result, textTransform, track, lineCount,
        gridColumnPx, gridRowPx, traceParts, error = '' }) =>
        `${scene}\t${result}\t${textTransform}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}\t${error || 'none'}`
      ).join('\n') + '\n');
  const failures = rows.filter(({ result }) => result !== 'PASS');
  assert.equal(failures.length, 0,
    `Android simulator matrix failed ${failures.length}/${scenes.length}: ${failures.map(({ scene, error }) => `${scene}: ${error}`).join('; ')}`);
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
    sourceHead: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
  }, null, 2)}\n`);

  const rows = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const sceneID = runtimeSceneID(scene);
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        udid, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-scene=${sceneID}`, `--spinon-c114-grid-run-id=${runID}`]);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: iOS process ID missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      while (Date.now() < deadline) {
        log = iosLog(udid, runID);
        if (log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${sceneID} `)) break;
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(500);
      }
      await writeFile(resolve(evidenceDirectory, `${sceneID}.log`), evidenceLog(log));
      try {
        rows.push(validateScene(log, { ...scene, id: sceneID }, runID));
        console.log(`ios ${index + 1}/${scenes.length}: ${sceneID} PASS`);
      } catch (error) {
        rows.push({ scene: sceneID, result: 'FAIL', textTransform: scene.textTransform,
          track: scene.track, lineCount: scene.lineAdvancesPx.length,
          gridColumnPx: '', gridRowPx: '', traceParts: '',
          error: String(error?.message ?? error).replace(/[\t\r\n]+/g, ' ') });
        console.error(`ios ${index + 1}/${scenes.length}: ${sceneID} FAIL: ${error?.message ?? error}`);
      }
      if (index === scenes.length - 1) {
        sleep(1000);
        command('xcrun', ['simctl', 'io', udid, 'screenshot',
          resolve(evidenceDirectory, 'representative-screen.png')]);
      }
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', udid, packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    'scene\tresult\ttext_transform\ttrack\tline_count\tgrid_column_px\tgrid_row_px\ttrace_parts\terror\n'
      + rows.map(({ scene, result, textTransform, track, lineCount,
        gridColumnPx, gridRowPx, traceParts, error = '' }) =>
        `${scene}\t${result}\t${textTransform}\t${track}\t${lineCount}\t${gridColumnPx}\t${gridRowPx}\t${traceParts}\t${error || 'none'}`
      ).join('\n') + '\n');
  const failures = rows.filter(({ result }) => result !== 'PASS');
  assert.equal(failures.length, 0,
    `iOS Simulator matrix failed ${failures.length}/${scenes.length}: ${failures.map(({ scene, error }) => `${scene}: ${error}`).join('; ')}`);
}

assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-computed-text-transform-simulators.mjs <android-avd|ios-simulator> <serial|UDID> [expected.json] [evidence-directory]');
assert.ok(deviceID, 'An explicit Android serial or iOS Simulator UDID is required');
assert.equal(expectedValues.browser, 'HeadlessChrome 156.0.0.0 via Agent Browser');
  assert.equal(scenes.length, 23);
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length);
await mkdir(evidenceDirectory, { recursive: true });
if (target === 'android-avd') await runAndroid(deviceID);
else await runIOS(deviceID);
