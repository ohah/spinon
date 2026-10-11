import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildC124RuntimeMatrix } from './build-c12-4-runtime-matrix.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const comparatorPath = join(repositoryRoot, 'tools/css-reference/compare-c12-4-runtime-matrix.mjs');

async function passingCapture() {
  const [matrix, inventory, reference] = await Promise.all([
    buildC124RuntimeMatrix(),
    import('../../tests/fixtures/css/c12/stacking-context-inventory.json', { with: { type: 'json' } })
      .then(({ default: value }) => value),
    import('../../tests/fixtures/css/references/c12-4-stacking-context-v1.json', { with: { type: 'json' } })
      .then(({ default: value }) => value),
  ]);
  const results = matrix.cases.map((fixtureCase, index) => {
    const expected = fixtureCase.expected.kind === 'supported' ? 'ready' : 'reject';
    let report;
    if (expected === 'reject') {
      const diagnostic = fixtureCase.expected.property === 'pseudo-element'
        ? 'cascade_error=지원하지 않는 CSS 선언 content'
        : fixtureCase.expected.property
          ? `node=Some(3) property=Some("${fixtureCase.expected.property}")`
          : 'TypeError: showModal is not a function';
      report = `status=-1 op=eval ${diagnostic}`;
    } else {
      const referenceCase = reference.observations[0].cases[index];
      const frames = [
        '0:node=1,x=0,y=0,width=320,height=240',
        '1:node=2,x=0,y=0,width=280,height=200',
        ...referenceCase.nodes.map((node, nodeIndex) => {
          const expectedNode = fixtureCase.nodeIds[nodeIndex];
          assert.equal(node.id, expectedNode);
          const x = node.frameRelativeToStage.x + inventory.layout.originCssPx.x;
          const y = node.frameRelativeToStage.y + inventory.layout.originCssPx.y;
          return `${nodeIndex + 2}:node=${nodeIndex + 3},x=${x},y=${y},width=${node.frameRelativeToStage.width},height=${node.frameRelativeToStage.height}`;
        }),
      ];
      if (fixtureCase.stylesheetRules.length > 0) {
        frames.push(`${frames.length}:node=${fixtureCase.nodeIds.length + 3},x=0,y=0,width=0,height=0`);
      }
      const paintOrder = frames.map((_, frameIndex) => `${frameIndex}:${frameIndex + 1}`).join(',');
      report = `status=0 layout=ready node_frames_css_px=[${frames.join(';')}] paint_node_order=[${paintOrder}]`;
    }
    return {
      index,
      id: fixtureCase.id,
      expected,
      actual: expected,
      reasonMatched: true,
      matched: true,
      report,
    };
  });
  return {
    schema: 'spinon-c124-ios-runtime-matrix/v1',
    runID: 'c124-ios-matrix-test-run',
    platform: 'iOS Simulator',
    device: 'iPhone',
    osVersion: '26.2',
    inventorySHA256: matrix.inventory.sha256,
    status: 'matched',
    cases: 26,
    nodes: 77,
    expectedReady: 21,
    expectedRejected: 5,
    matched: 26,
    results,
    error: null,
  };
}

function compare(capture) {
  const directory = mkdtempSync(join(tmpdir(), 'spinon-c124-matrix-'));
  try {
    const evidencePath = join(directory, 'capture.json');
    writeFileSync(evidencePath, JSON.stringify(capture));
    return spawnSync(process.execPath, [comparatorPath, evidencePath], {
      cwd: repositoryRoot,
      encoding: 'utf8',
      timeout: 30_000,
    });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

test('공통 비교기는 정상 iOS matrix에서 모든 Chromium frame을 확인한다', async () => {
  const result = compare(await passingCapture());
  assert.equal(result.status, 0, result.stderr);
  const comparison = JSON.parse(result.stdout);
  assert.equal(comparison.status, 'matched');
  assert.equal(comparison.platform, 'iPhone · iOS 26.2 Simulator');
  assert.equal(comparison.cases, 26);
  assert.equal(comparison.supportedCases, 21);
  assert.equal(comparison.rejectedCases, 5);
  assert.equal(comparison.comparedNodes, 64);
  assert.equal(comparison.maximumAbsoluteErrorCssPx, 0);
});

test('공통 비교기는 잘못된 inventory hash를 거부한다', async () => {
  const capture = await passingCapture();
  capture.inventorySHA256 = '0'.repeat(64);
  const result = compare(capture);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /inventory hash가 다릅니다/);
});

test('공통 비교기는 중복 case와 허용치를 넘는 개별 frame 오차를 거부한다', async () => {
  const duplicateCase = await passingCapture();
  duplicateCase.results[1].id = duplicateCase.results[0].id;
  const duplicateResult = compare(duplicateCase);
  assert.notEqual(duplicateResult.status, 0);
  assert.match(duplicateResult.stderr, /중복되었습니다/);

  const geometryMismatch = await passingCapture();
  geometryMismatch.results[0].report = geometryMismatch.results[0].report.replace(
    'x=30,y=30,width=110,height=60', 'x=31,y=30,width=110,height=60',
  );
  const geometryResult = compare(geometryMismatch);
  assert.notEqual(geometryResult.status, 0);
  assert.match(geometryResult.stderr, /frame differs|geometry\/paint order/);
});

test('공통 비교기는 기대 속성과 다른 거부 진단을 통과시키지 않는다', async () => {
  const capture = await passingCapture();
  const rejected = capture.results.find(({ expected }) => expected === 'reject');
  rejected.report = 'status=-1 op=eval node=Some(3) property=Some("z-index")';
  const result = compare(capture);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /진단이 없습니다/);
});
