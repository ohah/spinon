import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  reference: 'tests/fixtures/css/references/c12-1-position-static-relative-v1.json',
  inventory: 'tests/fixtures/css/c12/position-static-relative-inventory.json',
  html: 'tests/fixtures/css/c12/position-static-relative.html',
  runtimeFixture: 'tests/fixtures/css/c12/runtime-position-static-relative.js',
  capture: 'tools/css-reference/capture-c12-1-static-relative.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  androidRuntimeLog: 'spec/internal/evidence/c12-1-static-relative/android-physical.log',
  iosRuntimeLog: 'spec/internal/evidence/c12-1-static-relative/ios-simulator.log',
};
const read = async (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [reference, inventory] = await Promise.all([
  read(paths.reference).then((bytes) => JSON.parse(bytes.toString('utf8'))),
  read(paths.inventory).then((bytes) => JSON.parse(bytes.toString('utf8'))),
]);
const byState = (observation) => new Map(observation.states.map((state) => [state.state, state]));
const nodeMap = (state) => new Map(state.nodes.map((node) => [node.id, node]));

test('C12.1 기준은 fixture·캡처 도구·Chromium·WPT revision에 고정된다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c12-1-position-inventory/v1');
  assert.equal(inventory.fixtureId, 'C12.1-static-relative-v1');
  assert.equal(reference.schema, 'spinon-css-c12-1-position-reference/v1');
  assert.equal(reference.referenceId, 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-1');
  assert.equal(reference.chromium.version, '154.0.8037.98');
  assert.equal(reference.chromium.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.match(reference.chromium.binarySha256, /^[0-9a-f]{64}$/);
  assert.deepEqual(reference.cssPosition, {
    url: 'https://www.w3.org/TR/2025/WD-css-position-3-20251007/',
    versionDate: '2025-10-07',
    status: 'W3C Working Draft',
  });
  assert.equal(reference.wpt.execution, 'not-run');
  assert.equal(reference.wpt.revision, '9ec154ff43db468923997c08bb08f905ceab62a5');
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.equal(inventory.comparison.ownerMismatchIsFailure, true);
  assert.deepEqual(reference.observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  assert.equal(reference.observations.length, 2);

  for (const [path, digest] of [
    [paths.inventory, reference.fixture.inventorySha256],
    [paths.html, reference.fixture.htmlSha256],
    [paths.runtimeFixture, reference.fixture.runtimeSourceSha256],
    [paths.capture, reference.captureTool.sha256],
    [paths.helper, reference.captureTool.dependencies[0].sha256],
  ]) {
    assert.equal(sha256(await read(path)), digest, path + ' hash');
  }
});

test('각 관찰 상태에 inventory의 node와 source-order 관계가 정확히 보존된다', () => {
  for (const observation of reference.observations) {
    for (const state of observation.states) {
      assert.deepEqual(state.nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));
      const actual = nodeMap(state);
      for (const expected of inventory.nodes) {
        const node = actual.get(expected.id);
        if (expected.id === 'c12-root') {
          assert.equal(expected.parentId, null);
          assert.equal(expected.caseId, 'root');
        } else {
          const fixtureCase = inventory.cases.find(({ id }) => id === expected.caseId);
          assert.ok(fixtureCase?.nodeIds.includes(expected.id), expected.id + ' case membership');
          assert.ok(inventory.nodes.some(({ id }) => id === expected.parentId), expected.id + ' parent exists');
        }
        assert.equal(inventory.cases.filter(({ nodeIds: ids }) => ids.includes(expected.id)).length,
          expected.id === 'c12-root' ? 0 : 1, expected.id + ' unique case membership');
        assert.equal(node.parentId, expected.parentId, expected.id + ' parent');
        assert.equal(node.owner, state.state === 'ancestor-relative'
          ? (expected.ancestorMutationOwner ?? expected.expectedOwner)
          : expected.expectedOwner, expected.id + ' owner');
        assert.deepEqual(node.children, inventory.nodes
          .filter((candidate) => candidate.parentId === expected.id)
          .map(({ id }) => id), expected.id + ' children');
        assert.equal(node.hasLayoutBox, expected.hasLayoutBox !== false, expected.id + ' box');
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), expected.id + ' rect.' + field);
          assert.ok(Number.isFinite(node.flowRect[field]), expected.id + ' flowRect.' + field);
        }
      }
    }
  }
});

test('static은 inset을 무시하고 relative는 흐름 frame과 뒤 형제를 유지한다', () => {
  const states = byState(reference.observations[0]);
  const initial = nodeMap(states.get('initial'));
  assert.deepEqual(initial.get('static-target').rect, initial.get('static-target').flowRect);
  assert.equal(initial.get('static-target').properties.left, '12px');
  assert.equal(initial.get('static-target').properties.top, '6px');
  assert.equal(initial.get('static-target').rect.x, 0);
  assert.equal(initial.get('static-target').rect.y, 0);
  assert.deepEqual(initial.get('relative-target').flowRect, { x: 0, y: 40, width: 40, height: 12 });
  assert.deepEqual(initial.get('relative-target').rect, { x: 12, y: 37, width: 40, height: 12 });
  assert.deepEqual(initial.get('relative-after').rect, initial.get('relative-after').flowRect);
  assert.equal(initial.get('relative-after').rect.y, 52);
  assert.deepEqual(initial.get('flex-sibling').rect, initial.get('flex-sibling').flowRect);
});

