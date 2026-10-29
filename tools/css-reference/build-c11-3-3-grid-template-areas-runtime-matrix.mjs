import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json';
const htmlPath = 'tests/fixtures/css/c11/grid-template-areas-c1133.html';
const referencePath = 'tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json';
const supplementalHtmlPath = 'tests/fixtures/css/c11/grid-template-areas-auto-tracks-c1133-runtime.html';
const supplementalObservationPath = 'spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/chrome/chrome-observed.json';
const outputPath = 'tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json';
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

const [inventoryBytes, htmlBytes, referenceBytes, supplementalHtmlBytes, supplementalObservationBytes] = await Promise.all([
  readFile(resolve(repositoryRoot, inventoryPath)),
  readFile(resolve(repositoryRoot, htmlPath)),
  readFile(resolve(repositoryRoot, referencePath)),
  readFile(resolve(repositoryRoot, supplementalHtmlPath)),
  readFile(resolve(repositoryRoot, supplementalObservationPath)),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reference = JSON.parse(referenceBytes.toString('utf8'));
const supplementalObservation = JSON.parse(supplementalObservationBytes.toString('utf8'));
const parseErrors = [];
const document = parse(htmlBytes.toString('utf8'), { sourceCodeLocationInfo: true, onParseError: (error) => parseErrors.push(error) });
const supplementalParseErrors = [];
const supplementalDocument = parse(supplementalHtmlBytes.toString('utf8'), {
  sourceCodeLocationInfo: true, onParseError: (error) => supplementalParseErrors.push(error),
});
assert.deepEqual(parseErrors, [], 'C11.3.3 HTML fixture에 parse error가 있습니다.');
assert.deepEqual(supplementalParseErrors, [], 'C11.3.3 supplemental HTML fixture에 parse error가 있습니다.');
assert.equal(inventory.schema, 'spinon-css-c11-3-3-grid-template-areas-inventory/v1');
assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
assert.equal(inventory.cases.length, 26);
assert.deepEqual(reference.observations.map(({ deviceScaleFactor }) => deviceScaleFactor), [1, 2]);
assert.equal(supplementalObservation.schema, 'spinon-css-c11-3-3-area-auto-tracks-chrome-observation/v1');
assert.equal(supplementalObservation.fixture.path, supplementalHtmlPath);
assert.equal(supplementalObservation.fixture.sha256, sha256(supplementalHtmlBytes));
assert.equal(supplementalObservation.devicePixelRatio, 1);

const rules = parseRules(document);
const supplementalRules = parseRules(supplementalDocument);
const sections = descendants(document, (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(sections.map((node) => attributes(node).get('data-scene')),
  inventory.cases.map(({ id }) => id), 'HTML fixture scene ID/순서가 inventory와 다릅니다.');
const supplementalSections = descendants(supplementalDocument,
  (node) => node.tagName === 'div' && attributes(node).has('data-scene'));
assert.deepEqual(supplementalSections.map((node) => attributes(node).get('data-scene')),
  ['area-auto-tracks-none', 'area-auto-tracks-short-row', 'area-auto-tracks-short-col',
    'area-auto-tracks-auto', 'area-auto-tracks-long'], 'supplemental scene ID/순서가 다릅니다.');
const referenceCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
const expectedRejections = new Map([
  ['unknown-area-implicit-tracks', {
    code: 'unsupported_grid_auto_rows', property: 'grid-auto-rows', nodeId: 'unknown-area-grid',
  }],
]);
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
    html: { path: htmlPath, sha256: sha256(htmlBytes) },
    reference: { path: referencePath, sha256: sha256(referenceBytes) },
    supplementalHtml: { path: supplementalHtmlPath, sha256: sha256(supplementalHtmlBytes) },
    supplementalObservation: {
      path: supplementalObservationPath, sha256: sha256(supplementalObservationBytes),
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
assert.equal(nodeCount, 90);
assert.equal(cascadeFallbackCount, 5);
assert.equal(expectedRejectCount, 1);
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
