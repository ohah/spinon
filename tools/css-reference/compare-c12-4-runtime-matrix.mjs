import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { buildC124RuntimeMatrix } from './build-c12-4-runtime-matrix.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const [evidencePath] = process.argv.slice(2);
if (!evidencePath) {
  throw new Error('사용법: node compare-c12-4-runtime-matrix.mjs <Android logcat 또는 iOS matrix JSON 경로>');
}

const [evidenceText, inventory, reference, matrix] = await Promise.all([
  readFile(resolve(evidencePath), 'utf8'),
  readFile(join(repositoryRoot, 'tests/fixtures/css/c12/stacking-context-inventory.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(repositoryRoot, 'tests/fixtures/css/references/c12-4-stacking-context-v1.json'), 'utf8')
    .then(JSON.parse),
  buildC124RuntimeMatrix(),
]);

let iosCapture = null;
if (evidenceText.trimStart().startsWith('{')) {
  iosCapture = JSON.parse(evidenceText);
  assert.equal(iosCapture.schema, 'spinon-c124-ios-runtime-matrix/v1', 'iOS matrix schema가 다릅니다.');
  assert.match(iosCapture.runID, /^[A-Za-z0-9_-]{1,80}$/, 'iOS matrix run ID가 잘못되었습니다.');
  assert.equal(iosCapture.platform, 'iOS Simulator', 'iOS 결과를 Simulator로 식별하지 못했습니다.');
  assert.equal(iosCapture.inventorySHA256, matrix.inventory.sha256, 'iOS matrix 입력 inventory hash가 다릅니다.');
  assert.equal(iosCapture.status, 'matched', 'iOS runtime matrix가 완료되지 않았습니다.');
  assert.equal(iosCapture.cases, 26, 'iOS matrix summary의 case 수가 다릅니다.');
  assert.equal(iosCapture.nodes, 77, 'iOS matrix summary의 node 수가 다릅니다.');
  assert.equal(iosCapture.expectedReady, 21, 'iOS matrix 지원 기대 수가 다릅니다.');
  assert.equal(iosCapture.expectedRejected, 5, 'iOS matrix 거부 기대 수가 다릅니다.');
  assert.equal(iosCapture.matched, 26, 'iOS matrix 기대 결과 일치 수가 다릅니다.');
  assert.equal(iosCapture.results.length, 26, 'iOS matrix 개별 결과 수가 다릅니다.');
}

const lines = iosCapture
  ? iosCapture.results.map((item) => `SPINON_C124_MATRIX_CASE index=${item.index} id=${item.id}`
    + ` expected=${item.expected} actual=${item.actual}`
    + ` reason_matched=${item.reasonMatched} matched=${item.matched} ${item.report}`)
  : evidenceText.split(/\r?\n/).filter((line) => line.includes('SPINON_C124_MATRIX_CASE '));
assert.equal(lines.length, 26, 'runtime case가 정확히 26개여야 합니다.');

function parseFrames(report, caseId) {
  const match = report.match(/node_frames_css_px=\[([^\]]*)\]/);
  assert.ok(match, `${caseId}: node frame report가 없습니다.`);
  return match[1].split(';').filter(Boolean).map((item) => {
    const fields = item.match(/^(\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,]+)$/);
    assert.ok(fields, `${caseId}: frame 형식이 잘못되었습니다: ${item}`);
    const [, index, nodeId, x, y, width, height] = fields;
    const frame = {
      index: Number(index),
      nodeId: Number(nodeId),
      x: Number(x),
      y: Number(y),
      width: Number(width),
      height: Number(height),
    };
    assert.ok(Object.values(frame).every(Number.isFinite), `${caseId}: 유한하지 않은 frame입니다.`);
    return frame;
  });
}

function parsePaintOrder(report, caseId) {
  const match = report.match(/paint_node_order=\[([^\]]*)\]/);
  assert.ok(match, `${caseId}: paint order report가 없습니다.`);
  return new Map(match[1].split(',').filter(Boolean).map((item) => {
    const fields = item.match(/^(\d+):(\d+)$/);
    assert.ok(fields, `${caseId}: paint order 형식이 잘못되었습니다: ${item}`);
    return [Number(fields[2]), Number(fields[1])];
  }));
}

