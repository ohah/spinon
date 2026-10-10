import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const referencePath = 'tests/fixtures/css/references/c07-3-aspect-ratio-v1.json';
const inventoryPath = 'tests/fixtures/css/c07/aspect-ratio-inventory.json';
const reference = JSON.parse(await readFile(join(repositoryRoot, referencePath), 'utf8'));
const inventory = JSON.parse(await readFile(join(repositoryRoot, inventoryPath), 'utf8'));
const observations = reference.observations;
const readSource = (path) => readFile(join(repositoryRoot, path), 'utf8');
const nodeMap = (observation) => new Map(observation.nodes.map((node) => [node.id, node]));
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('고정 Chrome reference와 원본 입력 digest가 일치한다', async () => {
  assert.equal(reference.schema, 'spinon-css-c07-3-aspect-ratio-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256, digest(await readSource(inventoryPath)));
  assert.equal(reference.fixture.htmlSha256, digest(await readSource(reference.fixture.htmlPath)));
  assert.equal(reference.captureTool.sha256, digest(await readSource(reference.captureTool.path)));
  assert.equal(reference.captureTool.dependencies.length, 1);
  assert.equal(
    reference.captureTool.dependencies[0].sha256,
    digest(await readSource(reference.captureTool.dependencies[0].path)),
  );
  assert.equal(reference.oracle.executableSha256.length, 64);
});

test('두 DPR 관찰이 viewport·환경·35개 고유 node에서 일치한다', () => {
  assert.equal(inventory.schema, 'spinon-css-c07-3-aspect-ratio-inventory/v1');
  assert.equal(inventory.nodes.length, 35);
  assert.equal(new Set(inventory.nodes.map(({ id }) => id)).size, 35);
  assert.deepEqual(inventory.comparison.computedStyleMapPropertiesMustMatch, ['aspect-ratio', 'box-sizing']);
  assert.equal(inventory.comparison.getComputedStyleResolvedValuesAreDiagnosticOnly, true);
  assert.equal(inventory.nodes.filter(({ layoutSupport }) => layoutSupport === 'unsupported-min-max-ratio').length, 5);
  assert.deepEqual(inventory.viewport, { width: 400, height: 1200, deviceScaleFactors: [1, 2] });
  assert.equal(observations.length, 2);
  for (const [index, observation] of observations.entries()) {
    assert.equal(observation.viewport.deviceScaleFactor, index + 1);
    assert.equal(observation.nodes.length, 35);
    assert.deepEqual(observation.nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));
    assert.equal(observation.environment.locale, 'en-US');
    assert.equal(observation.environment.timeZone, 'UTC');
    assert.equal(observation.environment.dark, false);
    assert.equal(observation.environment.coarsePointer, true);
    assert.equal(observation.environment.hover, false);
    for (const node of observation.nodes) {
      assert.ok(['x', 'y', 'width', 'height'].every((field) => Number.isFinite(node.rect[field])), node.id);
    }
  }
  assert.deepEqual(observations[0].nodes, observations[1].nodes);
  assert.deepEqual(reference.comparison.rectFields, ['x', 'y', 'width', 'height']);
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
});

test('computed ratio의 width/height 방향과 CSS number 계산을 고정한다', () => {
  const nodes = nodeMap(observations[0]);
  for (const [id, rect] of [
    ['ratio-16-9-width', [160, 90]],
    ['ratio-one-width', [100, 100]],
    ['ratio-four-three-width', [120, 90]],
    ['ratio-empty-auto-sizes', [400, 225]],
    ['ratio-calc-number', [160, 90]],
  ]) {
    const actual = nodes.get(id);
    assert.ok(actual, id);
    assert.deepEqual([actual.rect.width, actual.rect.height], rect, id);
    assert.equal(actual.properties['aspect-ratio'].includes('/'), true, id);
  }
  assert.deepEqual(
    [nodes.get('ratio-definite-both').rect.width, nodes.get('ratio-definite-both').rect.height],
    [160, 80],
  );
});

test('box-sizing·padding·border·min/max 조합의 실제 outer rect를 고정한다', () => {
  const nodes = nodeMap(observations[0]);
  for (const [id, rect] of [
    ['ratio-content-box', [120, 70]],
    ['ratio-border-box', [120, 60]],
    ['ratio-border-box-border', [120, 60]],
    ['ratio-content-box-border', [124, 74]],
    ['ratio-min-height-transfer', [100, 80]],
    ['ratio-max-height-transfer', [160, 60]],
    ['ratio-min-width-transfer', [100, 40]],
    ['ratio-max-width-transfer', [120, 80]],
  ]) {
    const actual = nodes.get(id);
    assert.ok(actual, id);
    assert.deepEqual([actual.rect.width, actual.rect.height], rect, id);
  }
});

