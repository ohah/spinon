import assert from 'node:assert/strict';
import test from 'node:test';
import {
  parseC124LifecycleCycle,
  parseC124LifecycleReady,
  validateC124LifecycleRun,
} from './c12-4-repeated-session-lifecycle-validation.mjs';

const runId = 'c124-lifecycle-test-20261012';

function cycleLog(cycle, { ownerIds, workerIds } = {}) {
  const owners = ownerIds ?? Array.from({ length: 8 }, (_, index) => 100 + cycle * 20 + index);
  const workers = workerIds ?? Array.from({ length: 8 }, (_, index) => 200 + cycle * 20 + index);
  const lines = [
    `SPINON_C124_LIFECYCLE status=cycle_begin run_id=${runId} cycle=${cycle}`,
  ];
  owners.forEach((tid) => lines.push(`SPINON_C124_HOST=session=ready owner_tid=${tid} isolate_per_session=1`));
  workers.forEach((tid, sessionIndex) => lines.push(
    `SPINON_C124_LIFECYCLE_WORKER run_id=${runId} cycle=${cycle} session_index=${sessionIndex} report=`
      + JSON.stringify({
        schema: 'spinon-c12-4-css-worker-stack-residency/v1', status: 'ok', thread_id: tid,
      }),
  ));
  lines.push(
    `SPINON_C124_LIFECYCLE status=ready run_id=${runId} cycle=${cycle} session_count=8`,
    `SPINON_C124_LIFECYCLE status=freed run_id=${runId} cycle=${cycle} released=8 created=8`,
    `SPINON_C124_LIFECYCLE status=cycle_complete run_id=${runId} cycle=${cycle} created=8 released=8`,
  );
  return lines.join('\n');
}

function completeRun() {
  return [
    `SPINON_C124_LIFECYCLE status=started run_id=${runId} cycles=16 sessions_per_cycle=8`,
    ...Array.from({ length: 16 }, (_, index) => cycleLog(index + 1)),
    `SPINON_C124_LIFECYCLE status=finished run_id=${runId} result=passed cycles_completed=16 sessions_created=128`,
  ].join('\n');
}

test('16개 cycle의 owner와 CSS worker가 모두 기록된 실행을 검증한다', () => {
  assert.equal(validateC124LifecycleRun(completeRun(), runId).length, 16);
  assert.equal(parseC124LifecycleReady(cycleLog(1), runId, 1).workerThreadIds.length, 8);
});

test('누락 cycle, owner, worker, 중복 TID와 실패 marker를 거부한다', () => {
  assert.throws(() => validateC124LifecycleRun(completeRun().replace(cycleLog(7), ''), runId));
  assert.throws(() => parseC124LifecycleCycle(cycleLog(1, { ownerIds: [1, 2, 3, 4, 5, 6, 7] }), runId, 1));
  assert.throws(() => parseC124LifecycleCycle(cycleLog(1, { workerIds: [1, 2, 3, 4, 5, 6, 7] }), runId, 1));
  assert.throws(() => parseC124LifecycleCycle(cycleLog(1, {
    ownerIds: [1, 1, 2, 3, 4, 5, 6, 7],
  }), runId, 1));
  assert.throws(() => parseC124LifecycleCycle(
    cycleLog(1).replace(
      `SPINON_C124_LIFECYCLE status=freed run_id=${runId} cycle=1 released=8 created=8`,
      `SPINON_C124_LIFECYCLE status=free_failed run_id=${runId} cycle=1\n`
        + `SPINON_C124_LIFECYCLE status=freed run_id=${runId} cycle=1 released=8 created=8`,
    ), runId, 1));
});

test('run ID와 완료 집계가 맞지 않는 로그를 거부한다', () => {
  assert.throws(() => validateC124LifecycleRun(completeRun(), 'c124-lifecycle-other'));
  assert.throws(() => validateC124LifecycleRun(
    completeRun().replace('sessions_created=128', 'sessions_created=127'), runId));
  const outOfOrderRun = [
    `SPINON_C124_LIFECYCLE status=started run_id=${runId} cycles=16 sessions_per_cycle=8`,
    cycleLog(1), cycleLog(3), cycleLog(2),
    ...Array.from({ length: 13 }, (_, index) => cycleLog(index + 4)),
    `SPINON_C124_LIFECYCLE status=finished run_id=${runId} result=passed cycles_completed=16 sessions_created=128`,
  ].join('\n');
  assert.throws(() => validateC124LifecycleRun(outOfOrderRun, runId));
});
