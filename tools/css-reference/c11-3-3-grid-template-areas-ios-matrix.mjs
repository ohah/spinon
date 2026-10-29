import assert from 'node:assert/strict';

const EXPECTED_COUNTS = {
  success: 25,
  'cascade-fallback': 5,
  'expected-rejection': 1,
};

export function validateC1133IOSMatrixInputs(runtime, reference) {
  assert.equal(runtime.schema, 'spinon-css-c11-3-3-grid-template-areas-runtime-matrix/v1');
  assert.equal(runtime.profileId, 'RuntimeBlockFlexGridTemplateV1');
  assert.deepEqual(runtime.summary, {
    sceneCount: 31,
    nodeCount: 90,
    successCount: EXPECTED_COUNTS.success,
    cascadeFallbackCount: EXPECTED_COUNTS['cascade-fallback'],
    expectedRejectCount: EXPECTED_COUNTS['expected-rejection'],
  });
  const observation = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(observation, 'Chrome DPR 1 observation이 없습니다.');
  assert.equal(observation.cases.length, 26, '기존 Chrome 154 reference 장면 수가 바뀌었습니다.');
  assert.equal(runtime.cases.length, runtime.summary.sceneCount);

  const counts = { success: 0, 'cascade-fallback': 0, 'expected-rejection': 0 };
  const seen = new Set();
  let nodeCount = 0;
  for (const [index, scene] of runtime.cases.entries()) {
    assert.equal(scene.index, index, scene.id);
    assert.ok(!seen.has(scene.id), '중복 scene ID: ' + scene.id);
    seen.add(scene.id);
    assert.ok(Object.hasOwn(counts, scene.expected.outcome), scene.id + ': 알 수 없는 outcome');
    counts[scene.expected.outcome] += 1;
    nodeCount += scene.nodeIds.length;
    const chromeCase = scene.oracle ?? observation.cases.find(({ id }) => id === scene.id);
    assert.ok(chromeCase, scene.id + ': 고정 Chrome oracle이 없습니다');
    if (scene.oracle) {
      assert.equal(scene.oracle.sceneId, scene.id, scene.id + ': supplemental oracle scene ID');
      assert.equal(scene.oracle.nodes.length, scene.nodeIds.length, scene.id + ': oracle node 수');
    }
    assert.equal(chromeCase.rootId, scene.rootId, scene.id + ': root ID');
    assert.deepEqual(chromeCase.nodes.map(({ id }) => id), scene.nodeIds, scene.id + ': node 순서');
  }
  assert.equal(nodeCount, runtime.summary.nodeCount);
  assert.deepEqual(counts, EXPECTED_COUNTS);
  return observation;
}