test('degenerate ratio, variable substitution, invalid fallback, inheritance와 auto 기본값을 고정한다', () => {
  const nodes = nodeMap(observations[0]);
  for (const id of ['ratio-zero-numerator', 'ratio-zero-denominator', 'ratio-zero-both', 'ratio-auto-explicit', 'ratio-auto-initial']) {
    assert.deepEqual([nodes.get(id).rect.width, nodes.get(id).rect.height], [100, 0], id);
  }
  assert.equal(nodes.get('ratio-var').properties['aspect-ratio'], '4 / 3');
  assert.deepEqual([nodes.get('ratio-var').rect.width, nodes.get('ratio-var').rect.height], [120, 90]);
  assert.equal(nodes.get('ratio-invalid-fallback').properties['aspect-ratio'], '2 / 1');
  assert.deepEqual([nodes.get('ratio-invalid-fallback').rect.width, nodes.get('ratio-invalid-fallback').rect.height], [100, 50]);
  assert.equal(nodes.get('ratio-not-inherited').properties['aspect-ratio'], 'auto');
  assert.deepEqual([nodes.get('ratio-not-inherited').rect.width, nodes.get('ratio-not-inherited').rect.height], [100, 0]);
  assert.deepEqual([nodes.get('ratio-display-none').rect.width, nodes.get('ratio-display-none').rect.height], [0, 0]);
});

test('Flex stretch와 start 정렬에서 ratio 적용 차이를 보존한다', () => {
  const nodes = nodeMap(observations[0]);
  assert.deepEqual(
    [nodes.get('ratio-flex-row-child').rect.width, nodes.get('ratio-flex-row-child').rect.height],
    [100, 100],
  );
  assert.deepEqual(
    [nodes.get('ratio-flex-row-start-child').rect.width, nodes.get('ratio-flex-row-start-child').rect.height],
    [100, 50],
  );
  assert.deepEqual(
    [nodes.get('ratio-flex-column-child').rect.width, nodes.get('ratio-flex-column-child').rect.height],
    [100, 100],
  );
  assert.deepEqual(
    [nodes.get('ratio-flex-column-start-child').rect.width, nodes.get('ratio-flex-column-start-child').rect.height],
    [200, 100],
  );
  assert.deepEqual(
    [nodes.get('ratio-flex-min-max-child').rect.width, nodes.get('ratio-flex-min-max-child').rect.height],
    [120, 80],
  );
});

test('capture 도구의 node count와 고정 reference 덮어쓰기 방지를 유지한다', async () => {
  const source = await readSource('tools/css-reference/capture-c07-3-aspect-ratio.mjs');
  assert.match(source, /inventory\.nodes\.length !== 35/);
  assert.match(source, /기준 reference는 덮어쓰지 않습니다/);
  assert.match(source, /flag: 'wx'/);
});

test('C07.3 fixture가 Android·iOS의 실제 V8·WGPU runtime 경로에 연결되어 있다', async () => {
  const paths = {
    fixture: 'tests/fixtures/css/c07/runtime-aspect-ratio.js',
    ffi: 'crates/spinon-ffi/src/runtime_gpu/ffi/c07.rs',
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
    async ([name, path]) => [name, await readSource(path)],
  )));

  assert.match(source.fixture, /aspect-ratio: 16 \/ 9/);
  assert.match(source.fixture, /aspect-ratio: 1;/);
  assert.match(source.fixture, /aspect-ratio: 3 \/ 4/);
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_aspect_ratio_fixture/);
  assert.match(source.ffiSource, /runtime-aspect-ratio\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_aspect_ratio_fixture/);
  assert.match(source.androidJni, /nativeEvalAspectRatioFixture[\s\S]*SPINON_C073_EVAL/);
  assert.match(source.androidDemo, /nativeEvalAspectRatioFixture[\s\S]*spinon_c073_aspect_ratio/);
  assert.match(source.androidActivity, /spinon_c073_aspect_ratio/);
  assert.match(source.iosRunner, /evalRuntimeGpuAspectRatioFixture[\s\S]*SPINON_C073_EVAL/);
  assert.match(source.iosHeader, /evalRuntimeGpuAspectRatioFixture/);
  assert.match(source.iosDemo, /--spinon-c073-aspect-ratio[\s\S]*evalRuntimeGpuAspectRatioFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c073-aspect-ratio/);
});
