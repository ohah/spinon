import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-auto-placement-inventory-v3.json';
const htmlPath = 'tests/fixtures/css/c11/grid-auto-placement-v3.html';
const referencePath = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json';
const outputPath = 'tests/fixtures/css/c11/grid-auto-placement-runtime-matrix.json';
const wrapperStyle = 'display:flex;width:360px;height:800px';
const expectedRejects = new Map([
  ['reject-implicit-column', {
    code: 'invalid_grid_placement', nodeId: 'c1122-reject-column-item', property: 'grid-column-start',
  }],
  ['reject-auto-sized-implicit-row', {
    code: 'unsupported_grid_auto_rows', nodeId: 'c1122-reject-auto-row-grid', property: 'grid-auto-rows',
  }],
  ['reject-repeated-auto-row-pattern', {
    code: 'unsupported_computed_value', nodeId: 'c1122-reject-pattern-grid', property: 'grid-auto-rows',
  }],
  ['reject-dense-flow', {
    code: 'unsupported_grid_auto_flow', nodeId: 'c1122-reject-dense-grid', property: 'grid-auto-flow',
  }],
  ['reject-column-flow', {
    code: 'unsupported_inline_property', nodeId: 'c1122-reject-column-flow-grid', property: 'grid-auto-columns',
  }],
]);
const knownColors = new Set([
  'surface', 'blue', 'red', 'green', 'yellow', 'purple', 'cyan', 'orange', 'pink',
]);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
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

function paletteFromCss(document) {
  const styleElements = descendants(document, ({ tagName }) => tagName === 'style');
  assert.equal(styleElements.length, 1, 'Grid fixture style 요소 수가 바뀌었습니다.');
  const css = textContent(styleElements[0]);
  const palette = new Map();
  for (const match of css.matchAll(/\.([a-z][a-z0-9-]*)\s*\{\s*background-color:\s*(#[0-9a-f]{6})\s*;?\s*\}/gi)) {
    assert.equal(knownColors.has(match[1]), true, `허용되지 않은 color class입니다: ${match[1]}`);
    palette.set(match[1], match[2].toLowerCase());
  }
  for (const name of knownColors) assert.equal(palette.has(name), true, `paint color 누락: ${name}`);
  return palette;
}

function parseCases(htmlBytes, inventory, reference) {
  const errors = [];
  const document = parse(htmlBytes.toString('utf8'), {
    sourceCodeLocationInfo: true,
    onParseError: (error) => errors.push(error),
  });
  assert.deepEqual(errors, [], 'Grid auto-placement HTML에 parse error가 있습니다.');
  assert.equal(document.mode, 'no-quirks', 'Grid auto-placement fixture는 standards mode여야 합니다.');
  const palette = paletteFromCss(document);
  const sections = descendants(document, (node) => node.tagName === 'section' && attrs(node).has('data-scene'));
  assert.deepEqual(sections.map((node) => attrs(node).get('data-scene')), inventory.cases.map(({ id }) => id));
  const referenceCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1)?.cases;
  assert.ok(referenceCases, 'Chrome DPR 1 reference가 없습니다.');
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
        assert.equal(node.tagName, 'div', `${sceneId}: 허용되지 않은 요소 ${node.tagName}`);
        const nodeAttrs = attrs(node);
        assert.equal(nodeAttrs.has('data-node'), true, `${sceneId}: data-node 누락`);
        const id = nodeAttrs.get('id');
        assert.ok(id && !allIds.has(id), `${sceneId}: ID 누락 또는 중복: ${id}`);
        allIds.add(id);
        const className = nodeAttrs.get('class');
        assert.ok(className && palette.has(className), `${sceneId}/${id}: paint class를 찾지 못했습니다.`);
        const style = nodeAttrs.get('style') ?? '';
        assert.equal(/(?:^|;)\s*background-color\s*:/i.test(style), false,
          `${sceneId}/${id}: inline 배경색과 palette가 중복됩니다.`);
        nodes.push({
          id,
          parentId,
          authoredStyle: style,
          style: `${style}${style && !style.trimEnd().endsWith(';') ? ';' : ''}background-color:${palette.get(className)}`,
        });
        parentId = id;
      }
      for (const child of children(node)) visit(child, parentId);
    };
    visit(section);
    assert.deepEqual(nodes.map(({ id }) => id), expected.nodeIds, `${sceneId}: preorder ID 목록`);
    assert.deepEqual(nodes.map(({ id, parentId }) => ({ id, parentId })),
      referenceCase.nodes.map(({ id, parentId }) => ({ id, parentId })), `${sceneId}: parent graph`);
    const expectedById = new Map(referenceCase.nodes.map((node) => [node.id, node]));
    for (const node of nodes) {
      assert.equal(node.authoredStyle, expectedById.get(node.id)?.authoredStyle,
        `${sceneId}/${node.id}: pinned authored style가 달라졌습니다.`);
    }

    const lines = [
      '(() => {',
      'const refs = Object.create(null);',
      'const nodeIds = [];',
      'const sceneRoot = document.createElement("div");',
      'sceneRoot.setAttribute("id", "c1122-runtime-scene-root");',
      `sceneRoot.setAttribute("style", ${JSON.stringify(wrapperStyle)});`,
      'sceneRoot.setAttribute("data-c1122-grid-wrapper", "");',
      'document.appendChild(sceneRoot);',
    ];
    for (const [nodeIndex, node] of nodes.entries()) {
      const variable = `node_${nodeIndex}`;
      lines.push(`const ${variable} = document.createElement("div");`);
      lines.push(`${variable}.setAttribute("id", ${JSON.stringify(node.id)});`);
      lines.push(`${variable}.setAttribute("data-c1122-grid-node", "");`);
      lines.push(`${variable}.setAttribute("style", ${JSON.stringify(node.style)});`);
      const parent = node.parentId === null ? 'sceneRoot' : `refs[${JSON.stringify(node.parentId)}]`;
      if (node.parentId !== null) assert.ok(nodes.slice(0, nodeIndex).some(({ id }) => id === node.parentId));
      lines.push(`${parent}.appendChild(${variable});`);
      lines.push(`refs[${JSON.stringify(node.id)}] = ${variable};`);
      lines.push(`nodeIds.push(${JSON.stringify(node.id)});`);
    }
    lines.push(`return { sceneId:${JSON.stringify(sceneId)}, rootId:${JSON.stringify(expected.rootId)}, nodeIds };`);
    lines.push('})()');
    const source = lines.join('\n');
    assert.ok(source.length > 0 && source.length <= 64 * 1024, `${sceneId}: JS source 길이`);
    const rejection = expectedRejects.get(sceneId);
    return {
      index,
      id: sceneId,
      scope: expected.scope,
      rootId: expected.rootId,
      nodeIds: expected.nodeIds,
      parentIds: nodes.map(({ parentId }) => parentId),
      expected: rejection
        ? {
          outcome: 'expected-rejection',
          ...rejection,
          runtimeNodeId: expected.nodeIds.indexOf(rejection.nodeId) + 2,
        }
        : { outcome: 'success' },
      source,
      sourceSha256: sha256(source),
    };
  });
}