export function compareC1133IOSSceneGeometry(scene, log, chromeCase, runID) {
  assert.ok(scene.expected.outcome !== 'expected-rejection', scene.id + ': rejection 장면이 success 경로로 들어왔습니다.');
  const framePrefix = 'SPINON_C1133_GRID_NODE_FRAME_RUN run_id=' + runID
    + ' scene_id=' + scene.id + ' frame=';
  const frames = log.split(/\r?\n/).filter((line) => line.includes(framePrefix)).map((line) => {
    const match = line.match(/\bframe=(\d+):node=(\d+),x=([^,\s]+),y=([^,\s]+),width=([^,\s]+),height=([^,\s]+)/);
    assert.ok(match, scene.id + ': frame 형식 오류: ' + line);
    return {
      index: Number(match[1]),
      nodeId: Number(match[2]),
      x: Number(match[3]),
      y: Number(match[4]),
      width: Number(match[5]),
      height: Number(match[6]),
    };
  });
  assert.deepEqual(frames.map(({ index }) => index), frames.map((_, index) => index),
    scene.id + ': frame index가 연속적이지 않습니다.');
  assert.equal(new Set(frames.map(({ nodeId }) => nodeId)).size, frames.length,
    scene.id + ': frame node ID가 중복되었습니다.');
  const frameSummary = log.split(/\r?\n/).find((line) => line.includes(
    'SPINON_C1133_GRID_FRAME_SUMMARY run_id=' + runID + ' scene_id=' + scene.id + ' '));
  assert.ok(frameSummary?.includes('count=' + frames.length) && frameSummary.includes('marker=present'),
    scene.id + ': 완전한 frame summary가 없습니다.');

  const summary = log.split(/\r?\n/).find((line) => line.includes(
    'SPINON_C1133_GRID_SUMMARY run_id=' + runID + ' scene_id=' + scene.id + ' '));
  assert.ok(summary?.includes('status=0 layout=ready'), scene.id + ': 성공 summary가 없습니다.');
  const boxes = Number(summary.match(/\bbox(?:es)?=(\d+)/)?.[1]);
  assert.ok(Number.isSafeInteger(boxes) && boxes >= 0, scene.id + ': boxes 수가 잘못되었습니다.');

  const byNodeId = new Map(frames.map((frame) => [frame.nodeId, frame]));
  const comparisons = [];
  for (let index = 0; index < scene.nodeIds.length; index += 1) {
    const nodeId = index + 1;
    const nodeName = scene.nodeIds[index];
    const expectedNode = chromeCase.nodes.find(({ id }) => id === nodeName);
    assert.ok(expectedNode, scene.id + '/' + nodeName + ': Chrome geometry가 없습니다.');
    const expected = expectedNode.rect;
    const frame = byNodeId.get(nodeId);
    const hasArea = expected.width > 0 && expected.height > 0;
    if (!frame) {
      assert.equal(hasArea, false, scene.id + '/' + nodeName + ': Chrome에서 보이는 frame 누락');
      continue;
    }
    const delta = Object.fromEntries(['x', 'y', 'width', 'height']
      .map((key) => [key, Math.abs(frame[key] - expected[key])]));
    assert.ok(Object.values(delta).every((value) => value <= 0.5),
      scene.id + '/' + nodeName + ': Chrome geometry 불일치 expected='
        + JSON.stringify(expected) + ' actual=' + JSON.stringify(frame));
    comparisons.push({
      nodeId,
      nodeName,
      expected,
      actual: { x: frame.x, y: frame.y, width: frame.width, height: frame.height },
      maximumErrorCssPx: Math.max(...Object.values(delta)),
    });
  }
  for (const frame of frames) {
    assert.ok(frame.nodeId >= 1 && frame.nodeId <= scene.nodeIds.length,
      scene.id + ': 등록되지 않은 runtime node ID ' + frame.nodeId);
  }

  const lines = log.split(/\r?\n/);
  const paintMarker = 'SPINON_C1133_GRID_PAINT_ORDER_RUN run_id=' + runID + ' scene_id=' + scene.id + ' ';
  const paintReport = lines.find((line) => line.includes(paintMarker));
  const paintReportIndex = lines.findIndex((line) => line.includes(paintMarker));
  const paintOrder = paintReport?.match(/\bpaint_node_order=\[([^\]]*)\]/)?.[1];
  assert.notEqual(paintOrder, undefined, scene.id + ': paint node order가 없습니다.');
  const paintEntries = [...paintOrder.matchAll(/(?:^|,)(\d+):(\d+)/g)]
    .map((match) => ({ order: Number(match[1]), nodeId: Number(match[2]) }));
  const paintedNodeIds = paintEntries.map(({ nodeId }) => nodeId);
  assert.equal(paintEntries.map(({ order, nodeId }) => order + ':' + nodeId).join(','), paintOrder,
    scene.id + ': paint node order 형식이 잘못되었습니다.');
  assert.equal(new Set(paintedNodeIds).size, paintedNodeIds.length, scene.id + ': paint node ID 중복');
  assert.equal(boxes, paintedNodeIds.length, scene.id + ': summary boxes와 paint order 수 불일치');
  assert.ok(paintedNodeIds.every((nodeId) => byNodeId.has(nodeId)),
    scene.id + ': paint node에 layout frame이 없습니다.');
  const expectedPaintNodeIds = scene.nodeIds
    .map((nodeName, index) => ({ nodeName, nodeId: index + 1 }))
    .filter(({ nodeName }) => {
      const rect = chromeCase.nodes.find(({ id }) => id === nodeName)?.rect;
      assert.ok(rect, scene.id + '/' + nodeName + ': paint order Chrome geometry가 없습니다.');
      return rect.width > 0 && rect.height > 0;
    })
    .map(({ nodeId }) => nodeId);
  assert.deepEqual(paintEntries.map(({ order }) => order),
    expectedPaintNodeIds.map((_, index) => index), scene.id + ': paint order index가 연속적이지 않습니다.');
  assert.deepEqual(paintedNodeIds, expectedPaintNodeIds,
    scene.id + ': visible DOM source order와 paint node order가 다릅니다.');
  const successfulDraw = paintReportIndex >= 0 && lines.some((line, index) =>
    index > paintReportIndex && line.includes('SPINON_C0410_DRAW')
      && /\bstatus=0(?: presented)? boxes=(\d+)\b/.exec(line)?.[1] === String(boxes));
  assert.ok(successfulDraw, scene.id + ': WGPU draw 결과가 없거나 box 수가 다릅니다.');

  return { frames, comparisons, boxes, paintOrder: paintEntries };
}

