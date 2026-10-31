import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory-v2.json';
const baselineInventoryPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json';
const historicalRuntimePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
const htmlPath = 'tests/fixtures/css/c11/grid-template-areas-c1133.html';
const referencePath = 'tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json';
const supplementalHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-auto-tracks-c1133-runtime.html';
const supplementalObservationPath = 'spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/chrome/chrome-observed.json';
const cycleHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-auto-track-list-cycling-c1133-runtime.html';
const cycleObservationPath = 'spec/internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/observation.json';
const cycleScreenshotPath = 'spec/internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/chrome/fixture.png';
const typedAutoHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-typed-auto-track-sizing-c1133-runtime.html';
const typedAutoObservationPath = 'spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/observation.json';
const typedAutoScreenshotPath = 'spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/fixture.png';
const outputPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v2.json';
const checkOnly = process.argv.includes('--check');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const attributes = (node) => new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
const children = (node) => (node.childNodes ?? []).filter(({ nodeName }) => nodeName !== '#comment');

function descendants(node, predicate) {
  const found = [];
  const visit = (current) => {
    if (predicate(current)) found.push(current);
    for (const child of children(current)) visit(child);
  };
  visit(node);
  return found;
}

function parseRules(document) {
  const styleNodes = descendants(document, (node) => node.tagName === 'style');
  assert.equal(styleNodes.length, 1, 'C11.3.3 HTML fixture에는 style 요소가 하나여야 합니다.');
  const css = descendants(styleNodes[0], (node) => node.nodeName === '#text')
    .map(({ value }) => value).join('');
  const rules = [];
  for (const match of css.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    const selectors = match[1].split(',').map((selector) => selector.trim()).filter(Boolean);
    const declarations = match[2].split(';').map((declaration) => declaration.trim()).filter(Boolean)
      .map((declaration) => {
        const colon = declaration.indexOf(':');
        assert.ok(colon > 0, `CSS 선언을 읽지 못했습니다: ${declaration}`);
        const property = declaration.slice(0, colon).trim();
        const value = declaration.slice(colon + 1).trim();
        if (property === 'background' && /^#[0-9a-f]{6}$/i.test(value)) {
          return ['background-color', value];
        }
        return [property, value];
      });
    rules.push({ selectors, declarations });
  }
  return rules;
}

function selectorMatches(selector, node, attrs) {
  if (selector === '*') return true;
  if (selector === '[data-node]') return attrs.has('data-node');
  if (!selector.startsWith('.')) return false;
  const classes = new Set((attrs.get('class') ?? '').split(/\s+/).filter(Boolean));
  return selector.slice(1).split('.').every((className) => className && classes.has(className));
}

function authoredDeclarations(source, sceneId, nodeId) {
  return source.split(';').map((declaration) => declaration.trim()).filter(Boolean)
    .map((declaration) => {
      const colon = declaration.indexOf(':');
      assert.ok(colon > 0, `${sceneId}/${nodeId}: authored CSS 선언을 읽지 못했습니다: ${declaration}`);
      const property = declaration.slice(0, colon).trim();
      let value = declaration.slice(colon + 1).trim();
      if (property === 'background' && /^#[0-9a-f]{6}$/i.test(value)) {
        return ['background-color', value];
      }
      return [property, value];
    });
}

function styleFor(node, rules, sceneId) {
  const attrs = attributes(node);
  const declarations = [];
  for (const rule of rules) {
    if (rule.selectors.some((selector) => selectorMatches(selector, node, attrs))) {
      declarations.push(...rule.declarations);
    }
  }
  declarations.push(...authoredDeclarations(attrs.get('style') ?? '', sceneId, attrs.get('id')));
  const alignmentDeclarations = declarations.filter(([property]) =>
    property === 'align-content' || property === 'justify-content');
  assert.ok(alignmentDeclarations.every(([, value]) => value === 'start'),
    `${sceneId}/${attrs.get('id')}: 지원 profile 밖 alignment 선언은 start만 생략할 수 있습니다`);
  return declarations.filter(([property]) =>
    property !== 'align-content' && property !== 'justify-content')
    .map(([property, value]) => `${property}:${value}`).join(';');
}

