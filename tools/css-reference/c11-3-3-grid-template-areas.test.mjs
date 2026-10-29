import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';
import {
  selectAndroidAVDSerial,
  selectAndroidPhysicalSerial,
} from './c11-3-3-grid-template-areas-runtime-device.mjs';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const evidenceRoot = 'spec/internal/evidence/c11-3-3-grid-template-areas-precomparison-2026-10-14';
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json',
  html: 'tests/fixtures/css/c11/grid-template-areas-c1133.html',
  capture: 'tools/css-reference/capture-c11-3-3-grid-template-areas.mjs',
  classify: 'tools/css-reference/classify-c11-3-3-grid-template-areas-wpt-report.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  classificationHelper: 'tools/css-reference/c11-3-3-grid-template-areas-wpt-classification.mjs',
  reference: 'tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json',
  historicalClassifiedWpt: `${evidenceRoot}/wpt-chrome154.0.8037.92-classified-run01.json`,
  wptReport: `${evidenceRoot}/wpt-chrome154.0.8037.92-run01.json`,
};
const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventory = JSON.parse((await read(paths.inventory)).toString('utf8'));
const reference = JSON.parse((await read(paths.reference)).toString('utf8'));
const historicalClassifiedWpt = JSON.parse((await read(paths.historicalClassifiedWpt)).toString('utf8'));
const classifiedWpt = JSON.parse((await read(inventory.wpt.classifiedReportEvidence)).toString('utf8'));
const wptReport = JSON.parse((await read(paths.wptReport)).toString('utf8'));
const caseAt = (observation, id) => observation.cases.find((item) => item.id === id);
const frameAt = (observation, caseId, nodeId) => {
  const fixtureCase = caseAt(observation, caseId);
  const node = fixtureCase?.nodes.find((item) => item.id === nodeId);
  assert.ok(node, `${caseId}/${nodeId}가 없습니다.`);
  return node.rect;
};

test('기준 산출물이 고정 fixture, capture 도구, Chrome 실행 파일에 연결된다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, classifyBytes, helperBytes, classificationHelperBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.classify), read(paths.helper),
    read(paths.classificationHelper),
  ]);
  assert.equal(reference.schema, 'spinon-css-c11-3-3-grid-template-areas-precomparison/v1');
  assert.equal(reference.fixture.inventorySha256, historicalClassifiedWpt.inventorySha256);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{
    path: paths.helper,
    sha256: sha256(helperBytes),
  }]);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.92');
  assert.equal(reference.oracle.revision, '@334b65d254ccc35df4fca82706d1753227b01039');
  assert.equal(reference.oracle.executableSha256.length, 64);
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(historicalClassifiedWpt.schema, 'spinon-css-c11-3-3-wpt-chrome-classification/v1');
  assert.equal(historicalClassifiedWpt.classifier.path, paths.classify);
  assert.match(historicalClassifiedWpt.classifier.sha256, /^[a-f0-9]{64}$/);
  assert.equal(historicalClassifiedWpt.inventorySha256, reference.fixture.inventorySha256);
  assert.equal(historicalClassifiedWpt.reportSha256, sha256(await read(paths.wptReport)));
  assert.equal(classifiedWpt.classifier.path, paths.classify);
  assert.equal(classifiedWpt.classifier.sha256, sha256(classifyBytes));
  assert.equal(classifiedWpt.classifier.helperPath, paths.classificationHelper);
  assert.equal(classifiedWpt.classifier.helperSha256, sha256(classificationHelperBytes));
  assert.equal(classifiedWpt.inventorySha256, sha256(inventoryBytes));
  assert.equal(classifiedWpt.reportSha256, sha256(await read(paths.wptReport)));
});

test('26 scene의 CSSOM area matrix, track metadata와 DPR별 geometry가 보존된다', () => {
  assert.equal(inventory.cases.length, 26);
  assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  assert.equal(reference.screenshots.length, inventory.cases.length);
  for (const observation of reference.observations) {
    assert.equal(observation.cases.length, inventory.cases.length);
    for (const fixtureCase of inventory.cases) {
      const actual = caseAt(observation, fixtureCase.id);
      assert.ok(actual, `${fixtureCase.id} scene이 없습니다.`);
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodeIds, fixtureCase.id);
      assert.equal(actual.rootId, fixtureCase.rootId);
      assert.equal(actual.viewport.deviceScaleFactor, observation.deviceScaleFactor);
      assert.deepEqual(actual.environment, {
        direction: inventory.environment.direction,
        writingMode: inventory.environment.writingMode,
      });
      assert.deepEqual(actual.resolvedGrid.areas.rows, fixtureCase.expectedComputedAreaRows, fixtureCase.id);
      assert.equal(actual.resolvedGrid.columns.trackSizes.length, fixtureCase.expectedColumns, fixtureCase.id);
      assert.equal(actual.resolvedGrid.rows.trackSizes.length, fixtureCase.expectedRows, fixtureCase.id);
      assert.equal(actual.resolvedGrid.columns.lineNameGroups.length, fixtureCase.expectedColumns + 1, fixtureCase.id);
      assert.equal(actual.resolvedGrid.rows.lineNameGroups.length, fixtureCase.expectedRows + 1, fixtureCase.id);
      for (const node of actual.nodes) {
        assert.ok(node.parentId === null || fixtureCase.nodeIds.includes(node.parentId), `${fixtureCase.id}/${node.id}`);
        for (const field of inventory.comparison.rectFields) assert.ok(Number.isFinite(node.rect[field]));
      }
    }
  }
  const [dpr1, dpr2] = reference.observations;
  for (const fixtureCase of inventory.cases) {
    const first = caseAt(dpr1, fixtureCase.id);
    const second = caseAt(dpr2, fixtureCase.id);
    assert.deepEqual(first.rootRect, second.rootRect, fixtureCase.id);
    assert.deepEqual(first.nodes, second.nodes, fixtureCase.id);
    assert.deepEqual(first.resolvedGrid, second.resolvedGrid, fixtureCase.id);
  }
});

