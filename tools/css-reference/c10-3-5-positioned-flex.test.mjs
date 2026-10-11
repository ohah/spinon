import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { runInNewContext } from 'node:vm';

const root = join(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c10/positioned-flex-inventory.json';
const htmlPath = 'tests/fixtures/css/c10/positioned-flex.html';
const capturePath = 'tools/css-reference/capture-c10-3-5-positioned-flex.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const referencePath = 'tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json';
const runtimeInventoryPath = 'tests/fixtures/css/c10/positioned-flex-runtime-inventory.json';
const runtimeSourcePath = 'tests/fixtures/css/c10/runtime-positioned-flex.js';

async function load(relativePath) {
  return readFile(join(root, relativePath));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C10.3.5 inventory owns unique nodes and pinned, unexecuted WPT inputs', async () => {
  const inventory = JSON.parse((await load(inventoryPath)).toString('utf8'));
  const expectedCaseIds = [
    'direct-row-center-end', 'direct-row-reverse', 'direct-column-center',
    'mixed-insets', 'different-containing-block', 'block-wrapper-static-position',
    'paint-order', 'nested-paint-scope', 'direct-column-reverse', 'wrap-two-lines',
    'wrap-reverse-two-lines', 'direct-align-self-values', 'negative-space-around',
    'negative-space-evenly', 'safe-center-overflow', 'unsafe-center-overflow',
    'margin-border-content-edge', 'auto-margin-zero', 'inset-axis-matrix', 'display-none',
  ];
  const nodeIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));

  assert.equal(inventory.schema, 'spinon-css-c10-3-5-positioned-flex-inventory/v1');
  assert.equal(inventory.fixtureId, 'C10.3.5-positioned-flex-v1');
  assert.deepEqual(inventory.cases.map(({ id }) => id), expectedCaseIds);
  assert.equal(new Set(nodeIds).size, nodeIds.length);
  assert.equal(inventory.wpt.revision, 'd5a765f1089ce6d3f72300281481edf3dddff7f3');
  assert.equal(inventory.wpt.execution, 'not-run');
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);

  for (const fixtureCase of inventory.cases) {
    const ids = fixtureCase.nodes.map(({ id }) => id);
    assert.equal(new Set(ids).size, ids.length, fixtureCase.id);
    for (const node of fixtureCase.nodes) {
      assert.equal(typeof node.style, 'string', `${fixtureCase.id}/${node.id} style`);
      assert.ok(node.children.every((child) => ids.includes(child)), `${fixtureCase.id}/${node.id}`);
    }
  }
});

test('pinned Chrome observations match inputs, preserve flow geometry, and separate paint phases', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    load(inventoryPath), load(htmlPath), load(capturePath), load(helperPath), load(referencePath),
  ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const expectedPaintOrder = [
    'c1035-paint-abs-later', 'c1035-paint-abs', 'c1035-paint-positive',
    'c1035-paint-zero', 'c1035-paint-negative',
  ];
  const paintFixture = inventory.cases.find(({ id }) => id === 'paint-order');

  assert.equal(reference.schema, 'spinon-css-c10-3-5-positioned-flex-reference/v1');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.equal(reference.captureTool.helperSha256, sha256(helperBytes));
  assert.equal(reference.observations.length, 2);

  for (const [index, observation] of reference.observations.entries()) {
    assert.equal(observation.viewport.deviceScaleFactor, inventory.viewport.deviceScaleFactors[index]);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
    const paintCase = observation.cases.find(({ id }) => id === 'paint-order');
    assert.deepEqual(paintCase.paintOrder, expectedPaintOrder);
    assert.deepEqual(paintCase.paintOrder, paintFixture.paintProbe.topToBottom);
    for (const fixtureCase of observation.cases) {
      const expectedCase = inventory.cases.find(({ id }) => id === fixtureCase.id);
      const captured = fixtureCase.nodes;
      assert.deepEqual(captured.map(({ id }) => id), expectedCase.nodes.map(({ id }) => id));
      for (const node of expectedCase.nodes) {
        const observationNode = captured.find(({ id }) => id === node.id);
        assert.equal(observationNode.input.style, node.style, `${fixtureCase.id}/${node.id} style`);
        if (node.expectedContainingBlock !== undefined) {
          assert.equal(observationNode.owner, node.expectedContainingBlock, `${fixtureCase.id}/${node.id}`);
        }
        if (node.expectedStaticPositionOwner !== undefined) {
          assert.equal(observationNode.staticPositionOwner, node.expectedStaticPositionOwner,
            `${fixtureCase.id}/${node.id}`);
        }
        if (node.expectedHasLayoutBox !== undefined) {
          assert.equal(observationNode.hasLayoutBox, node.expectedHasLayoutBox,
            `${fixtureCase.id}/${node.id}`);
        }
      }
      if (fixtureCase.flowInvariant) {
        const absId = captured.find(({ properties, hasLayoutBox }) =>
          properties.position === 'absolute' && hasLayoutBox).id;
        const expectedFlow = captured
          .filter(({ id, properties }) => id !== absId && properties.position !== 'absolute')
          .map(({ id, rect }) => ({ id, rect }));
        assert.deepEqual(fixtureCase.flowInvariant, expectedFlow, fixtureCase.id);
      }
    }
  }
  assert.deepEqual(reference.observations[0].cases, reference.observations[1].cases);
});

