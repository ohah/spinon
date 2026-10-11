import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { closeSync, copyFileSync, mkdirSync, openSync, readFileSync, writeFileSync } from 'node:fs';
import { randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import { parseWorkerStackStage } from './c12-4-worker-stack-validation.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const platform = process.argv.find((argument) => argument.startsWith('--platform='))?.split('=')[1];
assert.ok(['ios', 'android'].includes(platform), '사용법: node capture-c12-4-concurrent-session-stack-cost.mjs --platform=ios|android');
const runID = `c124-${platform}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const outputDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-concurrent-session-stack-cost-2026-10-11/${runID}`);
const workerStackOutputDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c12-4-worker-stack-residency-2026-10-11/${runID}`);
const expectedStages = [0, 1, 2, 4, 8];
const appBundleID = 'dev.spinon.bootstrap';
const androidSerial = 'emulator-5554';
const androidPackage = 'dev.spinon.bootstrap';

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

function delay(milliseconds) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));
}

function readLog(path) {
  try {
    return readFileSync(path, 'utf8');
  } catch {
    return '';
  }
}

async function waitForMarker(path, marker, timeoutMilliseconds = 30_000) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    const log = readLog(path);
    const line = log.split(/\r?\n/).find((entry) => entry.includes(marker));
    if (line) return line;
    if (log.includes('status=create_failed')) {
      throw new Error(`session 생성 실패가 발생했습니다. marker=${marker}`);
    }
    await delay(150);
  }
  throw new Error(`제한 시간 안에 marker를 받지 못했습니다: ${marker}`);
}

async function waitForWorkerStackStage(path, stage) {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const lines = readLog(path);
    const count = lines.split(/\r?\n/).filter((line) =>
      line.includes(`SPINON_C124_WORKER_STACK_RESIDENCY stage=${stage} `)).length;
    if (count >= stage * 3) return parseWorkerStackStage(path, stage);
    await delay(100);
  }
  throw new Error(`stage ${stage}: CSS worker stack 표본 로그가 완성되지 않았습니다`);
}

function extractAndroidSmapsEntry(smaps, tid) {
  const lines = smaps.split(/\r?\n/);
  const header = new RegExp(`^[0-9a-f]+-[0-9a-f]+ .*\\[anon:stack_and_tls:${tid}\\]$`);
  const start = lines.findIndex((line) => header.test(line));
  assert.notEqual(start, -1, `TID ${tid}의 stack VMA가 smaps에 없습니다`);
  let end = start + 1;
  while (end < lines.length && !/^[0-9a-f]+-[0-9a-f]+ /.test(lines[end])) end++;
  return lines.slice(start, end).join('\n') + '\n';
}

async function sampleAndroidWorkerStackMappings(pid, stage, records) {
  const summaries = [];
  const selected = records.filter((record) => record.sample === 1);
  for (const record of selected) {
    const tid = record.report.thread_id;
    const prefix = `/proc/${pid}/task/${tid}`;
    const maps = command('adb', ['-s', androidSerial, 'shell', 'run-as', androidPackage,
      'cat', `${prefix}/maps`]);
    const stackMappings = maps.stdout.split(/\r?\n/)
      .filter((line) => line.includes(`[anon:stack_and_tls:${tid}]`))
      .map((line) => {
        const mapping = line.match(/^([0-9a-f]+)-([0-9a-f]+)/);
        assert.ok(mapping, `TID ${tid} stack VMA 주소를 해석하지 못했습니다: ${line}`);
        return {
          start: BigInt(`0x${mapping[1]}`),
          end: BigInt(`0x${mapping[2]}`),
          line,
        };
      })
      .sort((left, right) => left.start < right.start ? -1 : left.start > right.start ? 1 : 0);
    assert.ok(stackMappings.length > 0, `TID ${tid}의 stack VMA가 maps에 없습니다`);
    writeFileSync(resolve(workerStackOutputDirectory,
      `stage-${stage}-tid-${tid}-maps.txt`), `${stackMappings.map(({ line }) => line).join('\n')}\n`);
    const stackStart = BigInt(record.report.stack_start_address);
    const sampleStart = BigInt(record.report.mincore_sample_start_address);
    const sampleEnd = BigInt(record.report.mincore_sample_end_address);
    let coveredUntil = sampleStart;
    for (const mapping of stackMappings) {
      if (mapping.end <= coveredUntil || mapping.start > coveredUntil) continue;
      coveredUntil = mapping.end > coveredUntil ? mapping.end : coveredUntil;
      if (coveredUntil >= sampleEnd) break;
    }
    assert.ok(coveredUntil >= sampleEnd,
      `TID ${tid} mincore sample range가 동일 TID의 /proc VMA들로 완전히 덮이지 않습니다: `
      + `sample=0x${sampleStart.toString(16)}-0x${sampleEnd.toString(16)} `
      + `vmas=${stackMappings.map(({ line }) => line).join(' | ')}`);
    const smaps = command('adb', ['-s', androidSerial, 'shell', 'run-as', androidPackage,
      'cat', `${prefix}/smaps`]);
    const entry = extractAndroidSmapsEntry(smaps.stdout, tid);
    writeFileSync(resolve(workerStackOutputDirectory,
      `stage-${stage}-tid-${tid}-smaps.txt`), entry);
    const rss = entry.match(/^Rss:\s+(\d+) kB$/m)?.[1];
    const pss = entry.match(/^Pss:\s+(\d+) kB$/m)?.[1];
    assert.ok(rss !== undefined && pss !== undefined,
      `TID ${tid} smaps stack block에 RSS/PSS가 없습니다`);
    summaries.push({
      sessionIndex: record.sessionIndex,
      threadId: tid,
      pthreadStartAddress: record.report.stack_start_address,
      pthreadEndAddress: record.report.stack_end_address,
      mincoreSampleStartAddress: record.report.mincore_sample_start_address,
      mincoreSampleEndAddress: record.report.mincore_sample_end_address,
      vmas: stackMappings.map(({ line }) => line.trim()),
      rssKiB: Number(rss),
      pssKiB: Number(pss),
      smapsFile: `stage-${stage}-tid-${tid}-smaps.txt`,
    });
  }
  return summaries;
}

function spawnLogStream(binary, args, path) {
  const descriptor = openSync(path, 'a');
  const child = spawn(binary, args, {
    cwd: repositoryRoot,
    stdio: ['ignore', descriptor, descriptor],
  });
  closeSync(descriptor);
  return child;
}

async function sampleIOS(pid, stage, sample, phase = 'stage') {
  const name = phase === 'released'
    ? `released-sample-${sample}` : `stage-${String(stage).padStart(2, '0')}-sample-${sample}`;
  const footprint = command('footprint', ['-p', pid], { timeout: 30_000 });
  writeFileSync(resolve(outputDirectory, `${name}.footprint.txt`), footprint.stdout + footprint.stderr);
  if (sample === 1) {
    const vmmap = command('vmmap', ['-summary', pid], { allowFailure: true, timeout: 30_000 });
    writeFileSync(resolve(outputDirectory, `${name}.vmmap.txt`), [
      `exit_status=${vmmap.status}`,
      vmmap.stdout,
      vmmap.stderr,
    ].join('\n'));
  }
}

async function sampleAndroid(stage, sample, phase = 'stage') {
  const name = phase === 'released'
    ? `released-sample-${sample}` : `stage-${String(stage).padStart(2, '0')}-sample-${sample}`;
  const meminfo = command('adb', ['-s', androidSerial, 'shell', 'dumpsys', 'meminfo', androidPackage], {
    timeout: 30_000,
  });
  writeFileSync(resolve(outputDirectory, `${name}.dumpsys-meminfo.txt`), meminfo.stdout + meminfo.stderr);
  const pid = command('adb', ['-s', androidSerial, 'shell', 'pidof', androidPackage]).stdout.trim();
  assert.match(pid, /^\d+$/, `Android 앱 PID가 하나가 아닙니다: ${pid}`);
  const status = command('adb', ['-s', androidSerial, 'shell', 'cat', `/proc/${pid}/status`]);
  writeFileSync(resolve(outputDirectory, `${name}.proc-status.txt`), status.stdout + status.stderr);
  if ((phase === 'stage' && stage === 0 && sample === 1)
      || (phase === 'stage' && stage === 8)
      || (phase === 'released' && sample === 1)) {
    const rollup = command('adb', ['-s', androidSerial, 'shell', 'cat', `/proc/${pid}/smaps_rollup`], {
      allowFailure: true,
    });
    writeFileSync(resolve(outputDirectory, `${name}.smaps-rollup.txt`), [
      `exit_status=${rollup.status}`, rollup.stdout, rollup.stderr,
    ].join('\n'));
  }
  if ((phase === 'stage' && stage === 0 && sample === 1)
      || (phase === 'stage' && stage === 8 && sample === 1)
      || (phase === 'released' && sample === 1)) {
    const maps = command('adb', ['-s', androidSerial, 'shell', 'cat', `/proc/${pid}/maps`], {
      allowFailure: true,
    });
    writeFileSync(resolve(outputDirectory, `${name}.maps.txt`), [
      `exit_status=${maps.status}`, maps.stdout, maps.stderr,
    ].join('\n'));
  }
}

async function waitForAndroidPID() {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const output = command('adb', ['-s', androidSerial, 'shell', 'pidof', androidPackage], {
      allowFailure: true,
    }).stdout.trim();
    if (/^\d+$/.test(output)) return output;
    await delay(200);
  }
  throw new Error('Android AVD에서 앱 PID가 시작 제한 시간 안에 나타나지 않았습니다.');
}

function writeBuildMetadata(buildOutput) {
  writeFileSync(resolve(outputDirectory, 'build.log'), buildOutput);
}

async function captureIOS() {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const booted = Object.entries(devices).flatMap(([runtime, entries]) => entries
    .filter(({ state }) => state === 'Booted').map((device) => ({ ...device, runtime })));
  assert.equal(booted.length, 1, 'Booted iOS Simulator가 하나여야 합니다.');
  const simulator = booted[0];
  assert.equal(simulator.udid, 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D');
  assert.equal(simulator.name, 'iPhone 17 Pro');
  assert.match(simulator.runtime, /iOS-26-2$/);

  const build = spawnSync('mise', ['exec', '--', 'env',
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=0',
    'SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1',
    'bun', 'run', 'build:ios-sim'], {
    cwd: repositoryRoot, encoding: 'utf8', timeout: 900_000, maxBuffer: 64 * 1024 * 1024,
  });
  writeBuildMetadata(`${build.stdout ?? ''}${build.stderr ?? ''}`);
  assert.equal(build.status, 0, 'iOS Simulator 앱 빌드가 실패했습니다.');
  assert.ok(readLog(resolve(outputDirectory, 'build.log')).includes('** BUILD SUCCEEDED **'));

  const appPath = resolve(repositoryRoot,
    'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
  command('xcrun', ['simctl', 'install', simulator.udid, appPath]);
  writeFileSync(resolve(outputDirectory, 'device.txt'), [
    `platform=iOS Simulator`, `name=${simulator.name}`, `udid=${simulator.udid}`,
    `runtime=${simulator.runtime}`, command('xcodebuild', ['-version']).stdout.trim(),
  ].join('\n') + '\n');

  const logPath = resolve(outputDirectory, 'unified.log');
  const logStream = spawnLogStream('xcrun', [
    'simctl', 'spawn', simulator.udid, 'log', 'stream', '--style', 'compact',
    '--predicate', 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C124_SESSION_STACK" OR eventMessage CONTAINS[c] "SPINON_C124_WORKER_STACK_RESIDENCY")',
  ], logPath);
  try {
    await delay(500);
    const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
      simulator.udid, appBundleID, '--spinon-c124-session-stack-cost']);
    writeFileSync(resolve(outputDirectory, 'launch.log'), launch.stdout + launch.stderr);
    const pid = launch.stdout.match(/:\s*(\d+)/)?.[1];
    assert.ok(pid, `simulator launch PID를 해석하지 못했습니다: ${launch.stdout}`);
    writeFileSync(resolve(outputDirectory, 'pid.txt'), `${pid}\n`);

    const samples = [];
    const workerStackStages = [];
    for (const stage of expectedStages) {
      const marker = await waitForMarker(logPath,
        `SPINON_C124_SESSION_STACK status=ready session_count=${stage} `);
      samples.push({ stage, marker });
      const workerRecords = await waitForWorkerStackStage(logPath, stage);
      workerStackStages.push({ stage, records: workerRecords });
      writeFileSync(resolve(workerStackOutputDirectory, `stage-${stage}-samples.json`),
        `${JSON.stringify(workerRecords, null, 2)}\n`);
      await delay(1_000);
      await sampleIOS(pid, stage, 1);
      await delay(1_300);
      await sampleIOS(pid, stage, 2);
      await delay(1_300);
      await sampleIOS(pid, stage, 3);
    }
    const freed = await waitForMarker(logPath, 'SPINON_C124_SESSION_STACK status=freed session_count=0');
    samples.push({ stage: 0, marker: freed, released: true });
    await delay(1_000);
    await sampleIOS(pid, 0, 1, 'released');
    await delay(2_500);
    await sampleIOS(pid, 0, 2, 'released');
    await delay(2_500);
    await sampleIOS(pid, 0, 3, 'released');
    const finished = await waitForMarker(logPath, 'SPINON_C124_SESSION_STACK status=finished');
    assert.match(finished, /failed=0/);
    return { simulator, pid, samples, workerStackStages };
  } finally {
    logStream.kill('SIGTERM');
    command('xcrun', ['simctl', 'terminate', simulator.udid, appBundleID], { allowFailure: true });
  }
}

async function captureAndroid() {
  const devices = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${androidSerial} `) && line.includes(' device '));
  assert.equal(devices.length, 1, '고정 Android API 37 AVD가 연결되어 있지 않습니다.');
  const api = command('adb', ['-s', androidSerial, 'shell', 'getprop', 'ro.build.version.sdk']).stdout.trim();
  assert.equal(api, '37', `Android API 37이 아닙니다: ${api}`);
  const model = command('adb', ['-s', androidSerial, 'shell', 'getprop', 'ro.product.model']).stdout.trim();
  assert.match(model, /sdk_gphone/i, `실기기 또는 예상하지 않은 Android target입니다: ${model}`);

  const build = spawnSync('mise', ['exec', '--', 'env',
    'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1',
    'bun', 'run', 'build:android'], {
    cwd: repositoryRoot, encoding: 'utf8', timeout: 900_000, maxBuffer: 64 * 1024 * 1024,
  });
  writeBuildMetadata(`${build.stdout ?? ''}${build.stderr ?? ''}`);
  assert.equal(build.status, 0, 'Android AVD 앱 빌드가 실패했습니다.');
  const apk = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
  command('adb', ['-s', androidSerial, 'install', '-r', apk], { timeout: 180_000 });
  writeFileSync(resolve(outputDirectory, 'device.txt'), [
    `platform=Android AVD`, `serial=${androidSerial}`, `model=${model}`, `api=${api}`,
    command('adb', ['-s', androidSerial, 'shell', 'getprop', 'ro.build.version.release']).stdout.trim(),
  ].join('\n') + '\n');

  command('adb', ['-s', androidSerial, 'shell', 'am', 'force-stop', androidPackage]);
  command('adb', ['-s', androidSerial, 'logcat', '-c']);
  const logPath = resolve(outputDirectory, 'logcat.txt');
  const logStream = spawnLogStream('adb', [
    '-s', androidSerial, 'logcat', '-v', 'epoch', '-s', 'SpinonBootstrap:I',
  ], logPath);
  try {
    await delay(500);
    const launch = command('adb', ['-s', androidSerial, 'shell', 'am', 'start', '-n',
      'dev.spinon.bootstrap/.MainActivity', '--ez', 'spinon_c124_session_stack_cost', 'true']);
    writeFileSync(resolve(outputDirectory, 'launch.log'), launch.stdout + launch.stderr);
    const pid = await waitForAndroidPID();
    writeFileSync(resolve(outputDirectory, 'pid.txt'), `${pid}\n`);
    const samples = [];
    const workerStackStages = [];
    for (const stage of expectedStages) {
      const marker = await waitForMarker(logPath,
        `SPINON_C124_SESSION_STACK status=ready session_count=${stage} `);
      samples.push({ stage, marker });
      const workerRecords = await waitForWorkerStackStage(logPath, stage);
      const procCrossCheck = await sampleAndroidWorkerStackMappings(pid, stage, workerRecords);
      workerStackStages.push({ stage, records: workerRecords, procCrossCheck });
      writeFileSync(resolve(workerStackOutputDirectory, `stage-${stage}-samples.json`),
        `${JSON.stringify({ records: workerRecords, procCrossCheck }, null, 2)}\n`);
      await delay(1_000);
      await sampleAndroid(stage, 1);
      await delay(1_300);
      await sampleAndroid(stage, 2);
      await delay(1_300);
      await sampleAndroid(stage, 3);
    }
    const freed = await waitForMarker(logPath, 'SPINON_C124_SESSION_STACK status=freed session_count=0');
    samples.push({ stage: 0, marker: freed, released: true });
    const releasedMaps = command('adb', ['-s', androidSerial, 'shell', 'run-as', androidPackage,
      'cat', `/proc/${pid}/maps`]).stdout;
    const retainedWorkerStacks = workerStackStages.flatMap(({ records }) => records)
      .filter((record) => record.sample === 1)
      .map((record) => record.report.thread_id)
      .filter((tid) => releasedMaps.includes(`[anon:stack_and_tls:${tid}]`));
    assert.deepEqual(retainedWorkerStacks, [],
      `session 해제 뒤 CSS worker stack VMA가 남았습니다: ${retainedWorkerStacks.join(',')}`);
    writeFileSync(resolve(workerStackOutputDirectory, 'released-worker-stack-maps-check.json'),
      `${JSON.stringify({
        pid,
        observedWorkerTids: workerStackStages.at(-1)?.records
          .filter((record) => record.sample === 1).map((record) => record.report.thread_id) ?? [],
        retainedWorkerStacks,
      }, null, 2)}\n`);
    await delay(1_000);
    await sampleAndroid(0, 1, 'released');
    await delay(2_500);
    await sampleAndroid(0, 2, 'released');
    await delay(2_500);
    await sampleAndroid(0, 3, 'released');
    const finished = await waitForMarker(logPath, 'SPINON_C124_SESSION_STACK status=finished');
    assert.match(finished, /failed=false/);
    return { serial: androidSerial, model, api, pid, samples, workerStackStages };
  } finally {
    logStream.kill('SIGTERM');
    command('adb', ['-s', androidSerial, 'shell', 'am', 'force-stop', androidPackage], { allowFailure: true });
  }
}

