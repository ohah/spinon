import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  reference: 'tests/fixtures/css/references/c06-typed-css-math-v1.json',
  inventory: 'tests/fixtures/css/c06/typed-css-math-inventory.json',
  html: 'tests/fixtures/css/c06/typed-css-math.html',
  runtimeFixture: 'tests/fixtures/css/c06/runtime-typed-css-math.js',
  capture: 'tools/css-reference/capture-c06-typed-css-math.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  ffiHeader: 'crates/spinon-ffi/include/spinon_ffi.h',
  androidDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  androidJni: 'platforms/android/app/src/main/cpp/spinon_jni.cc',
  iosRunner: 'platforms/ios/Sources/SpinonRunner.mm',
  iosAppDelegate: 'platforms/ios/Sources/AppDelegate.swift',
  iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
};
const read = async (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [reference, inventory] = await Promise.all([
  read(paths.reference).then((bytes) => JSON.parse(bytes.toString('utf8'))),
  read(paths.inventory).then((bytes) => JSON.parse(bytes.toString('utf8'))),
]);
const nodes = new Map(reference.observations[0].nodes.map((node) => [node.id, node]));

test('C06.5 Chromium oracle is pinned to exact executable, source files and DPR pair', async () => {
  assert.equal(inventory.schema, 'spinon-css-c06-typed-css-math-inventory/v1');
  assert.equal(inventory.fixtureId, 'C06.5-typed-css-math-v1');
  assert.equal(reference.schema, 'spinon-css-c06-typed-css-math-reference/v1');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.observations.length, 2);
  assert.deepEqual(reference.observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  assert.deepEqual(reference.observations[0].nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));

  for (const [path, digest] of [
    [paths.inventory, reference.fixture.inventorySha256],
    [paths.html, reference.fixture.htmlSha256],
    [paths.capture, reference.captureTool.sha256],
    [paths.helper, reference.captureTool.dependencies[0].sha256],
  ]) {
    assert.equal(sha256(await read(path)), digest, path + ' hash');
  }
});

test('Chromium resolves calc, min, max, clamp and var values against the correct width basis', () => {
  assert.equal(nodes.get('calc-length').properties.width, '45px');
  assert.equal(nodes.get('calc-mixed').properties.width, '85px');
  assert.equal(nodes.get('calc-nested').properties.width, '60px');
  assert.equal(nodes.get('calc-product').properties.width, '35px');
  assert.equal(nodes.get('min-value').properties.width, '100px');
  assert.equal(nodes.get('max-value').properties.width, '150px');
  assert.equal(nodes.get('min-mixed').properties.width, '80px');
  assert.equal(nodes.get('max-nested').properties.width, '60px');
  assert.equal(nodes.get('clamp-value').properties.width, '100px');
  assert.equal(nodes.get('clamp-negative').properties.width, '20px');
  assert.equal(nodes.get('clamp-inverted-bounds').properties.width, '100px');
  assert.equal(nodes.get('var-math').properties.width, '50px');
});

test('Typed OM preserves mixed math nodes, while used CSS values preserve layout geometry', () => {
  assert.equal(nodes.get('calc-mixed').typed.width.type, 'CSSMathSum');
  assert.equal(nodes.get('min-value').typed.width.type, 'CSSMathMin');
  assert.equal(nodes.get('max-value').typed.width.type, 'CSSMathMax');
  assert.equal(nodes.get('clamp-value').typed.width.type, 'CSSMathClamp');
  assert.equal(nodes.get('var-math').typed.width.type, 'CSSMathSum');
  assert.equal(nodes.get('gap-container').typed['column-gap'].type, 'CSSMathSum');
  assert.equal(nodes.get('basis-math').typed['flex-basis'].type, 'CSSMathSum');
  assert.equal(nodes.get('gap-second').rect.x, 50);
  assert.equal(nodes.get('basis-math').rect.width, 70);
});

test('CSS bounds, margin sign and non-finite calculations match pinned Chromium observations', () => {
  assert.equal(nodes.get('negative-margin').properties['margin-left'], '-5px');
  assert.equal(nodes.get('negative-margin').rect.x, -5);
  assert.equal(nodes.get('padding-math').properties['padding-left'], '15px');
  assert.equal(nodes.get('divide-by-zero').rect.width, 33554428);
  assert.ok(Number.isFinite(nodes.get('divide-by-zero').rect.width));
  assert.equal(nodes.get('zero-divided-by-zero').properties.width, '0px');
  assert.equal(nodes.get('clamp-nan').properties.width, '0px');
  assert.equal(nodes.get('negative-size').properties.width, '0px');
});

test('invalid dimensions use the prior valid declaration and Chromium round() is outside this slice', () => {
  assert.equal(nodes.get('invalid-dimension').properties.width, '23px');
  assert.equal(nodes.get('round-function').properties.width, '12px');
});

test('CSS computed values and geometry are invariant across the captured device scale factors', () => {
  const one = reference.observations[0];
  const two = reference.observations[1];
  for (let index = 0; index < inventory.nodes.length; index += 1) {
    assert.deepEqual(one.nodes[index].properties, two.nodes[index].properties, one.nodes[index].id);
    assert.deepEqual(one.nodes[index].rect, two.nodes[index].rect, one.nodes[index].id);
  }
});

test('C06.5 runtime fixture reaches both native simulator hosts', async () => {
  const [runtimeFixture, ffiHeader, androidDemo, androidJni, iosRunner, iosAppDelegate, iosDemo] = await Promise.all([
    read(paths.runtimeFixture),
    read(paths.ffiHeader),
    read(paths.androidDemo),
    read(paths.androidJni),
    read(paths.iosRunner),
    read(paths.iosAppDelegate),
    read(paths.iosDemo),
  ]).then((files) => files.map((bytes) => bytes.toString('utf8')));

  for (const selector of [
    '#calc-mixed', '#min-value', '#max-value', '#clamp-value', '#clamp-nan',
    '#negative-margin', '#gap-row', '#basis-math',
  ]) {
    assert.ok(runtimeFixture.includes(selector), 'runtime CSS fixture must include ' + selector);
  }
  assert.match(ffiHeader, /spinon_runtime_gpu_host_eval_typed_css_math_fixture/);
  assert.match(androidDemo, /nativeEvalTypedCssMathFixture/);
  assert.match(androidJni, /nativeEvalTypedCssMathFixture[\s\S]*SPINON_C065_EVAL/);
  assert.match(iosRunner, /evalRuntimeGpuTypedCssMathFixture[\s\S]*SPINON_C065_EVAL/);
  assert.match(iosAppDelegate, /--spinon-c065-typed-css-math/);
  assert.match(iosDemo, /--spinon-c065-typed-css-math/);
  assert.match(iosDemo, /evalRuntimeGpuTypedCssMathFixture/);
});
