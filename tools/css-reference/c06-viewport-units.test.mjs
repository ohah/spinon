import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventory = JSON.parse(await readFile(join(repositoryRoot,
  'tests/fixtures/css/c06/viewport-units-inventory.json'), 'utf8'));
const reference = JSON.parse(await readFile(join(repositoryRoot,
  'tests/fixtures/css/references/c06-viewport-units-v1.json'), 'utf8'));
const byId = (observation) => new Map(observation.nodes.map((node) => [node.id, node]));
const px = (value) => Number.parseFloat(value);

test('pinned Chromium viewport-unit fixture covers every viewport and DPR', () => {
  assert.equal(reference.schema, 'spinon-css-c06-viewport-units-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.fixture.inventorySha256.length, 64);
  assert.equal(reference.fixture.htmlSha256.length, 64);
  assert.equal(reference.captureTool.sha256.length, 64);
  assert.equal(reference.observations.length, inventory.viewports.length * 2);
  assert.deepEqual(reference.comparison.unsupportedUnits,
    ['cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']);
});

test('default, small, large, and dynamic units resolve to the single native content viewport', () => {
  for (let viewportIndex = 0; viewportIndex < inventory.viewports.length; viewportIndex += 1) {
    const viewport = inventory.viewports[viewportIndex];
    const observation = reference.observations[viewportIndex * 2];
    const nodes = byId(observation);
    const width = viewport.width;
    const height = viewport.height;
    const expected = new Map([
      ['root', [width, height]],
      ['default-units', [width * 0.5, height * 0.05]],
      ['small-units', [width * 0.26 + Math.min(width, height) * 0.01,
        height * 0.06 + Math.max(width, height) * 0.01]],
      ['large-units', [width * 0.26 + Math.min(width, height) * 0.01,
        height * 0.06 + Math.max(width, height) * 0.01]],
      ['dynamic-units', [width * 0.26 + Math.min(width, height) * 0.01,
        height * 0.06 + Math.max(width, height) * 0.01]],
      ['logical-units', [width * 0.1, height * 0.05]],
      ['minmax-units', [Math.min(width, height) * 0.1, Math.max(width, height) * 0.05]],
      ['math-units', [width * 0.1 + 10, Math.min(height * 0.05, 45)]],
      ['var-units', [width * 0.12, 6]],
      ['gap-row', [width * 0.5, 6]],
      ['gap-a', [width * 0.1, 6]],
      ['basis-row', [width * 0.5, 6]],
      ['basis-item', [width * 0.25, 6]],
    ]);

    for (const [id, [expectedWidth, expectedHeight]] of expected) {
      const actual = nodes.get(id);
      assert.ok(actual, id);
      assert.ok(Math.abs(px(actual.properties.width) - expectedWidth) <= 0.02,
        `${id}.width: ${actual.properties.width} != ${expectedWidth}px`);
      assert.ok(Math.abs(px(actual.properties.height) - expectedHeight) <= 0.02,
        `${id}.height: ${actual.properties.height} != ${expectedHeight}px`);
    }

    assert.ok(Math.abs(px(nodes.get('spacing-units').properties['margin-left']) - width * 0.05) <= 0.02);
    assert.ok(Math.abs(px(nodes.get('spacing-units').properties['padding-left']) - width * 0.02) <= 0.02);
    assert.ok(Math.abs(px(nodes.get('gap-row').properties['column-gap']) - width * 0.05) <= 0.02);
    assert.ok(Math.abs(px(nodes.get('basis-item').properties['flex-basis']) - width * 0.25) <= 0.02);

    for (const property of ['width', 'height']) {
      assert.equal(nodes.get('small-units').properties[property], nodes.get('large-units').properties[property]);
      assert.equal(nodes.get('large-units').properties[property], nodes.get('dynamic-units').properties[property]);
    }
  }
});

test('viewport-unit CSS values and CSS geometry do not change when only DPR changes', () => {
  for (let viewportIndex = 0; viewportIndex < inventory.viewports.length; viewportIndex += 1) {
    const one = reference.observations[viewportIndex * 2];
    const two = reference.observations[viewportIndex * 2 + 1];
    assert.equal(one.viewport.deviceScaleFactor, 1);
    assert.equal(two.viewport.deviceScaleFactor, 2);
    assert.deepEqual(one.nodes, two.nodes);
  }
});

test('runtime viewport-unit fixture is connected through the Android and iOS host boundaries', async () => {
  const paths = {
    runtimeFixture: 'tests/fixtures/css/c06/runtime-viewport-units.js',
    ffiHeader: 'crates/spinon-ffi/include/spinon_ffi.h',
    ffiRuntime: 'crates/spinon-ffi/src/runtime_gpu.rs',
    ffiFunction: 'crates/spinon-ffi/src/runtime_gpu/ffi/c06.rs',
    androidDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    androidJni: 'platforms/android/app/src/main/cpp/spinon_jni.cc',
    androidActivity: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java',
    iosRunnerHeader: 'platforms/ios/Sources/SpinonRunner.h',
    iosRunner: 'platforms/ios/Sources/SpinonRunner.mm',
    iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    iosAppDelegate: 'platforms/ios/Sources/AppDelegate.swift',
  };
  const files = await Promise.all(Object.values(paths).map((path) =>
    readFile(join(repositoryRoot, path), 'utf8')));
  const source = Object.fromEntries(Object.keys(paths).map((key, index) => [key, files[index]]));

  for (const selector of [
    '#default-units', '#small-units', '#large-units', '#dynamic-units', '#logical-units',
    '#minmax-units', '#math-units', '#var-units', '#spacing-units', '#gap-row', '#basis-row',
  ]) {
    assert.ok(source.runtimeFixture.includes(selector), 'runtime fixture must include ' + selector);
  }
  assert.match(source.ffiRuntime, /RUNTIME_CSS_VIEWPORT_UNITS_FIXTURE_SOURCE/);
  assert.match(source.ffiFunction, /spinon_runtime_gpu_host_eval_viewport_units_fixture/);
  assert.match(source.ffiHeader, /spinon_runtime_gpu_host_eval_viewport_units_fixture/);
  assert.match(source.androidDemo, /nativeEvalViewportUnitsFixture[\s\S]*SPINON_C066A_EVAL/);
  assert.match(source.androidJni, /nativeEvalViewportUnitsFixture[\s\S]*SPINON_C066A_EVAL/);
  assert.match(source.androidActivity, /spinon_c066_viewport_units/);
  assert.match(source.iosRunnerHeader, /evalRuntimeGpuViewportUnitsFixture/);
  assert.match(source.iosRunner, /evalRuntimeGpuViewportUnitsFixture[\s\S]*SPINON_C066A_EVAL/);
  assert.match(source.iosDemo, /--spinon-c066-viewport-units[\s\S]*evalRuntimeGpuViewportUnitsFixture/);
  assert.match(source.iosAppDelegate, /--spinon-c066-viewport-units/);
});
