import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
const serial = process.argv[3] ?? 'emulator-5554';
const simulatorUDID = process.argv[3] ?? 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-4-auto-repeat-neighbors-simulator.mjs android-avd|ios-simulator [AVD serial|iOS Simulator UDID]');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const runPrefix = `c114-neighbors-${target}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-4-auto-repeat-neighbors-${date}`, `${target}-${runPrefix}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

const scenes = [
  {
    id: 'auto-fill-fixed-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=10,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=10,height=40]',
    tracks: [20, 46, 46, 46, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fit-fixed-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=154,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=154,height=40]',
    tracks: [20, 154, 0, 0, 10],
    collapsed: [false, false, true, true, false],
  },
  {
    id: 'auto-fill-pattern-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=10,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=10,height=40]',
    tracks: [20, 45, 20, 45, 20, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fit-span-neighbors',
    axis: 'Columns',
    boxCount: 3,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=12,height=12;2:node=3,x=190,y=0,width=12,height=12]',
    tracks: [20, 73, 73, 0, 10],
    collapsed: [false, false, false, true, false],
  },
  {
    id: 'auto-fit-empty-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=200,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60]',
    tracks: [20, 0, 0, 0, 10],
    collapsed: [false, true, true, true, false],
  },
  {
    id: 'auto-fill-prefix-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=12,height=12]',
    tracks: [20, 52, 52, 52],
    collapsed: [false, false, false, false],
  },
  {
    id: 'auto-fill-suffix-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=12,height=12]',
    tracks: [55.328125, 55.328125, 55.328125, 10],
    collapsed: [false, false, false, false],
  },
  {
    id: 'auto-fill-indefinite-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=86,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=86,height=60;1:node=2,x=76,y=0,width=12,height=12]',
    tracks: [20, 40, 10],
    collapsed: [false, false, false],
  },
  {
    id: 'auto-fill-max-size-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=182,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=182,height=60]',
    tracks: [20, 40, 40, 40, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fill-min-size-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=230,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=230,height=60]',
    tracks: [20, 40, 40, 40, 40, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-overflow-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;1:node=2,x=86,y=0,width=12,height=12]',
    tracks: [30, 40, 20],
    collapsed: [false, false, false],
  },
  {
    id: 'auto-fill-row-neighbors',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=50,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=200;1:node=2,x=0,y=190,width=12,height=12]',
    tracks: [20, 46, 46, 46, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fit-row-neighbors',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=50,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=200;1:node=2,x=0,y=28,width=12,height=12]',
    tracks: [20, 154, 0, 0, 10],
    collapsed: [false, false, true, true, false],
  },
];

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

async function saveLog(path, result) {
  await writeFile(resolve(evidenceDirectory, path), `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function currentAndroidDevice() {
  const connected = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(connected.length, 1, `Android AVD가 연결되어 있지 않습니다: ${serial}`);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  assert.equal(adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(), '1',
    '대상이 Android Emulator가 아닙니다. 실기기는 사용하지 않습니다.');
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  assert.equal(apiLevel, 37, `고정 Android API 37 AVD가 아닙니다: ${apiLevel}`);
  return {
    adb,
    metadata: {
      serial,
      apiLevel,
      model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    },
  };
}

function androidLog(adb, pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function iosProcessLog(pid) {
  const predicate = 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C114_GRID" '
    + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW" '
    + 'OR eventMessage CONTAINS[c] "node_frames_css_px=")';
  const output = command('xcrun', ['simctl', 'spawn', simulatorUDID, 'log', 'show', '--last', '30s',
    '--style', 'syslog', '--predicate', predicate], { timeout: 45_000 }).stdout;
  const processMarker = new RegExp('SpinonBootstrap\\[' + pid + '(?::|\\])');
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForLog(readLog, scene, runID) {
  const deadline = Date.now() + 150_000;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    const summary = latest.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
    const trace = latest.includes('SPINON_C114_GRID_TRACE_CHUNK');
    const draw = latest.includes('SPINON_C0410_DRAW')
      && latest.includes(`presented boxes=${scene.boxCount}`);
    if ((summary && trace && draw)
        || latest.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || latest.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) return latest;
    sleep(1_000);
  }
  throw new Error(`${scene.id}: Android/iOS runtime 결과 대기 시간이 초과되었습니다.\n${latest}`);
}

function verifyRuntimeLog(log, scene, runID) {
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  assert.ok(log.includes(summary), `${scene.id}: runtime summary가 없습니다.`);
  assert.ok(log.includes(`layout=ready boxes=${scene.boxCount}`),
    `${scene.id}: visible box 수가 예상과 다릅니다.`);
  assert.ok(log.includes(scene.rootFrame)
    && (scene.itemFrame === null || log.includes(scene.itemFrame)),
    `${scene.id}: root/item frame이 예상과 다릅니다.`);
  assert.ok(log.includes(scene.nodeFrames), `${scene.id}: 전체 node frame이 예상과 다릅니다.`);
  assert.ok(log.includes('SPINON_C114_GRID_TRACE_CHUNK'), `${scene.id}: track-sizing trace가 없습니다.`);
  assert.ok(log.includes('SPINON_C0410_DRAW')
    && log.includes(`presented boxes=${scene.boxCount}`),
    `${scene.id}: 화면 표시 결과가 없습니다.`);
  assert.ok(!log.includes('SPINON_C114_GRID_ASSERTION_ERROR'), `${scene.id}: frame assertion이 실패했습니다.`);
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: 실행 인자가 거부됐습니다.`);
  const traceParts = log.split(/\r?\n/).flatMap((line) => {
    if (!line.includes(`SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `)) return [];
    const match = line.match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  }).sort((left, right) => left.index - right.index);
  assert.ok(traceParts.length > 0 && traceParts.length === traceParts[0].total,
    `${scene.id}: track-sizing trace chunk이 누락됐습니다.`);
  assert.ok(traceParts.every(({ total }) => total === traceParts[0].total),
    `${scene.id}: track-sizing trace chunk 수가 일치하지 않습니다.`);
  const tracePayload = traceParts.map(({ payload }) => payload).join('')
    .replace(/^grid_track_sizing_diagnostics=/, '');
  const diagnostics = JSON.parse(tracePayload);
  const axisPass = diagnostics.trackSizingTraces[0].passes
    .filter(({ axis }) => axis === scene.axis).at(-1);
  assert.ok(axisPass, `${scene.id}: ${scene.axis} track-sizing pass가 없습니다.`);
  const finalTracks = axisPass.stages.at(-1).tracks.filter(({ gutter }) => !gutter);
  assert.deepEqual(finalTracks.map(({ collapsed }) => collapsed), scene.collapsed,
    `${scene.id}: collapsed track 상태가 예상과 다릅니다.`);
  assert.equal(finalTracks.length, scene.tracks.length, `${scene.id}: track 수가 다릅니다.`);
  finalTracks.forEach(({ baseSize }, index) => assert.ok(
    Math.abs(baseSize - scene.tracks[index]) <= 1 / 64,
    `${scene.id}: track ${index + 1} 크기 예상 ${scene.tracks[index]}, 실제 ${baseSize}`,
  ));
  return {
    sceneId: scene.id,
    summary: log.split(/\r?\n/).find((line) => line.includes(summary)),
    nodeFrames: scene.nodeFrames,
    tracks: finalTracks.map(({ baseSize, collapsed }) => ({ baseSize, collapsed })),
    trackTraceChunks: traceParts.length,
    drawPresented: true,
  };
}