function createSceneSource(sceneId, nodes, rules) {
  const lines = [
    '(() => {',
    'const refs = Object.create(null);',
    'const nodeIds = [];',
  ];
  for (const [index, node] of nodes.entries()) {
    const variable = `node_${index}`;
    lines.push(`const ${variable} = document.createElement('div');`);
    lines.push(`${variable}.setAttribute('id', ${JSON.stringify(node.id)});`);
    lines.push(`${variable}.setAttribute('data-node', '');`);
    lines.push(`${variable}.setAttribute('style', ${JSON.stringify(styleFor(node.element, rules, sceneId))});`);
    const parent = node.parentId === null ? 'document' : `refs[${JSON.stringify(node.parentId)}]`;
    if (node.parentId !== null) {
      assert.ok(nodes.slice(0, index).some(({ id }) => id === node.parentId),
        `${sceneId}/${node.id}: 부모가 preorder 앞에 없습니다.`);
    }
    lines.push(`${parent}.appendChild(${variable});`);
    lines.push(`refs[${JSON.stringify(node.id)}] = ${variable};`);
    lines.push(`nodeIds.push(${JSON.stringify(node.id)});`);
  }
  const rootId = nodes[0].id;
  lines.push(`return { sceneId:'${sceneId}', rootId:${JSON.stringify(rootId)}, nodeIds };`);
  lines.push('})()');
  return lines.join('\n');
}

