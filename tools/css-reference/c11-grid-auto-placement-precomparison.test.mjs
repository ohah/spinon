import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { access, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { parse } from 'parse5';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-auto-placement-inventory.json',
  html: 'tests/fixtures/css/c11/grid-auto-placement.html',
  capture: 'tools/css-reference/capture-c11-grid-auto-placement-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json',
};
const expectedWpt = [
  ['css/css-grid/placement/grid-layout-grid-span.html', '1be809e54f1b7398112923acd60ef423735934b528ff42c1a8f3b326adbebef7'],
  ['css/css-grid/placement/grid-negative-line-placement-001.html', 'e5d17120d87170282d2522ec483d751c6f7db9ac23ec09d2a540cce305d81d31'],
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const read = (path) => readFile(join(repositoryRoot, path));
const attrs = (node) => new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
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

function closestNodeParent(node, parentByNode) {
  let parent = node.parentNode;
  while (parent) {
    const id = attrs(parent).get('id');
    if (id && parentByNode.has(id)) return id;
    parent = parent.parentNode;
  }
  return null;
}

const [inventoryBytes, htmlBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
  read(paths.inventory), read(paths.html), read(paths.capture), read(paths.helper), read(paths.reference),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const parsedHtml = parse(htmlBytes.toString('utf8'));
const allScenes = descendants(parsedHtml, (node) => attrs(node).has('data-scene'));
const allIds = new Set();
const expectedParents = new Map();
const expectedGridIds = new Map();
const expectedGridTracks = new Map();

assert.equal(inventory.schema, 'spinon-css-c11-2-2-grid-auto-placement-inventory/v2');
assert.equal(inventory.fixtureId, 'C11-2-2-grid-auto-placement-precomparison-v2');
assert.equal(inventory.cases.length, 15);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').length, 10);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only').length, 5);
assert.deepEqual(inventory.wpt.candidates.map(({ path, sha256 }) => [path, sha256]), expectedWpt);
assert.equal(inventory.wpt.candidates.every(({ status }) => status === 'source-hash-verified-not-run'), true);
assert.deepEqual(allScenes.map((scene) => attrs(scene).get('data-scene')), inventory.cases.map(({ id }) => id));

for (const fixtureCase of inventory.cases) {
  const scene = allScenes.find((candidate) => attrs(candidate).get('data-scene') === fixtureCase.id);
  assert.ok(scene, `HTML scene 누락: ${fixtureCase.id}`);
  const nodes = descendants(scene, (node) => attrs(node).has('data-node'));
  const nodeIds = nodes.map((node) => attrs(node).get('id'));
  assert.deepEqual(nodeIds, fixtureCase.nodeIds, `${fixtureCase.id}: data-node DOM preorder`);
  assert.equal(nodeIds[0], fixtureCase.rootId, `${fixtureCase.id}: 첫 data-node는 grid root`);
  for (const node of nodes) {
    const id = attrs(node).get('id');
    assert.ok(id, `${fixtureCase.id}: 모든 data-node에는 id가 필요합니다`);
    assert.equal(allIds.has(id), false, `중복 node id: ${id}`);
    allIds.add(id);
  }
  for (const node of nodes) {
    const id = attrs(node).get('id');
    expectedParents.set(id, closestNodeParent(node, new Set(nodeIds)));
  }
  expectedGridIds.set(fixtureCase.id, descendants(scene, (node) => attrs(node).has('data-grid'))
    .map((node) => attrs(node).get('id')));
  expectedGridTracks.set(fixtureCase.id, new Map(descendants(scene, (node) => attrs(node).has('data-grid'))
    .map((node) => {
      const id = attrs(node).get('id');
      const declarations = new Map((attrs(node).get('style') ?? '').split(';').map((declaration) => {
        const colon = declaration.indexOf(':');
        return colon > 0 ? [declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()] : null;
      }).filter(Boolean));
      const countTracks = (property) => {
        const value = declarations.get(property) ?? '';
        assert.match(value, /^\d+(?:\.\d+)?px(?:\s+\d+(?:\.\d+)?px)*$/, `${id}/${property}: authored px track list`);
        return value.split(/\s+/).length;
      };
      return [id, { explicitColumns: countTracks('grid-template-columns'), explicitRows: countTracks('grid-template-rows') }];
    })));
}

