import assert from 'node:assert/strict';
import test from 'node:test';
import { validateWorkerStackStageRecords } from './c12-4-worker-stack-validation.mjs';

const schema = 'spinon-c12-4-css-worker-stack-residency/v1';

function report(threadId, residentPages = 1) {
  return {
    schema,
    status: 'ok',
    thread_id: threadId,
    pthread_reported_stack_start_address: '0x100000',
    pthread_reported_stack_bytes: 12288,
    stack_guard_bytes: 4096,
    stack_start_address: '0x101000',
    stack_end_address: '0x103000',
    requested_stack_bytes: 8 * 1024 * 1024,
    os_stack_range_bytes: 8192,
    mincore_sample_start_address: '0x101000',
    mincore_sample_end_address: '0x103000',
    mincore_sampled_stack_bytes: 8192,
    mincore_sampled_page_span_bytes: 8192,
    mincore_unprobed_stack_prefix_bytes: 0,
    mincore_sampled_tail_bytes: 0,
    mincore_sampled_pages: 2,
    page_size_bytes: 4096,
    mincore_resident_pages: residentPages,
    sampled_at_unix_ms: 1,
  };
}

function stageRecords(stage, threadForSession = (index) => 100 + index) {
  return Array.from({ length: stage }, (_, sessionIndex) =>
    [1, 2, 3].map((sample) => ({
      stage,
      sample,
      sessionIndex,
      report: report(threadForSession(sessionIndex)),
    }))).flat();
}

test('유효한 0개 및 다중 worker 단계를 받아들인다', () => {
  assert.deepEqual(validateWorkerStackStageRecords([], 0), []);
  assert.equal(validateWorkerStackStageRecords(stageRecords(2), 2).length, 6);
});

test('누락·중복 표본과 session 간 TID 재사용을 거부한다', () => {
  assert.throws(() => validateWorkerStackStageRecords(stageRecords(1).slice(1), 1));
  const duplicate = stageRecords(1);
  duplicate[2] = { ...duplicate[2], sample: 2 };
  assert.throws(() => validateWorkerStackStageRecords(duplicate, 1));
  assert.throws(() => validateWorkerStackStageRecords(stageRecords(2, () => 100), 2));
});

test('pthread guard, reported length, OS range, page span 불일치를 거부한다', () => {
  const badGuard = stageRecords(1);
  badGuard[0].report.stack_start_address = '0x100000';
  assert.throws(() => validateWorkerStackStageRecords(badGuard, 1));

  const badRange = stageRecords(1);
  badRange[0].report.os_stack_range_bytes += 1;
  assert.throws(() => validateWorkerStackStageRecords(badRange, 1));

  const badPageSpan = stageRecords(1);
  badPageSpan[0].report.mincore_sampled_pages = 3;
  assert.throws(() => validateWorkerStackStageRecords(badPageSpan, 1));
});

test('세 표본 사이 bounds 변경과 resident page 초과를 거부한다', () => {
  const changedBounds = stageRecords(1);
  changedBounds[1].report.stack_end_address = '0x104000';
  assert.throws(() => validateWorkerStackStageRecords(changedBounds, 1));

  const tooManyResidents = stageRecords(1);
  tooManyResidents[0].report.mincore_resident_pages = 3;
  assert.throws(() => validateWorkerStackStageRecords(tooManyResidents, 1));
});
