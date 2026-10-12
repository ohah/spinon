import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-layout-inventory.json',
  html: 'tests/fixtures/css/c11/grid-layout.html',
  capture: 'tools/css-reference/capture-c11-grid-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-grid-precomparison-v2.json',
};
const expectedWpt = [
  ['css/css-grid/grid-definition/grid-layout-basic.html', 'f3a6ad6b049aa62f9c63b17b779cff9b930b3161a0abfe6013670b98fcc9ad57'],
  ['css/css-grid/grid-definition/fr-unit.html', '5002f2cacc37bf549fd55df44e82a8da28606e601f51575a2d06dd41e35c5390'],
  ['css/css-grid/grid-definition/grid-support-grid-template-columns-rows-001.html', '5136529e97cc6512f7ff11bf38a3186dcad214cab65bb4b54e6651bbb1f67bbe'],
  ['css/css-grid/placement/grid-auto-placement-implicit-tracks-001.html', '785012f37a838995bdd6491bd9d27034d21e2441152a067f050bc82fd993ad53'],
];
const expectedParents = {
  'c11-fixed-grid': null,
  'c11-fixed-a': 'c11-fixed-grid', 'c11-fixed-b': 'c11-fixed-grid',
  'c11-fixed-c': 'c11-fixed-grid', 'c11-fixed-d': 'c11-fixed-grid',
  'c11-percent-grid': null,
  'c11-percent-a': 'c11-percent-grid', 'c11-percent-b': 'c11-percent-grid',
  'c11-fr-grid': null, 'c11-fr-a': 'c11-fr-grid', 'c11-fr-b': 'c11-fr-grid',
  'c11-mixed-grid': null,
  'c11-mixed-a': 'c11-mixed-grid', 'c11-mixed-b': 'c11-mixed-grid', 'c11-mixed-c': 'c11-mixed-grid',
  'c11-auto-grid': null,
  'c11-auto-a': 'c11-auto-grid', 'c11-auto-b': 'c11-auto-grid', 'c11-auto-c': 'c11-auto-grid',
  'c11-auto-d': 'c11-auto-grid', 'c11-auto-e': 'c11-auto-grid',
  'c11-empty-grid': null,
  'c11-single-grid': null, 'c11-single-item': 'c11-single-grid',
  'c11-block-parent': null, 'c11-block-grid': 'c11-block-parent',
  'c11-block-a': 'c11-block-grid', 'c11-block-b': 'c11-block-grid',
  'c11-flex-parent': null, 'c11-flex-grid-a': 'c11-flex-parent',
  'c11-flex-a1': 'c11-flex-grid-a', 'c11-flex-a2': 'c11-flex-grid-a',
  'c11-flex-grid-b': 'c11-flex-parent', 'c11-flex-b1': 'c11-flex-grid-b',
  'c11-flex-b2': 'c11-flex-grid-b',
  'c11-grid-flex-parent': null, 'c11-grid-flex-child': 'c11-grid-flex-parent',
  'c11-grid-flex-a': 'c11-grid-flex-child', 'c11-grid-flex-b': 'c11-grid-flex-child',
  'c11-grid-block-child': 'c11-grid-flex-parent',
};

const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventory = JSON.parse((await read(paths.inventory)).toString('utf8'));
const reference = JSON.parse((await read(paths.reference)).toString('utf8'));
const fixtureCase = (observation, id) => observation.cases.find((item) => item.id === id);
const node = (observation, id) => observation.cases.flatMap((item) => item.nodes).find((item) => item.id === id);
const rect = (observation, id) => {
  const value = node(observation, id)?.rect;
  assert.ok(value, `reference node가 없습니다: ${id}`);
  return [value.x, value.y, value.width, value.height];
};

