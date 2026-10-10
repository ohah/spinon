import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c09/block-formatting-inventory.json';
const htmlPath = 'tests/fixtures/css/c09/block-formatting.html';
const referencePath = 'tests/fixtures/css/references/c09-block-formatting-v1.json';
const capturePath = 'tools/css-reference/capture-c09-block-formatting.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const read = async (path) => readFile(join(repositoryRoot, path));
const [inventoryBytes, htmlBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
  read(inventoryPath), read(htmlPath), read(capturePath), read(helperPath), read(referencePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const observations = reference.observations;
const cases = inventory.cases;
const flatten = (tree, parentId = null, result = []) => {
  result.push({ id: tree.id, parentId });
  for (const child of tree.children ?? []) flatten(child, tree.id, result);
  return result;
};
const caseMap = (observation) => new Map(observation.cases.map((entry) => [entry.id, entry]));
const nodeMap = (entry) => new Map(entry.nodes.map((node) => [node.id, node]));
const baseCases = caseMap(observations[0]);
const scaledCases = caseMap(observations[1]);

test('C09 reference는 Chrome 154와 fixture·inventory·capture 입력 digest를 고정한다', () => {
  assert.equal(reference.schema, 'spinon-css-c09-block-formatting-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, hash(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, hash(htmlBytes));
  assert.equal(reference.captureTool.sha256, hash(captureBytes));
  assert.equal(reference.captureTool.dependencies[0].sha256, hash(helperBytes));
  assert.match(reference.oracle.executableSha256, /^[a-f0-9]{64}$/);
  assert.equal(inventory.cases.length, 23);
  assert.equal(cases.flatMap(({ tree }) => flatten(tree)).length, 76);
  assert.deepEqual(reference.environment.viewportCssPx, { width: 320, height: 240 });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
});

test('C09 inventory와 Chromium DOM preorder·부모 관계가 모두 일치한다', () => {
  assert.equal(observations.length, 2);
  assert.equal(observations[0].viewport.deviceScaleFactor, 1);
  assert.equal(observations[1].viewport.deviceScaleFactor, 2);
  for (const fixtureCase of cases) {
    const expectedNodes = flatten(fixtureCase.tree);
    for (const observation of observations) {
      const actualCase = caseMap(observation).get(fixtureCase.id);
      assert.ok(actualCase, fixtureCase.id);
      assert.equal(actualCase.harnessId, fixtureCase.harnessId);
      assert.deepEqual(actualCase.nodes.map(({ id, parentId }) => ({ id, parentId })), expectedNodes);
      assert.deepEqual(actualCase.harness.rect, { x: 0, y: 0, width: 320, height: 240 });
      assert.equal(actualCase.harness.display, 'flow-root');
      assert.equal(actualCase.harness.direction, 'ltr');
      assert.equal(actualCase.harness.writingMode, 'horizontal-tb');
      assert.equal(actualCase.harness.position, 'absolute');
      assert.ok(actualCase.nodes.every((node) => inventory.comparison.computedProperties.every(
        (property) => typeof node.properties[property] === 'string',
      )));
      assert.ok(actualCase.nodes.every((node) => node.properties.direction === 'ltr'
        && node.properties['writing-mode'] === 'horizontal-tb'
        && node.properties.position === 'static'
        && node.properties.float === 'none'
        && node.properties.clear === 'none'
        && node.properties.transform === 'none'
        && ['block', 'flow-root', 'none'].includes(node.properties.display)));
      assert.ok(actualCase.nodes.every((node) => inventory.comparison.typedProperties.every(
        (property) => node.typed[property] === null || typeof node.typed[property].text === 'string',
      )));
    }
  }
  assert.equal(baseCases.size, cases.length);
  assert.equal(scaledCases.size, cases.length);
});

test('fixture harness가 화면 원점에 있어 숨김 node의 영점 사각형도 안정적이다', () => {
  const hidden = nodeMap(baseCases.get('c091-hidden-subtree'));
  for (const id of ['c091-hidden-parent', 'c091-hidden-child']) {
    assert.deepEqual(hidden.get(id).rect, { x: 0, y: 0, width: 0, height: 0 }, id);
  }
  assert.deepEqual(hidden.get('c091-hidden-visible-sibling').rect,
    { x: 0, y: 0, width: 320, height: 10 });
});

test('Block auto width·horizontal auto margin·overconstraint·negative margin geometry를 고정한다', () => {
  const autoWidth = nodeMap(baseCases.get('c091-auto-width'));
  assert.deepEqual(autoWidth.get('c091-auto-width-parent').rect,
    { x: 7, y: 5, width: 287, height: 20 });
  assert.deepEqual(autoWidth.get('c091-auto-width-child').rect,
    { x: 28, y: 10, width: 240, height: 6 });
  assert.equal(autoWidth.get('c091-auto-width-child').properties.width, '225px');

  const minWidth = nodeMap(baseCases.get('c091-auto-width-min-width'));
  assert.equal(minWidth.get('c091-min-width-child').rect.x, 10);
  assert.equal(minWidth.get('c091-min-width-child').rect.width, 180);
  assert.equal(minWidth.get('c091-min-width-child').properties['min-width'], '180px');

  const percentage = nodeMap(baseCases.get('c091-percentage-containing-block'));
  assert.equal(percentage.get('c091-percentage-child').properties.width, '100px');
  assert.equal(percentage.get('c091-percentage-child').rect.width, 100);
  assert.equal(percentage.get('c091-percentage-child').properties['box-sizing'], 'border-box');

  const centered = nodeMap(baseCases.get('c091-centered-auto-margins'));
  assert.equal(centered.get('c091-centered-child').rect.x, 40);
  assert.equal(centered.get('c091-centered-child').rect.width, 120);

  const verticalAuto = nodeMap(baseCases.get('c091-vertical-auto-margins'));
  assert.equal(verticalAuto.get('c091-vertical-auto-child').rect.y, 0);
  assert.equal(verticalAuto.get('c091-vertical-auto-child').properties['margin-top'], '0px');
  assert.equal(verticalAuto.get('c091-vertical-auto-child').properties['margin-bottom'], '0px');

  const overconstrained = nodeMap(baseCases.get('c091-overconstrained-ltr'));
  assert.equal(overconstrained.get('c091-overconstrained-child').rect.x, 10);
  assert.equal(overconstrained.get('c091-overconstrained-child').rect.width, 240);
  assert.equal(overconstrained.get('c091-overconstrained-child').properties['margin-right'], '20px');

  const negative = nodeMap(baseCases.get('c091-negative-margin-auto-width'));
  assert.equal(negative.get('c091-negative-auto-width-child').rect.x, -10);
  assert.equal(negative.get('c091-negative-auto-width-child').rect.width, 190);
  const order = nodeMap(baseCases.get('c091-auto-height-order'));
  assert.equal(order.get('c091-auto-height-second').rect.y, 10);
  assert.equal(order.get('c091-auto-height-root').rect.height, 30);
});

test('양수·음수 margin strut과 parent·child·padding·used border 경계를 고정한다', () => {
  const positive = nodeMap(baseCases.get('c092-positive-siblings'));
  assert.equal(positive.get('c092-positive-second').rect.y, 40);

  const signed = nodeMap(baseCases.get('c092-signed-self-collapse'));
  assert.equal(signed.get('c092-signed-empty').rect.height, 0);
  assert.equal(signed.get('c092-signed-last').rect.y, 28);

  const fractional = nodeMap(baseCases.get('c092-fractional-signed-collapse'));
  assert.equal(fractional.get('c092-fractional-last').rect.y, 28.25);
  assert.equal(fractional.get('c092-fractional-root').rect.height, 38.25);

  const negative = nodeMap(baseCases.get('c092-negative-siblings'));
  assert.equal(negative.get('c092-negative-second').rect.y, -20);
  assert.equal(negative.get('c092-negative-sibling-root').rect.height, 0);

  const parentFirst = nodeMap(baseCases.get('c092-parent-first-child'));
  assert.equal(parentFirst.get('c092-parent-first').rect.y, 20);
  assert.equal(parentFirst.get('c092-parent-first-child').rect.y, 20);

  const parentLast = nodeMap(baseCases.get('c092-parent-last-child'));
  assert.equal(parentLast.get('c092-parent-last-after').rect.y, 30);

  const padding = nodeMap(baseCases.get('c092-padding-barrier'));
  assert.equal(padding.get('c092-padding-child').rect.y, 21);

  const noBorder = nodeMap(baseCases.get('c092-border-none'));
  assert.equal(noBorder.get('c092-border-none-parent').properties['border-top-width'], '0px');
  assert.equal(noBorder.get('c092-border-none-child').rect.y, 20);

  const border = nodeMap(baseCases.get('c092-border-solid'));
  assert.equal(border.get('c092-border-solid-parent').properties['border-top-width'], '4px');
  assert.equal(border.get('c092-border-solid-child').rect.y, 24);
});

test('flow-root 내부 margin 차단과 외부 parent·sibling margin을 구분한다', () => {
  const internal = nodeMap(baseCases.get('c093-flow-root-internal'));
  assert.equal(internal.get('c093-flow-root-internal-child').rect.y
    - internal.get('c093-flow-root-internal').rect.y, 20);
  assert.equal(internal.get('c093-flow-root-internal').rect.height, 60);

  const parentMargin = nodeMap(baseCases.get('c093-flow-root-parent-margin'));
  assert.equal(parentMargin.get('c093-flow-root-parent').rect.y, 24);
  assert.equal(parentMargin.get('c093-flow-root-parent-child').rect.y, 24);
  assert.equal(parentMargin.get('c093-flow-root-parent-grandchild').rect.y, 24);

  const parentLast = nodeMap(baseCases.get('c093-flow-root-parent-last-child'));
  assert.equal(parentLast.get('c093-flow-root-parent-last-child').rect.y, 0);
  assert.equal(parentLast.get('c093-flow-root-parent-last-child').rect.height, 10);
  assert.equal(parentLast.get('c093-flow-root-parent-last-after').rect.y, 34);

  const siblingMargin = nodeMap(baseCases.get('c093-flow-root-sibling-margin'));
  assert.equal(siblingMargin.get('c093-flow-root-sibling').rect.y, 40);
  assert.equal(siblingMargin.get('c093-flow-root-sibling-after').rect.y, 80);
  assert.equal(siblingMargin.get('c093-flow-root-sibling-root').rect.height, 90);
});

test('CSS px computed values와 used geometry는 DPR 1·2에서 변하지 않는다', () => {
  assert.deepEqual(observations[0].cases, observations[1].cases);
  assert.deepEqual(reference.comparison.rectFields, ['x', 'y', 'width', 'height']);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  for (const fixtureCase of cases) {
    const actualCase = baseCases.get(fixtureCase.id);
    for (const node of actualCase.nodes) {
      assert.ok(['x', 'y', 'width', 'height'].every((field) => Number.isFinite(node.rect[field])));
    }
  }
});
