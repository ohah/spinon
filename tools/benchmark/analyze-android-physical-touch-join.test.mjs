import assert from 'node:assert/strict';
import test from 'node:test';
import { analyzeCapture } from './analyze-android-physical-touch-join.mjs';

function timestampText(milliseconds) {
  const seconds = Math.floor(milliseconds / 1_000);
  const microseconds = (milliseconds % 1_000) * 1_000;
  return '[ ' + seconds + '.' + String(microseconds).padStart(6, '0') + ']';
}

function timeNs(milliseconds) {
  return BigInt(milliseconds) * 1_000_000n;
}

function rawEvent(milliseconds, type, code, value) {
  return timestampText(milliseconds) + ' EV_' + type + ' ' + code + ' ' + value;
}

function contactDown(milliseconds, slot, trackingId, x = 100, y = 100) {
  return [
    rawEvent(milliseconds, 'ABS', 'ABS_MT_SLOT', String(slot).padStart(8, '0')),
    rawEvent(milliseconds, 'ABS', 'ABS_MT_TRACKING_ID', trackingId.toString(16).padStart(8, '0')),
    rawEvent(milliseconds, 'ABS', 'ABS_MT_POSITION_X', x.toString(16).padStart(8, '0')),
    rawEvent(milliseconds, 'ABS', 'ABS_MT_POSITION_Y', y.toString(16).padStart(8, '0')),
    rawEvent(milliseconds, 'SYN', 'SYN_REPORT', '00000000'),
  ];
}

function contactUp(milliseconds, slot) {
  return [
    rawEvent(milliseconds, 'ABS', 'ABS_MT_SLOT', String(slot).padStart(8, '0')),
    rawEvent(milliseconds, 'ABS', 'ABS_MT_TRACKING_ID', 'ffffffff'),
    rawEvent(milliseconds, 'SYN', 'SYN_REPORT', '00000000'),
  ];
}

function rawContacts(contacts) {
  return contacts.flatMap(({ down, up, slot = 0, id, x, y }) => [
    ...contactDown(down, slot, id, x, y),
    ...contactUp(up, slot),
  ]).join('\n') + '\n';
}

function acceptedLog({ sequence, revision = sequence, generation = 1, releaseMs, requestId = sequence }) {
  const eventNs = timeNs(releaseMs);
  const anchor = eventNs + 1_000_000n;
  const before = anchor - 1_000n;
  const after = anchor + 1_000n;
  const latch = eventNs + 10_000_000n;
  const signal = latch + 1_000_000n;
  const prefix = '10-09 13:00:00.000 100 101 I SpinonBootstrap: ';
  const row = (kind, fields) => prefix + kind + ' ' + fields;
  return [
    row('SPINON_R05_INPUT renderer=wgpu', 'input_seq=' + sequence
      + ' phase=ACTION_UP timestamp_precision=nanosecond_representation input_source=unknown'
      + ' handler_clock_resolution=nanosecond_api event_time_ns=' + eventNs
      + ' input_uptime_anchor_ns=' + anchor + ' input_monotonic_before_ns=' + before
      + ' input_monotonic_after_ns=' + after + ' generation=' + generation),
    row('SPINON_R05_SUBMIT renderer=wgpu', 'input_seq=' + sequence
      + ' revision=' + revision + ' generation=' + generation + ' event_time_ns=' + eventNs
      + ' draw_accepted=true present_signal=unavailable attribution=input'),
    row('SPINON_R05_PRESENT_FENCE_LISTENER renderer=wgpu', 'request_id=' + requestId
      + ' input_seq=' + sequence + ' revision=' + revision + ' generation=' + generation
      + ' target_vsync_id=-1 state=registered timeout_ms=2000'),
    row('SPINON_R05_PRESENT_FENCE_APPLY renderer=wgpu', 'request_id=' + requestId
      + ' input_seq=' + sequence + ' revision=' + revision + ' generation=' + generation
      + ' target_vsync_id=unavailable transaction=queued_for_next_surface_frame listener_attached=true'),
    row('SPINON_R05_PRESENT_FENCE renderer=wgpu', 'request_id=' + requestId
      + ' input_seq=' + sequence + ' revision=' + revision + ' generation=' + generation
      + ' current_generation=' + generation + ' current_surface=true target_vsync_id=-1'
      + ' outcome=callback_received fence_valid=true fence_state=pending latch_time_ns=' + latch
      + ' input_offset_min_ns=-1000 input_offset_max_ns=1000 clock_offset_intervals_overlap=true'
      + ' processing_error=none fence_close_error=none'),
    row('SPINON_R05_FENCE_WAIT renderer=wgpu', 'request_id=' + requestId
      + ' input_seq=' + sequence + ' revision=' + revision + ' generation=' + generation
      + ' current_generation=' + generation + ' current_surface_at_callback=true target_vsync_id=-1'
      + ' outcome=signaled fence_valid_before=true await_returned=true fence_valid_after=true'
      + ' signal_time_ns=' + signal + ' signal_time_observed_monotonic_ns=' + (signal + 500n)
      + ' wait_start_monotonic_ns=' + (signal - 1_000n) + ' wait_return_monotonic_ns=' + (signal + 250n)
      + ' fence_signal_usable=true wait_error=none fence_close_error=none pending=0 active=0 queue_depth=0'),
  ];
}

