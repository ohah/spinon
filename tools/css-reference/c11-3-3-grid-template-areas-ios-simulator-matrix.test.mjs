import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import test from 'node:test';

import {
  compareC1133IOSSceneGeometry,
  c1133IOSSceneRunID,
  isC1133IOSGridRuntimeSuperseded,
  validateC1133IOSMatrixInputs,
  validateC1133IOSRejection,
  validateC1133IOSUnknownScene,
} from './c11-3-3-grid-template-areas-ios-matrix.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const runtime = JSON.parse(await readFile(resolve(repositoryRoot,
  'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json'), 'utf8'));
const reference = JSON.parse(await readFile(resolve(repositoryRoot,
  'tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json'), 'utf8'));
const observation = validateC1133IOSMatrixInputs(runtime, reference);

function successLog(scene, chromeCase, runID) {
  const frames = [];
  const paint = [];
  for (let index = 0; index < scene.nodeIds.length; index += 1) {
    const nodeName = scene.nodeIds[index];
    const expected = chromeCase.nodes.find(({ id }) => id === nodeName).rect;
    if (expected.width === 0 || expected.height === 0) continue;
    const nodeId = index + 1;
    frames.push('SPINON_C1133_GRID_NODE_FRAME_RUN run_id=' + runID + ' scene_id=' + scene.id
      + ' frame=' + frames.length + ':node=' + nodeId
      + ',x=' + expected.x + ',y=' + expected.y + ',width=' + expected.width + ',height=' + expected.height);
    paint.push(frames.length - 1 + ':' + nodeId);
  }
  return [
    'SPINON_C1133_GRID_EVAL_RUN run_id=' + runID + ' scene_id=' + scene.id + ' status=0 layout=ready boxes=' + frames.length,
    'SPINON_C1133_GRID_SUMMARY run_id=' + runID + ' scene_id=' + scene.id
      + ' status=0 layout=ready boxes=' + frames.length,
    ...frames,
    'SPINON_C1133_GRID_FRAME_SUMMARY run_id=' + runID + ' scene_id=' + scene.id
      + ' count=' + frames.length + ' marker=present',
    'SPINON_C1133_GRID_PAINT_ORDER_RUN run_id=' + runID + ' scene_id=' + scene.id
      + ' paint_node_order=[' + paint.join(',') + ']',
    'SPINON_C0410_DRAW generation=1 status=0 boxes=' + frames.length,
  ].join('\n');
}

test('iOS Simulator matrix 입력은 Chrome 고정 기준과 26개 장면·65개 노드를 일대일로 묶는다', () => {
  const ids = runtime.cases.map(({ id }) => id);
  assert.equal(new Set(ids).size, 26);
  assert.equal(runtime.cases.reduce((sum, scene) => sum + scene.nodeIds.length, 0), 65);
  assert.deepEqual(observation.cases.map(({ id }) => id), ids);
});

test('성공 scene의 node frame, paint order와 WGPU 제출을 Chrome DPR 1 기준에 대조한다', () => {
  const scene = runtime.cases.find(({ id }) => id === 'basic-rectangles');
  const chromeCase = observation.cases.find(({ id }) => id === scene.id);
  const result = compareC1133IOSSceneGeometry(scene, successLog(scene, chromeCase, 'ios-run'), chromeCase, 'ios-run');
  assert.equal(result.frames.length, 4);
  assert.equal(result.comparisons.length, 4);
  assert.equal(Math.max(...result.comparisons.map(({ maximumErrorCssPx }) => maximumErrorCssPx)), 0);
  assert.deepEqual(result.paintOrder.map(({ nodeId }) => nodeId), [1, 2, 3, 4]);
});

test('paint node 순서·ordinal 변조를 통과시키지 않는다', () => {
  const scene = runtime.cases.find(({ id }) => id === 'basic-rectangles');
  const chromeCase = observation.cases.find(({ id }) => id === scene.id);
  const valid = successLog(scene, chromeCase, 'ios-run');
  const reorderedNodes = valid.replace('paint_node_order=[0:1,1:2,2:3,3:4]',
    'paint_node_order=[0:1,1:3,2:2,3:4]');
  const reorderedOrdinals = valid.replace('paint_node_order=[0:1,1:2,2:3,3:4]',
    'paint_node_order=[1:1,0:2,2:3,3:4]');
  const drawLine = valid.match(/^SPINON_C0410_DRAW.*$/m)[0];
  const paintLine = valid.match(/^SPINON_C1133_GRID_PAINT_ORDER_RUN.*$/m)[0];
  const drawBeforePaint = valid.replace(drawLine, '').replace(paintLine, drawLine + '\n' + paintLine);
  assert.throws(() => compareC1133IOSSceneGeometry(scene, reorderedNodes, chromeCase, 'ios-run'));
  assert.throws(() => compareC1133IOSSceneGeometry(scene, reorderedOrdinals, chromeCase, 'ios-run'));
  assert.throws(() => compareC1133IOSSceneGeometry(scene, drawBeforePaint, chromeCase, 'ios-run'));
});

test('node frame에서 0.5 CSS px를 넘는 오차를 감지한다', () => {
  const scene = runtime.cases.find(({ id }) => id === 'basic-rectangles');
  const chromeCase = observation.cases.find(({ id }) => id === scene.id);
  const log = successLog(scene, chromeCase, 'ios-run').replace(
    'x=0,y=0,width=160,height=120', 'x=-1,y=0,width=160,height=120');
  assert.throws(() => compareC1133IOSSceneGeometry(scene, log, chromeCase, 'ios-run'));
});

