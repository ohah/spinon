import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const files = {
  inventory: 'tests/fixtures/css/c12/position-fixed-inventory.json',
  html: 'tests/fixtures/css/c12/position-fixed.html',
  capture: 'tools/css-reference/capture-c12-3-fixed-positioning.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c12-3-position-fixed-v1.json',
};
const load = async path => readFile(join(repositoryRoot, path));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const [inventoryBytes, htmlBytes, inventory, reference] = await Promise.all([
  load(files.inventory),
  load(files.html),
  load(files.inventory).then(bytes => JSON.parse(bytes.toString('utf8'))),
  load(files.reference).then(bytes => JSON.parse(bytes.toString('utf8'))),
]);
const nodeMap = observation => new Map(observation.nodes.map(node => [node.id, node]));
const closeRect = (actual, expected, tolerance = 0.0001) => {
  for (const field of ['x', 'y', 'width', 'height']) {
    assert.ok(Math.abs(actual[field] - expected[field]) <= tolerance,
      `${field}: expected ${expected[field]}, got ${actual[field]}`);
  }
};

test('C12.3 기준은 고정 Chrome, fixture 해시, viewport/DPR 행렬과 WPT subset에 묶인다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c12-3-position-fixed-inventory/v1');
  assert.equal(inventory.fixtureId, 'C12.3-viewport-fixed-v1');
  assert.equal(reference.schema, 'spinon-css-c12-3-position-fixed-reference/v1');
  assert.equal(reference.referenceId, 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-3');
  assert.equal(reference.chromium.version, '154.0.8037.98');
  assert.equal(reference.chromium.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.match(reference.chromium.binarySha256, /^[0-9a-f]{64}$/);
  assert.equal(reference.cssPosition.url, 'https://www.w3.org/TR/2025/WD-css-position-3-20251007/');
  assert.equal(reference.cssPosition.status, 'W3C Working Draft');
  assert.equal(reference.wpt.revision, '9ec154ff43db468923997c08bb08f905ceab62a5');
  assert.equal(reference.wpt.execution, 'not-run');
  assert.deepEqual(reference.observations.map(({ environment }) => [
    environment.width, environment.height, environment.deviceScaleFactor,
  ]), [
    [360, 800, 1], [360, 800, 2], [360, 800, 2.625], [360, 800, 3],
    [390, 844, 1], [390, 844, 2], [390, 844, 2.625], [390, 844, 3],
  ]);
  assert.equal(reference.observations.length, 8);
  assert.equal(reference.resizeObservations.length, 4);
  assert.equal(reference.inventorySummary.cases, 22);
  assert.equal(reference.inventorySummary.nodes, 79);
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.ok(htmlBytes.toString('utf8').includes('overflow:hidden'));
  for (const observation of [...reference.observations, ...reference.resizeObservations]) {
    assert.equal(observation.environment.clientWidth, observation.environment.width);
    assert.equal(observation.environment.clientHeight, observation.environment.height);
    assert.equal(observation.environment.locale, 'en-US');
    assert.equal(observation.environment.timeZone, 'UTC');
  }

  for (const [path, expectedDigest] of [
    [files.inventory, reference.fixture.inventorySha256],
    [files.html, reference.fixture.htmlSha256],
    [files.capture, reference.captureTool.sha256],
    [files.helper, reference.captureTool.helperSha256],
  ]) {
    assert.equal(digest(await load(path)), expectedDigest, `${path} digest`);
  }
});

test('지원·거부 owner, source parent, 표시 여부와 유한 frame이 모든 관찰에서 보존된다', () => {
  const expectedIds = inventory.nodes.map(({ id }) => id);
  assert.equal(new Set(expectedIds).size, expectedIds.length);
  for (const observation of [...reference.observations, ...reference.resizeObservations]) {
    assert.deepEqual(observation.nodes.map(({ id }) => id), expectedIds, observation.id);
    const actual = nodeMap(observation);
    for (const expected of inventory.nodes) {
      const node = actual.get(expected.id);
      assert.ok(node, `${observation.id}/${expected.id} exists`);
      assert.equal(node.caseId, expected.caseId, `${observation.id}/${expected.id} case`);
      assert.equal(node.parentId, expected.parentId, `${observation.id}/${expected.id} source parent`);
      assert.equal(node.owner, expected.expectedOwner, `${observation.id}/${expected.id} owner`);
      assert.equal(node.properties.position, expected.expectedPosition,
        `${observation.id}/${expected.id} position`);
      assert.equal(node.hasLayoutBox, expected.hasLayoutBox ?? true,
        `${observation.id}/${expected.id} layout box`);
      for (const field of inventory.comparison.rectFields) {
        assert.ok(Number.isFinite(node.rect[field]), `${observation.id}/${expected.id} rect.${field}`);
      }
      if (['absolute', 'fixed'].includes(node.properties.position) && node.hasLayoutBox) {
        assert.ok(node.ownerProbe, `${observation.id}/${expected.id} owner origin probe`);
        assert.ok(Math.abs(node.ownerProbe.coordinateError.x)
          <= inventory.comparison.maximumAbsoluteRectErrorCssPx,
        `${observation.id}/${expected.id} owner x coordinate`);
        assert.ok(Math.abs(node.ownerProbe.coordinateError.y)
          <= inventory.comparison.maximumAbsoluteRectErrorCssPx,
        `${observation.id}/${expected.id} owner y coordinate`);
      }
    }
  }
});

