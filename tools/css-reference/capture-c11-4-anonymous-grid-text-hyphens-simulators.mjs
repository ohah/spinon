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
const runPrefix = `c114-hyphens-${target}-${Date.now()}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-runtime-2026-11-01',
  target, runPrefix);
const softHyphen = '\u00ad';
const wordJoiner = '\u2060';
const cases = [
  ['hyphens-none-soft-min', `AB${softHyphen}CD`, 'none', 'min-content', 120],
  ['hyphens-none-soft-max', `AB${softHyphen}CD`, 'none', 'max-content', 120],
  ['hyphens-none-soft-fixed-30', `AB${softHyphen}CD`, 'none', '30px', 30],
  ['hyphens-manual-soft-min', `AB${softHyphen}CD`, 'manual', 'min-content', 120],
  ['hyphens-manual-soft-max', `AB${softHyphen}CD`, 'manual', 'max-content', 120],
  ['hyphens-manual-soft-fixed-30', `AB${softHyphen}CD`, 'manual', '30px', 30],
  ['hyphens-default-soft-fixed-30', `AB${softHyphen}CD`, 'initial', '30px', 30],
  ['hyphens-none-visible-min', 'AB-CD', 'none', 'min-content', 120],
  ['hyphens-none-visible-fixed-30', 'AB-CD', 'none', '30px', 30],
  ['hyphens-manual-visible-min', 'AB-CD', 'manual', 'min-content', 120],
  ['hyphens-manual-visible-fixed-30', 'AB-CD', 'manual', '30px', 30],
  ['hyphens-none-unbroken-min', 'ABCD', 'none', 'min-content', 120],
  ['hyphens-manual-unbroken-min', 'ABCD', 'manual', 'min-content', 120],
  ['hyphens-none-multiple-min', `AB${softHyphen}CD${softHyphen}EF`, 'none', 'min-content', 120],
  ['hyphens-none-multiple-fixed-30', `AB${softHyphen}CD${softHyphen}EF`, 'none', '30px', 30],
  ['hyphens-manual-multiple-min', `AB${softHyphen}CD${softHyphen}EF`, 'manual', 'min-content', 120],
  ['hyphens-manual-multiple-fixed-30', `AB${softHyphen}CD${softHyphen}EF`, 'manual', '30px', 30],
  ['hyphens-inherited-none-fixed-30', `AB${softHyphen}CD`, 'none', '30px', 30, true],
  ['hyphens-inherited-manual-fixed-30', `AB${softHyphen}CD`, 'manual', '30px', 30, true],
  ['hyphens-auto-hyphenation-min', 'hyphenation', 'auto', 'min-content', 120],
  ['hyphens-auto-hyphenation-max', 'hyphenation', 'auto', 'max-content', 120],
  ['hyphens-auto-hyphenation-fixed-50', 'hyphenation', 'auto', '50px', 50],
  ['hyphens-auto-internationalization-min', 'internationalization', 'auto', 'min-content', 120],
  ['hyphens-auto-internationalization-max', 'internationalization', 'auto', 'max-content', 120],
  ['hyphens-auto-internationalization-fixed-50', 'internationalization', 'auto', '50px', 50],
  ['hyphens-auto-representation-min', 'representation', 'auto', 'min-content', 120],
  ['hyphens-auto-representation-max', 'representation', 'auto', 'max-content', 120],
  ['hyphens-auto-representation-fixed-50', 'representation', 'auto', '50px', 50],
].map(([id, text, specified, track, gridWidth, inherited = false]) => ({
  id, text, specified, mode: specified === 'initial' ? 'manual' : specified, track, gridWidth,
  inherited, rootWidth: inherited ? 320 : gridWidth,
  gridNode: inherited ? 2 : 1, itemNode: inherited ? 3 : 2,
}));
const rejectionCases = [
  {
    id: 'hyphens-auto-expected-reject', expectedReject: true,
    code: 'layout_failed',
    propertyContains: 'hyphens 입력이 고정 Ahem 10px ASCII direct-text profile과 일치하지 않습니다',
    nodeID: 'missing',
  },
  {
    id: 'hyphens-invalid-expected-reject', expectedReject: true,
    code: 'unsupported_c114_hyphens_value', property: 'computed hyphens', nodeID: '1',
  },
];

function command(binary, args, {
  allowFailure = false, timeout = 120_000, encoding = 'utf8', maxBuffer = 96 * 1024 * 1024,
} = {}) {
  const result = spawnSync(binary, args, { cwd: repositoryRoot, encoding, timeout, maxBuffer });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} failed: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function derive(scene) {
  const visibleText = scene.text.replaceAll(softHyphen, '');
  const maxContent = [...visibleText].length * 10;
  let segments;
  let autoLayoutText = scene.text;
  if (scene.mode === 'auto') {
    const syllablesByWord = {
      hyphenation: ['hy', 'phen', 'ation'],
      internationalization: ['in', 'ter', 'na', 'tion', 'al', 'iza', 'tion'],
      representation: ['rep', 're', 'sen', 'ta', 'tion'],
    };
    const syllables = syllablesByWord[scene.text];
    assert.ok(syllables, `${scene.id}: missing fixed syllable oracle`);
    segments = syllables.map((syllable, index) =>
      syllable.length * 10 + (index < syllables.length - 1 ? 10 : 0));
    autoLayoutText = syllables.join(softHyphen);
  } else if (scene.text.includes('-')) {
    const dash = scene.text.indexOf('-') + 1;
    segments = [scene.text.slice(0, dash).length * 10, scene.text.slice(dash).length * 10];
  } else if (scene.mode === 'manual' && scene.text.includes(softHyphen)) {
    const words = scene.text.split(softHyphen);
    segments = words.map((word, index) => word.length * 10 + (index < words.length - 1 ? 10 : 0));
  } else {
    segments = [maxContent];
  }
  const minContent = Math.max(...segments);
  const trackWidth = scene.track === 'min-content' ? minContent
    : scene.track === 'max-content' ? maxContent : scene.gridWidth;
  const lineAdvances = scene.track === 'max-content' ? [maxContent] : segments;
  const rowHeight = lineAdvances.length * 10;
  return {
    ...scene,
    computedHyphens: scene.mode === 'none' ? 'None' : scene.mode === 'auto' ? 'Auto' : 'Manual',
    layoutText: scene.mode === 'none' ? scene.text.replaceAll(softHyphen, wordJoiner)
      : scene.mode === 'auto' ? autoLayoutText : scene.text,
    minContent, maxContent, minContentHeight: segments.length * 10,
    maxContentHeight: 10, trackWidth, rowHeight, lineAdvances,
    textWidth: Math.max(...lineAdvances),
  };
}

const expectedValues = {
  reference: {
    source: 'CSS Text 3 §5.3 and fixed Ahem 10px precomparison',
    ahemFontSha256: 'b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448',
    runtimeToleranceCssPx: 1 / 64,
    scope: 'Ahem 10px; existing none/manual cases plus exact English auto dictionary words in an opt-in fixture',
  },
  cases: [...cases.map(derive), ...rejectionCases],
};

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
      && part.index >= 1 && part.index <= total), `${sceneID}: incomplete trace chunks`);
  const payload = [...unique.values()].sort((a, b) => a.index - b.index)
    .map((part) => part.payload).join('');
  const markerOffset = payload.indexOf('grid_track_sizing_diagnostics=');
  assert.ok(markerOffset >= 0, `${sceneID}: trace JSON marker missing`);
  return {
    report: JSON.parse(payload.slice(markerOffset + 'grid_track_sizing_diagnostics='.length)
      .replaceAll('\\134', '\\')),
    partCount: unique.size,
  };
}

function validateScene(log, scene, runID) {
  const expected = derive(scene);
  const near = (actual, value, label) => assert.ok(Number.isFinite(actual)
    && Math.abs(actual - value) <= expectedValues.reference.runtimeToleranceCssPx,
  `${scene.id}: ${label} ${actual} differs from ${value}`);
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `));
  assert.ok(summary, `${scene.id}: summary missing`);
  assert.ok(summary.includes(`expected_root_frame=width=${scene.rootWidth},height=${expected.rowHeight}`),
    `${scene.id}: root frame mismatch`);
  const ready = summary.match(/status=0 layout=ready boxes=(\d+) generation=(\d+) document_revision=(\d+)/);
  assert.ok(ready, `${scene.id}: runtime status/box count mismatch`);
  assert.equal(Number(ready[1]), scene.inherited ? 2 : 1, `${scene.id}: layout box count`);
  const generation = Number(ready[2]);
  const documentRevision = Number(ready[3]);
  assert.ok(generation > 0 && documentRevision > 0, `${scene.id}: missing revisions`);
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
    && !log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: runtime assertion failed`);

  const { report, partCount } = parseTrace(log, runID, scene.id);
  const revision = report.layoutInputRevision;
  assert.ok(revision?.source?.startsWith('HostDocument')
    && revision.source.includes(`DocumentGeneration(${generation})`)
    && revision.source.includes(`DocumentRevision(${documentRevision})`)
    && ['style', 'environment', 'intrinsicSize'].every((key) =>
      Number.isFinite(revision[key]) && revision[key] >= 0), `${scene.id}: invalid revisions`);
  const itemFrame = report.gridItemFrames?.find(({ node }) => node === scene.itemNode);
  assert.ok(itemFrame, `${scene.id}: Grid item frame missing`);
  near(itemFrame.x, 0, 'Grid item x');
  near(itemFrame.y, 0, 'Grid item y');
  near(itemFrame.width, expected.trackWidth, 'Grid item width');
  near(itemFrame.height, expected.rowHeight, 'Grid item height');
  const layout = report.gridItemTextLayouts?.find(({ node }) => node === scene.itemNode);
  assert.ok(layout, `${scene.id}: text layout missing`);
  assert.equal(layout.text, expected.layoutText);
  near(layout.width, expected.textWidth, 'text layout width');
  near(layout.height, expected.rowHeight, 'text layout height');
  assert.equal(layout.paragraphStyle.fontFamily, 'Ahem');
  assert.equal(layout.paragraphStyle.locale, 'en');
  assert.equal(layout.paragraphStyle.hyphens, expected.computedHyphens);
  assert.equal(layout.paragraphStyle.lineBreak, 'Normal');
  if (scene.mode === 'auto') {
    const syllablesByWord = {
      hyphenation: ['hy', 'phen', 'ation'],
      internationalization: ['in', 'ter', 'na', 'tion', 'al', 'iza', 'tion'],
      representation: ['rep', 're', 'sen', 'ta', 'tion'],
    };
    const syllables = syllablesByWord[scene.text];
    const breakOffsets = [];
    let sourceOffset = 0;
    for (const syllable of syllables.slice(0, -1)) {
      sourceOffset += syllable.length;
      breakOffsets.push(sourceOffset);
    }
    const expectedSourceMap = [];
    let paragraphOffset = 0;
    for (let index = 0; index <= scene.text.length; index++) {
      if (breakOffsets.includes(index)) {
        expectedSourceMap.push({
          node: scene.itemNode, sourceUtf16: [index, index],
          paragraphUtf8: [paragraphOffset, paragraphOffset + 2], inlineAncestors: [],
        });
        paragraphOffset += 2;
      }
      if (index < scene.text.length) {
        expectedSourceMap.push({
          node: scene.itemNode, sourceUtf16: [index, index + 1],
          paragraphUtf8: [paragraphOffset, paragraphOffset + 1], inlineAncestors: [],
        });
        paragraphOffset += 1;
      }
    }
    assert.deepEqual(layout.normalizedSources, expectedSourceMap,
      `${scene.id}: synthetic hyphen source ranges must be empty UTF-16 ranges`);
    assert.deepEqual(layout.sources.map(({ node, text: sourceText }) => ({
      node, text: sourceText,
    })), [{ node: scene.itemNode, text: scene.text }],
    `${scene.id}: layout source text must remain unchanged`);
  }
  assert.equal(layout.lines.length, expected.lineAdvances.length, 'line count');
  layout.lines.forEach((line, index) => {
    near(line.advance, expected.lineAdvances[index], `line ${index + 1} advance`);
    near(line.lineHeight, 10, `line ${index + 1} height`);
  });
  const source = report.gridItemIntrinsicInputs?.find(({ node }) => node === scene.itemNode)?.source;
  assert.ok(source, `${scene.id}: intrinsic contribution missing`);
  near(source.minContent, expected.minContent, 'intrinsic min-content');
  near(source.minContentHeight, expected.minContentHeight, 'intrinsic min-content height');
  near(source.maxContent, expected.maxContent, 'intrinsic max-content');
  near(source.maxContentHeight, expected.maxContentHeight, 'intrinsic max-content height');
  const passes = report.trackSizingTraces?.find(({ grid }) => grid === scene.gridNode)?.passes;
  const finalTrackSizes = (axis) => passes?.find((pass) => pass.axis === axis)?.stages?.at(-1)
    ?.tracks?.filter((track) => !track.gutter && !track.collapsed).map((track) => track.baseSize);
  near(finalTrackSizes('Columns')?.[0], expected.trackWidth, 'final column track');
  near(finalTrackSizes('Rows')?.[0], expected.rowHeight, 'final row track');
  return {
    scene: scene.id, result: 'PASS', specified: scene.specified,
    computed: expected.computedHyphens, track: scene.track,
    minContent: expected.minContent, maxContent: expected.maxContent,
    trackWidth: expected.trackWidth, rowHeight: expected.rowHeight,
    lineAdvances: expected.lineAdvances.join(','), traceParts: partCount,
  };
}

function validateRejection(log, scene, runID) {
  const marker = `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  const summary = log.split(/\r?\n/).find((line) => line.includes(marker));
  assert.ok(summary, `${scene.id}: rejection summary missing`);
  assert.ok(summary.includes('expected=true status=-11')
    && summary.includes(`code=${scene.code}`)
    && summary.includes(`node_id=${scene.nodeID} frames=absent`),
  `${scene.id}: rejection contract mismatch: ${summary}`);
  if (scene.propertyContains) {
    assert.ok(summary.includes(`property=contains:${scene.propertyContains}`),
      `${scene.id}: rejection property mismatch: ${summary}`);
  } else {
    assert.ok(summary.includes(`property=${scene.property}`),
      `${scene.id}: rejection property mismatch: ${summary}`);
  }
  assert.ok(!log.includes('SPINON_C114_GRID_TRACE_CHUNK')
    && !log.includes('SPINON_C114_GRID_SUMMARY'), `${scene.id}: rejected scene published layout`);
  return {
    scene: scene.id, result: 'EXPECTED_REJECT', specified: '', computed: '', track: '',
    minContent: '', maxContent: '', trackWidth: '', rowHeight: '', lineAdvances: '', traceParts: 0,
  };
}

