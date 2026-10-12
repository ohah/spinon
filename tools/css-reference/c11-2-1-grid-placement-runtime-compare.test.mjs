import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import test from 'node:test';
import { buildC1121GridPlacementRuntimeMatrix } from './build-c11-2-1-grid-placement-runtime-matrix.mjs';
import { compareC1121GridPlacementScene } from './c11-2-1-grid-placement-runtime-compare.mjs';

const root = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const reference = JSON.parse(await readFile(resolve(root,
  'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json'), 'utf8'));
const matrix = await buildC1121GridPlacementRuntimeMatrix();
const runID = 'c1121-test-run-001';
const referenceCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
const chromeCaseFor = (id) => referenceCases.find((item) => item.id === id);
const fixtureFor = (id) => matrix.cases.find((item) => item.id === id);

function successLogs(id, { drawnBoxes, mutateFrame, duplicateFrame, paintOrder } = {}) {
  const fixture = fixtureFor(id);
  const chrome = chromeCaseFor(id);
  const expectedBoxes = fixture.nodeIds.length + 1;
  const frames = [{ id: 'viewport', rect: { x: 0, y: 0, width: 360, height: 800 } }, ...chrome.nodes];
  if (mutateFrame) mutateFrame(frames);
  const frameLines = frames.map((frame, index) => `SPINON_C1121_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${id} frame=${index}:node=${index + 1},x=${frame.rect.x},y=${frame.rect.y},width=${frame.rect.width},height=${frame.rect.height}`);
  if (duplicateFrame) frameLines.push(frameLines[0]);
  const order = paintOrder ?? frames.map((_, index) => `${index}:${index + 1}`).join(',');
  const evalLine = `SPINON_C1121_GRID_EVAL_RUN run_id=${runID} scene_id=${id} status=0 layout=ready boxes=${expectedBoxes}`;
  const rawEvalLine = `SPINON_C1121_GRID_EVAL status=0 node_frames_css_px=[${frames.map((frame, index) => `${index}:node=${index + 1},x=${frame.rect.x},y=${frame.rect.y},width=${frame.rect.width},height=${frame.rect.height}`).join(';')}] paint_node_order=[${order}]`;
  const lines = [rawEvalLine, evalLine,
    `SPINON_C1121_GRID_SUMMARY run_id=${runID} scene_id=${id} status=0 layout=ready boxes=${expectedBoxes}`,
    ...frameLines];
  if (id === 'overlap-source-order') {
    lines.push(`SPINON_C1121_GRID_PAINT_ORDER run_id=${runID} scene_id=${id} paint_node_order=[${order}]`);
  }
  lines.push(`SPINON_C0410_DRAW status=0 presented boxes=${drawnBoxes ?? expectedBoxes}`);
  return lines;
}

function rejectedLogs(id, overrides = {}) {
  const fixture = fixtureFor(id);
  const expected = fixture.expected;
  const code = overrides.code ?? expected.code;
  const nodeId = Object.hasOwn(overrides, 'nodeId') ? overrides.nodeId : expected.runtimeNodeId;
  const property = Object.hasOwn(overrides, 'property') ? overrides.property : expected.property;
  const status = overrides.status ?? -7;
  const nodeDiagnostic = nodeId === null ? 'node=None' : `node=Some(${nodeId})`;
  const propertyDiagnostic = property === null
    ? 'property=None' : `property=Some("${property}: 고정 실패 경계")`;
  const detail = expected.diagnosticIncludes ?? '';
  return [
    `SPINON_C1121_GRID_EVAL status=${status}`,
    `SPINON_C1121_GRID_EVAL_RUN run_id=${runID} scene_id=${id} status=${status}`,
    `SPINON_C1121_GRID_SUMMARY run_id=${runID} scene_id=${id} status=${status}`,
    `SPINON_C1121_GRID_DIAGNOSTIC run_id=${runID} scene_id=${id} CSS runtime 장면 생성 실패: ${code} ${nodeDiagnostic} ${propertyDiagnostic} ${detail}`,
  ];
}

test('6개 success scene은 전체 frame, WGPU 제출 및 겹침 paint 순서를 확인한다', () => {
  for (const fixture of matrix.cases.filter(({ expected }) => expected.outcome === 'success')) {
    const result = compareC1121GridPlacementScene({
      fixtureCase: fixture, chromeCase: chromeCaseFor(fixture.id),
      lines: successLogs(fixture.id), runID,
    });
    assert.equal(result.outcome, 'matched');
    assert.equal(result.frameCount, fixture.nodeIds.length + 1);
    assert.equal(result.maximumAbsoluteErrorCssPx, 0);
  }
  const overlap = compareC1121GridPlacementScene({
    fixtureCase: fixtureFor('overlap-source-order'),
    chromeCase: chromeCaseFor('overlap-source-order'),
    lines: successLogs('overlap-source-order', { paintOrder: '0:1,1:2,2:3,3:4' }),
    runID,
  });
  assert.deepEqual(overlap.paintOrderNodeIds, [1, 2, 3, 4]);
});