function assertObservationContract(candidate) {
  assert.equal(candidate.schema, 'spinon-css-c11-grid-precomparison/v1');
  assert.deepEqual(candidate.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  for (const observation of candidate.observations) {
    assert.equal(observation.cases.length, inventory.cases.length);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
    for (const fixture of inventory.cases) {
      const observed = fixtureCase(observation, fixture.id);
      assert.ok(observed, `scene이 없습니다: ${fixture.id}`);
      assert.deepEqual(observed.viewport, {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor: observation.deviceScaleFactor,
      }, fixture.id);
      assert.deepEqual(observed.environment, {
        direction: inventory.environment.direction,
        writingMode: inventory.environment.writingMode,
      }, fixture.id);
      for (const field of inventory.comparison.rectFields) {
        assert.ok(Number.isFinite(observed.rootRect[field]), `${fixture.id}/root/${field}가 유한하지 않습니다`);
      }
      assert.deepEqual(observed.nodes.map(({ id }) => id), fixture.nodeIds, fixture.id);
      assert.equal(new Set(observed.nodes.map(({ id }) => id)).size, fixture.nodeIds.length, fixture.id);
      assert.equal(observed.nodes[0].id, fixture.rootId, `${fixture.id}: root가 첫 node가 아닙니다`);
      assert.equal(observed.nodes[0].parentId, null, `${fixture.id}: root parent가 null이 아닙니다`);
      const seen = new Set();
      for (const item of observed.nodes) {
        assert.equal(item.parentId, expectedParents[item.id], `${fixture.id}/${item.id}: parent 불일치`);
        assert.ok(item.parentId === null || seen.has(item.parentId), `${fixture.id}/${item.id}: parent 선행 조건 위반`);
        assert.deepEqual(Object.keys(item.properties), inventory.comparison.computedProperties);
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(item.rect[field]), `${fixture.id}/${item.id}/${field}가 유한하지 않습니다`);
        }
        seen.add(item.id);
      }
    }
  }
  const normalizedCases = (observation) => observation.cases.map(({ id, rootId, rootRect, environment, nodes }) => ({
    id, rootId, rootRect, environment, nodes,
  }));
  assert.deepEqual(normalizedCases(candidate.observations[0]), normalizedCases(candidate.observations[1]),
    'DPR 1과 2 사이에 CSS computed 값 또는 CSS px geometry가 달라졌습니다');
}

test('고정 Chrome 기준은 입력·도구·실행 파일 hash와 WPT 원문 hash에 묶여 있다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper),
  ]);
  assert.equal(inventory.schema, 'spinon-css-c11-grid-inventory/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.ok(reference.referenceId.endsWith(
    `${sha256(inventoryBytes).slice(0, 12)}-${sha256(htmlBytes).slice(0, 12)}-${sha256(captureBytes).slice(0, 12)}`,
  ));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Google Chrome');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.oracle.executableSha256, 'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954');
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.equal(inventory.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
  assert.deepEqual(inventory.wpt.candidates.map(({ path, sha256: hash }) => [path, hash]), expectedWpt);
  assert.deepEqual(reference.wpt, inventory.wpt);
  assert.equal(reference.wpt.candidates.some(({ stage }) => stage.includes('Spinon WPT passed')), false);
});