async function main() {
  mkdirSync(outputDirectory, { recursive: true });
  mkdirSync(workerStackOutputDirectory, { recursive: true });
  const result = platform === 'ios' ? await captureIOS() : await captureAndroid();
  const processLog = platform === 'ios' ? 'unified.log' : 'logcat.txt';
  copyFileSync(resolve(outputDirectory, processLog),
    resolve(workerStackOutputDirectory, processLog));
  copyFileSync(resolve(outputDirectory, 'device.txt'),
    resolve(workerStackOutputDirectory, 'device.txt'));
  const metadata = {
    schema: 'spinon-c12-4-concurrent-session-stack-cost/v1',
    runID,
    platform,
    sessionStages: expectedStages,
    samplesPerStage: 3,
    releaseSamples: 3,
    cssWorkerStackBytesPerSession: 8 * 1024 * 1024,
    maximumCSSWorkerStackReservationBytes: 8 * 8 * 1024 * 1024,
    rendererCreated: false,
    result,
    interpretation: '프로세스 전체 메모리 관측은 CSS worker stack에 귀속하지 않는다. 별도 worker stack bounds/mincore 표본과 Android TID별 smaps 교차검증은 다른 metric으로 보존한다.',
    limitations: [
      '시뮬레이터/AVD 관측이며 실기기 RSS가 아니다.',
      'V8 heap과 session thread, 플랫폼 런타임 및 allocator가 process memory 변화에 포함된다.',
      'thread별 stack page commitment와 제품 최대 동시 session 수를 판정하지 않는다.',
      'mincore는 표본 순간 page residency이며 committed memory나 maximum stack use가 아니다.',
    ],
  };
  writeFileSync(resolve(outputDirectory, 'metadata.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const workerStackMetadata = {
    schema: 'spinon-c12-4-worker-stack-residency/v1',
    runID,
    platform,
    stages: result.workerStackStages,
    androidProcCrossCheck: platform === 'android',
    interpretation: 'mincore는 표본 순간 resident page 수다. Android smaps RSS/PSS는 해당 VMA에 대한 별도 OS 표본이며 서로 합산하지 않는다.',
    limitations: [
      'Android API 37 AVD와 iOS 26.2 Simulator 결과이며 실기기 수치가 아니다.',
      '이 worker memory probe는 hardware GPU 성능이나 WPT 적합성을 검증하지 않는다.',
      '전역 allocator OOM 복구, stack high-water mark 또는 누수 판정을 의미하지 않는다.',
    ],
  };
  writeFileSync(resolve(workerStackOutputDirectory, 'metadata.json'),
    `${JSON.stringify(workerStackMetadata, null, 2)}\n`);
  writeFileSync(resolve(workerStackOutputDirectory, 'README.md'), [
    '# C12.4 CSS worker stack bounds와 상주 페이지 표본',
    '',
    `- 실행 ID: ${runID}`,
    `- 플랫폼: ${platform}`,
    `- process-level 동시 session 측정: [${runID}](../c12-4-concurrent-session-stack-cost-2026-10-11/${runID}/README.md)`,
    '- 각 C12.4 CSS worker에서 OS stack bounds를 얻고 coordinator에서 mincore를 세 번 표본화했다.',
    '- Android에서는 같은 앱 UID로 TID별 stack VMA와 smaps RSS/PSS를 교차 확인하고 해제 뒤 worker stack VMA 부재를 확인했다.',
    '- resident page·RSS·PSS·요청 stack 크기는 별개 metric이다. commit·high-water mark·실기기 수치로 해석하지 않는다.',
    '',
  ].join('\n'));
  writeFileSync(resolve(outputDirectory, 'README.md'), [
    '# C12.4 동시 session stack 비용 시뮬레이터 관측',
    '',
    `- 실행 ID: ${runID}`,
    `- 플랫폼: ${platform}`,
    '- 단계: 0, 1, 2, 4, 8 sessions; 각 단계에서 process memory를 세 차례 수집했다.',
    '- session당 CSS worker의 stack 요청: 8 MiB. 8개 시점의 합계 예약은 64 MiB다.',
    '- 프로세스 memory는 V8과 플랫폼 런타임 등도 포함하며 CSS worker에 직접 귀속되지 않는다.',
    '- 실제 기기의 RSS나 thread별 stack commit은 측정하지 않았다.',
    '- 원본: `build.log`, `device.txt`, process 로그, stage별 footprint/vmmap 또는 dumpsys/proc 파일.',
    '',
  ].join('\n'));
  process.stdout.write(`C12.4 동시 session 비용 관측 완료 · ${outputDirectory}\n`);
}

main().catch((error) => {
  mkdirSync(outputDirectory, { recursive: true });
  writeFileSync(resolve(outputDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