function capture({ contacts, limit = contacts.length, overlapReports = 0, accepted = [], excluded = [], rawCount = contacts.length }) {
  const manifest = {
    schema: 'spinon-r05-android-physical-touch-capture-v1',
    startedUtc: '2026-10-09T04:59:55.000Z',
    preflightCompleteUtc: '2026-10-09T04:59:58.000Z',
    goUtc: '2026-10-09T05:00:00.000Z',
    firstRawContactUtc: '2026-10-09T05:00:01.000Z',
    contactLimitReachedUtc: '2026-10-09T05:00:02.000Z',
    stopRequestedUtc: '2026-10-09T05:00:03.000Z',
    stoppedUtc: '2026-10-09T05:00:04.000Z',
    captureStatus: 'raw_capture_complete_pending_join_validation',
    captureWindowValid: true,
    appId: 'dev.spinon.bootstrap',
    appProcessAgeSeconds: 10,
    installedApkSha256: 'a'.repeat(64),
    expectedApkSha256: 'a'.repeat(64),
    resumedActivity: 'topResumedActivity=dev.spinon.bootstrap/.MainActivity',
    stopReason: 'contact_limit',
    appPid: 100,
    inputEventPath: '/dev/input/event6',
    policy: {
      completeContactLimit: limit,
      maxProcessAgeSeconds: 300,
      firstContactTimeoutSeconds: 60,
      afterFirstContactTimeoutSeconds: 60,
      drainSeconds: 2,
    },
    captureError: null,
    collectors: [
      { label: 'getevent', pid: 201, parentPid: 200, startUtc: '2026-10-09T04:59:59.000Z', command: ['adb', '-s', '<redacted>', 'shell', 'getevent', '-lt', '/dev/input/event6'] },
      { label: 'logcat', pid: 202, parentPid: 200, startUtc: '2026-10-09T04:59:59.000Z', command: ['adb', '-s', '<redacted>', 'logcat', '--pid=100', '-v', 'threadtime'] },
    ],
    stopEscalation: [
      { label: 'getevent', result: { exited: true } },
      { label: 'logcat', result: { exited: true } },
    ],
    deviceGeteventProcessesAfter: [],
    raw: {
      completedContactGroups: rawCount,
      activeContactsAtStop: 0,
      pendingReleaseFramesAtStop: 0,
      overlappingContactReports: overlapReports,
      protocolErrors: [],
    },
  };
  const appText = [
    ...accepted.flatMap((item) => acceptedLog(item)),
    ...excluded.map((reason) => '10-09 13:00:00.000 100 101 I SpinonBootstrap: SPINON_R05_INPUT=excluded renderer=wgpu reason=' + reason),
  ].join('\n') + '\n';
  const inputDeviceText = 'add device 9: /dev/input/event6\n'
    + '  name:     "sec_touchscreen"\n  ABS (0003): ABS_MT_TRACKING_ID\n'
    + '  input props:\n    INPUT_PROP_DIRECT\n';
  return { manifest, rawText: rawContacts(contacts), appText, inputDeviceText };
}

test('first limit contacts are scored; later drain contacts remain outside the scoring set', () => {
  const contacts = [
    { id: 1, down: 100_000, up: 100_010 },
    { id: 2, down: 101_000, up: 101_010 },
    { id: 3, down: 102_000, up: 102_010 },
    { id: 4, down: 103_000, up: 103_010 },
  ];
  const result = analyzeCapture(capture({
    contacts, limit: 3,
    accepted: contacts.map((contact, index) => ({ sequence: index + 1, releaseMs: contact.up })),
  }));
  assert.equal(result.raw.scoredContacts, 3);
  assert.equal(result.raw.extraContactsOutsideScoringSet, 1);
  assert.equal(result.joins.exactRawReleaseToActionUp, 3);
  assert.equal(result.joins.currentGenerationCallback, 3);
  assert.equal(result.joins.usableFence, 3);
  assert.equal(result.joins.usableClockBracketCandidate, 3);
  assert.equal(result.app.acceptedActionUpsAfterScoringSet, 1);
  assert.equal(result.attempts[2].joinStatus, 'exact_join_usable_fence');
});

