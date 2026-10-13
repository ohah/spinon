import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';
import { parse } from 'parse5';
import test from 'node:test';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json',
  html: 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html',
  generator: 'tools/css-reference/build-c11-2-3-grid-auto-placement-fixture.mjs',
  capture: 'tools/css-reference/capture-c11-2-3-grid-auto-placement-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json',
  wptReport: 'spec/internal/evidence/c11-2-3-dense-column-placement-precomparison-2026-10-13/wpt-chrome-154-report.json',
  wptLog: 'spec/internal/evidence/c11-2-3-dense-column-placement-precomparison-2026-10-13/wpt-chrome-154-rerun.log',
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const read = (path) => readFile(join(repositoryRoot, path));
const attributes = (node) => new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
const children = (node) => (node.childNodes ?? []).filter(({ nodeName }) => nodeName !== '#comment');

function descendants(node, predicate) {
  const result = [];
  const visit = (current) => {
    if (predicate(current)) result.push(current);
    for (const child of children(current)) visit(child);
  };
  visit(node);
  return result;
}

function parentNodeId(node, memberIds) {
  let parent = node.parentNode;
  while (parent) {
    const id = attributes(parent).get('id');
    if (memberIds.has(id)) return id;
    parent = parent.parentNode;
  }
  return null;
}

const [inventoryBytes, htmlBytes, generatorBytes, captureBytes, helperBytes, referenceBytes, reportBytes, wptLogBytes] = await Promise.all([
  read(paths.inventory), read(paths.html), read(paths.generator), read(paths.capture), read(paths.helper),
  read(paths.reference), read(paths.wptReport), read(paths.wptLog),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const report = JSON.parse(reportBytes.toString('utf8'));
const parsedHtml = parse(htmlBytes.toString('utf8'));
const scenes = descendants(parsedHtml, (node) => attributes(node).has('data-scene'));
const sceneById = new Map(scenes.map((node) => [attributes(node).get('data-scene'), node]));

assert.equal(inventory.schema, 'spinon-css-c11-2-3-grid-auto-placement-inventory/v6');
assert.equal(inventory.fixtureId, 'C11-2-3-grid-auto-placement-precomparison-v6');
assert.equal(inventory.cases.length, 25);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').length, 21);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only').length, 4);
assert.deepEqual(scenes.map((node) => attributes(node).get('data-scene')), inventory.cases.map(({ id }) => id));

test('C11.2.3 fixture 생성 명령은 HTML과 inventory를 같은 입력으로 재현한다', () => {
  execFileSync(process.execPath, [paths.generator], { cwd: repositoryRoot, stdio: 'pipe' });
});

test('25개 장면의 preorder·NodeId·부모가 inventory와 HTML에서 일치한다', () => {
  const allNodeIds = [];
  for (const fixtureCase of inventory.cases) {
    const scene = sceneById.get(fixtureCase.id);
    assert.ok(scene, `scene 누락: ${fixtureCase.id}`);
    const nodes = descendants(scene, (node) => attributes(node).has('data-node'));
    const nodeIds = nodes.map((node) => attributes(node).get('id'));
    assert.deepEqual(nodeIds, fixtureCase.nodeIds, `${fixtureCase.id}: DOM preorder`);
    assert.equal(nodeIds[0], fixtureCase.rootId);
    const memberIds = new Set(nodeIds);
    assert.equal(parentNodeId(nodes[0], memberIds), null);
    for (const node of nodes) {
      const id = attributes(node).get('id');
      assert.ok(id, `${fixtureCase.id}: NodeId 필요`);
      allNodeIds.push(id);
      assert.equal(parentNodeId(node, memberIds), reference.observations[0].cases
        .find(({ id: caseId }) => caseId === fixtureCase.id).nodes
        .find(({ id: nodeId }) => nodeId === id).parentId);
    }
    const grids = descendants(scene, (node) => attributes(node).has('data-grid'));
    for (const grid of grids) {
      const id = attributes(grid).get('id');
      const style = new Map((attributes(grid).get('style') ?? '').split(';').map((declaration) => {
        const separator = declaration.indexOf(':');
        return separator > 0
          ? [declaration.slice(0, separator).trim(), declaration.slice(separator + 1).trim()]
          : null;
      }).filter(Boolean));
      assert.match(style.get('grid-template-columns') ?? '', /^\d+(?:\.\d+)?px(?:\s+\d+(?:\.\d+)?px)*$/, `${id}: columns`);
      assert.match(style.get('grid-template-rows') ?? '', /^\d+(?:\.\d+)?px(?:\s+\d+(?:\.\d+)?px)*$/, `${id}: rows`);
    }
  }
  assert.equal(new Set(allNodeIds).size, allNodeIds.length, 'scene 사이 NodeId 중복 금지');
  assert.ok(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only')
    .every(({ rejectionReason }) => typeof rejectionReason === 'string' && rejectionReason.length > 0));
});

