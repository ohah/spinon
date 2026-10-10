import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const referencePath = 'tests/fixtures/css/references/c07-2-border-width-layout-v1.json';
const highDprReferencePath = 'tests/fixtures/css/references/c07-2-border-width-high-dpr-v1.json';
const inventoryPath = 'tests/fixtures/css/c07/border-width-layout-inventory.json';
const reference = JSON.parse(await readFile(join(repositoryRoot, referencePath), 'utf8'));
const highDprReference = JSON.parse(await readFile(join(repositoryRoot, highDprReferencePath), 'utf8'));
const inventory = JSON.parse(await readFile(join(repositoryRoot, inventoryPath), 'utf8'));
const observations = reference.observations;
const readSource = (path) => readFile(join(repositoryRoot, path), 'utf8');
const nodeMap = (observation) => new Map(observation.nodes.map((node) => [node.id, node]));
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('고정 Chrome 기준의 원본 입력과 도구 digest가 일치한다', async () => {
  assert.equal(reference.schema, 'spinon-css-c07-2-border-width-layout-reference/v1');
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

test('고 DPR 정수 경계와 원문 viewport 단위의 CSS px 결과를 고정한다', async () => {
  assert.equal(highDprReference.schema, 'spinon-css-c07-2-border-width-boundary/v1');
  assert.equal(highDprReference.fixtureId, 'C07.2-border-width-high-dpr-v1');
  assert.equal(highDprReference.browserVersion.product, 'Chrome/154.0.8037.98');
  assert.equal(highDprReference.browserVersion.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(highDprReference.environment.deviceScaleFactors, [1, 2, 2.625, 3]);
  assert.equal(highDprReference.observations.length, 4);
  assert.equal(highDprReference.captureTool.sha256,
    digest(await readSource(highDprReference.captureTool.path)));
  assert.equal(highDprReference.captureTool.dependencies.length, 1);
  assert.equal(highDprReference.captureTool.dependencies[0].sha256,
    digest(await readSource(highDprReference.captureTool.dependencies[0].path)));

  for (const observation of highDprReference.observations) {
    assert.deepEqual(observation.widths.map(({ computedCssPx, borderBoxWidthCssPx }) => (
      [computedCssPx, borderBoxWidthCssPx]
    )), [['1px', 102], ['1px', 102], ['2px', 104], ['2px', 104]]);
    assert.deepEqual(observation.relativeWidths.map(({ unit, computedCssPx, borderBoxWidthCssPx }) => (
      [unit, computedCssPx, borderBoxWidthCssPx]
    )), [
      ['em', '2px', 104],
      ['vw', '2px', 104],
      ['vh', '2px', 104],
      ['var-vh', '2px', 104],
    ]);
  }
});

test('두 DPR 관찰은 고정 viewport·환경·50개 고유 node를 가진다', () => {
  assert.equal(inventory.schema, 'spinon-css-c07-2-border-width-inventory/v1');
  assert.equal(inventory.nodes.length, 50);
  assert.equal(new Set(inventory.nodes.map(({ id }) => id)).size, 50);
  assert.equal(observations.length, 2);
  assert.deepEqual(inventory.viewport, { width: 400, height: 1000, deviceScaleFactors: [1, 2] });
  assert.equal(observations[0].viewport.deviceScaleFactor, 1);
  assert.equal(observations[1].viewport.deviceScaleFactor, 2);
  for (const observation of observations) {
    assert.equal(observation.nodes.length, 50);
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

test('shorthand side mapping과 longhand cascade 승자를 보존한다', () => {
  const nodes = nodeMap(observations[0]);
  const sides = (id) => {
    const properties = nodes.get(id).properties;
    return ['top', 'right', 'bottom', 'left'].map((side) => properties[`border-${side}-width`]);
  };
  assert.deepEqual(sides('explicit-one-value'), ['2px', '2px', '2px', '2px']);
  assert.deepEqual(sides('shorthand-two'), ['2px', '4px', '2px', '4px']);
  assert.deepEqual(sides('shorthand-three'), ['1px', '2px', '3px', '2px']);
  assert.deepEqual(sides('shorthand-four'), ['1px', '2px', '3px', '4px']);
  assert.deepEqual(sides('side-overrides'), ['4px', '5px', '6px', '7px']);
  assert.deepEqual(sides('longhand-cascade'), ['1px', '2px', '5px', '7px']);
  assert.deepEqual(sides('border-shorthand-reset'), ['3px', '3px', '3px', '3px']);
  assert.equal(nodes.get('side-overrides').rect.width, 112);
  assert.equal(nodes.get('longhand-cascade').rect.width, 109);
  assert.deepEqual(
    [nodes.get('border-shorthand-reset').rect.width, nodes.get('border-shorthand-reset').rect.height],
    [106, 36],
  );
});

test('none·hidden과 모든 비-none 선 스타일의 geometry 기여를 구분한다', () => {
  const nodes = nodeMap(observations[0]);
  for (const id of ['none-zero', 'hidden-zero']) {
    const node = nodes.get(id);
    assert.deepEqual(
      ['top', 'right', 'bottom', 'left'].map((side) => node.properties[`border-${side}-width`]),
      ['0px', '0px', '0px', '0px'],
    );
    assert.deepEqual([node.rect.width, node.rect.height], [100, 30]);
  }
  assert.deepEqual(
    [nodes.get('default-medium-solid').rect.width, nodes.get('default-medium-solid').rect.height],
    [106, 36],
  );
  for (const id of [
    'dashed-contributes', 'dotted-contributes', 'double-contributes', 'groove-contributes',
    'ridge-contributes', 'inset-contributes', 'outset-contributes',
  ]) {
    assert.deepEqual([nodes.get(id).rect.width, nodes.get(id).rect.height], [108, 38], id);
  }
  assert.deepEqual([nodes.get('transparent-solid').rect.width, nodes.get('transparent-solid').rect.height], [106, 36]);
  assert.equal(nodes.get('transparent-solid').properties['border-top-color'], 'rgba(0, 0, 0, 0)');
});

test('content-box·border-box에서 padding·border·min/max가 서로 다르게 포함된다', () => {
  const nodes = nodeMap(observations[0]);
  assert.deepEqual([nodes.get('content-box').rect.width, nodes.get('content-box').rect.height], [124, 64]);
  assert.deepEqual([nodes.get('border-box').rect.width, nodes.get('border-box').rect.height], [100, 40]);
  assert.deepEqual([nodes.get('border-box-padding-floor').rect.width, nodes.get('border-box-padding-floor').rect.height], [28, 28]);
  assert.equal(nodes.get('border-box-min').rect.width, 80);
  assert.equal(nodes.get('border-box-max').rect.width, 80);
  assert.equal(nodes.get('content-box-min-border').rect.width, 94);
  assert.equal(nodes.get('content-box-max-border').rect.width, 94);
});

test('fractional 폭의 pinned Chrome snapping, zero 및 invalid 선언 fallback을 고정한다', () => {
  const nodes = nodeMap(observations[0]);
  const expectedWidths = [
    ['fractional-025-all', '1px'], ['fractional-050-all', '1px'], ['fractional-075-all', '1px'],
    ['fractional-125-all', '1px'], ['fractional-150-all', '1px'], ['fractional-175-all', '1px'],
    ['fractional-225-all', '2px'], ['fractional-250-all', '2px'], ['fractional-275-all', '2px'],
    ['fractional-350-all', '3px'], ['zero-width', '0px'],
  ];
  for (const [id, width] of expectedWidths) assert.equal(nodes.get(id).properties['border-top-width'], width, id);
  assert.deepEqual(
    ['invalid-negative-fallback', 'invalid-percent-fallback'].map((id) => nodes.get(id).properties['border-top-width']),
    ['4px', '4px'],
  );
  assert.deepEqual(
    ['invalid-negative-fallback', 'invalid-percent-fallback'].map((id) => nodes.get(id).rect.width),
    [108, 108],
  );
});

test('현재 C06 length math, flex shrink 외곽 크기, outline 제외를 고정한다', () => {
  const nodes = nodeMap(observations[0]);
  assert.deepEqual(
    ['thin-solid', 'medium-solid', 'thick-solid'].map((id) => nodes.get(id).properties['border-top-width']),
    ['1px', '3px', '5px'],
  );
  assert.equal(nodes.get('calc-width').properties['border-top-width'], '2px');
  assert.deepEqual(
    ['calc-width', 'calc-asymmetric', 'var-width'].map((id) => nodes.get(id).typed['border-top-width']),
    [
      { type: 'CSSUnitValue', text: '2px' },
      { type: 'CSSUnitValue', text: '1px' },
      { type: 'CSSUnitValue', text: '3px' },
    ],
  );
  assert.deepEqual(
    ['top', 'right', 'bottom', 'left'].map((side) => nodes.get('calc-asymmetric').properties[`border-${side}-width`]),
    ['1px', '2px', '3px', '4px'],
  );
  assert.deepEqual(
    [nodes.get('calc-asymmetric').rect.width, nodes.get('calc-asymmetric').rect.height],
    [106, 34],
  );
  assert.equal(nodes.get('var-width').properties['border-top-width'], '3px');
  assert.deepEqual(
    [nodes.get('flex-shrink-border').rect.width, nodes.get('flex-shrink-border').properties.width,
      nodes.get('flex-shrink-sibling').rect.x, nodes.get('flex-shrink-sibling').rect.width],
    [65, '55px', 65, 55],
  );
  assert.deepEqual([nodes.get('outline-excluded').rect.width, nodes.get('outline-excluded').rect.height], [80, 30]);
});

test('캡처 도구는 고정 reference를 자동으로 덮어쓰지 않는다', async () => {
  const source = await readSource('tools/css-reference/capture-c07-2-border-width-layout.mjs');
  assert.match(source, /기준 reference는 덮어쓰지 않습니다/);
  assert.match(source, /flag: 'wx'/);
  assert.match(source, /inventory\.nodes\.length !== 50/);
});

test('C07.2 V8 fixture가 실제 Android·iOS WGPU 런타임 경로에 연결되어 있다', async () => {
  const paths = {
    fixture: 'tests/fixtures/css/c07/runtime-border-width.js',
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

  assert.match(source.fixture, /border: 3px solid red/);
  assert.match(source.fixture, /border-width: 3px 5px/);
  assert.match(source.fixture, /border-style: none/);
  assert.match(source.ffi, /spinon_runtime_gpu_host_eval_border_width_fixture/);
  assert.match(source.ffiSource, /runtime-border-width\.js/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_border_width_fixture/);
  assert.match(source.androidJni, /nativeEvalBorderWidthFixture[\s\S]*SPINON_C072_EVAL/);
  assert.match(source.androidDemo, /nativeEvalBorderWidthFixture[\s\S]*spinon_c072_border_width/);
  assert.match(source.androidActivity, /spinon_c072_border_width/);
  assert.match(source.iosRunner, /evalRuntimeGpuBorderWidthFixture[\s\S]*SPINON_C072_EVAL/);
  assert.match(source.iosHeader, /evalRuntimeGpuBorderWidthFixture/);
  assert.match(source.iosDemo, /--spinon-c072-border-width[\s\S]*evalRuntimeGpuBorderWidthFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c072-border-width/);
});
