import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const files = {
  inventory: 'tests/fixtures/css/c12/position-absolute-block-inventory.json',
  html: 'tests/fixtures/css/c12/position-absolute-block.html',
  runtimeFixture: 'tests/fixtures/css/c12/runtime-position-absolute-block.js',
  capture: 'tools/css-reference/capture-c12-2-absolute-block.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c12-2-position-absolute-block-v1.json',
};
const load = async path => readFile(join(repositoryRoot, path));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const [inventory, reference] = await Promise.all([
  load(files.inventory).then(bytes => JSON.parse(bytes.toString('utf8'))),
  load(files.reference).then(bytes => JSON.parse(bytes.toString('utf8'))),
]);
const nodesById = observation => new Map(observation.nodes.map(node => [node.id, node]));
const rect = (nodes, id) => nodes.get(id)?.rect;
const closeRect = (actual, expected, tolerance = 0.0001) => {
  for (const field of ['x', 'y', 'width', 'height']) {
    assert.ok(Math.abs(actual[field] - expected[field]) <= tolerance,
      `${field}: expected ${expected[field]}, got ${actual[field]}`);
  }
};

test('C12.2 Chromium 기준은 고정 버전·fixture·표준·WPT revision에 묶인다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c12-2-position-absolute-inventory/v1');
  assert.equal(inventory.fixtureId, 'C12.2-block-absolute-v1');
  assert.equal(reference.schema, 'spinon-css-c12-2-position-absolute-reference/v1');
  assert.equal(reference.referenceId, 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-2');
  assert.equal(reference.chromium.version, '154.0.8037.98');
  assert.equal(reference.chromium.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.match(reference.chromium.binarySha256, /^[0-9a-f]{64}$/);
  assert.equal(reference.cssPosition.url, 'https://www.w3.org/TR/2025/WD-css-position-3-20251007/');
  assert.equal(reference.cssPosition.status, 'W3C Working Draft');
  assert.equal(reference.wpt.revision, '9ec154ff43db468923997c08bb08f905ceab62a5');
  assert.equal(reference.wpt.execution, 'not-run');
  assert.ok(reference.wpt.casePaths.includes('css/css-position/position-absolute-padding-percentage.html'));
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.equal(inventory.comparison.ownerMismatchIsFailure, true);
  assert.deepEqual(reference.observations.map(({ environment }) => environment.deviceScaleFactor), [1, 2]);

  for (const [path, expectedDigest] of [
    [files.inventory, reference.fixture.inventorySha256],
    [files.html, reference.fixture.htmlSha256],
    [files.runtimeFixture, reference.fixture.runtimeSourceSha256],
    [files.capture, reference.captureTool.sha256],
    [files.helper, reference.captureTool.helperSha256],
  ]) {
    assert.equal(digest(await load(path)), expectedDigest, `${path} digest`);
  }
});

test('23개 case와 82개 노드의 DOM 부모·case·position·containing-block owner가 보존된다', () => {
  assert.equal(inventory.cases.length, 23);
  assert.equal(inventory.nodes.length, 82);
  assert.equal(reference.observations.length, 2);
  const ids = inventory.nodes.map(({ id }) => id);
  assert.equal(new Set(ids).size, ids.length);
  for (const observation of reference.observations) {
    assert.deepEqual(observation.nodes.map(({ id }) => id), ids);
    const actual = nodesById(observation);
    for (const expected of inventory.nodes) {
      const node = actual.get(expected.id);
      assert.ok(node, `node exists: ${expected.id}`);
      assert.equal(node.caseId, expected.caseId, `${expected.id} case`);
      assert.equal(node.parentId, expected.parentId, `${expected.id} source parent`);
      assert.equal(node.properties.position, expected.expectedPosition, `${expected.id} position`);
      assert.equal(node.owner, expected.expectedOwner, `${expected.id} containing-block owner`);
      assert.equal(node.hasLayoutBox, expected.expectedOwner !== 'none', `${expected.id} box presence`);
      for (const field of inventory.comparison.rectFields) {
        assert.ok(Number.isFinite(node.rect[field]), `${expected.id} rect.${field}`);
      }
    }
  }
});

