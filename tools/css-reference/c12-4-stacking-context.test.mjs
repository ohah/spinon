import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import test from 'node:test';

const repositoryRoot = join(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c12/stacking-context-inventory.json';
const htmlPath = 'tests/fixtures/css/c12/stacking-context.html';
const capturePath = 'tools/css-reference/capture-c12-4-stacking-context.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const referencePath = 'tests/fixtures/css/references/c12-4-stacking-context-v1.json';
const screenshotDirectory = 'spec/internal/evidence/c12-4-stacking-context-2026-10-11';

async function load(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C12.4 fixture은 고정된 범위와 실행하지 않은 WPT source를 구분한다', async () => {
  const [inventoryBytes, htmlBytes] = await Promise.all([load(inventoryPath), load(htmlPath)]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const caseIds = inventory.cases.map(({ id }) => id);
  const nodeIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
  const controlIds = inventory.negativeControls.map(({ id }) => id);

  assert.equal(inventory.schema, 'spinon-css-c12-4-stacking-inventory/v1');
  assert.equal(inventory.fixtureId, 'C12.4-stacking-context-v1');
  assert.equal(inventory.cases.length, 26);
  assert.equal(inventory.negativeControls.length, 23);
  assert.equal(new Set(caseIds).size, caseIds.length);
  assert.equal(new Set(nodeIds).size, nodeIds.length);
  assert.equal(new Set(controlIds).size, controlIds.length);
  assert.equal(inventory.viewport.widthCssPx, 1000);
  assert.equal(inventory.viewport.heightCssPx, 1400);
  assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.equal(inventory.comparison.maximumPixelChannelError, 1);
  assert.match(htmlBytes.toString('utf8'), /id="spinon-c124-mount"/);
  assert.equal(inventory.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
  assert.equal(inventory.wpt.execution, 'not-run');
  assert.deepEqual(inventory.wpt.sources.map(({ path, sha256: digest }) => [path, digest]), [
    ['css/CSS2/zindex/z-index-stack-001.xht', '71ddebb4643d0a99f4b07e7fa9e8f507c6ce2319be0ba4d3ffff5a37264440d1'],
    ['css/CSS2/zindex/z-index-abspos-003.xht', 'd04f95aefe8ba06a49cd567e03595cd35dc580d32e234d5d79535e18cd9852c6'],
    ['css/css-position/position-absolute-under-non-containing-stacking-context.html', '6413b79fab31d82c3e84729e7e7364a93526ae5ffb19ba59c08dc25d9003e7ed'],
    ['css/css-flexbox/flexbox-items-as-stacking-contexts-001.xhtml', '4ca1879dd67d79897770e949064838b97d95c517abc8e8fc343038942bcb5ddb'],
    ['css/css-flexbox/flexbox-paint-ordering-002.xhtml', 'ebb748144bddd3b1fc6aa68b0a45d7a869c07db632ced97ab62fc2c7ff2c293d'],
  ]);

  for (const fixtureCase of inventory.cases) {
    const ids = fixtureCase.nodes.map(({ id }) => id);
    assert.ok(ids.length > 0, fixtureCase.id);
    assert.equal(new Set(ids).size, ids.length, fixtureCase.id);
    for (const [index, node] of fixtureCase.nodes.entries()) {
      assert.equal(typeof node.style, 'string', `${fixtureCase.id}/${node.id}`);
      assert.ok(node.parentId === null || ids.slice(0, index).includes(node.parentId),
        `parent는 preorder에서 앞서 선언되어야 함: ${fixtureCase.id}/${node.id}`);
    }
    for (const sample of fixtureCase.samples) {
      if (sample.topToBottom.length === 0) {
        assert.equal(sample.id, 'root-canvas-background', `${fixtureCase.id}/${sample.id}`);
      }
      assert.ok(sample.topToBottom.every((id) => ids.includes(id)), `${fixtureCase.id}/${sample.id}`);
      assert.equal(sample.rgba.length, 4, `${fixtureCase.id}/${sample.id}`);
    }
  }
});

test('C12.4 runtime 진입점이 iOS·Android에서 같은 Rust fixture를 호출한다', async () => {
  const [appDelegateBytes, iosDemoBytes, androidActivityBytes, androidDemoBytes, runnerHeaderBytes, jniBytes] = await Promise.all([
    load('platforms/ios/Sources/AppDelegate.swift'),
    load('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
    load('platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    load('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    load('platforms/ios/Sources/SpinonRunner.h'),
    load('platforms/android/app/src/main/cpp/spinon_jni.cc'),
  ]);
  const appDelegate = appDelegateBytes.toString('utf8');
  const iosDemo = iosDemoBytes.toString('utf8');
  const androidActivity = androidActivityBytes.toString('utf8');
  const androidDemo = androidDemoBytes.toString('utf8');
  const runnerHeader = runnerHeaderBytes.toString('utf8');
  const jni = jniBytes.toString('utf8');

  assert.match(appDelegate, /arguments\.contains\("--spinon-c124-stacking-context"\)/);
  assert.match(iosDemo, /\.contains\("--spinon-c124-stacking-context"\)/);
  assert.match(iosDemo, /createRuntimeGpuHostWithC12_4StackingFixture\(\)/);
  assert.match(iosDemo, /evalRuntimeGpuC12_4StackingFixture\(handle\)/);
  assert.match(iosDemo, /\? "SPINON_C124"/);
  assert.match(androidActivity, /getBooleanExtra\("spinon_c124_stacking_context", false\)/);
  assert.match(androidActivity, /getStringExtra\("spinon_c124_surface_case"\) != null/);
  assert.match(androidDemo, /getBooleanExtra\("spinon_c124_stacking_context", false\)/);
  assert.match(androidDemo, /getStringExtra\("spinon_c124_surface_case"\)/);
  assert.match(androidDemo, /nativeCreateC124StackingHost\(\)/);
  assert.match(androidDemo, /nativeEvalC124StackingFixture\(host\)/);
  assert.match(androidDemo, /nativeEvalC124Source\(host, fixtureCase\.getString\("source"\)\)/);
  assert.match(androidDemo, /C124SurfacePixelCapture\.request\(surfaceView/);
  assert.match(androidDemo, /SPINON_C124_SURFACE_CAPTURE status=ok/);
  const pixelCapture = (await load('platforms/android/app/src/main/java/dev/spinon/bootstrap/C124SurfacePixelCapture.java')).toString('utf8');
  assert.match(pixelCapture, /PixelCopy\.request\(surfaceView/);
  assert.match(pixelCapture, /originXCssPx \+ sample\.xCssPx \+ 0\.5/);
  assert.match(pixelCapture, /BitmapFactory\.decodeFile/);
  const readySurfaceInitialization = androidDemo.match(
    /private void schedulePositioningInitializationForReadySurface\(\) \{([\s\S]*?)\n    \}/,
  )?.[1];
  assert.ok(readySurfaceInitialization, 'ready-surface 초기화 경계를 확인해야 함');
  assert.match(readySurfaceInitialization, /!c124StackingProbeRequested/);
  assert.match(runnerHeader, /createRuntimeGpuHostWithC12_4StackingFixture/);
  assert.match(runnerHeader, /evalRuntimeGpuC12_4StackingFixture/);
  assert.match(jni, /spinon_runtime_gpu_host_new_c12_4_stacking_fixture/);
  assert.match(jni, /spinon_runtime_gpu_host_eval_c12_4_stacking_fixture/);
});

test('실기기 Android와 iOS Simulator가 같은 10개 frame·paint rank를 제출한다', async () => {
  const evidence = 'spec/internal/evidence/c12-4-stacking-runtime-2026-10-11';
  const [androidLogBytes, iosLogBytes, androidPng, iosPng] = await Promise.all([
    load(`${evidence}/android-sm-s731n-android16-vulkan.log`),
    load(`${evidence}/ios-simulator.log`),
    load(`${evidence}/android-sm-s731n-android16-vulkan.png`),
    load(`${evidence}/ios-26.2-iphone-17-pro-simulator.png`),
  ]);
  const androidLog = androidLogBytes.toString('utf8');
  const iosLog = iosLogBytes.toString('utf8');
  const expectedPaintOrder =
    'paint_node_order=[0:1,1:2,2:8,3:4,4:5,5:6,6:9,7:3,8:7,9:10]';
  const expectedFrames = [
    '0:node=1,x=0,y=0,width=320,height=240',
    '1:node=2,x=20,y=20,width=120,height=80',
    '2:node=3,x=20,y=20,width=100,height=70',
    '3:node=4,x=50,y=50,width=100,height=70',
    '4:node=5,x=165,y=20,width=120,height=80',
    '5:node=6,x=165,y=20,width=120,height=80',
    '6:node=7,x=165,y=20,width=120,height=80',
    '7:node=8,x=20,y=125,width=240,height=90',
    '8:node=9,x=20,y=125,width=150,height=70',
    '9:node=10,x=70,y=125,width=150,height=70',
  ];

  assert.match(androidLog, /SPINON_C124_EVAL status=0 .*document_nodes=10/);
  assert.match(iosLog, /SPINON_C124_EVAL status=0 .*document_nodes=10/);
  assert.match(androidLog, /SPINON_C0410_ENVIRONMENT viewport=320\.0x240\.0 scale=2\.8125 /);
  assert.match(androidLog, /SPINON_C0410_EVAL status=0 .*layout=ready boxes=10/);
  assert.match(iosLog, /SPINON_C0410_ENVIRONMENT viewport=320\.0x240\.0 scale=3\.0 .*layout=ready boxes=10/);
  assert.match(androidLog, /SPINON_C0410_RENDERER=backend=Vulkan device=Samsung Xclipse 940/);
  assert.match(iosLog, /SPINON_C0410_RENDERER status=0 backend=Metal device=Apple iOS simulator GPU/);
  assert.match(androidLog, /SPINON_C0410_DRAW .*status=0 presented boxes=10/);
  assert.match(iosLog, /SPINON_C0410_DRAW .*status=0 presented boxes=10/);
  for (const log of [androidLog, iosLog]) {
    assert.ok(log.includes(expectedPaintOrder), '두 플랫폼의 paint rank가 같아야 함');
    for (const frame of expectedFrames) {
      assert.ok(log.includes(frame), `플랫폼 frame 누락: ${frame}`);
    }
  }
  for (const png of [androidPng, iosPng]) {
    assert.equal(png.toString('ascii', 1, 4), 'PNG');
  }
  assert.equal(androidPng.readUInt32BE(16), 1080);
  assert.equal(androidPng.readUInt32BE(20), 2340);
  assert.equal(iosPng.readUInt32BE(16), 1206);
  assert.equal(iosPng.readUInt32BE(20), 2622);
});

test('Chrome 154 reference와 PNG는 fixture·도구·환경 해시 및 픽셀을 고정한다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    load(inventoryPath), load(htmlPath), load(capturePath), load(helperPath), load(referencePath),
  ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));

  assert.equal(reference.schema, 'spinon-css-c12-4-stacking-reference/v1');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.equal(reference.captureTool.helperSha256, sha256(helperBytes));
  assert.equal(reference.chromium.version, 'Chrome/154.0.8037.98');
  assert.equal(reference.chromium.cliVersion, 'Google Chrome 154.0.8037.98');
  assert.equal(reference.chromium.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.chromium.binarySha256,
    'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954');
  assert.equal(reference.wpt.execution, 'not-run');
  assert.equal(reference.inventorySummary.cases, inventory.cases.length);
  assert.equal(reference.inventorySummary.negativeControls, inventory.negativeControls.length);
  assert.equal(reference.observations.length, 2);
  assert.equal(reference.screenshotFiles.length, 2);
  assert.equal(reference.hitTestComparison.role, 'supplementary-observation-not-paint-order-oracle');

  const sampleCount = inventory.cases.reduce((sum, fixtureCase) => sum + fixtureCase.samples.length, 0);
  for (const [index, observation] of reference.observations.entries()) {
    const dpr = inventory.viewport.deviceScaleFactors[index];
    assert.equal(observation.environment.deviceScaleFactor, dpr);
    assert.equal(observation.environment.width, inventory.viewport.widthCssPx);
    assert.equal(observation.environment.height, inventory.viewport.heightCssPx);
    assert.equal(observation.environment.locale, inventory.environment.locale);
    assert.equal(observation.environment.timeZone, inventory.environment.timeZone);
    assert.equal(observation.cases.length, inventory.cases.length);
    assert.equal(observation.controls.length, inventory.negativeControls.length);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));

    for (const fixtureCase of inventory.cases) {
      const capturedCase = observation.cases.find(({ id }) => id === fixtureCase.id);
      assert.deepEqual(capturedCase.nodes.map(({ id }) => id), fixtureCase.nodes.map(({ id }) => id));
      for (const expectedNode of fixtureCase.nodes) {
        const node = capturedCase.nodes.find(({ id }) => id === expectedNode.id);
        assert.equal(node.parentId, expectedNode.parentId ?? null, `${fixtureCase.id}/${node.id} parent`);
        assert.equal(node.inputStyle, expectedNode.style, `${fixtureCase.id}/${node.id} source style`);
        for (const [property, value] of Object.entries(expectedNode.expectedComputed ?? {})) {
          assert.equal(node.properties[property], value, `${fixtureCase.id}/${node.id}/${property}`);
        }
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}`);
        }
      }
      for (const expectedSample of fixtureCase.samples) {
        const sample = capturedCase.samples.find(({ id }) => id === expectedSample.id);
        assert.ok(sample, `${fixtureCase.id}/${expectedSample.id} hit-test observation`);
        assert.ok(Array.isArray(sample.hitTestTopToBottom));
      }
    }

    const screenshot = reference.screenshotFiles[index];
    assert.equal(screenshot.deviceScaleFactor, dpr);
    assert.equal(screenshot.sampleCount, sampleCount);
    assert.deepEqual(screenshot.pixelSize, {
      width: inventory.viewport.widthCssPx * dpr,
      height: inventory.viewport.heightCssPx * dpr,
    });
    const pngBytes = await load(screenshot.path);
    assert.equal(sha256(pngBytes), screenshot.sha256);
    assert.equal(pngBytes.toString('ascii', 1, 4), 'PNG');
    assert.equal(pngBytes.readUInt32BE(16), screenshot.pixelSize.width);
    assert.equal(pngBytes.readUInt32BE(20), screenshot.pixelSize.height);
    assert.equal(screenshot.samples.length, sampleCount);
    for (const expectedSample of inventory.cases.flatMap((fixtureCase) =>
      fixtureCase.samples.map((sample) => ({ caseId: fixtureCase.id, ...sample })))) {
      const pixel = screenshot.samples.find(({ fixtureId, sampleId }) =>
        fixtureId === expectedSample.caseId && sampleId === expectedSample.id);
      assert.ok(pixel, `${expectedSample.caseId}/${expectedSample.id}`);
      assert.deepEqual(pixel.expectedRgba, expectedSample.rgba);
      assert.ok(pixel.rgba.every((channel, channelIndex) =>
        Math.abs(channel - expectedSample.rgba[channelIndex]) <= inventory.comparison.maximumPixelChannelError),
      `${expectedSample.caseId}/${expectedSample.id} pixel`);
    }
  }

  assert.deepEqual(reference.observations[0].cases, reference.observations[1].cases,
    'CSS frame, computed style and hit-test observation must be DPR-independent');
  assert.deepEqual(reference.observations[0].controls, reference.observations[1].controls);
  assert.deepEqual(reference.screenshotFiles[0].samples.map(({ fixtureId, sampleId, rgba }) =>
    [fixtureId, sampleId, rgba]), reference.screenshotFiles[1].samples.map(({ fixtureId, sampleId, rgba }) =>
    [fixtureId, sampleId, rgba]));
});

test('Chrome screenshot은 row-reverse의 관측 pixel과 top-layer 표본을 분리해 고정한다', async () => {
  const [inventoryBytes, referenceBytes] = await Promise.all([load(inventoryPath), load(referencePath)]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const reverseCase = inventory.cases.find(({ id }) => id === 'flex-row-reverse-paint');
  const reverseSample = reverseCase.samples.find(({ id }) => id === 'row-reverse-chrome-front-to-back');
  const reversePixels = reference.screenshotFiles.map(({ samples }) => samples.find(({ fixtureId, sampleId }) =>
    fixtureId === reverseCase.id && sampleId === reverseSample.id));

  assert.deepEqual(reverseSample.topToBottom, [
    'c124-row-reverse-source-first', 'c124-row-reverse-source-second',
  ]);
  assert.deepEqual(reverseSample.rgba, [40, 102, 246, 255]);
  assert.ok(reversePixels.every(({ rgba }) => JSON.stringify(rgba) === JSON.stringify(reverseSample.rgba)));

  const topLayerCase = inventory.cases.find(({ id }) => id === 'top-layer-dialog');
  const topLayerSample = topLayerCase.samples[0];
  for (const screenshot of reference.screenshotFiles) {
    const pixel = screenshot.samples.find(({ fixtureId, sampleId }) =>
      fixtureId === topLayerCase.id && sampleId === topLayerSample.id);
    assert.deepEqual(pixel.rgba, topLayerSample.rgba);
  }

  const chromeObservation = reference.hitTestComparison.deviceScaleFactors[0].cases
    .find(({ caseId }) => caseId === reverseCase.id).samples[0];
  assert.equal(chromeObservation.sampleId, reverseSample.id);
  assert.deepEqual(chromeObservation.observedHitTestTopToBottom, reverseSample.topToBottom);
  assert.equal(reference.hitTestComparison.role, 'supplementary-observation-not-paint-order-oracle');
});