const runtimeCases = new Map();
const expectedCaseById = new Map(matrix.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
assert.equal(expectedCaseById.size, 26, '고정 실행 matrix case가 중복되었습니다.');
for (const line of lines) {
  const match = line.match(/SPINON_C124_MATRIX_CASE index=(\d+) id=([^ ]+) expected=(ready|reject) actual=(ready|reject|reject-wrong-diagnostic|invalid-layout) reason_matched=(true|false) matched=(true|false) (.*)$/);
  assert.ok(match, `Android runtime case line 형식이 잘못되었습니다: ${line}`);
  const [, indexText, id, expected, actual, reasonMatchedText, matchedText, report] = match;
  const expectedCase = expectedCaseById.get(id);
  assert.ok(expectedCase, `${id}: 고정 실행 matrix에 없는 case입니다.`);
  const expectedOutcome = expectedCase.expected.kind === 'supported' ? 'ready' : 'reject';
  assert.equal(expected, expectedOutcome, `${id}: runtime의 예상 결과가 고정 inventory와 다릅니다.`);
  assert.equal(reasonMatchedText, 'true', `${id}: 실패 경계 진단이 예상 속성/API를 가리키지 않습니다.`);
  assert.equal(matchedText, 'true', `${id}: runtime 결과가 예상 outcome과 다릅니다.`);
  if (expectedOutcome === 'reject') {
    const { diagnostic, property, api } = expectedCase.expected;
    if (property === 'pseudo-element') {
      assert.ok(report.includes('cascade_error=')
        && report.includes('지원하지 않는 CSS 선언 content'),
      `${id}: CSS parser의 content 거부 진단이 없습니다.`);
    } else if (property) {
      assert.ok(report.includes('node=Some(')
        && report.includes(`property=Some("${property}")`),
      `${id}: NodeId와 ${property} 속성 진단이 없습니다.`);
    } else {
      assert.equal(api, 'showModal', `${id}: 검증하지 않은 API 거부 진단입니다.`);
      assert.ok(report.includes('TypeError:') && report.includes('showModal is not a function'),
        `${id}: showModal API 거부 진단이 없습니다.`);
    }
  }
  assert.equal(runtimeCases.has(id), false, `${id}: runtime case가 중복되었습니다.`);
  runtimeCases.set(id, {
    index: Number(indexText),
    expected,
    actual,
    report,
  });
}

const referenceCases = reference.observations[0].cases;
const toleranceCssPx = inventory.comparison.maximumAbsoluteRectErrorCssPx;
const supportedCases = inventory.cases.filter(({ id }) => runtimeCases.get(id)?.expected === 'ready');
let comparedNodes = 0;
let comparedHitTestObservations = 0;
let maximumAbsoluteErrorCssPx = 0;
let worstField = null;
const mismatchDetails = [];
const auxiliaryPaintOrderDifferences = [];

for (let caseIndex = 0; caseIndex < inventory.cases.length; caseIndex += 1) {
  const fixtureCase = inventory.cases[caseIndex];
  const runtimeCase = runtimeCases.get(fixtureCase.id);
  const referenceCase = referenceCases.find(({ id }) => id === fixtureCase.id);
  assert.ok(runtimeCase, `${fixtureCase.id}: Android runtime 결과가 없습니다.`);
  assert.equal(runtimeCase.index, caseIndex, `${fixtureCase.id}: matrix index 순서가 다릅니다.`);
  assert.ok(referenceCase, `${fixtureCase.id}: Chromium 고정 기준 case가 없습니다.`);
  assert.equal(referenceCase.nodes.length, fixtureCase.nodes.length, `${fixtureCase.id}: 기준 node 수가 다릅니다.`);

  if (runtimeCase.expected === 'reject') {
    assert.notEqual(runtimeCase.report.match(/\bstatus=(-?\d+)/)?.[1], '0',
      `${fixtureCase.id}: 거부 예상 case가 runtime에서 성공했습니다.`);
    continue;
  }

  const observedFrames = parseFrames(runtimeCase.report, fixtureCase.id);
  const generatedStyleElements = expectedCaseById.get(fixtureCase.id).stylesheetRules.length > 0 ? 1 : 0;
  assert.equal(observedFrames.length, fixtureCase.nodes.length + 2 + generatedStyleElements,
    `${fixtureCase.id}: root/stage/style을 포함한 frame 개수가 다릅니다.`);
  assert.equal(new Set(observedFrames.map(({ nodeId }) => nodeId)).size, observedFrames.length,
    `${fixtureCase.id}: runtime frame node ID가 중복되었습니다.`);
  const observedFrameByNodeId = new Map(observedFrames.map((frame) => [frame.nodeId, frame]));
  const paintRankByNodeId = parsePaintOrder(runtimeCase.report, fixtureCase.id);
  for (let nodeIndex = 0; nodeIndex < fixtureCase.nodes.length; nodeIndex += 1) {
    const expectedNode = fixtureCase.nodes[nodeIndex];
    const referenceNode = referenceCase.nodes[nodeIndex];
    const expectedNodeId = nodeIndex + 3;
    assert.equal(referenceNode.id, expectedNode.id, `${fixtureCase.id}: Chromium node 순서가 다릅니다.`);
    const actual = observedFrameByNodeId.get(expectedNodeId);
    assert.ok(actual, `${fixtureCase.id}/${expectedNode.id}: Android frame이 없습니다.`);
    for (const field of ['x', 'y', 'width', 'height']) {
      const offset = field === 'x' ? inventory.layout.originCssPx.x
        : field === 'y' ? inventory.layout.originCssPx.y : 0;
      const expected = referenceNode.frameRelativeToStage[field] + offset;
      const error = Math.abs(actual[field] - expected);
      if (error > maximumAbsoluteErrorCssPx) {
        maximumAbsoluteErrorCssPx = error;
        worstField = {
          case: fixtureCase.id,
          node: expectedNode.id,
          field,
          actual: actual[field],
          expected,
        };
      }
      if (error > toleranceCssPx) {
        mismatchDetails.push({ case: fixtureCase.id, node: expectedNode.id, field, actual: actual[field], expected, error });
      }
    }
    comparedNodes += 1;
  }

  for (const sample of referenceCase.samples) {
    const expectedTopToBottom = sample.hitTestTopToBottom;
    if (expectedTopToBottom.length < 2) continue;
    const ranked = expectedTopToBottom.map((nodeId) => {
      const nodeIndex = fixtureCase.nodes.findIndex(({ id }) => id === nodeId);
      assert.notEqual(nodeIndex, -1, `${fixtureCase.id}/${sample.id}: reference node가 fixture에 없습니다.`);
      const runtimeNodeId = nodeIndex + 3;
      assert.ok(paintRankByNodeId.has(runtimeNodeId), `${fixtureCase.id}/${sample.id}: paint rank가 없습니다.`);
      return { nodeId, rank: paintRankByNodeId.get(runtimeNodeId) };
    });
    const actualTopToBottom = [...ranked]
      .sort((left, right) => right.rank - left.rank)
      .map(({ nodeId }) => nodeId);
    if (actualTopToBottom.join('\0') !== expectedTopToBottom.join('\0')) {
      auxiliaryPaintOrderDifferences.push({
        case: fixtureCase.id,
        sample: sample.id,
        kind: 'runtime-paint-rank-vs-Chromium-elementsFromPoint',
        runtimePaintOrderCandidateTopToBottom: actualTopToBottom,
        chromiumHitTestTopToBottom: expectedTopToBottom,
        oracleLimit: 'Chromium elementsFromPoint는 paint-order oracle이 아니며 이 실행의 scene별 surface pixel readback이 없습니다.',
      });
    }
    comparedHitTestObservations += 1;
  }
}

const summaryLine = iosCapture
  ? `SPINON_C124_MATRIX_SUMMARY status=matched cases=${iosCapture.cases} nodes=${iosCapture.nodes}`
    + ` expected_ready=${iosCapture.expectedReady} expected_rejected=${iosCapture.expectedRejected}`
    + ` matched=${iosCapture.matched}`
  : evidenceText.split(/\r?\n/).find((line) => line.includes('SPINON_C124_MATRIX_SUMMARY '));
assert.ok(summaryLine, 'Android runtime matrix summary가 없습니다.');
assert.match(summaryLine, /status=matched cases=26 nodes=77 expected_ready=21 expected_rejected=5 matched=26/);
assert.equal(runtimeCases.size, 26);
process.stdout.write(`${JSON.stringify({
  status: mismatchDetails.length === 0 ? 'matched' : 'mismatch',
  platform: iosCapture
    ? `${iosCapture.device} · iOS ${iosCapture.osVersion} Simulator`
    : 'Android API 37.1 simulator (16 KB page size)',
  cases: runtimeCases.size,
  supportedCases: supportedCases.length,
  rejectedCases: runtimeCases.size - supportedCases.length,
  comparedNodes,
  comparedHitTestObservations,
  maximumAbsoluteErrorCssPx,
  toleranceCssPx,
  worstField,
  auxiliaryPaintOrderDifferences,
  mismatches: mismatchDetails,
}, null, 2)}\n`);
assert.equal(mismatchDetails.length, 0, 'Android geometry/paint order differs from the pinned Chromium reference.');