test('DPR 변경은 CSS px 좌표와 computed inset 값을 바꾸지 않는다', () => {
  const first = reference.observations[0];
  const second = reference.observations[1];
  assert.deepEqual(first.nodes.map(({ id, rect: value, properties }) => ({ id, rect: value, properties })),
    second.nodes.map(({ id, rect: value, properties }) => ({ id, rect: value, properties })));
});

test('padding edge, static ancestor skip, nested positioned owner, viewport fallback이 분리된다', () => {
  const nodes = nodesById(reference.observations[0]);
  assert.equal(nodes.get('viewport-absolute').owner, 'viewport');
  closeRect(rect(nodes, 'viewport-absolute'), { x: 17, y: 11, width: 30, height: 20 });
  assert.equal(nodes.get('edge-absolute').owner, 'edge-owner');
  closeRect(rect(nodes, 'edge-absolute'), { x: 9, y: 81, width: 30, height: 12 });
  assert.equal(nodes.get('edge-absolute').parentId, 'edge-static-wrapper');
  assert.equal(nodes.get('skip-absolute').owner, 'skip-owner');
  closeRect(rect(nodes, 'skip-absolute'), { x: 8, y: 146, width: 24, height: 10 });
  assert.equal(nodes.get('nested-absolute').owner, 'nested-inner');
  closeRect(rect(nodes, 'nested-absolute'), { x: 32, y: 232, width: 20, height: 12 });
});

test('out-of-flow 자식은 auto 부모 크기와 normal-flow 형제 좌표에 참여하지 않는다', () => {
  const nodes = nodesById(reference.observations[0]);
  closeRect(rect(nodes, 'flow-row'), { x: 0, y: 280, width: 300, height: 24 });
  closeRect(rect(nodes, 'flow-before'), { x: 0, y: 280, width: 40, height: 12 });
  closeRect(rect(nodes, 'flow-after'), { x: 0, y: 292, width: 40, height: 12 });
  assert.deepEqual(reference.observations[0].flowProbes.find(({ id }) => id === 'flow-absolute').rect,
    { x: 0, y: 292, width: 50, height: 16 });
});

test('축별 percentage·calc·right/bottom·auto size와 over-constrained LTR 기준을 고정한다', () => {
  const nodes = nodesById(reference.observations[0]);
  assert.equal(nodes.get('percent-absolute').properties.left, '48px');
  assert.equal(nodes.get('percent-absolute').properties.top, '11.1875px');
  closeRect(rect(nodes, 'percent-absolute'), { x: 52, y: 389.1875, width: 20, height: 10 });
  assert.equal(nodes.get('math-absolute').properties.left, '23.1875px');
  assert.equal(nodes.get('math-absolute').properties.top, '13.1875px');
  closeRect(rect(nodes, 'math-absolute'), { x: 27.1875, y: 461.1875, width: 20, height: 10 });
  closeRect(rect(nodes, 'opposite-absolute'), { x: 158, y: 341, width: 31, height: 13 });
  closeRect(rect(nodes, 'stretch-absolute'), { x: 12, y: 523, width: 172, height: 60 });
  closeRect(rect(nodes, 'over-absolute'), { x: 9, y: 590, width: 30, height: 12 });
});

