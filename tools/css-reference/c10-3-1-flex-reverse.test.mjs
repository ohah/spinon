import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c10/flex-reverse-inventory.json',
  html: 'tests/fixtures/css/c10/flex-reverse.html',
  runtime: 'tests/fixtures/css/c10/runtime-flex-reverse.js',
  reference: 'tests/fixtures/css/references/c10-3-1-flex-reverse-v1.json',
  capture: 'tools/css-reference/capture-c10-3-1-flex-reverse.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  androidLog: 'spec/internal/evidence/c10-3-1-flex-reverse/android-api37-emulator-logcat.txt',
  iosLog: 'spec/internal/evidence/c10-3-1-flex-reverse/ios-26.2-simulator-log.txt',
};
const read = async (path) => readFile(join(repositoryRoot, path));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const files = Object.fromEntries(await Promise.all(Object.entries(paths).map(
  async ([name, path]) => [name, await read(path)],
)));
const inventory = JSON.parse(files.inventory.toString('utf8'));
const reference = JSON.parse(files.reference.toString('utf8'));
const observations = reference.observations;
const firstObservationCases = new Map(observations[0].cases.map((entry) => [entry.id, entry]));
const nodeMap = (caseId) => new Map(firstObservationCases.get(caseId).nodes.map((node) => [node.id, node]));

function assertRect(caseId, nodeId, expected) {
  const actual = nodeMap(caseId).get(nodeId)?.rect;
  assert.ok(actual, `${caseId}/${nodeId}가 Chromium reference에 있어야 합니다.`);
  for (const field of ['x', 'y', 'width', 'height']) {
    assert.ok(Math.abs(actual[field] - expected[field]) <= 0.001,
      `${caseId}/${nodeId}.${field}: expected ${expected[field]}, got ${actual[field]}`);
  }
}

test('고정 Chrome reference가 fixture와 실행 도구 digest를 보존한다', () => {
  assert.equal(reference.schema, 'spinon-css-c10-3-1-flex-reverse-reference/v1');
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
  assert.equal(inventory.cases.length, 17);
  assert.equal(inventory.cases.reduce((count, entry) => count + entry.nodes.length, 0), 64);
  assert.deepEqual(inventory.viewport, { width: 320, height: 240, deviceScaleFactors: [1, 2] });
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.deepEqual(reference.wpt.paths.map(({ path }) => path), [
    'css/css-flexbox/flex-direction-row-reverse-001-visual.html',
    'css/css-flexbox/flex-direction-column-reverse-001-visual.html',
    'css/css-flexbox/flexbox-flex-wrap-wrap-reverse.htm',
    'css/css-flexbox/flex-flow-006.html',
    'css/css-flexbox/flex-flow-012.html',
  ]);
});