test('DPR만 바꾸면 CSS geometry와 computed style이 그대로이고 resize 왕복/no-op도 결정적이다', () => {
  for (const width of [360, 390]) {
    const byDpr = reference.observations.filter(({ environment }) => environment.width === width);
    const baseline = byDpr[0];
    for (const observation of byDpr.slice(1)) {
      assert.deepEqual(observation.nodes, baseline.nodes,
        `${width}px viewport DPR ${observation.environment.deviceScaleFactor}`);
    }
  }
  const [start, outbound, noop, returned] = reference.resizeObservations;
  assert.equal(outbound.environment.width, 390);
  assert.equal(noop.environment.width, 390);
  assert.deepEqual(noop.nodes, outbound.nodes, '같은 viewport를 다시 적용한 no-op');
  assert.deepEqual(returned.nodes, start.nodes, '360×800으로 돌아온 frame');
});

test('viewport inset·percentage·calc·stretch와 box-owner 경계를 대표값으로 고정한다', () => {
  const nodes360 = nodeMap(reference.observations[0]);
  closeRect(nodes360.get('viewport-top-left').rect, { x: 17, y: 11, width: 30, height: 20 });
  closeRect(nodes360.get('viewport-zero-inset').rect, { x: 0, y: 0, width: 8, height: 8 });
  closeRect(nodes360.get('viewport-margin').rect, { x: 35, y: 43, width: 30, height: 20 });
  closeRect(nodes360.get('viewport-aspect-ratio').rect, { x: 50, y: 60, width: 40, height: 20 });
  closeRect(nodes360.get('viewport-right-bottom').rect, { x: 319, y: 770, width: 33, height: 21 });
  closeRect(nodes360.get('viewport-percent').rect, { x: 90, y: 80, width: 40, height: 18 });
  closeRect(nodes360.get('viewport-calc').rect, { x: 21, y: 21, width: 25, height: 14 });
  closeRect(nodes360.get('viewport-stretch').rect, { x: 20, y: 120, width: 315, height: 530 });
  closeRect(nodes360.get('viewport-minmax').rect, { x: 10, y: 170, width: 150, height: 18 });
  closeRect(nodes360.get('absolute-child-of-fixed').rect, { x: 217, y: 259, width: 20, height: 10 });
  assert.equal(nodes360.get('absolute-child-of-fixed').owner, 'fixed-parent');
  assert.equal(nodes360.get('fixed-descendant-of-fixed').owner, 'viewport');
  assert.equal(nodes360.get('fixed-under-hidden').owner, 'none');
  assert.equal(nodes360.get('fixed-under-hidden').hasLayoutBox, false);

  const nodes390 = nodeMap(reference.observations[4]);
  closeRect(nodes390.get('viewport-right-bottom').rect, { x: 349, y: 814, width: 33, height: 21 });
  closeRect(nodes390.get('viewport-percent').rect, { x: 97.5, y: 84.4, width: 40, height: 18 }, 0.01);
  closeRect(nodes390.get('viewport-calc').rect, { x: 22.5, y: 21.875, width: 25, height: 14 });
  closeRect(nodes390.get('viewport-stretch').rect, { x: 20, y: 120, width: 345, height: 574 });
});

test('fixed containing block 효과는 거부 경계로 분리되고 will-change opacity는 viewport 대조군이다', () => {
  const nodeIds = new Set(inventory.nodes.map(({ id }) => id));
  for (const boundary of inventory.negativeBoundaries) {
    for (const id of boundary.nodeIds ?? []) assert.ok(nodeIds.has(id), `${boundary.id}/${id}`);
  }
  const first = nodeMap(reference.observations[0]);
  for (const id of [
    'fixed-under-author-transform',
    'fixed-under-transform-style',
    'fixed-under-rotate',
    'fixed-under-scale',
    'fixed-under-translate',
    'fixed-under-filter',
    'fixed-under-backdrop-filter',
    'fixed-under-perspective',
    'fixed-under-contain-layout',
    'fixed-under-contain-paint',
    'fixed-under-author-contain',
    'fixed-under-content-visibility',
    'fixed-under-will-change-transform',
    'fixed-under-will-change-filter',
    'fixed-under-will-change-perspective',
    'fixed-under-will-change-contain',
  ]) {
    assert.notEqual(first.get(id).owner, 'viewport', `${id} establishes a fixed containing block in Chrome`);
  }
  assert.equal(first.get('fixed-under-author-transform').owner, 'author-transform-owner');
  assert.equal(first.get('fixed-under-author-contain').owner, 'author-contain-owner');
  assert.equal(first.get('fixed-under-will-change-opacity').owner, 'viewport');
  assert.equal(inventory.negativeBoundaries.find(({ id }) => id === 'unsupported-root-fixed-and-static-position-inputs').execution,
    'not-applicable-to-Chromium-geometry-capture');
  assert.equal(inventory.negativeBoundaries.find(({ id }) => id === 'invalid-viewport-and-stale-runtime-revision').execution,
    'runtime-only');
});
