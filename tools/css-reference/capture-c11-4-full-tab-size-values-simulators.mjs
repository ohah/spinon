import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2] ?? 'both';
const androidSerial = process.argv[3] ?? 'emulator-5554';
const simulatorUDID = process.argv[4] ?? 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
const bundleID = 'dev.spinon.bootstrap';
const androidAPK = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const iosApp = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const expectedPath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/expected-values.json');
const evidenceRoot = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-tab-size-full-values-runtime-2026-10-31');
const runPrefix = `c114-tab-values-${new Date().toISOString().replace(/[-:.TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const representativeScene = 'anonymous-pre-tab-value-number-three-and-quarter-after-two';
const expectedData = JSON.parse(await readFile(expectedPath, 'utf8'));
const allScenes = expectedData.values.flatMap((value) => value.cases
  .filter((testCase) => testCase.layout !== 'ignored-whitespace-only-anonymous-grid-text')
  .map((testCase) => {
    const fixedOverflow = testCase.layout === 'fixed-overflow';
    const contentWidth = Number(testCase.max_content_width_px);
    const frameWidth = Number(testCase.expected_frame_width_px);
    const lineAdvances = testCase.line_advances_px.map(Number);
    const height = lineAdvances.length * 10;
    return {
      id: `anonymous-pre-tab-value-${testCase.id}`,
      text: testCase.text,
      normalizedText: testCase.text.replaceAll('\t', '').replaceAll('\r\n', '\n'),
      lineAdvances,
      contentWidth,
      frameWidth,
      rootWidth: fixedOverflow ? 25 : 120,
      height,
      caseID: testCase.id.slice(value.id.length + 1),
      fixedOverflow,
      minContent: testCase.layout === 'min-content',
      runPrefix,
    };
  }));
const sceneFilter = process.env.SPINON_C114_SCENE_FILTER;
const scenes = sceneFilter ? allScenes.filter((scene) => scene.id === sceneFilter) : allScenes;

assert.equal(expectedData.value_count, 14);
assert.equal(allScenes.length, 140);
assert.ok(scenes.length > 0, `No scene matched SPINON_C114_SCENE_FILTER=${sceneFilter}`);
assert.ok(['android', 'ios', 'both'].includes(target), `Unknown target: ${target}`);

function command(binary, args, { allowFailure = false, timeout = 30_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 64 * 1024 * 1024,
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

function sleep(milliseconds) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
}

function runDirectory(platform) {
  return resolve(evidenceRoot, runPrefix, platform);
}

async function saveLog(directory, name, result) {
  await writeFile(resolve(directory, name), `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function traceParts(log, runID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  const parts = log.split(/\r?\n/).flatMap((line) => {
    const offset = line.indexOf(marker);
    if (offset < 0) return [];
    const match = line.slice(offset + marker.length)
      .match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
  const total = parts[0]?.total ?? 0;
  const unique = new Map(parts.map((part) => [part.index, part]));
  return {
    complete: total > 0 && unique.size === total
      && [...unique.values()].every((part) => part.total === total
        && part.index >= 1 && part.index <= total),
    total,
    parts: [...unique.values()].sort((a, b) => a.index - b.index),
  };
}

function diagnosticsFromLog(log, scene, runID, platform) {
  const traces = traceParts(log, runID);
  assert.ok(traces.complete,
    `${scene.id}: incomplete diagnostics chunks ${traces.parts.length}/${traces.total}`);
  let raw = traces.parts.map((part) => part.payload).join('');
  if (platform === 'ios-simulator') raw = raw.replaceAll('\\134', '\\');
  const marker = 'grid_track_sizing_diagnostics=';
  const offset = raw.indexOf(marker);
  assert.ok(offset >= 0, `${scene.id}: diagnostics JSON marker missing`);
  return { diagnostics: JSON.parse(raw.slice(offset + marker.length)), traceParts: traces.parts.length };
}

function near(actual, expected, label) {
  assert.ok(Number.isFinite(actual) && Math.abs(actual - expected) <= 1 / 64,
    `${label}: expected ${expected}, received ${actual}`);
}

function validateTrace(diagnostics, scene) {
  const inputs = diagnostics.gridItemIntrinsicInputs;
  const frames = diagnostics.gridItemFrames;
  const layouts = diagnostics.gridItemTextLayouts;
  assert.equal(inputs?.length, 1, `${scene.id}: intrinsic inputs`);
  assert.equal(frames?.length, 1, `${scene.id}: item frame count`);
  assert.equal(layouts?.length, 1, `${scene.id}: text layout count`);
  const input = inputs[0];
  assert.equal(input.grid, 1);
  assert.equal(input.node, 2);
  assert.equal(input.source?.kind, 'text');
  near(input.source.minContent, scene.contentWidth, `${scene.id}: min-content`);
  near(input.source.maxContent, scene.contentWidth, `${scene.id}: max-content`);
  near(input.source.minContentHeight, scene.height, `${scene.id}: min-content height`);
  near(input.source.maxContentHeight, scene.height, `${scene.id}: max-content height`);
  assert.equal(input.textSources?.length, 1);
  assert.equal(input.textSources[0].text, scene.text);

  const frame = frames[0];
  assert.equal(frame.node, 2);
  near(frame.x, 0, `${scene.id}: item x`);
  near(frame.y, 0, `${scene.id}: item y`);
  near(frame.width, scene.frameWidth, `${scene.id}: item width`);
  near(frame.height, scene.height, `${scene.id}: item height`);

  const layout = layouts[0];
  assert.equal(layout.text, scene.normalizedText);
  assert.ok(layout.paragraphStyle?.whiteSpace?.startsWith('PreWithTabSize('));
  assert.equal(layout.lines?.length, scene.lineAdvances.length);
  for (const [index, line] of layout.lines.entries()) {
    near(line.advance, scene.lineAdvances[index], `${scene.id}: line ${index} advance`);
  }

  const traces = diagnostics.trackSizingTraces;
  assert.equal(traces?.length, 1, `${scene.id}: grid track trace count`);
  const passes = traces[0].passes;
  assert.deepEqual(passes.map(({ axis }) => axis), ['Columns', 'Rows']);
  for (const pass of passes) {
    const isColumns = pass.axis === 'Columns';
    const expectedTrack = isColumns
      ? (scene.fixedOverflow ? 25 : scene.contentWidth) : scene.height;
    const finalTracks = pass.stages.at(-1).tracks
      .filter((track) => !track.gutter && !track.collapsed);
    assert.equal(finalTracks.length, 1, `${scene.id}: ${pass.axis} track count`);
    near(finalTracks[0].baseSize, expectedTrack, `${scene.id}: ${pass.axis} track`);
    const shouldContribute = !isColumns || !scene.fixedOverflow;
    if (!shouldContribute) {
      assert.equal(pass.contributions.length, 0, `${scene.id}: fixed track contribution`);
      continue;
    }
    const expectedValue = isColumns ? scene.contentWidth : scene.height;
    const expectedKind = scene.minContent ? 'MinContent' : 'MaxContent';
    const contribution = pass.contributions.find((item) => item.item === 2 && item.span === 1
      && Math.abs(item.value - expectedValue) <= 1 / 64
      && (!isColumns || item.kind === expectedKind));
    assert.ok(contribution, `${scene.id}: ${pass.axis} contribution missing`);
    near(contribution.value, expectedValue, `${scene.id}: ${pass.axis} contribution value`);
  }
}

function validateLog(log, scene, runID, platform) {
  assert.ok(log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `),
    `${scene.id}: runtime summary missing\n${log}`);
  assert.ok(log.includes(`expected_root_frame=width=${scene.rootWidth},height=${scene.height}`),
    `${scene.id}: root-frame expectation differs`);
  assert.ok(log.includes(`layout=ready boxes=1`), `${scene.id}: layout not ready`);
  assert.ok(log.includes('SPINON_C0410_DRAW') && log.includes('presented boxes=1'),
    `${scene.id}: GPU draw/present summary missing`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`),
    `${scene.id}: native assertion failed\n${log}`);
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'),
    `${scene.id}: Android/iOS route rejected scene\n${log}`);
  const { diagnostics, traceParts: count } = diagnosticsFromLog(log, scene, runID, platform);
  validateTrace(diagnostics, scene);
  return { id: scene.id, traceParts: count, lineAdvances: scene.lineAdvances,
    itemWidth: scene.frameWidth, rootWidth: scene.rootWidth, height: scene.height };
}

