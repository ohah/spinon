import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-placement-inventory.json',
  html: 'tests/fixtures/css/c11/grid-placement.html',
  capture: 'tools/css-reference/capture-c11-grid-placement-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json',
};
const expectedWpt = [
  ['css/css-grid/placement/grid-layout-placement-shorthands.html', 'ab982c18feb208901fdff28be607ef4c63abadc1260eebed382856e0be59efb5'],
  ['css/css-grid/placement/grid-layout-grid-span.html', '1be809e54f1b7398112923acd60ef423735934b528ff42c1a8f3b326adbebef7'],
  ['css/css-grid/placement/grid-negative-line-placement-001.html', 'e5d17120d87170282d2522ec483d751c6f7db9ac23ec09d2a540cce305d81d31'],
  ['css/css-grid/placement/grid-placement-using-named-grid-lines-001.html', 'd31717c0e82d8030f0a99070a5e98333be71c2cc834e36950ae5b5d303fd180e'],
];
const expectedParents = {
  'c1121-numeric-grid': null, 'c1121-positive-lines': 'c1121-numeric-grid', 'c1121-negative-lines': 'c1121-numeric-grid',
  'c1121-span-grid': null, 'c1121-span-from-start': 'c1121-span-grid', 'c1121-span-from-end': 'c1121-span-grid',
  'c1121-reverse-grid': null, 'c1121-reversed-item': 'c1121-reverse-grid',
  'c1121-shorthand-grid': null, 'c1121-grid-area-item': 'c1121-shorthand-grid', 'c1121-cascade-item': 'c1121-shorthand-grid',
  'c1121-overlap-grid': null, 'c1121-overlap-first': 'c1121-overlap-grid', 'c1121-overlap-second': 'c1121-overlap-grid',
  'c1121-nested-outer': null, 'c1121-nested-auto-grid': 'c1121-nested-outer',
  'c1121-nested-auto-a': 'c1121-nested-auto-grid', 'c1121-nested-auto-b': 'c1121-nested-auto-grid',
  'c1121-nested-explicit-grid': 'c1121-nested-outer',
  'c1121-nested-explicit-a': 'c1121-nested-explicit-grid', 'c1121-nested-explicit-b': 'c1121-nested-explicit-grid',
  'c1121-implicit-grid': null, 'c1121-implicit-item': 'c1121-implicit-grid',
  'c1121-named-grid': null, 'c1121-named-item': 'c1121-named-grid',
  'c1121-mixed-grid': null, 'c1121-mixed-definite': 'c1121-mixed-grid', 'c1121-mixed-auto': 'c1121-mixed-grid',
  'c1121-equal-grid': null, 'c1121-equal-item': 'c1121-equal-grid',
};

const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventory = JSON.parse((await read(paths.inventory)).toString('utf8'));
const reference = JSON.parse((await read(paths.reference)).toString('utf8'));
const getCase = (observation, id) => observation.cases.find((item) => item.id === id);
const getNode = (observation, id) => observation.cases.flatMap((item) => item.nodes).find((item) => item.id === id);
const getRect = (observation, id) => {
  const value = getNode(observation, id)?.rect;
  assert.ok(value, `기준 node가 없습니다: ${id}`);
  return [value.x, value.y, value.width, value.height];
};

