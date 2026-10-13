import test from 'node:test';
import assert from 'node:assert/strict';
import { compareC1122GridAutoPlacementScene } from './c11-2-2-grid-auto-placement-runtime-compare.mjs';

const runID = 'test-run';
const sceneID = 'test-scene';
const successfulFixture = {
  id: sceneID,
  nodeIds: ['grid', 'item'],
  expected: { outcome: 'success' },
};
const successfulChrome = {
  id: sceneID,
  nodes: [
    { id: 'grid', rect: { x: 20, y: 20, width: 100, height: 44 } },
    { id: 'item', rect: { x: 20, y: 20, width: 40, height: 20 } },
  ],
};
const successfulLines = () => [
  `SPINON_C1122_GRID_EVAL_RUN run_id=${runID} scene_id=${sceneID} status=0`,
  `SPINON_C1122_GRID_SUMMARY run_id=${runID} scene_id=${sceneID} status=0 layout=ready boxes=3`,
  `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=0:node=1,x=0,y=0,width=360,height=800`,
  `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=1:node=2,x=20,y=20,width=100,height=44`,
  `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=2:node=3,x=20,y=20,width=40,height=20`,
  `SPINON_C1122_GRID_FRAME_SUMMARY run_id=${runID} scene_id=${sceneID} count=3 marker=present`,
  `SPINON_C1122_GRID_PAINT_ORDER run_id=${runID} scene_id=${sceneID} paint_node_order=[0:1,1:2,2:3]`,
  'SPINON_C0410_DRAW status=0 presented boxes=3',
];
const rejectedFixture = {
  id: sceneID,
  nodeIds: ['grid', 'item'],
  expected: {
    outcome: 'expected-rejection',
    code: 'invalid_grid_placement',
    nodeId: 'item',
    runtimeNodeId: 3,
    property: 'grid-column-start',
  },
};
const rejectedLines = () => [
  `SPINON_C1122_GRID_EVAL_RUN run_id=${runID} scene_id=${sceneID} status=-11`,
  `SPINON_C1122_GRID_SUMMARY run_id=${runID} scene_id=${sceneID} status=-11`,
  `SPINON_C1122_GRID_DIAGNOSTIC run_id=${runID} scene_id=${sceneID} CSS runtime 장면 생성 실패: invalid_grid_placement node=Some(3) property=Some("grid-column-start: unsupported")`,
];

function compareSuccess(lines = successfulLines(), overrides = {}) {
  return compareC1122GridAutoPlacementScene({
    fixtureCase: successfulFixture,
    chromeCase: successfulChrome,
    lines,
    runID,
    ...overrides,
  });
}

function compareRejected(lines = rejectedLines(), overrides = {}) {
  return compareC1122GridAutoPlacementScene({
    fixtureCase: rejectedFixture,
    chromeCase: { id: sceneID, nodes: [] },
    lines,
    runID,
    ...overrides,
  });
}

test('성공 scene은 모든 노드 geometry와 WGPU draw를 Chrome 기준에 비교한다', () => {
  const result = compareSuccess();
  assert.equal(result.outcome, 'matched');
  assert.equal(result.frameCount, 3);
  assert.equal(result.nodeCount, 2);
  assert.equal(result.maximumAbsoluteErrorCssPx, 0);
  assert.deepEqual(result.paintOrderNodeIds, [1, 2, 3]);
});

test('고정 거부 scene은 정확한 오류와 부분 출력 부재를 요구한다', () => {
  const result = compareRejected();
  assert.equal(result.outcome, 'expected-rejection');
  assert.equal(result.status, -11);
  assert.equal(result.diagnostic.property, 'grid-column-start');
});

test('이유 문자열이 없는 안정 오류에도 property 이름을 정확히 비교한다', () => {
  const lines = rejectedLines().map((line) => line
    .replace('property=Some("grid-column-start: unsupported")',
      'property=Some("grid-column-start")'));
  assert.equal(compareRejected(lines).diagnostic.property, 'grid-column-start');
});

