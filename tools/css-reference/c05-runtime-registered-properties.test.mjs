import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c05/runtime-registered-properties-inventory.json',
  html: 'tests/fixtures/css/c05/runtime-registered-properties.html',
  runtimeFixture: 'tests/fixtures/css/c05/runtime-registered-properties.js',
  capture: 'tools/css-reference/capture-c05-runtime-registered-properties.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c05-runtime-registered-properties-v1.json',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C05.2 Chromium reference is tied to immutable HTML and V8 runtime fixtures', async () => {
  const [inventoryBytes, htmlBytes, runtimeFixtureBytes, captureBytes, helperBytes, referenceBytes] =
    await Promise.all([
      read(paths.inventory), read(paths.html), read(paths.runtimeFixture), read(paths.capture),
      read(paths.helper), read(paths.reference),
    ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const html = htmlBytes.toString('utf8');
  const runtimeFixture = runtimeFixtureBytes.toString('utf8');

  assert.equal(inventory.schema, 'spinon-css-runtime-registered-properties-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-runtime-registered-properties-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.fixture.runtimeFixturePath, paths.runtimeFixture);
  assert.equal(reference.fixture.runtimeFixtureSha256, sha256(runtimeFixtureBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Google Chrome');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.deepEqual(reference.observation.nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));

  const htmlRegistrations = [...html.matchAll(/@property\s+(--[\w-]+)/g)].map(([, name]) => name);
  const runtimeRegistrations = [...runtimeFixture.matchAll(/@property\s+(--[\w-]+)/g)].map(([, name]) => name);
  assert.deepEqual(runtimeRegistrations, htmlRegistrations);
  const normalizeCss = (css) => css.trim().replace(/\s+/g, ' ');
  const htmlSheets = [...html.matchAll(/<style id="sheet-[^"]+">([\s\S]*?)<\/style>/g)]
    .map(([, css]) => normalizeCss(css));
  const runtimeSheets = [...runtimeFixture.matchAll(/appendChild\(document\.createTextNode\(`([\s\S]*?)`\)\);/g)]
    .map(([, css]) => normalizeCss(css));
  assert.deepEqual(runtimeSheets, htmlSheets);
  assert.ok(html.indexOf('#late {') < html.indexOf('@property --late-width'));
  assert.ok(runtimeFixture.indexOf('#late {') < runtimeFixture.indexOf('@property --late-width'));
  for (const required of ['inherits: false', 'inherits: true', 'vendor-extension: ignored', '47px !important']) {
    assert.ok(html.includes(required), `HTML oracle에 ${required} 입력이 있어야 합니다`);
    assert.ok(runtimeFixture.includes(required), `V8 fixture에 ${required} 입력이 있어야 합니다`);
  }
  assert.match(runtimeFixture, /document\.createElement\("style"\)/);
  assert.match(runtimeFixture, /document\.createTextNode\(/);
  assert.match(runtimeFixture, /document\.appendChild\(app\)/);

  const nodes = Object.fromEntries(reference.observation.nodes.map((node) => [node.id, node]));
  assert.equal(nodes.app.properties.display, 'flex');
  assert.equal(nodes.app.properties.gap, '5px');
  assert.equal(nodes.app.properties['background-color'], 'rgb(18, 52, 86)');
  assert.equal(nodes.declared.properties.width, '41px');
  assert.equal(nodes.declared.properties['flex-shrink'], '0');
  assert.equal(nodes['inherit-parent'].properties['flex-shrink'], '0');
  assert.equal(nodes.default.properties.width, '23px');
  assert.equal(nodes.invalid.properties.width, '23px');
  assert.equal(nodes['not-inherited'].properties.width, '23px');
  assert.equal(nodes.inherited.properties.width, '19px');
  assert.equal(nodes.late.properties.width, '31px');
  assert.equal(nodes.unknown.properties.width, '13px');
  assert.equal(nodes['inline-priority'].properties.width, '47px');
  assert.equal(nodes['color-override'].properties.width, '11px');
  assert.equal(nodes['color-override'].properties['background-color'], 'rgb(255, 102, 0)');
  assert.equal(nodes.duplicate.properties['--duplicate'], '3');
  assert.equal(nodes.duplicate.rect.width, 0);
  assert.deepEqual(reference.transitions.sheetTwoMovedBeforeSheetOne.customProperty, '29px');
  assert.ok(reference.transitions.sheetTwoMovedBeforeSheetOne.rect.width > 0);
  assert.equal(reference.transitions.sheetOneDetached.customProperty, '3');
  assert.equal(reference.transitions.sheetOneDetached.rect.height, 0);
  assert.equal(reference.transitions.sheetOneReinsertedAfterSheetTwo.customProperty, '29px');
  assert.ok(reference.transitions.sheetOneReinsertedAfterSheetTwo.rect.width > 0);
  const transitions = [
    reference.transitions.sheetTwoMovedBeforeSheetOne,
    reference.transitions.sheetOneDetached,
    reference.transitions.sheetOneReinsertedAfterSheetTwo,
  ];
  for (const transition of transitions) {
    for (const value of [transition.rect.x, transition.rect.y, transition.rect.width, transition.rect.height]) {
      assert.ok(Number.isFinite(value), 'transition geometry must be finite');
    }
  }
  assert.deepEqual(reference.transitions.sheetOneDetached.rect, { x: 0, y: 0, width: 301, height: 0 });
  for (const transition of [
    reference.transitions.sheetTwoMovedBeforeSheetOne,
    reference.transitions.sheetOneReinsertedAfterSheetTwo,
  ]) {
    assert.equal(transition.customProperty, '29px');
    for (const [field, expected] of Object.entries({ x: 317, y: 0, width: 29, height: 14 })) {
      assert.ok(Math.abs(transition.rect[field] - expected) <= 0.5, `${field} differs from captured transition`);
    }
  }
  for (const node of reference.observation.nodes) {
    for (const value of [node.rect.x, node.rect.y, node.rect.width, node.rect.height]) {
      assert.ok(Number.isFinite(value), `${node.id} geometry must be finite`);
    }
  }
});