test('two simultaneous pointer slots remain two failed attempts and are not collapsed into taps', () => {
  const rawText = [
    ...contactDown(100_000, 0, 11, 200, 300),
    ...contactDown(100_100, 1, 12, 3_000, 3_000),
    ...contactUp(100_200, 1),
    ...contactUp(100_300, 0),
    ...contactDown(101_000, 0, 13, 200, 300),
    ...contactUp(101_010, 0),
    ...contactDown(102_000, 0, 14, 200, 300),
    ...contactUp(102_010, 0),
  ].join('\n') + '\n';
  const accepted = [
    { sequence: 1, releaseMs: 101_010 },
    { sequence: 2, releaseMs: 102_010 },
  ];
  const input = capture({
    contacts: [], limit: 3, overlapReports: 1, accepted,
    excluded: ['multiple_pointers', 'outside_target'], rawCount: 4,
  });
  input.rawText = rawText;
  const result = analyzeCapture(input);
  assert.equal(result.raw.overlappingContactReports, 1);
  assert.equal(result.raw.scoredContacts, 3);
  assert.equal(result.raw.extraContactsOutsideScoringSet, 1);
  assert.equal(result.joins.exactRawReleaseToActionUp, 1);
  assert.equal(result.joins.usableFence, 1);
  assert.deepEqual(result.app.excludedReasons, { multiple_pointers: 1, outside_target: 1 });
});

test('equal raw timestamps retain kernel line order when selecting the first scoring contact', () => {
  const rawText = [
    ...contactDown(100_000, 1, 17),
    ...contactDown(100_000, 0, 16),
    ...contactUp(100_010, 0),
    ...contactUp(100_020, 1),
  ].join('\n') + '\n';
  const input = capture({
    contacts: [], limit: 1, overlapReports: 1, rawCount: 2,
    accepted: [{ sequence: 1, releaseMs: 100_010 }],
  });
  input.rawText = rawText;
  const result = analyzeCapture(input);
  assert.equal(result.attempts[0].slot, 1);
  assert.equal(result.attempts[0].rawOrdinal, 0);
  assert.equal(result.attempts[0].joinStatus, 'raw_release_without_action_up');
  assert.equal(result.raw.extraContactsOutsideScoringSet, 1);
});

test('duplicate ACTION_UP timestamps are ambiguous rather than joined by sequence order', () => {
  const contact = { id: 21, down: 100_000, up: 100_010 };
  const input = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  input.appText += acceptedLog({ sequence: 2, releaseMs: contact.up }).join('\n') + '\n';
  const result = analyzeCapture(input);
  assert.equal(result.joins.exactRawReleaseToActionUp, 0);
  assert.equal(result.attempts[0].joinStatus, 'ambiguous_action_up');
});

test('stale surface generation and unusable fence do not count as completed joins', () => {
  const contact = { id: 31, down: 100_000, up: 100_010 };
  const stale = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  stale.appText = stale.appText.replace('current_generation=1 current_surface=true', 'current_generation=2 current_surface=false');
  const staleResult = analyzeCapture(stale);
  assert.equal(staleResult.joins.exactRawReleaseToActionUp, 1);
  assert.equal(staleResult.joins.currentGenerationCallback, 0);
  assert.equal(staleResult.joins.usableFence, 0);

  const pending = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  pending.appText = pending.appText.replace('outcome=signaled fence_valid_before=true', 'outcome=timeout fence_valid_before=true');
  const pendingResult = analyzeCapture(pending);
  assert.equal(pendingResult.joins.usableFence, 0);
  assert.equal(pendingResult.attempts[0].joinStatus, 'usable_fence_wait_missing_or_invalid');
});

test('raw parser count mismatch is rejected before any result is emitted', () => {
  const contact = { id: 41, down: 100_000, up: 100_010 };
  const input = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }], rawCount: 2 });
  assert.throws(() => analyzeCapture(input), /원시 로그 접촉 수와 manifest 접촉 수가 다릅니다/);
});

test('different touchscreen source, host PID, or surviving collector blocks analysis', () => {
  const contact = { id: 51, down: 100_000, up: 100_010 };
  const sourceMismatch = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  sourceMismatch.inputDeviceText = sourceMismatch.inputDeviceText.replace('"sec_touchscreen"', '"sec_touchpad"');
  assert.throws(() => analyzeCapture(sourceMismatch), /direct sec_touchscreen/);

  const pidMismatch = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  pidMismatch.appText = pidMismatch.appText.replace(' 100 101 I SpinonBootstrap:', ' 999 101 I SpinonBootstrap:');
  assert.throws(() => analyzeCapture(pidMismatch), /다른 process PID/);

  const leftoverCollector = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  leftoverCollector.manifest.deviceGeteventProcessesAfter = ['/system/bin/getevent'];
  assert.throws(() => analyzeCapture(leftoverCollector), /collector가 남았습니다/);
});

