import assert from 'node:assert/strict';

function markerLine(lines, marker, runID, sceneID) {
  const matching = lines.filter((line) => line.includes(`${marker} `)
    && line.includes(`run_id=${runID}`) && line.includes(`scene_id=${sceneID}`));
  assert.equal(matching.length, 1,
    `${sceneID}: 현재 process의 run·scene ${marker} 로그가 하나여야 합니다.`);
  return matching[0];
}

function statusOf(line, context) {
  const value = line.match(/\bstatus=(-?\d+)\b/)?.[1];
  assert.ok(value, `${context}: status가 없습니다.`);
  return Number(value);
}

function framesFromLogs(lines, runID, sceneID) {
  const frames = [];
  for (const line of lines) {
    if (!line.includes(`SPINON_C1123_GRID_NODE_FRAME_RUN run_id=${runID} scene_id=${sceneID} frame=`)) continue;
    const match = line.match(/frame=(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/);
    assert.ok(match, `${sceneID}: 잘못된 frame 로그: ${line}`);
    frames.push({
      preorder: Number(match[1]),
      nodeId: Number(match[2]),
      rect: {
        x: Number(match[3]), y: Number(match[4]),
        width: Number(match[5]), height: Number(match[6]),
      },
    });
  }
  assert.equal(new Set(frames.map(({ preorder }) => preorder)).size, frames.length,
    `${sceneID}: 중복 frame 로그가 있습니다.`);
  return frames.sort((left, right) => left.preorder - right.preorder);
}

function drawBoxCounts(lines) {
  return lines.flatMap((line) => line.includes('SPINON_C0410_DRAW')
    ? [...line.matchAll(/presented boxes=(\d+)/g)].map((match) => Number(match[1])) : []);
}

function compareRect(actual, expected, context, toleranceCssPx) {
  let maximumError = 0;
  for (const field of ['x', 'y', 'width', 'height']) {
    const error = Math.abs(actual[field] - expected[field]);
    maximumError = Math.max(maximumError, error);
    assert.ok(error <= toleranceCssPx,
      `${context}/${field}: actual=${actual[field]} expected=${expected[field]} error=${error}`);
  }
  return maximumError;
}

export function compareC1123GridAutoPlacementScene({
  fixtureCase, chromeCase, lines, runID, toleranceCssPx = 0.5,
}) {
  assert.ok(Array.isArray(lines));
  const summary = markerLine(lines, 'SPINON_C1123_GRID_SUMMARY', runID, fixtureCase.id);
  const evalReport = markerLine(lines, 'SPINON_C1123_GRID_EVAL_RUN', runID, fixtureCase.id);
  const summaryStatus = statusOf(summary, `${fixtureCase.id} summary`);
  const evalStatus = statusOf(evalReport, `${fixtureCase.id} eval`);
  const frames = framesFromLogs(lines, runID, fixtureCase.id);
  const draws = drawBoxCounts(lines);

  if (fixtureCase.expected.outcome === 'expected-rejection') {
    assert.notEqual(summaryStatus, 0, `${fixtureCase.id}: 거부 장면이 성공했습니다.`);
    assert.equal(evalStatus, summaryStatus,
      `${fixtureCase.id}: 최초 eval과 최종 거부 상태가 다릅니다.`);
    const diagnostic = markerLine(lines, 'SPINON_C1123_GRID_DIAGNOSTIC', runID, fixtureCase.id);
    assert.ok(diagnostic.includes(fixtureCase.expected.code),
      `${fixtureCase.id}: 기대 오류 code가 없습니다.`);
    assert.ok(diagnostic.includes(`node=Some(${fixtureCase.expected.runtimeNodeId})`),
      `${fixtureCase.id}: 오류 NodeId가 다릅니다.`);
    const reportedProperty = diagnostic
      .match(/property=Some\("([^":]+)(?::[^"]*)?"\)/)?.[1];
    assert.equal(reportedProperty, fixtureCase.expected.property,
      `${fixtureCase.id}: 오류 property가 다릅니다.`);
    assert.equal(frames.length, 0, `${fixtureCase.id}: 거부 장면에서 frame이 제출되었습니다.`);
    assert.equal(summary.includes('node_frames_css_px=[')
      || evalReport.includes('node_frames_css_px=['), false,
    `${fixtureCase.id}: 거부 결과에 완성 frame snapshot이 있습니다.`);
    assert.equal(draws.some((count) => count > 0), false,
      `${fixtureCase.id}: 거부 장면에서 비어 있지 않은 draw가 제출되었습니다.`);
    return {
      sceneId: fixtureCase.id,
      outcome: 'expected-rejection',
      status: summaryStatus,
      diagnostic: {
        code: fixtureCase.expected.code,
        runtimeNodeId: fixtureCase.expected.runtimeNodeId,
        fixtureNodeId: fixtureCase.expected.nodeId,
        property: fixtureCase.expected.property,
      },
      nodeCount: fixtureCase.nodeIds.length,
      frameCount: 0,
      positiveDrawCount: 0,
    };
  }

  assert.equal(summaryStatus, 0, `${fixtureCase.id}: 성공 scene status`);
  assert.ok(summary.includes('layout=ready'), `${fixtureCase.id}: layout ready가 없습니다.`);
  assert.equal(evalStatus, 0, `${fixtureCase.id}: 최초 eval이 성공하지 않았습니다.`);
  const chromeNodeByID = new Map(chromeCase.nodes.map((node) => [node.id, node]));
  assert.deepEqual(chromeCase.nodes.map(({ id }) => id), fixtureCase.nodeIds,
    `${fixtureCase.id}: fixture와 Chrome source order가 다릅니다.`);
  const hiddenNodeIDs = chromeCase.nodes
    .filter(({ properties }) => properties?.display === 'none')
    .map(({ id }) => id);
  const expectedHiddenNodeIDs = fixtureCase.id === 'hidden-item-excluded'
    ? ['c1123-hidden-item-excluded-hidden'] : [];
  assert.deepEqual(hiddenNodeIDs, expectedHiddenNodeIDs,
    `${fixtureCase.id}: Chrome에서 생략된 DOM node 집합이 계약과 다릅니다.`);
  const visibleBoxCount = chromeCase.nodes
    .filter(({ properties }) => properties?.display !== 'none').length + 1;
  const expectedFrameCount = fixtureCase.nodeIds.length + 1;
  assert.ok(summary.includes(`boxes=${visibleBoxCount}`), `${fixtureCase.id}: box count가 다릅니다.`);
  assert.equal(frames.length, expectedFrameCount, `${fixtureCase.id}: frame 수가 다릅니다.`);
  assert.ok(draws.includes(visibleBoxCount),
    `${fixtureCase.id}: 같은 process에서 기대 box 수를 그리지 않았습니다.`);

  let maximumAbsoluteErrorCssPx = compareRect(
    frames[0].rect, { x: 0, y: 0, width: 360, height: 800 },
    `${fixtureCase.id}/wrapper`, toleranceCssPx,
  );
  assert.equal(frames[0].preorder, 0);
  assert.equal(frames[0].nodeId, 1);
  for (const [index, nodeID] of fixtureCase.nodeIds.entries()) {
    const actual = frames[index + 1];
    assert.equal(actual.preorder, index + 1, `${fixtureCase.id}/${nodeID}: preorder`);
    assert.equal(actual.nodeId, index + 2, `${fixtureCase.id}/${nodeID}: NodeId`);
    const expectedNode = chromeNodeByID.get(nodeID);
    if (expectedNode.properties?.display === 'none') {
      assert.deepEqual(actual.rect, { x: 0, y: 0, width: 0, height: 0 },
        `${fixtureCase.id}/${nodeID}: display:none node가 0×0이어야 합니다.`);
      continue;
    }
    maximumAbsoluteErrorCssPx = Math.max(maximumAbsoluteErrorCssPx,
      compareRect(actual.rect, expectedNode.rect,
        `${fixtureCase.id}/${expectedNode.id}`, toleranceCssPx));
  }

  const frameSummary = markerLine(lines, 'SPINON_C1123_GRID_FRAME_SUMMARY', runID, fixtureCase.id);
  assert.ok(frameSummary.includes(`count=${expectedFrameCount} marker=present`),
    `${fixtureCase.id}: frame summary가 frame snapshot과 다릅니다.`);
  const paintOrder = markerLine(lines, 'SPINON_C1123_GRID_PAINT_ORDER', runID, fixtureCase.id);
  const encodedOrder = paintOrder.match(/paint_node_order=\[([^\]]*)\]/)?.[1];
  assert.ok(encodedOrder, `${fixtureCase.id}: paint order가 없습니다.`);
  const paintNodeIds = [...encodedOrder.matchAll(/\d+:(\d+)/g)].map((match) => Number(match[1]));
  const expectedPaintNodeIDs = [1, ...fixtureCase.nodeIds
    .map((id, index) => chromeNodeByID.get(id)?.properties?.display !== 'none' ? index + 2 : null)
    .filter((id) => id !== null)];
  assert.deepEqual(paintNodeIds, expectedPaintNodeIDs,
    `${fixtureCase.id}: DOM source order가 paint snapshot에 보존되지 않았습니다.`);

  return {
    sceneId: fixtureCase.id,
    outcome: 'matched',
    nodeCount: fixtureCase.nodeIds.length,
    frameCount: frames.length,
    maximumAbsoluteErrorCssPx,
    toleranceCssPx,
    positiveDrawCount: draws.filter((count) => count === visibleBoxCount).length,
    paintOrderNodeIds: paintNodeIds,
  };
}
