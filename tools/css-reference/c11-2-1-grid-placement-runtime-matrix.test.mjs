import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import test from 'node:test';
import { buildC1121GridPlacementRuntimeMatrix } from './build-c11-2-1-grid-placement-runtime-matrix.mjs';

const root = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-placement-inventory.json',
  html: 'tests/fixtures/css/c11/grid-placement.html',
  reference: 'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json',
};
const read = (path) => readFile(resolve(root, path));

test('runtime matrix preserves fixed scene classes, preorder, source hashes, and failure owners', async () => {
  const matrix = await buildC1121GridPlacementRuntimeMatrix();
  assert.equal(matrix.schema, 'spinon-css-c11-2-1-grid-placement-runtime-matrix/v1');
  assert.deepEqual(matrix.cases.map(({ id }) => id), [
    'numeric-lines', 'positive-spans', 'reversed-lines', 'shorthand-cascade',
    'overlap-source-order', 'nested-placement-modes', 'deferred-implicit-track',
    'deferred-named-line', 'deferred-mixed-auto-explicit', 'deferred-equal-lines',
  ]);
  assert.deepEqual(matrix.cases.slice(0, 6).map(({ expected }) => expected),
    Array.from({ length: 6 }, () => ({ outcome: 'success' })));
  assert.deepEqual(matrix.cases.slice(6).map(({ id, expected }) => [
    id, expected.outcome, expected.code, expected.nodeId, expected.property,
    expected.runtimeNodeId, expected.diagnosticIncludes ?? null,
  ]), [
    ['deferred-implicit-track', 'expected-rejection', 'invalid_grid_placement', 'c1121-implicit-item', 'grid-column-start', 3, null],
    ['deferred-named-line', 'expected-rejection', 'cascade_failed', null, null, null, 'grid-template-columns'],
    ['deferred-mixed-auto-explicit', 'expected-rejection', 'invalid_grid_placement', 'c1121-mixed-auto', 'grid-row-start', 4, null],
    ['deferred-equal-lines', 'expected-rejection', 'invalid_grid_placement', 'c1121-equal-item', 'grid-column-end', 3, null],
  ]);
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.nodeCount, 30);
  assert.equal(matrix.cases.reduce((sum, item) => sum + item.nodeIds.length, 0), 30);
  for (const item of matrix.cases) {
    assert.equal(item.rootId, item.nodeIds[0]);
    assert.equal(new Set(item.nodeIds).size, item.nodeIds.length);
    assert.ok(item.source.length > 0 && item.source.length <= 64 * 1024);
    assert.equal(item.sourceSha256.length, 64);
    assert.match(item.source, new RegExp(`sceneId:\\"${item.id}\\"`));
    assert.equal(item.parentIds.length, item.nodeIds.length);
  }
  const overlap = matrix.cases.find(({ id }) => id === 'overlap-source-order');
  assert.ok(overlap.source.indexOf('c1121-overlap-first') < overlap.source.indexOf('c1121-overlap-second'));
  const nested = matrix.cases.find(({ id }) => id === 'nested-placement-modes');
  assert.deepEqual(nested.parentIds, [
    null, 'c1121-nested-outer', 'c1121-nested-auto-grid', 'c1121-nested-auto-grid',
    'c1121-nested-outer', 'c1121-nested-explicit-grid', 'c1121-nested-explicit-grid',
  ]);
});

test('HTML edits that are not represented by the pinned reference fail closed', async () => {
  const [inventory, html, reference] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.reference),
  ]);
  const mutated = Buffer.from(html.toString('utf8').replace('grid-column-start:1;grid-column-end:3',
    'grid-column-start:1;grid-column-end:4'));
  assert.notEqual(mutated.toString('utf8'), html.toString('utf8'));
  await assert.rejects(
    buildC1121GridPlacementRuntimeMatrix({ inventory, html: mutated, reference }),
    /Expected values to be strictly equal/,
  );
});

test('generated source escapes literals and contains no caller-provided JavaScript', async () => {
  const matrix = await buildC1121GridPlacementRuntimeMatrix();
  const combined = matrix.cases.map(({ source }) => source).join('\n');
  assert.equal(combined.includes('throw new Error'), false);
  assert.equal(combined.includes('eval('), false);
  assert.equal(combined.includes('Function('), false);
  assert.ok(matrix.cases.every(({ source }) => source.includes('setAttribute("data-c1121-grid-node"')));
});
