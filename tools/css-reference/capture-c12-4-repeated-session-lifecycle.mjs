import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

import {
  parseC124LifecycleCycle,
  parseC124LifecycleReady,
  validateC124LifecycleRun,
} from './c12-4-repeated-session-lifecycle-validation.mjs';
import { classifyC124AndroidTarget } from './c12-4-android-device.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const [serial = '', targetArgument = '--target=android-avd'] = process.argv.slice(2);
const requestedTarget = targetArgument.replace(/^--target=/, '');
assert.match(serial, /^[A-Za-z0-9._:-]+$/,
  '사용법: bun run css:verify:c12-4-session-lifecycle -- <Android serial> [--target=android-avd|android-physical] (기본 target: android-avd)');
assert.ok(['android-physical', 'android-avd'].includes(requestedTarget),
  `지원하지 않는 Android target입니다: ${requestedTarget}`);
const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const runId = `c124-lifecycle-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDate = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const outputDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-repeated-session-lifecycle-${evidenceDate}`, runId);
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const expectedCycles = 16;
const peakSampleCycles = new Set([1, 8, 16]);

function command(binary, args, { allowFailure = false, timeout = 120_000, env = process.env } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot,
    encoding: 'utf8',
    env,
    timeout,
    maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr ?? result.stdout}`);
  }
  return { status: result.status, stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function adb(args, options) {
  return command('adb', ['-s', serial, ...args], options);
}

function delay(milliseconds) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));
}

function currentLog() {
  return adb(['logcat', '-d', '-v', 'epoch', '-s', 'SpinonBootstrap:I'], {
    timeout: 20_000,
  }).stdout;
}

function parseTaskIds(output) {
  const ids = output.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  assert.ok(ids.length > 0, 'Android process task 목록이 비었습니다');
  assert.ok(ids.every((id) => /^\d+$/.test(id)), `task 목록에 TID가 아닌 행이 있습니다: ${ids.join(',')}`);
  return new Set(ids);
}

function parseMemorySummary(meminfo, status) {
  const pss = meminfo.match(/TOTAL PSS:\s*(\d+)/)?.[1];
  const rss = meminfo.match(/TOTAL RSS:\s*(\d+)/)?.[1];
  const vmRss = status.match(/^VmRSS:[ \t]*(\d+)[ \t]*kB\r?$/m)?.[1];
  const vmSize = status.match(/^VmSize:[ \t]*(\d+)[ \t]*kB\r?$/m)?.[1];
  const vmData = status.match(/^VmData:[ \t]*(\d+)[ \t]*kB\r?$/m)?.[1];
  assert.ok(pss && rss && vmRss && vmSize && vmData,
    'PSS/RSS 또는 Vm metric을 수집 결과에서 읽지 못했습니다');
  return {
    pssKiB: Number(pss),
    rssKiB: Number(rss),
    vmRssKiB: Number(vmRss),
    vmSizeKiB: Number(vmSize),
    vmDataKiB: Number(vmData),
  };
}

async function writeMemorySample(pid, name) {
  const meminfo = adb(['shell', 'dumpsys', 'meminfo', packageName], { timeout: 30_000 }).stdout;
  const status = adb(['shell', 'cat', `/proc/${pid}/status`]).stdout;
  await writeFile(resolve(outputDirectory, `${name}.dumpsys-meminfo.txt`), meminfo);
  await writeFile(resolve(outputDirectory, `${name}.proc-status.txt`), status);
  return parseMemorySummary(meminfo, status);
}

async function waitForLog(predicate, { deadlineMs = 20_000, onPoll } = {}) {
  const deadline = Date.now() + deadlineMs;
  let log = '';
  while (Date.now() < deadline) {
    log = currentLog();
    if (onPoll) await onPoll(log);
    if (predicate(log)) return log;
    if (!adb(['shell', 'pidof', packageName], { allowFailure: true }).stdout.trim()) {
      throw new Error(`검증 중 앱 process가 종료됐습니다. 마지막 로그:\n${log}`);
    }
    await delay(100);
  }
  throw new Error(`marker 제한 시간 초과. 마지막 로그:\n${log}`);
}

async function main() {
  await mkdir(dirname(outputDirectory), { recursive: true });
  await mkdir(outputDirectory, { recursive: false });
  const deviceState = adb(['get-state']).stdout.trim();
  assert.equal(deviceState, 'device', `선택한 Android target이 연결되지 않았습니다: ${serial} (${deviceState})`);
  const qemu = adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim();
  const model = adb(['shell', 'getprop', 'ro.product.model']).stdout.trim();
  const release = adb(['shell', 'getprop', 'ro.build.version.release']).stdout.trim();
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  const fingerprint = adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim();
  const target = classifyC124AndroidTarget({ serial, qemu, apiLevel });
  assert.equal(target.kind, requestedTarget === 'android-avd' ? 'avd' : 'physical-device',
    `요청한 ${requestedTarget} 실행 대상과 실제 Android target이 다릅니다.`);
  if (target.kind === 'physical-device') {
    assert.equal(model, 'SM-S731N', `계획에 고정한 실기기가 아닙니다: ${model}`);
    assert.equal(apiLevel, 36, `계획에 고정한 Android API가 아닙니다: ${apiLevel}`);
  }

  const build = command('mise', ['exec', '--', 'env',
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1',
    'bun', 'run', 'build:android'], { timeout: 900_000 });
  await writeFile(resolve(outputDirectory, 'build.log'), build.stdout + build.stderr);
  assert.match(build.stdout + build.stderr, /BUILD SUCCESSFUL/i, 'Android debug 앱 빌드가 성공하지 않았습니다');
  const apk = await readFile(apkPath);
  assert.ok(apk.length > 0, '검증 APK가 없습니다');
  const apkSha256 = createHash('sha256').update(apk).digest('hex');
  adb(['install', '-r', apkPath], { timeout: 180_000 });
  await writeFile(resolve(outputDirectory, 'device.txt'), [
    `serial=${serial}`, `model=${model}`, `android_release=${release}`,
    `api_level=${apiLevel}`, `qemu=${qemu}`, `build_fingerprint=${fingerprint}`,
    `apk_sha256=${apkSha256}`, `run_id=${runId}`, `cycles=${expectedCycles}`,
    'sessions_per_cycle=8', 'renderer_created=false', '',
  ].join('\n'));

  adb(['shell', 'am', 'force-stop', packageName]);
  adb(['logcat', '-c']);
  adb(['shell', 'am', 'start', '-W', '-n', component,
    '--ez', 'spinon_c124_repeated_session_lifecycle', 'true',
    '--es', 'spinon_c124_run_id', runId], { timeout: 30_000 });
  const pidOutput = await waitForLog((log) => log.includes(
    `SPINON_C124_LIFECYCLE status=started run_id=${runId} cycles=16 sessions_per_cycle=8`));
  const pid = adb(['shell', 'pidof', packageName]).stdout.trim();
  assert.match(pid, /^\d+$/, `앱 PID가 하나가 아닙니다: ${pid}`);
  const memorySamples = [{ phase: 'baseline', ...(await writeMemorySample(pid, 'memory-baseline')) }];
  const threadChecks = [];
  let log = pidOutput;

  for (let cycle = 1; cycle <= expectedCycles; cycle++) {
    const readyToken = `SPINON_C124_LIFECYCLE status=ready run_id=${runId} cycle=${cycle} session_count=8`;
    log = await waitForLog((candidate) => candidate.includes(readyToken), { deadlineMs: 30_000 });
    const ready = parseC124LifecycleReady(log, runId, cycle);
    const activeThreadIds = [...ready.ownerThreadIds, ...ready.workerThreadIds];
    const liveTaskIds = parseTaskIds(adb(['shell', 'run-as', packageName,
      'ls', '-1', `/proc/${pid}/task`]).stdout);
    const missingAtPeak = activeThreadIds.filter((tid) => !liveTaskIds.has(String(tid)));
    assert.deepEqual(missingAtPeak, [], `cycle ${cycle}: ready 단계에서 TID가 없습니다`);
    threadChecks.push({ cycle, phase: 'ready', pid, activeThreadIds, missing: missingAtPeak });

    if (peakSampleCycles.has(cycle)) {
      memorySamples.push({
        phase: `cycle-${cycle}-peak`,
        ...(await writeMemorySample(pid, `memory-cycle-${String(cycle).padStart(2, '0')}-peak`)),
      });
      const freedBeforeSnapshot = currentLog().includes(
        `SPINON_C124_LIFECYCLE status=freed run_id=${runId} cycle=${cycle} released=8 created=8`);
      assert.equal(freedBeforeSnapshot, false,
        `cycle ${cycle}: peak process memory snapshot completed after session release`);
      const tidsAfterSnapshot = parseTaskIds(adb(['shell', 'run-as', packageName,
        'ls', '-1', `/proc/${pid}/task`]).stdout);
      assert.deepEqual(activeThreadIds.filter((tid) => !tidsAfterSnapshot.has(String(tid))), [],
        `cycle ${cycle}: peak process memory snapshot 동안 runtime thread가 종료됐습니다`);
    }

    const freedToken = `SPINON_C124_LIFECYCLE status=freed run_id=${runId} cycle=${cycle} released=8 created=8`;
    log = await waitForLog((candidate) => candidate.includes(freedToken), { deadlineMs: 30_000 });
    const finishedCycle = parseC124LifecycleCycle(log, runId, cycle);
    assert.deepEqual(finishedCycle.ownerThreadIds, ready.ownerThreadIds);
    assert.deepEqual(finishedCycle.workerThreadIds, ready.workerThreadIds);
    const goneTaskIds = parseTaskIds(adb(['shell', 'run-as', packageName,
      'ls', '-1', `/proc/${pid}/task`]).stdout);
    const remaining = activeThreadIds.filter((tid) => goneTaskIds.has(String(tid)));
    assert.deepEqual(remaining, [], `cycle ${cycle}: 해제한 runtime thread가 남았습니다: ${remaining}`);
    threadChecks.push({ cycle, phase: 'freed', pid, activeThreadIds, remaining });
    await writeFile(resolve(outputDirectory, `cycle-${String(cycle).padStart(2, '0')}-threads.json`),
      `${JSON.stringify({ cycle, ownerThreadIds: ready.ownerThreadIds,
        cssWorkerThreadIds: ready.workerThreadIds, activeAtReady: true, absentAfterFree: true }, null, 2)}\n`);
    process.stdout.write(`cycle ${cycle}/${expectedCycles}: owner 8·CSS worker 8 생성 및 종료 확인\n`);
    log = await waitForLog((candidate) => candidate.includes(
      `SPINON_C124_LIFECYCLE status=cycle_complete run_id=${runId} cycle=${cycle} created=8 released=8`),
    { deadlineMs: 10_000 });
  }

  log = await waitForLog((candidate) => candidate.includes(
    `SPINON_C124_LIFECYCLE status=finished run_id=${runId} result=passed cycles_completed=16 sessions_created=128`),
  { deadlineMs: 15_000 });
  const parsedCycles = validateC124LifecycleRun(log, runId);
  assert.equal(parsedCycles.length, expectedCycles);
  await delay(350);
  const screenshotRemotePath = `/sdcard/${runId}.png`;
  const screenshotPath = resolve(outputDirectory, 'lifecycle-result.png');
  adb(['shell', 'screencap', '-p', screenshotRemotePath]);
  adb(['pull', screenshotRemotePath, screenshotPath], { timeout: 30_000 });
  adb(['shell', 'rm', '-f', screenshotRemotePath], { allowFailure: true });
  const screenshot = await readFile(screenshotPath);
  assert.deepEqual([...screenshot.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10],
    'AVD lifecycle result screen is not a PNG');
  const screenshotSha256 = createHash('sha256').update(screenshot).digest('hex');
  memorySamples.push({ phase: 'final', ...(await writeMemorySample(pid, 'memory-final')) });
  await writeFile(resolve(outputDirectory, 'logcat.txt'), log);
  await writeFile(resolve(outputDirectory, 'memory-samples.json'), `${JSON.stringify(memorySamples, null, 2)}\n`);
  await writeFile(resolve(outputDirectory, 'thread-lifecycle-checks.json'), `${JSON.stringify(threadChecks, null, 2)}\n`);
  await writeFile(resolve(outputDirectory, 'summary.json'), `${JSON.stringify({
    schema: 'spinon-c12-4-repeated-session-lifecycle/v1',
    status: 'passed', runId, pid, serial, model, release, apiLevel, apkSha256,
    resultScreen: { file: 'lifecycle-result.png', sha256: screenshotSha256 },
    target: target.kind, targetClassification: target.classification,
    cycles: parsedCycles.length, sessionsPerCycle: 8,
    totalSessionsCreated: parsedCycles.length * 8,
    ownerThreadCreateAndExitChecks: threadChecks.length / 2,
    workerThreadCreateAndExitChecks: threadChecks.length / 2,
    memorySamples,
    interpretation: 'process-wide memory snapshots are observational only; TID absence and repeated create/free are the lifecycle gates.',
    limitations: [
      target.kind === 'avd'
        ? 'single Android API 37 AVD with ANGLE/SwiftShader; not hardware GPU evidence'
        : 'single Samsung SM-S731N device and one Android 16/API 36 debug APK',
      'no renderer, GPU surface, layout evaluation, or WPT conformance test',
      'process PSS/RSS changes do not prove or disprove a memory leak',
      'stack-only commit/high-water and global allocator OOM recovery remain unmeasured',
    ],
  }, null, 2)}\n`);
  await writeFile(resolve(outputDirectory, 'README.md'), [
    `# C12.4 Android ${target.kind === 'avd' ? 'AVD' : '실기기'} 반복 session 수명 검증`, '',
    `- run ID: ${runId}`, `- 기기: ${model} · Android ${release}/API ${apiLevel} · serial ${serial}`,
    `- target: ${target.classification}`,
    `- APK SHA-256: ${apkSha256}`, `- 단일 process 안에서 ${parsedCycles.length} cycle × cycle당 session 8개를 생성·해제했다.`,
    '- debug-only status screen shows the completed lifecycle result:',
    '',
    '![C12.4 AVD lifecycle result](./lifecycle-result.png)',
    '',
    '- 매 cycle의 V8 owner 8개와 CSS worker 8개 TID가 생성 후 존재하고, 동기 해제 뒤 모두 사라짐을 확인했다.',
    '- baseline, cycle 1·8·16의 peak, 최종 process PSS/RSS와 VmSize/VmData를 저장했다. 이 값은 관찰값이며 누수 판정으로 사용하지 않는다.',
    '- renderer/surface/layout evaluation은 실행하지 않았다. WPT·CSS 적합성 근거가 아니다.',
    '- stack-only commit/high-water와 전역 allocator OOM은 이 실행에서 측정하지 않았다.', '',
  ].join('\n'));
  process.stdout.write(`반복 session 수명 검증 완료 · ${outputDirectory}\n`);
  adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
}

main().catch(async (error) => {
  await mkdir(outputDirectory, { recursive: true });
  try {
    await writeFile(resolve(outputDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
    await writeFile(resolve(outputDirectory, 'logcat-failure.txt'), currentLog());
  } catch {
    // preserve the primary validation failure
  }
  adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
