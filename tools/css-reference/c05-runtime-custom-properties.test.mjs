import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  input: 'tests/fixtures/css/c05/runtime-custom-properties.v1.json',
  html: 'tests/fixtures/css/c05/runtime-custom-properties.html',
  appScript: 'tests/fixtures/css/c05/runtime-custom-properties-app.js',
  capture: 'tools/css-reference/capture-c05-runtime-custom-properties.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c05-runtime-custom-properties-v1.json',
};

async function bytes(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

test('C05 custom-property Chromium oracle is tied to immutable fixture and capture inputs', async () => {
  const [inputBytes, htmlBytes, appScriptBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    bytes(paths.input), bytes(paths.html), bytes(paths.appScript), bytes(paths.capture),
    bytes(paths.helper), bytes(paths.reference),
  ]);
  const input = JSON.parse(inputBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(input.schema, 'spinon-css-c05-runtime-custom-properties-input/v1');
  assert.equal(reference.schema, 'spinon-css-c05-runtime-custom-properties-reference/v1');
  assert.equal(reference.fixtureId, input.fixtureId);
  assert.equal(reference.fixture.inputPath, paths.input);
  assert.equal(reference.fixture.inputSha256, sha256(inputBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.fixture.appJavascriptPath, paths.appScript);
  assert.equal(reference.fixture.appJavascriptSha256, sha256(appScriptBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Google Chrome');
  assert.match(reference.oracle.product, /^Chrome\/\d+\.\d+\.\d+\.\d+$/);
  assert.match(reference.oracle.revision, /^@[0-9a-f]+$/);
  assert.deepEqual(reference.environment.viewportCssPx, {
    width: input.viewport.widthCssPx,
    height: input.viewport.heightCssPx,
  });
  assert.deepEqual(reference.comparison, input.comparison);
  assert.deepEqual(Object.keys(reference.observations.initial), input.nodes);
  assert.deepEqual(Object.keys(reference.observations), [
    'initial', 'ancestorValueUpdated', 'movedToDifferentParent',
  ]);

  const initial = reference.observations.initial;
  assert.equal(initial['inherit-child'].width, '41px');
  assert.equal(initial['case-child'].width, '23px');
  assert.equal(initial['fallback-child'].width, '13px');
  assert.equal(initial['keyword-inherit'].width, '41px');
  assert.equal(initial['keyword-unset'].width, '41px');
  assert.equal(initial['keyword-initial'].width, '23px');
  assert.equal(initial['keyword-revert'].width, '41px');
  assert.equal(initial.cycle.width, '37px');
  assert.equal(initial.cycle.marginLeft, '7px');
  assert.equal(initial.invalid.marginLeft, '0px');
  assert.equal(initial.empty.marginLeft, '0px');
  assert.equal(initial.important.width, '19px');
  assert.equal(initial['gap-parent'].rowGap, '11px');
  assert.equal(initial['gap-parent'].columnGap, '11px');
  assert.equal(initial['gap-parent'].backgroundColor, 'rgb(18, 52, 86)');
  assert.equal(initial['gap-first'].backgroundColor, 'rgb(51, 102, 255)');
  assert.equal(initial['gap-second'].backgroundColor, 'rgb(255, 204, 51)');
  assert.equal(initial['gap-second'].rect.x - initial['gap-first'].rect.x, 31);
  assert.equal(reference.observations.ancestorValueUpdated['moving-target'].width, '73px');
  assert.equal(reference.observations.movedToDifferentParent['moving-target'].width, '91px');

  for (const [transition, result] of Object.entries(reference.observations)) {
    for (const [node, measurement] of Object.entries(result)) {
      for (const value of [measurement.rect.x, measurement.rect.y, measurement.rect.width, measurement.rect.height]) {
        assert.ok(Number.isFinite(value), `${transition}.${node} geometry must be finite`);
      }
    }
  }
});

test('C05 Android와 iOS fixture가 Chromium HTML과 같은 style·WGPU runtime 경로를 쓴다', async () => {
  const [htmlBytes, appScriptBytes, runtimeBytes, javaBytes, jniBytes, iosBytes, runnerBytes] =
    await Promise.all([
      bytes(paths.html), bytes(paths.appScript), bytes('crates/spinon-ffi/src/runtime_gpu.rs'),
      bytes('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
      bytes('platforms/android/app/src/main/cpp/spinon_jni.cc'),
      bytes('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
      bytes('platforms/ios/Sources/SpinonRunner.mm'),
    ]);
  const html = htmlBytes.toString('utf8');
  const appScript = appScriptBytes.toString('utf8');
  const appStyles = [...appScript.matchAll(/(root|opaque|transparent)\.setAttribute\('style','([^']+)'\)/g)]
    .map(([, target, style]) => [target, style]);
  const expectedHtmlStyles = ['gap-parent', 'gap-first', 'gap-second'].map((id) => {
    const match = html.match(new RegExp(`id="${id}" style="([^"]+)"`));
    assert.ok(match, `HTML fixture에 ${id}가 있어야 합니다`);
    return match[1];
  });

  assert.deepEqual(appStyles.map(([target]) => target), ['root', 'opaque', 'transparent']);
  assert.deepEqual(appStyles.map(([, style]) => style), expectedHtmlStyles);
  assert.doesNotMatch(appScript, /createElement|appendChild/);
  assert.ok(runtimeBytes.toString('utf8').includes(
    'include_str!("../../../tests/fixtures/css/c05/runtime-custom-properties-app.js")',
  ));
  const java = javaBytes.toString('utf8');
  const androidEval = java.slice(
    java.indexOf('private void evaluateCustomPropertiesFixture()'),
    java.indexOf('private void refreshEnvironment()'),
  );
  assert.match(androidEval, /enqueueRuntime\(\(\)\s*->/);
  assert.match(androidEval, /nativeEvalCustomPropertiesFixture\(host\)/);
  assert.match(jniBytes.toString('utf8'), /spinon_runtime_gpu_host_eval_custom_properties_fixture\(/);
  const ios = iosBytes.toString('utf8');
  const iosEval = ios.slice(
    ios.indexOf('@objc private func evaluateCustomPropertiesFixture()'),
    ios.indexOf('private func ensureRenderer(width: Int, height: Int)'),
  );
  assert.match(iosEval, /enqueueRuntime\s*\{/);
  assert.match(iosEval, /SpinonRunner\.evalRuntimeGpuCustomPropertiesFixture\(handle\)/);
  assert.match(runnerBytes.toString('utf8'), /spinon_runtime_gpu_host_eval_custom_properties_fixture\(/);
});