test('자동 margin, min/max, box-sizing, ratio와 negative inset을 별도 결과로 고정한다', () => {
  const nodes = nodesById(reference.observations[0]);
  closeRect(rect(nodes, 'margin-absolute'), { x: 80, y: 661, width: 40, height: 12 });
  closeRect(rect(nodes, 'minmax-absolute'), { x: 10, y: 729, width: 130, height: 12 });
  closeRect(rect(nodes, 'border-box-absolute'), { x: 4, y: 798, width: 60, height: 30 });
  closeRect(rect(nodes, 'content-box-absolute'), { x: 80, y: 798, width: 80, height: 50 });
  closeRect(rect(nodes, 'edge-margin-absolute'), { x: 3, y: 1171, width: 32, height: 12 });
  closeRect(rect(nodes, 'ratio-absolute'), { x: 5, y: 1314, width: 40, height: 20 });
});

test('auto inset static position, inline blockification, BFC, 숨김 subtree와 nested absolute를 구분한다', () => {
  const nodes = nodesById(reference.observations[0]);
  const flow = reference.observations[0].flowProbes.find(({ id }) => id === 'static-absolute').rect;
  assert.deepEqual(flow, { x: 0, y: 876, width: 40, height: 10 });
  closeRect(rect(nodes, 'static-absolute'), flow);
  assert.equal(nodes.get('static-diff-absolute').parentId, 'static-diff-wrapper');
  assert.equal(nodes.get('static-diff-absolute').owner, 'static-diff-owner');
  assert.notEqual(nodes.get('static-diff-absolute').parentId, nodes.get('static-diff-absolute').owner);
  const distinctOwnerFlow = reference.observations[0].flowProbes
    .find(({ id }) => id === 'static-diff-absolute').rect;
  closeRect(rect(nodes, 'static-diff-absolute'), distinctOwnerFlow);
  assert.equal(nodes.get('inline-absolute').properties.display, 'block');
  closeRect(rect(nodes, 'inline-absolute'), { x: 3, y: 894, width: 30, height: 12 });
  closeRect(rect(nodes, 'bfc-absolute'), { x: 3, y: 964, width: 80, height: 22 });
  assert.equal(nodes.get('hidden-absolute').owner, 'none');
  assert.equal(nodes.get('hidden-absolute').hasLayoutBox, false);
  assert.equal(nodes.get('abs-nested').owner, 'abs-parent');
  closeRect(rect(nodes, 'abs-nested'), { x: 26, y: 1117, width: 18, height: 9 });
});

test('shorthand cascade와 한쪽 auto inset의 used geometry가 computed style과 함께 유지된다', () => {
  const nodes = nodesById(reference.observations[0]);
  assert.deepEqual(
    ['top', 'right', 'bottom', 'left'].map(name => nodes.get('cascade-absolute').properties[name]),
    ['5px', '7px', '9px', '17px'],
  );
  closeRect(rect(nodes, 'cascade-absolute'), { x: 19, y: 1247, width: 30, height: 12 });
  assert.equal(nodes.get('single-auto-absolute').properties.left, '159px');
  closeRect(rect(nodes, 'single-auto-absolute'), { x: 161, y: 1388, width: 24, height: 10 });
});

for (const [platform, logPath] of [
  ['Android physical device', 'spec/internal/evidence/c12-2-absolute-block-2026-10-11/android-physical.log'],
  ['iOS Simulator', 'spec/internal/evidence/c12-2-absolute-block-2026-10-11/ios-simulator.log'],
]) {
  test(`${platform}의 최종 표시 장면과 82개 runtime node가 Chrome 기하와 일치한다`, () => {
    const result = spawnSync(process.execPath, [
      join(repositoryRoot, 'tools/css-reference/compare-c12-2-absolute-block-runtime.mjs'),
      join(repositoryRoot, logPath),
    ], { cwd: repositoryRoot, encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr || result.stdout);
    const comparison = JSON.parse(result.stdout);
    assert.equal(comparison.status, 'matched');
    assert.equal(comparison.platform, platform);
    assert.equal(comparison.matchedNodes, 82);
    assert.equal(comparison.toleranceCssPx, 0.5);
    assert.ok(comparison.maximumAbsoluteErrorCssPx <= comparison.toleranceCssPx);
  });
}
