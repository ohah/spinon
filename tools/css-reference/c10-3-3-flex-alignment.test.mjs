import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c10/flex-alignment-inventory.json',
  html: 'tests/fixtures/css/c10/flex-alignment.html',
  runtime: 'tests/fixtures/css/c10/runtime-flex-alignment.js',
  reference: 'tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json',
  capture: 'tools/css-reference/capture-c10-3-3-flex-alignment.mjs',
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
const caseById = (caseId) => {
  const found = cases.get(caseId);
  assert.ok(found, `${caseId}가 Chromium 기준에 있어야 합니다.`);
  return found;
};
const node = (caseId, id) => {
  const found = caseById(caseId).nodes.find((entry) => entry.id === id);
  assert.ok(found, `${caseId}/${id}가 Chromium 기준에 있어야 합니다.`);
  return found;
};

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

test('고정 Chrome reference가 C10.3.3 입력과 capture digest를 보존한다', () => {
  assert.equal(reference.schema, 'spinon-css-c10-3-3-flex-alignment-reference/v1');
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
  assert.equal(inventory.wpt.paths.length, 168);
  assert.equal(inventory.wpt.paths.filter(({ disposition }) => disposition === 'excluded').length, 22);
  assert.equal(new Set(inventory.wpt.paths.map(({ path }) => path)).size, inventory.wpt.paths.length);
  const fixtureCaseIds = new Set(inventory.cases.map(({ id }) => id));
  for (const entry of inventory.wpt.paths) {
    assert.equal(typeof entry.path, 'string');
    assert.ok(entry.path.startsWith('css/css-align/') || entry.path.startsWith('css/css-flexbox/'));
    assert.ok(entry.reason.length > 0, `${entry.path}에 매핑 근거가 있어야 합니다.`);
    if (entry.disposition === 'excluded') {
      assert.deepEqual(entry.caseIds, []);
    } else {
      assert.ok(['mapped-subset', 'partial'].includes(entry.disposition));
      assert.ok(entry.caseIds.length > 0);
      assert.ok(entry.caseIds.every((id) => fixtureCaseIds.has(id)));
    }
  }
  assert.equal(inventory.cases.length, 50);
  assert.equal(inventory.cases.reduce((total, entry) => total + entry.nodes.length, 0), 134);
  assert.deepEqual(inventory.viewport, { width: 320, height: 240, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('두 DPR에서 모든 computed longhand, 원본 자식 순서와 CSS px frame이 같다', () => {
  assert.equal(observations.length, 2);
  assert.deepEqual(observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  assert.deepEqual(observations[0].cases, observations[1].cases);
  for (const observation of observations) {
    assert.deepEqual([observation.viewport.width, observation.viewport.height], [320, 240]);
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
          `${expectedCase.id}/${expectedNode.id}: DOM source child order`);
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

test('normal, self positions, overflow safety와 cross-axis auto margin을 구분한다', () => {
  assertComputed('align-normal', 'c1033-normal-root', 'align-items', 'normal');
  assertComputed('align-normal', 'c1033-normal-item', 'align-self', 'auto');
  assertRect('align-normal', 'c1033-normal-item', { x: 0, y: 0, width: 20, height: 10 });
  assert.deepEqual(
    ['c1033-pos-start', 'c1033-pos-self-start', 'c1033-pos-flex-start']
      .map((id) => node('self-positions', id).rect.y),
    [0, 0, 0],
  );
  assert.deepEqual(
    ['c1033-pos-end', 'c1033-pos-self-end', 'c1033-pos-flex-end', 'c1033-pos-center']
      .map((id) => node('self-positions', id).rect.y),
    [50, 50, 50, 25],
  );
  assert.deepEqual(
    ['c1033-safety-safe-center', 'c1033-safety-unsafe-center', 'c1033-safety-safe-end']
      .map((id) => node('self-safety-overflow', id).rect.y),
    [0, -10, 0],
  );
  assert.deepEqual(
    ['c1033-auto-margin-top', 'c1033-auto-margin-both']
      .map((id) => node('cross-auto-margin', id).rect.y),
    [80, 40],
  );
});

test('stretch eligibility, clamp, nowrap, wrap 단일 줄과 line distribution을 고정한다', () => {
  assertRect('stretch-minmax-padding-border', 'c1033-stretch-item', {
    x: 0, y: 0, width: 20, height: 50,
  });
  assertRect('content-nowrap', 'c1033-nowrap-item', { x: 0, y: 0, width: 20, height: 20 });
  assertRect('content-wrap-one-line', 'c1033-one-line-item', {
    x: 0, y: 40, width: 20, height: 20,
  });
  assert.deepEqual(
    ['c1033-between-a', 'c1033-between-b', 'c1033-between-c']
      .map((id) => [node('content-space-between-gap', id).rect.x,
        node('content-space-between-gap', id).rect.y]),
    [[0, 0], [70, 0], [0, 80]],
  );
  assert.deepEqual(
    ['c1033-around-a', 'c1033-around-b']
      .map((id) => node('content-space-around', id).rect.y),
    [15, 65],
  );
  assert.deepEqual(
    ['c1033-evenly-a', 'c1033-evenly-b']
      .map((id) => node('content-space-evenly', id).rect.y),
    [20, 60],
  );
  assert.deepEqual(
    ['c1033-chain-a', 'c1033-chain-b']
      .map((id) => node('content-stretch-chain', id).rect.height),
    [50, 50],
  );
});

test('wrap-reverse, safe fallback, place shorthand와 justify-content 축 fallback을 고정한다', () => {
  assert.deepEqual(
    ['c1033-reverse-flex-a', 'c1033-reverse-flex-b']
      .map((id) => node('wrap-reverse-flex-start', id).rect.y),
    [80, 60],
  );
  assert.deepEqual(
    ['c1033-reverse-start-a', 'c1033-reverse-start-b']
      .map((id) => node('wrap-reverse-start', id).rect.y),
    [20, 0],
  );
  assert.deepEqual(
    ['c1033-content-safe-a', 'c1033-content-safe-b']
      .map((id) => node('content-overflow-safe', id).rect.y),
    [0, 20],
  );
  assert.deepEqual(
    ['c1033-content-unsafe-a', 'c1033-content-unsafe-b']
      .map((id) => node('content-overflow-unsafe', id).rect.y),
    [-5, 15],
  );
  assertComputed('place-items', 'c1033-place-items-root', 'align-items', 'end');
  assertComputed('place-items', 'c1033-place-items-root', 'justify-items', 'center');
  assertRect('place-items', 'c1033-place-items-item', { x: 0, y: 50, width: 20, height: 10 });
  assertComputed('place-self', 'c1033-place-self-item', 'align-self', 'safe end');
  assertComputed('place-self', 'c1033-place-self-item', 'justify-self', 'center');
  assertRect('place-self', 'c1033-place-self-item', { x: 0, y: 50, width: 20, height: 10 });
  assertComputed('place-content-single', 'c1033-place-content-single-root', 'align-content', 'end');
  assertComputed('place-content-single', 'c1033-place-content-single-root', 'justify-content', 'end');
  assert.deepEqual(
    ['c1033-place-content-single-a', 'c1033-place-content-single-b']
      .map((id) => [node('place-content-single', id).rect.x,
        node('place-content-single', id).rect.y]),
    [[40, 60], [40, 80]],
  );
  assertComputed('place-content-two-values', 'c1033-place-content-two-root',
    'align-content', 'center');
  assertComputed('place-content-two-values', 'c1033-place-content-two-root',
    'justify-content', 'space-between');
  assert.deepEqual(
    ['c1033-justify-normal-item', 'c1033-justify-stretch-item',
      'c1033-justify-left-row-item', 'c1033-justify-right-row-item']
      .map((id, index) => node([
        'justify-normal', 'justify-stretch', 'justify-left-row', 'justify-right-row',
      ][index], id).rect.x),
    [0, 0, 0, 80],
  );
  assertRect('justify-space-between-single-item', 'c1033-justify-single-item', {
    x: 0, y: 0, width: 20, height: 10,
  });
  assertRect('justify-center-row', 'c1033-justify-center-item', {
    x: 40, y: 0, width: 20, height: 10,
  });
  assertRect('justify-center-overflow', 'c1033-justify-center-overflow-item', {
    x: -15, y: 0, width: 130, height: 10,
  });
  assert.deepEqual(
    ['c1033-justify-between-negative-a', 'c1033-justify-between-negative-b']
      .map((id) => node('justify-space-between-negative', id).rect.x),
    [0, 70],
  );
  assert.deepEqual(
    [
      ['justify-left-row-reverse', 'c1033-justify-left-reverse-item'],
      ['justify-right-row-reverse', 'c1033-justify-right-reverse-item'],
    ].map(([caseId, id]) => node(caseId, id).rect.x),
    [0, 80],
  );
  assert.deepEqual(
    ['justify-left-column', 'justify-right-column']
      .map((caseId) => node(caseId, caseById(caseId).nodes[1].id).rect.y),
    [0, 0],
  );
  assert.deepEqual(
    ['justify-overflow-safe-right', 'justify-overflow-unsafe-right']
      .map((caseId) => node(caseId, caseById(caseId).nodes[1].id).rect.x),
    [0, -30],
  );
});

test('잘못된 선언은 앞선 유효 값을 유지하고 custom property와 Block 경계를 지킨다', () => {
  assertComputed('invalid-declaration-fallback', 'c1033-invalid-root', 'align-items', 'center');
  assertComputed('invalid-declaration-fallback', 'c1033-invalid-root', 'align-content', 'flex-end');
  assertComputed('invalid-declaration-fallback', 'c1033-invalid-root', 'justify-content', 'center');
  assertComputed('custom-property-values', 'c1033-custom-root', 'align-items', 'safe flex-end');
  assertComputed('custom-property-values', 'c1033-custom-root', 'align-content', 'space-evenly');
  assertComputed('custom-property-values', 'c1033-custom-a', 'align-self', 'safe center');
  assert.deepEqual(
    ['c1033-block-a', 'c1033-block-b']
      .map((id) => node('non-flex-boundary', id).rect.y),
    [20, 30],
  );
});
