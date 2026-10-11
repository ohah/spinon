import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

export function parseWorkerStackStage(path, stage) {
  const records = readFileSync(path, 'utf8').split(/\r?\n/).flatMap((line) => {
    const marker = line.indexOf('SPINON_C124_WORKER_STACK_RESIDENCY ');
    if (marker < 0) return [];
    const match = line.slice(marker).match(
      /stage=(\d+) sample=(\d+) session_index=(\d+) report=(\{.*\})$/,
    );
    if (!match || Number(match[1]) !== stage) return [];
    let report;
    try {
      report = JSON.parse(match[4]);
    } catch {
      throw new Error(`worker stack JSON이 잘못됐습니다: ${line}`);
    }
    return [{
      stage: Number(match[1]),
      sample: Number(match[2]),
      sessionIndex: Number(match[3]),
      report,
    }];
  });
  return validateWorkerStackStageRecords(records, stage);
}

export function validateWorkerStackStageRecords(records, stage) {
  assert.equal(records.length, stage * 3,
    `stage ${stage}: worker stack 표본 수가 다릅니다 (${records.length} != ${stage * 3})`);
  const threadIds = new Map();
  const threadOwners = new Map();
  const boundsBySession = new Map();
  for (const record of records) {
    const sample = record.report;
    assert.equal(sample.schema, 'spinon-c12-4-css-worker-stack-residency/v1');
    assert.equal(sample.status, 'ok', `worker stack 표본 오류: ${JSON.stringify(record)}`);
    assert.ok(Number.isSafeInteger(sample.thread_id) && sample.thread_id > 0);
    assert.equal(sample.requested_stack_bytes, 8 * 1024 * 1024);
    assert.ok(sample.os_stack_range_bytes > 0);
    assert.ok(sample.page_size_bytes > 0);
    assert.ok(Number.isSafeInteger(sample.mincore_resident_pages));
    assert.ok(sample.mincore_resident_pages >= 0);
    assert.ok(Number.isSafeInteger(sample.pthread_reported_stack_bytes));
    assert.ok(Number.isSafeInteger(sample.stack_guard_bytes));
    assert.ok(sample.stack_guard_bytes >= 0);
    const reportedStart = BigInt(sample.pthread_reported_stack_start_address);
    const stackStart = BigInt(sample.stack_start_address);
    const stackEnd = BigInt(sample.stack_end_address);
    const sampleStart = BigInt(sample.mincore_sample_start_address);
    const sampleEnd = BigInt(sample.mincore_sample_end_address);
    assert.equal(stackStart, reportedStart + BigInt(sample.stack_guard_bytes));
    assert.equal(stackEnd - stackStart,
      BigInt(sample.pthread_reported_stack_bytes - sample.stack_guard_bytes));
    assert.equal(BigInt(sample.os_stack_range_bytes), stackEnd - stackStart);
    assert.ok(stackStart <= sampleStart && sampleStart < sampleEnd && sampleEnd >= stackEnd);
    assert.equal(BigInt(sample.mincore_sampled_stack_bytes), stackEnd - sampleStart);
    assert.equal(BigInt(sample.mincore_sampled_page_span_bytes), sampleEnd - sampleStart);
    assert.equal(BigInt(sample.mincore_unprobed_stack_prefix_bytes), sampleStart - stackStart);
    assert.equal(BigInt(sample.mincore_sampled_tail_bytes), sampleEnd - stackEnd);
    assert.equal(sample.mincore_sampled_pages,
      Number((sampleEnd - sampleStart) / BigInt(sample.page_size_bytes)));
    assert.ok(sample.mincore_sampled_pages > 0);
    assert.ok(sample.mincore_resident_pages <= sample.mincore_sampled_pages);
    const known = threadIds.get(record.sessionIndex) ?? new Set();
    known.add(sample.thread_id);
    threadIds.set(record.sessionIndex, known);
    const owner = threadOwners.get(sample.thread_id);
    assert.ok(owner === undefined || owner === record.sessionIndex,
      `동일 CSS worker TID ${sample.thread_id}가 여러 session에 귀속됐습니다`);
    threadOwners.set(sample.thread_id, record.sessionIndex);
    const immutableBounds = {
      pthread_reported_stack_start_address: sample.pthread_reported_stack_start_address,
      pthread_reported_stack_bytes: sample.pthread_reported_stack_bytes,
      stack_guard_bytes: sample.stack_guard_bytes,
      stack_start_address: sample.stack_start_address,
      stack_end_address: sample.stack_end_address,
      requested_stack_bytes: sample.requested_stack_bytes,
      os_stack_range_bytes: sample.os_stack_range_bytes,
      mincore_sample_start_address: sample.mincore_sample_start_address,
      mincore_sample_end_address: sample.mincore_sample_end_address,
      mincore_sampled_stack_bytes: sample.mincore_sampled_stack_bytes,
      mincore_sampled_page_span_bytes: sample.mincore_sampled_page_span_bytes,
      mincore_unprobed_stack_prefix_bytes: sample.mincore_unprobed_stack_prefix_bytes,
      mincore_sampled_tail_bytes: sample.mincore_sampled_tail_bytes,
      mincore_sampled_pages: sample.mincore_sampled_pages,
      page_size_bytes: sample.page_size_bytes,
    };
    const previousBounds = boundsBySession.get(record.sessionIndex);
    assert.ok(previousBounds === undefined
      || JSON.stringify(previousBounds) === JSON.stringify(immutableBounds),
    `session ${record.sessionIndex}의 worker stack bounds가 표본 사이 바뀌었습니다`);
    boundsBySession.set(record.sessionIndex, immutableBounds);
  }
  for (let sessionIndex = 0; sessionIndex < stage; sessionIndex++) {
    const sessionRecords = records.filter((record) => record.sessionIndex === sessionIndex);
    assert.deepEqual(sessionRecords.map((record) => record.sample).sort(), [1, 2, 3]);
    assert.equal(threadIds.get(sessionIndex)?.size, 1,
      `session ${sessionIndex}의 CSS worker TID가 표본 사이 바뀌었습니다`);
  }
  return records;
}
