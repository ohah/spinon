import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c04/runtime-author-stylesheets-inventory.json',
  html: 'tests/fixtures/css/c04/runtime-author-stylesheets.html',
  runtimeFixture: 'tests/fixtures/css/c04/runtime-author-stylesheets.js',
  capture: 'tools/css-reference/capture-c04-runtime-author-stylesheets.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c04-runtime-author-stylesheets-v1.json',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C04.11 Chromium reference is bound to the fixed author stylesheet fixture', async () => {
  const [inventoryBytes, htmlBytes, runtimeFixtureBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.runtimeFixture), read(paths.capture), read(paths.helper), read(paths.reference),
  ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));

  assert.equal(inventory.schema, 'spinon-css-runtime-author-stylesheets-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-runtime-author-stylesheets-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  const runtimeFixture = runtimeFixtureBytes.toString('utf8');
  assert.match(runtimeFixture, /document\.createElement\("style"\)/);
  assert.match(runtimeFixture, /document\.createTextNode\(/);
  assert.match(runtimeFixture, /--space:\s*11px/);
  assert.match(runtimeFixture, /#second\s*\{\s*width:\s*43px\s*!important/);
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Chromium');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.deepEqual(reference.observation.nodes.map(({ id }) => id), [
    'app', 'sheet-one', 'first', 'sheet-two', 'second',
  ]);
  const nodes = Object.fromEntries(reference.observation.nodes.map((node) => [node.id, node]));
  assert.deepEqual(nodes.app.properties, {
    display: 'flex', width: '301px', height: '100px', gap: '11px', 'background-color': 'rgb(18, 52, 86)',
  });
  assert.deepEqual(nodes.first.properties, {
    display: 'block', width: '47px', height: '14px', 'background-color': 'rgb(51, 102, 255)',
  });
  assert.deepEqual(nodes.second.properties, {
    display: 'block', width: '43px', height: '14px', 'background-color': 'rgb(51, 102, 255)',
  });
  assert.equal(nodes['sheet-one'].properties.display, 'none');
  assert.equal(nodes['sheet-two'].properties.display, 'none');
  assert.deepEqual(nodes.first.rect, { x: 0, y: 0, width: 47, height: 14 });
  assert.deepEqual(nodes.second.rect, { x: 58, y: 0, width: 43, height: 14 });
  assert.deepEqual(reference.observation.environment, {
    locale: 'en-US', intlLocale: 'en-US', timeZone: 'UTC', dark: false, coarsePointer: true, hover: false,
  });
});
