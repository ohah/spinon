import assert from 'node:assert/strict';
import { chmod, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import {
  captureWindowIsValid,
  CaptureDeadline,
  RawContactCounter,
  parseGeteventLine,
  signalSteps,
  stopChildBounded,
} from './android-physical-touch-capture.mjs';
import {
  parseOptions,
  parseRemoteGeteventProcesses,
  processAgeSeconds,
  validateDeviceList,
  validateInputDevice,
  captureAndroidPhysicalTouches,
} from './capture-android-physical-touch.mjs';

test('getevent timestamp를 표현 해상도 그대로 nanosecond 정수로 바꾼다', () => {
  const parsed = parseGeteventLine('[ 1504357.934514] EV_ABS ABS_MT_TRACKING_ID 00004a52');
  assert.equal(parsed.timestampNs, 1_504_357_934_514_000n);
  assert.equal(parsed.type, 'ABS');
  assert.equal(parsed.value, 0x4a52);
  assert.equal(parseGeteventLine('[ 1504358.050479] EV_ABS ABS_MT_TRACKING_ID ffffffff').value, -1);
  assert.equal(parseGeteventLine('noise'), null);
  assert.equal(parseGeteventLine('[ 1504358.0504790001] EV_ABS ABS_MT_TRACKING_ID 00000001'), null);
});

test('tracking ID down/up은 SYN_REPORT 뒤에만 완결 접촉으로 센다', () => {
  const counter = new RawContactCounter();
  assert.equal(counter.consume('[ 1.000001] EV_ABS ABS_MT_TRACKING_ID 00000001').firstContact, true);
  counter.consume('[ 1.010001] EV_ABS ABS_MT_TRACKING_ID ffffffff');
  assert.equal(counter.completedContacts, 0);
  counter.consume('[ 1.010002] EV_SYN SYN_REPORT 00000000');
  assert.equal(counter.completedContacts, 1);
  assert.equal(counter.activeContacts, 0);
  assert.deepEqual(counter.protocolErrors, []);
  counter.consume('[ 1.010003] EV_ABS ABS_MT_TRACKING_ID 00000002');
  counter.consume('[ 1.010004] EV_ABS ABS_MT_TRACKING_ID ffffffff');
  assert.equal(counter.pendingCompletions, 1);
});

test('슬롯별 멀티포인터와 겹침을 보존하고 잘못된 release를 숨기지 않는다', () => {
  const counter = new RawContactCounter();
  counter.consume('[ 1.000001] EV_ABS ABS_MT_SLOT 00000000');
  counter.consume('[ 1.000002] EV_ABS ABS_MT_TRACKING_ID 00000001');
  counter.consume('[ 1.000003] EV_ABS ABS_MT_SLOT 00000001');
  counter.consume('[ 1.000004] EV_ABS ABS_MT_TRACKING_ID 00000002');
  assert.equal(counter.activeContacts, 2);
  assert.equal(counter.overlapReports, 1);
  counter.consume('[ 1.000005] EV_ABS ABS_MT_SLOT 00000001');
  counter.consume('[ 1.000005] EV_ABS ABS_MT_TRACKING_ID ffffffff');
  counter.consume('[ 1.000006] EV_ABS ABS_MT_SLOT 00000000');
  counter.consume('[ 1.000006] EV_ABS ABS_MT_TRACKING_ID ffffffff');
  counter.consume('[ 1.000007] EV_SYN SYN_REPORT 00000000');
  assert.equal(counter.completedContacts, 2);
  counter.consume('[ 1.000008] EV_ABS ABS_MT_TRACKING_ID ffffffff');
  assert.equal(counter.protocolErrors.length, 1);
});

test('첫 raw contact가 오기 전 timeout과 접촉 후 timeout을 따로 계산한다', () => {
  const deadline = new CaptureDeadline({ startAt: 100, noContactMs: 60, afterFirstContactMs: 70 });
  assert.equal(deadline.reasonAt(159), null);
  assert.equal(deadline.reasonAt(160), 'prestart_no_contact');
  deadline.noteContact(150);
  deadline.noteContact(155);
  assert.equal(deadline.firstContactAt, 150);
  assert.equal(deadline.reasonAt(219), null);
  assert.equal(deadline.reasonAt(220), 'timeout_after_first_contact');
});

test('성공 capture gate는 정상 종료·접촉 수·완결 frame·오류 부재를 모두 요구한다', () => {
  const base = {
    stopReason: 'contact_limit', completedContacts: 30, contactLimit: 30,
    captureError: null, activeContacts: 0, pendingReleaseFrames: 0, protocolErrors: [],
  };
  assert.equal(captureWindowIsValid(base), true);
  for (const change of [
    { stopReason: 'operator_interrupt' },
    { completedContacts: 29 },
    { captureError: 'collector failed' },
    { activeContacts: 1 },
    { pendingReleaseFrames: 1 },
    { protocolErrors: ['release_without_active_contact'] },
  ]) assert.equal(captureWindowIsValid({ ...base, ...change }), false);
});

test('수집기 인자는 필수 provenance와 화면 확인을 요구한다', () => {
  assert.throws(() => parseOptions([]), /serial/);
  const base = ['--serial', 'device-1', '--pid', '42', '--event', '/dev/input/event6', '--apk-sha256', 'a'.repeat(64), '--out', '/tmp/capture'];
  assert.throws(() => parseOptions(base), /confirm-r05-screen/);
  assert.throws(() => parseOptions([...base, '--confirm-r05-screen', '--pid', '43']), /두 번/);
  assert.throws(() => parseOptions([...base, '--confirm-r05-screen', '--touch-timeout', '999999999999999999999999']), /안전한 정수/);
  const invalidEvent = ['--serial', 'device-1', '--pid', '42', '--event', '/tmp/event', '--apk-sha256', 'a'.repeat(64), '--out', '/tmp/capture', '--confirm-r05-screen'];
  assert.throws(() => parseOptions(invalidEvent), /eventN/);
  assert.throws(() => parseOptions([...base, '--confirm-r05-screen', '--app-id', 'not-a-package']), /패키지 이름/);
  assert.equal(parseOptions([...base, '--confirm-r05-screen']).contactLimit, 30);
});

test('ADB 기기 목록에 지정된 online 기기 하나만 허용한다', () => {
  validateDeviceList('List of devices attached\ndevice-1\tdevice product:x\n', 'device-1');
  assert.throws(() => validateDeviceList('List of devices attached\ndevice-1\tdevice\ndevice-2\tdevice\n', 'device-1'), /일치하지 않습니다/);
  assert.throws(() => validateDeviceList('List of devices attached\ndevice-1\toffline\n', 'device-1'), /일치하지 않습니다/);
});

test('입력 노드 경로·이름·direct 속성과 MT tracking capability를 모두 확인한다', () => {
  const listing = 'add device 1: /dev/input/event6\n  name: "sec_touchscreen"\n  ABS_MT_TRACKING_ID\n  input props:\n    INPUT_PROP_DIRECT\n';
  assert.match(validateInputDevice(listing, '/dev/input/event6'), /sec_touchscreen/);
  assert.throws(() => validateInputDevice(listing.replace('/dev/input/event6', '/dev/input/event60'), '/dev/input/event6'), /경로/);
  assert.throws(() => validateInputDevice(listing, '/dev/input/event5'), /경로/);
  assert.throws(() => validateInputDevice(listing.replace('INPUT_PROP_DIRECT', 'INPUT_PROP_POINTER'), '/dev/input/event6'), /INPUT_PROP_DIRECT/);
});

test('process 시작 시각은 괄호 안 command name의 공백을 보존해 계산한다', () => {
  const fields = ['S', '1', ...Array(17).fill('0'), '200'];
  assert.equal(fields.length, 20);
  assert.equal(processAgeSeconds(`123 (spinon bootstrap) ${fields.join(' ')}`, '5.0 0.0', '100'), 3);
});

test('원격 getevent 잔존 검사는 절대 경로와 단독 명령을 찾고 비슷한 이름은 무시한다', () => {
  const output = 'PID ARGS\n1 init\n12 /system/bin/getevent -lt /dev/input/event6\n13 mygetevent-helper\n';
  assert.deepEqual(parseRemoteGeteventProcesses(output), ['12 /system/bin/getevent -lt /dev/input/event6']);
});

test('host collector는 identity가 그대로인 자체 PID에만 SIGINT→TERM→KILL을 보낸다', async () => {
  const child = { pid: 73, exitCode: null, signalCode: null };
  const identity = { pid: 73, parentPid: 1, startedAt: 'Fri Oct 9 12:00:00 2026', command: 'adb -s device-1 shell getevent' };
  const signals = [];
  let waits = 0;
  const result = await stopChildBounded(child, identity, {
    readIdentity: async () => identity,
    sendSignal: (signal) => signals.push(signal),
    waitForExit: async (_child, timeout) => timeout === 0 ? false : (++waits === 3),
    stepTimeoutMs: 20,
  });
  assert.deepEqual(signals, ['SIGINT', 'SIGTERM', 'SIGKILL']);
  assert.deepEqual(result.escalation, signalSteps);
  assert.equal(result.exited, true);
});

test('host PID 재사용 의심 identity 변경이면 signal을 보내지 않는다', async () => {
  const identity = { pid: 73, parentPid: 1, startedAt: 'Fri Oct 9 12:00:00 2026', command: 'adb getevent' };
  const signals = [];
  await assert.rejects(stopChildBounded({ pid: 73 }, identity, {
    readIdentity: async () => ({ ...identity, command: 'unrelated process' }),
    sendSignal: (signal) => signals.push(signal),
    waitForExit: async () => false,
  }), /identity changed/);
  assert.deepEqual(signals, []);
});

test('SIGKILL 뒤 bounded wait가 끝나도 살아 있는 collector를 성공 처리하지 않는다', async () => {
  const identity = { pid: 73, parentPid: 1, startedAt: 'Fri Oct 9 12:00:00 2026', command: 'adb getevent' };
  await assert.rejects(stopChildBounded({ pid: 73 }, identity, {
    readIdentity: async () => identity,
    sendSignal: () => {},
    waitForExit: async () => false,
  }), /SIGKILL bounded wait/);
});

test('이미 종료한 child에는 불필요한 signal을 보내지 않는다', async () => {
  const signals = [];
  const result = await stopChildBounded({ pid: 73, exitCode: 130, signalCode: null }, {
    pid: 73, parentPid: 1, startedAt: 'Fri Oct 9 12:00:00 2026', command: 'adb getevent',
  }, {
    readIdentity: async () => null,
    sendSignal: (signal) => signals.push(signal),
    waitForExit: async () => true,
  });
  assert.deepEqual(signals, []);
  assert.equal(result.exited, true);
});

test('가짜 ADB에서 두 stream 동시 수집, 접촉 제한 자동종료, manifest와 checksum을 확인한다', { timeout: 15_000 }, async () => {
  const directory = await mkdtemp(join(tmpdir(), 'spinon-r05-capture-test-'));
  const fakeAdbPath = join(directory, 'adb');
  const outputPath = join(directory, 'capture');
  const apkHash = 'a'.repeat(64);
  const fakeAdb = `#!/usr/bin/env node
const args = process.argv.slice(2);
const command = args[0] === '-s' ? args.slice(2) : args;
process.on('SIGINT', () => process.exit(130));
process.on('SIGTERM', () => process.exit(143));
const print = (value) => process.stdout.write(value);
if (command[0] === 'devices') print('List of devices attached\\nfake-device\\tdevice product:test\\n');
else if (command[0] === 'get-state') print('device\\n');
else if (command[0] === 'exec-out') process.stdout.write(Buffer.from('89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000b49444154789c636000020000050001a5f645400000000049454e44ae426082', 'hex'));
else if (command[0] === 'logcat' && command.includes('-d')) print('I SpinonBootstrap: SPINON_R05_FENCE_WAIT_START api=36\\n');
else if (command[0] === 'logcat') setInterval(() => {}, 1000);
else if (command[0] === 'shell' && command[1] === 'pidof') print('42\\n');
else if (command[0] === 'shell' && command[1] === 'cat' && command[2] === '/proc/uptime') print('10.0 0.0\\n');
else if (command[0] === 'shell' && command[1] === 'cat' && command[2] === '/proc/42/stat') print('42 (fake app) S 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 100\\n');
else if (command[0] === 'shell' && command[1] === 'getconf') print('100\\n');
else if (command[0] === 'shell' && command[1] === 'pm') print('package:/data/app/fake/base.apk\\n');
else if (command[0] === 'shell' && command[1] === 'sha256sum') print('${apkHash}  /data/app/fake/base.apk\\n');
else if (command[0] === 'shell' && command[1] === 'getevent' && command[2] === '-lp') print('add device 1: /dev/input/event6\\n  name:     "sec_touchscreen"\\n  ABS_MT_TRACKING_ID\\n  input props:\\n    INPUT_PROP_DIRECT\\n');
else if (command[0] === 'shell' && command[1] === 'getevent' && command[2] === '-lt') {
  setTimeout(() => {
    let time = 1;
    for (let id = 1; id <= 2; id += 1) {
      print('[ 1.' + String(time).padStart(6, '0') + '] EV_ABS ABS_MT_TRACKING_ID 0000000' + id + '\\n'); time += 1;
      print('[ 1.' + String(time).padStart(6, '0') + '] EV_SYN SYN_REPORT 00000000\\n'); time += 1;
      print('[ 1.' + String(time).padStart(6, '0') + '] EV_ABS ABS_MT_TRACKING_ID ffffffff\\n'); time += 1;
      print('[ 1.' + String(time).padStart(6, '0') + '] EV_SYN SYN_REPORT 00000000\\n'); time += 1;
    }
  }, 500);
  setInterval(() => {}, 1000);
}
else if (command[0] === 'shell' && command[1] === 'dumpsys' && command[2] === 'input') print('sec_touchscreen\\nRawToDisplay Transform\\n');
else if (command[0] === 'shell' && command[1] === 'dumpsys' && command[2] === 'activity') print('topResumedActivity=ActivityRecord dev.spinon.bootstrap/.MainActivity\\n');
else if (command[0] === 'shell' && command[1] === 'ps') print('PID ARGS\\n1 init\\n');
else print('ok\\n');
`;
  try {
    await writeFile(fakeAdbPath, fakeAdb);
    await chmod(fakeAdbPath, 0o755);
    const options = parseOptions([
      '--adb', fakeAdbPath,
      '--serial', 'fake-device',
      '--pid', '42',
      '--event', '/dev/input/event6',
      '--apk-sha256', apkHash,
      '--out', outputPath,
      '--confirm-r05-screen',
      '--contacts', '2',
      '--drain-seconds', '0',
      '--no-contact-timeout', '3',
      '--touch-timeout', '3',
    ]);
    const result = await captureAndroidPhysicalTouches(options);
    assert.equal(result.reason, 'contact_limit');
    assert.equal(result.manifest.raw.completedContactGroups, 2);
    assert.equal(result.manifest.captureWindowValid, true);
    assert.equal(result.manifest.analysisStatus, 'not_analyzed_join_required');
    const raw = await readFile(join(outputPath, 'raw-touch.log'), 'utf8');
    assert.equal((raw.match(/ABS_MT_TRACKING_ID/g) || []).length, 4);
    const sums = await readFile(join(outputPath, 'SHA256SUMS'), 'utf8');
    assert.match(sums, /raw-touch\.log/);
    assert.match(sums, /screen-after\.png/);
    assert.ok((await readFile(join(outputPath, 'screen-before.png'))).length > 45);

    const noContactPath = join(directory, 'no-contact');
    await writeFile(fakeAdbPath, fakeAdb.replace('id <= 2', 'id <= 0'));
    const noContactOptions = parseOptions([
      '--adb', fakeAdbPath,
      '--serial', 'fake-device',
      '--pid', '42',
      '--event', '/dev/input/event6',
      '--apk-sha256', apkHash,
      '--out', noContactPath,
      '--confirm-r05-screen',
      '--contacts', '2',
      '--drain-seconds', '0',
      '--no-contact-timeout', '1',
      '--touch-timeout', '1',
    ]);
    await assert.rejects(captureAndroidPhysicalTouches(noContactOptions), /prestart_no_contact/);
    const noContactManifest = JSON.parse(await readFile(join(noContactPath, 'capture-manifest.json'), 'utf8'));
    assert.equal(noContactManifest.stopReason, 'prestart_no_contact');
    assert.equal(noContactManifest.captureStatus, 'no_contact_timeout');
    assert.equal(noContactManifest.captureWindowValid, false);
    assert.equal(noContactManifest.preflightError, undefined);
    assert.match(noContactManifest.runError, /prestart_no_contact/);
    assert.deepEqual(noContactManifest.deviceGeteventProcessesAfter, []);
    assert.deepEqual(noContactManifest.stopEscalation.map((item) => item.result.escalation), [['SIGINT'], ['SIGINT']]);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
