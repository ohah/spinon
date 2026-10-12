import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-placement-inventory.json';
const htmlPath = 'tests/fixtures/css/c11/grid-placement.html';
const referencePath = 'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json';
const outputPath = 'tests/fixtures/css/c11/grid-placement-runtime-matrix.json';
const wrapperStyle = 'display:flex;width:360px;height:800px';
const expectedFailures = new Map([
  ['deferred-implicit-track', { code: 'invalid_grid_placement', nodeId: 'c1121-implicit-item', property: 'grid-column-start' }],
  ['deferred-named-line', {
    code: 'cascade_failed', nodeId: null, property: null,
    diagnosticIncludes: 'grid-template-columns',
  }],
  ['deferred-mixed-auto-explicit', { code: 'invalid_grid_placement', nodeId: 'c1121-mixed-auto', property: 'grid-row-start' }],
  ['deferred-equal-lines', { code: 'invalid_grid_placement', nodeId: 'c1121-equal-item', property: 'grid-column-end' }],
]);
const colors = new Map([
  ['surface', '#edf1f7'], ['blue', '#2866f6'], ['red', '#e5484d'], ['green', '#19a974'],
  ['yellow', '#ffc857'], ['purple', '#8758ff'], ['cyan', '#0ea5e9'], ['orange', '#f97316'],
  ['pink', '#ec4899'],
]);
const sha256 = (value) => createHash('sha256').update(value).digest('hex');
const attrs = (node) => new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
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

function textContent(node) {
  return node.nodeName === '#text' ? node.value : children(node).map(textContent).join('');
}