test('C11.2.2 기준 fixture가 15개 장면과 pinned WPT 후보를 분리한다', () => {
  assert.equal(new Set(allIds).size, allIds.size);
  assert.equal(inventory.cases.every(({ id, scope }) => Boolean(id) && Boolean(scope)), true);
  assert.equal(inventory.cases.filter(({ rejectionReason }) => rejectionReason).length, 5);
  assert.equal(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate')
    .every(({ rejectionReason }) => rejectionReason === undefined), true);
  assert.equal(inventory.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
});

test('Chrome 기준은 fixture·도구 hash와 각 DPR 관측을 완전하게 연결한다', async () => {
  assert.equal(reference.schema, 'spinon-css-c11-2-2-grid-auto-placement-precomparison/v3');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{
    path: paths.helper,
    sha256: sha256(helperBytes),
  }]);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(reference.environment.deviceScaleFactors, [1, 2]);
  assert.equal(reference.wpt.revision, inventory.wpt.revision);
  assert.deepEqual(reference.wpt.candidates, inventory.wpt.candidates);
  assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
  assert.equal(reference.screenshots.length, inventory.cases.length);

  for (const observation of reference.observations) {
    assert.equal(observation.cases.length, inventory.cases.length);
    assert.deepEqual(observation.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
    for (const fixtureCase of inventory.cases) {
      const actual = observation.cases.find(({ id }) => id === fixtureCase.id);
      assert.equal(actual.scope, fixtureCase.scope);
      assert.deepEqual(actual.nodes.map(({ id }) => id), fixtureCase.nodeIds);
      assert.equal(actual.nodes[0].id, fixtureCase.rootId);
      assert.deepEqual(actual.gridContainers.map(({ id }) => id), expectedGridIds.get(fixtureCase.id));
      for (const grid of actual.gridContainers) {
        const expected = expectedGridTracks.get(fixtureCase.id).get(grid.id);
        assert.ok(expected, `${fixtureCase.id}/${grid.id}: explicit track 기준 누락`);
        assert.equal(grid.trackCounts.explicitColumns, expected.explicitColumns);
        assert.equal(grid.trackCounts.explicitRows, expected.explicitRows);
        const propertyTrackCount = (property) => grid.properties[property] === 'none'
          ? 0
          : grid.properties[property].trim().split(/\s+/).length;
        assert.equal(grid.trackCounts.computedColumns, propertyTrackCount('grid-template-columns'));
        assert.equal(grid.trackCounts.computedRows, propertyTrackCount('grid-template-rows'));
        assert.equal(grid.trackCounts.implicitColumns,
          grid.trackCounts.computedColumns - grid.trackCounts.explicitColumns);
        assert.equal(grid.trackCounts.implicitRows,
          grid.trackCounts.computedRows - grid.trackCounts.explicitRows);
        assert.ok(grid.trackCounts.implicitColumns >= 0 && grid.trackCounts.implicitRows >= 0);
      }
      for (const node of actual.nodes) {
        assert.equal(node.parentId, expectedParents.get(node.id), `${fixtureCase.id}/${node.id}: parent`);
        assert.deepEqual(Object.keys(node.properties), inventory.comparison.computedProperties);
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}`);
        }
      }
    }
  }

  const comparable = (observation) => observation.cases.map(({ id, scope, rootId, rootRect, environment, gridContainers, nodes }) => (
    { id, scope, rootId, rootRect, environment, gridContainers, nodes }
  ));
  assert.deepEqual(comparable(reference.observations[0]), comparable(reference.observations[1]),
    'DPR 변경이 CSS px geometry·computed style·track 관측값을 바꾸면 안 됩니다');

  for (const screenshot of reference.screenshots) {
    const bytes = await read(screenshot.path);
    assert.equal(sha256(bytes), screenshot.sha256, `${screenshot.caseId}: screenshot hash`);
    assert.equal(bytes.subarray(1, 4).toString('ascii'), 'PNG');
    assert.equal(bytes.readUInt32BE(16), inventory.viewport.width, `${screenshot.caseId}: screenshot width`);
    assert.equal(bytes.readUInt32BE(20), inventory.viewport.height, `${screenshot.caseId}: screenshot height`);
  }
});

test('기준 scene이 definite-first, sparse cursor, gap, hidden, nested 동작을 명시한다', () => {
  const base = reference.observations[0].cases;
  const scene = (id) => base.find((candidate) => candidate.id === id);
  const node = (sceneId, id) => scene(sceneId).nodes.find((candidate) => candidate.id === id);
  const xy = (sceneId, id) => {
    const { x, y } = node(sceneId, id).rect;
    return [x, y];
  };
  const frame = (sceneId, id) => {
    const { x, y, width, height } = node(sceneId, id).rect;
    return [x, y, width, height];
  };

  assert.deepEqual(xy('definite-first-mixed', 'c1122-definite-first-fixed'), [20, 20]);
  assert.deepEqual(xy('definite-first-mixed', 'c1122-definite-first-auto-a'), [116, 20]);
  assert.deepEqual(xy('definite-first-mixed', 'c1122-definite-first-auto-b'), [20, 68]);
  assert.deepEqual(xy('positive-implicit-row', 'c1122-implicit-row-c'), [20, 68]);
  assert.deepEqual(xy('row-locked-auto-column', 'c1122-row-locked-b'), [75, 20]);
  assert.deepEqual(xy('column-locked-auto-row', 'c1122-column-locked-b'), [64, 44]);
  assert.deepEqual(xy('sparse-hole-not-backfilled', 'c1122-sparse-fixed'), [64, 20]);
  assert.deepEqual(xy('sparse-hole-not-backfilled', 'c1122-sparse-span'), [20, 44]);
  assert.deepEqual(frame('sparse-hole-not-backfilled', 'c1122-sparse-span'), [20, 44, 84, 18]);
  assert.deepEqual(xy('sparse-hole-not-backfilled', 'c1122-sparse-auto-b'), [108, 44]);
  assert.equal(scene('sparse-hole-not-backfilled').gridContainers[0].properties['grid-template-rows'], '20px 25px');
  assert.deepEqual(scene('sparse-hole-not-backfilled').gridContainers[0].trackCounts,
    { explicitColumns: 3, computedColumns: 3, implicitColumns: 0, explicitRows: 1, computedRows: 2, implicitRows: 1 });
  assert.deepEqual(xy('implicit-track-size-and-gap', 'c1122-gap-c'), [20, 45]);
  assert.equal(scene('implicit-track-size-and-gap').gridContainers[0].properties['grid-template-rows'], '20px 24px');
  assert.deepEqual(xy('hidden-child-excluded', 'c1122-hidden-visible-a'), [20, 20]);
  assert.deepEqual(xy('hidden-child-excluded', 'c1122-hidden-visible-b'), [116, 20]);
  assert.equal(node('hidden-child-excluded', 'c1122-hidden-item').properties.display, 'none');
  assert.deepEqual(xy('nested-independent-placement', 'c1122-nested-auto-fixed'), [116, 20]);
  assert.deepEqual(xy('nested-independent-placement', 'c1122-nested-auto-first'), [164, 20]);

  assert.equal(scene('reject-implicit-column').gridContainers[0].properties['grid-template-columns'], '40px 40px 0px 20px');
  assert.deepEqual(scene('reject-implicit-column').gridContainers[0].trackCounts,
    { explicitColumns: 2, computedColumns: 4, implicitColumns: 2, explicitRows: 1, computedRows: 1, implicitRows: 0 });
  assert.equal(scene('reject-auto-sized-implicit-row').gridContainers[0].properties['grid-template-rows'], '20px 20px');
  assert.equal(scene('reject-repeated-auto-row-pattern').gridContainers[0].properties['grid-template-rows'], '20px 16px 24px 16px');
  assert.deepEqual(xy('reject-repeated-auto-row-pattern', 'c1122-reject-pattern-g'), [20, 92]);
  assert.deepEqual(xy('reject-repeated-auto-row-pattern', 'c1122-reject-pattern-h'), [64, 92]);
  assert.deepEqual(xy('reject-dense-flow', 'c1122-reject-dense-auto-b'), [108, 20]);
  assert.deepEqual(xy('reject-column-flow', 'c1122-reject-column-flow-e'), [108, 20]);
  assert.equal(scene('reject-column-flow').gridContainers[0].properties['grid-template-columns'], '40px 40px 32px');
  assert.equal(scene('reject-column-flow').gridContainers[0].trackCounts.implicitColumns, 1);
  assert.equal(scene('reject-dense-flow').gridContainers[0].properties['grid-auto-flow'], 'dense');
  assert.equal(scene('reject-column-flow').gridContainers[0].properties['grid-auto-flow'], 'column');
  assert.equal(scene('reject-auto-sized-implicit-row').gridContainers[0].properties['grid-auto-rows'], 'auto');
  assert.equal(scene('reject-repeated-auto-row-pattern').gridContainers[0].properties['grid-auto-rows'], '16px 24px');
  assert.ok(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only')
    .every(({ rejectionReason }) => typeof rejectionReason === 'string' && rejectionReason.length > 0));
});

test('출력 경로와 screenshot 파일은 모두 고정되고 기존 기준 파일은 보호된다', async () => {
  assert.equal(paths.reference, 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  await access(join(repositoryRoot, reference.fixture.htmlPath));
  await access(join(repositoryRoot, reference.fixture.inventoryPath));
  for (const screenshot of reference.screenshots) await access(join(repositoryRoot, screenshot.path));
});
