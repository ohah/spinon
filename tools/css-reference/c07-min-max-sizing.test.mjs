import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventory = JSON.parse(await readFile(join(repositoryRoot,
  'tests/fixtures/css/c07/min-max-sizing-inventory.json'), 'utf8'));
const referencePath = 'tests/fixtures/css/references/c07-min-max-sizing-v1.json';
const reference = JSON.parse(await readFile(join(repositoryRoot, referencePath), 'utf8'));
const observations = reference.observations;
const nodeMap = (observation) => new Map(observation.nodes.map((node) => [node.id, node]));
const readSource = (path) => readFile(join(repositoryRoot, path), 'utf8');

test('고정 Chromium 기준은 fixture 입력·실행 파일·35개 노드를 고정한다', () => {
  assert.equal(reference.schema, 'spinon-css-c07-min-max-sizing-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256.length, 64);
  assert.equal(reference.fixture.htmlSha256.length, 64);
  assert.equal(reference.captureTool.sha256.length, 64);
  assert.equal(reference.oracle.executableSha256.length, 64);
  assert.equal(inventory.nodes.length, 35);
  assert.equal(observations.length, 2);
  assert.equal(observations[0].nodes.length, inventory.nodes.length);
  assert.equal(observations[1].nodes.length, inventory.nodes.length);
});

test('고정 기준은 최소·최대값과 box-sizing 경계를 관찰한다', () => {
  const nodes = nodeMap(observations[0]);
  const cases = [
    ['min-width-raises', 'width', 90],
    ['max-width-limits', 'width', 90],
    ['min-height-raises', 'height', 55],
    ['max-height-limits', 'height', 55],
    ['min-over-max', 'width', 80],
    ['content-box-max', 'width', 120],
    ['border-box-max', 'width', 100],
    ['border-box-min', 'width', 100],
    ['border-box-padding-floor', 'width', 24],
    ['percentage-min', 'width', 100],
    ['percentage-max', 'width', 150],
    ['percentage-height-max', 'height', 60],
    ['calc-min', 'width', 50],
    ['calc-max', 'width', 80],
    ['var-min', 'width', 60],
  ];

  for (const [id, axis, expected] of cases) {
    const node = nodes.get(id);
    assert.ok(node, id);
    assert.equal(node.rect[axis], expected, `${id}.${axis}`);
  }
  assert.equal(nodes.get('content-box-max').properties['max-width'], '100px');
  assert.equal(nodes.get('border-box-max').properties['max-width'], '100px');
  assert.equal(nodes.get('min-over-max').properties['min-width'], '80px');
  assert.equal(nodes.get('min-over-max').properties['max-width'], '50px');
  assert.equal(nodes.get('default-auto-none').properties['min-width'], '0px');
  assert.equal(nodes.get('default-auto-none').properties['max-width'], 'none');
  assert.equal(nodes.get('invalid-min-fallback').properties['min-width'], '25px');
  assert.equal(nodes.get('invalid-max-fallback').properties['max-width'], '100px');
});

test('고정 기준은 flex shrink·grow에서 각 최소·최대 제약을 적용한다', () => {
  const nodes = nodeMap(observations[0]);
  assert.deepEqual(
    [nodes.get('flex-min-a').rect.width, nodes.get('flex-min-b').rect.width],
    [80, 80],
  );
  assert.deepEqual(
    [nodes.get('flex-zero-a').rect.width, nodes.get('flex-zero-b').rect.width],
    [75, 75],
  );
  assert.deepEqual(
    [nodes.get('flex-max-a').rect.width, nodes.get('flex-max-b').rect.width],
    [60, 60],
  );
  assert.deepEqual(
    [nodes.get('flex-max-a').rect.x, nodes.get('flex-max-b').rect.x],
    [0, 140],
  );
  assert.equal(nodes.get('flex-auto-a').properties['min-width'], 'auto');
  assert.equal(nodes.get('flex-auto-a').rect.width, 50);
});

test('최소·최대 크기 computed 값과 CSS px geometry는 DPR에 따라 변하지 않는다', () => {
  assert.equal(observations[0].viewport.deviceScaleFactor, 1);
  assert.equal(observations[1].viewport.deviceScaleFactor, 2);
  assert.deepEqual(observations[0].nodes, observations[1].nodes);
  assert.deepEqual(reference.comparison.rectFields, ['x', 'y', 'width', 'height']);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('C07.1 실제 runtime fixture가 V8 문서와 author stylesheet를 사용한다', async () => {
  const source = await readSource('tests/fixtures/css/c07/runtime-min-max-sizing.js');
  assert.match(source, /document\.createElement\("style"\)/);
  assert.match(source, /min-width: 72px/);
  assert.match(source, /max-width: 70px/);
  assert.match(source, /max-width: 50px/);
  assert.match(source, /min-width: 25%/);
  assert.match(source, /max-width: 55px/);
});

test('C07.1 runtime 연결이 Rust FFI부터 Android·iOS launch route까지 이어진다', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/ffi/c07.rs',
    ffiRegistry: 'crates/spinon-ffi/src/runtime_gpu/ffi.rs',
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
    async ([name, path]) => [name, await readSource(path)],
  )));

  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_min_max_sizing_fixture/);
  assert.match(source.ffiRegistry, /mod c07;/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_min_max_sizing_fixture/);
  assert.match(source.androidJni, /nativeEvalMinMaxSizingFixture[\s\S]*SPINON_C071_EVAL/);
  assert.match(source.androidDemo, /nativeEvalMinMaxSizingFixture[\s\S]*spinon_c071_min_max_sizing/);
  assert.match(source.androidActivity, /spinon_c071_min_max_sizing/);
  assert.match(source.iosRunner, /evalRuntimeGpuMinMaxSizingFixture[\s\S]*SPINON_C071_EVAL/);
  assert.match(source.iosHeader, /evalRuntimeGpuMinMaxSizingFixture/);
  assert.match(source.iosDemo, /--spinon-c071-min-max-sizing[\s\S]*evalRuntimeGpuMinMaxSizingFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c071-min-max-sizing/);
});