await mkdir(evidenceDirectory, { recursive: true });
const observations = [];

if (target === 'ios-simulator') {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator를 찾지 못했습니다: ${simulatorUDID}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  assert.equal(simulator[0].name, 'iPhone 17 Pro');
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulatorUDID, appPath]));
  const appSha256 = sha256(await readFile(resolve(appPath, 'SpinonBootstrap')));
  for (const [index, scene] of scenes.entries()) {
    const runID = `${runPrefix}-${index}`;
    const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulatorUDID,
      packageName, '--spinon-c114-grid-track-sizing', `--spinon-c114-grid-run-id=${runID}`,
      `--spinon-c114-grid-scene=${scene.id}`]);
    await saveLog(`launch-${scene.id}.log`, launch);
    const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
    assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: launch PID가 없습니다.`);
    const log = waitForLog(() => iosProcessLog(pid), scene, runID);
    await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
    observations.push(verifyRuntimeLog(log, scene, runID));
    await saveLog(`screenshot-${scene.id}.log`, command('xcrun', ['simctl', 'io', simulatorUDID,
      'screenshot', resolve(evidenceDirectory, `${scene.id}.png`)]));
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target, udid: simulatorUDID, name: simulator[0].name, runtime: simulator[0].runtime, appSha256,
  }, null, 2)}\n`);
} else {
  const { adb, metadata } = currentAndroidDevice();
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const apkBytes = await readFile(apkPath);
  await saveLog('install.log', adb(['install', '-r', apkPath]));
  for (const [index, scene] of scenes.entries()) {
    const runID = `${runPrefix}-${index}`;
    const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', component,
      '--ez', 'spinon_c114_grid_track_sizing', 'true',
      '--es', 'spinon_c114_grid_scene', scene.id,
      '--es', 'spinon_c114_grid_run_id', runID]);
    await saveLog(`launch-${scene.id}.log`, launch);
    const deadline = Date.now() + 150_000;
    let pid = null;
    let log = '';
    while (Date.now() < deadline) {
      pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
        .stdout.trim().split(/\s+/)[0]) || null;
      if (pid) log = androidLog(adb, pid);
      const summary = log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
      const trace = log.includes('SPINON_C114_GRID_TRACE_CHUNK');
      const draw = log.includes('SPINON_C0410_DRAW')
        && log.includes(`presented boxes=${scene.boxCount}`);
      if ((summary && trace && draw)
          || log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
          || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
      sleep(300);
    }
    await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
    observations.push(verifyRuntimeLog(log, scene, runID));
    const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    ...metadata, target, apkSha256: sha256(apkBytes),
  }, null, 2)}\n`);
}

await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
