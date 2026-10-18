import assert from 'node:assert/strict';

const LIFECYCLE_MARKER = 'SPINON_C124_LIFECYCLE ';
const WORKER_MARKER = 'SPINON_C124_LIFECYCLE_WORKER ';
const HOST_MARKER = 'SPINON_C124_HOST=';
const STACK_SCHEMA = 'spinon-c12-4-css-worker-stack-residency/v1';

function markerIndex(lines, marker) {
  const expected = marker.startsWith(LIFECYCLE_MARKER)
    ? marker.slice(LIFECYCLE_MARKER.length) : marker;
  const indices = lines.flatMap((line, index) => lifecyclePayload(line) === expected ? [index] : []);
  assert.equal(indices.length, 1, `marker가 한 번 있어야 합니다: ${marker}`);
  return indices[0];
}

function lifecyclePayload(line) {
  const index = line.indexOf(LIFECYCLE_MARKER);
  return index < 0 ? '' : line.slice(index + LIFECYCLE_MARKER.length);
}

function parseThreadRecords(cycleLines, runId, cycle) {
  const ownerThreadIds = cycleLines.flatMap((line) => {
    const index = line.indexOf(HOST_MARKER);
    if (index < 0) return [];
    const payload = line.slice(index + HOST_MARKER.length);
    const match = payload.match(/^session=ready owner_tid=(\d+)\b/);
    return match ? [Number(match[1])] : [];
  });
  assert.equal(ownerThreadIds.length, 8, `cycle ${cycle}: V8 owner thread 기록이 8개가 아닙니다`);
  assert.ok(ownerThreadIds.every((tid) => Number.isSafeInteger(tid) && tid > 0));
  assert.equal(new Set(ownerThreadIds).size, 8, `cycle ${cycle}: 중복 V8 owner TID`);

  const workerRecords = cycleLines.flatMap((line) => {
    const index = line.indexOf(WORKER_MARKER);
    if (index < 0) return [];
    const payload = line.slice(index + WORKER_MARKER.length);
    const match = payload.match(/^run_id=(\S+) cycle=(\d+) session_index=(\d+) report=(\{.*\})$/);
    assert.ok(match, `cycle ${cycle}: worker marker 형식이 잘못됐습니다: ${line}`);
    assert.equal(match[1], runId);
    assert.equal(Number(match[2]), cycle);
    let report;
    try {
      report = JSON.parse(match[4]);
    } catch {
      throw new Error(`cycle ${cycle}: worker JSON이 잘못됐습니다: ${line}`);
    }
    assert.equal(report.schema, STACK_SCHEMA);
    assert.equal(report.status, 'ok');
    assert.ok(Number.isSafeInteger(report.thread_id) && report.thread_id > 0);
    return [{ sessionIndex: Number(match[3]), threadId: report.thread_id }];
  });
  assert.equal(workerRecords.length, 8, `cycle ${cycle}: CSS worker 기록이 8개가 아닙니다`);
  assert.deepEqual(workerRecords.map(({ sessionIndex }) => sessionIndex).sort((a, b) => a - b),
    [0, 1, 2, 3, 4, 5, 6, 7]);
  const workerThreadIds = workerRecords.map(({ threadId }) => threadId);
  assert.equal(new Set(workerThreadIds).size, 8, `cycle ${cycle}: 중복 CSS worker TID`);
  assert.equal(new Set([...ownerThreadIds, ...workerThreadIds]).size, 16,
    `cycle ${cycle}: owner와 CSS worker가 같은 TID를 보고했습니다`);
  return { ownerThreadIds, workerThreadIds };
}