test('반대 방향·음수·auto·초과 제약 inset이 LTR 물리 축 규칙과 일치한다', () => {
  const nodes = nodeMap(reference.observations[0].states[0]);
  assert.equal(nodes.get('right-only').rect.x, -8);
  assert.equal(nodes.get('bottom-only').rect.y, 76);
  assert.equal(nodes.get('auto-insets').rect.x, nodes.get('auto-insets').flowRect.x);
  assert.equal(nodes.get('overconstrained-target').rect.x, 5);
  assert.equal(nodes.get('overconstrained-target').rect.y, 123);
  assert.equal(nodes.get('overconstrained-after').rect.y, nodes.get('overconstrained-after').flowRect.y);
});

test('percentage와 calc inset이 containing block 축 및 custom property fallback을 따른다', () => {
  const nodes = nodeMap(reference.observations[0].states[0]);
  assert.equal(nodes.get('percent-container').rect.width, 220);
  assert.equal(nodes.get('percent-container').rect.height, 60);
  assert.equal(nodes.get('percent-target').properties.left, '20px');
  assert.equal(nodes.get('percent-target').properties.top, '10px');
  assert.deepEqual(nodes.get('percent-target').rect, { x: 30, y: 180, width: 20, height: 10 });
  assert.equal(nodes.get('calc-target').properties.left, '24px');
  assert.equal(nodes.get('calc-target').properties.top, '-5px');
  assert.deepEqual(nodes.get('calc-target').rect, { x: 34, y: 237, width: 20, height: 10 });
});

test('inset shorthand·longhand·cascade layer·important의 계산값이 보존된다', () => {
  const nodes = nodeMap(reference.observations[0].states[0]);
  assert.deepEqual(
    ['inset-one', 'inset-two', 'inset-three', 'inset-four'].map((id) => [
      nodes.get(id).properties.top,
      nodes.get(id).properties.right,
      nodes.get(id).properties.bottom,
      nodes.get(id).properties.left,
    ]),
    [
      ['6px', '6px', '6px', '6px'],
      ['7px', '8px', '7px', '8px'],
      ['9px', '10px', '11px', '10px'],
      ['1px', '2px', '3px', '4px'],
    ],
  );
  assert.deepEqual(
    ['layered-target', 'important-target'].map((id) => [
      nodes.get(id).properties.top,
      nodes.get(id).properties.right,
      nodes.get(id).properties.bottom,
      nodes.get(id).properties.left,
    ]),
    [['6px', '9px', '3px', '11px'], ['3px', '3px', '3px', '13px']],
  );
});

test('중첩 relative box가 가장 가까운 positioned owner를 보고하고 흐름 형제를 유지한다', () => {
  const nodes = nodeMap(reference.observations[0].states[0]);
  assert.equal(nodes.get('nested-child').owner, 'nested-parent');
  assert.equal(nodes.get('nested-grandchild').owner, 'nested-child');
  assert.equal(nodes.get('nested-parent-after').owner, 'nested-parent');
  assert.equal(nodes.get('nested-outside-after').owner, 'initial');
  assert.deepEqual(nodes.get('nested-grandchild').rect, {
    ...nodes.get('nested-grandchild').flowRect,
    x: 10,
    y: 351,
  });
  assert.deepEqual(nodes.get('nested-outside-after').rect, nodes.get('nested-outside-after').flowRect);
});

test('display:none은 positioned subtree 상자를 제거하고 뒤 형제는 흐름에 남긴다', () => {
  const nodes = nodeMap(reference.observations[0].states[0]);
  for (const id of ['hidden-parent', 'hidden-child']) {
    assert.equal(nodes.get(id).hasLayoutBox, false);
    assert.equal(nodes.get(id).owner, 'none');
    assert.deepEqual(nodes.get(id).rect, { x: 0, y: 0, width: 0, height: 0 });
  }
  assert.equal(nodes.get('hidden-sibling').hasLayoutBox, true);
  assert.equal(nodes.get('hidden-sibling').rect.y, 528);
});