test('4개 profile 밖 scene은 정확한 code·NodeId·property와 미제출을 요구한다', () => {
  for (const fixture of matrix.cases.filter(({ expected }) => expected.outcome !== 'success')) {
    const result = compareC1121GridPlacementScene({
      fixtureCase: fixture, chromeCase: chromeCaseFor(fixture.id),
      lines: rejectedLogs(fixture.id), runID,
    });
    assert.equal(result.outcome, 'expected-rejection');
    assert.equal(result.diagnostic.fixtureNodeId, fixture.expected.nodeId);
  }
});

test('초기 장면이 환경 갱신에 밀리면 최종 ready summary가 있어야 통과한다', () => {
  const id = 'positive-spans';
  const logs = successLogs(id);
  const evalIndex = logs.findIndex((line) => line.includes('SPINON_C1121_GRID_EVAL_RUN'));
  assert.notEqual(evalIndex, -1);
  logs[evalIndex] = `SPINON_C1121_GRID_EVAL_RUN run_id=${runID} scene_id=${id} status=-12 op=eval status=0`;
  logs.splice(evalIndex + 1, 0,
    `SPINON_C1121_GRID_SUPERSEDED run_id=${runID} scene_id=${id} status=-12 op=eval status=0`);

  const result = compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id), lines: logs, runID,
  });
  assert.equal(result.outcome, 'matched');

  const missingSupersededMarker = logs.filter((line) =>
    !line.includes('SPINON_C1121_GRID_SUPERSEDED'));
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id),
    lines: missingSupersededMarker, runID,
  }), /SPINON_C1121_GRID_SUPERSEDED/);

  const unrelatedError = missingSupersededMarker.map((line) =>
    line.includes('SPINON_C1121_GRID_EVAL_RUN')
      ? line.replace('status=-12 op=eval status=0', 'status=-7 op=eval status=0')
      : line);
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id),
    lines: unrelatedError, runID,
  }), /초기 eval이 실패했습니다/);

  const withoutReadyScene = [
    logs[evalIndex], logs[evalIndex + 1],
    `SPINON_C0410_DRAW status=0 presented boxes=${fixtureFor(id).nodeIds.length + 1}`,
  ];
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id),
    lines: withoutReadyScene, runID,
  }), /GRID_SUMMARY/);
});

test('runner 오염·오류 진단 변경·frame 누락·geometry 변이·미제출·겹침 순서 오류는 실패한다', () => {
  const numeric = fixtureFor('numeric-lines');
  const numericChrome = chromeCaseFor('numeric-lines');
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: numeric, chromeCase: numericChrome,
    lines: successLogs('numeric-lines'), runID: 'stale-run',
  }));
  const badFrame = successLogs('numeric-lines');
  const targetFrameIndex = badFrame.findIndex((line) => line.includes('NODE_FRAME_RUN') && line.includes('frame=2:node=3'));
  badFrame[targetFrameIndex] = badFrame[targetFrameIndex].replace('width=165', 'width=166');
  assert.notEqual(targetFrameIndex, -1);
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: numeric, chromeCase: numericChrome, lines: badFrame, runID,
  }), /error=/);
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: numeric, chromeCase: numericChrome,
    lines: successLogs('numeric-lines', { drawnBoxes: 0 }), runID,
  }), /WGPU/);
  const shortFrames = successLogs('numeric-lines').filter((line) => !line.includes('frame=3:node=4'));
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: numeric, chromeCase: numericChrome, lines: shortFrames, runID,
  }), /frame 수/);
  const implicit = fixtureFor('deferred-implicit-track');
  const wrongDiagnostic = rejectedLogs('deferred-implicit-track', { property: 'grid-row-start' });
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: implicit, chromeCase: chromeCaseFor(implicit.id), lines: wrongDiagnostic, runID,
  }), /property/);
  const leakedFrame = [...rejectedLogs(implicit.id),
    `SPINON_C1121_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${implicit.id} frame=0:node=1,x=0,y=0,width=360,height=800`];
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: implicit, chromeCase: chromeCaseFor(implicit.id), lines: leakedFrame, runID,
  }), /frame/);
  const drawnReject = [...rejectedLogs(implicit.id), 'SPINON_C0410_DRAW status=0 presented boxes=3'];
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: implicit, chromeCase: chromeCaseFor(implicit.id), lines: drawnReject, runID,
  }), /비어 있지 않은 scene/);
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: fixtureFor('overlap-source-order'),
    chromeCase: chromeCaseFor('overlap-source-order'),
    lines: successLogs('overlap-source-order', { paintOrder: '0:1,1:2,2:4,3:3' }),
    runID,
  }), /source order/);
});

test('중복 frame은 내용이 같아야 하며 다른 중복 report는 실패한다', () => {
  const id = 'numeric-lines';
  compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id),
    lines: successLogs(id, { duplicateFrame: true }), runID,
  });
  const conflicting = successLogs(id, { duplicateFrame: true });
  conflicting[conflicting.length - 2] = conflicting[3].replace('width=360', 'width=359');
  assert.throws(() => compareC1121GridPlacementScene({
    fixtureCase: fixtureFor(id), chromeCase: chromeCaseFor(id), lines: conflicting, runID,
  }), /중복 frame 로그 값이 다릅니다/);
});
