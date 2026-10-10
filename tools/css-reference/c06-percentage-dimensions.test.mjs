import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  reference: 'tests/fixtures/css/references/c06-percentage-dimensions-v1.json',
  inventory: 'tests/fixtures/css/c06/percentage-dimensions-inventory.json',
  html: 'tests/fixtures/css/c06/percentage-dimensions.html',
  capture: 'tools/css-reference/capture-c06-percentage-dimensions.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C06.1 Chromium oracle is bound to immutable inputs and partial support scope', async () => {
  const [referenceBytes, inventoryBytes, htmlBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.reference), read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper),
  ]);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const nodes = reference.observation.nodes;

  assert.equal(reference.schema, 'spinon-css-c06-percentage-dimensions-reference/v1');
  assert.equal(reference.fixture.id, 'C06.1-percentage-dimensions-v1');
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(reference.environment.viewportCssPx, inventory.viewport);
  assert.equal(nodes.length, 22);
  assert.deepEqual(nodes.map(({ id, kind, properties }) => ({ id, kind, properties: Object.keys(properties) })), inventory.nodes);
  assert.equal(reference.comparison.geometry, 'each x, y, width, and height differs by at most 0.5 CSS px');
  assert.match(reference.comparison.fixtureConstraint, /No nonzero borders or positioned layout/);

  for (const node of nodes) {
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Number.isFinite(node.rect[field]), `${node.id}.${field} must be finite`);
    }
    for (const value of Object.values(node.properties)) {
      assert.equal(typeof value, 'string', `${node.id} CSSOM values must remain observations`);
    }
  }
});
