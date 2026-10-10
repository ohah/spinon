import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c06/spacing-percentages-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c06/spacing-percentages.html';
const captureRelativePath = 'tools/css-reference/capture-c06-spacing-percentages.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const referenceRelativePath = 'tests/fixtures/css/references/c06-spacing-percentages-v1.json';
const readJson = async (relativePath) => JSON.parse(await readFile(join(repositoryRoot, relativePath), 'utf8'));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [inventory, reference] = await Promise.all([
  readJson(inventoryRelativePath),
  readJson(referenceRelativePath),
]);
const observations = new Map(reference.observation.nodes.map((item) => [item.id, item]));
const get = (id) => {
  const item = observations.get(id);
  assert.ok(item, `Chromium reference에 ${id} 관찰이 있어야 합니다`);
  return item;
};
const assertClose = (actual, expected) => assert.ok(Math.abs(actual - expected) <= 0.001, `${actual} != ${expected}`);

test('C06.2 Chromium reference는 고정 버전·입력 hash·inventory를 보존한다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c06-spacing-percentages-inventory/v1');
  assert.equal(inventory.fixtureId, 'C06.2-spacing-percentages-v1');
  assert.equal(reference.schema, 'spinon-css-c06-spacing-percentages-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.observation.nodes.length, 78);
  assert.deepEqual(reference.observation.nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));
  assert.equal(new Set(inventory.nodes.map(({ id }) => id)).size, inventory.nodes.length);

  for (const [relativePath, expectedHash] of [
    [inventoryRelativePath, reference.fixture.inventorySha256],
    [htmlRelativePath, reference.fixture.htmlSha256],
    [captureRelativePath, reference.captureTool.sha256],
    [helperRelativePath, reference.captureTool.dependencies[0].sha256],
  ]) {
    assert.equal(sha256(await readFile(join(repositoryRoot, relativePath))), expectedHash, `${relativePath} hash`);
  }
});

test('margin·padding 백분율은 containing block content width를 사용한다', () => {
  for (const edge of ['top', 'right', 'bottom', 'left']) {
    assert.equal(get('m-physical').properties[`margin-${edge}`], '20px');
    assert.equal(get('p-physical').properties[`padding-${edge}`], '20px');
  }
  assert.equal(get('m-negative').properties['margin-top'], '-10px');
  assert.equal(get('m-zero').properties['margin-top'], '0px');
  assert.equal(get('m-fraction').properties['margin-top'], '66.6562px');
  assert.equal(get('m-over100').properties['margin-top'], '250px');
  assert.deepEqual(
    ['top', 'right', 'bottom', 'left'].map((edge) => get('m-four').properties[`margin-${edge}`]),
    ['2px', '4px', '6px', '8px'],
  );
  assert.deepEqual(
    ['top', 'right', 'bottom', 'left'].map((edge) => get('m-two').properties[`margin-${edge}`]),
    ['20px', '10px', '20px', '10px'],
  );
  assert.deepEqual(
    ['top', 'right', 'bottom', 'left'].map((edge) => get('m-three').properties[`margin-${edge}`]),
    ['20px', '10px', '4px', '10px'],
  );
  assert.equal(get('m-override').properties['margin-left'], '8px');
  assert.equal(get('m-logical-ltr').properties['margin-left'], '30px');
  assert.equal(get('m-logical-ltr').properties['margin-right'], '50px');
  assert.equal(get('m-logical-rtl').properties['margin-left'], '50px');
  assert.equal(get('m-logical-rtl').properties['margin-right'], '30px');

  assert.equal(get('box-cb').rect.width, 260);
  assert.equal(get('m-box-basis').properties['margin-left'], '20px');
  assert.equal(get('p-box-basis').properties['padding-left'], '20px');
  assert.equal(get('p-box-basis').rect.width, 60);
  assert.equal(get('p-box-basis').rect.height, 50);
});

test('Flex gap은 대응 content-box 차원·cascade·invalid value를 보존한다', () => {
  assertClose(get('g-row-b').rect.x, 70);
  assertClose(get('g-column-b').rect.y, 1280);
  assert.equal(get('g-shorthand').properties['row-gap'], '25%');
  assert.equal(get('g-shorthand').properties['column-gap'], '5%');
  assertClose(get('g-shorthand-b').rect.x, 60);
  assertClose(get('g-one-value-b').rect.x, 70);
  assertClose(get('g-zero-b').rect.x, 50);
  assertClose(get('g-over100-b').rect.x, 350);
  assert.equal(get('g-negative').properties['column-gap'], '8px');
  assertClose(get('g-negative-b').rect.x, 58);
  assert.equal(get('g-normal').properties['column-gap'], 'normal');
  assertClose(get('g-normal-b').rect.x, 50);
  assert.equal(get('g-var').properties['column-gap'], '12.5%');
  assertClose(get('g-var-b').rect.x, 75);
  assert.equal(get('g-var-negative').properties['column-gap'], 'normal');
  assertClose(get('g-var-negative-b').rect.x, 50);
});

test('computed-value invalidation과 cyclic·inline-flex oracle 경계가 서로 구분된다', () => {
  assert.equal(get('p-invalid-negative').properties['padding-left'], '8px');
  assert.equal(get('p-var-negative').properties['padding-left'], '0px');
  assert.equal(get('g-auto-column').rect.height, 20);
  assert.equal(get('g-auto-column').properties['row-gap'], '10%');
  assert.equal(get('g-auto-column-b').rect.y - get('g-auto-column-a').rect.y, 10);
  assert.equal(get('g-auto-width-row').properties.width, '320px');
  assert.equal(get('g-auto-width-row-b').rect.x - get('g-auto-width-row-a').rect.x, 52);
  assert.equal(get('g-stretched-column').properties.height, '120px');
  assert.equal(get('g-stretched-column-b').rect.y - get('g-stretched-column-a').rect.y, 22);
  assert.equal(get('g-auto-row-inline').rect.width, 40);
  assert.equal(get('g-auto-row-inline-b').rect.x - get('g-auto-row-inline-a').rect.x, 24);
  assert.equal(reference.comparison.scope.includes('inline-flex is an oracle-only boundary probe'), true);
});