test('mobile runtime subset reuses pinned case styles and preserves their source tree', async () => {
  const [inventoryBytes, runtimeInventoryBytes, runtimeSourceBytes] = await Promise.all([
    load(inventoryPath), load(runtimeInventoryPath), load(runtimeSourcePath),
  ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const runtimeInventory = JSON.parse(runtimeInventoryBytes.toString('utf8'));
  assert.equal(runtimeInventory.schema,
    'spinon-css-c10-3-5-positioned-flex-runtime-inventory/v1');
  assert.equal(runtimeInventory.fixtureId, 'C10.3.5-positioned-flex-runtime-v1');
  assert.deepEqual(runtimeInventory.viewport, { width: 320, height: 240 });
  assert.equal(runtimeInventory.styleNormalization.from, 'background');
  assert.equal(runtimeInventory.styleNormalization.to, 'background-color');
  assert.deepEqual(runtimeInventory.cases.map(({ id }) => id), [
    'direct-row-center-end', 'paint-order', 'block-wrapper-static-position',
  ]);
  assert.deepEqual(runtimeInventory.cases.map(({ frameOffset }) => frameOffset), [
    { x: 0, y: 0 }, { x: 0, y: 100 }, { x: 0, y: 190 },
  ]);

  class FixtureElement {
    constructor(tagName) {
      this.tagName = tagName;
      this.attributes = new Map();
      this.children = [];
    }

    setAttribute(name, value) {
      this.attributes.set(name, value);
    }

    appendChild(child) {
      this.children.push(child);
      return child;
    }
  }
  const document = new FixtureElement('#document');
  const context = {
    document: {
      createElement: (tagName) => new FixtureElement(tagName),
      appendChild: (element) => document.appendChild(element),
    },
    __spinonC1035FixtureInventory: inventory,
    __spinonC1035RuntimeInventory: runtimeInventory,
  };
  context.globalThis = context;
  runInNewContext(runtimeSourceBytes.toString('utf8'), context);

  const runtimeRoot = document.children[0];
  assert.equal(runtimeRoot.attributes.get('id'), 'c1035-runtime-root');
  assert.equal(runtimeRoot.attributes.get('style'), runtimeInventory.rootStyle);
  const expectedIds = ['c1035-runtime-root'];
  for (const runtimeCase of runtimeInventory.cases) {
    const fixtureCase = inventory.cases.find(({ id }) => id === runtimeCase.id);
    const caseRoot = runtimeRoot.children.find((element) =>
      element.attributes.get('id') === fixtureCase.nodes[0].id);
    assert.ok(caseRoot, `${runtimeCase.id} root attached`);
    const normalizedStyle = (style) => style.replace(/(^|;)background:/g,
      '$1background-color:');
    const expectedStyle = normalizedStyle(fixtureCase.nodes[0].style)
      + (runtimeCase.rootStyleAppend ? `;${runtimeCase.rootStyleAppend}` : '');
    assert.equal(caseRoot.attributes.get('style'), expectedStyle, runtimeCase.id);
    const nodeElements = new Map();
    const visit = (element) => {
      const id = element.attributes.get('id');
      if (id) nodeElements.set(id, element);
      element.children.forEach(visit);
    };
    visit(caseRoot);
    assert.deepEqual([...nodeElements.keys()], fixtureCase.nodes.map(({ id }) => id));
    for (const node of fixtureCase.nodes) {
      assert.equal(nodeElements.get(node.id).attributes.get('style'), normalizedStyle(node.style)
        + (node === fixtureCase.nodes[0] && runtimeCase.rootStyleAppend
          ? `;${runtimeCase.rootStyleAppend}` : ''), `${runtimeCase.id}/${node.id}`);
      expectedIds.push(node.id);
    }
  }
  assert.deepEqual(Array.from(context.__spinonC1035RuntimeNodeIds), expectedIds);
});