test('area 직사각형, null cell과 shorthand 배치가 Chrome geometry 기준을 따른다', () => {
  const dpr1 = reference.observations[0];
  assert.deepEqual(caseAt(dpr1, 'basic-rectangles').resolvedGrid.areas.rectangles.main,
    { rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 3 });
  assert.deepEqual(caseAt(dpr1, 'null-cell-dot-runs').resolvedGrid.areas.rows,
    [['.', 'hero', '.'], ['left', 'hero', 'right']]);
  assert.deepEqual(caseAt(dpr1, 'all-null-cells').resolvedGrid.areas.rectangles, {});
  assert.deepEqual(frameAt(dpr1, 'basic-rectangles', 'basic-head'), { x: 0, y: 0, width: 100, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'basic-rectangles', 'basic-main'), { x: 40, y: 40, width: 120, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'null-cell-dot-runs', 'null-hero'), { x: 40, y: 0, width: 60, height: 120 });
  assert.deepEqual(frameAt(dpr1, 'bare-area-edge-aliases', 'alias-area'), { x: 0, y: 0, width: 100, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'grid-area-one-value', 'area-one-item'), { x: 40, y: 40, width: 120, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'grid-area-two-values', 'area-two-item'), { x: 40, y: 0, width: 60, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'grid-area-three-values', 'area-three-item'), { x: 40, y: 40, width: 60, height: 80 });
  assert.deepEqual(frameAt(dpr1, 'grid-area-four-values', 'area-four-item'), { x: 0, y: 0, width: 100, height: 80 });
  assert.deepEqual(frameAt(dpr1, 'grid-template-shorthand', 'template-item'), { x: 40, y: 40, width: 120, height: 80 });
  assert.deepEqual(frameAt(dpr1, 'grid-shorthand', 'grid-shorthand-item'), { x: 40, y: 40, width: 120, height: 80 });
});

test('bare area custom ident와 숫자+ident, 충돌 이름, implicit named area를 구분한다', () => {
  const dpr1 = reference.observations[0];
  assert.deepEqual(frameAt(dpr1, 'bare-area-vs-explicit-one-name', 'bare-area-name'),
    { x: 40, y: 0, width: 120, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'bare-area-vs-explicit-one-name', 'explicit-one-name'),
    { x: 0, y: 40, width: 40, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'explicit-line-before-area', 'line-before-area'),
    { x: 0, y: 0, width: 160, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'explicit-line-after-area', 'line-after-area'),
    { x: 40, y: 0, width: 120, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'area-names-ending-start-or-end', 'suffix-area-name'),
    { x: 40, y: 0, width: 120, height: 40 });
  assert.deepEqual(frameAt(dpr1, 'implicit-area-from-explicit-lines', 'implicit-area-item'),
    { x: 0, y: 0, width: 100, height: 80 });
  assert.deepEqual(frameAt(dpr1, 'escaped-and-unicode-identifiers', 'unicode-escaped-item'),
    { x: 0, y: 0, width: 40, height: 120 });
  assert.deepEqual(frameAt(dpr1, 'escaped-and-unicode-identifiers', 'unicode-asian-item'),
    { x: 100, y: 0, width: 60, height: 120 });
});

test('invalid area declaration은 cascade fallback을 남기고 dimensions·implicit track은 별도 거부 경계다', () => {
  const dpr1 = reference.observations[0];
  const fallback = [
    ['fallback', 'fallback', '.'],
    ['left', 'right', 'right'],
    ['foot', 'foot', 'foot'],
  ];
  for (const id of [
    'invalid-l-shape-fallback', 'invalid-disconnected-area-fallback', 'invalid-row-width-fallback',
    'invalid-trash-token-fallback', 'empty-string-fallback',
  ]) {
    assert.deepEqual(caseAt(dpr1, id).resolvedGrid.areas.rows, fallback, id);
    const fixtureCase = inventory.cases.find((item) => item.id === id);
    assert.equal(fixtureCase.expectedSpinonOutcome, 'cascade-fallback');
  }
  const mismatch = caseAt(dpr1, 'area-track-count-mismatch');
  assert.deepEqual(mismatch.resolvedGrid.columns.trackSizes, ['40px', '120px', '0px']);
  assert.equal(inventory.cases.find(({ id }) => id === 'area-track-count-mismatch').authoredExplicitColumns, 2);
  assert.equal(inventory.cases.find(({ id }) => id === 'area-track-count-mismatch').expectedSpinonOutcome,
    'reject-area-track-count-mismatch');
  assert.deepEqual(frameAt(dpr1, 'area-track-count-mismatch', 'mismatch-last-cell'),
    { x: 160, y: 0, width: 0, height: 60 });
  const unknown = caseAt(dpr1, 'unknown-area-implicit-tracks');
  assert.equal(unknown.resolvedGrid.columns.trackSizes.length, 4);
  assert.equal(unknown.resolvedGrid.rows.trackSizes.length, 4);
  assert.equal(inventory.cases.find(({ id }) => id === 'unknown-area-implicit-tracks').expectedSpinonOutcome,
    'reject-unsupported-implicit-area-tracks');
});

test('nested container, hidden child와 DOM source order가 유지된다', () => {
  const dpr1 = reference.observations[0];
  const nested = caseAt(dpr1, 'nested-hidden-source-order');
  assert.deepEqual(nested.nodes.map(({ id }) => id), [
    'nested-outer-grid', 'nested-outer-item', 'nested-inner-grid',
    'nested-inner-item', 'nested-bottom-item', 'nested-hidden-item',
  ]);
  assert.equal(nested.nodes[2].parentId, 'nested-outer-grid');
  assert.equal(nested.nodes[3].parentId, 'nested-inner-grid');
  assert.deepEqual(frameAt(dpr1, 'nested-hidden-source-order', 'nested-inner-grid'),
    { x: 40, y: 40, width: 120, height: 80 });
  assert.deepEqual(frameAt(dpr1, 'nested-hidden-source-order', 'nested-hidden-item'),
    { x: 0, y: 0, width: 0, height: 0 });
});

