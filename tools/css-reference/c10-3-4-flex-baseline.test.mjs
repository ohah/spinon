import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c10/flex-baseline-inventory.json',
  html: 'tests/fixtures/css/c10/flex-baseline.html',
  runtime: 'tests/fixtures/css/c10/runtime-flex-baseline.js',
  reference: 'tests/fixtures/css/references/c10-3-4-flex-baseline-v1.json',
  capture: 'tools/css-reference/capture-c10-3-4-flex-baseline.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
};
const read = async (path) => readFile(join(repositoryRoot, path));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const files = Object.fromEntries(await Promise.all(Object.entries(paths).map(
  async ([name, path]) => [name, await read(path)],
)));
const inventory = JSON.parse(files.inventory.toString('utf8'));
const reference = JSON.parse(files.reference.toString('utf8'));
const observations = reference.observations;
const cases = new Map(observations[0].cases.map((entry) => [entry.id, entry]));

function caseById(caseId) {
  const found = cases.get(caseId);
  assert.ok(found, `${caseId}가 Chromium reference에 있어야 합니다.`);
  return found;
}

function node(caseId, id) {
  const found = caseById(caseId).nodes.find((entry) => entry.id === id);
  assert.ok(found, `${caseId}/${id}가 Chromium reference에 있어야 합니다.`);
  return found;
}

function assertRect(caseId, id, expected) {
  const actual = node(caseId, id).rect;
  for (const field of ['x', 'y', 'width', 'height']) {
    assert.ok(Math.abs(actual[field] - expected[field]) <= 0.001,
      `${caseId}/${id}.${field}: expected ${expected[field]}, got ${actual[field]}`);
  }
}

function assertComputed(caseId, id, property, expected) {
  assert.equal(node(caseId, id).properties[property], expected,
    `${caseId}/${id}.${property}`);
}

function borderBottom(caseId, id) {
  const { y, height } = node(caseId, id).rect;
  return y + height;
}

function marginBottom(style) {
  const declaration = style.match(/(?:^|;)margin:([^;]+)/)?.[1];
  assert.ok(declaration, `margin shorthand가 있어야 합니다: ${style}`);
  const values = declaration.split(/\s+/);
  const value = values.length === 1 ? values[0]
    : values.length === 2 ? values[0]
      : values.length === 3 ? values[2] : values[2];
  return Number.parseFloat(value);
}