test('display:none DOM 노드는 0×0 frame으로 보존하되 paint와 draw에서 제외한다', () => {
  const fixtureCase = {
    id: 'hidden-child-excluded',
    nodeIds: ['c1122-hidden-grid', 'c1122-hidden-item',
      'c1122-hidden-visible-a', 'c1122-hidden-visible-b'],
    expected: { outcome: 'success' },
  };
  const chromeCase = {
    id: fixtureCase.id,
    nodes: [
      { id: 'c1122-hidden-grid', properties: { display: 'grid' },
        rect: { x: 20, y: 20, width: 184, height: 40 } },
      { id: 'c1122-hidden-item', properties: { display: 'none' },
        rect: { x: 0, y: 0, width: 0, height: 0 } },
      { id: 'c1122-hidden-visible-a', rect: { x: 20, y: 20, width: 30, height: 20 } },
      { id: 'c1122-hidden-visible-b', rect: { x: 116, y: 20, width: 30, height: 20 } },
    ],
  };
  const lines = [
    `SPINON_C1122_GRID_EVAL_RUN run_id=${runID} scene_id=${fixtureCase.id} status=0`,
    `SPINON_C1122_GRID_SUMMARY run_id=${runID} scene_id=${fixtureCase.id} status=0 layout=ready boxes=4`,
    `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${fixtureCase.id} frame=0:node=1,x=0,y=0,width=360,height=800`,
    `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${fixtureCase.id} frame=1:node=2,x=20,y=20,width=184,height=40`,
    `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${fixtureCase.id} frame=2:node=3,x=0,y=0,width=0,height=0`,
    `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${fixtureCase.id} frame=3:node=4,x=20,y=20,width=30,height=20`,
    `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${fixtureCase.id} frame=4:node=5,x=116,y=20,width=30,height=20`,
    `SPINON_C1122_GRID_FRAME_SUMMARY run_id=${runID} scene_id=${fixtureCase.id} count=5 marker=present`,
    `SPINON_C1122_GRID_PAINT_ORDER run_id=${runID} scene_id=${fixtureCase.id} paint_node_order=[0:1,1:2,2:4,3:5]`,
    'SPINON_C0410_DRAW status=0 presented boxes=4',
  ];
  const result = compareC1122GridAutoPlacementScene({
    fixtureCase, chromeCase, lines, runID,
  });
  assert.equal(result.frameCount, 5);
  assert.equal(result.nodeCount, 4);
  assert.deepEqual(result.paintOrderNodeIds, [1, 2, 4, 5]);
});

const hostileCases = [
  ['최종 summary 누락', (lines) => lines.splice(1, 1), compareSuccess],
  ['동일 run의 summary 중복', (lines) => lines.push(lines[1]), compareSuccess],
  ['최초 eval 누락', (lines) => lines.splice(0, 1), compareSuccess],
  ['성공인데 eval 실패', (lines) => { lines[0] = lines[0].replace('status=0', 'status=-11'); }, compareSuccess],
  ['성공 node frame 누락', (lines) => lines.splice(3, 1), compareSuccess],
  ['중복 node frame', (lines) => lines.push(lines[2]), compareSuccess],
  ['frame NodeId 변경', (lines) => { lines[2] = lines[2].replace('node=1', 'node=99'); }, compareSuccess],
  ['geometry 허용 오차 초과', (lines) => { lines[3] = lines[3].replace('x=20', 'x=20.501'); }, compareSuccess],
  ['frame summary 누락', (lines) => lines.splice(5, 1), compareSuccess],
  ['paint order 누락', (lines) => lines.splice(6, 1), compareSuccess],
  ['source order 손실', (lines) => { lines[6] = lines[6].replace('[0:1,1:2,2:3]', '[0:1,1:3,2:2]'); }, compareSuccess],
  ['양수 draw 누락', (lines) => lines.pop(), compareSuccess],
  ['기대 거부 scene 성공 처리', (lines) => { lines[1] = lines[1].replace('status=-11', 'status=0'); }, compareRejected],
  ['eval과 최종 거부 코드 불일치', (lines) => { lines[1] = lines[1].replace('status=-11', 'status=-12'); }, compareRejected],
  ['거부 code 변조', (lines) => { lines[2] = lines[2].replace('invalid_grid_placement', 'unsupported_grid_auto_flow'); }, compareRejected],
  ['거부 NodeId 변조', (lines) => { lines[2] = lines[2].replace('node=Some(3)', 'node=Some(2)'); }, compareRejected],
  ['거부 CSS property 변조', (lines) => { lines[2] = lines[2].replace('grid-column-start:', 'grid-row-start:'); }, compareRejected],
  ['거부 진단 누락', (lines) => lines.splice(2, 1), compareRejected],
  ['거부 뒤 부분 frame 출력', (lines) => lines.push(`SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=0:node=1,x=0,y=0,width=360,height=800`), compareRejected],
  ['거부 뒤 양수 draw', (lines) => lines.push('SPINON_C0410_DRAW status=0 presented boxes=3'), compareRejected],
];

for (const [name, mutate, compare] of hostileCases) {
  test(`적대 경로: ${name}`, () => {
    const lines = compare === compareSuccess ? successfulLines() : rejectedLines();
    mutate(lines);
    assert.throws(() => compare(lines));
  });
}

test('기본 허용 오차와 다른 run의 무관한 기록은 구분한다', () => {
  const lines = successfulLines();
  lines.push(`SPINON_C1122_GRID_SUMMARY run_id=old-run scene_id=${sceneID} status=-11`);
  lines[3] = lines[3].replace('x=20,y=20', 'x=20.5,y=20');
  assert.equal(compareSuccess(lines).maximumAbsoluteErrorCssPx, 0.5);
});

test('다른 fixture source order를 Chrome oracle과 합치지 않는다', () => {
  assert.throws(() => compareC1122GridAutoPlacementScene({
    fixtureCase: { ...successfulFixture, nodeIds: ['item', 'grid'] },
    chromeCase: successfulChrome,
    lines: successfulLines(),
    runID,
  }));
});

test('잘못된 frame 로그 형식을 정상 geometry로 해석하지 않는다', () => {
  const lines = successfulLines();
  lines[2] = `SPINON_C1122_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=bad`;
  assert.throws(() => compareSuccess(lines));
});