test('기존 고정 WPT 분류 산출물과 Chrome oracle 이력이 보존된다', () => {
  assert.equal(historicalClassifiedWpt.runner.revision, inventory.wpt.revision);
  assert.equal(historicalClassifiedWpt.runner.browserVersion, '154.0.8037.92');
  assert.equal(historicalClassifiedWpt.outcome.tests, 4);
  assert.equal(historicalClassifiedWpt.outcome.testSubtests, 73);
  assert.equal(historicalClassifiedWpt.outcome.skipped, 0);
  assert.equal(historicalClassifiedWpt.outcome.failures, 0);
  assert.deepEqual(historicalClassifiedWpt.tests.map(({ status }) => status), ['OK', 'OK', 'OK', 'OK']);
  assert.deepEqual(wptReport.results.map(({ status }) => status), ['OK', 'OK', 'OK', 'OK']);
  assert.equal(wptReport.run_info.revision, inventory.wpt.revision);
  assert.equal(wptReport.run_info.browser_version, '154.0.8037.92');
  assert.equal(historicalClassifiedWpt.tests.reduce((sum, item) => sum + item.subtests.length, 0), 73);
  for (const testResult of historicalClassifiedWpt.tests) {
    const candidate = inventory.wpt.candidates.find(({ path }) => path === testResult.path);
    assert.ok(candidate, testResult.path);
    assert.equal(testResult.sourceSha256, candidate.sha256);
    assert.ok(testResult.subtests.every(({ status, classification, reason }) => {
      const expectedClassification = testResult.path.endsWith('grid-placement-unknown-named-area-001.html')
        ? 'currently-unsupported'
        : 'spinon-fixture-candidate';
      return status === 'PASS' && classification === expectedClassification && typeof reason === 'string';
    }), testResult.path);
  }
  const unsupported = historicalClassifiedWpt.tests.find(({ path }) => path.endsWith('grid-placement-unknown-named-area-001.html'));
  assert.equal(unsupported.subtests.length, 4);
  assert.ok(unsupported.subtests.every(({ classification }) => classification === 'currently-unsupported'));
});

test('현재 WPT 분류는 unknown-area 하위 사례를 고정 C11.3.3 scene에 연결한다', async () => {
  const candidate = inventory.wpt.candidates.find(({ path }) => path.endsWith('grid-placement-unknown-named-area-001.html'));
  assert.ok(candidate);
  const result = classifiedWpt.tests.find(({ path }) => path === candidate.path);
  assert.ok(result);
  assert.equal(classifiedWpt.outcome.tests, 4);
  assert.equal(classifiedWpt.outcome.testSubtests, 73);
  assert.equal(classifiedWpt.outcome.skipped, 0);
  assert.equal(classifiedWpt.outcome.failures, 0);
  assert.equal(classifiedWpt.inventorySha256, sha256(await read(paths.inventory)));
  assert.equal(classifiedWpt.reportEvidence, inventory.wpt.chromeReportEvidence);
  assert.equal(result.sourceSha256, candidate.sha256);
  assert.equal(result.fixtureSceneSource, candidate.fixtureSceneSource);
  assert.equal(result.fixtureSceneSourceSha256, sha256(await read(candidate.fixtureSceneSource)));
  assert.deepEqual(result.subtests.map(({ name, status, classification, fixtureSceneId, fixtureEvidence }) => ({
    name, status, classification, fixtureSceneId, fixtureEvidence,
  })), candidate.subtestCoverage.map((coverage) => ({
    name: coverage.name,
    status: 'PASS',
    classification: coverage.classification,
    fixtureSceneId: coverage.fixtureSceneId,
    fixtureEvidence: coverage.fixtureEvidence,
  })));
});

test('scene screenshot의 PNG hash가 기준 JSON과 실제 파일에 연결된다', async () => {
  assert.equal(reference.screenshots.length, inventory.cases.length);
  for (const screenshot of reference.screenshots) {
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, screenshot.caseId);
    assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', screenshot.caseId);
  }
});