function androidLog(serial, pid) {
  return command('adb', ['-s', serial, 'logcat', '-d', '-v', 'threadtime', `--pid=${pid}`],
    { timeout: 15_000 }).stdout;
}

function waitForAndroid(serial, scene, runID) {
  const deadline = Date.now() + 60_000;
  let latest = '';
  let pid = '';
  while (Date.now() < deadline) {
    pid = command('adb', ['-s', serial, 'shell', 'pidof', bundleID], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0] ?? '';
    if (pid) latest = androidLog(serial, pid);
    if (latest.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `)
        && traceParts(latest, runID).complete
        && latest.includes('presented boxes=1')) return { pid, log: latest };
    if (latest.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || latest.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) return { pid, log: latest };
    sleep(200);
  }
  throw new Error(`${scene.id}: Android scene timed out\n${latest}`);
}

async function runAndroid() {
  const directory = runDirectory('android-avd');
  const runtimeDirectory = resolve(directory, 'runtime');
  await mkdir(runtimeDirectory, { recursive: true });
  const adb = (args, options) => command('adb', ['-s', androidSerial, ...args], options);
  const online = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .some((line) => line.startsWith(`${androidSerial} `) && line.includes(' device '));
  assert.ok(online, `Android target is not connected: ${androidSerial}`);
  await saveLog(directory, 'install.log', adb(['install', '-r', androidAPK]));
  const localApkSha256 = sha256(await readFile(androidAPK));
  const installedPath = adb(['shell', 'pm', 'path', bundleID]).stdout
    .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim())
    .find((path) => path.endsWith('/base.apk'));
  assert.ok(installedPath, 'Installed Android base.apk path missing');
  const installedApkSha256 = adb(['shell', 'sha256sum', installedPath]).stdout.trim().split(/\s+/)[0];
  assert.equal(installedApkSha256, localApkSha256, 'Installed Android APK hash differs');
  const metadata = {
    target: 'android-avd', serial: androidSerial,
    qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(),
    apiLevel: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
    abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
    product: adb(['shell', 'getprop', 'ro.product.name']).stdout.trim(),
    pageSizeBytes: Number(adb(['shell', 'getconf', 'PAGE_SIZE']).stdout.trim()),
    model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
    fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    localApkSha256, installedApkSha256,
  };
  assert.equal(metadata.qemu, '1');
  assert.equal(metadata.apiLevel, 37);
  assert.equal(metadata.abi, 'arm64-v8a');
  await writeFile(resolve(directory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const observations = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const runID = `${runPrefix}-android-${index}`;
      const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n',
        `${bundleID}/.MainActivity`, '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.id,
        '--es', 'spinon_c114_grid_run_id', runID]);
      await saveLog(runtimeDirectory, `launch-${scene.id}.log`, launch);
      const { pid, log } = waitForAndroid(androidSerial, scene, runID);
      const observation = validateLog(log, scene, runID, 'android-avd');
      observation.pid = Number(pid);
      observations.push(observation);
      await writeFile(resolve(runtimeDirectory, `runtime-${scene.id}.log`), `${log}\n`);
      if (scene.id === representativeScene) {
        const capture = adb(['exec-out', 'screencap', '-p'], { encoding: null });
        await writeFile(resolve(directory, 'representative.png'), capture.stdout);
      }
      if ((index + 1) % 10 === 0 || index + 1 === scenes.length) {
        console.log(`android-avd ${index + 1}/${scenes.length}: ${scene.id} passed`);
      }
    }
  } finally {
    adb(['shell', 'am', 'force-stop', bundleID], { allowFailure: true });
  }
  const result = { target: 'android-avd', runPrefix, sceneCount: observations.length,
    positiveScenes: observations.length, rejectedScenes: 0, retryCount: 0, observations };
  await writeFile(resolve(directory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
  return result;
}

function iosLog(pid, runID) {
  const predicate = `process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "${runID}" `
    + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW")';
  const output = command('xcrun', ['simctl', 'spawn', simulatorUDID, 'log', 'show',
    '--last', '2m', '--style', 'compact', '--predicate', predicate], { timeout: 20_000 }).stdout;
  const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForIOS(scene, runID, pid) {
  const deadline = Date.now() + 60_000;
  let latest = '';
  while (Date.now() < deadline) {
    latest = iosLog(pid, runID);
    if (latest.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `)
        && traceParts(latest, runID).complete
        && latest.includes('presented boxes=1')) return latest;
    if (latest.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || latest.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) return latest;
    sleep(400);
  }
  throw new Error(`${scene.id}: iOS scene timed out\n${latest}`);
}