export async function buildC1122GridAutoPlacementRuntimeMatrix(inputs) {
  const [inventoryBytes, htmlBytes, referenceBytes] = inputs
    ? [Buffer.from(inputs.inventory), Buffer.from(inputs.html), Buffer.from(inputs.reference)]
    : await Promise.all([
      readFile(resolve(repositoryRoot, inventoryPath)),
      readFile(resolve(repositoryRoot, htmlPath)),
      readFile(resolve(repositoryRoot, referencePath)),
    ]);
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(inventory.schema, 'spinon-css-c11-2-2-grid-auto-placement-inventory/v3');
  assert.equal(inventory.fixtureId, 'C11-2-2-grid-auto-placement-precomparison-v3');
  assert.equal(reference.schema, 'spinon-css-c11-2-2-grid-auto-placement-precomparison/v4');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.deepEqual(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').map(({ id }) => id), [
    'definite-first-mixed', 'auto-only-fill', 'positive-implicit-row', 'row-locked-auto-column',
    'column-locked-auto-row', 'auto-column-span', 'sparse-hole-not-backfilled',
    'implicit-track-size-and-gap', 'hidden-child-excluded', 'nested-independent-placement',
    'column-cursor-wrap',
  ]);
  assert.equal(inventory.cases.length, 16);
  assert.deepEqual(reference.comparison, inventory.comparison);
  const cases = parseCases(htmlBytes, inventory, reference);
  assert.equal(cases.reduce((sum, item) => sum + item.nodeIds.length, 0), 69);
  return {
    schema: 'spinon-css-c11-2-2-grid-auto-placement-runtime-matrix/v1',
    inputs: {
      inventory: { path: inventoryPath, sha256: sha256(inventoryBytes) },
      html: { path: htmlPath, sha256: sha256(htmlBytes) },
      chromiumReference: { path: referencePath, sha256: sha256(referenceBytes), referenceId: reference.referenceId },
      generator: { path: 'tools/css-reference/build-c11-2-2-grid-auto-placement-runtime-matrix.mjs' },
    },
    viewportCssPx: { width: inventory.viewport.width, height: inventory.viewport.height },
    wrapperStyle,
    cases,
    summary: { sceneCount: 16, candidateCount: 11, expectedRejectCount: 5, nodeCount: 69 },
  };
}

async function main() {
  const matrix = await buildC1122GridAutoPlacementRuntimeMatrix();
  const generatorPath = resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-auto-placement-runtime-matrix.mjs');
  matrix.inputs.generator.sha256 = sha256(await readFile(generatorPath));
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output,
      `${outputPath}가 고정 inventory, HTML, Chrome reference에서 재생성한 결과와 다릅니다.`);
  } else {
    await writeFile(target, output);
  }
  process.stdout.write(`C11.2.2 runtime matrix: 16 scene · 11 success candidate · 5 expected reject · 69 node · ${process.argv.includes('--check') ? '확인' : '생성'}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
