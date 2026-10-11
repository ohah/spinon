import assert from 'node:assert/strict';

export const C124_IOS_PIXEL_CASES = Object.freeze([
  { direction: 'row', blueOrder: 0, redOrder: 0, cssX: 70, cssY: 155, rgba: '#e5484dff' },
  { direction: 'row', blueOrder: 1, redOrder: 0, cssX: 70, cssY: 155, rgba: '#2866f6ff' },
  { direction: 'row-reverse', blueOrder: 0, redOrder: 0, cssX: 70, cssY: 155, rgba: '#2866f6ff' },
  { direction: 'row-reverse', blueOrder: 1, redOrder: 0, cssX: 70, cssY: 155, rgba: '#e5484dff' },
  { direction: 'column', blueOrder: 0, redOrder: 0, cssX: 70, cssY: 155, rgba: '#e5484dff' },
  { direction: 'column', blueOrder: 1, redOrder: 0, cssX: 70, cssY: 155, rgba: '#2866f6ff' },
  { direction: 'column-reverse', blueOrder: 0, redOrder: 0, cssX: 70, cssY: 175, rgba: '#2866f6ff' },
  { direction: 'column-reverse', blueOrder: 1, redOrder: 0, cssX: 70, cssY: 175, rgba: '#e5484dff' },
].map((item) => Object.freeze({
  ...item,
  caseID: `flex-${item.direction}-paint-order-${item.blueOrder}-${item.redOrder}`,
})));

function parsePixelLine(line) {
  const marker = 'SPINON_C124_METAL_PIXEL ';
  const markerIndex = line.indexOf(marker);
  assert.notEqual(markerIndex, -1, 'Metal pixel marker가 없습니다.');
  const fields = Object.fromEntries(
    [...line.slice(markerIndex + marker.length).matchAll(/([a-z_]+)=([^\s]+)/g)]
      .map(([, name, value]) => [name, value]),
  );
  for (const key of [
    'status', 'run_id', 'case', 'direction', 'blue_order', 'red_order',
    'css_x', 'css_y', 'scale', 'pixel_x', 'pixel_y', 'rgba', 'expected',
    'surface', 'renderer',
  ]) assert.ok(fields[key], `Metal pixel record에 ${key}가 없습니다: ${line}`);
  return fields;
}

export function compareC124IOSMetalPixelLogs(
  log,
  { runID, runIDs, expectedCases = C124_IOS_PIXEL_CASES } = {},
) {
  assert.ok(
    (typeof runID === 'string' && runID.length > 0)
      || (runIDs && typeof runIDs === 'object'),
    'run ID 또는 case별 run ID가 필요합니다.',
  );
  const lines = log.split(/\r?\n/);
  const relevant = lines.filter((line) => line.includes('SPINON_C124_METAL_PIXEL status='));
  assert.ok(relevant.length > 0, 'Metal surface pixel 결과 기록이 없습니다.');
  const records = relevant.map(parsePixelLine);
  const expectedRunIDs = new Map(expectedCases.map(({ caseID }) => [caseID, runIDs?.[caseID] ?? runID]));
  const runRecords = records.filter((record) => expectedRunIDs.get(record.case) === record.run_id);
  assert.equal(runRecords.length, expectedCases.length,
    `고정 case별 결과 ${expectedCases.length}개를 찾지 못했습니다.`);
  assert.ok(records.every((record) => expectedRunIDs.get(record.case) === record.run_id),
    '다른 case 또는 오래된 run ID가 섞였습니다.');
  assert.ok(runRecords.every((record) => record.status === 'ok'), '실패 표본 기록이 있습니다.');

  const byCase = new Map();
  for (const record of runRecords) {
    assert.ok(!byCase.has(record.case), `중복 case 결과입니다: ${record.case}`);
    byCase.set(record.case, record);
  }
  assert.deepEqual([...byCase.keys()].sort(), expectedCases.map(({ caseID }) => caseID).sort(),
    '고정된 C12.4 8개 case 집합과 다릅니다.');

  const verified = expectedCases.map((expected) => {
    const record = byCase.get(expected.caseID);
    assert.equal(record.direction, expected.direction, `${expected.caseID}: 방향이 다릅니다.`);
    assert.equal(Number(record.blue_order), expected.blueOrder, `${expected.caseID}: 파란 item order가 다릅니다.`);
    assert.equal(Number(record.red_order), expected.redOrder, `${expected.caseID}: 빨간 item order가 다릅니다.`);
    assert.equal(Number(record.css_x), expected.cssX, `${expected.caseID}: CSS 표본 x가 다릅니다.`);
    assert.equal(Number(record.css_y), expected.cssY, `${expected.caseID}: CSS 표본 y가 다릅니다.`);
    assert.equal(record.renderer, 'metal-surface', `${expected.caseID}: offscreen 결과를 surface로 잘못 표시했습니다.`);
    assert.equal(record.rgba, expected.rgba, `${expected.caseID}: 실제 RGBA가 고정 oracle과 다릅니다.`);
    assert.equal(record.expected, expected.rgba, `${expected.caseID}: 로그 기대 RGBA가 고정 oracle과 다릅니다.`);

    const scale = Number(record.scale);
    assert.ok(Number.isFinite(scale) && scale > 0, `${expected.caseID}: scale이 잘못되었습니다.`);
    const pixelX = Math.floor((expected.cssX + 0.5) * scale);
    const pixelY = Math.floor((expected.cssY + 0.5) * scale);
    assert.equal(Number(record.pixel_x), pixelX, `${expected.caseID}: 물리 pixel x가 다릅니다.`);
    assert.equal(Number(record.pixel_y), pixelY, `${expected.caseID}: 물리 pixel y가 다릅니다.`);
    const [surfaceWidth, surfaceHeight] = record.surface.split('x').map(Number);
    assert.equal(surfaceWidth, Math.round(320 * scale), `${expected.caseID}: drawable width가 다릅니다.`);
    assert.equal(surfaceHeight, Math.round(240 * scale), `${expected.caseID}: drawable height가 다릅니다.`);
    assert.ok(pixelX < surfaceWidth && pixelY < surfaceHeight,
      `${expected.caseID}: 표본이 drawable 바깥입니다.`);
    return {
      caseID: expected.caseID,
      direction: expected.direction,
      blueOrder: expected.blueOrder,
      redOrder: expected.redOrder,
      cssSample: [expected.cssX, expected.cssY],
      physicalPixel: [pixelX, pixelY],
      rgba: record.rgba,
      scale,
      surface: [surfaceWidth, surfaceHeight],
      renderer: record.renderer,
    };
  });

  return {
    schema: 'spinon-c12-4-ios-metal-surface-pixels/v1',
    runID: runID ?? 'case-scoped',
    result: 'passed',
    comparisonCount: verified.length,
    tolerancePerChannel: 0,
    claim: 'iOS Simulator Metal surface subset comparison only; not Spinon WPT conformance or full CSS support',
    cases: verified,
  };
}
