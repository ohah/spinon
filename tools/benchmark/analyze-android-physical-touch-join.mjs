#!/usr/bin/env node
import { readFile, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { parseGeteventLine } from './android-physical-touch-capture.mjs';

function fail(message) {
  throw new Error(message);
}

function parseKeyValues(line, marker) {
  const index = line.indexOf(marker);
  if (index < 0) return null;
  const fields = {};
  for (const token of line.slice(index + marker.length).trim().split(/\s+/)) {
    const separator = token.indexOf('=');
    if (separator > 0) {
      const key = token.slice(0, separator);
      if (Object.hasOwn(fields, key)) fields.__duplicateFields = true;
      fields[key] = token.slice(separator + 1);
    }
  }
  return fields;
}

function parseRawContacts(rawText) {
  let currentSlot = 0;
  const activeBySlot = new Map();
  const pendingRelease = [];
  const contacts = [];
  const protocolErrors = [];
  let overlapReports = 0;
  let lastTimestampNs = null;
  let startOrdinal = 0;

  for (const line of rawText.split(/\r?\n/)) {
    const event = parseGeteventLine(line);
    if (!event) continue;
    if (lastTimestampNs !== null && event.timestampNs < lastTimestampNs) {
      protocolErrors.push('raw_timestamp_regressed:' + event.timestampNs);
    }
    lastTimestampNs = event.timestampNs;

    if (event.type === 'ABS' && event.code === 'ABS_MT_SLOT' && typeof event.value === 'number') {
      currentSlot = event.value;
      continue;
    }

    if (event.type === 'ABS' && event.code === 'ABS_MT_TRACKING_ID' && typeof event.value === 'number') {
      if (event.value === -1) {
        const contact = activeBySlot.get(currentSlot);
        if (!contact) {
          protocolErrors.push('release_without_active_contact:' + currentSlot + ':' + event.timestampNs);
        } else {
          activeBySlot.delete(currentSlot);
          contact.releaseNs = event.timestampNs;
          pendingRelease.push(contact);
        }
      } else {
        const previous = activeBySlot.get(currentSlot);
        if (previous) {
          protocolErrors.push('tracking_id_replaced_without_release:' + currentSlot + ':' + event.timestampNs);
          previous.incompleteReason = 'tracking_id_replaced_without_release';
          contacts.push(previous);
        }
        const contact = {
          trackingId: event.value,
          slot: currentSlot,
          rawOrdinal: startOrdinal,
          downNs: event.timestampNs,
          releaseNs: null,
          x: null,
          y: null,
          complete: false,
          incompleteReason: null,
        };
        startOrdinal += 1;
        if (activeBySlot.size > 0) overlapReports += 1;
        activeBySlot.set(currentSlot, contact);
      }
      continue;
    }

    const active = activeBySlot.get(currentSlot);
    if (active && event.type === 'ABS' && event.code === 'ABS_MT_POSITION_X') active.x = event.value;
    if (active && event.type === 'ABS' && event.code === 'ABS_MT_POSITION_Y') active.y = event.value;

    if (event.type === 'SYN' && event.code === 'SYN_REPORT' && pendingRelease.length > 0) {
      for (const contact of pendingRelease.splice(0)) {
        contact.complete = true;
        contacts.push(contact);
      }
    }
  }

  for (const contact of pendingRelease) {
    contact.incompleteReason = 'release_without_syn_report';
    contacts.push(contact);
  }
  for (const [slot, contact] of activeBySlot) {
    contact.incompleteReason = 'contact_active_at_capture_end';
    protocolErrors.push('active_contact_at_capture_end:' + slot + ':' + contact.trackingId);
    contacts.push(contact);
  }

  contacts.sort((left, right) => left.downNs < right.downNs ? -1
    : left.downNs > right.downNs ? 1
      : left.rawOrdinal - right.rawOrdinal);
  return {
    contacts,
    completeCount: contacts.filter((contact) => contact.complete).length,
    overlapReports,
    protocolErrors,
  };
}

function parseLogRows(appText, marker) {
  const rows = appText.split(/\r?\n/)
    .filter((line) => line.includes(marker))
    .map((line) => parseKeyValues(line, marker))
    .filter(Boolean);
  if (rows.some((row) => row.__duplicateFields)) fail('앱 로그 한 행에 같은 필드가 중복됐습니다');
  return rows;
}

function oneMatching(rows, predicate) {
  const matches = rows.filter(predicate);
  return matches.length === 1 ? matches[0] : null;
}

function bigintField(row, name) {
  if (!row || !/^\d+$/.test(row[name] ?? '')) return null;
  return BigInt(row[name]);
}

function numericField(row, name) {
  if (!row || !/^\d+$/.test(row[name] ?? '')) return null;
  const value = Number(row[name]);
  return Number.isSafeInteger(value) ? value : null;
}

function analyzeLatency(input, wait, callback) {
  const eventTime = bigintField(input, 'event_time_ns');
  const uptimeAnchor = bigintField(input, 'input_uptime_anchor_ns');
  const monotonicBefore = bigintField(input, 'input_monotonic_before_ns');
  const monotonicAfter = bigintField(input, 'input_monotonic_after_ns');
  const signalTime = bigintField(wait, 'signal_time_ns');
  const latchTime = bigintField(callback, 'latch_time_ns');
  const signalObserved = bigintField(wait, 'signal_time_observed_monotonic_ns');
  const waitReturned = bigintField(wait, 'wait_return_monotonic_ns');
  if ([eventTime, uptimeAnchor, monotonicBefore, monotonicAfter, signalTime, latchTime,
    signalObserved, waitReturned].some((value) => value === null)) {
    return { valid: false, reason: 'clock_or_signal_field_missing' };
  }
  if (monotonicBefore > monotonicAfter || signalTime <= 0n || signalTime >= 9_000_000_000_000_000_000n) {
    return { valid: false, reason: 'clock_or_signal_value_invalid' };
  }
  const waitStarted = bigintField(wait, 'wait_start_monotonic_ns');
  if (waitStarted === null || signalTime < latchTime || signalObserved < signalTime
    || waitReturned < signalTime || waitReturned < waitStarted) {
    return { valid: false, reason: 'signal_or_observation_order_invalid' };
  }

  const offsetLow = uptimeAnchor - monotonicAfter;
  const offsetHigh = uptimeAnchor - monotonicBefore;
  if (callback.input_offset_min_ns !== offsetLow.toString()
    || callback.input_offset_max_ns !== offsetHigh.toString()
    || callback.clock_offset_intervals_overlap !== 'true') {
    return { valid: false, reason: 'clock_bracket_does_not_match_callback' };
  }
  const eventMonotonicLower = eventTime - offsetHigh;
  const eventMonotonicUpper = eventTime - offsetLow;
  const intervalLower = signalTime - eventMonotonicUpper;
  const intervalUpper = signalTime - eventMonotonicLower;
  if (intervalLower < 0n || intervalUpper < intervalLower) {
    return { valid: false, reason: 'candidate_interval_invalid' };
  }
  return {
    valid: true,
    lowerNs: intervalLower.toString(),
    upperNs: intervalUpper.toString(),
  };
}

export function analyzeCapture({ manifest, rawText, appText, inputDeviceText }) {
  if (!manifest || manifest.schema !== 'spinon-r05-android-physical-touch-capture-v1') {
    fail('지원하지 않는 R05 수집 manifest입니다');
  }
  if (manifest.captureWindowValid !== true
    || manifest.captureStatus !== 'raw_capture_complete_pending_join_validation'
    || manifest.stopReason !== 'contact_limit'
    || manifest.captureError !== null) {
    fail('유효한 contact_limit 원시 캡처가 아닙니다');
  }
  if (manifest.appId !== 'dev.spinon.bootstrap'
    || !Number.isSafeInteger(manifest.appPid) || manifest.appPid < 1
    || !/^[a-f0-9]{64}$/.test(manifest.expectedApkSha256 ?? '')
    || manifest.installedApkSha256 !== manifest.expectedApkSha256
    || !manifest.resumedActivity?.includes('dev.spinon.bootstrap/.MainActivity')) {
    fail('R05 앱 foreground 또는 설치 APK provenance가 일치하지 않습니다');
  }
  const limit = manifest.policy?.completeContactLimit;
  if (!Number.isSafeInteger(limit) || limit < 1 || limit > 300) fail('manifest의 접촉 제한 값이 올바르지 않습니다');
  for (const key of ['firstContactTimeoutSeconds', 'afterFirstContactTimeoutSeconds', 'drainSeconds']) {
    if (!Number.isSafeInteger(manifest.policy?.[key]) || manifest.policy[key] < 0) {
      fail('manifest의 수집 시간 정책 값이 올바르지 않습니다');
    }
  }
  if (manifest.policy.firstContactTimeoutSeconds < 1
    || manifest.policy.afterFirstContactTimeoutSeconds < 1
    || manifest.policy.drainSeconds > 10) {
    fail('manifest의 수집 시간 정책 범위가 올바르지 않습니다');
  }
  if (!Number.isSafeInteger(manifest.policy?.maxProcessAgeSeconds)
    || !Number.isFinite(manifest.appProcessAgeSeconds)
    || manifest.appProcessAgeSeconds < 0
    || manifest.appProcessAgeSeconds > manifest.policy.maxProcessAgeSeconds) {
    fail('fresh process age 조건을 충족하지 않습니다');
  }
  const captureTimes = [
    manifest.startedUtc,
    manifest.preflightCompleteUtc,
    manifest.goUtc,
    manifest.firstRawContactUtc,
    manifest.contactLimitReachedUtc,
    manifest.stopRequestedUtc,
    manifest.stoppedUtc,
  ].map((value) => typeof value === 'string' ? Date.parse(value) : Number.NaN);
  if (captureTimes.some((value) => !Number.isFinite(value))
    || captureTimes.some((value, index) => index > 0 && value < captureTimes[index - 1])) {
    fail('수집 시각 순서가 올바르지 않습니다');
  }
  const [startedAt, preflightAt, goAt, firstContactAt, limitReachedAt, stopRequestedAt, stoppedAt] = captureTimes;
  if (startedAt > preflightAt || preflightAt > goAt
    || firstContactAt - goAt > manifest.policy.firstContactTimeoutSeconds * 1_000
    || limitReachedAt - firstContactAt > manifest.policy.afterFirstContactTimeoutSeconds * 1_000
    || stopRequestedAt - limitReachedAt > manifest.policy.drainSeconds * 1_000 + 1_000
    || stopRequestedAt > stoppedAt) {
    fail('수집 deadline 또는 종료 시각이 사전 고정 정책을 벗어났습니다');
  }
  if (!/^\/dev\/input\/event\d+$/.test(manifest.inputEventPath ?? '')) fail('manifest의 raw input 경로가 올바르지 않습니다');
  if (typeof inputDeviceText !== 'string') fail('input device capability 기록이 없습니다');
  const deviceBlock = inputDeviceText.split(/(?=^add device \d+:)/m)
    .filter((block) => block.match(/^add device \d+:\s+(\S+)/m)?.[1] === manifest.inputEventPath);
  if (deviceBlock.length !== 1 || !deviceBlock[0].includes('"sec_touchscreen"')
    || !deviceBlock[0].includes('INPUT_PROP_DIRECT') || !deviceBlock[0].includes('ABS_MT_TRACKING_ID')) {
    fail('manifest input 경로가 direct sec_touchscreen capability 기록과 일치하지 않습니다');
  }
  const collectors = manifest.collectors;
  const collectorLabels = collectors?.map((collector) => collector.label) ?? [];
  const geteventCollector = collectors?.find((collector) => collector.label === 'getevent');
  const logcatCollector = collectors?.find((collector) => collector.label === 'logcat');
  if (collectorLabels.length !== 2 || new Set(collectorLabels).size !== 2
    || !Array.isArray(geteventCollector?.command) || !geteventCollector.command.includes('getevent')
    || !geteventCollector.command.includes(manifest.inputEventPath)
    || !Array.isArray(logcatCollector?.command) || !logcatCollector.command.includes('logcat')
    || !logcatCollector.command.includes('--pid=' + manifest.appPid)
    || !Number.isSafeInteger(geteventCollector.pid) || !Number.isSafeInteger(logcatCollector.pid)
    || geteventCollector.pid < 1 || logcatCollector.pid < 1
    || geteventCollector.pid === logcatCollector.pid
    || !Number.isSafeInteger(geteventCollector.parentPid) || !Number.isSafeInteger(logcatCollector.parentPid)
    || !Number.isFinite(Date.parse(geteventCollector.startUtc))
    || !Number.isFinite(Date.parse(logcatCollector.startUtc))
    || Date.parse(geteventCollector.startUtc) > Date.parse(manifest.goUtc)
    || Date.parse(logcatCollector.startUtc) > Date.parse(manifest.goUtc)) {
    fail('host collector process identity가 raw input 및 앱 PID에 연결되지 않습니다');
  }
  if (Number.isSafeInteger(manifest.appPid)) {
    const taggedAppLines = appText.split(/\r?\n/).filter((line) => line.includes('SPINON_R05_'));
    for (const line of taggedAppLines) {
      const pid = line.match(/^\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}\s+(\d+)\s+/)?.[1];
      if (pid !== String(manifest.appPid)) fail('앱 로그에 manifest와 다른 process PID가 섞여 있습니다');
    }
  }

  const parsed = parseRawContacts(rawText);
  if (parsed.completeCount !== manifest.raw?.completedContactGroups) {
    fail('원시 로그 접촉 수와 manifest 접촉 수가 다릅니다');
  }
  const stoppedLabels = manifest.stopEscalation?.map((item) => item.label).sort() ?? [];
  if (manifest.raw?.activeContactsAtStop !== 0 || manifest.raw?.pendingReleaseFramesAtStop !== 0
    || manifest.deviceGeteventProcessesAfter?.length !== 0
    || manifest.stopEscalation?.length !== 2
    || stoppedLabels.join(',') !== 'getevent,logcat'
    || manifest.stopEscalation.some((item) => item.result?.exited !== true)) {
    fail('수집 종료 뒤 contact frame 또는 getevent/host collector가 남았습니다');
  }
  if (parsed.completeCount < limit) fail('완전한 원시 접촉 수가 사전 고정 limit에 미달합니다');
  if (parsed.overlapReports !== manifest.raw?.overlappingContactReports) {
    fail('원시 로그 다중 접촉 수와 manifest 값이 다릅니다');
  }
  if (parsed.protocolErrors.length !== manifest.raw?.protocolErrors?.length) {
    fail('원시 로그 protocol error 수와 manifest 값이 다릅니다');
  }
  if (parsed.protocolErrors.length > 0) fail('원시 로그에 접촉 protocol error가 있습니다');

  const acceptedInputs = parseLogRows(appText, 'SPINON_R05_INPUT renderer=wgpu ')
    .filter((row) => row.phase === 'ACTION_UP');
  const submits = parseLogRows(appText, 'SPINON_R05_SUBMIT renderer=wgpu ');
  const listeners = parseLogRows(appText, 'SPINON_R05_PRESENT_FENCE_LISTENER renderer=wgpu ');
  const applies = parseLogRows(appText, 'SPINON_R05_PRESENT_FENCE_APPLY renderer=wgpu ');
  const callbacks = parseLogRows(appText, 'SPINON_R05_PRESENT_FENCE renderer=wgpu ');
  const waits = parseLogRows(appText, 'SPINON_R05_FENCE_WAIT renderer=wgpu ');
  const excludedInputs = parseLogRows(appText, 'SPINON_R05_INPUT=excluded renderer=wgpu ');
  const allApplyRequestIds = applies.map((row) => row.request_id).filter(Boolean);
  const allListenerRequestIds = listeners.map((row) => row.request_id).filter(Boolean);
  const allCallbackRequestIds = callbacks.map((row) => row.request_id).filter(Boolean);
  const allWaitRequestIds = waits.map((row) => row.request_id).filter(Boolean);
  const attempts = parsed.contacts.slice(0, limit).map((contact, index) => {
    const base = {
      attempt: index + 1,
      rawOrdinal: contact.rawOrdinal,
      trackingId: contact.trackingId,
      slot: contact.slot,
      rawDownNs: contact.downNs.toString(),
      rawReleaseNs: contact.releaseNs?.toString() ?? null,
      completeRawContact: contact.complete,
      appInputSeq: null,
      joinStatus: 'raw_contact_incomplete',
      submit: false,
      callback: false,
      usableFence: false,
      targetVsyncId: null,
      latencyCandidateNs: null,
    };
    if (!contact.complete || contact.releaseNs === null) return base;

    const actionUp = oneMatching(acceptedInputs, (row) => row.event_time_ns === contact.releaseNs.toString());
    const actionUpMatches = acceptedInputs.filter((row) => row.event_time_ns === contact.releaseNs.toString());
    if (actionUpMatches.length !== 1) {
      return { ...base, joinStatus: actionUpMatches.length === 0 ? 'raw_release_without_action_up' : 'ambiguous_action_up' };
    }

    const inputSeq = numericField(actionUp, 'input_seq');
    const generation = numericField(actionUp, 'generation');
    if (inputSeq === null || generation === null) return { ...base, joinStatus: 'action_up_fields_invalid' };
    base.appInputSeq = inputSeq;
    if (acceptedInputs.filter((row) => row.input_seq === String(inputSeq)).length !== 1) {
      return { ...base, joinStatus: 'app_input_sequence_ambiguous' };
    }

    const submit = oneMatching(submits, (row) => row.input_seq === String(inputSeq));
    if (!submit) return { ...base, joinStatus: 'submit_missing_or_ambiguous' };
    const revision = numericField(submit, 'revision');
    if (revision === null || submit.generation !== String(generation)
      || submit.event_time_ns !== contact.releaseNs.toString() || submit.draw_accepted !== 'true') {
      return { ...base, joinStatus: 'submit_identity_or_acceptance_mismatch', submit: true };
    }

    const apply = oneMatching(applies, (row) => row.input_seq === String(inputSeq)
      && row.revision === String(revision) && row.generation === String(generation));
    if (!apply || !/^\d+$/.test(apply.request_id ?? '')) {
      return { ...base, joinStatus: 'present_request_missing_or_ambiguous', submit: true };
    }
    const requestId = apply.request_id;
    const listener = oneMatching(listeners, (row) => row.request_id === requestId
      && row.input_seq === String(inputSeq) && row.revision === String(revision)
      && row.generation === String(generation));
    if (!listener || listener.state !== 'registered'
      || applies.filter((row) => row.request_id === requestId).length !== 1
      || listeners.filter((row) => row.request_id === requestId).length !== 1
      || new Set(allApplyRequestIds).size !== allApplyRequestIds.length
      || new Set(allListenerRequestIds).size !== allListenerRequestIds.length) {
      return { ...base, joinStatus: 'present_listener_or_request_identity_invalid', submit: true };
    }
    const callback = oneMatching(callbacks, (row) => row.request_id === requestId
      && row.input_seq === String(inputSeq) && row.revision === String(revision)
      && row.generation === String(generation));
    if (!callback || callback.outcome !== 'callback_received'
      || callback.current_surface !== 'true' || callback.current_generation !== String(generation)
      || callback.fence_valid !== 'true' || callback.processing_error !== 'none'
      || callback.fence_close_error !== 'none'
      || callbacks.filter((row) => row.request_id === requestId).length !== 1
      || new Set(allCallbackRequestIds).size !== allCallbackRequestIds.length
      || apply.listener_attached !== 'true'
      || apply.transaction !== 'queued_for_next_surface_frame'
      || listener.target_vsync_id !== callback.target_vsync_id) {
      return { ...base, joinStatus: 'current_surface_callback_missing_or_invalid', submit: true };
    }

    const wait = oneMatching(waits, (row) => row.request_id === requestId
      && row.input_seq === String(inputSeq) && row.revision === String(revision)
      && row.generation === String(generation));
    if (!wait || wait.outcome !== 'signaled' || wait.fence_signal_usable !== 'true'
      || wait.await_returned !== 'true' || wait.fence_valid_before !== 'true'
      || wait.fence_valid_after !== 'true' || wait.wait_error !== 'none'
      || wait.fence_close_error !== 'none' || wait.current_surface_at_callback !== 'true'
      || wait.current_generation !== String(generation)
      || wait.pending !== '0' || wait.active !== '0' || wait.queue_depth !== '0'
      || waits.filter((row) => row.request_id === requestId).length !== 1
      || new Set(allWaitRequestIds).size !== allWaitRequestIds.length
      || wait.target_vsync_id !== callback.target_vsync_id) {
      return { ...base, joinStatus: 'usable_fence_wait_missing_or_invalid', submit: true, callback: true };
    }

    const latency = analyzeLatency(actionUp, wait, callback);
    base.submit = true;
    base.callback = true;
    base.usableFence = true;
    base.targetVsyncId = callback.target_vsync_id ?? null;
    base.latencyCandidateNs = latency.valid ? { lower: latency.lowerNs, upper: latency.upperNs } : null;
    base.joinStatus = latency.valid ? 'exact_join_usable_fence' : 'exact_join_latency_unavailable:' + latency.reason;
    return base;
  });

  const exact = attempts.filter((attempt) => attempt.joinStatus === 'exact_join_usable_fence'
    || attempt.joinStatus.startsWith('exact_join_latency_unavailable:'));
  const candidate = attempts.filter((attempt) => attempt.latencyCandidateNs !== null);
  const extraRawContacts = parsed.contacts.slice(limit);
  const scoredReleaseTimes = new Set(attempts.map((attempt) => attempt.rawReleaseNs).filter(Boolean));
  const extraReleaseTimes = new Set(extraRawContacts.map((contact) => contact.releaseNs?.toString()).filter(Boolean));
  const acceptedActionUpsAfterScoring = acceptedInputs.filter((row) => extraReleaseTimes.has(row.event_time_ns)).length;
  const acceptedActionUpsNotJoinedToScoredRaw = acceptedInputs.filter((row) => !scoredReleaseTimes.has(row.event_time_ns)).length;
  const unjoinedScoredContactsByStatus = {};
  for (const attempt of attempts.filter((item) => item.appInputSeq === null)) {
    unjoinedScoredContactsByStatus[attempt.joinStatus] = (unjoinedScoredContactsByStatus[attempt.joinStatus] ?? 0) + 1;
  }
  const reasons = {};
  for (const row of excludedInputs) {
    const reason = row.reason ?? 'unspecified';
    reasons[reason] = (reasons[reason] ?? 0) + 1;
  }

  return {
    schema: 'spinon-r05-android-physical-touch-join-v1',
    analysisStatus: 'analyzed',
    scoringRule: 'GO 이후 raw tracking ID 시작 시각순 첫 ' + limit + '개만 채점하고 뒤의 입력은 원본에 보존하되 제외한다.',
    captureStatus: manifest.captureStatus,
    rawCaptureValid: manifest.captureWindowValid,
    raw: {
      completedContacts: parsed.completeCount,
      scoredContacts: attempts.length,
      extraContactsOutsideScoringSet: extraRawContacts.length,
      overlappingContactReports: parsed.overlapReports,
      protocolErrors: parsed.protocolErrors.length,
    },
    app: {
      acceptedActionUps: acceptedInputs.length,
      acceptedActionUpsAfterScoringSet: acceptedActionUpsAfterScoring,
      acceptedActionUpsNotJoinedToScoredRaw: acceptedActionUpsNotJoinedToScoredRaw,
      excludedInputs: excludedInputs.length,
      excludedReasons: reasons,
      exclusionToRawAttemptAssociation: '앱 제외 행에 event timestamp가 없어 개별 raw 접촉과 연결하지 않는다.',
    },
    joins: {
      exactRawReleaseToActionUp: attempts.filter((attempt) => attempt.appInputSeq !== null).length,
      submit: attempts.filter((attempt) => attempt.submit).length,
      currentGenerationCallback: attempts.filter((attempt) => attempt.callback).length,
      usableFence: attempts.filter((attempt) => attempt.usableFence).length,
      usableClockBracketCandidate: candidate.length,
      unjoinedScoredContactsByStatus,
    },
    candidateIntervalRangeNs: candidate.length === 0 ? null : {
      minimumLower: candidate.reduce((min, item) => BigInt(item.latencyCandidateNs.lower) < min
        ? BigInt(item.latencyCandidateNs.lower) : min, BigInt(candidate[0].latencyCandidateNs.lower)).toString(),
      maximumUpper: candidate.reduce((max, item) => BigInt(item.latencyCandidateNs.upper) > max
        ? BigInt(item.latencyCandidateNs.upper) : max, BigInt(candidate[0].latencyCandidateNs.upper)).toString(),
    },
    statisticalClaim: '한 block 결과이며 p95·제품 input-to-photon·VSync·scanout을 주장하지 않는다.',
    attempts,
  };
}

