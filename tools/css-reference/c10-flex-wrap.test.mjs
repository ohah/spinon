import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c10/flex-wrap-inventory.json';
const htmlPath = 'tests/fixtures/css/c10/flex-wrap.html';
const referencePath = 'tests/fixtures/css/references/c10-flex-wrap-v1.json';
const capturePath = 'tools/css-reference/capture-c10-flex-wrap.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const runtimeFixturePath = 'tests/fixtures/css/c10/runtime-flex-wrap.js';
const androidLogPath = 'spec/internal/evidence/c10-1-flex-wrap/android-api37-emulator-logcat.txt';
const iosLogPath = 'spec/internal/evidence/c10-1-flex-wrap/ios-26.2-simulator-log.txt';
const read = async (path) => readFile(join(repositoryRoot, path));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [inventoryBytes, htmlBytes, referenceBytes, captureBytes, helperBytes, runtimeFixtureBytes,
  androidLogBytes, iosLogBytes] = await Promise.all([
  read(inventoryPath), read(htmlPath), read(referencePath), read(capturePath), read(helperPath),
  read(runtimeFixturePath), read(androidLogPath), read(iosLogPath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const observations = reference.observations;
const caseMap = (observation) => new Map(observation.cases.map((entry) => [entry.id, entry]));
const nodeMap = (entry) => new Map(entry.nodes.map((node) => [node.id, node]));
const dprOne = caseMap(observations[0]);

test('고정 Chrome reference가 fixture·capture 도구·실행 파일의 digest를 보존한다', () => {
  assert.equal(reference.schema, 'spinon-css-c10-flex-wrap-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, hash(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, hash(htmlBytes));
  assert.equal(reference.captureTool.sha256, hash(captureBytes));
  assert.equal(reference.captureTool.dependencies[0].sha256, hash(helperBytes));
  assert.match(reference.oracle.executableSha256, /^[a-f0-9]{64}$/);
  assert.equal(inventory.cases.length, 12);
  assert.equal(inventory.cases.reduce((count, entry) => count + entry.nodes.length, 0), 44);
  assert.deepEqual(inventory.viewport, { width: 320, height: 240, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.deepEqual(reference.comparison.rectFields, ['x', 'y', 'width', 'height']);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('각 case의 Chromium node 순서와 DPR 1·2 결과가 동일하다', () => {
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
    }
    if (index === 1) assert.deepEqual(observation.cases, observations[0].cases);
  }
});

test('row wrap의 default·exact fit·1px 초과·여러 줄 gap 경계를 고정한다', () => {
  const defaultRow = nodeMap(dprOne.get('row-default-nowrap'));
  assert.equal(defaultRow.get('c101-default-root').properties['flex-wrap'], 'nowrap');
  assert.deepEqual(defaultRow.get('c101-default-second').rect, { x: 60, y: 0, width: 60, height: 10 });

  const exact = nodeMap(dprOne.get('row-exact-fit'));
  assert.deepEqual(exact.get('c101-exact-second').rect, { x: 55, y: 0, width: 50, height: 10 });

  const over = nodeMap(dprOne.get('row-one-pixel-over'));
  assert.deepEqual(over.get('c101-over-second').rect, { x: 0, y: 10, width: 50, height: 10 });

  const lines = nodeMap(dprOne.get('row-three-lines-gap'));
  for (const [id, rect] of [
    ['c101-lines-first', { x: 0, y: 0, width: 40, height: 10 }],
    ['c101-lines-second', { x: 45, y: 0, width: 40, height: 10 }],
    ['c101-lines-third', { x: 0, y: 12, width: 40, height: 10 }],
    ['c101-lines-fourth', { x: 45, y: 12, width: 40, height: 10 }],
    ['c101-lines-fifth', { x: 0, y: 24, width: 20, height: 10 }],
  ]) assert.deepEqual(lines.get(id).rect, rect, id);
});

test('column wrap에서 row-gap은 주축, column-gap은 교차축 간격으로 동작한다', () => {
  const nodes = nodeMap(dprOne.get('column-wrap-gap'));
  assert.equal(nodes.get('c101-column-root').properties['flex-direction'], 'column');
  assert.deepEqual(nodes.get('c101-column-second').rect, { x: 0, y: 22, width: 10, height: 18 });
  assert.deepEqual(nodes.get('c101-column-third').rect, { x: 13, y: 0, width: 10, height: 18 });
});

test('실제 Android·iOS 앱 fixture의 일곱 DOM frame을 고정한다', () => {
  const runtime = nodeMap(dprOne.get('runtime-three-lines-gap'));
  assert.deepEqual(runtime.get('c101-runtime-root').rect,
    { x: 0, y: 0, width: 320, height: 240 });
  assert.equal(runtime.get('c101-runtime-root').properties.display, 'flex');
  assert.equal(runtime.get('c101-runtime-root').properties['flex-direction'], 'column');
  assert.deepEqual(runtime.get('c101-runtime-flex').rect,
    { x: 8, y: 8, width: 170, height: 68 });
  assert.equal(runtime.get('c101-runtime-flex').properties['flex-wrap'], 'wrap');
  for (const [id, rect] of [
    ['c101-runtime-item-1', { x: 8, y: 8, width: 70, height: 20 }],
    ['c101-runtime-item-2', { x: 84, y: 8, width: 70, height: 20 }],
    ['c101-runtime-item-3', { x: 8, y: 32, width: 70, height: 20 }],
    ['c101-runtime-item-4', { x: 84, y: 32, width: 70, height: 20 }],
    ['c101-runtime-item-5', { x: 8, y: 56, width: 70, height: 20 }],
  ]) assert.deepEqual(runtime.get(id).rect, rect, id);
});

test('Android API 37·iOS 26.2 simulator의 실제 runtime frame이 Chromium과 일치한다', () => {
  const expected = dprOne.get('runtime-three-lines-gap').nodes.map(({ rect }) => rect);
  const pattern = /SPINON_C101_NODE_FRAME (\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g;
  for (const [platform, bytes] of [['Android', androidLogBytes], ['iOS', iosLogBytes]]) {
    const log = bytes.toString('utf8');
    assert.match(log, /SPINON_C0410_DRAW .*presented boxes=7/, platform);
    const frames = [...log.matchAll(pattern)].map((match) => ({
      preorder: Number(match[1]),
      node: Number(match[2]),
      rect: { x: Number(match[3]), y: Number(match[4]), width: Number(match[5]), height: Number(match[6]) },
    }));
    assert.equal(frames.length, expected.length, `${platform} frame 수`);
    assert.deepEqual(frames.map(({ preorder }) => preorder), [0, 1, 2, 3, 4, 5, 6], `${platform} DOM preorder`);
    assert.equal(new Set(frames.map(({ node }) => node)).size, expected.length, `${platform} node ID 고유성`);
    assert.deepEqual(frames.map(({ rect }) => rect), expected, `${platform} vs Chromium`);
  }
});

test('empty·single·display:none·shorthand·Block·비상속 경계를 고정한다', () => {
  assert.deepEqual(nodeMap(dprOne.get('empty-container')).get('c101-empty-root').rect,
    { x: 0, y: 0, width: 100, height: 20 });
  assert.deepEqual(nodeMap(dprOne.get('single-item')).get('c101-single-child').rect,
    { x: 0, y: 0, width: 20, height: 10 });

  const hidden = nodeMap(dprOne.get('display-none-child'));
  assert.deepEqual(hidden.get('c101-hidden-item').rect, { x: 0, y: 0, width: 0, height: 0 });
  assert.deepEqual(hidden.get('c101-hidden-last').rect, { x: 55, y: 0, width: 45, height: 10 });

  const shorthand = nodeMap(dprOne.get('flow-shorthand-override'));
  assert.equal(shorthand.get('c101-flow-root').properties['flex-wrap'], 'wrap');
  assert.equal(shorthand.get('c101-flow-root').properties['flex-direction'], 'row');
  assert.deepEqual(shorthand.get('c101-flow-second').rect, { x: 55, y: 0, width: 50, height: 10 });

  const block = nodeMap(dprOne.get('block-wrap-noop'));
  assert.equal(block.get('c101-block-root').properties.display, 'block');
  assert.equal(block.get('c101-block-root').properties['flex-wrap'], 'wrap');
  assert.deepEqual(block.get('c101-block-second').rect, { x: 0, y: 10, width: 50, height: 10 });

  const nested = nodeMap(dprOne.get('nested-wrap-not-inherited'));
  assert.equal(nested.get('c101-nested-outer').properties['flex-wrap'], 'wrap');
  assert.equal(nested.get('c101-nested-inner').properties['flex-wrap'], 'nowrap');
  assert.deepEqual(nested.get('c101-nested-second').rect, { x: 40, y: 0, width: 40, height: 10 });
  assert.deepEqual(nested.get('c101-nested-sibling').rect, { x: 0, y: 10, width: 45, height: 10 });
});

test('capture는 reference를 자동 갱신하지 않고 실행 시 고정된 Chrome만 받는다', async () => {
  const capture = captureBytes.toString('utf8');
  assert.match(capture, /Google Chrome 154\.0\.8037\.98/);
  assert.match(capture, /기준 reference는 덮어쓰지 않습니다/);
  assert.match(capture, /flag: 'wx'/);
});

test('실제 runtime fixture가 Android·iOS에서 같은 V8→Stylo→Taffy→WGPU 경로를 쓴다', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/ffi/c10.rs',
    ffiModule: 'crates/spinon-ffi/src/runtime_gpu/ffi.rs',
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
  const fixture = runtimeFixtureBytes.toString('utf8');
  assert.match(fixture, /flex-wrap:wrap/);
  assert.match(fixture, /row-gap:4px;column-gap:6px/);
  assert.match(fixture, /const tiles = \[[^\]]+\]/);
  assert.match(fixture, /index < tiles\.length/);
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_flex_wrap_fixture/);
  assert.match(source.ffiModule, /mod c10;/);
  assert.match(source.ffiSource, /runtime-flex-wrap\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_flex_wrap_fixture/);
  assert.match(source.androidJni, /nativeEvalFlexWrapFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_wrap_fixture/);
  assert.match(source.androidDemo, /spinon_c101_flex_wrap[\s\S]*nativeEvalFlexWrapFixture/);
  assert.match(source.androidActivity, /spinon_c101_flex_wrap/);
  assert.match(source.iosRunner, /evalRuntimeGpuFlexWrapFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_wrap_fixture/);
  assert.match(source.iosHeader, /evalRuntimeGpuFlexWrapFixture/);
  assert.match(source.iosDemo, /--spinon-c101-flex-wrap[\s\S]*evalRuntimeGpuFlexWrapFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c101-flex-wrap/);
});
