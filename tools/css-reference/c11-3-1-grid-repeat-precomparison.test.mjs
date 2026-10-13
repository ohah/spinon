import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-template-repeat-c1131-inventory.json',
  html: 'tests/fixtures/css/c11/grid-template-repeat-c1131.html',
  capture: 'tools/css-reference/capture-c11-3-1-grid-repeat-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-3-1-grid-repeat-precomparison-v1.json',
  wptRun: 'spec/internal/evidence/c11-3-1-repeat-precomparison-2026-10-13/wpt-chrome154.0.8037.92-run02.json',
};
const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventory = JSON.parse((await read(paths.inventory)).toString('utf8'));
const reference = JSON.parse((await read(paths.reference)).toString('utf8'));
const wptRun = JSON.parse((await read(paths.wptRun)).toString('utf8'));
const caseAt = (observation, id) => observation.cases.find((item) => item.id === id);

test('기준은 고정한 fixture·도구·Chrome 실행 파일·WPT revision에 연결된다', async () => {
  const [inventoryBytes, htmlBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper),
  ]);
  assert.equal(inventory.schema, 'spinon-css-c11-3-1-grid-repeat-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-c11-3-1-grid-repeat-precomparison/v1');
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
  assert.deepEqual(wptRun.results.map(({ status }) => status), Array(5).fill('PASS'));
  assert.equal(wptRun.run_info.browser_version, '154.0.8037.92');
  assert.equal(wptRun.run_info.revision, inventory.wpt.revision);
  assert.ok(reference.wpt.candidates.every(({ stage }) =>
    stage.includes('Chrome oracle only') && stage.includes('does not establish Spinon conformance')));
});

test('12개 독립 장면의 tree·DPR·CSS px geometry·track 수가 보존된다', () => {
  assert.equal(inventory.cases.length, 12);
  assert.equal(inventory.cases.flatMap(({ nodeIds }) => nodeIds).length, 29);
  assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size, 29);
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
      for (const node of actual.nodes) {
        assert.ok(node.parentId === null || fixtureCase.nodeIds.includes(node.parentId));
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}`);
        }
      }
    }
  }
  const dpr1 = reference.observations[0];
  const dpr2 = reference.observations[1];
  for (const fixtureCase of inventory.cases) {
    const first = caseAt(dpr1, fixtureCase.id);
    const second = caseAt(dpr2, fixtureCase.id);
    assert.deepEqual(first.rootRect, second.rootRect, fixtureCase.id);
    assert.deepEqual(first.nodes, second.nodes, fixtureCase.id);
    assert.deepEqual(first.resolvedGrid, second.resolvedGrid, fixtureCase.id);
  }
});

test('repeat seam과 중복 line name의 Chrome resolved CSSOM 값이 명시된 순서를 유지한다', () => {
  const dpr1 = reference.observations[0];
  assert.deepEqual(caseAt(dpr1, 'repeat-one').resolvedGrid.columns.trackSizes, ['320px']);
  assert.deepEqual(caseAt(dpr1, 'no-repeat-control').resolvedGrid.columns.trackSizes, ['320px']);
  assert.deepEqual(caseAt(dpr1, 'multi-track-repeat').resolvedGrid.columns.trackSizes,
    ['20px', '80px', '40px', '36px', '40px', '36px', '20px']);
  assert.deepEqual(caseAt(dpr1, 'line-name-seam').resolvedGrid.columns.lineNameGroups, [
    ['outer-start'],
    ['outer-end', 'repeat-start', 'item-start'],
    ['seam'],
    ['item-end', 'item-start'],
    ['seam'],
    ['item-end', 'trailing'],
    ['outer-last'],
  ]);
  assert.deepEqual(caseAt(dpr1, 'duplicate-line-names').resolvedGrid.columns.lineNameGroups, [
    ['same', 'same'], ['same', 'same'], ['same', 'same'], ['end'],
  ]);
  assert.deepEqual(caseAt(dpr1, 'mixed-units-and-gaps').resolvedGrid.columns.trackSizes,
    ['50px', '64px', '55px', '64px', '55px']);
});

test('Chrome의 10,000 경계·거부 후보·CSSOM fallback을 Spinon 결과와 혼동하지 않는다', () => {
  const dpr1 = reference.observations[0];
  assert.equal(inventory.cases.find(({ id }) => id === 'limit-10000').expected, 'support');
  for (const id of ['limit-10000', 'limit-10001', 'auto-fill-outside-profile', 'minmax-outside-profile',
    'nested-repeat-invalid', 'zero-repeat-invalid']) {
    if (id === 'limit-10000') continue;
    assert.equal(inventory.cases.find(({ id: candidate }) => candidate === id).expected, 'reject', id);
    assert.ok(caseAt(dpr1, id).nodes[0].properties['grid-template-columns'].length > 0, id);
  }
  assert.equal(caseAt(dpr1, 'limit-10000').resolvedGrid.columns.trackSizes.length, 10_000);
  assert.equal(caseAt(dpr1, 'limit-10001').resolvedGrid.columns.trackSizes.length, 10_001);
  assert.equal(caseAt(dpr1, 'auto-fill-outside-profile').resolvedGrid.columns.trackSizes.length, 8);
  assert.equal(caseAt(dpr1, 'minmax-outside-profile').resolvedGrid.columns.trackSizes.length, 2);
  assert.equal(caseAt(dpr1, 'nested-repeat-invalid').nodes[0].properties['grid-template-columns'], '320px');
  assert.equal(caseAt(dpr1, 'zero-repeat-invalid').nodes[0].properties['grid-template-columns'], '320px');
});

test('DPR 1 화면 캡처 경로와 해시가 실제 파일과 일치한다', async () => {
  assert.equal(reference.screenshots.length, 12);
  for (const screenshot of reference.screenshots) {
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, screenshot.caseId);
    assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', screenshot.caseId);
  }
});
