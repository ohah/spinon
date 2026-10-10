import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c10/flex-order-alignment-inventory.json',
  html: 'tests/fixtures/css/c10/flex-order-alignment.html',
  runtime: 'tests/fixtures/css/c10/runtime-flex-order-alignment.js',
  reference: 'tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json',
  capture: 'tools/css-reference/capture-c10-3-flex-order-alignment.mjs',
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
const node = (caseId, id) => {
  const found = cases.get(caseId)?.nodes.find((entry) => entry.id === id);
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

test('고정 Chrome 기준은 fixture와 capture 도구 digest를 보존한다', async () => {
  assert.equal(reference.schema, 'spinon-css-c10-3-2-flex-order-reference/v1');
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
  assert.equal(inventory.cases.length, 11);
  assert.equal(inventory.cases.reduce((total, entry) => total + entry.nodes.length, 0), 48);
  assert.deepEqual(inventory.viewport, { width: 320, height: 240, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.deepEqual(reference.wpt.paths.map(({ path }) => path), [
    'css/css-flexbox/flex-order.html',
    'css/css-flexbox/flexbox-order-from-lowest.html',
    'css/css-flexbox/flexbox-order-only-flexitems.html',
    'css/css-flexbox/flexbox-paint-ordering-001.xhtml',
  ]);
});

test('두 DPR에서 computed integer, 원본 자식 순서와 CSS px frame이 같다', () => {
  assert.equal(observations.length, 2);
  assert.deepEqual(observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  for (const [index, observation] of observations.entries()) {
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
          `${expectedCase.id}/${expectedNode.id}: browser DOM children stay in source order`);
        assert.ok(Number.isInteger(actualNode.typed.order), `${actualNode.id} Typed OM order`);
        assert.ok(inventory.comparison.rectFields.every((field) => Number.isFinite(actualNode.rect[field])));
      }
    }
    if (index === 1) assert.deepEqual(observation.cases, observations[0].cases);
  }
});

test('기본 순서, tie, 줄 수집, grow와 중첩 컨테이너 경계를 고정한다', () => {
  assert.deepEqual(
    ['c1032-basic-b', 'c1032-basic-c', 'c1032-basic-a'].map((id) =>
      node('order-basic-mixed', id).rect.x),
    [0, 40, 80],
  );
  assert.deepEqual(
    ['c1032-tie-b', 'c1032-tie-a', 'c1032-tie-c', 'c1032-tie-d']
      .map((id) => node('order-ties-and-hidden', id).rect.x),
    [0, 30, 60, 90],
  );
  assertRect('order-ties-and-hidden', 'c1032-tie-hidden', { x: 0, y: 0, width: 0, height: 0 });
  assert.deepEqual(
    ['c1032-wrap-b', 'c1032-wrap-c', 'c1032-wrap-a']
      .map((id) => [node('order-wrap-line-collection', id).rect.x,
        node('order-wrap-line-collection', id).rect.y]),
    [[0, 0], [60, 0], [0, 30]],
  );
  assert.deepEqual(
    ['c1032-grow-b', 'c1032-grow-c', 'c1032-grow-a']
      .map((id) => [node('order-grow-distribution', id).rect.x,
        node('order-grow-distribution', id).rect.width]),
    [[0, 160], [160, 40], [200, 100]],
  );
  assert.deepEqual(
    ['c1032-nested-b', 'c1032-nested-a'].map((id) => node('order-nested-scope', id).rect.x),
    [0, 60],
  );
  assert.deepEqual(
    ['c1032-nested-a2', 'c1032-nested-a3', 'c1032-nested-a1']
      .map((id) => node('order-nested-scope', id).rect.x),
    [60, 80, 100],
  );
});

test('invalid fractional declaration, calc rounding, custom properties와 int32 clamp를 고정한다', () => {
  assert.deepEqual(
    ['c1032-calc-a', 'c1032-calc-b', 'c1032-calc-c', 'c1032-calc-d']
      .map((id) => node('order-calc-invalid', id).typed.order),
    [4, 0, 2, -1],
  );
  assert.deepEqual(
    ['c1032-high-a', 'c1032-high-b', 'c1032-high-c']
      .map((id) => node('order-int32-upper-clamp', id).typed.order),
    [2147483647, 2147483647, 2147483647],
  );
  assert.deepEqual(
    ['c1032-low-a', 'c1032-low-b', 'c1032-low-c']
      .map((id) => node('order-int32-lower-clamp', id).typed.order),
    [-2147483647, -2147483648, -2147483648],
  );
  assert.deepEqual(
    ['c1032-custom-b', 'c1032-custom-c', 'c1032-custom-a']
      .map((id) => node('order-custom-property', id).rect.x),
    [0, 40, 80],
  );
});

test('비-Flex 자식은 source order를 유지하고 overlap case의 order 그룹은 서로 덮친다', () => {
  assertRect('order-non-flex', 'c1032-block-a', { x: 0, y: 0, width: 50, height: 20 });
  assertRect('order-non-flex', 'c1032-block-b', { x: 0, y: 20, width: 50, height: 20 });
  const overlap = ['c1032-paint-a', 'c1032-paint-b', 'c1032-paint-c']
    .map((id) => node('order-overlap-paint', id).rect);
  assert.ok(overlap.every(({ x, width }) => x < 0 && x + width > 0),
    '0px에서 세 형제 Flex item이 모두 겹쳐 paint 결과를 구분할 수 있어야 합니다.');
  assert.deepEqual(
    ['c1032-paint-b', 'c1032-paint-c', 'c1032-paint-a']
      .map((id) => node('order-overlap-paint', id).typed.order),
    [-1, 0, 2],
  );
});

test('실제 V8 fixture가 Android·iOS의 C10.3.2 WGPU 실행 경로에 연결된다', async () => {
  const sourcePaths = {
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
  const source = Object.fromEntries(await Promise.all(Object.entries(sourcePaths).map(
    async ([name, path]) => [name, (await read(path)).toString('utf8')],
  )));
  assert.match(files.runtime.toString('utf8'), /\["a", 2,/);
  assert.match(files.runtime.toString('utf8'), /\["b", -1,/);
  assert.match(files.runtime.toString('utf8'), /c1032-runtime-overlap/);
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_flex_order_fixture/);
  assert.match(source.ffiSource, /runtime-flex-order-alignment\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_flex_order_fixture/);
  assert.match(source.androidJni, /nativeEvalFlexOrderFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_order_fixture/);
  assert.match(source.androidDemo, /spinon_c1032_flex_order[\s\S]*nativeEvalFlexOrderFixture/);
  assert.match(source.androidActivity, /spinon_c1032_flex_order/);
  assert.match(source.iosRunner, /evalRuntimeGpuFlexOrderFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_order_fixture/);
  assert.match(source.iosHeader, /evalRuntimeGpuFlexOrderFixture/);
  assert.match(source.iosDemo, /--spinon-c1032-flex-order[\s\S]*evalRuntimeGpuFlexOrderFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c1032-flex-order/);
});

test('Android·iOS Simulator의 실제 9개 frame과 WGPU 제출 결과가 서로 일치한다', async () => {
  const evidence = {
    android: (await read('spec/internal/evidence/c10-3-2-flex-order/android-api37-emulator.log'))
      .toString('utf8'),
    ios: (await read('spec/internal/evidence/c10-3-2-flex-order/ios-26.2-iphone-17-pro.log'))
      .toString('utf8'),
  };
  const expectedFrames = [
    [1, 0, 0, 320, 240],
    [2, 0, 0, 180, 40],
    [3, 120, 0, 60, 40],
    [4, 0, 0, 60, 40],
    [5, 60, 0, 60, 40],
    [6, 0, 52, 120, 40],
    [7, 70, 52, 80, 40],
    [8, -30, 52, 80, 40],
    [9, 20, 52, 80, 40],
  ];
  const parseFrames = (log, platform) => {
    assert.match(log, /SPINON_C1032_EVAL status=0/, `${platform} V8 evaluation must succeed`);
    assert.match(log, /SPINON_C0410_DRAW[^\n]*status=0 presented boxes=9/,
      `${platform} WGPU must present all nine scene boxes`);
    const matches = [...log.matchAll(
      /SPINON_C1032_NODE_FRAME \d+:node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g,
    )];
    assert.ok(matches.length >= expectedFrames.length, `${platform} must log all nine frames`);
    return matches.slice(-expectedFrames.length).map((match) => match.slice(1).map(Number));
  };

  const androidFrames = parseFrames(evidence.android, 'Android API 37 emulator');
  const iosFrames = parseFrames(evidence.ios, 'iOS 26.2 simulator');
  assert.deepEqual(androidFrames, expectedFrames);
  assert.deepEqual(iosFrames, expectedFrames);
  assert.deepEqual(androidFrames, iosFrames);
  assert.match(evidence.android, /SPINON_C0410_RENDERER=.*SwiftShader/,
    'Android 결과는 hardware GPU 측정으로 오인하지 않게 backend를 고정한다');
});
