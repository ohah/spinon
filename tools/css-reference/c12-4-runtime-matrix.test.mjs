import assert from 'node:assert/strict';
import test from 'node:test';
import { buildC124RuntimeMatrix } from './build-c12-4-runtime-matrix.mjs';

test('C12.4 Android matrix derives all pinned cases and nodes from the inventory', async () => {
  const matrix = await buildC124RuntimeMatrix();

  assert.equal(matrix.schema, 'spinon-css-c12-4-runtime-matrix/v1');
  assert.equal(matrix.inventory.path, 'tests/fixtures/css/c12/stacking-context-inventory.json');
  assert.match(matrix.inventory.sha256, /^[a-f0-9]{64}$/);
  assert.equal(matrix.summary.cases, 26);
  assert.equal(matrix.summary.nodes, 77);
  assert.equal(matrix.summary.expectedSupported, 21);
  assert.equal(matrix.summary.expectedRejected, 5);
  assert.equal(new Set(matrix.cases.map(({ id }) => id)).size, 26);
  assert.equal(matrix.cases.reduce((sum, item) => sum + item.nodeIds.length, 0), 77);
  assert.equal(matrix.cases.filter(({ expected }) => expected.kind === 'supported')
    .reduce((sum, item) => sum + item.samples.length, 0), 22);
});

test('C12.4 Android matrix keeps unsupported boundaries explicit and emits per-scene V8 sources', async () => {
  const matrix = await buildC124RuntimeMatrix();
  const byId = new Map(matrix.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));

  assert.deepEqual(byId.get('display-none-visibility-hidden').expected,
    { kind: 'css-rejection', property: 'visibility', diagnostic: 'visibility' });
  assert.deepEqual(byId.get('visibility-collapse-flex').expected,
    { kind: 'css-rejection', property: 'visibility', diagnostic: 'visibility' });
  assert.deepEqual(byId.get('absolute-owner-vs-opacity-context').expected,
    { kind: 'css-rejection', property: 'opacity', diagnostic: 'opacity' });
  assert.deepEqual(byId.get('top-layer-dialog').expected,
    { kind: 'api-rejection', api: 'showModal', diagnostic: 'showModal' });
  assert.deepEqual(byId.get('pseudo-element-paint').expected,
    { kind: 'css-rejection', property: 'pseudo-element', diagnostic: 'content' });
  assert.match(byId.get('cascade-layer-and-inline').source, /@layer c124-base/);
  assert.match(byId.get('registered-property-z-index').source, /@property --c124-registered-z/);
  assert.doesNotMatch(byId.get('custom-property-z-index').source, /\.c124-var-transform/);
  assert.match(byId.get('pseudo-element-paint').source, /::before/);
  assert.match(byId.get('top-layer-dialog').source, /showModal\(\)/);
  assert.deepEqual(byId.get('root-negative-flow').samples[0], {
    id: 'negative-under-flow',
    xCssPx: 75,
    yCssPx: 45,
    rgba: [229, 72, 77, 255],
  });
});