test('10개 scene·40개 node의 tree, CSS px geometry와 DPR 고정값이 보존된다', () => {
  assert.equal(inventory.cases.length, 10);
  assert.equal(inventory.cases.flatMap(({ nodeIds }) => nodeIds).length, 40);
  assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size, 40);
  assert.equal(Object.keys(expectedParents).length, 40);
  assert.deepEqual(Object.keys(expectedParents).sort(), inventory.cases.flatMap(({ nodeIds }) => nodeIds).sort());
  assertObservationContract(reference);

  const dpr1 = reference.observations[0];
  for (const [id, columns, rows] of [
    ['c11-fixed-grid', '80px 100px 120px', '50px 70px'],
    ['c11-percent-grid', '80px 240px', '90px'],
    ['c11-fr-grid', '102.656px 205.344px', '90px'],
    ['c11-mixed-grid', '80px 90px 166px', '90px'],
    ['c11-auto-grid', '100px 100px', '40px 40px 40px'],
    ['c11-empty-grid', '80px 100px 120px', '90px'],
    ['c11-single-grid', '60px 80px 100px', '90px'],
  ]) {
    const observed = node(dpr1, id);
    assert.equal(observed.properties.display, 'grid', id);
    assert.equal(observed.properties['grid-template-columns'], columns, `${id} columns`);
    assert.equal(observed.properties['grid-template-rows'], rows, `${id} rows`);
  }
  for (const [id, expected] of [
    ['c11-fixed-b', [88, 0, 50, 24]], ['c11-fixed-c', [196, 0, 60, 26]],
    ['c11-fixed-d', [0, 56, 30, 18]], ['c11-percent-b', [80, 0, 30, 20]],
    ['c11-fr-b', [114.65625, 0, 30, 20]], ['c11-mixed-c', [194, 0, 30, 20]],
    ['c11-block-grid', [25, 10, 200, 60]], ['c11-flex-grid-b', [160, 0, 150, 60]],
    ['c11-grid-block-child', [170, 0, 20, 20]],
  ]) assert.deepEqual(rect(dpr1, id), expected, id);
  assert.deepEqual(
    ['c11-auto-a', 'c11-auto-b', 'c11-auto-c', 'c11-auto-d', 'c11-auto-e'].map((id) => rect(dpr1, id)),
    [[0, 0, 20, 20], [110, 0, 20, 20], [0, 50, 20, 20], [110, 50, 20, 20], [0, 100, 20, 20]],
  );
  assert.equal(fixtureCase(dpr1, 'empty-grid').nodes.length, 1);
  assert.equal(fixtureCase(dpr1, 'single-item-grid').nodes.length, 2);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('10개 Chrome 화면 캡처는 고유 scene·SHA-256·PNG 크기와 연결된다', async () => {
  const expectedIds = inventory.cases.map(({ id }) => id);
  assert.deepEqual(reference.screenshots.map(({ caseId }) => caseId), expectedIds);
  assert.equal(new Set(reference.screenshots.map(({ path }) => path)).size, expectedIds.length);
  for (const screenshot of reference.screenshots) {
    assert.ok(screenshot.path.startsWith('spec/internal/evidence/c11-grid-precomparison-2026-10-11/'));
    assert.ok(screenshot.path.includes(`/${reference.runId}/`));
    assert.equal(screenshot.path.split('/').includes('..'), false);
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, `${screenshot.caseId}: PNG hash`);
    assert.deepEqual([...bytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10], `${screenshot.caseId}: PNG signature`);
    assert.equal(bytes.readUInt32BE(16), inventory.viewport.width, `${screenshot.caseId}: PNG width`);
    assert.equal(bytes.readUInt32BE(20), inventory.viewport.height, `${screenshot.caseId}: PNG height`);
  }
});

test('reference 검증은 빠진·중복 node와 DPR 불일치를 거부한다', () => {
  const missingNode = structuredClone(reference);
  missingNode.observations[0].cases[0].nodes.pop();
  assert.throws(() => assertObservationContract(missingNode), assert.AssertionError);

  const duplicateNode = structuredClone(reference);
  duplicateNode.observations[0].cases[0].nodes[1].id = duplicateNode.observations[0].cases[0].nodes[0].id;
  assert.throws(() => assertObservationContract(duplicateNode), assert.AssertionError);

  const dprMismatch = structuredClone(reference);
  dprMismatch.observations[1].cases[0].nodes[1].rect.x += 0.25;
  assert.throws(() => assertObservationContract(dprMismatch), assert.AssertionError);

  const nonFinite = structuredClone(reference);
  nonFinite.observations[0].cases[0].nodes[1].rect.width = Number.NaN;
  assert.throws(() => assertObservationContract(nonFinite), assert.AssertionError);
});