function validate(sceneLog, scene, runID) {
  return scene.expectedReject ? validateRejection(sceneLog, scene, runID)
    : validateScene(sceneLog, scene, runID);
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
  const output = command('xcrun', ['simctl', 'spawn', udid, 'log', 'show', '--last', '2m',
    '--style', 'json', '--predicate', predicate], { timeout: 45_000 }).stdout;
  const records = JSON.parse(output);
  assert.ok(Array.isArray(records), 'iOS unified log output must be a JSON event array');
  return records.map(({ eventMessage }) => eventMessage ?? '').join('\n');
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
  assert.equal(installedHash, apkHash, 'Installed APK hash differs from built APK');
  Object.assign(metadata, { apkSha256: apkHash, installedApkSha256: installedHash });
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);

  const rows = [];
  try {
    for (const [index, scene] of expectedValues.cases.entries()) {
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      adb(['shell', 'am', 'start', '-S', '-W', '-n', `${packageName}/.MainActivity`,
        '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.id, '--es', 'spinon_c114_grid_run_id', runID]);
      const pid = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim().split(/\s+/)[0];
      assert.ok(pid, `${scene.id}: Android process missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      const completion = scene.expectedReject
        ? `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `
        : `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
      while (Date.now() < deadline) {
        log = androidLog(adb, pid);
        if (log.includes(completion)
            || log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 300);
      }
      await writeFile(resolve(evidenceDirectory, `${scene.id}.log`), `${log.replace(/[ \t]+$/gm, '').trimEnd()}\n`);
      rows.push(validate(log, scene, runID));
      if (scene.id === 'hyphens-manual-soft-fixed-30') {
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 1000);
        const screenshot = command('/opt/homebrew/bin/adb', ['-s', serial, 'exec-out', 'screencap', '-p'],
          { encoding: null, maxBuffer: 16 * 1024 * 1024 });
        await writeFile(resolve(evidenceDirectory, 'representative-screen.png'), screenshot.stdout);
      }
      console.log(`android ${index + 1}/${expectedValues.cases.length}: ${scene.id} PASS`);
    }
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    `scene\tresult\tspecified\tcomputed\ttrack\tmin_content\tmax_content\ttrack_width\trow_height\tline_advances\ttrace_parts\n`
      + rows.map((row) => [row.scene, row.result, row.specified, row.computed, row.track,
        row.minContent ?? '', row.maxContent ?? '', row.trackWidth ?? '', row.rowHeight ?? '',
        row.lineAdvances, row.traceParts].join('\t')).join('\n') + '\n');
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
    for (const [index, scene] of expectedValues.cases.entries()) {
      const runID = `${runPrefix}-${String(index + 1).padStart(2, '0')}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        udid, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-scene=${scene.id}`, `--spinon-c114-grid-run-id=${runID}`]);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: iOS process ID missing`);
      let log = '';
      const deadline = Date.now() + 90_000;
      const completion = scene.expectedReject
        ? `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `
        : `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
      while (Date.now() < deadline) {
        log = iosLog(udid, runID);
        if (log.includes(completion)
            || log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 500);
      }
      await writeFile(resolve(evidenceDirectory, `${scene.id}.log`), `${log.replace(/[ \t]+$/gm, '').trimEnd()}\n`);
      rows.push(validate(log, scene, runID));
      if (scene.id === 'hyphens-manual-soft-fixed-30') {
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 1000);
        command('xcrun', ['simctl', 'io', udid, 'screenshot',
          resolve(evidenceDirectory, 'representative-screen.png')]);
      }
      console.log(`ios ${index + 1}/${expectedValues.cases.length}: ${scene.id} PASS`);
    }
  } finally {
    command('xcrun', ['simctl', 'terminate', udid, packageName], { allowFailure: true });
  }
  await writeFile(resolve(evidenceDirectory, 'matrix.tsv'),
    `scene\tresult\tspecified\tcomputed\ttrack\tmin_content\tmax_content\ttrack_width\trow_height\tline_advances\ttrace_parts\n`
      + rows.map((row) => [row.scene, row.result, row.specified, row.computed, row.track,
        row.minContent ?? '', row.maxContent ?? '', row.trackWidth ?? '', row.rowHeight ?? '',
        row.lineAdvances, row.traceParts].join('\t')).join('\n') + '\n');
}

assert.ok(['android-avd', 'ios-simulator'].includes(target),
  'Usage: capture-c11-4-anonymous-grid-text-hyphens-simulators.mjs <android-avd|ios-simulator> <serial|UDID>');
assert.ok(deviceID, 'An explicit Android serial or iOS Simulator UDID is required');
await mkdir(evidenceDirectory, { recursive: true });
await writeFile(resolve(evidenceDirectory, 'expected-values.json'), `${JSON.stringify(expectedValues, null, 2)}\n`);
if (target === 'android-avd') await runAndroid(deviceID);
else await runIOS(deviceID);