test('고정 Chrome reference가 C10.3.4 입력·capture digest와 WPT 경계를 보존한다', () => {
  assert.equal(reference.schema, 'spinon-css-c10-3-4-flex-baseline-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, hash(files.inventory));
  assert.equal(reference.fixture.htmlSha256, hash(files.html));
  assert.equal(reference.fixture.runtimeSourcePath, paths.runtime);
  assert.equal(reference.fixture.runtimeSourceSha256, hash(files.runtime));
  assert.equal(reference.captureTool.sha256, hash(files.capture));
  assert.equal(reference.captureTool.dependencies[0].sha256, hash(files.helper));
  assert.match(reference.oracle.executableSha256, /^[a-f0-9]{64}$/);
  assert.deepEqual(reference.wpt, inventory.wpt);
  assert.equal(inventory.wpt.revision, 'd5a765f1089ce6d3f72300281481edf3dddff7f3');
  assert.equal(inventory.wpt.execution, 'not-run');
  assert.ok(inventory.wpt.paths.some(({ disposition, caseIds }) =>
    disposition === 'mapped-subset' && caseIds.includes('nested-first-wrap')));
  assert.ok(inventory.wpt.paths.every(({ path, reason }) =>
    path.startsWith('css/css-flexbox/') && reason.length > 0));
  assert.equal(inventory.cases.length, 18);
  assert.equal(inventory.cases.reduce((total, entry) => total + entry.nodes.length, 0), 73);
  assert.deepEqual(inventory.viewport, { width: 320, height: 640, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(reference.comparison.maxCssPxDelta, 0.5);
});

test('두 DPR에서 DOM 순서, computed value와 CSS px geometry가 동일하다', () => {
  assert.equal(observations.length, 2);
  assert.deepEqual(observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  assert.deepEqual(observations[0].cases, observations[1].cases);
  for (const observation of observations) {
    assert.deepEqual([observation.viewport.width, observation.viewport.height], [320, 640]);
    assert.equal(observation.environment.locale, 'en-US');
    assert.equal(observation.environment.timeZone, 'UTC');
    assert.equal(observation.cases.length, inventory.cases.length);
    for (const [caseIndex, expectedCase] of inventory.cases.entries()) {
      const actualCase = observation.cases[caseIndex];
      assert.equal(actualCase.id, expectedCase.id);
      assert.deepEqual(actualCase.nodes.map(({ id }) => id), expectedCase.nodes.map(({ id }) => id));
      for (const [nodeIndex, expectedNode] of expectedCase.nodes.entries()) {
        const actualNode = actualCase.nodes[nodeIndex];
        assert.deepEqual(actualNode.children, expectedNode.children,
          `${expectedCase.id}/${expectedNode.id}: 원본 DOM 자식 순서`);
        assert.ok(inventory.comparison.computedProperties.every(
          (property) => typeof actualNode.properties[property] === 'string',
        ));
        assert.ok(inventory.comparison.rectFields.every(
          (field) => Number.isFinite(actualNode.rect[field]),
        ));
      }
    }
  }
});

test('first·last sharing group은 각각 독립되고 단일 참여자는 반대쪽 fallback을 쓴다', () => {
  assertComputed('first-group', 'c1034-first-root', 'align-items', 'baseline');
  assert.deepEqual(
    ['c1034-first-a', 'c1034-first-b', 'c1034-first-c']
      .map((id) => borderBottom('first-group', id)),
    [40, 40, 40],
  );
  assertComputed('first-explicit-and-place-self', 'c1034-explicit-first',
    'align-self', 'baseline');
  assertComputed('first-explicit-and-place-self', 'c1034-explicit-place',
    'place-self', 'baseline');
  assert.deepEqual(
    ['c1034-explicit-first', 'c1034-explicit-place']
      .map((id) => borderBottom('first-explicit-and-place-self', id)),
    [40, 40],
  );
  assertComputed('last-group', 'c1034-last-root', 'align-items', 'last baseline');
  assert.deepEqual(
    ['c1034-last-a', 'c1034-last-b', 'c1034-last-c']
      .map((id) => borderBottom('last-group', id)),
    [100, 100, 100],
  );
  assert.deepEqual(
    ['c1034-mixed-first-a', 'c1034-mixed-first-b']
      .map((id) => borderBottom('mixed-groups', id)),
    [40, 40],
  );
  assert.deepEqual(
    ['c1034-mixed-last-a', 'c1034-mixed-last-b']
      .map((id) => borderBottom('mixed-groups', id)),
    [100, 100],
  );
  assert.equal(node('single-first-participant', 'c1034-single-first-participant').rect.y, 0);
  assert.equal(borderBottom('single-last-participant', 'c1034-single-last-participant'), 100);
});

test('last baseline은 cross margin을 보존하고 합성 기준은 border edge다', () => {
  assert.deepEqual(
    ['c1034-margin-a', 'c1034-margin-b']
      .map((id) => borderBottom('last-cross-margins', id)),
    [93, 93],
  );
  assert.deepEqual(
    ['c1034-margin-a', 'c1034-margin-b']
      .map((id) => node('last-cross-margins', id).rect.y
        + node('last-cross-margins', id).rect.height
        + marginBottom(node('last-cross-margins', id).input.style)),
    [100, 96],
  );
  assert.deepEqual(
    ['c1034-border-a', 'c1034-border-b']
      .map((id) => borderBottom('border-padding-synthesized', id)),
    [40, 40],
  );
  assert.match(node('border-padding-synthesized', 'c1034-border-a').input.style,
    /padding:0 0 4px;border:0;border-bottom:3px solid/);
});

test('column·content alignment과 nested Flex baseline set은 서로 다른 경계다', () => {
  assertComputed('column-fallback', 'c1034-column-root', 'align-items', 'baseline');
  assert.deepEqual(
    ['c1034-column-a', 'c1034-column-b']
      .map((id) => node('column-fallback', id).rect.x),
    [0, 0],
  );
  assertComputed('align-content-first-baseline', 'c1034-content-first-root',
    'align-content', 'baseline');
  assert.deepEqual(
    ['c1034-content-first-a', 'c1034-content-first-b']
      .map((id) => node('align-content-first-baseline', id).rect.y),
    [0, 20],
  );
  assertComputed('align-content-last-invalid-fallback', 'c1034-content-last-root',
    'align-content', 'center');
  assert.deepEqual(
    ['c1034-content-last-a', 'c1034-content-last-b']
      .map((id) => node('align-content-last-invalid-fallback', id).rect.y),
    [25, 45],
  );
  assert.equal(
    node('nested-first-wrap', 'c1034-nested-first-flex').rect.y
      + borderBottom('nested-first-wrap', 'c1034-nested-first-a')
      - node('nested-first-wrap', 'c1034-nested-first-flex').rect.y,
    25,
  );
  assert.equal(
    node('nested-last-wrap', 'c1034-nested-last-flex').rect.y
      + (borderBottom('nested-last-wrap', 'c1034-nested-last-b')
        - node('nested-last-wrap', 'c1034-nested-last-flex').rect.y),
    100,
  );
  assert.equal(node('nested-first-wrap-reverse-order', 'c1034-reverse-a').rect.y, 5);
  assert.equal(node('nested-first-wrap-reverse-order', 'c1034-reverse-b').rect.y, 25);
  assert.equal(node('nested-first-wrap-reverse-order', 'c1034-reverse-a').input.style.match(/order:([^;]+)/)?.[1], '1');
  assert.equal(node('nested-first-wrap-reverse-order', 'c1034-reverse-b').input.style.match(/order:([^;]+)/)?.[1], '-1');
});