async function main(directory) {
  const root = resolve(directory);
  const manifest = JSON.parse(await readFile(join(root, 'capture-manifest.json'), 'utf8'));
  const rawText = await readFile(join(root, 'raw-touch.log'), 'utf8');
  const appText = await readFile(join(root, 'app.log'), 'utf8');
  const inputDeviceText = await readFile(join(root, 'input-devices.txt'), 'utf8');
  const summary = analyzeCapture({ manifest, rawText, appText, inputDeviceText });
  await writeFile(join(root, 'join-summary.json'), JSON.stringify(summary, null, 2) + '\n');
  process.stdout.write(JSON.stringify({
    analysisStatus: summary.analysisStatus,
    scoredContacts: summary.joins.exactRawReleaseToActionUp,
    attempts: summary.raw.scoredContacts,
    currentGenerationCallbacks: summary.joins.currentGenerationCallback,
    usableFences: summary.joins.usableFence,
    latencyCandidates: summary.joins.usableClockBracketCandidate,
    output: join(root, 'join-summary.json'),
  }, null, 2) + '\n');
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  if (!process.argv[2] || process.argv.length > 3 || process.argv[2] === '--help') {
    process.stdout.write('사용법: node tools/benchmark/analyze-android-physical-touch-join.mjs <capture-directory>\n');
    process.exitCode = process.argv[2] === '--help' ? 0 : 2;
  } else {
    main(process.argv[2]).catch((error) => {
      process.stderr.write('분석 실패: ' + error.message + '\n');
      process.exitCode = 1;
    });
  }
}