function readPalette(document) {
  const styles = descendants(document, ({ tagName }) => tagName === 'style');
  assert.equal(styles.length, 1, 'placement fixture style 요소가 하나여야 합니다.');
  const css = textContent(styles[0]);
  const palette = new Map();
  for (const match of css.matchAll(/\.([a-z][a-z0-9-]*)\s*\{\s*background-color:\s*(#[0-9a-f]{6})\s*;?\s*\}/gi)) {
    assert.equal(colors.has(match[1]), true, `허용되지 않은 color class입니다: ${match[1]}`);
    palette.set(match[1], match[2].toLowerCase());
  }
  assert.deepEqual(palette, colors, 'fixture paint palette가 고정값과 다릅니다.');
  return palette;
}

function styleWithColor(node, palette, sceneId) {
  const attributes = attrs(node);
  const rawStyle = attributes.get('style') ?? '';
  const className = attributes.get('class');
  assert.ok(className && palette.has(className), `${sceneId}/${attributes.get('id')}: 알 수 없는 paint class입니다.`);
  assert.equal(/(?:^|;)\s*background-color\s*:/i.test(rawStyle), false,
    `${sceneId}/${attributes.get('id')}: inline color와 palette color가 겹칩니다.`);
  return `${rawStyle}${rawStyle && !rawStyle.trimEnd().endsWith(';') ? ';' : ''}background-color:${palette.get(className)}`;
}

function parseScenes(html, inventory, reference) {
  const errors = [];
  const document = parse(html, { sourceCodeLocationInfo: true, onParseError: (error) => errors.push(error) });
  assert.deepEqual(errors, [], 'placement HTML fixture에 parse error가 있습니다.');
  assert.equal(document.mode, 'no-quirks', 'placement fixture는 standards mode여야 합니다.');
  const palette = readPalette(document);
  const sections = descendants(document, (node) => node.tagName === 'section' && attrs(node).has('data-scene'));
  assert.deepEqual(sections.map((node) => attrs(node).get('data-scene')), inventory.cases.map(({ id }) => id),
    'HTML scene ID 또는 순서가 inventory와 다릅니다.');
  const referenceCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1)?.cases;
  assert.ok(referenceCases, 'DPR 1 Chrome reference가 없습니다.');
  const allIds = new Set();

  return sections.map((section, index) => {
    const expected = inventory.cases[index];
    const sceneId = attrs(section).get('data-scene');
    const referenceCase = referenceCases[index];
    assert.equal(referenceCase.id, sceneId);
    assert.equal(referenceCase.scope, expected.scope);
    assert.equal(referenceCase.rootId, expected.rootId);
    const wrappers = descendants(section, (node) => node.tagName === 'div' && attrs(node).get('class') === 'scene-root');
    assert.equal(wrappers.length, 1, `${sceneId}: scene-root 수`);
    const nodes = [];
    const visit = (node, parentId = null) => {
      if (node.tagName && node !== section && node !== wrappers[0]) {
        assert.equal(node.tagName, 'div', `${sceneId}: 지원하지 않는 element ${node.tagName}`);
        const nodeAttrs = attrs(node);
        assert.equal(nodeAttrs.has('data-node'), true, `${sceneId}: data-node가 빠졌습니다.`);
        const attributeNames = [...nodeAttrs.keys()].sort();
        assert.ok(attributeNames.join(',') === 'class,data-node,id'
          || attributeNames.join(',') === 'class,data-node,id,style',
        `${sceneId}: 고정 attribute grammar가 바뀌었습니다.`);
        const id = nodeAttrs.get('id');
        assert.ok(id && !allIds.has(id), `${sceneId}: 비어 있거나 중복된 node ID입니다: ${id}`);
        allIds.add(id);
        nodes.push({ id, parentId, style: styleWithColor(node, palette, sceneId) });
        parentId = id;
      }
      for (const child of children(node)) visit(child, parentId);
    };
    visit(section);
    assert.deepEqual(nodes.map(({ id }) => id), expected.nodeIds, `${sceneId}: inventory node preorder`);
    assert.deepEqual(nodes.map(({ id, parentId }) => ({ id, parentId })),
      referenceCase.nodes.map(({ id, parentId }) => ({ id, parentId })), `${sceneId}: reference parent graph`);
    assert.equal(nodes[0].id, expected.rootId, `${sceneId}: root ID`);
    const referenceNodes = new Map(referenceCase.nodes.map((node) => [node.id, node]));
    for (const node of nodes) {
      const sourceNode = descendants(section, (candidate) => attrs(candidate).get('id') === node.id)[0];
      const authoredStyle = attrs(sourceNode).get('style') ?? '';
      assert.equal(authoredStyle, referenceNodes.get(node.id)?.authoredStyle,
        `${sceneId}/${node.id}: authored style가 pinned Chrome input과 다릅니다.`);
    }

    const lines = [
      '(() => {',
      'const refs = Object.create(null);',
      'const nodeIds = [];',
      'const sceneRoot = document.createElement("div");',
      'sceneRoot.setAttribute("id", "c1121-runtime-scene-root");',
      `sceneRoot.setAttribute("style", ${JSON.stringify(wrapperStyle)});`,
      'sceneRoot.setAttribute("data-c1121-grid-wrapper", "");',
      'document.appendChild(sceneRoot);',
    ];
    for (const [nodeIndex, item] of nodes.entries()) {
      const variable = `node_${nodeIndex}`;
      lines.push(`const ${variable} = document.createElement("div");`);
      lines.push(`${variable}.setAttribute("id", ${JSON.stringify(item.id)});`);
      lines.push(`${variable}.setAttribute("data-c1121-grid-node", "");`);
      lines.push(`${variable}.setAttribute("style", ${JSON.stringify(item.style)});`);
      const parent = item.parentId === null ? 'sceneRoot' : `refs[${JSON.stringify(item.parentId)}]`;
      if (item.parentId !== null) assert.ok(nodes.slice(0, nodeIndex).some(({ id }) => id === item.parentId));
      lines.push(`${parent}.appendChild(${variable});`);
      lines.push(`refs[${JSON.stringify(item.id)}] = ${variable};`);
      lines.push(`nodeIds.push(${JSON.stringify(item.id)});`);
    }
    lines.push(`return { sceneId:${JSON.stringify(sceneId)}, rootId:${JSON.stringify(expected.rootId)}, nodeIds };`);
    lines.push('})()');
    const source = lines.join('\n');
    assert.ok(source.length > 0 && source.length <= 64 * 1024, `${sceneId}: JS source 크기`);
    const failure = expectedFailures.get(sceneId);
    return {
      index,
      id: sceneId,
      scope: expected.scope,
      rootId: expected.rootId,
      nodeIds: expected.nodeIds,
      parentIds: nodes.map(({ parentId }) => parentId),
      expected: failure
        ? {
          outcome: 'expected-rejection',
          ...failure,
          runtimeNodeId: failure.nodeId === null ? null : expected.nodeIds.indexOf(failure.nodeId) + 2,
        }
        : { outcome: 'success' },
      source,
      sourceSha256: sha256(source),
    };
  });
}

export async function buildC1121GridPlacementRuntimeMatrix(inputs) {
  const [inventoryBytes, htmlBytes, referenceBytes] = inputs
    ? [Buffer.from(inputs.inventory), Buffer.from(inputs.html), Buffer.from(inputs.reference)]
    : await Promise.all([
      readFile(resolve(repositoryRoot, inventoryPath)),
      readFile(resolve(repositoryRoot, htmlPath)),
      readFile(resolve(repositoryRoot, referencePath)),
    ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(inventory.schema, 'spinon-css-c11-2-1-grid-placement-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-c11-2-1-grid-placement-precomparison/v1');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.deepEqual(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').map(({ id }) => id), [
    'numeric-lines', 'positive-spans', 'reversed-lines', 'shorthand-cascade', 'overlap-source-order', 'nested-placement-modes',
  ]);
  assert.equal(inventory.cases.length, 10);
  assert.deepEqual(reference.comparison, inventory.comparison);
  const cases = parseScenes(htmlBytes.toString('utf8'), inventory, reference);
  assert.equal(cases.reduce((sum, item) => sum + item.nodeIds.length, 0), 30);
  return {
    schema: 'spinon-css-c11-2-1-grid-placement-runtime-matrix/v1',
    inputs: {
      inventory: { path: inventoryPath, sha256: sha256(inventoryBytes) },
      html: { path: htmlPath, sha256: sha256(htmlBytes) },
      chromiumReference: { path: referencePath, sha256: sha256(referenceBytes), referenceId: reference.referenceId },
      generator: { path: 'tools/css-reference/build-c11-2-1-grid-placement-runtime-matrix.mjs' },
    },
    viewportCssPx: { width: inventory.viewport.width, height: inventory.viewport.height },
    wrapperStyle,
    cases,
    summary: { sceneCount: cases.length, candidateCount: 6, expectedRejectCount: 4, nodeCount: 30 },
  };
}

async function main() {
  const matrix = await buildC1121GridPlacementRuntimeMatrix();
  const generatorPath = resolve(repositoryRoot, 'tools/css-reference/build-c11-2-1-grid-placement-runtime-matrix.mjs');
  matrix.inputs.generator.sha256 = sha256(await readFile(generatorPath));
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output,
      `${outputPath}가 고정 inventory, HTML, Chrome reference로 재생성한 결과와 다릅니다.`);
  } else {
    await writeFile(target, output);
  }
  process.stdout.write(`C11.2.1 runtime matrix: 10 scene · 6 success candidate · 4 expected reject · 30 node · ${process.argv.includes('--check') ? '확인' : '생성'}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