test('누락 frame, 중복 node frame, draw count 불일치를 통과시키지 않는다', () => {
  const scene = runtime.cases.find(({ id }) => id === 'basic-rectangles');
  const chromeCase = observation.cases.find(({ id }) => id === scene.id);
  const valid = successLog(scene, chromeCase, 'ios-run');
  const missing = valid.replace(/SPINON_C1133_GRID_NODE_FRAME_RUN[^\n]*frame=1:[^\n]+\n/, '');
  assert.throws(() => compareC1133IOSSceneGeometry(scene, missing, chromeCase, 'ios-run'));
  const duplicate = valid.replace('frame=0:node=1', 'frame=0:node=2');
  assert.throws(() => compareC1133IOSSceneGeometry(scene, duplicate, chromeCase, 'ios-run'));
  const wrongDraw = valid.replace('status=0 boxes=4', 'status=0 boxes=3');
  assert.throws(() => compareC1133IOSSceneGeometry(scene, wrongDraw, chromeCase, 'ios-run'));
});

test('기대 rejection은 정확한 error·property·node와 frame/draw/readback 부재를 요구한다', () => {
  const scene = runtime.cases.find(({ expected }) => expected.outcome === 'expected-rejection');
  const nodeId = scene.nodeIds.indexOf(scene.expected.nodeId) + 1;
  const log = 'SPINON_C1133_GRID_REJECTION_SUMMARY run_id=ios-run scene_id=' + scene.id
    + ' expected=true status=-11 code=' + scene.expected.code
    + ' property=' + scene.expected.property + ' node_id=' + nodeId + ' frames=absent';
  assert.equal(validateC1133IOSRejection(scene, log, 'ios-run').status, 'expected-rejection');
  assert.throws(() => validateC1133IOSRejection(scene, log.replace('node_id=' + nodeId,
    'node_id=' + (nodeId + 1)), 'ios-run'));
  assert.throws(() => validateC1133IOSRejection(scene, log.replace('property=' + scene.expected.property,
    'property=wrong-property'), 'ios-run'));
  assert.throws(() => validateC1133IOSRejection(scene, log + '\nSPINON_C0410_DRAW status=0 presented boxes=1', 'ios-run'));
});

test('두 번째 기대 rejection은 grid-row-start와 item node 2를 정확히 기록한다', () => {
  const scene = runtime.cases.find(({ id }) => id === 'unknown-area-implicit-tracks');
  const log = 'SPINON_C1133_GRID_REJECTION_SUMMARY run_id=ios-run scene_id=' + scene.id
    + ' expected=true status=-11 code=' + scene.expected.code
    + ' property=' + scene.expected.property + ' node_id=2 frames=absent';
  assert.equal(validateC1133IOSRejection(scene, log, 'ios-run').runtimeNodeId, 2);
  assert.throws(() => validateC1133IOSRejection(scene,
    log.replace('grid-row-start', 'grid-column-start'), 'ios-run'));
});

test('한 번의 -12 superseded 응답만 같은 iOS scene 재실행 조건으로 잡는다', () => {
  const log = 'SPINON_C1133_GRID_EVAL_RUN run_id=ios-run scene_id=basic-rectangles status=-12 layout=ready';
  assert.equal(isC1133IOSGridRuntimeSuperseded(log, 'ios-run', 'basic-rectangles'), true);
  assert.equal(isC1133IOSGridRuntimeSuperseded(log, 'other-run', 'basic-rectangles'), false);
  assert.equal(isC1133IOSGridRuntimeSuperseded(log, 'ios-run', 'other-scene'), false);
  assert.equal(isC1133IOSGridRuntimeSuperseded(log.replace('status=-12', 'status=0'),
    'ios-run', 'basic-rectangles'), false);
});

test('가장 긴 scene 이름도 iOS의 80자 run ID 제한 안에 유지한다', () => {
  const prefix = 'c1133m-20261016T220218905Z-d9dbac89';
  const longestScene = runtime.cases.reduce((current, scene) =>
    scene.id.length > current.id.length ? scene : current, runtime.cases[0]);
  const first = c1133IOSSceneRunID(prefix, longestScene.index);
  const retry = c1133IOSSceneRunID(prefix, longestScene.index, 1);
  assert.ok(first.length <= 80);
  assert.ok(retry.length <= 80);
  assert.match(first, /^[A-Za-z0-9_-]+$/);
  assert.notEqual(first, retry);
});

test('알 수 없는 iOS scene은 runtime 평가·draw·Metal readback 전에 거부되어야 한다', () => {
  assert.equal(validateC1133IOSUnknownScene(
    'SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true', 'ios-bad-scene').status,
  'rejected-before-runtime-draw');
  assert.throws(() => validateC1133IOSUnknownScene(
    'SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true\nSPINON_C0410_DRAW status=0 presented boxes=1',
    'ios-bad-scene'));
  assert.throws(() => validateC1133IOSUnknownScene(
    'SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true\nSPINON_C1133_GRID_EVAL_RUN run_id=ios-bad-scene',
    'ios-bad-scene'));
});
