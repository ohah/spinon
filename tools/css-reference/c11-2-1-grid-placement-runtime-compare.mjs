import assert from 'node:assert/strict';

const number = (value, description) => {
  const parsed = Number(value);
  assert.ok(Number.isFinite(parsed), `${description}: 유한한 숫자가 아닙니다.`);
  return parsed;
};

function markerLine(lines, marker, runID, sceneID) {
  const matching = lines.filter((line) => line.includes(`${marker} `)
    && line.includes(`run_id=${runID}`) && line.includes(`scene_id=${sceneID}`));
  assert.equal(matching.length, 1, `${sceneID}: 동일 PID·run·scene ${marker} 로그가 하나여야 합니다.`);
  return matching[0];
}

function framesFromLogs(lines, runID, sceneID) {
  const parsed = lines.flatMap((line) => {
    if (!line.includes(`SPINON_C1121_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=`)) return [];
    const match = line.match(/frame=(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/);
    assert.ok(match, `frame 로그 형식이 잘못되었습니다: ${line}`);
    return [{
      preorder: Number(match[1]), nodeId: Number(match[2]),
      rect: { x: number(match[3], 'x'), y: number(match[4], 'y'),
        width: number(match[5], 'width'), height: number(match[6], 'height') },
    }];
  });
  const byPreorder = new Map();
  for (const frame of parsed) {
    const previous = byPreorder.get(frame.preorder);
    if (previous) {
      assert.deepEqual(frame, previous, `preorder=${frame.preorder}: 중복 frame 로그 값이 다릅니다.`);
    } else {
      byPreorder.set(frame.preorder, frame);
    }
  }
  return [...byPreorder.values()].sort((left, right) => left.preorder - right.preorder);
}

function drawBoxCounts(lines) {
  return lines.flatMap((line, index) => line.includes('SPINON_C0410_DRAW')
    ? [...line.matchAll(/presented boxes=(\d+)/g)].map((match) => ({
      index,
      count: Number(match[1]),
    }))
    : []);
}

function compareRect(actual, expected, sceneID, nodeID, toleranceCssPx) {
  let maximumError = 0;
  for (const field of ['x', 'y', 'width', 'height']) {
    const error = Math.abs(actual[field] - expected[field]);
    maximumError = Math.max(maximumError, error);
    assert.ok(error <= toleranceCssPx,
      `${sceneID}/node=${nodeID}/${field}: actual=${actual[field]} expected=${expected[field]} error=${error}`);
  }
  return maximumError;
}

export function compareC1121GridPlacementScene({ fixtureCase, chromeCase, lines, runID, toleranceCssPx = 0.5 }) {
  assert.ok(Array.isArray(lines));
  const summary = markerLine(lines, 'SPINON_C1121_GRID_SUMMARY', runID, fixtureCase.id);
  const evalReport = markerLine(lines, 'SPINON_C1121_GRID_EVAL_RUN', runID, fixtureCase.id);
  const summaryIndex = lines.indexOf(summary);
  const isSuccess = fixtureCase.expected.outcome === 'success';
  const reportedStatus = summary.match(/\bstatus=(-?\d+)\b/)?.[1];
  assert.ok(reportedStatus, `${fixtureCase.id}: 요약 status가 없습니다.`);
  const evalStatus = evalReport.match(/\bstatus=(-?\d+)\b/)?.[1];
  assert.ok(evalStatus, `${fixtureCase.id}: 초기 eval status가 없습니다.`);

  if (!isSuccess) {
    assert.notEqual(reportedStatus, '0', `${fixtureCase.id}: 거부 장면이 성공했습니다.`);
    assert.equal(evalStatus, reportedStatus,
      `${fixtureCase.id}: 초기 eval과 거부 요약의 status가 다릅니다.`);
    const diagnostic = markerLine(lines, 'SPINON_C1121_GRID_DIAGNOSTIC', runID, fixtureCase.id);
    assert.ok(diagnostic.includes(fixtureCase.expected.code), `${fixtureCase.id}: 기대 오류 code가 없습니다.`);
    const expectedNode = fixtureCase.expected.runtimeNodeId === null
      ? 'node=None' : `node=Some(${fixtureCase.expected.runtimeNodeId})`;
    assert.ok(diagnostic.includes(expectedNode),
      `${fixtureCase.id}: 오류 NodeId가 다릅니다.`);
    const expectedProperty = fixtureCase.expected.property === null
      ? 'property=None' : `property=Some("${fixtureCase.expected.property}:`;
    assert.ok(diagnostic.includes(expectedProperty),
      `${fixtureCase.id}: 오류 property가 다릅니다.`);
    if (fixtureCase.expected.diagnosticIncludes) {
      assert.ok(diagnostic.includes(fixtureCase.expected.diagnosticIncludes),
        `${fixtureCase.id}: 기대 cascade 진단을 포함하지 않습니다.`);
    }
    assert.equal(framesFromLogs(lines, runID, fixtureCase.id).length, 0,
      `${fixtureCase.id}: 거부 뒤 frame이 제출되었습니다.`);
    assert.equal(summary.includes('node_frames_css_px=[')
      || evalReport.includes('node_frames_css_px=['), false,
      `${fixtureCase.id}: 거부 결과에 완성 frame snapshot이 있습니다.`);
    assert.equal(drawBoxCounts(lines).some(({ index, count }) => index > summaryIndex && count > 0), false,
      `${fixtureCase.id}: 거부 뒤 비어 있지 않은 scene이 제출되었습니다.`);
    return {
      sceneId: fixtureCase.id,
      outcome: 'expected-rejection',
      status: Number(reportedStatus),
      diagnostic: {
        code: fixtureCase.expected.code,
        runtimeNodeId: fixtureCase.expected.runtimeNodeId,
        fixtureNodeId: fixtureCase.expected.nodeId,
        property: fixtureCase.expected.property,
        detail: fixtureCase.expected.diagnosticIncludes ?? null,
      },
      frameCount: 0,
      positiveDrawCount: 0,
    };
  }

  assert.equal(reportedStatus, '0', `${fixtureCase.id}: success status`);
  assert.ok(summary.includes('layout=ready'), `${fixtureCase.id}: layout ready marker가 없습니다.`);
  if (evalStatus === '-12') {
    const superseded = markerLine(lines, 'SPINON_C1121_GRID_SUPERSEDED', runID, fixtureCase.id);
    assert.ok(superseded.includes('status=-12') && superseded.includes('op=eval status=0'),
      `${fixtureCase.id}: -12 eval이 성공한 장면의 superseded 응답으로 기록되지 않았습니다.`);
    assert.ok(lines.indexOf(superseded) > lines.indexOf(evalReport)
      && lines.indexOf(superseded) < summaryIndex,
    `${fixtureCase.id}: superseded marker의 순서가 잘못되었습니다.`);
  } else {
    assert.equal(evalStatus, '0', `${fixtureCase.id}: 성공 장면의 초기 eval이 실패했습니다.`);
  }
  const expectedBoxes = fixtureCase.nodeIds.length + 1;
  assert.ok(summary.includes(`boxes=${expectedBoxes}`), `${fixtureCase.id}: box count가 다릅니다.`);
  const actualFrames = framesFromLogs(lines, runID, fixtureCase.id);
  assert.equal(actualFrames.length, expectedBoxes, `${fixtureCase.id}: frame 수`);
  const wrapper = { x: 0, y: 0, width: 360, height: 800 };
  let maximumAbsoluteErrorCssPx = compareRect(
    actualFrames[0].rect, wrapper, fixtureCase.id, actualFrames[0].nodeId, toleranceCssPx,
  );
  assert.equal(actualFrames[0].preorder, 0);
  assert.equal(actualFrames[0].nodeId, 1);
  for (const [index, expectedNode] of chromeCase.nodes.entries()) {
    const actual = actualFrames[index + 1];
    assert.equal(actual.preorder, index + 1, `${fixtureCase.id}/${expectedNode.id}: preorder`);
    assert.equal(actual.nodeId, index + 2, `${fixtureCase.id}/${expectedNode.id}: HostDocument NodeId`);
    maximumAbsoluteErrorCssPx = Math.max(maximumAbsoluteErrorCssPx, compareRect(
      actual.rect, expectedNode.rect, fixtureCase.id, expectedNode.id, toleranceCssPx,
    ));
  }
  assert.ok(drawBoxCounts(lines).some(({ index, count }) => index > summaryIndex && count === expectedBoxes),
    `${fixtureCase.id}: 같은 process에서 WGPU가 기대 box 수를 제출하지 않았습니다.`);

  let paintOrderNodeIds = null;
  if (fixtureCase.id === 'overlap-source-order') {
    const paintOrder = markerLine(lines, 'SPINON_C1121_GRID_PAINT_ORDER', runID, fixtureCase.id);
    const order = paintOrder.match(/paint_node_order=\[([^\]]*)\]/)?.[1];
    assert.ok(order, `${fixtureCase.id}: paint_node_order가 없습니다.`);
    paintOrderNodeIds = [...order.matchAll(/\d+:(\d+)/g)].map((match) => Number(match[1]));
    const first = fixtureCase.nodeIds.indexOf('c1121-overlap-first') + 2;
    const second = fixtureCase.nodeIds.indexOf('c1121-overlap-second') + 2;
    assert.ok(paintOrderNodeIds.indexOf(first) >= 0 && paintOrderNodeIds.indexOf(second) >= 0,
      `${fixtureCase.id}: 겹친 item이 paint snapshot에 없습니다.`);
    assert.ok(paintOrderNodeIds.indexOf(first) < paintOrderNodeIds.indexOf(second),
      `${fixtureCase.id}: DOM source order가 paint snapshot에 보존되지 않았습니다.`);
  }
  return {
    sceneId: fixtureCase.id,
    outcome: 'matched',
    frameCount: actualFrames.length,
    maximumAbsoluteErrorCssPx,
    toleranceCssPx,
    positiveDrawCount: drawBoxCounts(lines)
      .filter(({ index, count }) => index > summaryIndex && count === expectedBoxes).length,
    paintOrderNodeIds,
  };
}
