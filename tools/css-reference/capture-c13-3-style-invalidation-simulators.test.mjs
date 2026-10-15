import assert from 'node:assert/strict';
import { deflateSync } from 'node:zlib';
import test from 'node:test';

import {
  validatePaintScreenshot,
  validateStyleInvalidationLog,
} from './capture-c13-3-style-invalidation-simulators.mjs';

function styleLine(mode, geometry) {
  return `SPINON_C133_STYLE_UPDATE status=0 c13_3_style_invalidation=applied mode=${mode} document_revision=1→2 render_tree_revision=1→2 style_revision=0→1 scroll_state_revision=1→1 geometry_unchanged=${geometry} inner_offset=66,106 inner_range=66,106`;
}

function batch(gesture, trace, terminal = true) {
  return `SPINON_C133_INPUT_BATCH status=0 c13_3_input_batch=processed processed_count=1 pending_regular=0 scroll_state_revision=1 offsets=root:0,0;outer:0,0;inner:66,106 traces=[${trace}${terminal ? `;event=terminal sequence=${gesture + 2} gesture=${gesture} pointer=1 kind=End result=ended` : ''}]`;
}

function iosProcessBatch(gesture, trace, terminal = true) {
  return `SPINON_C133_PROCESS status=0 c13_3_input_batch=processed processed_count=1 pending_regular=0 scroll_state_revision=1 offsets=root:0,0;outer:0,0;inner:66,106 traces=[${trace}${terminal ? `;event=terminal sequence=${gesture + 2} gesture=${gesture} pointer=1 kind=End result=ended` : ''}]`;
}

function pngChunk(type, data) {
  const typeBytes = Buffer.from(type);
  const payload = Buffer.concat([typeBytes, data]);
  let crc = 0xffffffff;
  for (const byte of payload) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc & 1) ? (0xedb88320 ^ (crc >>> 1)) : (crc >>> 1);
    }
  }
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length);
  const checksum = Buffer.alloc(4);
  checksum.writeUInt32BE((crc ^ 0xffffffff) >>> 0);
  return Buffer.concat([length, payload, checksum]);
}

function screenshot({ paint, outside = [27, 52, 65, 255] }) {
  const width = 500;
  const height = 300;
  const pixels = Buffer.alloc(width * height * 4);
  for (let index = 0; index < width * height; index += 1) {
    pixels.set([16, 24, 39, 255], index * 4);
  }
  const set = (x, y, rgba) => pixels.set(rgba, (y * width + x) * 4);
  for (let y = 36; y < 244; y += 1) {
    for (let x = 26; x < 394; x += 1) {
      if (x < 29 || x >= 391 || y < 39 || y >= 241) set(x, y, [240, 198, 116, 255]);
    }
  }
  set(95, 110, paint);
  set(210, 120, outside);
  const rows = Buffer.alloc(height * (1 + width * 4));
  for (let y = 0; y < height; y += 1) {
    const row = y * (1 + width * 4);
    rows[row] = 0;
    pixels.copy(rows, row + 1, y * width * 4, (y + 1) * width * 4);
  }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header[8] = 8;
  header[9] = 6;
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  return Buffer.concat([
    signature,
    pngChunk('IHDR', header),
    pngChunk('IDAT', deflateSync(rows)),
    pngChunk('IEND', Buffer.alloc(0)),
  ]);
}

test('style policy 갱신은 기존 축을 유지하고 새 gesture에 touch-action:none을 적용한다', () => {
  const log = [
    styleLine(8, true),
    iosProcessBatch(1, 'event=pan_began sequence=2 gesture=1 result=pan_started'),
    iosProcessBatch(1, 'event=move sequence=3 gesture=1 policy_epoch=1 result=moved'),
    iosProcessBatch(2, 'event=pan_began sequence=5 gesture=2 result=pan_rejected'),
  ].join('\n');
  assert.equal(validateStyleInvalidationLog({ id: 8, name: 'style-policy', requiredGestures: 2 }, log).terminalEnds, 3);
});

test('Android INPUT_BATCH 로그도 같은 입력 검증에 사용한다', () => {
  const log = [
    styleLine(8, true),
    batch(1, 'event=pan_began sequence=2 gesture=1 result=pan_started'),
    batch(1, 'event=move sequence=3 gesture=1 policy_epoch=1 result=moved'),
    batch(2, 'event=pan_began sequence=5 gesture=2 result=pan_rejected'),
  ].join('\n');
  assert.equal(validateStyleInvalidationLog({ id: 8, name: 'style-policy', requiredGestures: 2 }, log).batches, 3);
});