const [inventoryBytes, baselineInventoryBytes, historicalRuntimeBytes, htmlBytes, referenceBytes, supplementalHtmlBytes, supplementalObservationBytes,
  cycleHtmlBytes, cycleObservationBytes, cycleScreenshotBytes, typedAutoHtmlBytes, typedAutoObservationBytes, typedAutoScreenshotBytes] = await Promise.all([
  readFile(resolve(repositoryRoot, inventoryPath)),
  readFile(resolve(repositoryRoot, baselineInventoryPath)),
  readFile(resolve(repositoryRoot, historicalRuntimePath)),
  readFile(resolve(repositoryRoot, htmlPath)),
  readFile(resolve(repositoryRoot, referencePath)),
  readFile(resolve(repositoryRoot, supplementalHtmlPath)),
  readFile(resolve(repositoryRoot, supplementalObservationPath)),
  readFile(resolve(repositoryRoot, cycleHtmlPath)),
  readFile(resolve(repositoryRoot, cycleObservationPath)),
  readFile(resolve(repositoryRoot, cycleScreenshotPath)),
  readFile(resolve(repositoryRoot, typedAutoHtmlPath)),
  readFile(resolve(repositoryRoot, typedAutoObservationPath)),
  readFile(resolve(repositoryRoot, typedAutoScreenshotPath)),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const baselineInventory = JSON.parse(baselineInventoryBytes.toString('utf8'));
const historicalRuntime = JSON.parse(historicalRuntimeBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const supplementalObservation = JSON.parse(supplementalObservationBytes.toString('utf8'));
const cycleObservation = JSON.parse(cycleObservationBytes.toString('utf8'));
const typedAutoObservation = JSON.parse(typedAutoObservationBytes.toString('utf8'));
const parseErrors = [];
const document = parse(htmlBytes.toString('utf8'), { sourceCodeLocationInfo: true, onParseError: (error) => parseErrors.push(error) });
const supplementalParseErrors = [];
const cycleParseErrors = [];
const typedAutoParseErrors = [];
const supplementalDocument = parse(supplementalHtmlBytes.toString('utf8'), {
  sourceCodeLocationInfo: true, onParseError: (error) => supplementalParseErrors.push(error),
});
const cycleDocument = parse(cycleHtmlBytes.toString('utf8'), {
  sourceCodeLocationInfo: true, onParseError: (error) => cycleParseErrors.push(error),
});
const typedAutoDocument = parse(typedAutoHtmlBytes.toString('utf8'), {
  sourceCodeLocationInfo: true, onParseError: (error) => typedAutoParseErrors.push(error),
});
assert.deepEqual(parseErrors, [], 'C11.3.3 HTML fixture에 parse error가 있습니다.');
assert.deepEqual(supplementalParseErrors, [], 'C11.3.3 supplemental HTML fixture에 parse error가 있습니다.');
assert.deepEqual(cycleParseErrors, [], 'C11.3.3 track-list HTML fixture에 parse error가 있습니다.');
assert.deepEqual(typedAutoParseErrors, [], 'C11.3.3 typed auto-track HTML fixture에 parse error가 있습니다.');
assert.equal(inventory.schema, 'spinon-css-c11-3-3-grid-template-areas-inventory/v2');
assert.equal(inventory.supersedes.path, baselineInventoryPath);
assert.equal(inventory.supersedes.sha256, sha256(baselineInventoryBytes));
assert.equal(reference.fixture.inventoryPath, baselineInventoryPath);
assert.equal(reference.fixture.inventorySha256, historicalRuntime.inputs.inventory.sha256);
assert.equal(historicalRuntime.inputs.reference.path, referencePath);
assert.equal(historicalRuntime.inputs.reference.sha256, sha256(referenceBytes));
assert.equal(historicalRuntime.cases.find(({ id }) => id === 'unknown-area-implicit-tracks')
  ?.expected.outcome, 'expected-rejection');
const baselineProjection = JSON.parse(JSON.stringify(inventory));
baselineProjection.schema = baselineInventory.schema;
delete baselineProjection.supersedes;
for (const baselineCase of baselineInventory.cases) {
  const projectedCase = baselineProjection.cases.find(({ id }) => id === baselineCase.id);
  assert.ok(projectedCase, `${baselineCase.id}: baseline inventory case`);
  projectedCase.expectedSpinonOutcome = baselineCase.expectedSpinonOutcome;
}
assert.deepEqual(baselineProjection, baselineInventory,
  'runtime inventory v2는 기존 fixture 분류와 unknown-area 결과만 달라야 합니다.');
assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
assert.equal(inventory.cases.length, 26);
assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
assert.equal(supplementalObservation.schema, 'spinon-css-c11-3-3-area-auto-tracks-chrome-observation/v1');
assert.equal(supplementalObservation.fixture.path, supplementalHtmlPath);
assert.equal(supplementalObservation.fixture.sha256, sha256(supplementalHtmlBytes));
assert.equal(supplementalObservation.devicePixelRatio, 1);
assert.equal(cycleObservation.schema, 'spinon-css-c11-3-3-area-auto-track-list-cycle-chrome-observation/v1');
assert.equal(cycleObservation.fixture.path, cycleHtmlPath);
assert.equal(cycleObservation.fixture.sha256, sha256(cycleHtmlBytes));
assert.equal(cycleObservation.devicePixelRatio, 1);
assert.deepEqual(cycleObservation.viewport, { width: 1280, height: 633 });
assert.equal(cycleObservation.capture.tool, 'agent-browser');
assert.equal(cycleObservation.capture.toolVersion, '0.31.1');
assert.equal(cycleObservation.capture.browserVersion, '156.0.0.0');
assert.match(cycleObservation.userAgent, /HeadlessChrome\/156\.0\.0\.0/);
assert.equal(cycleObservation.screenshot.path, cycleScreenshotPath);
assert.equal(cycleObservation.screenshot.sha256, sha256(cycleScreenshotBytes));
assert.equal(typedAutoObservation.schema,
  'spinon-css-c11-3-3-template-area-typed-auto-track-sizing-chrome-observation/v1');
assert.equal(typedAutoObservation.fixture.path, typedAutoHtmlPath);
assert.equal(typedAutoObservation.fixture.sha256, sha256(typedAutoHtmlBytes));
assert.equal(typedAutoObservation.devicePixelRatio, 1);
assert.deepEqual(typedAutoObservation.viewport, { width: 1280, height: 633 });
assert.equal(typedAutoObservation.capture.tool, 'agent-browser');
assert.equal(typedAutoObservation.capture.toolVersion, '0.31.1');
assert.equal(typedAutoObservation.capture.browserVersion, '156.0.0.0');
assert.match(typedAutoObservation.userAgent, /HeadlessChrome\/156\.0\.0\.0/);
assert.equal(typedAutoObservation.screenshot.path, typedAutoScreenshotPath);
assert.equal(typedAutoObservation.screenshot.sha256, sha256(typedAutoScreenshotBytes));

const rules = parseRules(document);
const supplementalRules = parseRules(supplementalDocument);
const cycleRules = parseRules(cycleDocument);
const typedAutoRules = parseRules(typedAutoDocument);
const sections = descendants(document, (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(sections.map((node) => attributes(node).get('data-scene')),
  inventory.cases.map(({ id }) => id), 'HTML fixture scene ID/순서가 inventory와 다릅니다.');
const supplementalSections = descendants(supplementalDocument,
  (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(supplementalSections.map((node) => attributes(node).get('data-scene')),
  ['area-auto-tracks-none', 'area-auto-tracks-short-row', 'area-auto-tracks-short-col',
    'area-auto-tracks-auto', 'area-auto-tracks-long'], 'supplemental scene ID/순서가 다릅니다.');
const cycleSections = descendants(cycleDocument,
  (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(cycleSections.map((node) => attributes(node).get('data-scene')),
  ['both-axis-cycle', 'row-cycle-from-first-implicit', 'column-cycle-from-first-implicit',
    'authored-tracks-exceed-area', 'row-cycle-continues-after-area',
    'column-cycle-continues-after-area'], 'track-list scene ID/순서가 다릅니다.');
const typedAutoSections = descendants(typedAutoDocument,
  (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(typedAutoSections.map((node) => attributes(node).get('data-scene')),
  ['typed-area-minmax-row', 'typed-area-percentage-column'], 'typed auto-track scene ID/순서가 다릅니다.');
const referenceCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
const expectedRejections = new Map();
const allNodeIds = new Set();
let nodeCount = 0;
let cascadeFallbackCount = 0;
let expectedRejectCount = 0;

function nodesFor(section, sceneId) {
  const nodes = [];
  const visit = (node, parentId = null) => {
    const attrs = attributes(node);
    let nextParent = parentId;
    if (attrs.has('data-node')) {
      assert.equal(node.tagName, 'div', `${sceneId}: 지원하지 않는 element ${node.tagName}`);
      const id = attrs.get('id');
      assert.ok(id && !allNodeIds.has(id), `${sceneId}: 비어 있거나 중복된 node ID ${id}`);
      allNodeIds.add(id);
      nodes.push({ id, parentId, element: node });
      nextParent = id;
    }
    for (const child of children(node)) visit(child, nextParent);
  };
  visit(section);
  return nodes;
}

const definitions = [
  ...sections.map((section, index) => {
    const fixtureCase = inventory.cases[index];
    const sceneId = attributes(section).get('data-scene');
    const chromeCase = referenceCases.find((candidate) => candidate.id === sceneId);
    assert.ok(chromeCase, `${sceneId}: DPR 1 Chrome scene이 없습니다.`);
    return { section, sceneId, fixtureCase, rules, chromeCase,
      oracleProduct: reference.oracle.product, oraclePath: referencePath, supplemental: false };
  }),
  ...supplementalSections.map((section) => {
    const sceneId = attributes(section).get('data-scene');
    const observed = supplementalObservation.observations[sceneId];
    assert.ok(observed, `${sceneId}: supplemental Chrome observation이 없습니다.`);
    const rootId = attributes(section).get('id');
    const fixtureNodeIds = descendants(section,
      (node) => attributes(node).has('data-node'))
      .map((node) => attributes(node).get('id'));
    const oracleNodes = observed.nodes.map(({ id, rect }) => ({ id, rect }));
    assert.deepEqual(fixtureNodeIds, oracleNodes.map(({ id }) => id),
      `${sceneId}: supplemental Chrome node 순서가 다릅니다.`);
    return {
      section, sceneId, fixtureCase: {
        rootId, nodeIds: fixtureNodeIds, expectedSpinonOutcome: 'success',
      }, rules: supplementalRules,
      chromeCase: {
        id: sceneId,
        rootId,
        rootRect: { x: 0, y: 0, width: observed.root.width, height: observed.root.height },
        nodes: oracleNodes,
      },
      oracleProduct: `Chrome/${supplementalObservation.capture.browserVersion}`,
      oraclePath: supplementalObservationPath, supplemental: true,
    };
  }),
  ...cycleSections.map((section) => {
    const sceneId = attributes(section).get('data-scene');
    const observed = cycleObservation.observations[sceneId];
    assert.ok(observed, `${sceneId}: track-list Chrome observation이 없습니다.`);
    const rootId = attributes(section).get('id');
    const fixtureNodeIds = descendants(section,
      (node) => attributes(node).has('data-node'))
      .map((node) => attributes(node).get('id'));
    const oracleNodes = observed.nodes.map(({ id, rect }) => ({ id, rect }));
    assert.deepEqual(fixtureNodeIds, oracleNodes.map(({ id }) => id),
      `${sceneId}: track-list Chrome node 순서가 다릅니다.`);
    return {
      section, sceneId, fixtureCase: {
        rootId, nodeIds: fixtureNodeIds, expectedSpinonOutcome: 'success',
      }, rules: cycleRules,
      chromeCase: {
        id: sceneId,
        rootId,
        rootRect: { x: 0, y: 0, width: observed.root.width, height: observed.root.height },
        nodes: oracleNodes,
      },
      oracleProduct: `Chrome/${cycleObservation.capture.browserVersion}`,
      oraclePath: cycleObservationPath, supplemental: true,
    };
  }),
  ...typedAutoSections.map((section) => {
    const sceneId = attributes(section).get('data-scene');
    const observed = typedAutoObservation.observations[sceneId];
    assert.ok(observed, `${sceneId}: typed auto-track Chrome observation이 없습니다.`);
    const rootId = attributes(section).get('id');
    const fixtureNodeIds = descendants(section,
      (node) => attributes(node).has('data-node'))
      .map((node) => attributes(node).get('id'));
    const oracleNodes = observed.nodes.map(({ id, rect }) => ({ id, rect }));
    assert.deepEqual(fixtureNodeIds, oracleNodes.map(({ id }) => id),
      `${sceneId}: typed auto-track Chrome node 순서가 다릅니다.`);
    return {
      section, sceneId, fixtureCase: {
        rootId, nodeIds: fixtureNodeIds, expectedSpinonOutcome: 'success',
      }, rules: typedAutoRules,
      chromeCase: {
        id: sceneId,
        rootId,
        rootRect: { x: 0, y: 0, width: observed.root.width, height: observed.root.height },
        nodes: oracleNodes,
      },
      oracleProduct: `Chrome/${typedAutoObservation.capture.browserVersion}`,
      oraclePath: typedAutoObservationPath, supplemental: true,
    };
  }),
];

const cases = definitions.map(({ section, sceneId, fixtureCase, rules: sceneRules, chromeCase,
  oracleProduct, oraclePath, supplemental }, index) => {
  const nodes = nodesFor(section, sceneId);
  assert.deepEqual(nodes.map(({ id }) => id), fixtureCase.nodeIds, `${sceneId}: inventory node preorder`);
  if (!supplemental) {
    assert.deepEqual(nodes.map(({ id, parentId }) => ({ id, parentId })),
      chromeCase.nodes.map(({ id, parentId }) => ({ id, parentId })), `${sceneId}: Chrome parent graph`);
  }
  assert.equal(nodes[0]?.id, fixtureCase.rootId, `${sceneId}: root ID`);
  nodeCount += nodes.length;

  const outcome = sceneId === 'area-track-count-mismatch'
    ? 'success' : fixtureCase.expectedSpinonOutcome;
  let expected;
  if (outcome === 'success') {
    expected = { outcome: 'success' };
  } else if (outcome === 'cascade-fallback') {
    cascadeFallbackCount += 1;
    expected = { outcome: 'cascade-fallback', runtimeOutcome: 'success' };
  } else if (outcome === 'reject-unsupported-implicit-area-tracks') {
    expectedRejectCount += 1;
    const rejection = expectedRejections.get(sceneId);
    assert.ok(rejection, `${sceneId}: 고정된 rejection 계약이 없습니다.`);
    expected = { outcome: 'expected-rejection', ...rejection };
  } else {
    throw new Error(`${sceneId}: 알 수 없는 expectedSpinonOutcome ${outcome}`);
  }
  const source = createSceneSource(sceneId, nodes, sceneRules);
  assert.ok(source.length > 0 && source.length <= 16 * 1024, `${sceneId}: JS source byte limit`);
  assert.ok(source.includes(`sceneId:'${sceneId}'`), `${sceneId}: source scene marker`);
  return {
    index,
    id: sceneId,
    expected,
    rootId: fixtureCase.rootId,
    nodeIds: nodes.map(({ id }) => id),
    parentIds: nodes.map(({ parentId }) => parentId),
    oracle: {
      product: oracleProduct,
      path: oraclePath,
      sceneId: chromeCase.id,
      rootId: chromeCase.rootId,
      rootRect: chromeCase.rootRect,
      nodes: chromeCase.nodes.map(({ id, rect }) => ({ id, rect })),
    },
    source,
  };
});

const matrix = {
  schema: 'spinon-css-c11-3-3-grid-template-areas-runtime-matrix/v1',
  profileId: 'RuntimeBlockFlexGridTemplateV1',
  inputs: {
    inventory: { path: inventoryPath, sha256: sha256(inventoryBytes) },
    baselineInventory: { path: baselineInventoryPath, sha256: sha256(baselineInventoryBytes) },
    historicalRuntimeMatrix: { path: historicalRuntimePath, sha256: sha256(historicalRuntimeBytes) },
    html: { path: htmlPath, sha256: sha256(htmlBytes) },
    reference: { path: referencePath, sha256: sha256(referenceBytes) },
    supplementalHtml: { path: supplementalHtmlPath, sha256: sha256(supplementalHtmlBytes) },
    supplementalObservation: {
      path: supplementalObservationPath, sha256: sha256(supplementalObservationBytes),
    },
    cycleHtml: { path: cycleHtmlPath, sha256: sha256(cycleHtmlBytes) },
    cycleObservation: {
      path: cycleObservationPath, sha256: sha256(cycleObservationBytes),
    },
    cycleScreenshot: { path: cycleScreenshotPath, sha256: sha256(cycleScreenshotBytes) },
    typedAutoHtml: { path: typedAutoHtmlPath, sha256: sha256(typedAutoHtmlBytes) },
    typedAutoObservation: {
      path: typedAutoObservationPath, sha256: sha256(typedAutoObservationBytes),
    },
    typedAutoScreenshot: {
      path: typedAutoScreenshotPath, sha256: sha256(typedAutoScreenshotBytes),
    },
  },
  summary: {
    sceneCount: cases.length,
    nodeCount,
    successCount: cases.length - cascadeFallbackCount - expectedRejectCount,
    cascadeFallbackCount,
    expectedRejectCount,
  },
  cases,
};
assert.equal(cases.length, 39);
assert.equal(nodeCount, 126);
assert.equal(cascadeFallbackCount, 5);
assert.equal(expectedRejectCount, 0);
assert.equal(allNodeIds.size, nodeCount);

const output = `${JSON.stringify(matrix, null, 2)}\n`;
const outputFile = resolve(repositoryRoot, outputPath);
if (checkOnly) {
  const existing = await readFile(outputFile, 'utf8');
  assert.equal(existing, output, 'C11.3.3 runtime matrix가 inventory/HTML/Chrome reference와 다릅니다. 다시 생성하세요.');
  process.stdout.write(`C11.3.3 runtime matrix 고정 확인: ${cases.length} scene · ${nodeCount} node · ${cascadeFallbackCount} cascade fallback · ${expectedRejectCount} reject\n`);
} else {
  await writeFile(outputFile, output);
  process.stdout.write(`C11.3.3 runtime matrix 생성: ${outputPath} · ${cases.length} scene · ${nodeCount} node\n`);
}
