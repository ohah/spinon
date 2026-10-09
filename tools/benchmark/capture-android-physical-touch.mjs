#!/usr/bin/env node
import { execFile, spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { createWriteStream } from 'node:fs';
import { mkdir, readFile, readdir, rename, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { finished } from 'node:stream/promises';
import { StringDecoder } from 'node:string_decoder';
import { promisify } from 'node:util';
import { pathToFileURL } from 'node:url';
import { performance } from 'node:perf_hooks';
import {
  captureWindowIsValid,
  CaptureDeadline,
  RawContactCounter,
  stopChildBounded,
} from './android-physical-touch-capture.mjs';

const execFileAsync = promisify(execFile);
const APP_ID = 'dev.spinon.bootstrap';
const R05_MARKER = 'SPINON_R05_FENCE_WAIT_START';
const DEFAULT_FIRST_TIMEOUT_SECONDS = 60;
const DEFAULT_TOUCH_TIMEOUT_SECONDS = 60;
const DEFAULT_CONTACT_LIMIT = 30;
const DEFAULT_DRAIN_SECONDS = 2;
const DEFAULT_MAX_PROCESS_AGE_SECONDS = 300;
const HOST_STOP_TIMEOUT_MS = 2_000;
const MAX_RAW_LINE_BYTES = 4_096;

function fail(message) {
  throw new Error(message);
}

export function parseOptions(args) {
  const options = {
    adb: process.env.ANDROID_ADB || 'adb',
    appId: APP_ID,
    firstTimeoutSeconds: DEFAULT_FIRST_TIMEOUT_SECONDS,
    touchTimeoutSeconds: DEFAULT_TOUCH_TIMEOUT_SECONDS,
    contactLimit: DEFAULT_CONTACT_LIMIT,
    drainSeconds: DEFAULT_DRAIN_SECONDS,
    maxProcessAgeSeconds: DEFAULT_MAX_PROCESS_AGE_SECONDS,
    confirmR05Screen: false,
  };
  const values = new Map([
    ['--adb', 'adb'], ['--serial', 'serial'], ['--pid', 'pid'], ['--event', 'event'],
    ['--apk-sha256', 'apkSha256'], ['--out', 'out'], ['--app-id', 'appId'],
    ['--no-contact-timeout', 'firstTimeoutSeconds'], ['--touch-timeout', 'touchTimeoutSeconds'],
    ['--contacts', 'contactLimit'], ['--drain-seconds', 'drainSeconds'],
    ['--max-process-age', 'maxProcessAgeSeconds'],
  ]);
  const seen = new Set();

  for (let index = 0; index < args.length; index += 1) {
    const argument = args[index];
    if (argument === '--confirm-r05-screen') {
      options.confirmR05Screen = true;
      continue;
    }
    if (argument === '--help' || argument === '-h') {
      options.help = true;
      return options;
    }
    const key = values.get(argument);
    if (!key) fail(`알 수 없는 옵션: ${argument}`);
    if (seen.has(key)) fail(`옵션을 두 번 지정했습니다: ${argument}`);
    seen.add(key);
    const value = args[index + 1];
    if (value === undefined || value.startsWith('--')) fail(`옵션 값이 없습니다: ${argument}`);
    options[key] = value;
    index += 1;
  }

  for (const required of ['serial', 'pid', 'event', 'apkSha256', 'out']) {
    if (!options[required]) fail(`필수 옵션이 없습니다: ${required}`);
  }
  for (const [name, value] of Object.entries({ serial: options.serial, adb: options.adb, appId: options.appId, out: options.out })) {
    if (!String(value).trim() || /[\u0000-\u001f\u007f]/.test(String(value))) fail(`${name} 값이 비어 있거나 제어 문자를 포함합니다`);
  }
  if (!/^[A-Za-z0-9_]+(?:\.[A-Za-z0-9_]+)+$/.test(options.appId)) fail('--app-id는 Android 패키지 이름이어야 합니다');
  if (!/^\d+$/.test(options.pid) || Number(options.pid) < 1) fail('--pid는 양의 정수여야 합니다');
  if (!/^\/dev\/input\/event\d+$/.test(options.event)) fail('--event는 /dev/input/eventN 형식이어야 합니다');
  if (!/^[a-f0-9]{64}$/.test(options.apkSha256)) fail('--apk-sha256은 소문자 SHA-256 64자리여야 합니다');
  for (const key of ['firstTimeoutSeconds', 'touchTimeoutSeconds', 'contactLimit', 'drainSeconds', 'maxProcessAgeSeconds']) {
    const value = options[key];
    if (value !== undefined && !/^\d+$/.test(String(value))) fail(`${key}는 0 이상의 정수여야 합니다`);
    options[key] = Number(value);
    if (!Number.isSafeInteger(options[key])) fail(`${key}가 안전한 정수 범위를 벗어났습니다`);
  }
  if (options.firstTimeoutSeconds < 1 || options.touchTimeoutSeconds < 1) fail('접촉 timeout은 1초 이상이어야 합니다');
  if (options.contactLimit < 1 || options.contactLimit > 300) fail('--contacts는 1~300 범위여야 합니다');
  if (options.drainSeconds > 10) fail('--drain-seconds는 0~10 범위여야 합니다');
  if (options.maxProcessAgeSeconds < 1) fail('--max-process-age는 1초 이상이어야 합니다');
  if (!options.confirmR05Screen) fail('R05 target 화면을 눈으로 확인한 뒤 --confirm-r05-screen을 지정해야 합니다');
  options.pid = Number(options.pid);
  options.out = resolve(options.out);
  return options;
}

export function validateDeviceList(output, expectedSerial) {
  const entries = output.split(/\r?\n/).slice(1).map((line) => line.trim()).filter(Boolean);
  const devices = entries.map((line) => line.split(/\s+/));
  if (devices.length !== 1 || devices[0][0] !== expectedSerial || devices[0][1] !== 'device') {
    fail(`ADB 대상이 하나의 지정된 online 기기와 일치하지 않습니다: ${JSON.stringify(devices.map((row) => row.slice(0, 2)))}`);
  }
}

export function validateInputDevice(listing, eventPath) {
  const blocks = listing.split(/(?=^add device \d+:)/m);
  const block = blocks.find((candidate) => candidate.match(/^add device \d+:\s+(\S+)/m)?.[1] === eventPath);
  if (!block) fail(`getevent 목록에 지정 경로가 없습니다: ${eventPath}`);
  for (const required of ['"sec_touchscreen"', 'INPUT_PROP_DIRECT', 'ABS_MT_TRACKING_ID']) {
    if (!block.includes(required)) fail(`지정 input node가 예상한 direct touchscreen이 아닙니다: ${required}`);
  }
  return block;
}

export function processAgeSeconds(procStat, uptime, clockTicks) {
  const rightParen = procStat.lastIndexOf(')');
  if (rightParen < 0) fail('proc stat 형식이 올바르지 않습니다');
  const fields = procStat.slice(rightParen + 1).trim().split(/\s+/);
  const startTicks = BigInt(fields[19] || '');
  const ticks = BigInt(clockTicks.trim());
  const uptimeSeconds = Number.parseFloat(uptime.trim().split(/\s+/)[0]);
  if (!Number.isFinite(uptimeSeconds) || ticks <= 0n) fail('process age 기준을 확인할 수 없습니다');
  return uptimeSeconds - Number(startTicks) / Number(ticks);
}

function parseHostIdentity(output) {
  const match = output.trim().match(/^\s*(\d+)\s+(\d+)\s+(.*)$/);
  if (!match) return null;
  const tokens = match[3].trim().split(/\s+/);
  if (tokens.length < 6) return null;
  return {
    pid: Number(match[1]),
    parentPid: Number(match[2]),
    startedAt: tokens.slice(0, 5).join(' '),
    command: tokens.slice(5).join(' '),
  };
}

function serialize(value) {
  return JSON.stringify(value, (_key, item) => typeof item === 'bigint' ? item.toString() : item, 2) + '\n';
}

async function atomicWriteJson(path, value) {
  const temporary = `${path}.tmp`;
  await writeFile(temporary, serialize(value), { flag: 'wx' });
  await rename(temporary, path);
}

async function runCommand(adb, serial, args, options = {}) {
  const { stdout, stderr } = await execFileAsync(adb, ['-s', serial, ...args], {
    encoding: options.encoding ?? 'utf8',
    maxBuffer: options.maxBuffer ?? 24 * 1024 * 1024,
    timeout: options.timeout ?? 10_000,
    killSignal: 'SIGKILL',
    windowsHide: true,
  });
  return { stdout, stderr };
}

async function captureCommand(adb, serial, args, directory, name, options = {}) {
  const result = await runCommand(adb, serial, args, options);
  if (options.encoding === 'buffer') {
    await writeFile(join(directory, name), result.stdout);
    return result.stdout;
  }
  await writeFile(join(directory, name), result.stdout);
  if (result.stderr) await writeFile(join(directory, `${name}.stderr`), result.stderr);
  return result.stdout;
}

async function readIdentity(pid) {
  try {
    const { stdout } = await execFileAsync('ps', ['-p', String(pid), '-o', 'pid=,ppid=,lstart=,command='], {
      encoding: 'utf8', windowsHide: true,
    });
    const identity = parseHostIdentity(stdout);
    return identity?.pid === pid ? identity : null;
  } catch {
    return null;
  }
}

function childExited(child) {
  return child.exitCode !== null || child.signalCode !== null;
}

function waitForSpawn(child) {
  if (child.pid) return Promise.resolve();
  return new Promise((resolveSpawn, rejectSpawn) => {
    child.once('spawn', resolveSpawn);
    child.once('error', rejectSpawn);
  });
}

function waitForChildExit(child, timeoutMs) {
  if (childExited(child)) return Promise.resolve(true);
  if (timeoutMs === 0) return Promise.resolve(false);
  return new Promise((resolveWait) => {
    const timer = setTimeout(() => finish(false), timeoutMs);
    const onClose = () => finish(true);
    function finish(exited) {
      clearTimeout(timer);
      child.removeListener('close', onClose);
      resolveWait(exited || childExited(child));
    }
    child.once('close', onClose);
  });
}

export function parseRemoteGeteventProcesses(psOutput) {
  return psOutput.split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => /(?:^|[\s/])getevent(?:\s|$)/.test(line));
}

function sha256(buffer) {
  return createHash('sha256').update(buffer).digest('hex');
}

function isPngImage(buffer) {
  return Buffer.isBuffer(buffer)
    && buffer.length >= 45
    && buffer.subarray(0, 8).toString('hex') === '89504e470d0a1a0a'
    && buffer.readUInt32BE(8) === 13
    && buffer.subarray(12, 16).toString('ascii') === 'IHDR'
    && buffer.readUInt32BE(16) > 0
    && buffer.readUInt32BE(20) > 0
    && buffer.includes(Buffer.from('IEND'));
}

async function fileSha256(path) {
  return sha256(await readFile(path));
}

async function writeChecksums(directory, names) {
  const lines = [];
  for (const name of names.sort()) lines.push(`${await fileSha256(join(directory, name))}  ${name}`);
  await writeFile(join(directory, 'SHA256SUMS'), `${lines.join('\n')}\n`);
}

async function createRunDirectory(path) {
  await mkdir(dirname(path), { recursive: true });
  await mkdir(path, { recursive: false });
}

async function preflight(options, directory, manifest) {
  Object.assign(manifest, {
    schema: 'spinon-r05-android-physical-touch-capture-v1',
    startedUtc: new Date().toISOString(),
    deviceSerial: options.serial,
    appId: options.appId,
    appPid: options.pid,
    inputEventPath: options.event,
    expectedApkSha256: options.apkSha256,
    policy: {
      firstContactTimeoutSeconds: options.firstTimeoutSeconds,
      afterFirstContactTimeoutSeconds: options.touchTimeoutSeconds,
      completeContactLimit: options.contactLimit,
      drainSeconds: options.drainSeconds,
      maxProcessAgeSeconds: options.maxProcessAgeSeconds,
      hostStopEscalation: ['SIGINT', 'SIGTERM', 'SIGKILL'],
      stopStepTimeoutMs: HOST_STOP_TIMEOUT_MS,
    },
    host: { platform: process.platform, node: process.version, parentPid: process.ppid },
    collectors: [],
  });

  const deviceList = await execFileAsync(options.adb, ['devices', '-l'], { encoding: 'utf8', windowsHide: true, timeout: 10_000, killSignal: 'SIGKILL' });
  const devices = deviceList.stdout;
  await writeFile(join(directory, 'adb-devices.txt'), devices);
  if (deviceList.stderr) await writeFile(join(directory, 'adb-devices.stderr'), deviceList.stderr);
  validateDeviceList(devices, options.serial);
  const state = await captureCommand(options.adb, options.serial, ['get-state'], directory, 'adb-state.txt');
  if (state.trim() !== 'device') fail(`ADB 기기 상태가 online이 아닙니다: ${state.trim()}`);

  const pidOutput = await captureCommand(options.adb, options.serial, ['shell', 'pidof', options.appId], directory, 'app-pid.txt');
  const pids = pidOutput.trim().split(/\s+/).filter(Boolean).map(Number);
  if (pids.length !== 1 || pids[0] !== options.pid) fail(`foreground 앱 PID와 지정 PID가 다릅니다: ${pids.join(',') || '없음'}`);

  const uptime = await captureCommand(options.adb, options.serial, ['shell', 'cat', '/proc/uptime'], directory, 'device-uptime-before.txt');
  const stat = await captureCommand(options.adb, options.serial, ['shell', 'cat', `/proc/${options.pid}/stat`], directory, 'app-proc-stat.txt');
  const ticks = await captureCommand(options.adb, options.serial, ['shell', 'getconf', 'CLK_TCK'], directory, 'device-clock-ticks.txt');
  const ageSeconds = processAgeSeconds(stat, uptime, ticks);
  manifest.appProcessAgeSeconds = ageSeconds;
  if (ageSeconds < 0 || ageSeconds > options.maxProcessAgeSeconds) {
    fail(`앱 process가 fresh 조건을 벗어났습니다: ${ageSeconds.toFixed(3)}초`);
  }

  const paths = await captureCommand(options.adb, options.serial, ['shell', 'pm', 'path', options.appId], directory, 'installed-apk-paths.txt');
  const apkPaths = paths.split(/\r?\n/).map((line) => line.trim().replace(/^package:/, '')).filter(Boolean);
  if (apkPaths.length !== 1 || !apkPaths[0].endsWith('/base.apk')) fail(`설치 APK 경로가 단일 base.apk가 아닙니다: ${apkPaths.length}개`);
  const installedHashOutput = await captureCommand(options.adb, options.serial, ['shell', 'sha256sum', apkPaths[0]], directory, 'installed-apk-sha256.txt');
  const installedHash = installedHashOutput.trim().split(/\s+/)[0];
  manifest.installedApkSha256 = installedHash;
  if (installedHash !== options.apkSha256) fail('설치 APK SHA-256이 고정한 값과 다릅니다');

  const inputList = await captureCommand(options.adb, options.serial, ['shell', 'getevent', '-lp'], directory, 'input-devices.txt');
  validateInputDevice(inputList, options.event);
  const inputState = await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'input'], directory, 'input-state-before.txt');
  if (!inputState.includes('RawToDisplay Transform') || !inputState.includes('sec_touchscreen')) {
    fail('dumpsys input에서 touchscreen viewport 또는 raw 변환을 확인할 수 없습니다');
  }
  await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'display'], directory, 'display-before.txt');
  await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'thermalservice'], directory, 'thermal-before.txt');
  await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'battery'], directory, 'battery-before.txt');
  const activities = await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'activity', 'activities'], directory, 'activity-before.txt');
  const resumedActivity = activities.split(/\r?\n/).find((line) =>
    /(?:mResumedActivity|topResumedActivity)\s*[:=]/.test(line) && line.includes(options.appId));
  if (!resumedActivity) fail('Spinon 앱이 foreground resumed activity가 아닙니다');
  manifest.resumedActivity = resumedActivity.trim();
  await captureCommand(options.adb, options.serial, ['shell', 'dumpsys', 'window', 'windows'], directory, 'windows-before.txt');

  const prestartLog = await captureCommand(options.adb, options.serial, ['logcat', '-d', `--pid=${options.pid}`, '-v', 'threadtime'], directory, 'app-log-prestart.txt');
  if (!prestartLog.includes(R05_MARKER)) fail(`R05 실험 시작 marker가 PID ${options.pid} 로그에 없습니다`);
  const screenshot = await runCommand(options.adb, options.serial, ['exec-out', 'screencap', '-p'], { encoding: 'buffer' });
  if (!isPngImage(screenshot.stdout)) {
    fail('시작 화면 캡처가 PNG가 아닙니다');
  }
  await writeFile(join(directory, 'screen-before.png'), screenshot.stdout);

  const remoteProcesses = await captureCommand(options.adb, options.serial, ['shell', 'ps', '-A', '-o', 'PID,ARGS'], directory, 'device-processes-before.txt');
  const existingGetevent = parseRemoteGeteventProcesses(remoteProcesses);
  if (existingGetevent.length) fail(`수집 시작 전에 getevent process가 이미 있습니다: ${existingGetevent.length}개`);
  await captureCommand(options.adb, options.serial, ['shell', 'cat', '/proc/uptime'], directory, 'device-uptime-prestart.txt');
  manifest.preflightCompleteUtc = new Date().toISOString();
  return manifest;
}