async function runIOS() {
  const directory = runDirectory('ios-simulator');
  const runtimeDirectory = resolve(directory, 'runtime');
  await mkdir(runtimeDirectory, { recursive: true });
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator not found: ${simulatorUDID}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  const modelIdentifier = command('xcrun', ['simctl', 'getenv', simulatorUDID,
    'SIMULATOR_MODEL_IDENTIFIER']).stdout.trim();
  assert.equal(modelIdentifier, 'iPhone18,1');
  await saveLog(directory, 'install.log', command('xcrun', ['simctl', 'install', simulatorUDID, iosApp]));
  const executablePath = resolve(iosApp, 'SpinonBootstrap');
  const executableSha256 = sha256(await readFile(executablePath));
  const installedPath = command('xcrun', ['simctl', 'get_app_container', simulatorUDID,
    bundleID, 'app']).stdout.trim();
  const installedExecutableSha256 = sha256(await readFile(resolve(installedPath, 'SpinonBootstrap')));
  assert.equal(installedExecutableSha256, executableSha256, 'Installed iOS executable hash differs');
  const metadata = { target: 'ios-simulator', udid: simulatorUDID, name: simulator[0].name,
    modelIdentifier, runtime: simulator[0].runtime, executableSha256, installedExecutableSha256 };
  await writeFile(resolve(directory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const observations = [];
  try {
    for (const [index, scene] of scenes.entries()) {
      const runID = `${runPrefix}-ios-${index}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        simulatorUDID, bundleID, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-run-id=${runID}`, `--spinon-c114-grid-scene=${scene.id}`]);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: iOS launch PID missing`);
      await saveLog(runtimeDirectory, `launch-${scene.id}.log`, launch);
      const log = waitForIOS(scene, runID, pid);
      const observation = validateLog(log, scene, runID, 'ios-simulator');
      observation.pid = pid;
      observations.push(observation);
      await writeFile(resolve(runtimeDirectory, `runtime-${scene.id}.log`), `${log}\n`);
      if (scene.id === representativeScene) {
        await saveLog(directory, 'screenshot.log', command('xcrun', ['simctl', 'io',
          simulatorUDID, 'screenshot', resolve(directory, 'representative.png')]));
      }
      if ((index + 1) % 10 === 0 || index + 1 === scenes.length) {
        console.log(`ios-simulator ${index + 1}/${scenes.length}: ${scene.id} passed`);
      }
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', simulatorUDID, bundleID], { allowFailure: true });
  }
  const result = { target: 'ios-simulator', runPrefix, sceneCount: observations.length,
    positiveScenes: observations.length, rejectedScenes: 0, retryCount: 0, observations };
  await writeFile(resolve(directory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
  return result;
}

await mkdir(resolve(evidenceRoot, runPrefix), { recursive: true });
const output = { schemaVersion: 1, runPrefix, sourceHead: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
  target, expectedSceneCount: scenes.length, results: [] };
try {
  if (target === 'android' || target === 'both') output.results.push(await runAndroid());
  if (target === 'ios' || target === 'both') output.results.push(await runIOS());
  await writeFile(resolve(evidenceRoot, runPrefix, 'result.json'), `${JSON.stringify(output, null, 2)}\n`);
  console.log(JSON.stringify({
    evidenceDirectory: resolve(evidenceRoot, runPrefix),
    results: output.results.map(({ target: platform, sceneCount, rejectedScenes, retryCount }) =>
      ({ target: platform, sceneCount, rejectedScenes, retryCount })),
  }, null, 2));
} catch (error) {
  output.failure = String(error?.stack ?? error);
  await writeFile(resolve(evidenceRoot, runPrefix, 'failure.json'), `${JSON.stringify(output, null, 2)}\n`);
  throw error;
}