test('Android의 동일한 STYLE_UPDATE 보고를 한 update로 센다', () => {
  const style = styleLine(8, true);
  const log = [
    style,
    `${style} eval=[logcat duplicate with a different truncation point]`,
    batch(1, 'event=pan_began sequence=2 gesture=1 result=pan_started'),
    batch(1, 'event=move sequence=3 gesture=1 policy_epoch=1 result=moved'),
    batch(2, 'event=pan_began sequence=5 gesture=2 result=pan_rejected'),
  ].join('\n');
  assert.equal(
    validateStyleInvalidationLog({ id: 8, name: 'style-policy', requiredGestures: 2 }, log)
      .duplicateStyleUpdateReports,
    1,
  );
});

test('Android의 PROCESS와 INPUT_BATCH 중복 보고를 한 batch로 센다', () => {
  const begin = 'status=0 c13_3_input_batch=processed processed_count=1 traces=[event=pan_began gesture=1 result=pan_started;event=terminal kind=End result=ended]';
  const report = 'status=0 c13_3_input_batch=processed processed_count=1 traces=[event=move policy_epoch=1 result=moved;event=terminal kind=End result=ended]';
  const log = [
    styleLine(10, true),
    `SPINON_C133_PROCESS ${begin}`,
    `SPINON_C133_PROCESS ${report}`,
    `SPINON_C133_INPUT_BATCH ${report}`,
  ].join('\n');
  assert.equal(validateStyleInvalidationLog({ id: 10, name: 'paint', requiredGestures: 1 }, log).batches, 2);
});

test('geometry 갱신은 활성 gesture를 취소한 뒤 새 geometry에서 pan을 허용한다', () => {
  const log = [
    styleLine(9, false),
    batch(1, 'event=pan_began sequence=2 gesture=1 result=pan_started'),
    batch(1, 'event=move sequence=3 gesture=1 result=cancelled GeometryChanged'),
    batch(2, 'event=pan_began sequence=5 gesture=2 result=pan_started'),
  ].join('\n');
  assert.equal(validateStyleInvalidationLog({ id: 9, name: 'geometry', requiredGestures: 2 }, log).terminalEnds, 3);
});

test('paint 갱신은 geometry와 scroll revision을 유지하고 active pan을 계속한다', () => {
  const log = [
    styleLine(10, true),
    batch(1, 'event=pan_began sequence=2 gesture=1 result=pan_started'),
    batch(1, 'event=move sequence=3 gesture=1 policy_epoch=1 result=moved'),
  ].join('\n');
  assert.equal(validateStyleInvalidationLog({ id: 10, name: 'paint', requiredGestures: 1 }, log).terminalEnds, 2);
});

test('stale policy epoch, geometry regression, and paint cancellation are rejected', () => {
  const policyLog = [styleLine(8, false), batch(1, 'event=move sequence=3 gesture=1 policy_epoch=1 result=moved'),
    batch(2, 'event=pan_began sequence=5 gesture=2 result=pan_rejected')].join('\n');
  assert.throws(() => validateStyleInvalidationLog({ id: 8, name: 'style-policy', requiredGestures: 2 }, policyLog));
  const geometryLog = [styleLine(9, false), batch(1, 'event=move sequence=3 gesture=1 result=moved'),
    batch(2, 'event=pan_began sequence=5 gesture=2 result=pan_started')].join('\n');
  assert.throws(() => validateStyleInvalidationLog({ id: 9, name: 'geometry', requiredGestures: 2 }, geometryLog));
  const paintLog = [styleLine(10, true), batch(1, 'event=move sequence=3 gesture=1 result=cancelled GeometryChanged')].join('\n');
  assert.throws(() => validateStyleInvalidationLog({ id: 10, name: 'paint', requiredGestures: 1 }, paintLog));
});

test('screenshot의 paint 변경과 inner border 바깥 clip pixel을 검증한다', () => {
  const before = screenshot({ paint: [23, 37, 58, 255] });
  const after = screenshot({ paint: [14, 86, 80, 255] });
  const result = validatePaintScreenshot(before, after, 1);
  assert.deepEqual(result.samples.outsideInnerScrollport.rgba, [27, 52, 65, 255]);
  const leaked = screenshot({ paint: [14, 86, 80, 255], outside: [14, 86, 80, 255] });
  assert.throws(() => validatePaintScreenshot(before, leaked, 1), /border 바깥으로 새지 않아야/);
});