test('모든 Chromium 노드에서 computed CSS와 DPR별 CSS px 결과가 고정돼 있다', () => {
  assert.equal(observations.length, 2);
  assert.deepEqual(observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  for (const [index, observation] of observations.entries()) {
    assert.deepEqual([observation.viewport.width, observation.viewport.height], [320, 240]);
    assert.equal(observation.environment.locale, 'en-US');
    assert.equal(observation.environment.timeZone, 'UTC');
    assert.equal(observation.cases.length, inventory.cases.length);
    for (const [caseIndex, fixtureCase] of inventory.cases.entries()) {
      const actual = observation.cases[caseIndex];
      assert.equal(actual.id, fixtureCase.id);
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodes.map(({ id }) => id));
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

test('역방향 주축, 역방향 줄 쌓기, shorthand와 줄 경계의 기준 좌표를 고정한다', () => {
  assertRect('row-reverse-start', 'c1031-row-start-first', { x: 80, y: 0, width: 20, height: 10 });
  assertRect('row-reverse-start', 'c1031-row-start-third', { x: 40, y: 0, width: 20, height: 10 });
  assertRect('column-reverse-start', 'c1031-column-start-first', { x: 0, y: 60, width: 40, height: 20 });
  assertRect('row-wrap-reverse-three-lines', 'c1031-row-wrap-first', { x: 0, y: 24, width: 40, height: 10 });
  assertRect('column-wrap-reverse-two-lines', 'c1031-column-wrap-first', { x: 13, y: 0, width: 10, height: 18 });
  assertRect('row-reverse-wrap-reverse', 'c1031-row-combined-first', { x: 55, y: 24, width: 40, height: 10 });
  assertRect('column-reverse-wrap-reverse', 'c1031-column-combined-first', { x: 13, y: 22, width: 10, height: 18 });
  assertRect('row-reverse-exact-fit', 'c1031-exact-first', { x: 45, y: 0, width: 40, height: 10 });
  assertRect('row-reverse-one-pixel-over', 'c1031-over-second', { x: 44, y: 10, width: 40, height: 10 });
  assert.equal(nodeMap('flex-flow-direction-first').get('c1031-flow-direction-root').properties['flex-flow'], 'row-reverse wrap');
  assert.equal(nodeMap('flex-flow-wrap-first').get('c1031-flow-wrap-root').properties['flex-flow'], 'row-reverse wrap');
  assert.equal(nodeMap('flex-flow-longhand-override').get('c1031-flow-override-root').properties['flex-direction'], 'row');
});

test('실제 runtime JS fixture가 Android·iOS의 C10.3.1 WGPU 경로에 연결된다', async () => {
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
  assert.match(files.runtime.toString('utf8'), /flex-direction:row-reverse;flex-wrap:wrap-reverse/);
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_flex_reverse_fixture/);
  assert.match(source.ffiSource, /runtime-flex-reverse\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_flex_reverse_fixture/);
  assert.match(source.androidJni, /nativeEvalFlexReverseFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_reverse_fixture/);
  assert.match(source.androidDemo, /spinon_c1031_flex_reverse[\s\S]*nativeEvalFlexReverseFixture/);
  assert.match(source.androidActivity, /spinon_c1031_flex_reverse/);
  assert.match(source.iosRunner, /evalRuntimeGpuFlexReverseFixture[\s\S]*spinon_runtime_gpu_host_eval_flex_reverse_fixture/);
  assert.match(source.iosHeader, /evalRuntimeGpuFlexReverseFixture/);
  assert.match(source.iosDemo, /--spinon-c1031-flex-reverse[\s\S]*evalRuntimeGpuFlexReverseFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c1031-flex-reverse/);
});

test('Android·iOS Simulator의 실제 V8 frame과 WGPU 제출이 Chromium runtime 기준에 맞는다', async () => {
  const [androidLog, iosLog] = await Promise.all([
    read(paths.androidLog).then((bytes) => bytes.toString('utf8')),
    read(paths.iosLog).then((bytes) => bytes.toString('utf8')),
  ]);
  const expected = firstObservationCases.get('runtime-row-wrap-reverse').nodes;
  const tolerance = reference.comparison.maximumAbsoluteRectErrorCssPx;

  for (const [platform, log] of [['Android API 37', androidLog], ['iOS 26.2', iosLog]]) {
    assert.match(log, /SPINON_C1031_EVAL[^\n]*status=0[^\n]*document_nodes=7/,
      `${platform}: 실제 V8 평가가 7개 노드로 성공해야 합니다.`);
    if (platform === 'iOS 26.2') {
      assert.match(log, /SPINON_C1031_FRAME_SUMMARY frames=7 marker=present/,
        `${platform}: 7개 DOM frame이 별도 로그로 보존되어야 합니다.`);
    }
    assert.match(log, /SPINON_C0410_DRAW[^\n]*status=0 presented boxes=7/,
      `${platform}: WGPU가 7개 box를 제출해야 합니다.`);
    assert.doesNotMatch(log, /SPINON_C0410_QUEUE_FAILURE|SPINON_C1031_EVAL[^\n]*status=-/,
      `${platform}: C10.3.1 실행을 큐 실패 또는 평가 오류로 보고하면 안 됩니다.`);

    const frames = [...log.matchAll(
      /SPINON_C1031_NODE_FRAME\s+(\d+):node=(\d+),x=(-?[\d.]+),y=(-?[\d.]+),width=([\d.]+),height=([\d.]+)/g,
    )].map(([, index, node, x, y, width, height]) => ({
      index: Number(index), node: Number(node),
      rect: { x: Number(x), y: Number(y), width: Number(width), height: Number(height) },
    }));
    assert.equal(frames.length, expected.length, `${platform}: frame 개수`);
    for (const [index, frame] of frames.entries()) {
      assert.equal(frame.index, index, `${platform}: preorder index ${index}`);
      assert.equal(frame.node, index + 1, `${platform}: NodeId는 source preorder를 유지해야 합니다.`);
      for (const field of reference.comparison.rectFields) {
        const expectedRect = expected[index].rect[field];
        assert.ok(Math.abs(frame.rect[field] - expectedRect) <= tolerance,
          `${platform}: node=${frame.node}.${field} expected ${expectedRect}, got ${frame.rect[field]}`);
      }
    }
  }
});