test('runtime position 변경은 흐름을 보존하고 visual offset과 descendant owner를 갱신한다', () => {
  const states = byState(reference.observations[0]);
  const initial = nodeMap(states.get('initial'));
  const targetRelative = nodeMap(states.get('target-relative'));
  const ancestorRelative = nodeMap(states.get('ancestor-relative'));
  for (const id of ['dynamic-parent', 'dynamic-target', 'dynamic-after']) {
    assert.deepEqual(targetRelative.get(id).flowRect, initial.get(id).flowRect, id + ' target flow');
  }
  assert.deepEqual(targetRelative.get('dynamic-target').rect, { x: 15, y: 570, width: 40, height: 12 });
  assert.equal(ancestorRelative.get('dynamic-target').owner, 'dynamic-parent');
  assert.deepEqual(ancestorRelative.get('dynamic-target').rect, { x: 25, y: 573, width: 40, height: 12 });
  assert.equal(ancestorRelative.get('dynamic-nested-child').owner, 'dynamic-nested-parent');
  assert.deepEqual(ancestorRelative.get('dynamic-nested-child').rect, { x: 14, y: 593, width: 40, height: 12 });
  assert.deepEqual(ancestorRelative.get('dynamic-after').rect, initial.get('dynamic-after').rect);
});

test('DPR만 바뀔 때 computed CSSOM 값과 geometry는 달라지지 않는다', () => {
  const one = byState(reference.observations[0]);
  const two = byState(reference.observations[1]);
  for (const stateName of one.keys()) {
    const first = nodeMap(one.get(stateName));
    const second = nodeMap(two.get(stateName));
    for (const { id } of inventory.nodes) {
      assert.deepEqual(second.get(id).properties, first.get(id).properties, stateName + '/' + id + ' properties');
      assert.deepEqual(second.get(id).rect, first.get(id).rect, stateName + '/' + id + ' rect');
      assert.deepEqual(second.get(id).flowRect, first.get(id).flowRect, stateName + '/' + id + ' flowRect');
    }
  }
});

test('Android 실기기와 iOS Simulator의 V8 runtime frame이 고정 Chromium geometry와 일치한다', async () => {
  const expectedStates = byState(reference.observations[0]);
  for (const [platform, path] of [
    ['Android 실기기', paths.androidRuntimeLog],
    ['iOS Simulator', paths.iosRuntimeLog],
  ]) {
    const log = (await read(path)).toString('utf8');
    assert.match(log, /SPINON_C0410_DRAW .*status=0 presented boxes=45/, platform + ' WGPU 제출');

    for (const stateName of ['initial', 'target-relative', 'ancestor-relative']) {
      assert.match(
        log,
        new RegExp(
          'SPINON_C121_(?:SUMMARY state=' + stateName + '|STATE name=' + stateName
            + ' attempts=\\d+) .*status=0 layout=ready boxes=45',
        ),
        platform + ' ' + stateName + ' runtime layout',
      );
      assert.match(
        log,
        new RegExp('SPINON_C121_FRAME_SUMMARY state=' + stateName + ' frames=48 marker=present'),
        platform + ' ' + stateName + ' frame count',
      );
      const frames = new Map();
      const pattern = new RegExp(
        'SPINON_C121_NODE_FRAME state=' + stateName
          + ' \\d+:node=(\\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^\\s]+)',
        'g',
      );
      for (const match of log.toString().matchAll(pattern)) {
        frames.set(Number(match[1]), match.slice(2).map(Number));
      }
      assert.equal(frames.size, 48, platform + ' ' + stateName + ' raw frame count');

      const expectedNodes = nodeMap(expectedStates.get(stateName));
      let maximumError = 0;
      for (let index = 0; index < inventory.nodes.length; index++) {
        const fixtureNode = inventory.nodes[index];
        // 런타임은 root(1), style(2), style text(3) 뒤에 fixture div를 source order로 만든다.
        const runtimeNodeId = index === 0 ? 1 : index + 3;
        const actual = frames.get(runtimeNodeId);
        assert.ok(actual, platform + ' ' + stateName + ' ' + fixtureNode.id + ' frame');
        const expected = expectedNodes.get(fixtureNode.id).rect;
        const expectedValues = [expected.x, expected.y, expected.width, expected.height];
        for (let axis = 0; axis < expectedValues.length; axis++) {
          // 모바일 fixture viewport는 301×100 CSS px라 root의 viewport 크기만 Chrome과 다르다.
          if (index === 0 && axis >= 2) continue;
          const error = Math.abs(actual[axis] - expectedValues[axis]);
          maximumError = Math.max(maximumError, error);
          assert.ok(error <= inventory.comparison.maximumAbsoluteRectErrorCssPx,
            platform + ' ' + stateName + ' ' + fixtureNode.id + ' rect axis ' + axis
              + ': actual=' + actual[axis] + ', expected=' + expectedValues[axis]);
        }
      }
      assert.ok(maximumError <= inventory.comparison.maximumAbsoluteRectErrorCssPx);
    }
  }
});

test('기준 재수집은 명시적 교체 없이는 기존 파일을 보존한다', async () => {
  const bytesBefore = await read(paths.reference);
  const result = spawnSync(process.execPath, [join(repositoryRoot, paths.capture)], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    timeout: 5_000,
  });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /기존 Chromium reference를 덮어쓰지 않습니다/);
  assert.equal(sha256(await read(paths.reference)), sha256(bytesBefore));
});
