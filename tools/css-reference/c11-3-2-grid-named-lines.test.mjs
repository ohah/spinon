import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-named-lines-c1132-inventory.json',
  html: 'tests/fixtures/css/c11/grid-named-lines-c1132.html',
  capture: 'tools/css-reference/capture-c11-3-2-grid-named-lines.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-3-2-grid-named-lines-precomparison-v1.json',
  wptRun: 'spec/internal/evidence/c11-3-2-named-lines-precomparison-2026-10-14/wpt-chrome154.0.8037.92-run02.json',
  androidEntry: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java',
  androidDemo: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  iosAppDelegate: 'platforms/ios/Sources/AppDelegate.swift',
  iosDemo: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
};
const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventory = JSON.parse((await read(paths.inventory)).toString('utf8'));
const reference = JSON.parse((await read(paths.reference)).toString('utf8'));
const wptRun = JSON.parse((await read(paths.wptRun)).toString('utf8'));
const caseAt = (observation, id) => observation.cases.find((item) => item.id === id);

test('기준 자료가 고정 fixture·도구·Chrome 실행 파일·WPT revision에 연결된다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper),
  ]);
  assert.equal(inventory.schema, 'spinon-css-c11-3-2-grid-named-lines-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-c11-3-2-grid-named-lines-precomparison/v1');
  assert.equal(reference.fixture.inventoryPath, paths.inventory);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.ok(reference.referenceId.endsWith(
    `${sha256(inventoryBytes).slice(0, 12)}-${sha256(htmlBytes).slice(0, 12)}-${sha256(captureBytes).slice(0, 12)}`,
  ));
  assert.equal(reference.oracle.name, 'Google Chrome for Testing');
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.92');
  assert.equal(reference.oracle.revision, '@334b65d254ccc35df4fca82706d1753227b01039');
  assert.equal(reference.oracle.executableSha256.length, 64);
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(inventory.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
  assert.deepEqual(reference.wpt, inventory.wpt);
  assert.deepEqual(wptRun.results.map(({ status }) => status), ['PASS', 'PASS', 'OK']);
  assert.equal(wptRun.run_info.browser_version, '154.0.8037.92');
  assert.equal(wptRun.run_info.browser_channel, 'stable');
  assert.equal(wptRun.run_info.revision, inventory.wpt.revision);
  assert.ok(wptRun.results.flatMap(({ subtests }) => subtests).every(({ status }) => status === 'PASS'));
  assert.ok(inventory.wpt.candidates.every(({ classification }) =>
    classification.includes('Chrome oracle only')
      && classification.includes('does not establish Spinon conformance')));
});

