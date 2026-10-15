import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const reference = JSON.parse(await readFile(resolve(
  repositoryRoot,
  'tests/fixtures/css/references/c13-3-style-invalidation-precomparison-v1.json',
), 'utf8'));

test('active overscroll mutation Chrome comparison remains distinct from Spinon contract', () => {
  assert.equal(reference.schema, 'spinon-css-c13-3-style-invalidation-precomparison/v1');
  assert.equal(reference.oracle.browser, 'Chrome/155.0.8059.40');
  assert.equal(reference.oracle.agentBrowserVersion, 'agent-browser 0.31.1');
  assert.deepEqual(reference.oracle.viewport, {
    width: 400,
    height: 240,
    deviceScaleFactor: 1,
  });
  assert.equal(reference.fixture.sha256.length, 64);
  assert.deepEqual(reference.screenshot.dimensions, [400, 240]);

  const { autoControl, activeOverscrollMutation } = reference.cases;
  assert.deepEqual(autoControl.initialOffsets.inner, [0, 106]);
  assert.deepEqual(autoControl.afterFirstMove.outer, [0, 15]);
  assert.deepEqual(autoControl.finalOffsets.outer, [0, 55]);
  assert.deepEqual(activeOverscrollMutation.policyAfterMutation, ['auto', 'contain']);
  assert.deepEqual(activeOverscrollMutation.afterFirstMove, autoControl.afterFirstMove);
  assert.deepEqual(activeOverscrollMutation.finalOffsets, autoControl.finalOffsets);
  assert.deepEqual(activeOverscrollMutation.finalPolicy, ['auto', 'contain']);
  assert.match(reference.scope, /not Spinon runtime or app verification/);
});
