import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const serial = 'emulator-5554';
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const v8Directory = process.env.SPINON_V8_DIR
  ?? '/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8';
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const runID = `c13-1-stacking-android-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/android-avd/${runID}`);
const identitySha256 = '866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483';
const oracleSha256 = '279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2';

function command(binary, args, {
  timeout = 60_000, allowFailure = false, encoding = 'utf8', maxBuffer = 96 * 1024 * 1024,
} = {}) {
  const result = spawnSync(binary, args, { cwd: repositoryRoot, encoding, timeout, maxBuffer });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const stdout = Buffer.isBuffer(result.stdout) ? result.stdout.toString('utf8') : result.stdout;
    const stderr = Buffer.isBuffer(result.stderr) ? result.stderr.toString('utf8') : result.stderr;
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? stderr ?? stdout}`);
  }
  return result;
}

const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function pngDimensions(bytes) {
  assert.equal(bytes.toString('hex', 0, 8), '89504e470d0a1a0a', 'PNG signature');
  return { width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20) };
}

function waitFor(predicate, description, timeout = 180_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = predicate();
    if (result) return result;
    sleep(500);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function appPID() {
  const output = adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim();
  const pid = Number(output.split(/\s+/)[0]);
  return Number.isInteger(pid) && pid > 0 ? pid : null;
}

function processLog(pid) {
  const output = adb(['logcat', '-d', '-v', 'threadtime', 'SpinonBootstrap:V', '*:S']).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  });
}

function revisionTuple(line) {
  const match = line.match(/generation=(\d+) document_revision=(\d+) render_tree_revision=(\d+) style_revision=(\d+)/);
  assert.ok(match, `revision tuple이 없습니다: ${line}`);
  return match.slice(1).map(Number);
}

async function stopPreviousProcess(label) {
  const result = adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  await writeFile(resolve(evidenceDirectory, `${label}-force-stop.log`), `${result.stdout}${result.stderr}`);
  waitFor(() => appPID() === null, `${label}: 이전 앱 process 종료`, 30_000);
}

async function launchFixture(label, { stale = false, conflict = false } = {}) {
  await stopPreviousProcess(label);
  await writeFile(resolve(evidenceDirectory, `${label}-logcat-clear.log`),
    adb(['logcat', '-c'], { allowFailure: true }).stdout ?? '');
  const args = ['shell', 'am', 'start', '-n', component];
  if (conflict) {
    args.push('--ez', 'spinon_c131_stacking_descendant_clip', 'true');
    args.push('--ez', 'spinon_c131_compact_surface_matrix', 'true');
  } else {
    args.push('--ez', 'spinon_c131_stacking_descendant_clip', 'true');
    if (stale) args.push('--ez', 'spinon_c131_stacking_descendant_clip_stale_revision', 'true');
  }
  const launch = adb(args, { timeout: 90_000 });
  await writeFile(resolve(evidenceDirectory, `${label}-launch.log`), launch.stdout + launch.stderr);
  const pid = waitFor(() => appPID(), `${label}: app PID 시작`, 30_000);
  return { pid, launch };
}

async function captureSuccessfulRun(index, priorPIDs, expectedScreenDimensions) {
  const name = `run-${index}`;
  const launched = await launchFixture(name);
  const { pid } = launched;
  assert.equal(priorPIDs.has(pid), false, `${name}: 새 앱 process PID가 이전 run과 같습니다.`);
  const result = waitFor(() => {
    const lines = processLog(pid);
    const gridLine = lines.find((line) => line.includes(
      'SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=matched'));
    const evalLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_EVAL_RUN run_id='));
    const pixelLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_PIXELCOPY_SUMMARY status=matched'));
    const fixtureLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_SUMMARY status=ok'));
    const hitLine = lines.find((line) => line.includes('SPINON_C131_STACKING_CLIP_HIT_TEST_SUMMARY status=ok'));
    const drawLine = lines.find((line) => line.includes('SPINON_C0410_DRAW generation=')
      && line.includes('status=0 presented boxes=13'));
    const failure = lines.find((line) => line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=error')
      || line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=mismatch')
      || line.includes('SPINON_C131_STACKING_CLIP_SUMMARY status=error')
      || line.includes('SPINON_C131_STACKING_CLIP_PIXELCOPY status=error'));
    if (failure) throw new Error(`${name}: Android AVD 검증 실패\n${lines.join('\n')}`);
    const gridMatch = gridLine?.match(/run_id=([a-f0-9]{12}) runtime_generation=(\d+)/);
    const evalMatch = evalLine?.match(/SPINON_C131_STACKING_CLIP_EVAL_RUN run_id=([a-f0-9]{12}) status=0\b.*layout=ready boxes=13/);
    const pixelMatch = pixelLine?.match(/run_id=([a-f0-9]{12}) pid=(\d+) surface_generation=(\d+) presentation_sequence=(\d+) draw_generation=(\d+) document_revision=(\d+) render_tree_revision=(\d+) style_revision=(\d+) samples=(\d+)\/18 png=(\S+) sha256=([a-f0-9]{64})/);
    const evalTuple = evalLine && revisionTuple(evalLine);
    const gridTupleGeneration = Number(gridMatch?.[2]);
    const pixelTuple = pixelMatch && [
      Number(pixelMatch[5]), Number(pixelMatch[6]), Number(pixelMatch[7]), Number(pixelMatch[8]),
    ];
    const drawGeneration = Number(drawLine?.match(/SPINON_C0410_DRAW generation=(\d+)/)?.[1]);
    const gridValid = gridMatch && gridMatch[1] === evalMatch?.[1]
      && gridMatch[1] === pixelMatch?.[1]
      && Number(pixelMatch?.[2]) === pid
      && gridTupleGeneration === evalTuple?.[0]
      && gridTupleGeneration === pixelTuple?.[0]
      && evalTuple?.every((value, tupleIndex) => value === pixelTuple?.[tupleIndex])
      && drawGeneration === Number(pixelMatch?.[3])
      && gridLine.includes('points=69120 matched=69120 mismatches=0 edge_controls=11/11 preview=0')
      && gridLine.includes(`identity_sha256=${identitySha256}`)
      && gridLine.includes(`oracle_sha256=${oracleSha256}`);
    const frameMarker = 'SPINON_C131_STACKING_CLIP_NODE_FRAME ';
    const frames = new Set(lines.filter((line) => line.includes(frameMarker))
      .map((line) => line.slice(line.indexOf(frameMarker) + frameMarker.length)));
    const hits = lines.filter((line) => line.includes('SPINON_C131_STACKING_CLIP_HIT_TEST id='));
    const pixels = lines.filter((line) => line.includes('SPINON_C131_STACKING_CLIP_PIXEL run_id='));
    const captureValid = pixelMatch && Number(pixelMatch[9]) === 18 && pixels.length === 18
      && pixels.every((line) => line.includes('exact=true'));
    if (gridValid && frames.size === 13 && hits.length === 18
        && hits.every((line) => line.includes('status=matched'))
        && captureValid && fixtureLine && hitLine && drawLine) {
      return { lines, gridLine, evalLine, pixelLine, drawLine, runID: gridMatch[1], generation: Number(pixelMatch[3]), pixelMatch };
    }
    return null;
  }, `${name}: Android API 37 AVD 13 frames, 18 samples, 69,120 hit identities and PixelCopy`);

  await writeFile(resolve(evidenceDirectory, `${name}-runtime.log`), `${result.lines.join('\n')}\n`);
  const pngPath = result.pixelMatch[10];
  const pixelCopy = adb(['exec-out', 'run-as', packageName, 'cat', pngPath], {
    encoding: 'buffer', timeout: 45_000,
  });
  assert.equal(sha256(pixelCopy.stdout), result.pixelMatch[11], `${name}: PixelCopy PNG SHA mismatch.`);
  const pixelDimensions = pngDimensions(pixelCopy.stdout);
  const expectedPixelSize = result.drawLine.match(/surface_texture=(\d+)x(\d+)/);
  assert.ok(expectedPixelSize, `${name}: draw log lacks the PixelCopy surface dimensions.`);
  assert.deepEqual(pixelDimensions, {
    width: Number(expectedPixelSize[1]), height: Number(expectedPixelSize[2]),
  }, `${name}: PixelCopy PNG dimensions disagree with the drawn surface.`);
  const pixelCopyPath = resolve(evidenceDirectory, `${name}-pixelcopy.png`);
  await writeFile(pixelCopyPath, pixelCopy.stdout);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: 'buffer', timeout: 45_000 });
  const screenshotPath = resolve(evidenceDirectory, `${name}-android-avd-screen.png`);
  await writeFile(screenshotPath, screenshot.stdout);
  const screenshotDimensions = pngDimensions(screenshot.stdout);
  assert.deepEqual(screenshotDimensions, expectedScreenDimensions,
    `${name}: AVD screen dimensions disagree with wm size.`);
  return {
    name, pid, runID: result.runID, surfaceGeneration: result.generation,
    revisions: revisionTuple(result.evalLine), points: 69_120, matched: 69_120,
    edgeControls: 11, fixedHitSamples: 18, nodeFrames: 13,
    identitySha256, oracleSha256,
    pixelCopy: {
      file: `${name}-pixelcopy.png`, sha256: sha256(pixelCopy.stdout),
      dimensions: pixelDimensions, samples: 18,
    },
    screenshot: `${name}-android-avd-screen.png`,
    screenshotDimensions, screenshotSha256: sha256(screenshot.stdout),
    runtimeLog: `${name}-runtime.log`,
  };
}

async function captureStaleRevisionControl() {
  const { pid } = await launchFixture('stale-revision-control', { stale: true });
  const lines = waitFor(() => {
    const filtered = processLog(pid);
    return filtered.some((line) => line.includes(
      'SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL name=stale-document-revision status=rejected field=document_revision surface_readback=not-requested'))
      ? filtered : null;
  }, 'Android stale document revision negative control');
  assert.ok(lines.some((line) => line.includes('SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=matched')));
  assert.equal(lines.some((line) => line.includes('SPINON_C131_STACKING_CLIP_PIXELCOPY_SUMMARY status=matched')), false,
    'stale draw must be rejected before PixelCopy completes.');
  await writeFile(resolve(evidenceDirectory, 'stale-revision-control.log'), `${lines.join('\n')}\n`);
  return { status: 'rejected-before-pixelcopy', pid, field: 'document_revision' };
}

async function captureRouteConflictControl() {
  const { pid } = await launchFixture('route-conflict-control', { conflict: true });
  const lines = waitFor(() => {
    const filtered = processLog(pid);
    return filtered.some((line) => line.includes('SPINON_C131_ARGUMENT_ERROR conflicting=multiple-fixture-routes'))
      ? filtered : null;
  }, 'Android conflicting fixture route negative control');
  assert.equal(lines.some((line) => line.includes('SPINON_C131_HOST_CREATED')), false);
  assert.equal(lines.some((line) => line.includes('SPINON_C131_STACKING_CLIP_EVAL_RUN')), false);
  await writeFile(resolve(evidenceDirectory, 'route-conflict-control.log'), `${lines.join('\n')}\n`);
  return { status: 'rejected-before-host-init', pid, hostCreated: false, evalStarted: false };
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  assert.equal(adb(['get-state']).stdout.trim(), 'device');
  const device = {
    serial,
    model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
    release: adb(['shell', 'getprop', 'ro.build.version.release']).stdout.trim(),
    sdk: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
    abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
    qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim() === '1',
    fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    displaySize: adb(['shell', 'wm', 'size']).stdout.trim(),
    displayDensity: adb(['shell', 'wm', 'density']).stdout.trim(),
  };
  assert.equal(device.sdk, 37, `Android API 37 AVD가 아닙니다: ${device.sdk}`);
  assert.equal(device.qemu, true, '실기기는 실행하지 않습니다.');
  assert.match(device.abi, /arm64-v8a/);
  const displaySizeMatches = [...device.displaySize.matchAll(/(?:Physical|Override) size: (\d+)x(\d+)/g)];
  assert.ok(displaySizeMatches.length > 0, `wm size 응답을 해석하지 못했습니다: ${device.displaySize}`);
  const activeDisplaySize = displaySizeMatches.at(-1);
  const expectedScreenDimensions = {
    width: Number(activeDisplaySize[1]), height: Number(activeDisplaySize[2]),
  };
  const requiredV8Revision = (await readFile(resolve(repositoryRoot, 'tools/v8/v8-revision.txt'), 'utf8')).trim();
  const actualV8Revision = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD']).stdout.trim();
  assert.equal(actualV8Revision, requiredV8Revision);
  assert.equal(command('git', ['-C', v8Directory, 'status', '--porcelain']).stdout.trim(), '', 'V8 checkout is dirty.');
  const v8Archive = resolve(v8Directory, 'out/boson-android-mac/obj/libv8_monolith.a');
  const build = command('mise', ['exec', '--', 'env', `SPINON_V8_DIR=${v8Directory}`,
    'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1',
    'bun', 'run', 'build:android'], { timeout: 900_000 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.match(`${build.stdout}${build.stderr}`, /BUILD SUCCESSFUL/);
  assert.match(`${build.stdout}${build.stderr}`, /c13_1_overflow_clip_fixture_test=1/);
  const apk = await readFile(apkPath);
  const install = adb(['install', '-r', apkPath], { timeout: 120_000 });
  await writeFile(resolve(evidenceDirectory, 'install.log'), install.stdout + install.stderr);
  await writeFile(resolve(evidenceDirectory, 'target-device.json'), `${JSON.stringify(device, null, 2)}\n`);

  const processIDs = new Set();
  const runs = [];
  for (let index = 1; index <= 3; index += 1) {
    const result = await captureSuccessfulRun(index, processIDs, expectedScreenDimensions);
    assert.equal(processIDs.has(result.pid), false);
    assert.equal(runs.some((run) => run.runID === result.runID), false);
    processIDs.add(result.pid);
    runs.push(result);
    process.stdout.write(`${result.name}: ${result.matched}/${result.points} hit identity · pid=${result.pid}\n`);
  }
  const staleRevisionControl = await captureStaleRevisionControl();
  const routeConflictControl = await captureRouteConflictControl();
  await stopPreviousProcess('final-app-shutdown');
  const summary = {
    schema: 'spinon-c13-1-stacking-full-viewport-android-avd/v1',
    status: 'matched', runID,
    sourceRevision: command('git', ['rev-parse', 'HEAD']).stdout.trim(),
    device, v8: { revision: actualV8Revision, archive: v8Archive },
    apkSha256: sha256(apk), oracle: { identitySha256, oracleSha256 },
    expected: { nodeFrames: 13, fixedHitSamples: 18, edgeControls: 11, gridPoints: 69_120 },
    runs, staleRevisionControl, routeConflictControl,
    scope: 'Android API 37 ARM64 AVD app route and SurfaceView PixelCopy 18-point readback; no physical device, DOM event dispatch, WPT, or full C13.1 claim.',
  };
  await writeFile(resolve(evidenceDirectory, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C13.1 stacking full viewport · Android AVD', '',
    `- 실행 묶음: \`${runID}\``,
    `- 대상: ${device.serial} · ${device.model} · API ${device.sdk} · ${device.abi}`,
    '- 새 앱 프로세스 3개에서 각각 13개 frame, 18개 hit/RGBA 표본, 11개 경계 제어와 69,120개 integer CSS point identity를 확인했다.',
    '- 각 run에서 evaluation/draw의 generation·document/render-tree/style revision tuple이 같고 Chrome 155 identity digest가 일치했다.',
    '- stale document revision과 route collision negative control을 보존했다. 성공 run 앱은 PixelCopy와 화면 캡처 뒤 종료했다.',
    '- [실행 요약](./summary.json) · build/install/runtime 로그, PixelCopy 표면과 AVD 화면을 보존했다.',
    '- API 37 ARM64 AVD만 확인했다. 실기기, hardware GPU, DOM event dispatch, WPT 및 C13.1 전체 지원 완료를 뜻하지 않는다.', '',
  ].join('\n'));
  await writeFile(resolve(evidenceDirectory, 'artifacts.sha256'), (await Promise.all(runs.flatMap(async (run) => [
    `${sha256(await readFile(resolve(evidenceDirectory, run.pixelCopy.file)))}  ${run.pixelCopy.file}`,
    `${sha256(await readFile(resolve(evidenceDirectory, run.screenshot)))}  ${run.screenshot}`,
    `${sha256(await readFile(resolve(evidenceDirectory, run.runtimeLog)))}  ${run.runtimeLog}`,
  ]))).join('\n') + '\n');
  process.stdout.write(`${JSON.stringify({ status: summary.status, runID, serial,
    runs: runs.length, staleRevisionControl, routeConflictControl, apkSha256: summary.apkSha256,
    evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  try { adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true }); } catch {}
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