test('고정 WPT revision과 Chrome runner 로그·세부 결과가 연결된다', () => {
  assert.equal(inventory.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
  assert.equal(inventory.wpt.chrome.product, 'Chrome/154.0.8037.98');
  assert.equal(inventory.wpt.chrome.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(inventory.wpt.candidates.map(({ path, sha256 }) => [path, sha256]), [
    ['css/css-grid/placement/grid-auto-flow-sparse-001.html', '9d30cb69227ab5bf80fda3f557b03eb89be7c9d5171de73ef7db8ecc5f80c0cf'],
    ['css/css-grid/placement/grid-auto-placement-fixed-row-column-001.html', '14c4a5c29bfd093473c7a5794637ea8d5aea44f4298c7a4f17df360d373762a2'],
    ['css/css-grid/placement/grid-auto-placement-implicit-tracks-001.html', '785012f37a838995bdd6491bd9d27034d21e2441152a067f050bc82fd993ad53'],
  ]);
  assert.equal(inventory.wpt.candidates.every(({ status }) => status === 'chrome-wpt-run-pass'), true);
  assert.deepEqual(report.results.map(({ test: path, status }) => [path, status]), inventory.wpt.candidates
    .map(({ path }) => [`/${path}`, 'OK']));
  assert.equal(report.results.reduce((total, result) => total + result.subtests.length, 0), 36);
  assert.equal(report.results.flatMap(({ subtests }) => subtests).every(({ status }) => status === 'PASS'), true);
  assert.match(wptLogBytes.toString('utf8'), /3 ran as expected\. 0 tests skipped\./);
  assert.equal(inventory.wpt.supportCss.sha256, 'b2f35862e7b8fa9dcae8f4f8d25add78d93f3e7993e08a4337030a4dd2d4425e');
});

test('Chrome reference가 정확한 입력 hash·캡처 도구·DPR 관측과 25개 screenshot을 담는다', async () => {
  assert.equal(reference.schema, 'spinon-css-c11-2-3-grid-auto-placement-precomparison/v7');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.fixtureGenerator, {
    path: paths.generator,
    sha256: sha256(generatorBytes),
  });
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(reference.wpt.candidates, inventory.wpt.candidates);
  assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  assert.equal(reference.screenshots.length, inventory.cases.length);
  assert.equal(reference.observations.every(({ cases }) => cases.length === inventory.cases.length), true);

  for (const screenshot of reference.screenshots) {
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, `${screenshot.caseId}: screenshot hash`);
    assert.equal(bytes.readUInt32BE(16), inventory.viewport.width, `${screenshot.caseId}: png width`);
    assert.equal(bytes.readUInt32BE(20), inventory.viewport.height, `${screenshot.caseId}: png height`);
  }

  const [base, scaled] = reference.observations;
  for (const fixtureCase of inventory.cases) {
    const baseCase = base.cases.find(({ id }) => id === fixtureCase.id);
    const scaledCase = scaled.cases.find(({ id }) => id === fixtureCase.id);
    assert.deepEqual(baseCase.nodes, scaledCase.nodes, `${fixtureCase.id}: DPR-invariant nodes`);
    assert.deepEqual(baseCase.gridContainers, scaledCase.gridContainers, `${fixtureCase.id}: DPR-invariant Grid`);
    assert.deepEqual(baseCase.nodes.map(({ id }) => id), fixtureCase.nodeIds);
    for (const node of baseCase.nodes) {
      for (const field of inventory.comparison.rectFields) assert.ok(Number.isFinite(node.rect[field]));
      assert.deepEqual(Object.keys(node.properties), inventory.comparison.computedProperties);
    }
  }
});