test('C11.3.3 앱 runtime matrix와 표면 pixel fixture가 Chrome 기준에 고정된다', async () => {
  const runtimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v2.json';
  const historicalRuntimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
  const runtimeInventoryPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory-v2.json';
  const mobilePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-mobile-surface-pixel-matrix-v1.json';
  const cyclePixelsPath = 'tests/fixtures/css/c11/grid-template-areas-track-list-cycling-mobile-pixels-v1.json';
  const builderPath = 'tools/css-reference/build-c11-3-3-grid-template-areas-runtime-matrix.mjs';
  const capturePath = 'tools/css-reference/capture-c11-3-3-grid-template-areas-runtime.mjs';
  const matrixCapturePath = 'tools/css-reference/capture-c11-3-3-grid-template-areas-runtime-matrix.mjs';
  const iosMatrixCapturePath = 'tools/css-reference/capture-c11-3-3-grid-template-areas-ios-simulator-matrix.mjs';
  const revisionReuseCapturePath = 'tools/css-reference/capture-c11-3-3-grid-template-areas-revision-reuse.mjs';
  const deviceHelperPath = 'tools/css-reference/c11-3-3-grid-template-areas-runtime-device.mjs';
  const appDelegatePath = 'platforms/ios/Sources/AppDelegate.swift';
  const iosDemoPath = 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift';
  const androidMainActivityPath = 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java';
  const androidDemoPath = 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java';
  const [runtimeBytes, historicalRuntimeBytes, runtimeInventoryBytes, mobileBytes, cyclePixelsBytes, builderBytes, captureBytes, matrixCaptureBytes,
    iosMatrixCaptureBytes, revisionReuseCaptureBytes,
    deviceHelperBytes, appDelegateBytes, iosDemoBytes, androidMainActivityBytes, androidDemoBytes,
    inventoryBytes, htmlBytes] = await Promise.all([
    read(runtimePath), read(historicalRuntimePath), read(runtimeInventoryPath), read(mobilePath),
    read(cyclePixelsPath), read(builderPath), read(capturePath),
    read(matrixCapturePath), read(iosMatrixCapturePath),
    read(revisionReuseCapturePath),
    read(deviceHelperPath), read(appDelegatePath), read(iosDemoPath), read(androidMainActivityPath),
    read(androidDemoPath), read(paths.inventory), read(paths.html),
  ]);
  const runtime = JSON.parse(runtimeBytes.toString('utf8'));
  const runtimeInventory = JSON.parse(runtimeInventoryBytes.toString('utf8'));
  const historicalRuntime = JSON.parse(historicalRuntimeBytes.toString('utf8'));
  const mobile = JSON.parse(mobileBytes.toString('utf8'));
  const cyclePixels = JSON.parse(cyclePixelsBytes.toString('utf8'));
  const matrixInventory = runtimeInventory;
  const chromeCases = reference.observations[0].cases;
  assert.equal(runtime.schema, 'spinon-css-c11-3-3-grid-template-areas-runtime-matrix/v1');
  assert.equal(runtime.profileId, 'RuntimeBlockFlexGridTemplateV1');
  assert.equal(runtimeInventory.schema, 'spinon-css-c11-3-3-grid-template-areas-inventory/v2');
  assert.equal(reference.fixture.inventorySha256, historicalRuntime.inputs.inventory.sha256);
  assert.deepEqual(runtime.summary, {
    sceneCount: 37, nodeCount: 118, successCount: 32, cascadeFallbackCount: 5, expectedRejectCount: 0,
  });
  const supplementalHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-auto-tracks-c1133-runtime.html';
  const supplementalObservationPath = 'spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/chrome/chrome-observed.json';
  const cycleHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-auto-track-list-cycling-c1133-runtime.html';
  const cycleObservationPath = 'spec/internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/observation.json';
  const cycleScreenshotPath = 'spec/internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/fixture.png';
  const [supplementalHtmlBytes, supplementalObservationBytes] = await Promise.all([
    read(supplementalHtmlPath), read(supplementalObservationPath),
  ]);
  const supplementalObservation = JSON.parse(supplementalObservationBytes.toString('utf8'));
  const [cycleHtmlBytes, cycleObservationBytes, cycleScreenshotBytes] = await Promise.all([
    read(cycleHtmlPath), read(cycleObservationPath), read(cycleScreenshotPath),
  ]);
  const cycleObservation = JSON.parse(cycleObservationBytes.toString('utf8'));
  assert.deepEqual(runtime.inputs, {
    inventory: { path: runtimeInventoryPath, sha256: sha256(runtimeInventoryBytes) },
    baselineInventory: { path: paths.inventory, sha256: sha256(inventoryBytes) },
    historicalRuntimeMatrix: { path: historicalRuntimePath, sha256: sha256(historicalRuntimeBytes) },
    html: { path: paths.html, sha256: sha256(htmlBytes) },
    reference: { path: paths.reference, sha256: sha256(await read(paths.reference)) },
    supplementalHtml: { path: supplementalHtmlPath, sha256: sha256(supplementalHtmlBytes) },
    supplementalObservation: {
      path: supplementalObservationPath, sha256: sha256(supplementalObservationBytes),
    },
    cycleHtml: { path: cycleHtmlPath, sha256: sha256(cycleHtmlBytes) },
    cycleObservation: { path: cycleObservationPath, sha256: sha256(cycleObservationBytes) },
    cycleScreenshot: { path: cycleScreenshotPath, sha256: sha256(cycleScreenshotBytes) },
  });
  const supplementalIds = [
    'area-auto-tracks-none', 'area-auto-tracks-short-row', 'area-auto-tracks-short-col',
    'area-auto-tracks-auto', 'area-auto-tracks-long',
  ];
  const cycleIds = [
    'both-axis-cycle', 'row-cycle-from-first-implicit', 'column-cycle-from-first-implicit',
    'authored-tracks-exceed-area', 'row-cycle-continues-after-area',
    'column-cycle-continues-after-area',
  ];
  const cycleHtml = cycleHtmlBytes.toString('utf8');
  assert.ok(cycleHtml.includes('grid-row:area-row-end / span 1 ghost;grid-column:1'));
  assert.ok(cycleHtml.includes('grid-row:1;grid-column:area-col-end / span 1 ghost'));
  assert.deepEqual(runtime.cases.map(({ id }) => id), [
    ...matrixInventory.cases.map(({ id }) => id), ...supplementalIds, ...cycleIds,
  ]);
  assert.equal(runtime.cases.reduce((sum, item) => sum + item.nodeIds.length, 0), 118);
  for (const [index, item] of runtime.cases.slice(0, matrixInventory.cases.length).entries()) {
    const sourceCase = matrixInventory.cases[index];
    const chromeCase = chromeCases.find(({ id }) => id === item.id);
    assert.ok(chromeCase, `${item.id}: Chrome DPR 1 case`);
    assert.equal(item.index, index);
    assert.equal(item.rootId, sourceCase.rootId);
    assert.deepEqual(item.nodeIds, sourceCase.nodeIds);
    assert.deepEqual(item.nodeIds, chromeCase.nodes.map(({ id }) => id));
    assert.deepEqual(item.parentIds, chromeCase.nodes.map(({ parentId }) => parentId));
    assert.ok(item.source.length <= 16 * 1024, `${item.id}: source limit`);
    assert.ok(item.source.includes(`sceneId:'${item.id}'`), `${item.id}: source marker`);
    assert.ok(item.source.includes('document.createElement'), `${item.id}: DOM construction source`);
    assert.equal(item.source.includes('writing-mode:'), false,
      `${item.id}: 기본 writing mode는 지원되지 않는 inline 선언으로 합성하지 않습니다`);
    assert.equal(item.source.includes('align-content:'), false,
      `${item.id}: 현재 profile에서 지원하지 않는 grid alignment는 inline 선언으로 합성하지 않습니다`);
    assert.equal(item.source.includes('justify-content:'), false,
      `${item.id}: 현재 profile에서 지원하지 않는 grid alignment는 inline 선언으로 합성하지 않습니다`);
    assert.equal(item.oracle.product, reference.oracle.product);
    assert.equal(item.oracle.sceneId, item.id);
  }
  for (const item of runtime.cases.slice(matrixInventory.cases.length,
    matrixInventory.cases.length + supplementalIds.length)) {
    const observed = supplementalObservation.observations[item.id];
    assert.ok(observed, `${item.id}: Chrome 156 observation`);
    assert.equal(item.oracle.product, 'Chrome/156.0.0.0');
    assert.equal(item.oracle.sceneId, item.id);
    assert.equal(item.oracle.rootId, item.rootId);
    assert.deepEqual(item.nodeIds, observed.nodes.map(({ id }) => id));
    assert.deepEqual(item.oracle.nodes.map(({ id, rect }) => ({ id, rect })), observed.nodes);
    assert.equal(item.expected.outcome, 'success');
  }
  assert.deepEqual(supplementalObservation.observations['area-auto-tracks-auto'].root,
    { width: 50, height: 30 });
  assert.equal(cycleObservation.schema,
    'spinon-css-c11-3-3-area-auto-track-list-cycle-chrome-observation/v1');
  assert.equal(cycleObservation.capture.tool, 'agent-browser');
  assert.equal(cycleObservation.capture.browserVersion, '156.0.0.0');
  assert.equal(cycleObservation.fixture.sha256, sha256(cycleHtmlBytes));
  assert.equal(cycleObservation.screenshot.sha256, sha256(cycleScreenshotBytes));
  const cycleScreenshot = decodeRgbaPng(cycleScreenshotBytes);
  for (const item of runtime.cases.slice(matrixInventory.cases.length + supplementalIds.length)) {
    const observed = cycleObservation.observations[item.id];
    assert.ok(observed, `${item.id}: Agent Browser Chrome observation`);
    assert.equal(item.oracle.product, 'Chrome/156.0.0.0');
    assert.equal(item.oracle.sceneId, item.id);
    assert.equal(item.oracle.rootId, item.rootId);
    assert.deepEqual(item.nodeIds, observed.nodes.map(({ id }) => id));
    assert.deepEqual(item.oracle.nodes.map(({ id, rect }) => ({ id, rect })), observed.nodes);
    assert.deepEqual(item.oracle.rootRect, {
      x: 0, y: 0, width: observed.root.width, height: observed.root.height,
    });
    assert.equal(item.expected.outcome, 'success');
  }
  assert.equal(runtime.cases[0].expected.outcome, 'success');
  assert.equal(runtime.cases.filter(({ expected }) => expected.outcome === 'cascade-fallback').length, 5);
  assert.equal(runtime.cases.filter(({ expected }) => expected.outcome === 'expected-rejection').length, 0);
  const trackMismatch = runtime.cases.find(({ id }) => id === 'area-track-count-mismatch');
  assert.equal(trackMismatch.expected.outcome, 'success');
  assert.equal(trackMismatch.oracle.product, 'Chrome/154.0.8037.92');
  const unknownArea = runtime.cases.find(({ id }) => id === 'unknown-area-implicit-tracks');
  assert.deepEqual(unknownArea.expected, { outcome: 'success' });
  assert.deepEqual(unknownArea.oracle.nodes, [
    { id: 'unknown-area-grid', rect: { x: 0, y: 0, width: 160, height: 120 } },
    { id: 'unknown-area-item', rect: { x: 160, y: 120, width: 0, height: 0 } },
  ]);
  assert.equal(runtimeInventory.supersedes.path, paths.inventory);
  assert.equal(runtimeInventory.supersedes.sha256, sha256(inventoryBytes));
  assert.equal(inventory.cases.find(({ id }) => id === 'unknown-area-implicit-tracks').expectedSpinonOutcome,
    'reject-unsupported-implicit-area-tracks');
  assert.equal(historicalRuntime.cases.find(({ id }) => id === 'unknown-area-implicit-tracks').expected.outcome,
    'expected-rejection');
  assert.equal(runtimeInventory.cases.find(({ id }) => id === 'unknown-area-implicit-tracks').expectedSpinonOutcome,
    'success');
  const builder = builderBytes.toString('utf8');
  assert.ok(builder.includes("from 'parse5'"));
  assert.ok(builder.includes("process.argv.includes('--check')"));
  assert.ok(builder.includes("writeFile(outputFile, output)"));
  assert.equal(mobile.schema, 'spinon-css-c11-3-3-grid-template-areas-mobile-surface-pixel-matrix/v1');
  assert.equal(mobile.sceneId, 'basic-rectangles');
  assert.equal(mobile.inputs.runtimeMatrix.sha256, sha256(historicalRuntimeBytes));
  assert.equal(mobile.inputs.reference.sha256, sha256(await read(paths.reference)));
  assert.equal(mobile.oracle.referenceSha256, sha256(await read(paths.reference)));
  const screenshot = await read(mobile.oracle.screenshotPath);
  assert.equal(sha256(screenshot), mobile.oracle.screenshotSha256);
  assert.deepEqual(mobile.samples.map(({ id, rgba }) => [id, rgba]), [
    ['grid-background', [16, 24, 32, 255]],
    ['item-blue', [62, 99, 221, 255]],
  ]);
  assert.equal(cyclePixels.schema,
    'spinon-css-c11-3-3-area-auto-track-list-cycle-mobile-pixel-matrix/v1');
  assert.equal(cyclePixels.inputs.runtimeMatrix.sha256, sha256(historicalRuntimeBytes));
  assert.equal(cyclePixels.inputs.chromeObservation.sha256, sha256(cycleObservationBytes));
  assert.equal(cyclePixels.oracle.observationSha256, sha256(cycleObservationBytes));
  assert.equal(cyclePixels.oracle.screenshotSha256, sha256(cycleScreenshotBytes));
  assert.equal(cycleScreenshot.width, cyclePixels.oracle.viewportCssPx.width);
  assert.equal(cycleScreenshot.height, cyclePixels.oracle.viewportCssPx.height);
  assert.deepEqual(cyclePixels.scenes.map(({ sceneId }) => sceneId), cycleIds);
  let cycleSampleCount = 0;
  for (const scene of cyclePixels.scenes) {
    const origin = cycleObservation.observations[scene.sceneId].screenshotRect;
    assert.deepEqual(scene.oracleScreenshotOriginCssPx, { x: origin.x, y: origin.y });
    for (const sample of scene.samples) {
      assert.ok(sample.xCssPx >= 0 && sample.xCssPx < cyclePixels.runtime.viewportCssPx.width);
      assert.ok(sample.yCssPx >= 0 && sample.yCssPx < cyclePixels.runtime.viewportCssPx.height);
      const rgba = cycleScreenshot.getPixel(
        sample.xCssPx + origin.x, sample.yCssPx + origin.y);
      assert.deepEqual(rgba, sample.rgba, `${scene.sceneId}/${sample.id}: Chrome screenshot pixel`);
      cycleSampleCount++;
    }
  }
  assert.equal(cycleSampleCount, 28);
  const androidDemo = androidDemoBytes.toString('utf8');
  const cycleColorSwitch = androidDemo.match(
    /if \(c1133TemplateAreas\) \{\s*switch \(sampleId\) \{([\s\S]*?)\n\s*default:/,
  )?.[1];
  assert.ok(cycleColorSwitch, 'Android C11.3.3 pixel oracle color mapping');
  for (const sample of cyclePixels.scenes.flatMap(({ samples }) => samples)) {
    assert.ok(cycleColorSwitch.includes(`case "${sample.id}":`),
      `Android pixel oracle sample id ${sample.id}`);
    assert.ok(cycleColorSwitch.includes(
      `return new int[] {${sample.rgba.slice(0, 3).join(', ')}};`,
    ), `Android pixel oracle color ${sample.id}`);
  }
  const capture = captureBytes.toString('utf8');
  assert.ok(capture.includes('frames.length, 4'));
  assert.ok(capture.includes('android-physical [serial]'));
  assert.ok(capture.includes('android-avd'));
  assert.ok(capture.includes('SM-S731N'));
  assert.ok(capture.includes('selectAndroidPhysicalSerial'));
  assert.ok(capture.includes('selectAndroidAVDSerial'));
  assert.ok(capture.includes('target ==='));
  assert.ok(capture.includes('assert.equal(sdk, 37'));
  assert.ok(capture.includes('/SwiftShader/i'));
  assert.ok(deviceHelperBytes.toString('utf8').includes("model === 'SM-S731N'"));
  assert.ok(capture.includes("'emulator-5554'"));
  const scripts = JSON.parse((await read('package.json')).toString('utf8')).scripts;
  assert.equal(scripts['css:capture:c11-3-3-grid-template-areas:android-physical-matrix'],
    'node tools/css-reference/capture-c11-3-3-grid-template-areas-runtime-matrix.mjs android-physical');
  assert.equal(scripts['css:capture:c11-3-3-grid-template-areas:android-avd'],
    'node tools/css-reference/capture-c11-3-3-grid-template-areas-runtime-matrix.mjs android-avd');
  assert.equal(scripts['css:capture:c11-3-3-grid-template-areas:android-avd:single'],
    'node tools/css-reference/capture-c11-3-3-grid-template-areas-runtime.mjs android-avd');
  assert.equal(scripts['css:capture:c11-3-3-grid-template-areas:android-avd:revision-reuse'],
    'node tools/css-reference/capture-c11-3-3-grid-template-areas-revision-reuse.mjs android-avd');
  const matrixCapture = matrixCaptureBytes.toString('utf8');
  assert.ok(matrixCapture.includes("['android-avd', 'android-physical'].includes(target)"));
  assert.ok(matrixCapture.includes('runNegativeControls'));
  assert.ok(matrixCapture.includes('selectAndroidPhysicalSerial'));
  assert.ok(matrixCapture.includes("serial: '[REDACTED]'"));
  assert.ok(matrixCapture.includes('assert.equal(sdk, 36'));
  assert.ok(matrixCapture.includes("if (target === 'android-avd')"));
  assert.ok(matrixCapture.includes('screenSha256 = null'));
  assert.ok(matrixCapture.includes('function formatLog(log)'));
  assert.ok(matrixCapture.includes('scene.oracle ?? observation.cases.find(({ id }) => id === scene.id)'));
  assert.ok(matrixCapture.includes('maximumGeometryErrorCssPx'));
  assert.ok(matrixCapture.includes('paint_node_order'));
  assert.ok(matrixCapture.includes('SPINON_C1133_GRID_REJECTION_SUMMARY'));
  assert.ok(matrixCapture.includes("outcome: 'expected-rejection', rejection:"));
  assert.ok(matrixCapture.includes('compareC11AndroidPixelCopyScene'));
  assert.ok(matrixCapture.includes('cyclePixelScenes'));
  assert.ok(matrixCapture.includes('cyclePixelComparisons'));
  assert.ok(matrixCapture.includes('pixelcopy-${scene.id}.png'));
  assert.ok(matrixCapture.includes("'SPINON_ENABLE_C04_RUNTIME_GPU=1'"));
  assert.ok(matrixCapture.includes("commit: command('git', ['rev-parse', 'HEAD'])"));
  assert.ok(matrixCapture.includes('androidRuntimeSourceSha256:'));
  assert.ok(matrixCapture.includes('captureScriptSha256:'));
  assert.ok(matrixCapture.includes('- Source commit:'));
  const iosMatrixCapture = iosMatrixCaptureBytes.toString('utf8');
  assert.ok(iosMatrixCapture.includes('cyclePixelScenes'));
  assert.ok(iosMatrixCapture.includes('cyclePixelComparisons'));
  assert.ok(iosMatrixCapture.includes('ios-track-list-cycle-screen.png'));
  assert.ok(iosMatrixCapture.includes('compareC11IOSMetalPixelLogs'));
  const revisionReuseCapture = revisionReuseCaptureBytes.toString('utf8');
  assert.ok(revisionReuseCapture.includes("assert.equal(target, 'android-avd'"));
  assert.ok(revisionReuseCapture.includes('same_host=true first_status=0 second_status=0'));
  assert.ok(revisionReuseCapture.includes('second_document_revision'));
  assert.ok(revisionReuseCapture.includes('second_render_tree_revision'));
  assert.ok(capture.includes('--spinon-c1133-grid-template-areas'));
  assert.ok(appDelegateBytes.toString('utf8')
    .includes('arguments.contains("--spinon-c1133-grid-template-areas")'));
  const androidMainActivity = androidMainActivityBytes.toString('utf8');
  assert.ok(androidMainActivity.includes('getBooleanExtra("spinon_c1133_grid_template_areas", false)'));
  assert.ok(androidMainActivity.includes('isC1133GridTemplateAreasScene(c11GridCase)'));
  assert.ok(androidMainActivity.includes('|| c1133GridTemplateAreasRequested'));
  assert.ok(androidMainActivity.includes('SPINON_C1133_GRID_ARGUMENT_ERROR'));
  const sceneIds = runtime.cases.map(({ id }) => id);
  const runtimeSceneIds = [...sceneIds, 'unknown-area-fixed-implicit-tracks',
    'positive-missing-occurrences', 'negative-missing-occurrences', 'backward-span-one',
    'backward-span-two', 'backward-span-partial-explicit', 'forward-span-two',
    'backward-span-uses-authored-name'];
  const mainActivityAllowlist = androidMainActivity.match(
    /C1133_GRID_TEMPLATE_AREAS_SCENE_IDS = \{([\s\S]*?)\};/,
  )?.[1];
  assert.ok(mainActivityAllowlist);
  assert.deepEqual([...mainActivityAllowlist.matchAll(/"([^"]+)"/g)].map(([, id]) => id),
    runtimeSceneIds);
  const androidAllowlist = androidDemo.match(/C1133_GRID_TEMPLATE_AREAS_SCENE_IDS = \{([\s\S]*?)\};/)?.[1];
  assert.ok(androidAllowlist);
  assert.deepEqual([...androidAllowlist.matchAll(/"([^"]+)"/g)].map(([, id]) => id), runtimeSceneIds);
  assert.ok(androidDemo.includes('host, c1133GridTemplateAreasSceneId'));
  assert.ok(androidDemo.includes('scheduleRuntimeInitializationForReadySurface'));
  const iosDemo = iosDemoBytes.toString('utf8');
  const iosAllowlist = iosDemo.match(/c1133GridTemplateAreasSceneIDs: Set<String> = \[([\s\S]*?)\]/)?.[1];
  assert.ok(iosAllowlist);
  assert.deepEqual([...iosAllowlist.matchAll(/"([^"]+)"/g)].map(([, id]) => id), runtimeSceneIds);
  assert.ok(iosDemo.includes('--spinon-c1133-grid-scene='));
  assert.ok(iosDemo.includes('sceneID: c1133GridTemplateAreasSceneID'));
  assert.ok(iosDemo.includes('c1133TrackListCyclePixelSamples'));
  assert.ok(androidDemo.includes('c1133TrackListCyclePixelSamples'));
});

test('unknown-area fixed implicit tracks root 바깥 paint가 Chrome geometry와 pixel에 고정된다', async () => {
  const fixturePath = 'tests/fixtures/css/c11/unknown-area-fixed-implicit-tracks-mobile-pixels-v1.json';
  const referencePath = 'tests/fixtures/css/references/c11-3-3-unknown-named-area-implicit-lines-precomparison-v1.json';
  const [fixtureBytes, referenceBytes, androidDemoBytes, iosDemoBytes,
    androidCaptureBytes, iosCaptureBytes] = await Promise.all([
    read(fixturePath), read(referencePath),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
    read('tools/css-reference/capture-c11-3-3-grid-template-areas-runtime-matrix.mjs'),
    read('tools/css-reference/capture-c11-3-3-grid-template-areas-ios-simulator-matrix.mjs'),
  ]);
  const fixture = JSON.parse(fixtureBytes.toString('utf8'));
  const fixedReference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(fixture.schema, 'spinon-css-c11-3-3-unknown-area-overflow-mobile-pixels/v1');
  assert.equal(fixture.sceneId, 'unknown-area-fixed-implicit-tracks');
  assert.equal(fixture.inputs.reference.sha256, sha256(referenceBytes));
  assert.deepEqual(fixture.runtime.viewportCssPx, { width: 320, height: 240 });
  assert.equal(fixture.oracle.referenceId, fixedReference.referenceId);
  assert.equal(fixture.oracle.product, 'Chrome/154.0.8037.92');
  assert.deepEqual(fixture.samples.map(({ id }) => id), [
    'grid-interior', 'known-area', 'unknown-area-overflow',
  ]);
  assert.deepEqual(fixture.excludedReferenceSamples, [{
    id: 'outside-root-background',
    referenceRgba: [255, 255, 255, 255],
    reason: 'Chrome paints the uncovered viewport white while the native runtime surface backdrop is host-owned. This C11.3.3 comparison isolates the Grid root and its visible overflow item; page backdrop color is tracked separately.',
  }]);

  const observation = fixedReference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  const chromeCase = caseAt(observation, 'unknown-template-area-with-sibling');
  assert.deepEqual(chromeCase.nodes.map(({ rect }) => rect), [
    { x: 0, y: 0, width: 160, height: 120 },
    { x: 0, y: 0, width: 80, height: 60 },
    { x: 175, y: 125, width: 15, height: 5 },
  ]);
  const screenshot = await read(fixture.oracle.screenshotPath);
  assert.equal(sha256(screenshot), fixture.oracle.screenshotSha256);
  const png = decodeRgbaPng(screenshot);
  assert.deepEqual([png.width, png.height], [320, 240]);
  for (const sample of fixture.samples) {
    assert.deepEqual(png.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba, sample.id);
  }
  assert.deepEqual(fixture.samples.find(({ id }) => id === 'unknown-area-overflow').rgba,
    [229, 72, 77, 255]);
  assert.deepEqual(png.getPixel(170, 127), [255, 255, 255, 255],
    'Chrome page backdrop reference sample');

  const android = androidDemoBytes.toString('utf8');
  const ios = iosDemoBytes.toString('utf8');
  assert.ok(android.includes('unknown-area-fixed-implicit-tracks'));
  assert.ok(android.includes('unknown-area-overflow'));
  assert.ok(android.includes('case "unknown-area-overflow"'));
  assert.ok(android.includes('unsupported_grid_auto_rows'));
  assert.ok(ios.includes('unknown-area-fixed-implicit-tracks'));
  assert.ok(ios.includes('unknown-area-overflow'));
  assert.ok(ios.includes('unknown-area-overflow'));
  assert.ok(ios.includes('unsupported_grid_auto_rows'));
  assert.ok(androidCaptureBytes.toString('utf8').includes('captureUnknownAreaOverflow'));
  assert.ok(iosCaptureBytes.toString('utf8').includes('unknownAreaOverflow'));
});

test('leading named implicit line 모바일 fixture가 oracle geometry와 양 플랫폼 제한 경로에 고정된다', async () => {
  const fixturePath = 'tests/fixtures/css/c11/grid-leading-named-implicit-lines-mobile-pixels-v1.json';
  const inventoryPath = 'tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133-inventory.json';
  const referencePath = 'tests/fixtures/css/references/c11-3-3-leading-named-implicit-lines-precomparison-v1.json';
  const [fixtureBytes, inventoryBytes, referenceBytes, rustFixtureBytes,
    mainActivityBytes, androidDemoBytes, iosDemoBytes, androidCaptureBytes, iosCaptureBytes] = await Promise.all([
    read(fixturePath), read(inventoryPath), read(referencePath),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c11_3_3.rs'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
    read('tools/css-reference/capture-c11-3-3-grid-template-areas-runtime-matrix.mjs'),
    read('tools/css-reference/capture-c11-3-3-grid-template-areas-ios-simulator-matrix.mjs'),
  ]);
  const fixture = JSON.parse(fixtureBytes.toString('utf8'));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(fixture.schema,
    'spinon-css-c11-3-3-leading-named-implicit-lines-mobile-pixels/v1');
  assert.equal(fixture.oracle.referenceSha256, sha256(referenceBytes));
  assert.equal(fixture.oracle.referenceId, reference.referenceId);
  assert.deepEqual(fixture.runtime.viewportCssPx, { width: 320, height: 240 });
  assert.deepEqual(fixture.scenes.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  assert.deepEqual(reference.cases.map(({ id }) => id), fixture.scenes.map(({ id }) => id));
  for (let index = 0; index < fixture.scenes.length; index++) {
    const scene = fixture.scenes[index];
    const expected = inventory.cases[index];
    const oracle = reference.cases[index];
    assert.deepEqual(scene.columnsPx, expected.columnsPx, scene.id);
    assert.deepEqual(scene.rootRect, oracle.rootRect, scene.id);
    assert.deepEqual(scene.itemRect, oracle.itemRect, scene.id);
    assert.equal(scene.rootRect[2], scene.columnsPx.reduce((sum, size) => sum + size, 0), scene.id);
    assert.equal(scene.samples.length, 1, scene.id);
    const [sample] = scene.samples;
    assert.equal(sample.nodeId, scene.itemId, scene.id);
    assert.ok(sample.xCssPx >= scene.itemRect[0]
      && sample.xCssPx < scene.itemRect[0] + scene.itemRect[2], `${scene.id}: x sample outside item`);
    assert.ok(sample.yCssPx >= scene.itemRect[1]
      && sample.yCssPx < scene.itemRect[1] + scene.itemRect[3], `${scene.id}: y sample outside item`);
    assert.deepEqual(sample.rgba, [40, 102, 246, 255], scene.id);
  }

  const sceneIds = fixture.scenes.map(({ id }) => id);
  const rustFixture = rustFixtureBytes.toString('utf8');
  const androidMain = mainActivityBytes.toString('utf8');
  const android = androidDemoBytes.toString('utf8');
  const ios = iosDemoBytes.toString('utf8');
  const androidCapture = androidCaptureBytes.toString('utf8');
  const iosCapture = iosCaptureBytes.toString('utf8');
  for (const source of [rustFixture, androidMain, android, ios]) {
    for (const id of sceneIds) assert.ok(source.includes(id), `${id}: scene route 누락`);
  }
  const leadingNamedSourceStart = rustFixture.indexOf('const C11_3_3_LEADING_NAMED_IMPLICIT_LINE_SOURCES:');
  const leadingNamedSourceEnd = rustFixture.indexOf('\nconst C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS',
    leadingNamedSourceStart);
  assert.ok(leadingNamedSourceStart >= 0 && leadingNamedSourceEnd > leadingNamedSourceStart);
  const leadingNamedSources = rustFixture.slice(leadingNamedSourceStart, leadingNamedSourceEnd);
  assert.equal((leadingNamedSources.match(/width:\d+px/g) ?? []).length, 7);
  for (const scene of fixture.scenes) {
    assert.ok(leadingNamedSources.includes(`width:${scene.rootRect[2]}px`), scene.id);
  }
  assert.equal((leadingNamedSources.match(/background-color:#2866f6/g) ?? []).length, 7);
  assert.ok(android.includes('c1133LeadingNamedLineItemId'));
  assert.ok(ios.includes('c1133LeadingNamedImplicitPixelSamples'));
  assert.ok(android.includes('named-implicit-item'));
  assert.ok(android.includes('if ("named-implicit-item".equals(sampleId)) {\n            return new int[] {40, 102, 246};'));
  assert.ok(ios.includes('named-implicit-item'));
  assert.ok(androidCapture.includes(fixturePath));
  assert.ok(androidCapture.includes('captureLeadingNamedImplicitLines'));
  assert.ok(androidCapture.includes('compareC11AndroidPixelCopyScene'));
  assert.ok(androidCapture.includes("const sceneRunID = `${runID}-leading-${String(index).padStart(2, '0')}`;"));
  assert.ok(androidCapture.includes('assert.match(sceneRunID, /^[A-Za-z0-9_-]{1,80}$/'));
  assert.ok(iosCapture.includes(fixturePath));
  assert.ok(iosCapture.includes('captureLeadingNamedImplicitLines'));
  assert.ok(iosCapture.includes('compareC11IOSMetalPixelLogs'));
  assert.ok(iosCapture.includes('ios-leading-'));
});

test('Android capture는 고정 실기기를 자동 선택하고 명시 대상·중복·AVD를 분리한다', () => {
  const devices = [
    'List of devices attached',
    'emulator-5554 device product:generic model:sdk_gphone64_arm64 device:emu64a transport_id:1',
    'R3CY123ABC device product:dm2qks model:SM_S731N device:dm2q transport_id:2',
  ].join('\n');
  assert.equal(selectAndroidPhysicalSerial(devices), 'R3CY123ABC');
  assert.equal(selectAndroidPhysicalSerial(devices, 'R3CY123ABC'), 'R3CY123ABC');
  assert.throws(() => selectAndroidPhysicalSerial(devices, 'emulator-5554'), /실기기 SM-S731N이 필요합니다/);
  assert.throws(() => selectAndroidPhysicalSerial(devices.replace('SM_S731N', 'SM_S731B')),
    /SM-S731N 실기기를 찾지 못했습니다/);
  assert.throws(() => selectAndroidPhysicalSerial(`${devices}\nR3CY456DEF device product:dm2qks model:SM-S731N device:dm2q`),
    /여러 대 연결/);
});

test('Android AVD capture는 emulator serial만 허용하고 실기기와 다중 대상을 거부한다', () => {
  const devices = [
    'List of devices attached',
    'emulator-5554 device product:sdk_gphone64_arm64 model:sdk_gphone64_arm64 device:emu64a transport_id:1',
    'R3CY123ABC device product:dm2qks model:SM_S731N device:dm2q transport_id:2',
  ].join('\n');
  assert.equal(selectAndroidAVDSerial(devices), 'emulator-5554');
  assert.equal(selectAndroidAVDSerial(devices, 'emulator-5554'), 'emulator-5554');
  assert.throws(() => selectAndroidAVDSerial(devices, 'R3CY123ABC'), /serial 형식이 아닙니다/);
  assert.throws(() => selectAndroidAVDSerial(devices, 'emulator-5556'), /연결되지 않았습니다/);
  assert.throws(() => selectAndroidAVDSerial(devices.replace('emulator-5554 device', 'emulator-5554 offline')),
    /AVD를 찾지 못했습니다/);
  assert.throws(() => selectAndroidAVDSerial('List of devices attached'), /AVD를 찾지 못했습니다/);
  assert.throws(() => selectAndroidAVDSerial([devices, 'emulator-5556 device product:sdk_gphone64_arm64'].join('\n')),
    /여러 대 연결/);
});

test('area가 없는 bare ident 기준은 pinned Chrome 154 첫 named line에 고정된다', async () => {
  const capture = (await read('tools/css-reference/capture-c11-3-3-bare-ident-no-area-precomparison.mjs'))
    .toString('utf8');
  assert.ok(capture.includes("assert.equal(browserVersion.product, 'Chrome/154.0.8037.92')"));
  assert.ok(capture.includes("assert.equal(browserVersion.revision, '@334b65d254ccc35df4fca82706d1753227b01039')"));
  assert.ok(capture.includes("assert.equal(captureResult.result.computed.gridTemplateAreas, 'none')"));
  assert.ok(capture.includes("{ x: 40, y: 30, width: 20, height: 20 }"));
  const scripts = JSON.parse((await read('package.json')).toString('utf8')).scripts;
  assert.equal(scripts['css:reference:c11-3-3-bare-ident-no-area'],
    'node tools/css-reference/capture-c11-3-3-bare-ident-no-area-precomparison.mjs');
});
