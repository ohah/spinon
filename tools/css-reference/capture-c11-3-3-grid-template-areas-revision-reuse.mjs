import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { selectAndroidAVDSerial } from './c11-3-3-grid-template-areas-runtime-device.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
assert.equal(target, 'android-avd',
  '사용법: node capture-c11-3-3-grid-template-areas-revision-reuse.mjs android-avd [emulator serial]');
const serialHint = process.argv[3] ?? process.env.SPINON_ANDROID_EMULATOR_SERIAL ?? 'emulator-5554';
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const runID = `c1133-revision-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-3-3-grid-template-areas-runtime-${date}`, `android-avd-${runID}`);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

async function saveLog(path, result) {
  await writeFile(resolve(evidenceDirectory, path), `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function currentProcessLog(adb, pid) {
  const output = adb(['shell', 'logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function waitForAndroidProcess(adb, timeoutMs = 30_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
      .stdout.trim().split(/\s+/)[0]);
    if (Number.isInteger(pid) && pid > 0) return pid;
    sleep(200);
  }
  throw new Error('AVD에서 Android 앱 process가 시작되지 않았습니다.');
}

function waitForLog(readLog, predicate, description, timeoutMs = 120_000) {
  const deadline = Date.now() + timeoutMs;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    if (predicate(latest)) return latest;
    sleep(250);
  }
  throw new Error(`${description} 대기 시간이 초과됐습니다.\n${latest}`);
}

function markerFields(line, marker) {
  const offset = line.indexOf(marker);
  assert.notEqual(offset, -1, `marker가 없습니다: ${marker}`);
  return Object.fromEntries([...line.slice(offset + marker.length).matchAll(/([a-z0-9_]+)=([^\s]+)/g)]
    .map(([, key, value]) => [key, value]));
}

await mkdir(evidenceDirectory, { recursive: true });
const spinonV8Dir = process.env.SPINON_V8_DIR ?? resolve(repositoryRoot, 'build/v8-source/v8');
const build = command('mise', ['exec', '--', 'env', `SPINON_V8_DIR=${spinonV8Dir}`,
  'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:android'], { timeout: 900_000 });
await saveLog('build.log', build);
const apkBytes = await readFile(apkPath);
const appSha256 = hash(apkBytes);

const connectedDeviceText = command('adb', ['devices', '-l']).stdout;
const serial = selectAndroidAVDSerial(connectedDeviceText, serialHint);
const connectedSerials = connectedDeviceText.split(/\r?\n/)
  .map((line) => line.match(/^(\S+)\s+device\b/)?.[1]).filter(Boolean);
assert.deepEqual(connectedSerials, [serial],
  '이 검증은 지정한 Android AVD 하나만 연결된 경우 실행합니다. 실기기는 사용하지 않습니다.');
const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const getprop = (name) => adb(['shell', 'getprop', name]).stdout.trim();
assert.equal(getprop('ro.kernel.qemu'), '1', `선택한 대상이 Android AVD가 아닙니다: ${serial}`);
const sdk = Number(getprop('ro.build.version.sdk'));
assert.equal(sdk, 37, `Android API 37 AVD가 아닙니다: API ${sdk}`);
const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
  .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger GLES 정보 없음';
assert.match(renderer, /SwiftShader/i, `Android AVD renderer가 SwiftShader가 아닙니다: ${renderer}`);
const metadata = {
  serial, model: getprop('ro.product.model'), release: getprop('ro.build.version.release'), sdk,
  qemu: true, fingerprint: getprop('ro.build.fingerprint'), renderer,
};
await saveLog('install.log', adb(['install', '-r', apkPath]));
const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', component,
  '--ez', 'spinon_c1133_grid_template_areas', 'true',
  '--es', 'spinon_c1133_grid_template_areas_run_id', runID,
  '--es', 'spinon_c11_grid_case', 'revision-reuse'], { timeout: 90_000 });
await saveLog('launch.log', launch);
const pid = waitForAndroidProcess(adb);
const readLog = () => currentProcessLog(adb, pid);
const summaryMarker = `SPINON_C1133_GRID_REVISION_SUMMARY run_id=${runID} `;
let log = waitForLog(readLog, (value) => value.includes(summaryMarker)
  || value.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'), '같은 host revision 재평가');
await writeFile(resolve(evidenceDirectory, 'revision-reuse.log'), `${log}\n`);
assert.ok(log.includes(`${summaryMarker}scene_id=revision-reuse `),
  `revision reuse summary가 없습니다.\n${log}`);
assert.ok(log.includes(`${summaryMarker}scene_id=revision-reuse host_id=`)
  && log.includes('same_host=true first_status=0 second_status=0')
  && log.includes('expected=true'), `revision/reuse 검증에 실패했습니다.\n${log}`);
const summary = log.split(/\r?\n/).find((line) => line.includes(summaryMarker));
const fields = markerFields(summary, summaryMarker);
assert.ok(Number(fields.second_document_revision) > Number(fields.first_document_revision),
  `document revision이 증가하지 않았습니다: ${summary}`);
assert.ok(Number(fields.second_render_tree_revision) > Number(fields.first_render_tree_revision),
  `render-tree revision이 증가하지 않았습니다: ${summary}`);
assert.equal(fields.first_item_node, fields.second_item_node, `같은 item node가 아닙니다: ${summary}`);
const revisionFrames = log.split(/\r?\n/).filter((line) =>
  line.includes(`SPINON_C1133_GRID_REVISION_NODE_FRAME run_id=${runID} `));
assert.equal(revisionFrames.length, 2, 'before/after node frame marker가 각각 하나여야 합니다.');
const parseFrame = (line, phase, x) => {
  const match = line.match(new RegExp(`phase=${phase} frame=\\d+:node=(\\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^\\s]+)`));
  assert.ok(match, `${phase}: frame marker를 해석하지 못했습니다: ${line}`);
  const frame = { nodeId: Number(match[1]), x: Number(match[2]), y: Number(match[3]),
    width: Number(match[4]), height: Number(match[5]) };
  assert.deepEqual(frame, { nodeId: Number(fields.first_item_node), x, y: 0, width: 80, height: 40 });
  return frame;
};
const before = parseFrame(revisionFrames[0], 'before', 0);
const after = parseFrame(revisionFrames[1], 'after', 80);
log = waitForLog(readLog, (value) => value.includes(
  `SPINON_C0410_DRAW status=0 presented boxes=2`), '변경된 장면 GPU draw/present', 60_000);
assert.ok(log.split(/\r?\n/).some((line) => line.includes(
  `SPINON_C1133_GRID_SUMMARY run_id=${runID} scene_id=revision-reuse status=0 layout=ready boxes=2`)),
'revision reuse 최종 layout summary가 없습니다.');
await writeFile(resolve(evidenceDirectory, 'revision-reuse.log'), `${log}\n`);

const result = {
  schema: 'spinon-c1133-grid-template-areas-revision-reuse-run/v1',
  runId: runID, target: 'android-avd', device: metadata, appSha256,
  sameHost: true,
  first: { status: Number(fields.first_status), documentRevision: Number(fields.first_document_revision),
    renderTreeRevision: Number(fields.first_render_tree_revision), itemFrame: before },
  second: { status: Number(fields.second_status), documentRevision: Number(fields.second_document_revision),
    renderTreeRevision: Number(fields.second_render_tree_revision), itemFrame: after },
  finalDrawPresentedTwoBoxes: true,
  limitations: ['Android API 37 AVD software renderer only', 'physical device and hardware GPU not verified'],
};
await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, appSha256, result }, null, 2));
