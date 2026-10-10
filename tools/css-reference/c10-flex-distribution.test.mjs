import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c10/flex-distribution-inventory.json';
const htmlPath = 'tests/fixtures/css/c10/flex-distribution.html';
const runtimeFixturePath = 'tests/fixtures/css/c10/runtime-flex-distribution.js';
const referencePath = 'tests/fixtures/css/references/c10-flex-distribution-v1.json';
const capturePath = 'tools/css-reference/capture-c10-flex-distribution.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const read = async (path) => readFile(join(repositoryRoot, path));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [inventoryBytes, htmlBytes, runtimeFixtureBytes, referenceBytes, captureBytes, helperBytes] = await Promise.all([
  read(inventoryPath), read(htmlPath), read(runtimeFixturePath), read(referencePath), read(capturePath), read(helperPath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const observations = reference.observations;
const casesByDpr = observations.map((observation) => new Map(
  observation.cases.map((entry) => [entry.id, entry]),
));
const dprOne = casesByDpr[0];
const nodeMap = (id) => new Map(dprOne.get(id).nodes.map((node) => [node.id, node]));

function assertRect(caseId, nodeId, expected) {
  const actual = nodeMap(caseId).get(`c102-${nodeId}`)?.rect;
  assert.ok(actual, `${caseId}/${nodeId}가 reference에 있어야 합니다.`);
  for (const field of ['x', 'y', 'width', 'height']) {
    assert.ok(Math.abs(actual[field] - expected[field]) <= 0.001,
      `${caseId}/${nodeId}.${field}: expected ${expected[field]}, got ${actual[field]}`);
  }
}

test('고정 Chrome reference가 fixture·capture 도구·실행 파일 digest를 보존한다', () => {
  assert.equal(reference.schema, 'spinon-css-c10-flex-distribution-reference/v1');
  assert.equal(reference.fixture.id, 'C10.2-flex-distribution-v1');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, hash(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, hash(htmlBytes));
  assert.equal(reference.fixture.runtimeSourcePath, runtimeFixturePath);
  assert.equal(reference.fixture.runtimeSourceSha256, hash(runtimeFixtureBytes));
  assert.equal(reference.captureTool.sha256, hash(captureBytes));
  assert.equal(reference.captureTool.dependencies[0].sha256, hash(helperBytes));
  assert.match(reference.oracle.executableSha256, /^[a-f0-9]{64}$/);
  assert.equal(inventory.cases.length, 27);
  assert.equal(inventory.cases.reduce((count, entry) => count + entry.nodes.length, 0), 92);
  assert.deepEqual(inventory.viewport, { width: 320, height: 240, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.deepEqual(reference.comparison.rectFields, ['x', 'y', 'width', 'height']);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('모든 관찰 node의 computed CSS와 geometry가 고정돼 있고 DPR 1·2 결과가 같다', () => {
  assert.equal(observations.length, 2);
  assert.deepEqual(observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  for (const [index, observation] of observations.entries()) {
    assert.deepEqual([observation.viewport.width, observation.viewport.height], [320, 240]);
    assert.equal(observation.environment.locale, 'en-US');
    assert.equal(observation.environment.timeZone, 'UTC');
    assert.equal(observation.environment.dark, false);
    assert.equal(observation.environment.coarsePointer, true);
    assert.equal(observation.environment.hover, false);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
    for (const [caseIndex, fixtureCase] of inventory.cases.entries()) {
      const actual = observation.cases[caseIndex];
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodes, fixtureCase.id);
      assert.ok(actual.nodes.every((node) => inventory.comparison.computedProperties.every(
        (property) => typeof node.properties[property] === 'string',
      )), fixtureCase.id);
      assert.ok(actual.nodes.every((node) => inventory.comparison.rectFields.every(
        (field) => Number.isFinite(node.rect[field]),
      )), fixtureCase.id);
      const nodeIds = new Set(fixtureCase.nodes);
      assert.ok(actual.nodes.every((node) => node.children.every((child) => nodeIds.has(child))
        && typeof node.sourceSize?.width === 'string'
        && typeof node.sourceSize?.height === 'string'), fixtureCase.id);
    }
    if (index === 1) assert.deepEqual(observation.cases, observations[0].cases);
  }
});

test('grow와 scaled shrink가 CSS factor와 basis를 함께 반영한다', () => {
  assertRect('grow-equal', 'grow-equal-first', { x: 0, y: 0, width: 300, height: 10 });
  assertRect('grow-equal', 'grow-equal-second', { x: 300, y: 0, width: 300, height: 10 });
  assertRect('grow-weighted', 'grow-weighted-first', { x: 0, y: 0, width: 175, height: 10 });
  assertRect('grow-weighted', 'grow-weighted-second', { x: 175, y: 0, width: 250, height: 10 });
  assertRect('grow-weighted', 'grow-weighted-third', { x: 425, y: 0, width: 175, height: 10 });
  assertRect('grow-subunit', 'grow-subunit-first', { x: 0, y: 0, width: 125, height: 10 });
  assertRect('grow-subunit', 'grow-subunit-second', { x: 125, y: 0, width: 125, height: 10 });
  assertRect('grow-zero', 'grow-zero-first', { x: 0, y: 0, width: 100, height: 10 });
  assertRect('grow-zero', 'grow-zero-second', { x: 100, y: 0, width: 200, height: 10 });
  assertRect('shrink-scaled', 'shrink-scaled-first', { x: 0, y: 0, width: 160, height: 10 });
  assertRect('shrink-scaled', 'shrink-scaled-second', { x: 160, y: 0, width: 80, height: 10 });
  assertRect('shrink-weighted', 'shrink-weighted-first', { x: 0, y: 0, width: 150, height: 10 });
  assertRect('shrink-weighted', 'shrink-weighted-second', { x: 150, y: 0, width: 50, height: 10 });
  assertRect('shrink-zero', 'shrink-zero-first', { x: 0, y: 0, width: 200, height: 10 });
  assertRect('shrink-zero', 'shrink-zero-second', { x: 200, y: 0, width: 40, height: 10 });
});

test('flex-basis 우선순위와 definite row·column의 auto·percentage 계산을 고정한다', () => {
  assertRect('basis-overrides-size', 'basis-overrides-item', { x: 0, y: 0, width: 100, height: 10 });
  assertRect('basis-auto-explicit-size', 'basis-auto-row-item', { x: 0, y: 0, width: 200, height: 10 });
  assertRect('basis-auto-explicit-size', 'basis-auto-column-item', { x: 0, y: 30, width: 10, height: 60 });
  assertRect('basis-percent-row-column', 'basis-percent-row-first', { x: 0, y: 0, width: 100, height: 10 });
  assertRect('basis-percent-row-column', 'basis-percent-row-second', { x: 100, y: 0, width: 200, height: 10 });
  assertRect('basis-percent-row-column', 'basis-percent-column-first', { x: 0, y: 30, width: 10, height: 60 });
  assertRect('basis-percent-row-column', 'basis-percent-column-second', { x: 0, y: 90, width: 10, height: 120 });
});

test('shorthand·author stylesheet·invalid 선언의 computed style과 cascade 결과를 고정한다', () => {
  const shorthand = nodeMap('flex-shorthand');
  assert.equal(shorthand.get('c102-shorthand-one').properties.flex, '1 1 0%');
  assert.equal(shorthand.get('c102-shorthand-auto').properties.flex, '1 1 auto');
  assert.equal(shorthand.get('c102-shorthand-none').properties.flex, '0 0 auto');
  assert.equal(shorthand.get('c102-shorthand-three-part').properties.flex, '2 1 80px');
  assert.equal(shorthand.get('c102-shorthand-override').properties['flex-grow'], '0.5');
  assertRect('flex-shorthand', 'shorthand-one', { x: 0, y: 0, width: 102.21875, height: 10 });
  assertRect('flex-shorthand', 'shorthand-auto', { x: 102.21875, y: 0, width: 202.21875, height: 10 });
  assertRect('flex-shorthand', 'shorthand-none', { x: 304.4375, y: 0, width: 100, height: 10 });
  assertRect('flex-shorthand', 'shorthand-three-part', { x: 404.4375, y: 0, width: 284.453125, height: 10 });
  assertRect('flex-shorthand', 'shorthand-override', { x: 688.890625, y: 0, width: 111.109375, height: 10 });
  const stylesheet = nodeMap('stylesheet-grow');
  assert.equal(stylesheet.get('c102-stylesheet-first').properties['flex-grow'], '1');
  assert.equal(stylesheet.get('c102-stylesheet-second').properties['flex-grow'], '3');
  assertRect('stylesheet-grow', 'stylesheet-first', { x: 0, y: 0, width: 125, height: 10 });
  assertRect('stylesheet-grow', 'stylesheet-second', { x: 125, y: 0, width: 175, height: 10 });
  const invalid = nodeMap('invalid-css-factor');
  assert.equal(invalid.get('c102-invalid-first').properties['flex-grow'], '1');
  assert.equal(invalid.get('c102-invalid-first').properties['flex-basis'], '100px');
  assertRect('invalid-css-factor', 'invalid-first', { x: 0, y: 0, width: 150, height: 10 });
  assertRect('invalid-css-factor', 'invalid-second', { x: 150, y: 0, width: 150, height: 10 });
});

test('min·max freeze 반복과 hypothetical main size 분기 결과를 고정한다', () => {
  assertRect('grow-max-freeze', 'grow-max-first', { x: 0, y: 0, width: 120, height: 10 });
  assertRect('grow-max-freeze', 'grow-max-second', { x: 120, y: 0, width: 190, height: 10 });
  assertRect('grow-max-freeze', 'grow-max-third', { x: 310, y: 0, width: 190, height: 10 });
  assertRect('grow-min-freeze', 'grow-min-first', { x: 0, y: 0, width: 180, height: 10 });
  assertRect('grow-min-freeze', 'grow-min-second', { x: 180, y: 0, width: 120, height: 10 });
  assertRect('shrink-min-freeze', 'shrink-min-first', { x: 0, y: 0, width: 150, height: 10 });
  assertRect('shrink-min-freeze', 'shrink-min-second', { x: 150, y: 0, width: 50, height: 10 });
  assertRect('shrink-max-freeze', 'shrink-max-first', { x: 0, y: 0, width: 120, height: 10 });
  assertRect('shrink-max-freeze', 'shrink-max-second', { x: 120, y: 0, width: 80, height: 10 });
  assertRect('shrink-min-overflow', 'shrink-overflow-first', { x: 0, y: 0, width: 150, height: 10 });
  assertRect('shrink-min-overflow', 'shrink-overflow-second', { x: 150, y: 0, width: 150, height: 10 });
  assertRect('mixed-violation-freeze', 'mixed-first', { x: 0, y: 0, width: 120, height: 10 });
  assertRect('mixed-violation-freeze', 'mixed-second', { x: 120, y: 0, width: 200, height: 10 });
  assertRect('mixed-violation-freeze', 'mixed-third', { x: 320, y: 0, width: 180, height: 10 });
  assertRect('min-over-max', 'min-over-max-item', { x: 0, y: 0, width: 150, height: 10 });
  assertRect('hypothetical-factor-choice', 'hypothetical-first', { x: 0, y: 0, width: 100, height: 10 });
  assertRect('hypothetical-factor-choice', 'hypothetical-second', { x: 100, y: 0, width: 140, height: 10 });
});

test('gap·margin·wrap line·box sizing·소수 좌표·큰 factor frame을 고정한다', () => {
  assertRect('gap-accounting', 'gap-first', { x: 0, y: 0, width: 140, height: 10 });
  assertRect('gap-accounting', 'gap-second', { x: 160, y: 0, width: 140, height: 10 });
  assertRect('fixed-margin-accounting', 'margin-first', { x: 10, y: 0, width: 130, height: 10 });
  assertRect('fixed-margin-accounting', 'margin-second', { x: 160, y: 0, width: 130, height: 10 });
  assertRect('wrapped-per-line', 'wrapped-first', { x: 0, y: 0, width: 250, height: 10 });
  assertRect('wrapped-per-line', 'wrapped-second', { x: 0, y: 10, width: 156.671875, height: 10 });
  assertRect('wrapped-per-line', 'wrapped-third', { x: 156.671875, y: 10, width: 93.328125, height: 10 });
  assertRect('box-sizing-basis', 'box-sizing-content', { x: 0, y: 0, width: 130, height: 20 });
  assertRect('box-sizing-basis', 'box-sizing-border', { x: 130, y: 0, width: 100, height: 20 });
  assertRect('fractional-and-large-factors', 'fractional-first', { x: 0, y: 0, width: 175.5, height: 10 });
  assertRect('fractional-and-large-factors', 'fractional-second', { x: 175.5, y: 0, width: 325, height: 10 });
  const fraction = nodeMap('fractional-and-large-factors');
  assert.equal(fraction.get('c102-fractional-first').properties['flex-grow'], '1e+20');
  assert.equal(fraction.get('c102-fractional-second').properties['flex-grow'], '3e+20');
});

test('실제 V8 runtime fixture의 mixed min·max freeze와 custom paint frame을 고정한다', () => {
  assertRect('runtime-flex-distribution', 'runtime-root', { x: 0, y: 0, width: 320, height: 240 });
  assertRect('runtime-flex-distribution', 'runtime-flex', { x: 8, y: 8, width: 304, height: 24 });
  assertRect('runtime-flex-distribution', 'runtime-item-1', { x: 8, y: 8, width: 80, height: 20 });
  assertRect('runtime-flex-distribution', 'runtime-item-2', { x: 88, y: 8, width: 120, height: 20 });
  assertRect('runtime-flex-distribution', 'runtime-item-3', { x: 208, y: 8, width: 104, height: 20 });
  const source = runtimeFixtureBytes.toString('utf8');
  assert.match(source, /flex:1 0 60px/);
  assert.match(source, /max-width:80px/);
  assert.match(source, /min-width:120px/);
  assert.match(source, /background-color:var\(\$\{tile\.color\}\)/);
});

test('실제 runtime fixture가 Android·iOS에서 C10.2 V8→Stylo→Taffy→WGPU 경로로 연결된다', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/ffi/c10.rs',
    ffiSource: 'crates/spinon-ffi/src/runtime_gpu.rs',
    ffiHeader: 'crates/spinon-ffi/include/spinon_ffi.h',
    androidJni: 'platforms/android/app/src/main/cpp/spinon_jni.cc',
    androidDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    androidActivity: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java',
    iosRunner: 'platforms/ios/Sources/SpinonRunner.mm',
    iosHeader: 'platforms/ios/Sources/SpinonRunner.h',
    iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    iosAppDelegate: 'platforms/ios/Sources/AppDelegate.swift',
  };
  const source = Object.fromEntries(await Promise.all(Object.entries(paths).map(
    async ([name, path]) => [name, (await read(path)).toString('utf8')],
  )));
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_flex_distribution_fixture/);
  assert.match(source.ffiSource, /runtime-flex-distribution\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_flex_distribution_fixture/);
  assert.match(source.androidJni, /nativeEvalFlexDistributionFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_distribution_fixture/);
  assert.match(source.androidDemo, /spinon_c102_flex_distribution[\s\S]*nativeEvalFlexDistributionFixture/);
  assert.match(source.androidActivity, /spinon_c102_flex_distribution/);
  assert.match(source.iosRunner, /evalRuntimeGpuFlexDistributionFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_distribution_fixture/);
  assert.match(source.iosHeader, /evalRuntimeGpuFlexDistributionFixture/);
  assert.match(source.iosDemo, /--spinon-c102-flex-distribution[\s\S]*evalRuntimeGpuFlexDistributionFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c102-flex-distribution/);
});

test('Android·iOS Simulator의 실제 V8 frame과 WGPU box 제출이 Chromium과 일치한다', async () => {
  const evidenceRoot = 'spec/internal/evidence/c10-2-flex-distribution';
  const [androidLog, iosLog] = await Promise.all([
    read(`${evidenceRoot}/android-api37-emulator-logcat.txt`),
    read(`${evidenceRoot}/ios-26.2-simulator-log.txt`),
  ]);
  const expected = nodeMap('runtime-flex-distribution');
  const pattern = /SPINON_C102_NODE_FRAME (\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g;
  for (const [platform, bytes] of [['Android', androidLog], ['iOS', iosLog]]) {
    const log = bytes.toString('utf8');
    assert.match(log, /SPINON_C102_EVAL status=0 /, `${platform} runtime evaluation`);
    assert.match(log, /SPINON_C0410_DRAW status=0 presented boxes=5/, `${platform} WGPU submission`);
    const frames = [...log.matchAll(pattern)].map((match) => ({
      preorder: Number(match[1]),
      node: Number(match[2]),
      rect: { x: Number(match[3]), y: Number(match[4]), width: Number(match[5]), height: Number(match[6]) },
    }));
    assert.equal(frames.length, expected.size, `${platform} frame count`);
    assert.deepEqual(frames.map(({ preorder }) => preorder), [0, 1, 2, 3, 4], `${platform} DOM preorder`);
    assert.equal(new Set(frames.map(({ node }) => node)).size, expected.size, `${platform} unique NodeId`);
    for (const [index, frame] of frames.entries()) {
      const referenceNode = [...expected.values()][index];
      assert.equal(frame.node, index + 1, `${platform} ${referenceNode.id} NodeId`);
      for (const field of ['x', 'y', 'width', 'height']) {
        assert.ok(Math.abs(frame.rect[field] - referenceNode.rect[field]) <= 0.5,
          `${platform} ${referenceNode.id}.${field}: expected ${referenceNode.rect[field]}, got ${frame.rect[field]}`);
      }
    }
  }
});

test('capture는 고정 Chromium만 허용하고 reference 자동 덮어쓰기를 막는다', () => {
  const capture = captureBytes.toString('utf8');
  assert.match(capture, /Google Chrome 154\.0\.8037\.98/);
  assert.match(capture, /기준 reference는 덮어쓰지 않습니다/);
  assert.match(capture, /flag: 'wx'/);
});
