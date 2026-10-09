import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c04/runtime-css-to-gpu-inventory.json',
  html: 'tests/fixtures/css/c04/runtime-css-to-gpu.html',
  script: 'tests/fixtures/css/c04/runtime-css-to-gpu.js',
  capture: 'tools/css-reference/capture-runtime-css-to-gpu.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c04-runtime-css-to-gpu-v1.json',
  javaDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  androidJni: 'platforms/android/app/src/main/cpp/spinon_jni.cc',
  iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  iosRunner: 'platforms/ios/Sources/SpinonRunner.mm',
  runtimeCore: 'crates/spinon-ffi/src/runtime_gpu.rs',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C04 runtime CSS to GPU reference is bound to the fixed Chromium inputs', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper), read(paths.reference),
  ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));

  assert.equal(inventory.schema, 'spinon-css-runtime-gpu-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-runtime-css-to-gpu-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Chromium');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.oracle.executableSha256, 'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954');
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.deepEqual(reference.observation.nodes.map(({ id }) => id), ['root', 'opaque', 'transparent', 'hidden']);
  assert.deepEqual(reference.observation.nodes.map(({ properties }) => properties['background-color']), [
    'rgb(18, 52, 86)', 'rgb(51, 102, 255)', 'rgba(0, 0, 0, 0)', 'rgb(255, 0, 0)',
  ]);
  assert.deepEqual(reference.observation.nodes.map(({ properties }) => properties.display), [
    'flex', 'block', 'block', 'none',
  ]);
  assert.deepEqual(reference.observation.nodes.map(({ rect }) => rect), [
    { x: 0, y: 0, width: 301, height: 100 },
    { x: 0, y: 0, width: 51, height: 31 },
    { x: 62, y: 0, width: 41, height: 31 },
    { x: 0, y: 0, width: 0, height: 0 },
  ]);
  assert.deepEqual(reference.observation.environment, {
    locale: 'en-US', intlLocale: 'en-US', timeZone: 'UTC', dark: false, coarsePointer: true, hover: false,
  });
});

test('Android and iOS run the same checked-in JavaScript fixture through V8', async () => {
  const [scriptBytes, htmlBytes, javaBytes, jniBytes, iosBytes, runnerBytes, coreBytes] = await Promise.all([
    read(paths.script), read(paths.html), read(paths.javaDemo), read(paths.androidJni),
    read(paths.iosDemo), read(paths.iosRunner), read(paths.runtimeCore),
  ]);
  const script = scriptBytes.toString('utf8');
  const html = htmlBytes.toString('utf8');
  const java = javaBytes.toString('utf8');
  const jni = jniBytes.toString('utf8');
  const ios = iosBytes.toString('utf8');
  const runner = runnerBytes.toString('utf8');
  const runtimeCore = coreBytes.toString('utf8');

  assert.ok(runtimeCore.includes('include_str!("../../../tests/fixtures/css/c04/runtime-css-to-gpu-resize.js")'));
  assert.match(java, /nativeEvalFixture\(host\)/);
  assert.match(jni, /spinon_runtime_gpu_host_eval_fixture\(\s*host,/);
  assert.match(ios, /SpinonRunner\.evalRuntimeGpuFixture\(handle\)/);
  assert.match(runner, /spinon_runtime_gpu_host_eval_fixture\(/);
  assert.match(runner, /spinon_runtime_gpu_host_prepare_uikit_surface\(/);
  assert.match(runner, /spinon_runtime_gpu_host_create_uikit\(/);

  const ensureIosRenderer = ios.slice(
    ios.indexOf('private func ensureRenderer(width: Int, height: Int)'),
    ios.indexOf('private func resizeRenderer(to size: CGSize)'),
  );
  assert.match(ensureIosRenderer, /dispatchPrecondition\(condition: \.onQueue\(\.main\)\)/);
  assert.ok(
    ensureIosRenderer.indexOf('prepareRuntimeGpuWgpuSurface')
      < ensureIosRenderer.indexOf('enqueueRender'),
    'UIKit surface 준비는 background 렌더 작업을 넣기 전에 main thread에서 끝나야 합니다',
  );

  const javaNativeMethods = [...java.matchAll(/private static native [^;]*?\b(native\w+)\s*\(/gs)]
    .map(([, method]) => method)
    .sort();
  const jniMethods = [...jni.matchAll(/Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_(native\w+)\s*\(/g)]
    .map(([, method]) => method)
    .sort();
  assert.deepEqual(jniMethods, javaNativeMethods);

  const reconcileRenderState = java.slice(
    java.indexOf('private void reconcileRenderState()'),
    java.indexOf('private void drawCurrentRenderer('),
  );
  assert.match(reconcileRenderState, /nativeCreateSurface\(host, currentSurface, width, height, backend\)/);
  assert.match(reconcileRenderState, /stale\s*=\s*closing\s*\|\|\s*!surfaceAvailable\s*\|\|\s*surfaceGeneration\s*!=\s*generation/);
  assert.doesNotMatch(reconcileRenderState, /refreshEnvironment\(\)|requestDraw\(\)/);
  const dispose = java.slice(java.indexOf('void dispose()'), java.indexOf('@Override', java.indexOf('void dispose()')));
  assert.ok(dispose.indexOf('hostPresentationSequence = nativeBeginPresentationUpdate(hostHandle)')
    < dispose.indexOf('closing = true'));
  const iosShutdownStart = ios.indexOf('private func beginShutdown()');
  const iosShutdown = ios.slice(iosShutdownStart, ios.indexOf('\n    deinit {', iosShutdownStart));
  assert.ok(iosShutdown.indexOf('_ = beginPresentationUpdate()') < iosShutdown.indexOf('closing = true'));
  assert.match(ios, /SpinonRunner\.beginRuntimeGpuPresentationUpdate\(handle\)/);

  const scriptStyles = [...script.matchAll(/setAttribute\('style','([^']+)'\)/g)]
    .map(([, style]) => style);
  const htmlStyles = [...html.matchAll(/\sstyle="([^"]+)"/g)]
    .map(([, style]) => style);
  assert.deepEqual(scriptStyles, htmlStyles);
  assert.deepEqual(
    [...script.matchAll(/const (root|opaque|transparent|hidden)=document\.createElement\('div'\)/g)]
      .map(([, id]) => id),
    ['root', 'opaque', 'transparent', 'hidden'],
  );
  assert.deepEqual(
    [...html.matchAll(/id="(root|opaque|transparent|hidden)"/g)].map(([, id]) => id),
    ['root', 'opaque', 'transparent', 'hidden'],
  );
});