test('15개 장면의 DOM 순서, 양쪽 DPR의 CSSOM·track 수·geometry가 보존된다', () => {
  assert.equal(inventory.cases.length, 15);
  assert.equal(inventory.cases.flatMap(({ nodeIds }) => nodeIds).length, 30);
  assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size, 30);
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);

  for (const observation of reference.observations) {
    assert.equal(observation.cases.length, inventory.cases.length);
    for (const fixtureCase of inventory.cases) {
      const actual = caseAt(observation, fixtureCase.id);
      assert.ok(actual, `scene이 없습니다: ${fixtureCase.id}`);
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodeIds);
      assert.equal(actual.rootId, fixtureCase.rootId);
      assert.equal(actual.viewport.deviceScaleFactor, observation.deviceScaleFactor);
      assert.deepEqual(actual.environment, {
        direction: inventory.environment.direction,
        writingMode: inventory.environment.writingMode,
      });
      assert.equal(actual.resolvedGrid.derivedFromCssomResolvedValues, true);
      assert.equal(actual.resolvedGrid.columns.trackSizes.length, fixtureCase.expectedColumns, fixtureCase.id);
      assert.equal(actual.resolvedGrid.columns.lineNameGroups.length, fixtureCase.expectedColumns + 1, fixtureCase.id);
      assert.equal(actual.resolvedGrid.rows.trackSizes.length, fixtureCase.expectedRows, fixtureCase.id);
      assert.equal(actual.resolvedGrid.rows.lineNameGroups.length, fixtureCase.expectedRows + 1, fixtureCase.id);
      for (const node of actual.nodes) {
        assert.ok(node.parentId === null || fixtureCase.nodeIds.includes(node.parentId));
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}`);
        }
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

test('named line occurrence, span 방향, repeat seam, row 축, conflict 처리가 Chrome 기준과 맞는다', () => {
  const dpr1 = reference.observations[0];
  const cases = new Map(inventory.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
  const itemFrame = (id) => caseAt(dpr1, id).nodes[1].rect;
  const assertFrame = (id, expected) => {
    const actual = itemFrame(id);
    for (const [field, value] of Object.entries(expected)) assert.equal(actual[field], value, `${id}/${field}`);
  };

  assertFrame('positive-occurrence', { x: 80, y: 0, width: 80, height: 40 });
  assertFrame('negative-occurrence', { x: 40, y: 0, width: 40, height: 40 });
  assertFrame('integer-before-name', { x: 80, y: 0, width: 80, height: 40 });
  assertFrame('named-span-forward', { x: 0, y: 0, width: 80, height: 40 });
  assertFrame('named-span-backward', { x: 80, y: 0, width: 80, height: 40 });
  assertFrame('named-span-name-before-count', { x: 0, y: 0, width: 80, height: 40 });
  assertFrame('same-line-duplicate-name', { x: 0, y: 0, width: 80, height: 40 });
  assertFrame('repeat-seam-name', { x: 40, y: 0, width: 40, height: 40 });
  assertFrame('row-axis-named-lines', { x: 0, y: 80, width: 40, height: 40 });
  assertFrame('reversed-line-conflict', { x: 0, y: 0, width: 160, height: 40 });
  assertFrame('equal-line-conflict', { x: 40, y: 0, width: 40, height: 40 });
  assertFrame('lone-named-span', { x: 0, y: 0, width: 40, height: 40 });

  assert.deepEqual(caseAt(dpr1, 'same-line-duplicate-name').resolvedGrid.columns.lineNameGroups[1],
    ['mid', 'mid']);
  assert.deepEqual(caseAt(dpr1, 'repeat-seam-name').resolvedGrid.columns.lineNameGroups[1],
    ['mid', 'mid']);
  assert.deepEqual(caseAt(dpr1, 'repeat-seam-name').resolvedGrid.columns.lineNameGroups[2],
    ['mid', 'end']);
  assert.equal(cases.get('positive-occurrence').expectedSpinonOutcome, 'success');
});

test('explicit grid 밖 named line 요청은 Chrome 결과와 지원 범위를 분리해 둔다', () => {
  const unsupported = inventory.cases.filter(({ expectedSpinonOutcome }) =>
    expectedSpinonOutcome === 'reject-unsupported-named-line');
  assert.deepEqual(unsupported.map(({ id }) => id), [
    'missing-name-implicit-line',
    'occurrence-shortage-implicit-line',
    'named-span-shortage-implicit-line',
  ]);
  for (const fixtureCase of unsupported) {
    const actual = caseAt(reference.observations[0], fixtureCase.id);
    assert.ok(Number.isFinite(actual.nodes[1].documentRect.x), fixtureCase.id);
    assert.ok(Number.isFinite(actual.nodes[1].documentRect.width), fixtureCase.id);
  }
  assert.ok(inventory.wpt.candidates.some(({ path, classification }) =>
    path.endsWith('grid-placement-unknown-named-grid-line-001.html')
      && classification.includes('implicit-line behavior is outside this implementation')));
});

test('DPR 1의 scene screenshot 경로와 hash가 실제 PNG 파일과 일치한다', async () => {
  assert.equal(reference.screenshots.length, inventory.cases.length);
  for (const screenshot of reference.screenshots) {
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, screenshot.caseId);
    assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', screenshot.caseId);
  }
});

test('Android·iOS C11.3.2 진입과 거부 경로가 고정 scene 및 run ID를 검사한다', async () => {
  const [androidEntry, androidDemo, iosAppDelegate, iosDemo] = await Promise.all([
    read(paths.androidEntry), read(paths.androidDemo), read(paths.iosAppDelegate), read(paths.iosDemo),
  ]);
  assert.match(androidEntry.toString('utf8'),
    /c1132GridNamedLinesRequested\s*&&\s*"positive-occurrence"\.equals\(c11GridCase\)/);
  assert.match(androidDemo.toString('utf8'),
    /isC1132GridNamedLinePixelSamples\(pixelMatrixSamples\)/);
  assert.match(androidDemo.toString('utf8'),
    /C1132_GRID_NAMED_LINE_REJECT_SCENE_ID/);
  const runtimeDispatch = iosAppDelegate.toString('utf8')
    .match(/let runRuntimeGpu =([\s\S]*?)\n\s*let runR05AttributionProbe/)?.[1];
  assert.ok(runtimeDispatch, 'iOS AppDelegate runtime GPU routing declaration is missing');
  assert.match(runtimeDispatch, /arguments\.contains\("--spinon-c1132-grid-named-lines"\)/);
  assert.match(iosDemo.toString('utf8'),
    /hasFixedC1132NamedLineSamples\(samples\)/);
  assert.match(iosDemo.toString('utf8'),
    /SPINON_C1132_GRID_REJECTION/);
  assert.match(iosDemo.toString('utf8'),
    /logC1132GridNamedLineFrames\(result\)/);
});