export function validateC1133IOSRejection(scene, log, runID) {
  assert.equal(scene.expected.outcome, 'expected-rejection', scene.id + ': expected-rejection이 아닙니다.');
  const expected = scene.expected;
  const expectedNodeId = scene.nodeIds.indexOf(expected.nodeId) + 1;
  assert.ok(expectedNodeId > 0, scene.id + ': expected node ID가 scene에 없습니다.');
  const summary = log.split(/\r?\n/).find((line) => line.includes(
    'SPINON_C1133_GRID_REJECTION_SUMMARY run_id=' + runID + ' scene_id=' + scene.id + ' '));
  assert.ok(summary, scene.id + ': rejection summary가 없습니다.');
  assert.ok(summary.includes('expected=true status=-11')
      && summary.includes('code=' + expected.code)
      && summary.includes('property=' + expected.property)
      && summary.includes('node_id=' + expectedNodeId)
      && summary.includes('frames=absent'),
  scene.id + ': 기대 rejection 계약 불일치: ' + summary);
  assert.equal(log.includes('SPINON_C1133_GRID_NODE_FRAME'), false,
    scene.id + ': 거부 결과에 layout frame이 있습니다.');
  assert.equal(log.includes('SPINON_C0410_DRAW'), false,
    scene.id + ': 거부 전에 GPU draw/present가 발생했습니다.');
  assert.equal(log.includes('SPINON_C1133_METAL_PIXEL_BEGIN'), false,
    scene.id + ': 거부 전에 Metal readback이 시작됐습니다.');
  return {
    status: 'expected-rejection',
    code: expected.code,
    property: expected.property,
    nodeId: expected.nodeId,
    runtimeNodeId: expectedNodeId,
  };
}

export function validateC1133IOSUnknownScene(log, runID) {
  assert.ok(log.includes('SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true'),
    '알 수 없는 iOS scene을 거부하지 않았습니다.');
  assert.equal(log.includes('SPINON_C1133_GRID_EVAL_RUN run_id=' + runID), false,
    '알 수 없는 iOS scene이 runtime 평가로 이어졌습니다.');
  assert.equal(log.includes('SPINON_C1133_GRID_SUMMARY run_id=' + runID), false,
    '알 수 없는 iOS scene이 성공으로 처리되었습니다.');
  assert.equal(log.includes('SPINON_C0410_DRAW'), false,
    '알 수 없는 iOS scene이 draw/present로 이어졌습니다.');
  assert.equal(log.includes('SPINON_C1133_METAL_PIXEL_BEGIN'), false,
    '알 수 없는 iOS scene이 Metal readback으로 이어졌습니다.');
  return { status: 'rejected-before-runtime-draw', runID };
}

export function isC1133IOSGridRuntimeSuperseded(log, runID, sceneID) {
  return log.split(/\r?\n/).some((line) => line.includes(
    'SPINON_C1133_GRID_EVAL_RUN run_id=' + runID + ' scene_id=' + sceneID + ' ')
      && /\bstatus=-12\b/.test(line));
}

export function c1133IOSSceneRunID(prefix, sceneIndex, retry = 0) {
  assert.match(prefix, /^[A-Za-z0-9_-]+$/, 'scene run ID prefix 문자가 잘못되었습니다.');
  assert.ok(Number.isSafeInteger(sceneIndex) && sceneIndex >= 0 && sceneIndex < 100,
    'scene index가 두 자리 범위를 벗어났습니다.');
  assert.ok(retry === 0 || retry === 1, 'scene run ID는 최초 실행 또는 단회 재실행만 허용합니다.');
  const runID = prefix + '-' + String(sceneIndex).padStart(2, '0') + (retry === 0 ? '' : '-r1');
  assert.ok(runID.length <= 80, 'iOS scene run ID가 80자를 넘습니다.');
  return runID;
}