function assertObservationContract(candidate) {
  assert.equal(candidate.schema, 'spinon-css-c11-2-1-grid-placement-precomparison/v1');
  assert.deepEqual(candidate.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  for (const observation of candidate.observations) {
    assert.equal(observation.cases.length, inventory.cases.length);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
    for (const fixtureCase of inventory.cases) {
      const actual = getCase(observation, fixtureCase.id);
      assert.ok(actual, `scene 누락: ${fixtureCase.id}`);
      assert.equal(actual.scope, fixtureCase.scope);
      assert.deepEqual(actual.viewport, {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor: observation.deviceScaleFactor,
      });
      assert.deepEqual(actual.environment, {
        direction: inventory.environment.direction,
        writingMode: inventory.environment.writingMode,
      });
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodeIds);
      assert.equal(actual.nodes[0].id, fixtureCase.rootId);
      const seen = new Set();
      for (const node of actual.nodes) {
        assert.equal(node.parentId, expectedParents[node.id], `${fixtureCase.id}/${node.id}: parent`);
        assert.ok(node.parentId === null || seen.has(node.parentId), `${fixtureCase.id}/${node.id}: parent 순서`);
        assert.deepEqual(Object.keys(node.properties), inventory.comparison.computedProperties);
        assert.equal(typeof node.authoredStyle, 'string');
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}가 유한하지 않습니다`);
        }
        seen.add(node.id);
      }
    }
  }
  const comparable = (observation) => observation.cases.map(({ id, scope, rootId, rootRect, environment, nodes }) => (
    { id, scope, rootId, rootRect, environment, nodes }
  ));
  assert.deepEqual(comparable(candidate.observations[0]), comparable(candidate.observations[1]),
    'DPR 1과 2에서 CSS computed longhand 또는 CSS px frame이 달라졌습니다');
}

test('기준 입력·Chrome 실행 파일·캡처 도구와 WPT 원문 후보가 hash로 고정된다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper),
  ]);
  assert.equal(inventory.schema, 'spinon-css-c11-2-1-grid-placement-inventory/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Google Chrome');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.match(reference.oracle.executableSha256, /^[0-9a-f]{64}$/);
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.equal(reference.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
  assert.deepEqual(reference.wpt.candidates.map(({ path, sha256: digest }) => [path, digest]), expectedWpt);
  assert.equal(reference.wpt.candidates.some(({ stage }) => stage.includes('Spinon WPT 통과')), false);
  assert.ok(reference.referenceId.endsWith(
    `${sha256(inventoryBytes).slice(0, 12)}-${sha256(htmlBytes).slice(0, 12)}-${sha256(captureBytes).slice(0, 12)}`,
  ));
});

test('10개 scene·30개 node의 분류, DOM preorder, parent identity와 DPR이 고정된다', () => {
  assert.equal(inventory.cases.length, 10);
  assert.equal(inventory.cases.flatMap(({ nodeIds }) => nodeIds).length, 30);
  assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size, 30);
  assert.deepEqual(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').map(({ id }) => id), [
    'numeric-lines', 'positive-spans', 'reversed-lines', 'shorthand-cascade', 'overlap-source-order', 'nested-placement-modes',
  ]);
  assert.deepEqual(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only').map(({ id }) => id), [
    'deferred-implicit-track', 'deferred-named-line', 'deferred-mixed-auto-explicit', 'deferred-equal-lines',
  ]);
  assert.equal(Object.keys(expectedParents).length, 30);
  assert.deepEqual(Object.keys(expectedParents).sort(), inventory.cases.flatMap(({ nodeIds }) => nodeIds).sort());
  assertObservationContract(reference);
});

test('양·음수 line, span, 역순 및 shorthand cascade의 Chrome computed 값과 rect가 보존된다', () => {
  const observation = reference.observations[0];
  const props = (id) => getNode(observation, id).properties;
  assert.deepEqual(getRect(observation, 'c1121-positive-lines'), [0, 0, 165, 40]);
  assert.deepEqual(getRect(observation, 'c1121-negative-lines'), [170, 45, 80, 40]);
  assert.equal(props('c1121-negative-lines')['grid-column-start'], '-2');
  assert.equal(props('c1121-negative-lines')['grid-row-start'], '-2');
  assert.deepEqual(getRect(observation, 'c1121-span-from-start'), [0, 0, 165, 85]);
  assert.deepEqual(getRect(observation, 'c1121-span-from-end'), [85, 45, 165, 40]);
  assert.equal(props('c1121-span-from-start')['grid-column-end'], 'span 2');
  assert.equal(props('c1121-span-from-end')['grid-column-start'], 'span 2');
  assert.deepEqual(getRect(observation, 'c1121-reversed-item'), [0, 0, 165, 40]);
  assert.equal(props('c1121-reversed-item')['grid-column-start'], '3');
  assert.equal(props('c1121-reversed-item')['grid-column-end'], '1');
  assert.equal(props('c1121-reversed-item')['grid-row-start'], '2');
  assert.equal(props('c1121-reversed-item')['grid-row-end'], '1');
  assert.equal(props('c1121-grid-area-item')['grid-row-end'], '3');
  assert.deepEqual(getRect(observation, 'c1121-grid-area-item'), [0, 0, 165, 85]);
  assert.equal(props('c1121-cascade-item')['grid-column-start'], '1');
  assert.equal(props('c1121-cascade-item')['grid-column-end'], '4');
  assert.deepEqual(getRect(observation, 'c1121-cascade-item'), [0, 0, 250, 40]);
});

test('겹침·중첩별 배치 모드와 profile 밖의 Chrome 관찰값을 분리한다', () => {
  const observation = reference.observations[0];
  assert.deepEqual(getRect(observation, 'c1121-overlap-first'), [0, 0, 165, 85]);
  assert.deepEqual(getRect(observation, 'c1121-overlap-second'), [0, 0, 165, 85]);
  assert.ok(getCase(observation, 'overlap-source-order').nodes.findIndex(({ id }) => id === 'c1121-overlap-first')
    < getCase(observation, 'overlap-source-order').nodes.findIndex(({ id }) => id === 'c1121-overlap-second'));
  assert.deepEqual(getRect(observation, 'c1121-nested-auto-a'), [0, 0, 20, 20]);
  assert.deepEqual(getRect(observation, 'c1121-nested-auto-b'), [65, 0, 20, 20]);
  assert.deepEqual(getRect(observation, 'c1121-nested-explicit-a'), [195, 0, 55, 60]);
  assert.deepEqual(getRect(observation, 'c1121-nested-explicit-b'), [130, 0, 55, 60]);
  assert.equal(getCase(observation, 'deferred-implicit-track').scope, 'chromium-oracle-only');
  assert.equal(getNode(observation, 'c1121-named-item').properties['grid-column-start'], 'left');
  assert.equal(getNode(observation, 'c1121-mixed-auto').properties['grid-column-start'], 'auto');
  assert.equal(getNode(observation, 'c1121-equal-item').properties['grid-column-start'], '2');
  assert.equal(getNode(observation, 'c1121-equal-item').properties['grid-column-end'], '2');
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('Chrome 캡처 PNG는 scene·SHA-256·화면 크기와 연결된다', async () => {
  assert.deepEqual(reference.screenshots.map(({ caseId }) => caseId), inventory.cases.map(({ id }) => id));
  assert.equal(new Set(reference.screenshots.map(({ path }) => path)).size, inventory.cases.length);
  for (const screenshot of reference.screenshots) {
    assert.ok(screenshot.path.startsWith('spec/internal/evidence/c11-2-1-explicit-grid-placement-precomparison-2026-10-12/'));
    assert.ok(screenshot.path.includes(`/${reference.runId}/`));
    assert.equal(screenshot.path.split('/').includes('..'), false);
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, `${screenshot.caseId}: PNG hash`);
    assert.deepEqual([...bytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10]);
    assert.equal(bytes.readUInt32BE(16), inventory.viewport.width);
    assert.equal(bytes.readUInt32BE(20), inventory.viewport.height);
  }
});

test('oracle 검증은 누락·중복 node, 비유한 frame 및 profile 오분류를 거부한다', () => {
  const missing = structuredClone(reference);
  missing.observations[0].cases[0].nodes.pop();
  assert.throws(() => assertObservationContract(missing), assert.AssertionError);

  const duplicate = structuredClone(reference);
  duplicate.observations[0].cases[0].nodes[1].id = duplicate.observations[0].cases[0].nodes[0].id;
  assert.throws(() => assertObservationContract(duplicate), assert.AssertionError);

  const nonFinite = structuredClone(reference);
  nonFinite.observations[0].cases[0].nodes[1].rect.width = Number.NaN;
  assert.throws(() => assertObservationContract(nonFinite), assert.AssertionError);

  const scopeMismatch = structuredClone(reference);
  scopeMismatch.observations[0].cases[6].scope = 'implementation-candidate';
  assert.throws(() => assertObservationContract(scopeMismatch), assert.AssertionError);

  const dprMismatch = structuredClone(reference);
  dprMismatch.observations[1].cases[0].nodes[1].rect.x += 0.25;
  assert.throws(() => assertObservationContract(dprMismatch), assert.AssertionError);
});