export function parseC124LifecycleReady(log, runId, cycle) {
  assert.match(runId, /^c124-lifecycle-[A-Za-z0-9-]+$/);
  assert.ok(Number.isInteger(cycle) && cycle >= 1 && cycle <= 16);
  const lines = log.split(/\r?\n/);
  const beginToken = `${LIFECYCLE_MARKER}status=cycle_begin run_id=${runId} cycle=${cycle}`;
  const readyToken = `${LIFECYCLE_MARKER}status=ready run_id=${runId} cycle=${cycle} session_count=8`;
  const begin = markerIndex(lines, beginToken);
  const ready = markerIndex(lines, readyToken);
  assert.ok(begin < ready, `cycle ${cycle} ready marker 순서가 잘못됐습니다`);
  const cycleLines = lines.slice(begin, ready + 1);
  assert.equal(cycleLines.some((line) => /status=(?:create_failed|probe_failed|free_failed|cycle_failed)/
    .test(lifecyclePayload(line))), false, `cycle ${cycle} 실패 marker가 있습니다`);
  return { cycle, ...parseThreadRecords(cycleLines, runId, cycle), begin, ready };
}

export function parseC124LifecycleCycle(log, runId, cycle, { requireComplete = false } = {}) {
  assert.match(runId, /^c124-lifecycle-[A-Za-z0-9-]+$/);
  assert.ok(Number.isInteger(cycle) && cycle >= 1 && cycle <= 16);
  const lines = log.split(/\r?\n/);
  const beginToken = `${LIFECYCLE_MARKER}status=cycle_begin run_id=${runId} cycle=${cycle}`;
  const readyToken = `${LIFECYCLE_MARKER}status=ready run_id=${runId} cycle=${cycle} session_count=8`;
  const freedToken = `${LIFECYCLE_MARKER}status=freed run_id=${runId} cycle=${cycle} released=8 created=8`;
  const completeToken = `${LIFECYCLE_MARKER}status=cycle_complete run_id=${runId} cycle=${cycle} created=8 released=8`;
  const begin = markerIndex(lines, beginToken);
  const ready = markerIndex(lines, readyToken);
  const freed = markerIndex(lines, freedToken);
  assert.ok(begin < ready && ready < freed, `cycle ${cycle} marker 순서가 잘못됐습니다`);
  let complete = -1;
  if (requireComplete) {
    complete = markerIndex(lines, completeToken);
    assert.ok(freed < complete, `cycle ${cycle} 종료 순서가 잘못됐습니다`);
  }
  const end = requireComplete ? complete : freed;
  const cycleLines = lines.slice(begin, end + 1);
  assert.equal(cycleLines.some((line) => /status=(?:create_failed|probe_failed|free_failed|cycle_failed)/
    .test(lifecyclePayload(line))), false, `cycle ${cycle} 실패 marker가 있습니다`);

  const threads = parseThreadRecords(cycleLines, runId, cycle);
  return { cycle, ...threads, begin, ready, freed, complete };
}

export function validateC124LifecycleRun(log, runId) {
  const lines = log.split(/\r?\n/);
  const startToken = `${LIFECYCLE_MARKER}status=started run_id=${runId} cycles=16 sessions_per_cycle=8`;
  const finishToken = `${LIFECYCLE_MARKER}status=finished run_id=${runId} result=passed cycles_completed=16 sessions_created=128`;
  assert.equal(markerIndex(lines, startToken) >= 0, true);
  assert.equal(markerIndex(lines, finishToken) >= 0, true);
  assert.equal(lines.some((line) => line.includes(`${LIFECYCLE_MARKER}status=configuration_error`)), false);
  assert.equal(lines.some((line) => lifecyclePayload(line).includes(`run_id=${runId}`)
    && /status=(?:create_failed|probe_failed|free_failed|cycle_failed)/.test(lifecyclePayload(line))), false);
  for (const status of ['cycle_begin', 'ready', 'freed', 'cycle_complete']) {
    const count = lines.filter((line) => lifecyclePayload(line)
      .startsWith(`status=${status} run_id=${runId} `)).length;
    assert.equal(count, 16, `${status} marker 수가 16이 아닙니다`);
  }
  const cycles = Array.from({ length: 16 }, (_, index) =>
    parseC124LifecycleCycle(log, runId, index + 1, { requireComplete: true }));
  assert.deepEqual(cycles.map(({ cycle }) => cycle),
    Array.from({ length: 16 }, (_, index) => index + 1));
  for (let index = 1; index < cycles.length; index++) {
    assert.ok(cycles[index - 1].complete < cycles[index].begin,
      `cycle ${index}가 앞 cycle의 완료보다 먼저 시작했습니다`);
  }
  return cycles;
}