test('regressing raw clock is rejected instead of silently sorting corrupted events', () => {
  const contact = { id: 61, down: 100_000, up: 100_010 };
  const input = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  const lines = input.rawText.split('\n');
  lines[1] = lines[1].replace('[ 100.000000]', '[ 99.999999]');
  input.rawText = lines.join('\n');
  assert.throws(() => analyzeCapture(input), /protocol error 수와 manifest 값이 다릅니다/);
});

test('clock bracket disagreement keeps the touch/fence join but withholds latency candidate', () => {
  const contact = { id: 71, down: 100_000, up: 100_010 };
  const input = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  input.appText = input.appText.replace('input_offset_min_ns=-1000', 'input_offset_min_ns=9000');
  const result = analyzeCapture(input);
  assert.equal(result.joins.exactRawReleaseToActionUp, 1);
  assert.equal(result.joins.usableFence, 1);
  assert.equal(result.joins.usableClockBracketCandidate, 0);
  assert.match(result.attempts[0].joinStatus, /^exact_join_latency_unavailable:/);
});

test('excluded app events are not falsely attributed to raw contacts without event timestamps', () => {
  const contact = { id: 81, down: 100_000, up: 100_010 };
  const input = capture({ contacts: [contact], excluded: ['cancelled', 'outside_target'] });
  const result = analyzeCapture(input);
  assert.equal(result.attempts[0].joinStatus, 'raw_release_without_action_up');
  assert.deepEqual(result.app.excludedReasons, { cancelled: 1, outside_target: 1 });
  assert.match(result.app.exclusionToRawAttemptAssociation, /연결하지 않는다/);
});

test('capture timestamps must be ordered and fit the fixed collection deadlines', () => {
  const contact = { id: 82, down: 100_000, up: 100_010 };
  const beforeGo = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  beforeGo.manifest.firstRawContactUtc = '2026-10-09T04:59:59.999Z';
  assert.throws(() => analyzeCapture(beforeGo), /시각 순서가 올바르지 않습니다/);

  const afterDeadline = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  afterDeadline.manifest.contactLimitReachedUtc = '2026-10-09T05:01:02.000Z';
  afterDeadline.manifest.stopRequestedUtc = '2026-10-09T05:01:04.000Z';
  afterDeadline.manifest.stoppedUtc = '2026-10-09T05:01:05.000Z';
  assert.throws(() => analyzeCapture(afterDeadline), /deadline 또는 종료 시각/);
});

test('request IDs are unique across stage logs and target frame sentinels agree', () => {
  const contact = { id: 83, down: 100_000, up: 100_010 };
  const duplicated = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  duplicated.appText += duplicated.appText.match(/.*SPINON_R05_PRESENT_FENCE renderer=wgpu .*\n/)?.[0]
    .replace('revision=1', 'revision=99') ?? '';
  const duplicateResult = analyzeCapture(duplicated);
  assert.equal(duplicateResult.joins.usableFence, 0);

  const mismatched = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  mismatched.appText = mismatched.appText.replace('target_vsync_id=-1 state=registered', 'target_vsync_id=7 state=registered');
  assert.equal(analyzeCapture(mismatched).joins.currentGenerationCallback, 0);
});

test('duplicate key-value fields and duplicate input device paths reject analysis', () => {
  const contact = { id: 84, down: 100_000, up: 100_010 };
  const duplicateField = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  duplicateField.appText = duplicateField.appText.replace('generation=1\n', 'generation=1 generation=2\n');
  assert.throws(() => analyzeCapture(duplicateField), /같은 필드가 중복/);

  const duplicateDevice = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  duplicateDevice.inputDeviceText += duplicateDevice.inputDeviceText;
  assert.throws(() => analyzeCapture(duplicateDevice), /direct sec_touchscreen/);
});

test('malformed process identity or timing policy is rejected before joining', () => {
  const contact = { id: 85, down: 100_000, up: 100_010 };
  const invalidPid = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  invalidPid.manifest.appPid = 0;
  assert.throws(() => analyzeCapture(invalidPid), /foreground 또는 설치 APK provenance/);

  const invalidPolicy = capture({ contacts: [contact], accepted: [{ sequence: 1, releaseMs: contact.up }] });
  invalidPolicy.manifest.policy.afterFirstContactTimeoutSeconds = Number.NaN;
  assert.throws(() => analyzeCapture(invalidPolicy), /수집 시간 정책 값/);
});
