import assert from 'node:assert/strict';
import test from 'node:test';

import {
  C124_IOS_PIXEL_CASES,
  compareC124IOSMetalPixelLogs,
} from './compare-c12-4-ios-metal-surface-pixels.mjs';

const runID = 'run-review-001';

function record(caseDefinition, overrides = {}) {
  const scale = 3;
  const values = {
    status: 'ok',
    run_id: runID,
    case: caseDefinition.caseID,
    direction: caseDefinition.direction,
    blue_order: caseDefinition.blueOrder,
    red_order: caseDefinition.redOrder,
    css_x: caseDefinition.cssX,
    css_y: caseDefinition.cssY,
    scale,
    pixel_x: Math.floor((caseDefinition.cssX + 0.5) * scale),
    pixel_y: Math.floor((caseDefinition.cssY + 0.5) * scale),
    rgba: caseDefinition.rgba,
    expected: caseDefinition.rgba,
    surface: '960x720',
    renderer: 'metal-surface',
    ...overrides,
  };
  return `SPINON_C124_METAL_PIXEL ${Object.entries(values)
    .map(([key, value]) => `${key}=${value}`).join(' ')}`;
}

function validLog() {
  return C124_IOS_PIXEL_CASES.map((item) => record(item)).join('\n');
}

test('고정된 iOS Metal surface 8개 표본을 비교한다', () => {
  const result = compareC124IOSMetalPixelLogs(validLog(), { runID });
  assert.equal(result.result, 'passed');
  assert.equal(result.comparisonCount, 8);
  assert.equal(result.cases[6].cssSample[1], 175);
});

test('누락 case를 거부한다', () => {
  assert.throws(() => compareC124IOSMetalPixelLogs(
    C124_IOS_PIXEL_CASES.slice(1).map((item) => record(item)).join('\n'), { runID },
  ), /고정 case별 결과 8개/);
});

test('중복 case를 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item) => record(item));
  lines[1] = lines[0];
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /중복 case/);
});

test('잘못된 run ID를 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item) => record(item, { run_id: 'old-run' }));
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /고정 case별 결과 8개/);
});

test('pixel color mutation을 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item, index) => record(
    item, index === 0 ? { rgba: '#000000ff' } : {},
  ));
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /실제 RGBA/);
});

test('좌표 뒤바꿈을 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item, index) => record(
    item, index === 0 ? { pixel_x: 466, pixel_y: 211 } : {},
  ));
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /물리 pixel x/);
});

test('offscreen texture 결과를 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item, index) => record(
    item, index === 0 ? { renderer: 'offscreen-texture' } : {},
  ));
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /offscreen/);
});

test('실패 상태 marker가 섞인 실행을 거부한다', () => {
  const lines = C124_IOS_PIXEL_CASES.map((item) => record(item));
  lines[0] = lines[0].replace('status=ok', 'status=error');
  assert.throws(() => compareC124IOSMetalPixelLogs(lines.join('\n'), { runID }), /실패 표본/);
});