test('Chrome가 row·column dense의 hole backfill과 양의 implicit track을 분리해 관찰한다', () => {
  const observation = reference.observations[0];
  const position = (caseId, suffix) => {
    const fixtureCase = observation.cases.find(({ id }) => id === caseId);
    const node = fixtureCase.nodes.find(({ id }) => id.endsWith(`-${suffix}`));
    return [node.rect.x, node.rect.y];
  };
  assert.deepEqual(position('row-dense-hole-fill', 'small-c'), [108, 20]);
  assert.deepEqual(position('row-sparse-hole-control', 'small-c'), [108, 44]);
  assert.deepEqual(position('column-dense-hole-fill', 'small-c'), [20, 76]);
  assert.deepEqual(position('column-sparse-hole-control', 'small-c'), [68, 76]);

  const grid = (caseId) => observation.cases.find(({ id }) => id === caseId).gridContainers[0];
  assert.equal(grid('row-dense-positive-implicit-row').trackCounts.implicitRows, 2);
  assert.equal(grid('row-dense-positive-implicit-row').properties['grid-auto-rows'], '25px');
  assert.equal(grid('column-sparse-positive-implicit-column').trackCounts.implicitColumns, 2);
  assert.equal(grid('column-dense-positive-implicit-column').trackCounts.implicitColumns, 2);
  assert.equal(grid('column-dense-positive-implicit-column').properties['grid-auto-columns'], '28px');
  assert.equal(grid('nested-mixed-flow').properties['grid-auto-flow'], 'row');
  const nested = observation.cases.find(({ id }) => id === 'nested-mixed-flow').gridContainers[1];
  assert.equal(nested.properties['grid-auto-rows'], '18px');
  assert.equal(nested.trackCounts.implicitRows, 2);
  assert.equal(grid('auto-track-no-implicit').properties['grid-auto-rows'], 'auto');
  assert.equal(grid('auto-track-no-implicit').trackCounts.implicitRows, 0);

  const caseValue = (caseId) => observation.cases.find(({ id }) => id === caseId);
  assert.equal(grid('auto-span-at-track-limit').trackCounts.computedColumns, 10_000);
  assert.equal(grid('auto-span-at-track-limit').trackCounts.implicitColumns, 9_999);
  assert.equal(grid('auto-span-over-track-limit').trackCounts.computedColumns, 10_001);
  assert.deepEqual(inventory.cases.find(({ id }) => id === 'auto-span-at-track-limit').expectedRuntimeResult,
    { status: 'success', computedColumns: 10_000, maximum: 10_000 });
  assert.deepEqual(inventory.cases.find(({ id }) => id === 'auto-span-over-track-limit').expectedRuntimeResult,
    { status: 'reject', errorCode: 'GridTrackExpansionOverflow', maximum: 10_000 });
  const authoredStyle = (caseId) => caseValue(caseId).nodes[0].authoredStyle;
  assert.match(authoredStyle('reject-negative-implicit-line'), /(?:^|;)height:20px(?:;|$)/);
  assert.equal(grid('reject-negative-implicit-line').trackCounts.implicitColumns, 1);
  assert.doesNotMatch(authoredStyle('reject-indefinite-axis'), /(?:^|;)height:/);
  const hidden = caseValue('hidden-item-excluded');
  const hiddenNode = hidden.nodes.find(({ id }) => id.endsWith('-hidden'));
  const visible = hidden.nodes.find(({ id }) => id.endsWith('-visible-a'));
  assert.equal(hiddenNode.properties.display, 'none');
  assert.deepEqual(hiddenNode.rect, { x: 0, y: 0, width: 0, height: 0 });
  assert.deepEqual([visible.rect.x, visible.rect.y], [20, 20]);
});
