import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c04/runtime-css-to-gpu-resize-inventory.json',
  html: 'tests/fixtures/css/c04/runtime-css-to-gpu-resize.html',
  javascript: 'tests/fixtures/css/c04/runtime-css-to-gpu-resize.js',
  capture: 'tools/css-reference/capture-runtime-css-to-gpu-resize.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c04-runtime-css-to-gpu-resize-v1.json',
  javaDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  runtimeCore: 'crates/spinon-ffi/src/runtime_gpu.rs',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('두 viewport Chromium reference가 고정 입력과 oracle에 묶여 있다', async () => {
  const [inventoryBytes, htmlBytes, javascriptBytes, captureBytes, helperBytes, referenceBytes] =
    await Promise.all([
      read(paths.inventory), read(paths.html), read(paths.javascript), read(paths.capture),
      read(paths.helper), read(paths.reference),
    ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const viewports = [
    { id: 'baseline', width: 301, height: 100, deviceScaleFactor: 1 },
    { id: 'expanded', width: 341, height: 128, deviceScaleFactor: 1 },
  ];

  assert.equal(inventory.schema, 'spinon-css-runtime-gpu-resize-inventory/v1');
  assert.equal(inventory.fixtureId, 'C04-runtime-css-to-gpu-resize-v1');
  assert.deepEqual(inventory.viewports, viewports);
  assert.equal(reference.schema, 'spinon-css-runtime-css-to-gpu-resize-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.fixture.javascriptPath, paths.javascript);
  assert.equal(reference.fixture.javascriptSha256, sha256(javascriptBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.name, 'Chromium');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.oracle.executableSha256, 'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954');
  assert.deepEqual(reference.comparison, inventory.comparison);
  assert.deepEqual(Object.keys(reference.observations).sort(), ['baseline', 'expanded']);

  for (const viewport of viewports) {
    const observation = reference.observations[viewport.id];
    assert.equal(observation.fixtureId, inventory.fixtureId);
    assert.deepEqual(observation.viewport, {
      width: viewport.width,
      height: viewport.height,
      deviceScaleFactor: viewport.deviceScaleFactor,
    });
    assert.deepEqual(observation.nodes.map(({ id }) => id), ['root', 'opaque', 'transparent', 'hidden']);
    assert.deepEqual(observation.nodes[0].properties, {
      display: 'flex',
      width: `${viewport.width}px`,
      height: `${viewport.height}px`,
      'background-color': 'rgb(18, 52, 86)',
    });
    assert.deepEqual(observation.nodes[0].rect, {
      x: 0, y: 0, width: viewport.width, height: viewport.height,
    });
    assert.deepEqual(observation.nodes[1].rect, { x: 0, y: 0, width: 51, height: 31 });
    assert.deepEqual(observation.nodes[2].rect, { x: 62, y: 0, width: 41, height: 31 });
    assert.equal(observation.nodes[2].properties['background-color'], 'rgba(0, 0, 0, 0)');
    assert.equal(observation.nodes[3].properties.display, 'none');
    assert.deepEqual(observation.nodes[3].rect, { x: 0, y: 0, width: 0, height: 0 });
  }
});

test('Android와 iOS runtime viewport가 고정 fixture 크기 대신 surface bounds를 사용한다', async () => {
  const [inventoryBytes, htmlBytes, javascriptBytes, javaBytes, iosBytes, runtimeBytes] =
    await Promise.all([
      read(paths.inventory), read(paths.html), read(paths.javascript), read(paths.javaDemo),
      read(paths.iosDemo), read(paths.runtimeCore),
    ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const html = htmlBytes.toString('utf8');
  const javascript = javascriptBytes.toString('utf8');
  const java = javaBytes.toString('utf8');
  const ios = iosBytes.toString('utf8');
  const runtime = runtimeBytes.toString('utf8');
  const javascriptStyles = [...javascript.matchAll(/setAttribute\('style','([^']+)'\)/g)]
    .map(([, style]) => style);
  const htmlStyles = [...html.matchAll(/\sstyle="([^"]+)"/g)].map(([, style]) => style);

  assert.deepEqual(javascriptStyles, htmlStyles);
  assert.ok(javascriptStyles[0].includes('width:100vw;height:100vh'));
  assert.ok(inventory.nodes.some(({ id }) => id === 'root'));
  assert.ok(runtime.includes('include_str!("../../../tests/fixtures/css/c04/runtime-css-to-gpu-resize.js")'));

  assert.match(java, /surfaceWidth\s*>\s*0\s*\?\s*\(float\)\s*surfaceWidth\s*\/\s*density/);
  assert.match(java, /surfaceHeight\s*>\s*0\s*\?\s*\(float\)\s*surfaceHeight\s*\/\s*density/);
  assert.match(java, /hostPresentationSequence\s*==\s*sequence/);
  assert.match(java, /표면 크기 전환/);
  assert.match(java, /expandedSurface\s*\?\s*341\s*:\s*301/);
  assert.match(java, /expandedSurface\s*\?\s*128\s*:\s*100/);
  assert.match(java, /previousRenderer\s*=\s*!available\s*\|\|\s*rendererGeneration\s*!=\s*generation\s*\?\s*rendererHandle\s*:\s*0/);
  assert.match(java, /if\s*\(previousRenderer\s*!=\s*0\)\s*nativeDestroySurface\(previousRenderer\)/);
  assert.match(java, /if\s*\(width\s*<=\s*0\s*\|\|\s*height\s*<=\s*0\)/);
  assert.match(java, /stale\s*=\s*closing\s*\|\|\s*!surfaceAvailable\s*\|\|\s*surfaceGeneration\s*!=\s*generation/);
  assert.match(java, /awaitUninterruptibly\(renderQueueDrained\)/);
  assert.doesNotMatch(java, /nativeSetEnvironment\(hostHandle,\s*301,\s*100/);

  assert.match(ios, /viewportWidthCssPx\s*=\s*Float\(viewportSize\.width\)/);
  assert.match(ios, /viewportHeightCssPx\s*=\s*Float\(viewportSize\.height\)/);
  assert.match(ios, /return\s*\(viewportWidthCssPx,\s*viewportHeightCssPx,\s*displayScale,\s*darkMode\)/);
  assert.match(ios, /presentationSequence\s*==\s*sequence/);
  assert.match(ios, /표면 크기 전환/);
  assert.match(ios, /expandedSurface\s*\?\s*341\s*:\s*301/);
  assert.match(ios, /expandedSurface\s*\?\s*128\s*:\s*100/);
  assert.match(ios, /--spinon-c0410-auto-resize/);
  assert.match(ios, /private func runAutomaticResizeProbe\(step: Int\)/);
  assert.match(ios, /guard step < 3, !isClosing else \{ return \}/);
  assert.match(ios, /step \+ 1 < 3/);
  assert.match(ios, /surfaceConfigurationPending = true/);
  assert.match(ios, /rendererSurfaceGeneration = nil/);
  assert.match(ios, /rendererSurfaceGeneration == surfaceGeneration/);
  assert.doesNotMatch(ios, /return\s*\(301,\s*100,\s*displayScale/);
  assert.match(runtime, /surface_texture=\{\}x\{\}/);
  assert.match(runtime, /render_viewport_css_px=\{\}x\{\}/);
});