function processLineBuffer(decoder, buffer, chunk, onLine, onOverlongLine) {
  const text = buffer + decoder.write(chunk);
  const lines = text.split('\n');
  const remainder = lines.pop() ?? '';
  for (const line of lines) {
    const normalized = line.replace(/\r$/, '');
    if (Buffer.byteLength(normalized) > MAX_RAW_LINE_BYTES) onOverlongLine(Buffer.byteLength(normalized));
    else onLine(normalized);
  }
  if (Buffer.byteLength(remainder) > MAX_RAW_LINE_BYTES) onOverlongLine(Buffer.byteLength(remainder));
  return remainder;
}

async function runCapture(options, directory, manifest) {
  const rawPath = join(directory, 'raw-touch.log');
  const appPath = join(directory, 'app.log');
  const rawErrorPath = join(directory, 'getevent-stderr.txt');
  const appErrorPath = join(directory, 'logcat-stderr.txt');
  const rawOutput = createWriteStream(rawPath, { flags: 'wx' });
  const appOutput = createWriteStream(appPath, { flags: 'wx' });
  const rawErrors = createWriteStream(rawErrorPath, { flags: 'wx' });
  const appErrors = createWriteStream(appErrorPath, { flags: 'wx' });
  const rawCounter = new RawContactCounter();
  const rawDecoder = new StringDecoder('utf8');
  let rawLineBuffer = '';
  let firstContactTimer;
  let completionTimer;
  let stopPromise;
  let stopReason = null;
  let captureError = null;
  let firstContactUtc = null;
  let rawChild;
  let appChild;
  let rawIdentity;
  let appIdentity;
  let captureArmed = false;
  let deadlineState;
  let onInterrupt;

  const streamError = (error) => {
    captureError ??= String(error?.message || error);
    if (stopPromise) return;
    if (rawChild && appChild && rawIdentity && appIdentity) void stop('collector_error');
  };
  for (const stream of [rawOutput, appOutput, rawErrors, appErrors]) stream.on('error', streamError);

  const recordCollector = async (label, child, command, spawnReady) => {
    await spawnReady;
    if (!child.pid) fail(`host ${label} collector가 PID를 만들지 못했습니다`);
    const identity = await readIdentity(child.pid);
    if (!identity) fail(`host ${label} collector process identity를 읽지 못했습니다`);
    manifest.collectors.push({ label, pid: child.pid, ...identity, command, startUtc: new Date().toISOString() });
    return identity;
  };

  const handleRawLine = (line) => {
    if (!captureArmed) return;
    const result = rawCounter.consume(line);
    if (result.firstContact && !firstContactUtc) {
      firstContactUtc = new Date().toISOString();
      manifest.firstRawContactUtc = firstContactUtc;
      deadlineState.noteContact(performance.now());
      clearTimeout(firstContactTimer);
      firstContactTimer = setTimeout(() => void stop(deadlineState.reasonAt(performance.now()) || 'timeout_after_first_contact'), options.touchTimeoutSeconds * 1_000);
    }
    if (result.completedContacts >= options.contactLimit && !completionTimer) {
      manifest.contactLimitReachedUtc = new Date().toISOString();
      completionTimer = setTimeout(() => void stop('contact_limit'), options.drainSeconds * 1_000);
    }
  };

  const rawCommand = [options.adb, '-s', options.serial, 'shell', 'getevent', '-lt', options.event];
  const appCommand = [options.adb, '-s', options.serial, 'logcat', `--pid=${options.pid}`, '-v', 'threadtime'];
  let startupComplete = false;
  let interruptRequested = false;
  onInterrupt = () => {
    if (!startupComplete) {
      interruptRequested = true;
      return;
    }
    void stop('operator_interrupt');
  };
  process.on('SIGINT', onInterrupt);
  process.on('SIGTERM', onInterrupt);
  rawChild = spawn(options.adb, rawCommand.slice(1), { stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
  const rawSpawnReady = waitForSpawn(rawChild);
  rawChild.on('error', streamError);
  appChild = spawn(options.adb, appCommand.slice(1), { stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
  const appSpawnReady = waitForSpawn(appChild);
  appChild.on('error', streamError);
  try {
    rawIdentity = await recordCollector('getevent', rawChild, rawCommand, rawSpawnReady);
    appIdentity = await recordCollector('logcat', appChild, appCommand, appSpawnReady);
  } catch (error) {
    captureError = String(error?.message || error);
    const spawned = [[rawChild, rawIdentity], [appChild, appIdentity]];
    for (const [child, knownIdentity] of spawned) {
      if (!child?.pid || childExited(child)) continue;
      const identity = knownIdentity || await readIdentity(child.pid);
      if (!identity) {
        captureError += `; ${child.pid} process identity unavailable`;
        continue;
      }
      await stopChildBounded(child, identity, { readIdentity, waitForExit: waitForChildExit, stepTimeoutMs: HOST_STOP_TIMEOUT_MS })
        .catch((stopError) => { captureError += `; ${String(stopError)}`; });
    }
    for (const stream of [rawOutput, appOutput, rawErrors, appErrors]) stream.end();
    await Promise.allSettled([rawOutput, appOutput, rawErrors, appErrors].map((stream) => finished(stream)));
    manifest.captureError = captureError;
    manifest.stopReason = 'collector_start_error';
    manifest.collectorExit = [rawChild, appChild].filter(Boolean).map((child) => ({ pid: child.pid, code: child.exitCode, signal: child.signalCode }));
    await atomicWriteJson(join(directory, 'capture-manifest.json'), manifest);
    process.removeListener('SIGINT', onInterrupt);
    process.removeListener('SIGTERM', onInterrupt);
    throw error;
  }
  rawChild.stdout.on('data', (chunk) => {
    if (!rawOutput.write(chunk)) rawChild.stdout.pause(), rawOutput.once('drain', () => rawChild.stdout.resume());
    rawLineBuffer = processLineBuffer(rawDecoder, rawLineBuffer, chunk, handleRawLine, (length) => {
      captureError ??= `raw getevent line exceeded ${MAX_RAW_LINE_BYTES} bytes (${length})`;
      void stop('collector_error');
    });
  });
  rawChild.stderr.on('data', (chunk) => rawErrors.write(chunk));
  appChild.stdout.on('data', (chunk) => { if (!appOutput.write(chunk)) appChild.stdout.pause(), appOutput.once('drain', () => appChild.stdout.resume()); });
  appChild.stderr.on('data', (chunk) => appErrors.write(chunk));
  for (const child of [rawChild, appChild]) {
    child.once('close', (code, signal) => {
      if (!stopPromise) {
        captureError ??= `collector exited before stop (code=${code}, signal=${signal})`;
        void stop('collector_exit');
      }
    });
  }

  await new Promise((resolveReady) => setTimeout(resolveReady, 300));
  startupComplete = true;
  if (interruptRequested || stopPromise) {
    const result = await stop('operator_interrupt');
    return result;
  }
  if (childExited(rawChild) || childExited(appChild)) {
    captureError ??= 'collector exited before GO';
    await stop('collector_exit');
    fail('collector가 GO 전에 종료됐습니다');
  }
  if (captureError) {
    const result = await stop('collector_error');
    return result;
  }

  let goUtc;
  let goMonotonicNs;
  try {
    const goUptime = await runCommand(options.adb, options.serial, ['shell', 'cat', '/proc/uptime']);
    if (stopPromise) return stopPromise;
    goUtc = new Date().toISOString();
    goMonotonicNs = process.hrtime.bigint();
    deadlineState = new CaptureDeadline({
      startAt: performance.now(),
      noContactMs: options.firstTimeoutSeconds * 1_000,
      afterFirstContactMs: options.touchTimeoutSeconds * 1_000,
    });
    manifest.goUtc = goUtc;
    manifest.hostMonotonicGoNs = goMonotonicNs;
    manifest.stopReason = null;
    manifest.analysisStatus = 'not_analyzed_join_required';
    await writeFile(join(directory, 'GO.txt'), `${goUtc}\nhost_monotonic_ns=${goMonotonicNs}\n`);
    await writeFile(join(directory, 'device-uptime-go.txt'), goUptime.stdout);
    captureArmed = true;
  } catch (error) {
    captureError ??= `GO anchor write failed: ${String(error?.message || error)}`;
    return stop('collector_error');
  }
  firstContactTimer = setTimeout(() => void stop(deadlineState.reasonAt(performance.now()) || 'prestart_no_contact'), options.firstTimeoutSeconds * 1_000);
  process.stdout.write(`수집 시작 · GO ${goUtc} · 첫 raw 접촉 ${options.firstTimeoutSeconds}초 대기 · 이후 ${options.touchTimeoutSeconds}초 · ${options.contactLimit}개 접촉에서 자동 종료\n`);

  function stop(reason) {
    if (stopPromise) return stopPromise;
    stopReason = reason;
    manifest.stopReason = reason;
    manifest.stopRequestedUtc = new Date().toISOString();
    clearTimeout(firstContactTimer);
    clearTimeout(completionTimer);
    stopPromise = (async () => {
      const stopOne = async (child, identity) => stopChildBounded(child, identity, {
        readIdentity,
        waitForExit: waitForChildExit,
        stepTimeoutMs: HOST_STOP_TIMEOUT_MS,
      });
      const results = await Promise.allSettled([
        stopOne(rawChild, rawIdentity),
        stopOne(appChild, appIdentity),
      ]);
      manifest.stopEscalation = results.map((result, index) => ({
        label: index === 0 ? 'getevent' : 'logcat',
        result: result.status === 'fulfilled' ? result.value : { error: String(result.reason) },
      }));
      if (results.some((result) => result.status === 'rejected')) captureError ??= 'host collector bounded shutdown failed';
      if (results.some((result) => result.status === 'fulfilled' && result.value.exitedBeforeSignal)) {
        captureError ??= 'host collector exited before the requested shutdown';
      }
      const finalBuffer = rawDecoder.end();
      const tail = rawLineBuffer + finalBuffer;
      if (tail) {
        rawOutput.write(tail);
        manifest.trailingRawLine = tail;
        captureError ??= 'raw stream ended with an incomplete final line';
      }
      for (const stream of [rawOutput, appOutput, rawErrors, appErrors]) stream.end();
      const streamResults = await Promise.allSettled([rawOutput, appOutput, rawErrors, appErrors].map((stream) => finished(stream)));
      if (streamResults.some((result) => result.status === 'rejected')) captureError ??= 'capture output file could not be flushed';
      try {
        const remoteAfter = await runCommand(options.adb, options.serial, ['shell', 'ps', '-A', '-o', 'PID,ARGS']);
        await writeFile(join(directory, 'device-processes-after.txt'), remoteAfter.stdout);
        const remainingGetevent = parseRemoteGeteventProcesses(remoteAfter.stdout);
        manifest.deviceGeteventProcessesAfter = remainingGetevent;
        if (remainingGetevent.length) captureError ??= `device getevent process remains: ${remainingGetevent.length}`;
      } catch (error) {
        captureError ??= `device getevent teardown could not be verified: ${String(error)}`;
      }
      for (const [args, name] of [
        [['shell', 'cat', '/proc/uptime'], 'device-uptime-after.txt'],
        [['shell', 'dumpsys', 'activity', 'activities'], 'activity-after.txt'],
      ]) {
        await captureCommand(options.adb, options.serial, args, directory, name).catch((error) => { captureError ??= String(error); });
      }
      try {
        const screenshot = await runCommand(options.adb, options.serial, ['exec-out', 'screencap', '-p'], { encoding: 'buffer' });
        if (!isPngImage(screenshot.stdout)) fail('종료 화면 캡처가 유효한 PNG가 아닙니다');
        await writeFile(join(directory, 'screen-after.png'), screenshot.stdout);
      } catch (error) {
        captureError ??= `post-capture screenshot failed: ${String(error)}`;
      }
      manifest.raw = {
        completedContactGroups: rawCounter.completedContacts,
        activeContactsAtStop: rawCounter.activeContacts,
        pendingReleaseFramesAtStop: rawCounter.pendingCompletions,
        overlappingContactReports: rawCounter.overlapReports,
        protocolErrors: rawCounter.protocolErrors,
      };
      manifest.captureError = captureError;
      manifest.stoppedUtc = new Date().toISOString();
      manifest.hostMonotonicStopNs = process.hrtime.bigint();
      manifest.collectorExit = [rawChild, appChild].map((child, index) => ({
        label: index === 0 ? 'getevent' : 'logcat',
        code: child.exitCode,
        signal: child.signalCode,
      }));
      manifest.captureWindowValid = captureWindowIsValid({
        stopReason,
        completedContacts: rawCounter.completedContacts,
        contactLimit: options.contactLimit,
        captureError,
        activeContacts: rawCounter.activeContacts,
        pendingReleaseFrames: rawCounter.pendingCompletions,
        protocolErrors: rawCounter.protocolErrors,
      });
      manifest.captureStatus = manifest.captureWindowValid
        ? 'raw_capture_complete_pending_join_validation'
        : stopReason === 'prestart_no_contact'
          ? 'no_contact_timeout'
          : captureError
            ? 'capture_or_cleanup_failed'
            : 'capture_incomplete';
      await atomicWriteJson(join(directory, 'capture-manifest.json'), manifest);
      const entries = await readdir(directory, { withFileTypes: true });
      const existingChecksumNames = entries
        .filter((entry) => entry.isFile() && entry.name !== 'SHA256SUMS' && entry.name !== 'capture-manifest.json' && !entry.name.endsWith('.tmp'))
        .map((entry) => entry.name);
      await writeChecksums(directory, existingChecksumNames).catch((error) => { captureError ??= `checksum generation failed: ${String(error)}`; });
      manifest.captureError = captureError;
      manifest.captureWindowValid = manifest.captureWindowValid && !captureError;
      await atomicWriteJson(join(directory, 'capture-manifest.json'), manifest);
      process.removeListener('SIGINT', onInterrupt);
      process.removeListener('SIGTERM', onInterrupt);
      return { reason: stopReason, captureError, manifest };
    })();
    return stopPromise;
  }

  const result = await new Promise((resolveResult) => {
    const poll = setInterval(() => {
      if (stopPromise) {
        clearInterval(poll);
        stopPromise.then(resolveResult, (error) => resolveResult({ reason: stopReason, captureError: String(error), manifest }));
      }
    }, 50);
  });
  return result;
}

export async function captureAndroidPhysicalTouches(options) {
  await createRunDirectory(options.out);
  const manifest = {
    schema: 'spinon-r05-android-physical-touch-capture-v1',
    startedUtc: new Date().toISOString(),
    deviceSerial: options.serial,
    appId: options.appId,
    appPid: options.pid,
    inputEventPath: options.event,
    expectedApkSha256: options.apkSha256,
  };
  try {
    await preflight(options, options.out, manifest);
    const result = await runCapture(options, options.out, manifest);
    if (result.captureError) fail(`수집/종료 실패: ${result.captureError}`);
    if (result.reason === 'prestart_no_contact') fail('prestart_no_contact: raw direct touch가 제한 시간 안에 없었습니다');
    if (result.reason === 'timeout_after_first_contact') fail('timeout_after_first_contact: 제한 시간 안에 목표 contact 수를 채우지 못했습니다');
    if (result.reason === 'operator_interrupt') fail('operator_interrupt: 사용자/운영자가 중단한 미완료 수집입니다');
    if (result.reason === 'collector_exit' || result.reason === 'collector_error') fail(`collector 중단: ${result.reason}`);
    if (!result.manifest.captureWindowValid) fail('capture window가 규약을 만족하지 않아 표본 수집에 사용할 수 없습니다');
    process.stdout.write(`collector 종료 · 이유=${result.reason} · raw contact groups=${result.manifest.raw.completedContactGroups}\n`);
    return result;
  } catch (error) {
    if (manifest?.preflightCompleteUtc) manifest.runError = String(error?.message || error);
    else manifest.preflightError = String(error?.message || error);
    if (manifest) await atomicWriteJson(join(options.out, 'capture-manifest.json'), manifest).catch(() => {});
    throw error;
  }
}

function usage() {
  return `사용법:\n  mise exec -- node tools/benchmark/capture-android-physical-touch.mjs --serial <ADB serial> --pid <fresh 앱 PID> --event /dev/input/eventN --apk-sha256 <64자리 SHA-256> --out <새 결과 폴더> --confirm-r05-screen\n\nR05 target 화면을 눈으로 확인한 뒤 실행합니다. raw touch와 PID 한정 logcat을 동시에 저장합니다. 첫 입력 전 제한 시간은 60초, 첫 입력 뒤 제한 시간은 60초이며 raw contact ${DEFAULT_CONTACT_LIMIT}개가 완결되면 ${DEFAULT_DRAIN_SECONDS}초 drain 뒤 자동 종료합니다.\n`;
}

async function main() {
  try {
    const options = parseOptions(process.argv.slice(2));
    if (options.help) {
      process.stdout.write(usage());
      return;
    }
    await captureAndroidPhysicalTouches(options);
  } catch (error) {
    process.stderr.write(`오류: ${error?.message || error}\n`);
    process.exitCode = 1;
  }
}

  if (import.meta.url === pathToFileURL(resolve(process.argv[1])).href) await main();
